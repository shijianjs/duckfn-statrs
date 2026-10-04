// ============================================================================
// statrs::stats_tests 的 9 个假设检验（对应关系见 ../mod.rs）。
//
// 样本经 LIST(DOUBLE) 参数进来（聚合列可以先 `list(x)` 聚上来）；返回
// LIST(DOUBLE) [statistic, p_value]，与 statrs 的元组一一对应。检验自身的
// 前置不满足（样本太小、类别数不足）是 statrs 的 Err → 查询错误。
//
// All nine statrs hypothesis tests. Samples arrive as LIST(DOUBLE); the result is
// LIST(DOUBLE) [statistic, p_value], the tuple statrs returns.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{
    ContinuousCDF, Exp as Exponential, Gumbel, LogNormal, Normal, Uniform as ContUniform, Weibull,
};
use statrs::stats_tests::NaNPolicy;
use statrs::stats_tests::anderson_darling::anderson_darling;
use statrs::stats_tests::chisquare::chisquare;
use statrs::stats_tests::f_oneway::f_oneway;
use statrs::stats_tests::fisher::{fishers_exact, fishers_exact_with_odds_ratio};
use statrs::stats_tests::ks_test::{
    ks_onesample, ks_twosample, KSOneSampleAlternativeMethod, KSTwoSampleAlternativeMethod,
};
use statrs::stats_tests::mannwhitneyu::{mannwhitneyu, MannWhitneyUMethod};
use statrs::stats_tests::skewtest::skewtest;
use statrs::stats_tests::ttest_onesample::ttest_onesample;

use super::{alternative, nan_policy, pair};
use crate::extension::functions::{as_u64, as_usize_vec};

/// `sr_ttest_onesample(x, mu, alternative, nan_policy)`：单样本 t 检验
/// （statrs::stats_tests::ttest_onesample）。返回 [t 统计量, p 值]。
#[duck_scalar_function(
    description = "One-sample t-test of a LIST(DOUBLE) sample against popmean: LIST [t statistic, p-value]; alternative 1 two-sided 2 less 3 greater, NaN policy 1 propagate 2 emit 3 error",
    example = "SELECT sr_ttest_onesample([1.0, 2.0, 3.0, 4.0], 2.5, 1.0, 1.0)"
)]
fn sr_ttest_onesample(
    x: Vec<f64>,
    popmean: f64,
    alt: f64,
    nan: f64,
) -> DuckOptionResult<Vec<f64>> {
    let alternative = alternative("sr_ttest_onesample", alt)?;
    let nan_policy = nan_policy("sr_ttest_onesample", nan)?;
    let (statistic, p_value) = ttest_onesample(x, popmean, alternative, nan_policy)
        .map_err(|e| duck_error(format!("sr_ttest_onesample: {e}")))?;
    Ok(Some(pair(statistic, p_value)))
}

/// `sr_skewtest(x, alternative, nan_policy)`：偏度 Z 检验（statrs::skewtest）。
#[duck_scalar_function(
    description = "Skewness z-test of a LIST(DOUBLE) sample: LIST [z statistic, p-value]; same alternative and NaN-policy codes as sr_ttest_onesample",
    example = "SELECT sr_skewtest([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 1.0, 1.0)"
)]
fn sr_skewtest(x: Vec<f64>, alt: f64, nan: f64) -> DuckOptionResult<Vec<f64>> {
    let alternative = alternative("sr_skewtest", alt)?;
    let nan_policy = nan_policy("sr_skewtest", nan)?;
    let (statistic, p_value) = skewtest(x, alternative, nan_policy)
        .map_err(|e| duck_error(format!("sr_skewtest: {e}")))?;
    Ok(Some(pair(statistic, p_value)))
}

/// Anderson-Darling 的泛型入口：statrs 的 anderson_darling 对 dist 要求 `T: Sized`，
/// 所以按名字分发到各个具体分布后逐个调用（不能走 &dyn 动态分发）。
fn ad<T: ContinuousCDF<f64, f64>>(x: &[f64], d: &T) -> Result<Option<Vec<f64>>, ExtensionError> {
    anderson_darling(x, d)
        .map(|(statistic, critical)| Some(pair(statistic, critical)))
        .map_err(ad_bad)
}

/// 泛型错误包装：各个 *Error 类型都 impl Display，函数项在每个调用点独立实例化，
/// 这点比闭包好用（闭包会被推断钉死在第一个错误类型上）。
fn ad_bad<E: core::fmt::Display>(e: E) -> ExtensionError {
    duck_error(format!("sr_anderson_darling: {e}"))
}

/// `sr_anderson_darling(x, dist, params)`：Anderson-Darling 拟合优度检验
/// （statrs::anderson_darling，泛型 dist 按名字实例化）。返回 [A² 统计量, 临界值(5%)]。
/// dist ∈ { normal(mu,sd), lognormal(loc,scale), exponential(rate), gumbel(loc,scale),
/// weibull(shape,scale), uniform(min,max) }；其余名字报查询错误。
#[duck_scalar_function(
    description = "Anderson-Darling goodness-of-fit test of a LIST(DOUBLE) sample against a named distribution (normal / lognormal / exponential / gumbel / weibull / uniform) with its parameter LIST: LIST [A-squared, 5% critical value]",
    example = "SELECT sr_anderson_darling([1.0, 2.0, 3.0, 4.0], 'normal', [2.5, 1.0])"
)]
fn sr_anderson_darling(
    x: Vec<f64>,
    dist: String,
    params: Vec<f64>,
) -> DuckOptionResult<Vec<f64>> {
    let expect = |name: &str, need: usize| -> Result<Vec<f64>, ExtensionError> {
        if params.len() != need {
            return Err(duck_error(format!(
                "sr_anderson_darling: {name} takes {need} parameters, got {}",
                params.len()
            )));
        }
        Ok(params.clone())
    };
    match dist.as_str() {
        "normal" => {
            let p = expect("normal", 2)?;
            let d = Normal::new(p[0], p[1]).map_err(ad_bad)?;
            ad(&x, &d)
        }
        "lognormal" => {
            let p = expect("lognormal", 2)?;
            let d = LogNormal::new(p[0], p[1]).map_err(ad_bad)?;
            ad(&x, &d)
        }
        "exponential" => {
            let p = expect("exponential", 1)?;
            let d = Exponential::new(p[0]).map_err(ad_bad)?;
            ad(&x, &d)
        }
        "gumbel" => {
            let p = expect("gumbel", 2)?;
            let d = Gumbel::new(p[0], p[1]).map_err(ad_bad)?;
            ad(&x, &d)
        }
        "weibull" => {
            let p = expect("weibull", 2)?;
            let d = Weibull::new(p[0], p[1]).map_err(ad_bad)?;
            ad(&x, &d)
        }
        "uniform" => {
            let p = expect("uniform", 2)?;
            let d = ContUniform::new(p[0], p[1]).map_err(ad_bad)?;
            ad(&x, &d)
        }
        other => Err(duck_error(format!(
            "sr_anderson_darling: unknown distribution '{other}' (supported: normal, lognormal, exponential, gumbel, weibull, uniform)"
        ))),
    }
}

/// 单样本 KS 的泛型入口：同 ad，statrs 的 ks_onesample 对 dist 要求 `T: ContinuousCDF`，
/// 按名字分发到各具体分布后逐个调用（不能走 &dyn 动态分发）。
fn kso<T: ContinuousCDF<f64, f64>>(
    x: Vec<f64>,
    d: &T,
    m: KSOneSampleAlternativeMethod,
    np: NaNPolicy,
) -> Result<Option<Vec<f64>>, ExtensionError> {
    ks_onesample(x, d, m, np)
        .map(|(statistic, p_value)| Some(pair(statistic, p_value)))
        .map_err(kso_bad)
}

/// 泛型错误包装（同 ad_bad）：各 *Error 类型都 impl Display。
fn kso_bad<E: core::fmt::Display>(e: E) -> ExtensionError {
    duck_error(format!("sr_ks_onesample: {e}"))
}

/// `sr_ks_onesample(x, dist, params, method, nan_policy)`：单样本 Kolmogorov-Smirnov
/// 拟合优度检验（statrs::ks_onesample，泛型 dist 按名字实例化）。返回 [KS 统计量, p 值]。
/// dist ∈ { normal(mu,sd), lognormal(loc,scale), exponential(rate), gumbel(loc,scale),
/// weibull(shape,scale), uniform(min,max) }；其余名字报查询错误。method 1 = less、
/// 2 = greater、3 = two-sided(精确)、4 = two-sided(渐近)、5 = two-sided(近似)，
/// 对应 KSOneSampleAlternativeMethod。
#[duck_scalar_function(
    description = "One-sample Kolmogorov-Smirnov test of a LIST(DOUBLE) sample against a named distribution (normal / lognormal / exponential / gumbel / weibull / uniform) with its parameter LIST: LIST [KS statistic, p-value]; method 1 less 2 greater 3 two-sided exact 4 two-sided asymptotic 5 two-sided approximate",
    example = "SELECT sr_ks_onesample([1.0, 2.0, 3.0, 4.0], 'normal', [2.5, 1.0], 4.0, 1.0)"
)]
fn sr_ks_onesample(
    x: Vec<f64>,
    dist: String,
    params: Vec<f64>,
    method: f64,
    nan: f64,
) -> DuckOptionResult<Vec<f64>> {
    let method = match method {
        1.0 => KSOneSampleAlternativeMethod::Less,
        2.0 => KSOneSampleAlternativeMethod::Greater,
        3.0 => KSOneSampleAlternativeMethod::TwoSidedExact,
        4.0 => KSOneSampleAlternativeMethod::TwoSidedAsymptotic,
        5.0 => KSOneSampleAlternativeMethod::TwoSidedApproximate,
        other => {
            return Err(duck_error(format!(
                "sr_ks_onesample: the method must be 1 (less), 2 (greater), 3 (two-sided exact), 4 (two-sided asymptotic) or 5 (two-sided approximate), got {other}"
            )));
        }
    };
    let nan_policy = nan_policy("sr_ks_onesample", nan)?;
    let expect = |name: &str, need: usize| -> Result<Vec<f64>, ExtensionError> {
        if params.len() != need {
            return Err(duck_error(format!(
                "sr_ks_onesample: {name} takes {need} parameters, got {}",
                params.len()
            )));
        }
        Ok(params.clone())
    };
    match dist.as_str() {
        "normal" => {
            let p = expect("normal", 2)?;
            let d = Normal::new(p[0], p[1]).map_err(kso_bad)?;
            kso(x, &d, method, nan_policy)
        }
        "lognormal" => {
            let p = expect("lognormal", 2)?;
            let d = LogNormal::new(p[0], p[1]).map_err(kso_bad)?;
            kso(x, &d, method, nan_policy)
        }
        "exponential" => {
            let p = expect("exponential", 1)?;
            let d = Exponential::new(p[0]).map_err(kso_bad)?;
            kso(x, &d, method, nan_policy)
        }
        "gumbel" => {
            let p = expect("gumbel", 2)?;
            let d = Gumbel::new(p[0], p[1]).map_err(kso_bad)?;
            kso(x, &d, method, nan_policy)
        }
        "weibull" => {
            let p = expect("weibull", 2)?;
            let d = Weibull::new(p[0], p[1]).map_err(kso_bad)?;
            kso(x, &d, method, nan_policy)
        }
        "uniform" => {
            let p = expect("uniform", 2)?;
            let d = ContUniform::new(p[0], p[1]).map_err(kso_bad)?;
            kso(x, &d, method, nan_policy)
        }
        other => Err(duck_error(format!(
            "sr_ks_onesample: unknown distribution '{other}' (supported: normal, lognormal, exponential, gumbel, weibull, uniform)"
        ))),
    }
}

/// `sr_ks_twosample(x, y, method, nan_policy)`：两样本 Kolmogorov-Smirnov 检验
/// （statrs::ks_twosample）。method 1 = less(渐近)、2 = greater(渐近)、
/// 3 = two-sided(精确)、4 = two-sided(渐近)，对应 KSTwoSampleAlternativeMethod。
#[duck_scalar_function(
    description = "Two-sample Kolmogorov-Smirnov test of two LIST(DOUBLE) samples: LIST [KS distance, p-value]; method 1 less 2 greater (asymptotic), 3 two-sided exact, 4 two-sided asymptotic",
    example = "SELECT sr_ks_twosample([1.0, 2.0, 3.0], [2.0, 3.0, 4.0], 4.0, 1.0)"
)]
fn sr_ks_twosample(x: Vec<f64>, y: Vec<f64>, method: f64, nan: f64) -> DuckOptionResult<Vec<f64>> {
    let method = match method {
        1.0 => KSTwoSampleAlternativeMethod::LessAsymptotic,
        2.0 => KSTwoSampleAlternativeMethod::GreaterAsymptotic,
        3.0 => KSTwoSampleAlternativeMethod::TwoSidedExact,
        4.0 => KSTwoSampleAlternativeMethod::TwoSidedAsymptotic,
        other => {
            return Err(duck_error(format!(
                "sr_ks_twosample: the method must be 1 (less), 2 (greater), 3 (two-sided exact) or 4 (two-sided asymptotic), got {other}"
            )));
        }
    };
    let nan_policy = nan_policy("sr_ks_twosample", nan)?;
    let (statistic, p_value) = ks_twosample(x, y, method, nan_policy)
        .map_err(|e| duck_error(format!("sr_ks_twosample: {e}")))?;
    Ok(Some(pair(statistic, p_value)))
}

/// `sr_mannwhitneyu(x, y, method, alternative)`：Mann-Whitney U 检验
/// （statrs::mannwhitneyu）。method 1 = Automatic、2 = Exact、
/// 3 = 渐近含连续性校正、4 = 渐近不含连续性校正。
#[duck_scalar_function(
    description = "Mann-Whitney U test of two LIST(DOUBLE) samples: LIST [U statistic, p-value]; method 1 automatic 2 exact 3 asymptotic incl. continuity correction 4 excl., plus the alternative codes",
    example = "SELECT sr_mannwhitneyu([1.0, 2.0, 3.0], [2.0, 3.0, 4.0], 1.0, 1.0)"
)]
fn sr_mannwhitneyu(x: Vec<f64>, y: Vec<f64>, method: f64, alt: f64) -> DuckOptionResult<Vec<f64>> {
    let method = match method {
        1.0 => MannWhitneyUMethod::Automatic,
        2.0 => MannWhitneyUMethod::Exact,
        3.0 => MannWhitneyUMethod::AsymptoticInclContinuityCorrection,
        4.0 => MannWhitneyUMethod::AsymptoticExclContinuityCorrection,
        other => {
            return Err(duck_error(format!(
                "sr_mannwhitneyu: the method must be 1 (automatic), 2 (exact), 3 (asymptotic incl. correction) or 4 (asymptotic excl. correction), got {other}"
            )));
        }
    };
    let alternative = alternative("sr_mannwhitneyu", alt)?;
    let (statistic, p_value) = mannwhitneyu(&x, &y, method, alternative)
        .map_err(|e| duck_error(format!("sr_mannwhitneyu: {e}")))?;
    Ok(Some(pair(statistic, p_value)))
}

/// `sr_chisquare(observed, expected, ddof)`：卡方拟合优度检验（statrs::chisquare）。
/// observed 为计数 LIST（statrs 侧是 &[usize]），expected 可为 NULL（均匀假设，
/// statrs 侧是 &[f64]）；ddof 可为 NULL（默认 0）。
#[duck_scalar_function(
    special_null_handling = true,
    description = "Chi-square goodness-of-fit test on an observed-count LIST (expected frequencies optional, defaults to uniform; ddof optional): LIST [chi-square statistic, p-value]",
    example = "SELECT sr_chisquare([16.0, 18.0, 16.0, 14.0, 12.0, 12.0], NULL, NULL)"
)]
fn sr_chisquare(
    observed: Vec<f64>,
    expected: Option<Vec<f64>>,
    ddof: Option<f64>,
) -> DuckOptionResult<Vec<f64>> {
    let f_obs = as_usize_vec("sr_chisquare", &observed)?;
    let ddof = ddof
        .map(|d| as_u64("sr_chisquare", d))
        .transpose()?
        .map(|d| d as usize);
    let (statistic, p_value) = chisquare(&f_obs, expected.as_deref(), ddof)
        .map_err(|e| duck_error(format!("sr_chisquare: {e}")))?;
    Ok(Some(pair(statistic, p_value)))
}

/// `sr_f_oneway(samples, nan_policy)`：单因素方差分析（statrs::f_oneway），
/// samples 是 LIST(LIST(DOUBLE))。
#[duck_scalar_function(
    description = "One-way ANOVA over a LIST of sample LISTs: LIST [F statistic, p-value]; NaN policy codes as elsewhere",
    example = "SELECT sr_f_oneway([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0]], 1.0)"
)]
fn sr_f_oneway(samples: Vec<Vec<f64>>, nan: f64) -> DuckOptionResult<Vec<f64>> {
    let nan_policy = nan_policy("sr_f_oneway", nan)?;
    let (statistic, p_value) = f_oneway(samples, nan_policy)
        .map_err(|e| duck_error(format!("sr_f_oneway: {e}")))?;
    Ok(Some(pair(statistic, p_value)))
}

/// `sr_fishers_exact(table, alternative)`：Fisher 精确检验的 p 值
/// （statrs::fishers_exact），table 是 2×2 列联表的 LIST（行主序 4 项）。
#[duck_scalar_function(
    description = "Fisher's exact test p-value on a 2x2 contingency table given as a 4-entry whole-number LIST, row-major; alternative codes as elsewhere",
    example = "SELECT sr_fishers_exact([1.0, 2.0, 3.0, 4.0], 1.0)"
)]
fn sr_fishers_exact(table: Vec<f64>, alt: f64) -> DuckOptionResult<f64> {
    let table = fisher_table("sr_fishers_exact", table)?;
    let alternative = alternative("sr_fishers_exact", alt)?;
    fishers_exact(&table, alternative)
        .map(Some)
        .map_err(|e| duck_error(format!("sr_fishers_exact: {e}")))
}

/// `sr_fishers_exact_with_odds_ratio(table, alternative)`：同上，返回
/// [odds ratio, p 值]（statrs::fishers_exact_with_odds_ratio）。
#[duck_scalar_function(
    description = "Fisher's exact test on a 2x2 contingency table (4-entry whole-number LIST, row-major): LIST [odds ratio, p-value]",
    example = "SELECT sr_fishers_exact_with_odds_ratio([1.0, 2.0, 3.0, 4.0], 1.0)"
)]
fn sr_fishers_exact_with_odds_ratio(table: Vec<f64>, alt: f64) -> DuckOptionResult<Vec<f64>> {
    let table = fisher_table("sr_fishers_exact_with_odds_ratio", table)?;
    let alternative = alternative("sr_fishers_exact_with_odds_ratio", alt)?;
    let (odds_ratio, p_value) = fishers_exact_with_odds_ratio(&table, alternative)
        .map_err(|e| duck_error(format!("sr_fishers_exact_with_odds_ratio: {e}")))?;
    Ok(Some(pair(odds_ratio, p_value)))
}

fn fisher_table(name: &str, values: Vec<f64>) -> Result<[u64; 4], ExtensionError> {
    if values.len() != 4 {
        return Err(duck_error(format!(
            "{name}: the contingency table needs exactly 4 entries (row-major 2x2), got {}",
            values.len()
        )));
    }
    let converted: Vec<u64> = values
        .iter()
        .map(|v| as_u64(name, *v))
        .collect::<Result<_, _>>()?;
    Ok([converted[0], converted[1], converted[2], converted[3]])
}
