// ============================================================================
// statrs::statistics 的聚合版包装：一列进、一个值出
//
// 为什么是聚合而不是「LIST + 标量」：`SELECT sr_mean(x) FROM t GROUP BY g` 是数据库用户
// 写统计查询的默认形状，换成标量就得先 `list(x)` 再喂给函数，SQL 更长、也更绕。
//
// 形状：`#[duck_aggregate_function(auto_collect = true)]`（duckfn 0.0.18 起）。
// 注解函数本身就是 finalize 处理器 —— `Vec<T>` 参数是逐行收集的列，`DuckFirst<T>` 是
// 每查询一个的常量，宏按 SummaryState 的形状生成状态、`simple_combine` 与 `result`，
// 这里不再有手写状态结构体、marker 类型或 `DuckAggregateState` impl。
//
// NULL 语义（与 statrs 对齐、向 SQL 用户侧传播；auto_collect 全部免费给）：
//   - SQL NULL 行不进收集（非 `Option` 的列参数，整行跳过 —— 协方差两列因此保持配对）；
//   - statrs 算不出的（空组、方差类不足 2 个值、quantile 的 tau 越界…）返回 NAN，
//     统一经 `nan_to_null` 折成 SQL NULL —— NAN 不会作为值出现在结果里；
//   - 空组上 `DuckFirst<T>`（如 sr_quantile 的 tau）没有值可解，auto_collect 直接对
//     该组报 NULL、不调用函数 —— 与手写时代的守卫语义一致。
//
// Wrapping statrs::statistics as aggregates: a column in, one value out.
//
// Shape: `#[duck_aggregate_function(auto_collect = true)]` (duckfn 0.0.18+). The annotated
// function *is* the finalize handler — a `Vec<T>` parameter collects the column across rows,
// a `DuckFirst<T>` parameter is a per-query constant, and the macro generates the state,
// `simple_combine` and `result`. No hand-written state structs, marker types or
// `DuckAggregateState` impls anymore.
//
// NULL semantics (aligned with statrs, propagated to the SQL side; all free with
// auto_collect): NULL rows never enter the collection; whatever statrs spells NAN becomes
// SQL NULL through the shared `nan_to_null`; an empty group has no value to resolve for
// `DuckFirst<T>`, so auto_collect reports NULL without calling the function.
// ============================================================================

use duckfn::{DuckFirst, DuckOptionResult, duck_aggregate_function};
use statrs::statistics::{Data, OrderStatistics, Statistics};

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
    super::nan_to_null(values.mean())
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
    super::nan_to_null(values.geometric_mean())
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
    super::nan_to_null(values.harmonic_mean())
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
    super::nan_to_null(values.quadratic_mean())
}

/// `sr_median(x)`：中位数（`OrderStatistics::median`，就地选择算法），偶数个取中间两数平均。
///
/// ```sql
/// SELECT sr_median(x) FROM (VALUES (-1.0), (5.0), (0.0), (-3.0), (10.0), (-0.5), (4.0), (1.0), (6.0)) t(x);  -- 1.0
/// SELECT sr_median(x) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x);  -- 2.5
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Median of a DOUBLE column, NULL when no row is non-NULL",
    comment = "Even-length inputs average the two middle values, statrs' own convention",
    example = "SELECT sr_median(x) FROM (VALUES (3.0), (1.0), (2.0)) t(x)"
)]
fn sr_median(values: Vec<f64>) -> DuckOptionResult<f64> {
    // 收集来的 Vec 直接交给 Data 就地选择 —— finalize 之后它就被释放，动的是拷贝的所有权。
    // The collected Vec moves into the in-place `Data`; mutating it here is fine, the state is
    // done with it at finalize.
    let mut data = Data::new(values);
    super::nan_to_null(data.median())
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
    super::nan_to_null(values.variance())
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
    super::nan_to_null(values.std_dev())
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
    super::nan_to_null(values.population_variance())
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
    super::nan_to_null(values.population_std_dev())
}

/// `sr_quantile(x, tau)`：tau 分位数（`OrderStatistics::quantile`）。第二个参数是
/// `DuckFirst<f64>` —— 每查询解析一次的常量，写成 `sr_quantile(x, 0.975)` 即可。
/// tau 为 NULL 的行整行不进收集；空组没有值可解，auto_collect 直接报 NULL、不调用本函数。
///
/// ```sql
/// SELECT sr_quantile(x, 0.5) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x);  -- 2.5
/// SELECT sr_quantile(x, 1.5) FROM (VALUES (1.0), (2.0)) t(x);                -- NULL（tau 越界）
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Tau quantile of a DOUBLE column, tau as the second (constant) argument, NULL when empty or tau is not in [0, 1]",
    comment = "Write the tau argument as a literal; a NULL tau skips the row entirely, like any NULL input",
    example = "SELECT sr_quantile(x, 0.5) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)"
)]
fn sr_quantile(values: Vec<f64>, tau: DuckFirst<f64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(values);
    super::nan_to_null(data.quantile(tau))
}
