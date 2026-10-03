// ============================================================================
// 协方差聚合：两列成对进、一个值出
//
// 形状同 aggregate_summary.rs：`auto_collect = true`，函数本身就是 finalize，没有手写状态。
//
// 两列参数按行配对：任何一列是 NULL 的行整行不进收集（auto_collect 对非 `Option` 列的
// NULL 传播规则），所以「两条边长度不等」在这个形态下根本构造不出来 —— LIST + 标量的
// 形态才需要手工检查长度并报错（statrs 对不等长是直接 panic 的）。
//
// NULL 语义同样对齐 statrs：配对后不足 2 行返回 NAN，统一折成 SQL NULL。
//
// Covariance aggregates: two columns in, paired row by row.
//
// Same shape as aggregate_summary.rs: `auto_collect = true`, the function is the finalize
// handler, no hand-written state.
//
// A NULL in either column skips the whole row, so the "unequal-length" case that statrs
// panics on is unconstructible here — the row pairing itself takes care of it.
// ============================================================================

use duckfn::{DuckOptionResult, duck_aggregate_function};
use statrs::statistics::Statistics;

/// `sr_covariance(x, y)`：样本协方差（`Statistics::covariance`，Bessel 修正）。
///
/// ```sql
/// SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y);  -- -5.5
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Sample covariance of two DOUBLE columns (Bessel-corrected), NULL when fewer than two rows are fully non-NULL",
    comment = "A row with a NULL in either column is skipped entirely, keeping the two columns paired",
    example = "SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)"
)]
fn sr_covariance(x: Vec<f64>, y: Vec<f64>) -> DuckOptionResult<f64> {
    super::nan_to_null(x.covariance(y))
}

/// `sr_population_covariance(x, y)`：总体协方差（除以 N）。
///
/// ```sql
/// SELECT sr_population_covariance(x, y)
/// FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y);  -- -11/3
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Population covariance of two DOUBLE columns, NULL when no row is fully non-NULL",
    comment = "A row with a NULL in either column is skipped entirely, keeping the two columns paired",
    example = "SELECT sr_population_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)"
)]
fn sr_population_covariance(x: Vec<f64>, y: Vec<f64>) -> DuckOptionResult<f64> {
    super::nan_to_null(x.population_covariance(y))
}
