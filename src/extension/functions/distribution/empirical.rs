// ============================================================================
// Empirical（statrs::distribution::Empirical）：数据驱动的分布
//
// 聚合形态：收集整列做样本（statrs 的 FromIterator<f64> 构造），x / p 写成
// `DuckFirst` 每查询常量。statrs 只给它实现了 ContinuousCDF（cdf / sf /
// inverse_cdf），没有 pdf 族 —— 经验分布的密度不是 statrs 的能力，点估计密度在
// density.rs（KDE）出口。
//
// Empirical: a data-driven distribution, collected as an aggregate. statrs only gives
// it ContinuousCDF (cdf / sf / inverse_cdf) — density estimation lives in density.rs.
// ============================================================================

use duckfn::{DuckFirst, DuckOptionResult, duck_aggregate_function};
use statrs::distribution::{ContinuousCDF, Empirical};

use crate::extension::functions::nan_to_null;

/// 把收集到的样本构造成经验分布。
///
/// Builds the empirical distribution from the collected sample.
fn empirical(values: Vec<f64>) -> Empirical {
    values.into_iter().collect()
}

/// `sr_empirical_cdf(x, p)`：经验分布 P(X ≤ p)（样本列进、一个值出；空组 → NULL）。
///
/// ```sql
/// SELECT sr_empirical_cdf(v, 2.0) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v);  -- 0.5
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical CDF of a DOUBLE column evaluated at the second (constant) argument",
    comment = "The sample is the whole column; an empty group yields NULL",
    example = "SELECT sr_empirical_cdf(v, 2.0) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)"
)]
fn sr_empirical_cdf(values: Vec<f64>, at: DuckFirst<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(empirical(values).cdf(at)))
}

/// `sr_empirical_sf(x, p)`：经验生存函数 P(X > p)。
///
/// ```sql
/// SELECT sr_empirical_sf(v, 2.0) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v);  -- 0.5
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical survival function of a DOUBLE column evaluated at the second (constant) argument",
    example = "SELECT sr_empirical_sf(v, 2.0) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)"
)]
fn sr_empirical_sf(values: Vec<f64>, at: DuckFirst<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(empirical(values).sf(at)))
}

/// `sr_empirical_quantile(x, p)`：经验分位数（statrs 的 inverse_cdf，阶梯型）。
///
/// ```sql
/// SELECT sr_empirical_quantile(v, 0.5) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v);  -- 2.0
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical quantile function of a DOUBLE column at the second (constant) probability in [0, 1]",
    example = "SELECT sr_empirical_quantile(v, 0.5) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)"
)]
fn sr_empirical_quantile(values: Vec<f64>, prob: DuckFirst<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    let empirical = empirical(values);
    nan_to_null(empirical.inverse_cdf(prob))
}

// ============================================================================
// 分布矩与域（追加分区）：mean / variance / std_dev / entropy / skewness /
// min / max
//
// 聚合形态：整列进、一个值出；空组一律 NULL。复用上面的 empirical helper。
// Distribution trait 的五个矩都是 Option<f64>，直接透传；min / max 是裸 f64。
// statrs 未给 Empirical 实现 Median / Mode，故无 median / mode；entropy / skewness 也没覆写，
// 走 Distribution trait 默认的 None，两个出口因此恒为 NULL。
//
// Moments and support for the empirical distribution, built on the helper above.
// ============================================================================

use statrs::statistics::{Distribution, Max, Min};

/// `sr_empirical_mean(v)`：经验分布均值（空组 → NULL）。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical mean of a DOUBLE column",
    comment = "The sample is the whole column; an empty group yields NULL",
    example = "SELECT sr_empirical_mean(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_mean(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(empirical(values).mean())
}

/// `sr_empirical_variance(v)`：经验分布方差（空组 → NULL）。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical variance of a DOUBLE column",
    comment = "The sample is the whole column; an empty group yields NULL",
    example = "SELECT sr_empirical_variance(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_variance(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(empirical(values).variance())
}

/// `sr_empirical_std_dev(v)`：经验分布标准差（空组 → NULL）。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical standard deviation of a DOUBLE column",
    comment = "The sample is the whole column; an empty group yields NULL",
    example = "SELECT sr_empirical_std_dev(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_std_dev(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(empirical(values).std_dev())
}

/// `sr_empirical_entropy(v)`：经验分布熵 —— statrs 未给 Empirical 覆写 entropy，
/// 故与空组一样返回 NULL。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical entropy of a DOUBLE column; always NULL, since statrs implements no entropy for the empirical distribution",
    comment = "statrs' Distribution default returns None here, so the result is always NULL",
    example = "SELECT sr_empirical_entropy(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_entropy(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(empirical(values).entropy())
}

/// `sr_empirical_skewness(v)`：经验分布偏度 —— statrs 未给 Empirical 覆写 skewness，
/// 故与空组一样返回 NULL。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical skewness of a DOUBLE column; always NULL, since statrs implements no skewness for the empirical distribution",
    comment = "statrs' Distribution default returns None here, so the result is always NULL",
    example = "SELECT sr_empirical_skewness(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_skewness(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(empirical(values).skewness())
}

/// `sr_empirical_min(v)`：样本最小值（空组 → NULL）。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical minimum of a DOUBLE column",
    comment = "The sample is the whole column; an empty group yields NULL",
    example = "SELECT sr_empirical_min(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_min(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(empirical(values).min()))
}

/// `sr_empirical_max(v)`：样本最大值（空组 → NULL）。
#[duck_aggregate_function(
    auto_collect = true,
    description = "Empirical maximum of a DOUBLE column",
    comment = "The sample is the whole column; an empty group yields NULL",
    example = "SELECT sr_empirical_max(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_empirical_max(values: Vec<f64>) -> DuckOptionResult<f64> {
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(empirical(values).max()))
}
