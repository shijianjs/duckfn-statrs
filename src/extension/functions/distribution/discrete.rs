// ============================================================================
// 离散分布（statrs::distribution 的 bernoulli / binomial / discrete_uniform /
// geometric / hypergeometric / negative_binomial / poisson）
//
// 方法集：`sr_<分布>_pmf / ln_pmf / cdf / sf / quantile`（statrs 的 Discrete trait 用 pmf 一词）。
// 类型与 statrs 原文一致：这些分布的 x 是 u64（DiscreteUniform 是 i64），二项的 n、超几何的三个
// 计数也是 u64 —— 对应 SQL 的 UBIGINT / BIGINT，不再是「整数校验过的 DOUBLE」；quantile 走
// DiscreteCDF::inverse_cdf，返回 statrs 原生的 u64/i64。越界概率与非法参数报查询错误，
// statrs 算不出的（NAN）折成 NULL。
//
// Discrete distributions; pmf / ln_pmf / cdf / sf / quantile each. Types follow statrs literally:
// x is u64 (i64 for DiscreteUniform), binomial n and the hypergeometric counts are u64 — UBIGINT /
// BIGINT on the SQL side, no longer whole-number DOUBLEs. quantile delegates to
// DiscreteCDF::inverse_cdf and returns statrs' native u64/i64.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{
    Bernoulli, Binomial, Discrete, DiscreteCDF, DiscreteUniform, Geometric, Hypergeometric,
    NegativeBinomial, Poisson,
};
// NegativeBinomial 是唯一实现 DiscreteDistribution 而非 Distribution 的分布，故两者都要在作用域内。
use statrs::statistics::{DiscreteDistribution, Distribution, Max, Median, Min, Mode};

use super::check_probability;

// ---------------------------------------------------------------------------
// Bernoulli（statrs::distribution::Bernoulli）
// ---------------------------------------------------------------------------

fn bernoulli(name: &str, p: f64) -> Result<Bernoulli, ExtensionError> {
    Bernoulli::new(p).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 伯努利概率质量 P(X = x)，x ∈ {0, 1}。
#[duck_scalar_function(
    description = "Bernoulli probability mass P(X = x) for x in {0, 1}, given success probability p",
    example = "SELECT sr_bernoulli_pmf(1, 0.7)"
)]
fn sr_bernoulli_pmf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_pmf", p)?.pmf(x)))
}

/// 伯努利对数概率质量。
#[duck_scalar_function(
    description = "Bernoulli log probability mass at x",
    example = "SELECT sr_bernoulli_ln_pmf(1, 0.7)"
)]
fn sr_bernoulli_ln_pmf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_ln_pmf", p)?.ln_pmf(x)))
}

/// 伯努利累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Bernoulli cumulative distribution function P(X <= x)",
    example = "SELECT sr_bernoulli_cdf(1, 0.7)"
)]
fn sr_bernoulli_cdf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_cdf", p)?.cdf(x)))
}

/// 伯努利生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Bernoulli survival function P(X > x)",
    example = "SELECT sr_bernoulli_sf(0, 0.7)"
)]
fn sr_bernoulli_sf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_sf", p)?.sf(x)))
}

/// 伯努利分位数函数（返回 0 或 1，以 DOUBLE 呈现）。
#[duck_scalar_function(
    description = "Bernoulli quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_bernoulli_quantile(0.5, 0.7)"
)]
fn sr_bernoulli_quantile(prob: f64, p: f64) -> DuckOptionResult<u64> {
    check_probability("sr_bernoulli_quantile", prob)?;
    Ok(Some(bernoulli("sr_bernoulli_quantile", p)?.inverse_cdf(prob)))
}

// ---------------------------------------------------------------------------
// Binomial（statrs::distribution::Binomial）
// ---------------------------------------------------------------------------

fn binomial(name: &str, p: f64, n: u64) -> Result<Binomial, ExtensionError> {
    Binomial::new(p, n).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 二项概率质量 P(X = x)（成功概率 p、试验次数 n）。
#[duck_scalar_function(
    description = "Binomial probability mass P(X = x), given success probability p and number of trials n",
    example = "SELECT sr_binomial_pmf(3, 0.5, 10)"
)]
fn sr_binomial_pmf(x: u64, p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(Some(binomial("sr_binomial_pmf", p, n)?.pmf(x)))
}

/// 二项对数概率质量。
#[duck_scalar_function(
    description = "Binomial log probability mass at x",
    example = "SELECT sr_binomial_ln_pmf(3, 0.5, 10)"
)]
fn sr_binomial_ln_pmf(x: u64, p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(Some(binomial("sr_binomial_ln_pmf", p, n)?.ln_pmf(x)))
}

/// 二项累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Binomial cumulative distribution function P(X <= x)",
    example = "SELECT sr_binomial_cdf(3, 0.5, 10)"
)]
fn sr_binomial_cdf(x: u64, p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(Some(binomial("sr_binomial_cdf", p, n)?.cdf(x)))
}

/// 二项生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Binomial survival function P(X > x)",
    example = "SELECT sr_binomial_sf(3, 0.5, 10)"
)]
fn sr_binomial_sf(x: u64, p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(Some(binomial("sr_binomial_sf", p, n)?.sf(x)))
}

/// 二项分位数函数。
#[duck_scalar_function(
    description = "Binomial quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_binomial_quantile(0.5, 0.5, 10)"
)]
fn sr_binomial_quantile(prob: f64, p: f64, n: u64) -> DuckOptionResult<u64> {
    check_probability("sr_binomial_quantile", prob)?;
    Ok(Some(binomial("sr_binomial_quantile", p, n)?.inverse_cdf(prob)))
}

// ---------------------------------------------------------------------------
// DiscreteUniform（statrs::distribution::DiscreteUniform，边界为 i64）
// ---------------------------------------------------------------------------

fn discrete_uniform(name: &str, min: i64, max: i64) -> Result<DiscreteUniform, ExtensionError> {
    DiscreteUniform::new(min, max)
        .map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 离散均匀概率质量 P(X = x)（整数边界 [min, max]）。
#[duck_scalar_function(
    description = "Discrete uniform probability mass P(X = x) on the integer range [min, max]",
    example = "SELECT sr_discrete_uniform_pmf(2, 1, 6)"
)]
fn sr_discrete_uniform_pmf(x: i64, min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_pmf", min, max)?.pmf(x)))
}

/// 离散均匀对数概率质量。
#[duck_scalar_function(
    description = "Discrete uniform log probability mass at x",
    example = "SELECT sr_discrete_uniform_ln_pmf(2, 1, 6)"
)]
fn sr_discrete_uniform_ln_pmf(x: i64, min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_ln_pmf", min, max)?.ln_pmf(x)))
}

/// 离散均匀累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Discrete uniform cumulative distribution function P(X <= x)",
    example = "SELECT sr_discrete_uniform_cdf(3, 1, 6)"
)]
fn sr_discrete_uniform_cdf(x: i64, min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_cdf", min, max)?.cdf(x)))
}

/// 离散均匀生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Discrete uniform survival function P(X > x)",
    example = "SELECT sr_discrete_uniform_sf(3, 1, 6)"
)]
fn sr_discrete_uniform_sf(x: i64, min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_sf", min, max)?.sf(x)))
}

/// 离散均匀分位数函数。
#[duck_scalar_function(
    description = "Discrete uniform quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_discrete_uniform_quantile(0.5, 1, 6)"
)]
fn sr_discrete_uniform_quantile(prob: f64, min: i64, max: i64) -> DuckOptionResult<i64> {
    check_probability("sr_discrete_uniform_quantile", prob)?;
    Ok(Some(discrete_uniform("sr_discrete_uniform_quantile", min, max)?.inverse_cdf(prob)))
}

// ---------------------------------------------------------------------------
// Geometric（statrs::distribution::Geometric）
// ---------------------------------------------------------------------------

fn geometric(name: &str, p: f64) -> Result<Geometric, ExtensionError> {
    Geometric::new(p).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 几何概率质量 P(X = x)：首次成功所需的试验数（statrs 的支撑从 1 起）。
#[duck_scalar_function(
    description = "Geometric probability mass P(X = x) for the number of trials until the first success (statrs' support starts at 1)",
    example = "SELECT sr_geometric_pmf(2, 0.5)"
)]
fn sr_geometric_pmf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(geometric("sr_geometric_pmf", p)?.pmf(x)))
}

/// 几何对数概率质量。
#[duck_scalar_function(
    description = "Geometric log probability mass at x",
    example = "SELECT sr_geometric_ln_pmf(2, 0.5)"
)]
fn sr_geometric_ln_pmf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(geometric("sr_geometric_ln_pmf", p)?.ln_pmf(x)))
}

/// 几何累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Geometric cumulative distribution function P(X <= x)",
    example = "SELECT sr_geometric_cdf(2, 0.5)"
)]
fn sr_geometric_cdf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(geometric("sr_geometric_cdf", p)?.cdf(x)))
}

/// 几何生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Geometric survival function P(X > x)",
    example = "SELECT sr_geometric_sf(2, 0.5)"
)]
fn sr_geometric_sf(x: u64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(geometric("sr_geometric_sf", p)?.sf(x)))
}

/// 几何分位数函数。
#[duck_scalar_function(
    description = "Geometric quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_geometric_quantile(0.5, 0.5)"
)]
fn sr_geometric_quantile(prob: f64, p: f64) -> DuckOptionResult<u64> {
    check_probability("sr_geometric_quantile", prob)?;
    Ok(Some(geometric("sr_geometric_quantile", p)?.inverse_cdf(prob)))
}

// ---------------------------------------------------------------------------
// Hypergeometric（statrs::distribution::Hypergeometric，三个计数都是 u64）
// ---------------------------------------------------------------------------

fn hypergeometric(
    name: &str,
    population: u64,
    successes: u64,
    draws: u64,
) -> Result<Hypergeometric, ExtensionError> {
    Hypergeometric::new(population, successes, draws)
    .map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 超几何概率质量 P(X = x)：不放回抽样命中的成功数。
#[duck_scalar_function(
    description = "Hypergeometric probability mass P(X = x): successes drawn without replacement (population, successes, draws as whole-number DOUBLEs)",
    example = "SELECT sr_hypergeometric_pmf(2, 10, 5, 4)"
)]
fn sr_hypergeometric_pmf(x: u64, population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    let dist = hypergeometric("sr_hypergeometric_pmf", population, successes, draws)?;
    Ok(Some(dist.pmf(x)))
}

/// 超几何对数概率质量。
#[duck_scalar_function(
    description = "Hypergeometric log probability mass at x",
    example = "SELECT sr_hypergeometric_ln_pmf(2, 10, 5, 4)"
)]
fn sr_hypergeometric_ln_pmf(x: u64, population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    let dist = hypergeometric("sr_hypergeometric_ln_pmf", population, successes, draws)?;
    Ok(Some(dist.ln_pmf(x)))
}

/// 超几何累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Hypergeometric cumulative distribution function P(X <= x)",
    example = "SELECT sr_hypergeometric_cdf(2, 10, 5, 4)"
)]
fn sr_hypergeometric_cdf(x: u64, population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    let dist = hypergeometric("sr_hypergeometric_cdf", population, successes, draws)?;
    Ok(Some(dist.cdf(x)))
}

/// 超几何生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Hypergeometric survival function P(X > x)",
    example = "SELECT sr_hypergeometric_sf(2, 10, 5, 4)"
)]
fn sr_hypergeometric_sf(x: u64, population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    let dist = hypergeometric("sr_hypergeometric_sf", population, successes, draws)?;
    Ok(Some(dist.sf(x)))
}

/// 超几何分位数函数。
#[duck_scalar_function(
    description = "Hypergeometric quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_hypergeometric_quantile(0.5, 10, 5, 4)"
)]
fn sr_hypergeometric_quantile(prob: f64, population: u64, successes: u64, draws: u64) -> DuckOptionResult<u64> {
    check_probability("sr_hypergeometric_quantile", prob)?;
    let dist = hypergeometric("sr_hypergeometric_quantile", population, successes, draws)?;
    Ok(Some(dist.inverse_cdf(prob)))
}

// ---------------------------------------------------------------------------
// NegativeBinomial（statrs::distribution::NegativeBinomial）
// ---------------------------------------------------------------------------

fn negative_binomial(name: &str, r: f64, p: f64) -> Result<NegativeBinomial, ExtensionError> {
    NegativeBinomial::new(r, p).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 负二项概率质量 P(X = x)：第 r 次成功前的失败次数。
#[duck_scalar_function(
    description = "Negative-binomial probability mass P(X = x): failures before the r-th success (real r, success probability p)",
    example = "SELECT sr_negative_binomial_pmf(3, 2.0, 0.5)"
)]
fn sr_negative_binomial_pmf(x: u64, r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(negative_binomial("sr_negative_binomial_pmf", r, p)?.pmf(x)))
}

/// 负二项对数概率质量。
#[duck_scalar_function(
    description = "Negative-binomial log probability mass at x",
    example = "SELECT sr_negative_binomial_ln_pmf(3, 2.0, 0.5)"
)]
fn sr_negative_binomial_ln_pmf(x: u64, r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(negative_binomial("sr_negative_binomial_ln_pmf", r, p)?.ln_pmf(x)))
}

/// 负二项累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Negative-binomial cumulative distribution function P(X <= x)",
    example = "SELECT sr_negative_binomial_cdf(3, 2.0, 0.5)"
)]
fn sr_negative_binomial_cdf(x: u64, r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(negative_binomial("sr_negative_binomial_cdf", r, p)?.cdf(x)))
}

/// 负二项生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Negative-binomial survival function P(X > x)",
    example = "SELECT sr_negative_binomial_sf(3, 2.0, 0.5)"
)]
fn sr_negative_binomial_sf(x: u64, r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(negative_binomial("sr_negative_binomial_sf", r, p)?.sf(x)))
}

/// 负二项分位数函数。
#[duck_scalar_function(
    description = "Negative-binomial quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_negative_binomial_quantile(0.5, 2.0, 0.5)"
)]
fn sr_negative_binomial_quantile(prob: f64, r: f64, p: f64) -> DuckOptionResult<u64> {
    check_probability("sr_negative_binomial_quantile", prob)?;
    Ok(Some(negative_binomial("sr_negative_binomial_quantile", r, p)?.inverse_cdf(prob)))
}

// ---------------------------------------------------------------------------
// Poisson（statrs::distribution::Poisson）
// ---------------------------------------------------------------------------

fn poisson(name: &str, lambda: f64) -> Result<Poisson, ExtensionError> {
    Poisson::new(lambda).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 泊松概率质量 P(X = x)（强度 lambda）。
#[duck_scalar_function(
    description = "Poisson probability mass P(X = x), given the rate lambda",
    example = "SELECT sr_poisson_pmf(2, 3.0)"
)]
fn sr_poisson_pmf(x: u64, lambda: f64) -> DuckOptionResult<f64> {
    Ok(Some(poisson("sr_poisson_pmf", lambda)?.pmf(x)))
}

/// 泊松对数概率质量。
#[duck_scalar_function(
    description = "Poisson log probability mass at x",
    example = "SELECT sr_poisson_ln_pmf(2, 3.0)"
)]
fn sr_poisson_ln_pmf(x: u64, lambda: f64) -> DuckOptionResult<f64> {
    Ok(Some(poisson("sr_poisson_ln_pmf", lambda)?.ln_pmf(x)))
}

/// 泊松累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Poisson cumulative distribution function P(X <= x)",
    example = "SELECT sr_poisson_cdf(2, 3.0)"
)]
fn sr_poisson_cdf(x: u64, lambda: f64) -> DuckOptionResult<f64> {
    Ok(Some(poisson("sr_poisson_cdf", lambda)?.cdf(x)))
}

/// 泊松生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Poisson survival function P(X > x)",
    example = "SELECT sr_poisson_sf(2, 3.0)"
)]
fn sr_poisson_sf(x: u64, lambda: f64) -> DuckOptionResult<f64> {
    Ok(Some(poisson("sr_poisson_sf", lambda)?.sf(x)))
}

/// 泊松分位数函数。
#[duck_scalar_function(
    description = "Poisson quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_poisson_quantile(0.5, 3.0)"
)]
fn sr_poisson_quantile(prob: f64, lambda: f64) -> DuckOptionResult<u64> {
    check_probability("sr_poisson_quantile", prob)?;
    Ok(Some(poisson("sr_poisson_quantile", lambda)?.inverse_cdf(prob)))
}

// ============================================================================
// 分布矩与域（追加分区）：mean / variance / std_dev / entropy / skewness /
// min / max / median / mode
//
// 复用上面的构造函数 helper。statrs 的 Distribution trait 把 mean / variance /
// std_dev / entropy / skewness 都定为 Option<f64>，直接透传；min / max 是裸整数
// （离散均匀是 i64，其余是 u64），按原类型透出（UBIGINT/BIGINT；支撑上界常为
// u64::MAX = 18446744073709551615）；median 是裸 f64；mode 的类型是
// Option<u64> / Option<i64>，负二项是 Option<f64>。
//
// Moments and support for the distributions above, built on the existing constructors.
// min / max / mode keep statrs' integer types instead of being flattened to DOUBLE.
// ============================================================================

// ---------------------------------------------------------------------------
// Bernoulli（statrs::distribution::Bernoulli）
// ---------------------------------------------------------------------------

/// 伯努利均值。
#[duck_scalar_function(
    description = "Bernoulli mean",
    example = "SELECT sr_bernoulli_mean(0.7)"
)]
fn sr_bernoulli_mean(p: f64) -> DuckOptionResult<f64> {
    Ok(bernoulli("sr_bernoulli_mean", p)?.mean())
}

/// 伯努利方差。
#[duck_scalar_function(
    description = "Bernoulli variance",
    example = "SELECT sr_bernoulli_variance(0.7)"
)]
fn sr_bernoulli_variance(p: f64) -> DuckOptionResult<f64> {
    Ok(bernoulli("sr_bernoulli_variance", p)?.variance())
}

/// 伯努利标准差。
#[duck_scalar_function(
    description = "Bernoulli standard deviation",
    example = "SELECT sr_bernoulli_std_dev(0.7)"
)]
fn sr_bernoulli_std_dev(p: f64) -> DuckOptionResult<f64> {
    Ok(bernoulli("sr_bernoulli_std_dev", p)?.std_dev())
}

/// 伯努利熵。
#[duck_scalar_function(
    description = "Bernoulli entropy",
    example = "SELECT sr_bernoulli_entropy(0.7)"
)]
fn sr_bernoulli_entropy(p: f64) -> DuckOptionResult<f64> {
    Ok(bernoulli("sr_bernoulli_entropy", p)?.entropy())
}

/// 伯努利偏度。
#[duck_scalar_function(
    description = "Bernoulli skewness",
    example = "SELECT sr_bernoulli_skewness(0.7)"
)]
fn sr_bernoulli_skewness(p: f64) -> DuckOptionResult<f64> {
    Ok(bernoulli("sr_bernoulli_skewness", p)?.skewness())
}

/// 伯努利取值下界（0）。
#[duck_scalar_function(
    description = "Bernoulli minimum of the support (0)",
    example = "SELECT sr_bernoulli_min(0.7)"
)]
fn sr_bernoulli_min(p: f64) -> DuckOptionResult<u64> {
    Ok(Some(bernoulli("sr_bernoulli_min", p)?.min()))
}

/// 伯努利取值上界（1）。
#[duck_scalar_function(
    description = "Bernoulli maximum of the support (1)",
    example = "SELECT sr_bernoulli_max(0.7)"
)]
fn sr_bernoulli_max(p: f64) -> DuckOptionResult<u64> {
    Ok(Some(bernoulli("sr_bernoulli_max", p)?.max()))
}

/// 伯努利中位数。
#[duck_scalar_function(
    description = "Bernoulli median",
    example = "SELECT sr_bernoulli_median(0.7)"
)]
fn sr_bernoulli_median(p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_median", p)?.median()))
}

/// 伯努利众数。
#[duck_scalar_function(
    description = "Bernoulli mode",
    example = "SELECT sr_bernoulli_mode(0.7)"
)]
fn sr_bernoulli_mode(p: f64) -> DuckOptionResult<u64> {
    Ok(bernoulli("sr_bernoulli_mode", p)?.mode())
}

// ---------------------------------------------------------------------------
// Binomial（statrs::distribution::Binomial）
// ---------------------------------------------------------------------------

/// 二项均值。
#[duck_scalar_function(
    description = "Binomial mean",
    example = "SELECT sr_binomial_mean(0.5, 10)"
)]
fn sr_binomial_mean(p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(binomial("sr_binomial_mean", p, n)?.mean())
}

/// 二项方差。
#[duck_scalar_function(
    description = "Binomial variance",
    example = "SELECT sr_binomial_variance(0.5, 10)"
)]
fn sr_binomial_variance(p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(binomial("sr_binomial_variance", p, n)?.variance())
}

/// 二项标准差。
#[duck_scalar_function(
    description = "Binomial standard deviation",
    example = "SELECT sr_binomial_std_dev(0.5, 10)"
)]
fn sr_binomial_std_dev(p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(binomial("sr_binomial_std_dev", p, n)?.std_dev())
}

/// 二项熵。
#[duck_scalar_function(
    description = "Binomial entropy",
    example = "SELECT sr_binomial_entropy(0.5, 10)"
)]
fn sr_binomial_entropy(p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(binomial("sr_binomial_entropy", p, n)?.entropy())
}

/// 二项偏度。
#[duck_scalar_function(
    description = "Binomial skewness",
    example = "SELECT sr_binomial_skewness(0.5, 10)"
)]
fn sr_binomial_skewness(p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(binomial("sr_binomial_skewness", p, n)?.skewness())
}

/// 二项取值下界（0）。
#[duck_scalar_function(
    description = "Binomial minimum of the support (0)",
    example = "SELECT sr_binomial_min(0.5, 10)"
)]
fn sr_binomial_min(p: f64, n: u64) -> DuckOptionResult<u64> {
    Ok(Some(binomial("sr_binomial_min", p, n)?.min()))
}

/// 二项取值上界（n）。
#[duck_scalar_function(
    description = "Binomial maximum of the support (n)",
    example = "SELECT sr_binomial_max(0.5, 10)"
)]
fn sr_binomial_max(p: f64, n: u64) -> DuckOptionResult<u64> {
    Ok(Some(binomial("sr_binomial_max", p, n)?.max()))
}

/// 二项中位数。
#[duck_scalar_function(
    description = "Binomial median",
    example = "SELECT sr_binomial_median(0.5, 10)"
)]
fn sr_binomial_median(p: f64, n: u64) -> DuckOptionResult<f64> {
    Ok(Some(binomial("sr_binomial_median", p, n)?.median()))
}

/// 二项众数。
#[duck_scalar_function(
    description = "Binomial mode",
    example = "SELECT sr_binomial_mode(0.5, 10)"
)]
fn sr_binomial_mode(p: f64, n: u64) -> DuckOptionResult<u64> {
    Ok(binomial("sr_binomial_mode", p, n)?.mode())
}

// ---------------------------------------------------------------------------
// DiscreteUniform（statrs::distribution::DiscreteUniform，边界为 i64）
// ---------------------------------------------------------------------------

/// 离散均匀均值。
#[duck_scalar_function(
    description = "Discrete uniform mean",
    example = "SELECT sr_discrete_uniform_mean(1, 6)"
)]
fn sr_discrete_uniform_mean(min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(discrete_uniform("sr_discrete_uniform_mean", min, max)?.mean())
}

/// 离散均匀方差。
#[duck_scalar_function(
    description = "Discrete uniform variance",
    example = "SELECT sr_discrete_uniform_variance(1, 6)"
)]
fn sr_discrete_uniform_variance(min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(discrete_uniform("sr_discrete_uniform_variance", min, max)?.variance())
}

/// 离散均匀标准差。
#[duck_scalar_function(
    description = "Discrete uniform standard deviation",
    example = "SELECT sr_discrete_uniform_std_dev(1, 6)"
)]
fn sr_discrete_uniform_std_dev(min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(discrete_uniform("sr_discrete_uniform_std_dev", min, max)?.std_dev())
}

/// 离散均匀熵。
#[duck_scalar_function(
    description = "Discrete uniform entropy",
    example = "SELECT sr_discrete_uniform_entropy(1, 6)"
)]
fn sr_discrete_uniform_entropy(min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(discrete_uniform("sr_discrete_uniform_entropy", min, max)?.entropy())
}

/// 离散均匀偏度。
#[duck_scalar_function(
    description = "Discrete uniform skewness",
    example = "SELECT sr_discrete_uniform_skewness(1, 6)"
)]
fn sr_discrete_uniform_skewness(min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(discrete_uniform("sr_discrete_uniform_skewness", min, max)?.skewness())
}

/// 离散均匀取值下界（min）。
#[duck_scalar_function(
    description = "Discrete uniform minimum of the support (min)",
    example = "SELECT sr_discrete_uniform_min(1, 6)"
)]
fn sr_discrete_uniform_min(min: i64, max: i64) -> DuckOptionResult<i64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_min", min, max)?.min()))
}

/// 离散均匀取值上界（max）。
#[duck_scalar_function(
    description = "Discrete uniform maximum of the support (max)",
    example = "SELECT sr_discrete_uniform_max(1, 6)"
)]
fn sr_discrete_uniform_max(min: i64, max: i64) -> DuckOptionResult<i64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_max", min, max)?.max()))
}

/// 离散均匀中位数。
#[duck_scalar_function(
    description = "Discrete uniform median",
    example = "SELECT sr_discrete_uniform_median(1, 6)"
)]
fn sr_discrete_uniform_median(min: i64, max: i64) -> DuckOptionResult<f64> {
    Ok(Some(discrete_uniform("sr_discrete_uniform_median", min, max)?.median()))
}

/// 离散均匀众数。
#[duck_scalar_function(
    description = "Discrete uniform mode",
    example = "SELECT sr_discrete_uniform_mode(1, 6)"
)]
fn sr_discrete_uniform_mode(min: i64, max: i64) -> DuckOptionResult<i64> {
    Ok(discrete_uniform("sr_discrete_uniform_mode", min, max)?
        .mode())
}

// ---------------------------------------------------------------------------
// Geometric（statrs::distribution::Geometric）
// ---------------------------------------------------------------------------

/// 几何分布的均值 `1/p`。函数名带 `_dist`：`sr_geometric_mean` 已被
/// statistics/summary.rs 的聚合（一列的几何平均）占用，DuckDB 不允许标量与聚合同名。
#[duck_scalar_function(
    description = "Mean of the Geometric distribution, 1/p (named _dist to avoid clashing with the sr_geometric_mean aggregate over a column)",
    example = "SELECT sr_geometric_dist_mean(0.5)"
)]
fn sr_geometric_dist_mean(p: f64) -> DuckOptionResult<f64> {
    Ok(geometric("sr_geometric_dist_mean", p)?.mean())
}

/// 几何方差。
#[duck_scalar_function(
    description = "Geometric variance",
    example = "SELECT sr_geometric_variance(0.5)"
)]
fn sr_geometric_variance(p: f64) -> DuckOptionResult<f64> {
    Ok(geometric("sr_geometric_variance", p)?.variance())
}

/// 几何标准差。
#[duck_scalar_function(
    description = "Geometric standard deviation",
    example = "SELECT sr_geometric_std_dev(0.5)"
)]
fn sr_geometric_std_dev(p: f64) -> DuckOptionResult<f64> {
    Ok(geometric("sr_geometric_std_dev", p)?.std_dev())
}

/// 几何熵。
#[duck_scalar_function(
    description = "Geometric entropy",
    example = "SELECT sr_geometric_entropy(0.5)"
)]
fn sr_geometric_entropy(p: f64) -> DuckOptionResult<f64> {
    Ok(geometric("sr_geometric_entropy", p)?.entropy())
}

/// 几何偏度。
#[duck_scalar_function(
    description = "Geometric skewness",
    example = "SELECT sr_geometric_skewness(0.5)"
)]
fn sr_geometric_skewness(p: f64) -> DuckOptionResult<f64> {
    Ok(geometric("sr_geometric_skewness", p)?.skewness())
}

/// 几何取值下界（1）。
#[duck_scalar_function(
    description = "Geometric minimum of the support (1)",
    example = "SELECT sr_geometric_min(0.5)"
)]
fn sr_geometric_min(p: f64) -> DuckOptionResult<u64> {
    Ok(Some(geometric("sr_geometric_min", p)?.min()))
}

/// 几何取值上界（u64::MAX，以 1.8e19 呈现）。
#[duck_scalar_function(
    description = "Geometric maximum of the support (u64::MAX, shown as 1.8e19)",
    example = "SELECT sr_geometric_max(0.5)"
)]
fn sr_geometric_max(p: f64) -> DuckOptionResult<u64> {
    Ok(Some(geometric("sr_geometric_max", p)?.max()))
}

/// 几何中位数。
#[duck_scalar_function(
    description = "Geometric median",
    example = "SELECT sr_geometric_median(0.5)"
)]
fn sr_geometric_median(p: f64) -> DuckOptionResult<f64> {
    Ok(Some(geometric("sr_geometric_median", p)?.median()))
}

/// 几何众数。
#[duck_scalar_function(
    description = "Geometric mode",
    example = "SELECT sr_geometric_mode(0.5)"
)]
fn sr_geometric_mode(p: f64) -> DuckOptionResult<u64> {
    Ok(geometric("sr_geometric_mode", p)?.mode())
}

// ---------------------------------------------------------------------------
// Hypergeometric（statrs::distribution::Hypergeometric，三个计数都是 u64）
// ---------------------------------------------------------------------------

/// 超几何均值。
#[duck_scalar_function(
    description = "Hypergeometric mean",
    example = "SELECT sr_hypergeometric_mean(10, 5, 4)"
)]
fn sr_hypergeometric_mean(population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    Ok(hypergeometric("sr_hypergeometric_mean", population, successes, draws)?.mean())
}

/// 超几何方差。
#[duck_scalar_function(
    description = "Hypergeometric variance",
    example = "SELECT sr_hypergeometric_variance(10, 5, 4)"
)]
fn sr_hypergeometric_variance(population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    Ok(hypergeometric("sr_hypergeometric_variance", population, successes, draws)?.variance())
}

/// 超几何标准差。
#[duck_scalar_function(
    description = "Hypergeometric standard deviation",
    example = "SELECT sr_hypergeometric_std_dev(10, 5, 4)"
)]
fn sr_hypergeometric_std_dev(population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    Ok(hypergeometric("sr_hypergeometric_std_dev", population, successes, draws)?.std_dev())
}

/// 超几何熵。
#[duck_scalar_function(
    description = "Hypergeometric entropy",
    example = "SELECT sr_hypergeometric_entropy(10, 5, 4)"
)]
fn sr_hypergeometric_entropy(population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    Ok(hypergeometric("sr_hypergeometric_entropy", population, successes, draws)?.entropy())
}

/// 超几何偏度。
#[duck_scalar_function(
    description = "Hypergeometric skewness",
    example = "SELECT sr_hypergeometric_skewness(10, 5, 4)"
)]
fn sr_hypergeometric_skewness(population: u64, successes: u64, draws: u64) -> DuckOptionResult<f64> {
    Ok(hypergeometric("sr_hypergeometric_skewness", population, successes, draws)?.skewness())
}

/// 超几何取值下界。
#[duck_scalar_function(
    description = "Hypergeometric minimum of the support",
    example = "SELECT sr_hypergeometric_min(10, 5, 4)"
)]
fn sr_hypergeometric_min(population: u64, successes: u64, draws: u64) -> DuckOptionResult<u64> {
    Ok(Some(
        hypergeometric("sr_hypergeometric_min", population, successes, draws)?.min()
    ))
}

/// 超几何取值上界。
#[duck_scalar_function(
    description = "Hypergeometric maximum of the support",
    example = "SELECT sr_hypergeometric_max(10, 5, 4)"
)]
fn sr_hypergeometric_max(population: u64, successes: u64, draws: u64) -> DuckOptionResult<u64> {
    Ok(Some(
        hypergeometric("sr_hypergeometric_max", population, successes, draws)?.max()
    ))
}

/// 超几何众数。
#[duck_scalar_function(
    description = "Hypergeometric mode",
    example = "SELECT sr_hypergeometric_mode(10, 5, 4)"
)]
fn sr_hypergeometric_mode(population: u64, successes: u64, draws: u64) -> DuckOptionResult<u64> {
    Ok(hypergeometric("sr_hypergeometric_mode", population, successes, draws)?
        .mode())
}

// ---------------------------------------------------------------------------
// NegativeBinomial（statrs::distribution::NegativeBinomial）
// ---------------------------------------------------------------------------

/// 负二项均值。
#[duck_scalar_function(
    description = "Negative-binomial mean",
    example = "SELECT sr_negative_binomial_mean(2.0, 0.5)"
)]
fn sr_negative_binomial_mean(r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(negative_binomial("sr_negative_binomial_mean", r, p)?.mean())
}

/// 负二项方差。
#[duck_scalar_function(
    description = "Negative-binomial variance",
    example = "SELECT sr_negative_binomial_variance(2.0, 0.5)"
)]
fn sr_negative_binomial_variance(r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(negative_binomial("sr_negative_binomial_variance", r, p)?.variance())
}

/// 负二项标准差。
#[duck_scalar_function(
    description = "Negative-binomial standard deviation",
    example = "SELECT sr_negative_binomial_std_dev(2.0, 0.5)"
)]
fn sr_negative_binomial_std_dev(r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(negative_binomial("sr_negative_binomial_std_dev", r, p)?.std_dev())
}

/// 负二项熵。
#[duck_scalar_function(
    description = "Negative-binomial entropy",
    example = "SELECT sr_negative_binomial_entropy(2.0, 0.5)"
)]
fn sr_negative_binomial_entropy(r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(negative_binomial("sr_negative_binomial_entropy", r, p)?.entropy())
}

/// 负二项偏度。
#[duck_scalar_function(
    description = "Negative-binomial skewness",
    example = "SELECT sr_negative_binomial_skewness(2.0, 0.5)"
)]
fn sr_negative_binomial_skewness(r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(negative_binomial("sr_negative_binomial_skewness", r, p)?.skewness())
}

/// 负二项取值下界（0）。
#[duck_scalar_function(
    description = "Negative-binomial minimum of the support (0)",
    example = "SELECT sr_negative_binomial_min(2.0, 0.5)"
)]
fn sr_negative_binomial_min(r: f64, p: f64) -> DuckOptionResult<u64> {
    Ok(Some(negative_binomial("sr_negative_binomial_min", r, p)?.min()))
}

/// 负二项取值上界（u64::MAX，以 1.8e19 呈现）。
#[duck_scalar_function(
    description = "Negative-binomial maximum of the support (u64::MAX, shown as 1.8e19)",
    example = "SELECT sr_negative_binomial_max(2.0, 0.5)"
)]
fn sr_negative_binomial_max(r: f64, p: f64) -> DuckOptionResult<u64> {
    Ok(Some(negative_binomial("sr_negative_binomial_max", r, p)?.max()))
}

/// 负二项众数（statrs 返回实数 Option<f64>）。
#[duck_scalar_function(
    description = "Negative-binomial mode (statrs returns a real-valued Option<f64>)",
    example = "SELECT sr_negative_binomial_mode(2.0, 0.5)"
)]
fn sr_negative_binomial_mode(r: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(negative_binomial("sr_negative_binomial_mode", r, p)?.mode())
}

// ---------------------------------------------------------------------------
// Poisson（statrs::distribution::Poisson）
// ---------------------------------------------------------------------------

/// 泊松均值。
#[duck_scalar_function(
    description = "Poisson mean",
    example = "SELECT sr_poisson_mean(3.0)"
)]
fn sr_poisson_mean(lambda: f64) -> DuckOptionResult<f64> {
    Ok(poisson("sr_poisson_mean", lambda)?.mean())
}

/// 泊松方差。
#[duck_scalar_function(
    description = "Poisson variance",
    example = "SELECT sr_poisson_variance(3.0)"
)]
fn sr_poisson_variance(lambda: f64) -> DuckOptionResult<f64> {
    Ok(poisson("sr_poisson_variance", lambda)?.variance())
}

/// 泊松标准差。
#[duck_scalar_function(
    description = "Poisson standard deviation",
    example = "SELECT sr_poisson_std_dev(3.0)"
)]
fn sr_poisson_std_dev(lambda: f64) -> DuckOptionResult<f64> {
    Ok(poisson("sr_poisson_std_dev", lambda)?.std_dev())
}

/// 泊松熵。
#[duck_scalar_function(
    description = "Poisson entropy",
    example = "SELECT sr_poisson_entropy(3.0)"
)]
fn sr_poisson_entropy(lambda: f64) -> DuckOptionResult<f64> {
    Ok(poisson("sr_poisson_entropy", lambda)?.entropy())
}

/// 泊松偏度。
#[duck_scalar_function(
    description = "Poisson skewness",
    example = "SELECT sr_poisson_skewness(3.0)"
)]
fn sr_poisson_skewness(lambda: f64) -> DuckOptionResult<f64> {
    Ok(poisson("sr_poisson_skewness", lambda)?.skewness())
}

/// 泊松取值下界（0）。
#[duck_scalar_function(
    description = "Poisson minimum of the support (0)",
    example = "SELECT sr_poisson_min(3.0)"
)]
fn sr_poisson_min(lambda: f64) -> DuckOptionResult<u64> {
    Ok(Some(poisson("sr_poisson_min", lambda)?.min()))
}

/// 泊松取值上界（u64::MAX，以 1.8e19 呈现）。
#[duck_scalar_function(
    description = "Poisson maximum of the support (u64::MAX, shown as 1.8e19)",
    example = "SELECT sr_poisson_max(3.0)"
)]
fn sr_poisson_max(lambda: f64) -> DuckOptionResult<u64> {
    Ok(Some(poisson("sr_poisson_max", lambda)?.max()))
}

/// 泊松中位数。
#[duck_scalar_function(
    description = "Poisson median",
    example = "SELECT sr_poisson_median(3.0)"
)]
fn sr_poisson_median(lambda: f64) -> DuckOptionResult<f64> {
    Ok(Some(poisson("sr_poisson_median", lambda)?.median()))
}

/// 泊松众数。
#[duck_scalar_function(
    description = "Poisson mode",
    example = "SELECT sr_poisson_mode(3.0)"
)]
fn sr_poisson_mode(lambda: f64) -> DuckOptionResult<u64> {
    Ok(poisson("sr_poisson_mode", lambda)?.mode())
}
