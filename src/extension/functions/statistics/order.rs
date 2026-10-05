// ============================================================================
// statrs::statistics::OrderStatistics 的聚合包装：顺序统计量一族
// （对应 statrs 的 statistics/order_statistics.rs；median/quantile 从旧文件移到这里，
// 让「顺序的」和「求和求平均的」各归各位）
//
// 都走 `auto_collect = true`：状态就是收集好的整列，`Data` 的就地选择算法在 finalize
// 里对一份可丢弃的拷贝跑。带额外参数的（tau、order、p、tie-breaker）写成 `DuckFirst`
// 每查询常量。
//
// Aggregates over statrs' OrderStatistics trait. Everything is `auto_collect`; the extra
// arguments (tau, order, p, tie-breaker) are per-query constants via `DuckFirst`.
// ============================================================================

use duckfn::{DuckFirst, DuckOptionResult, duck_aggregate_function, duck_error};
use statrs::statistics::{Data, OrderStatistics, RankTieBreaker};

use crate::extension::functions::nan_to_null;

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
    let mut data = Data::new(values);
    nan_to_null(data.median())
}

/// `sr_quantile(x, tau)`：tau 分位数（`OrderStatistics::quantile`）。tau 写成第二个常量
/// 参数；tau 越出 `[0, 1]` 是 NAN → NULL。
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
    nan_to_null(data.quantile(tau))
}

/// `sr_order_statistic(x, k)`：第 k 小的值（`OrderStatistics::order_statistic`，1-based）。
/// statrs 的 order 是 `usize` → `k` 用 UBIGINT；k 越出 `1..=n` 是 NAN → NULL。
///
/// ```sql
/// SELECT sr_order_statistic(x, 2) FROM (VALUES (-1.0), (5.0), (0.0), (-3.0), (10.0), (-0.5), (4.0), (1.0), (6.0)) t(x);  -- -1.0
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "k-th smallest value of a DOUBLE column (1-based, k as UBIGINT), NULL when k is outside the data range",
    example = "SELECT sr_order_statistic(x, 2) FROM (VALUES (3.0), (1.0), (2.0)) t(x)"
)]
fn sr_order_statistic(values: Vec<f64>, order: DuckFirst<u64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(values);
    nan_to_null(data.order_statistic(order as usize))
}

/// `sr_percentile(x, p)`：p 百分位（`OrderStatistics::percentile`，0..=100 的整数）。
/// statrs 的 p 是 `usize` → UBIGINT；越界 NAN → NULL；小数百分位用 sr_quantile。
///
/// ```sql
/// SELECT sr_percentile(x, 50) FROM (VALUES (1.0), (5.0), (3.0), (4.0), (10.0), (9.0), (6.0), (7.0), (8.0), (2.0)) t(x);  -- 5.5
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "p-th percentile of a DOUBLE column (p as a UBIGINT in 0..=100), NULL when out of range",
    comment = "Use sr_quantile for non-integer positions",
    example = "SELECT sr_percentile(x, 50) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)"
)]
fn sr_percentile(values: Vec<f64>, p: DuckFirst<u64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(values);
    nan_to_null(data.percentile(p as usize))
}

/// `sr_lower_quartile(x)`：下四分位数（`OrderStatistics::lower_quartile`）。
///
/// ```sql
/// SELECT sr_lower_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x);  -- 1.416666666666666
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "First quartile (lower hinge) of a DOUBLE column, NULL when no row is non-NULL",
    example = "SELECT sr_lower_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)"
)]
fn sr_lower_quartile(values: Vec<f64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(values);
    nan_to_null(data.lower_quartile())
}

/// `sr_upper_quartile(x)`：上四分位数（`OrderStatistics::upper_quartile`）。
///
/// ```sql
/// SELECT sr_upper_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x);  -- 3.5833333333333333
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Third quartile (upper hinge) of a DOUBLE column, NULL when no row is non-NULL",
    example = "SELECT sr_upper_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)"
)]
fn sr_upper_quartile(values: Vec<f64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(values);
    nan_to_null(data.upper_quartile())
}

/// `sr_interquartile_range(x)`：四分位距（`OrderStatistics::interquartile_range`）。
///
/// ```sql
/// SELECT sr_interquartile_range(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x);  -- 2.166666666666667
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Interquartile range of a DOUBLE column, NULL when no row is non-NULL",
    example = "SELECT sr_interquartile_range(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)"
)]
fn sr_interquartile_range(values: Vec<f64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(values);
    nan_to_null(data.interquartile_range())
}

/// `sr_ranks(x, method)`：整列的秩（`OrderStatistics::ranks`），输出 `LIST(DOUBLE)`。
/// method 是每查询常量：1=并列取平均（Average）、2=取最小秩、3=取最大秩、4=并列按出现顺序
/// （statrs 的 First）；其它取值是调用错误。空组 → NULL。
///
/// ```sql
/// SELECT sr_ranks(x, 1.0) FROM (VALUES (1.0), (3.0), (2.0), (2.0)) t(x);  -- [1.0, 4.0, 2.5, 2.5]
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Ranks of a DOUBLE column as LIST(DOUBLE); method 1=average 2=min 3=max 4=first (statrs' tie breakers)",
    comment = "Ties follow the chosen RankTieBreaker; an out-of-range method is a query error",
    example = "SELECT sr_ranks(x, 1.0) FROM (VALUES (1.0), (3.0), (2.0), (2.0)) t(x)"
)]
fn sr_ranks(
    values: Vec<f64>,
    method: DuckFirst<f64>,
) -> DuckOptionResult<Vec<f64>> {
    if values.is_empty() {
        return Ok(None);
    }
    let tie_breaker = match method {
        1.0 => RankTieBreaker::Average,
        2.0 => RankTieBreaker::Min,
        3.0 => RankTieBreaker::Max,
        4.0 => RankTieBreaker::First,
        other => {
            return Err(duck_error(format!(
                "sr_ranks: the tie-breaker method must be 1 (average), 2 (min), 3 (max) or 4 (first), got {other}"
            )));
        }
    };
    let mut data = Data::new(values);
    Ok(Some(data.ranks(tie_breaker)))
}
