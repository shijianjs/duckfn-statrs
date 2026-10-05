// 注册到 DuckDB 的函数在这里逐个挂：一个文件一个（或一组）函数，文件名写清「哪一类 + 做什么」。
//
// **首要规则：statrs 的全部能力都要有 SQL 出口，不允许缺失。** 目录与 statrs 模块树同构，
// 逐条核对「statrs 有什么 ↔ 这里暴露了什么」：
//
//   statrs                        duckfn_statrs
//   src/consts.rs                 functions/consts.rs
//   src/prec.rs                   functions/consts.rs 尾部（5 个精度阈值 + almost_eq；
//                                 与 consts 同属精度策略，放一起比另开文件好找）
//   src/function/                 functions/function.rs（含 kernel 族与 evaluate::polynomial）
//   src/statistics/               functions/statistics/
//   src/distribution/             functions/distribution/（含 Categorical、Empirical、
//                                 多元分布 Dyn 维度；二项采样在 sampling.rs）
//   src/density/                  functions/density.rs（kde_pdf / knn_pdf）
//   src/generate.rs               functions/generate.rs（波形/对数间距序列取前 n 项 → LIST）
//   src/stats_tests/              functions/stats_tests/（9 个假设检验）
//   src/euclid.rs                 functions/euclid.rs（Modulus 规范化取模，f64 标量出口）
//
// 有意不包装的，只剩两类：
//   1. `checked_*` 变体（function/ 里）：SQL 侧统一「算不出 → NULL」，checked 的错误路径
//      与直接版的 NAN 收敛到同一个出口，包两份只是把同一个函数注册两次。
//   2. statrs 里只在别的类型上重复的同义实现（Modulus 的 f32/整数版、各分布构造函数本身）：
//      DuckDB 的隐式转换已覆盖，或 SQL 里没有对应的形状。
//
// 类型面：参数与返回类型尽可能照 statrs 原文透出 —— u64 → UBIGINT、i64 → BIGINT、
// u32 → UINTEGER，实数 → DOUBLE，布尔 → BOOLEAN；statrs 的向量/矩阵走 LIST(DOUBLE)
// （多元协方差、精度矩阵为行主序摊平的 LIST），计数向量走 LIST(UBIGINT)（DiscreteUniform
// 是 i64 → LIST(BIGINT)）。唯一刻意的简化：一个参数若是 `Into<f64>` 泛型（多种类型都能进），
// 不为其重复声明多个入参类型，只留实数入口，其余一律按 statrs 的原始类型。
//
// FIRST RULE: every capability of statrs gets a SQL surface; nothing may be missing. The tree
// mirrors statrs' modules row by row (see the table above). Argument and return types follow
// statrs literally: u64 → UBIGINT, i64 → BIGINT, u32 → UINTEGER, reals → DOUBLE, bools →
// BOOLEAN; vector/matrix parameters ride on LIST(DOUBLE) (covariance and precision matrices as
// row-major flattened lists), count vectors on LIST(UBIGINT) (DiscreteUniform's i64 bounds are
// LIST(BIGINT)). The one deliberate simplification: a generic `Into<f64>` parameter is declared
// once, at the real-valued entry point, instead of one wrapper per accepted type.
// Only two kinds of item stay unwrapped: the `checked_*` error-path twins of already-wrapped
// functions, and statrs' same-capability repeats on other types (see the numbered list above).

pub(crate) mod distribution;
pub(crate) mod statistics;
pub(crate) mod stats_tests;

mod consts;
mod density;
mod euclid;
mod function;
mod generate;
mod sampling;

use duckfn::DuckOptionResult;

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