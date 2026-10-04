// 注册到 DuckDB 的函数在这里逐个挂：一个文件一个（或一组）函数，文件名写清「哪一类 + 做什么」。
//
// **首要规则：statrs 的全部能力都要有 SQL 出口，不允许缺失。** 目录与 statrs 模块树同构，
// 逐条核对「statrs 有什么 ↔ 这里暴露了什么」：
//
//   statrs                        duckfn_statrs
//   src/consts.rs                 functions/consts.rs
//   src/function/                 functions/function.rs（含 kernel 族与 evaluate::polynomial）
//   src/statistics/               functions/statistics/
//   src/distribution/             functions/distribution/（含 Categorical、Empirical、
//                                 多元分布 Dyn 维度；二项采样在 sampling.rs）
//   src/density/                  functions/density.rs（kde_pdf / knn_pdf）
//   src/generate.rs               functions/generate.rs（波形/对数间距序列取前 n 项 → LIST）
//   src/stats_tests/              functions/stats_tests/（9 个假设检验）
//   src/euclid.rs                 functions/euclid.rs（Modulus 规范化取模，f64 标量出口）
//   src/prec.rs                   内部精度策略，非计算能力，不包装
//
// 类型面：标量用 DOUBLE；statrs 的向量/矩阵参数走 LIST(DOUBLE)（多元协方差为行主序摊平的
// LIST），计数向量走 LIST(BIGINT)。不存在「为 int/f32 等再包一层转换函数」的重复劳动 ——
// DuckDB 的隐式转换已覆盖。statrs 签名里的 u64/i64 槽位仍经 as_u64/as_i64 做整数校验。
//
// FIRST RULE: every capability of statrs gets a SQL surface; nothing may be missing. The tree
// mirrors statrs' modules row by row (see the table above). Vector and matrix parameters ride on
// LIST(DOUBLE) (covariances as row-major flattened lists), count vectors on LIST(BIGINT) — there
// is no extra wrapper per scalar integer/float type, DuckDB's implicit conversions cover that.
// statrs' own u64/i64 slots stay validated through as_u64/as_i64.

pub(crate) mod distribution;
pub(crate) mod statistics;
pub(crate) mod stats_tests;

mod consts;
mod density;
mod euclid;
mod function;
mod generate;
mod sampling;

use duckfn::{DuckOptionResult, duck_error};
use quack_rs::error::ExtensionError;

/// statrs 用 NAN 表示「算不出」（空输入、样本不足、参数越界…），SQL 侧用 NULL 表达同一个意思。
/// 所有统计量包装统一走这里，不要把 NAN 直接递给用户：它不是 SQL 语义里的值，而是「没有结果」。
///
/// statrs spells "undefined" (empty input, too few samples, an out-of-range parameter) with NAN;
/// SQL says the same thing with NULL. Every statistic wrapper routes through here — never hand a
/// NAN to the user: it is not a value in SQL terms but the absence of one.
pub(crate) fn nan_to_null(value: f64) -> DuckOptionResult<f64> {
    if value.is_nan() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

/// statrs 的离散接口吃无符号整数（阶乘的 n、二项的 k、pmf 的 x）：SQL 字面量进来时做整数
/// 校验与非负检查后转换 —— 3.5 这种「写错的整数」要报错，静默四舍五入会把错误的调用洗白。
/// 错误前缀带函数名，与其它包装同一条约定。
///
/// statrs' discrete interfaces take unsigned integers (factorial n, binomial k, pmf x): literals
/// arrive as doubles, so validate that they really are non-negative integers here — 3.5 is a bad
/// call and must fail, not be silently rounded.
pub(crate) fn as_u64(fn_name: &str, value: f64) -> Result<u64, ExtensionError> {
    if value.is_nan() || value < 0.0 || value.fract() != 0.0 || value > u64::MAX as f64 {
        return Err(duck_error(format!(
            "{fn_name}: expected a non-negative whole number, got {value}"
        )));
    }
    Ok(value as u64)
}

/// `as_u64` 的有符号版（statrs 的 DiscreteUniform 吃 i64）。
///
/// The signed counterpart of `as_u64` (statrs' DiscreteUniform takes i64 bounds).
pub(crate) fn as_i64(fn_name: &str, value: f64) -> Result<i64, ExtensionError> {
    if value.is_nan() || value.fract() != 0.0
        || (value < i64::MIN as f64 || value > i64::MAX as f64)
    {
        return Err(duck_error(format!(
            "{fn_name}: expected a whole number, got {value}"
        )));
    }
    Ok(value as i64)
}

/// u64 列表 → usize 列表（chisquare 的 f_obs 等）。
///
/// A LIST of whole numbers to Vec<usize> (chisquare's observed counts).
pub(crate) fn as_usize_vec(fn_name: &str, values: &[f64]) -> Result<Vec<usize>, ExtensionError> {
    values
        .iter()
        .map(|v| {
            as_u64(fn_name, *v).map(|i| i as usize)
        })
        .collect()
}
