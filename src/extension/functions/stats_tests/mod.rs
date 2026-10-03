// statrs::stats_tests 的包装层（目录对应 statrs 的 src/stats_tests/），8 个假设检验
// 全数落地在 tests.rs —— 对应关系：
//
//   statrs                              duckfn_statrs
//   ttest_onesample::ttest_onesample    sr_ttest_onesample
//   skewtest::skewtest                  sr_skewtest
//   anderson_darling::anderson_darling  sr_anderson_darling（dist 参数化）
//   ks_test::ks_twosample               sr_ks_twosample
//   mannwhitneyu::mannwhitneyu          sr_mannwhitneyu
//   chisquare::chisquare                sr_chisquare
//   f_oneway::f_oneway                  sr_f_oneway
//   fisher::{fishers_exact, ..._with_odds_ratio}  sr_fishers_exact{,_with_odds_ratio}
//
// 返回形状：statrs 返回 (statistic, p_value) 元组的，SQL 侧出 LIST(DOUBLE)
// [statistic, p_value]；fishers_exact 单独出 p 值一个 DOUBLE。
// Alternative / NaNPolicy / 方法枚举走数值 code（与各文件头注释的对照表一致），
// 与 sr_ranks 的 tie-breaker 同一模式。

mod tests;

use quack_rs::error::ExtensionError;
use statrs::stats_tests::{Alternative, NaNPolicy};

pub(crate) const ALTERNATIVES: &str =
    "1 = two-sided, 2 = less, 3 = greater (statrs' Alternative)";

/// SQL 数值 code → statrs::stats_tests::Alternative。
pub(crate) fn alternative(fn_name: &str, code: f64) -> Result<Alternative, ExtensionError> {
    match code {
        1.0 => Ok(Alternative::TwoSided),
        2.0 => Ok(Alternative::Less),
        3.0 => Ok(Alternative::Greater),
        other => Err(duckfn::duck_error(format!(
            "{fn_name}: the alternative must be {ALTERNATIVES}, got {other}"
        ))),
    }
}

/// SQL 数值 code → statrs::stats_tests::NaNPolicy。SQL 的 NULL 本就进不了样本，
/// NaN 仍可能出现（DOUBLE 能存 'nan'::DOUBLE），策略语义与 statrs 一致。
pub(crate) fn nan_policy(fn_name: &str, code: f64) -> Result<NaNPolicy, ExtensionError> {
    match code {
        1.0 => Ok(NaNPolicy::Propogate),
        2.0 => Ok(NaNPolicy::Emit),
        3.0 => Ok(NaNPolicy::Error),
        other => Err(duckfn::duck_error(format!(
            "{fn_name}: the NaN policy must be 1 = propagate, 2 = emit (filter), 3 = error, got {other}"
        ))),
    }
}

/// 两个 f64 → LIST(DOUBLE) [statistic, value]，检验类函数的统一出口。
pub(crate) fn pair(a: f64, b: f64) -> Vec<f64> {
    vec![a, b]
}
