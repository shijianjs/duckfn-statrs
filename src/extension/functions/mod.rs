// 注册到 DuckDB 的函数在这里逐个挂上：一个文件一个（或一组）函数，文件名写清「哪一类 + 做什么」。
// 功能长大之后再像 duckfn 示例那样分成子目录（`functions/<功能>/mod.rs` + 各司其职的文件）。
//
// Registered functions are attached here one by one: one file per function (or per small group), with
// the file name saying "which kind + what it does". When a feature outgrows a single file, split it
// into a subdirectory the way the duckfn example does (`functions/<feature>/mod.rs` plus one file per
// concern).
mod aggregate_covariance;
mod aggregate_summary;
mod scalar_normal;

use duckfn::DuckOptionResult;

/// statrs 用 NAN 表示「算不出」（空输入、样本不足、参数越界…），SQL 侧用 NULL 表达同一个意思。
/// 所有包装统一走这里，不要把 NAN 直接递给用户：它不是 SQL 语义里的值，而是「没有结果」。
///
/// statrs spells "undefined" (empty input, too few samples, an out-of-range parameter) with NAN;
/// SQL says the same thing with NULL. Every wrapper routes through here — never hand a NAN to the
/// user: it is not a value in SQL terms but the absence of one.
pub(crate) fn nan_to_null(value: f64) -> DuckOptionResult<f64> {
    if value.is_nan() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}
