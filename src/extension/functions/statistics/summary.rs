// ============================================================================
// statrs::statistics::Statistics 的聚合包装：一列进、一个值出
// （顺序统计量在 order.rs，两列协方差在 covariance.rs；对应 statrs 的
// statistics/statistics.rs + statistics/iter_statistics.rs）
//
// 为什么是聚合而不是「LIST + 标量」：`SELECT sr_mean(x) FROM t GROUP BY g` 是数据库用户
// 写统计查询的默认形状，换成标量就得先 `list(x)` 再喂给函数，SQL 更长、也更绕。
//
// 形状：`#[duck_aggregate_function(auto_collect = true)]`（duckfn 0.0.18 起）—— 被注解
// 函数就是 finalize 处理器，`Vec<f64>` 参数是逐行收集的列，状态由宏生成。
//
// NULL 语义（与 statrs 对齐、向 SQL 用户侧传播）：SQL NULL 行不进收集；statrs 算不出的
// （空组、方差类不足 2 个值、几何/调和平均遇到负数…）返回 NAN，统一经 `nan_to_null`
// 折成 SQL NULL —— NAN 不会作为值出现在结果里。
//
// Aggregates over statrs' `Statistics` trait: a column in, one value out (order statistics live
// in order.rs, the two-column covariances in covariance.rs).
//
// NULL semantics: SQL NULL rows never enter the collection; whatever statrs spells NAN becomes
// SQL NULL through the shared `nan_to_null`.
// ============================================================================

use duckfn::{DuckOptionResult, duck_aggregate_function};
use statrs::statistics::Statistics;

use crate::extension::functions::nan_to_null;

/// `sr_mean(x)`：算术平均（`Statistics::mean`）。
///
/// ```sql
/// SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);  -- 2.0
/// SELECT sr_mean(x) FROM range(0) t(x);                      -- NULL（空组）
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Arithmetic mean of a DOUBLE column, NULL when no row is non-NULL",
    comment = "SQL NULL rows are skipped; statrs' NAN for an empty group becomes SQL NULL",
    example = "SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_mean(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.mean())
}

/// `sr_geometric_mean(x)`：几何平均（`Statistics::geometric_mean`），含负数 NULL、含 0 出 0。
///
/// ```sql
/// SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);  -- 1.8171205928321397
/// SELECT sr_geometric_mean(x) FROM (VALUES (-1.0), (2.0)) t(x);        -- NULL
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Geometric mean of a DOUBLE column, NULL when a value is negative or no row is non-NULL",
    comment = "A negative value makes the statistic undefined, which comes back as NULL",
    example = "SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_geometric_mean(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.geometric_mean())
}

/// `sr_harmonic_mean(x)`：调和平均（`Statistics::harmonic_mean`）。
///
/// ```sql
/// SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);  -- 1.6363636363636365
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Harmonic mean of a DOUBLE column, NULL when a value is negative or no row is non-NULL",
    comment = "A negative value makes the statistic undefined, which comes back as NULL",
    example = "SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_harmonic_mean(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.harmonic_mean())
}

/// `sr_quadratic_mean(x)`：平方均值 / RMS（`Statistics::quadratic_mean`）。
///
/// ```sql
/// SELECT sr_quadratic_mean(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 2.0816659994661326
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Quadratic mean (root mean square) of a DOUBLE column, NULL when no row is non-NULL",
    comment = "SQL NULL rows are skipped; statrs' NAN for an empty group becomes SQL NULL",
    example = "SELECT sr_quadratic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_quadratic_mean(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.quadratic_mean())
}

/// `sr_variance(x)`：样本方差（`Statistics::variance`，Bessel 修正、除以 N-1）。
///
/// ```sql
/// SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 19/3
/// SELECT sr_variance(x) FROM (VALUES (1.0)) t(x);                 -- NULL
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Sample variance of a DOUBLE column (Bessel-corrected), NULL when fewer than two rows are non-NULL",
    comment = "One value has no sample variance; the result is NULL, not 0",
    example = "SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_variance(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.variance())
}

/// `sr_std_dev(x)`：样本标准差。
///
/// ```sql
/// SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- sqrt(19/3)
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Sample standard deviation of a DOUBLE column (Bessel-corrected), NULL when fewer than two rows are non-NULL",
    comment = "One value has no sample standard deviation; the result is NULL, not 0",
    example = "SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_std_dev(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.std_dev())
}

/// `sr_population_variance(x)`：总体方差（除以 N）。
///
/// ```sql
/// SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 38/9
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Population variance of a DOUBLE column, NULL when no row is non-NULL",
    comment = "Single-row groups yield a real number here (dividing by N), unlike the sample family",
    example = "SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_population_variance(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.population_variance())
}

/// `sr_population_std_dev(x)`：总体标准差（除以 N）。
///
/// ```sql
/// SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- sqrt(38/9)
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Population standard deviation of a DOUBLE column, NULL when no row is non-NULL",
    comment = "Single-row groups yield a real number here (dividing by N), unlike the sample family",
    example = "SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_population_std_dev(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.population_std_dev())
}

/// `sr_min(x)`：最小值（`Statistics::min`）。DuckDB 自带 `min` 是聚合、语义相同；这里
/// 是为了「statrs 全覆盖」的对照，NaN 传播规则反而更严格（含 NaN 输入在 SQL 侧是 NULL 行，
/// 不会污染）。空组 → NULL。
///
/// ```sql
/// SELECT sr_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- -2.0
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Minimum of a DOUBLE column (statrs' Statistics::min), NULL when no row is non-NULL",
    example = "SELECT sr_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_min(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.min())
}

/// `sr_max(x)`：最大值（`Statistics::max`）。
///
/// ```sql
/// SELECT sr_max(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 3.0
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Maximum of a DOUBLE column (statrs' Statistics::max), NULL when no row is non-NULL",
    example = "SELECT sr_max(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_max(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.max())
}

/// `sr_abs_min(x)`：绝对值最小（`Statistics::abs_min`）——DuckDB 原生没有的最近零点。
///
/// ```sql
/// SELECT sr_abs_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 0.0
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Smallest absolute value in a DOUBLE column (statrs' Statistics::abs_min), NULL when no row is non-NULL",
    example = "SELECT sr_abs_min(x) FROM (VALUES (3.0), (-2.0)) t(x)"
)]
fn sr_abs_min(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.abs_min())
}

/// `sr_abs_max(x)`：绝对值最大（`Statistics::abs_max`）——离零点最远的值。
///
/// ```sql
/// SELECT sr_abs_max(x) FROM (VALUES (0.0), (3.0), (-8.0)) t(x);  -- 8.0
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Largest absolute value in a DOUBLE column (statrs' Statistics::abs_max), NULL when no row is non-NULL",
    example = "SELECT sr_abs_max(x) FROM (VALUES (0.0), (3.0), (-8.0)) t(x)"
)]
fn sr_abs_max(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.abs_max())
}
