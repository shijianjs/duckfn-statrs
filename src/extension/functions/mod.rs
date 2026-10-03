// 注册到 DuckDB 的函数在这里逐个挂：一个文件一个（或一组）函数，文件名写清「哪一类 + 做什么」。
//
// 目录结构刻意与 statrs 的模块树同构，方便逐条核对「statrs 有什么 ↔ 这里暴露了什么」：
//
//   statrs                        duckfn_statrs
//   src/consts.rs                 functions/consts.rs
//   src/function/                 functions/function.rs
//   src/statistics/               functions/statistics/
//   src/distribution/             functions/distribution/
//   src/density/ src/generate/    有意不包装（见各模块头注释与本文件末尾说明）
//   src/stats_tests/              待后续（返回二元统计量/p 值，形状需另行设计）
//
// 对外类型只有一个 DOUBLE：statrs 的 u64/i64 参数（阶乘、分布的 n/k 等）在 SQL 侧都写
// 字面浮点数，转换与整数校验在函数体内做；离散分布的取值 x 同样吃 DOUBLE。
//
// The file tree mirrors statrs' module tree on purpose, so "what statrs has vs. what is exposed
// here" can be checked row by row. Every SQL-facing type is DOUBLE only: statrs' u64/i64
// parameters (factorials, distribution n/k, ...) are written as plain number literals and
// converted inside, and the discrete distributions' x is a DOUBLE too.

pub(crate) mod distribution;
pub(crate) mod statistics;

mod consts;
mod function;

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

/// 对外只有一个 DOUBLE，但 statrs 的离散接口吃无符号整数（阶乘的 n、二项的 k、pmf 的 x）：
/// SQL 侧写成字面浮点数，这里做整数校验与非负检查后转换 —— 3.5 这种「写错的整数」要报错，
/// 静默四舍五入会把错误的调用洗白。错误前缀带函数名，与其它包装同一条约定。
///
/// The SQL surface is DOUBLE only, but statrs' discrete interfaces take unsigned integers
/// (factorial n, binomial k, pmf x): literals arrive as floats, so validate that they really are
/// non-negative integers here — 3.5 is a bad call and must fail, not be silently rounded.
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
