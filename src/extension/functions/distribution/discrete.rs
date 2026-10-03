// ============================================================================
// 离散分布（statrs::distribution 的 bernoulli / binomial / discrete_uniform /
// geometric / hypergeometric / negative_binomial / poisson）
//
// 方法集：`sr_<分布>_pmf / ln_pmf / cdf / sf / quantile`（statrs 的 Discrete trait 用
// pmf 一词）。x 与整数参数（二项的 n、超几何的三个计数）对外都是 DOUBLE 字面量，
// 进 statrs 前过 `as_u64`/`as_i64` 的整数校验；quantile 的返回（statrs 侧是 u64/i64）
// 以 DOUBLE 呈现。越界概率与非法参数报查询错误，statrs 算不出的（NAN）折成 NULL。
//
// Discrete distributions; pmf / ln_pmf / cdf / sf / quantile each. Integer parameters
// (x, binomial n, the hypergeometric counts) are validated whole-number DOUBLEs; the
// quantile result comes back as DOUBLE too.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{
    Bernoulli, Binomial, Discrete, DiscreteCDF, DiscreteUniform, Geometric, Hypergeometric,
    NegativeBinomial, Poisson,
};

use super::check_probability;
use crate::extension::functions::{as_i64, as_u64};

// ---------------------------------------------------------------------------
// Bernoulli（statrs::distribution::Bernoulli）
// ---------------------------------------------------------------------------

fn bernoulli(name: &str, p: f64) -> Result<Bernoulli, ExtensionError> {
    Bernoulli::new(p).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 伯努利概率质量 P(X = x)，x ∈ {0, 1}。
#[duck_scalar_function(
    description = "Bernoulli probability mass P(X = x) for x in {0, 1}, given success probability p",
    example = "SELECT sr_bernoulli_pmf(1.0, 0.7)"
)]
fn sr_bernoulli_pmf(x: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_pmf", p)?.pmf(as_u64("sr_bernoulli_pmf", x)?)))
}

/// 伯努利对数概率质量。
#[duck_scalar_function(
    description = "Bernoulli log probability mass at x",
    example = "SELECT sr_bernoulli_ln_pmf(1.0, 0.7)"
)]
fn sr_bernoulli_ln_pmf(x: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_ln_pmf", p)?.ln_pmf(as_u64("sr_bernoulli_ln_pmf", x)?)))
}

/// 伯努利累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Bernoulli cumulative distribution function P(X <= x)",
    example = "SELECT sr_bernoulli_cdf(1.0, 0.7)"
)]
fn sr_bernoulli_cdf(x: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_cdf", p)?.cdf(as_u64("sr_bernoulli_cdf", x)?)))
}

/// 伯努利生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Bernoulli survival function P(X > x)",
    example = "SELECT sr_bernoulli_sf(0.0, 0.7)"
)]
fn sr_bernoulli_sf(x: f64, p: f64) -> DuckOptionResult<f64> {
    Ok(Some(bernoulli("sr_bernoulli_sf", p)?.sf(as_u64("sr_bernoulli_sf", x)?)))
}

/// 伯努利分位数函数（返回 0 或 1，以 DOUBLE 呈现）。
#[duck_scalar_function(
    description = "Bernoulli quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_bernoulli_quantile(0.5, 0.7)"
)]
fn sr_bernoulli_quantile(prob: f64, p: f64) -> DuckOptionResult<f64> {
    check_probability("sr_bernoulli_quantile", prob)?;
    Ok(Some(bernoulli("sr_bernoulli_quantile", p)?.inverse_cdf(prob) as f64))
}

// ---------------------------------------------------------------------------
// Binomial（statrs::distribution::Binomial）
// ---------------------------------------------------------------------------

fn binomial(name: &str, p: f64, n: f64) -> Result<Binomial, ExtensionError> {
    Binomial::new(p, as_u64(name, n)?).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 二项概率质量 P(X = x)（成功概率 p、试验次数 n）。
#[duck_scalar_function(
    description = "Binomial probability mass P(X = x), given success probability p and number of trials n",
    example = "SELECT sr_binomial_pmf(3.0, 0.5, 10.0)"
)]
fn sr_binomial_pmf(x: f64, p: f64, n: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_binomial_pmf", x)?;
    Ok(Some(binomial("sr_binomial_pmf", p, n)?.pmf(x)))
}

/// 二项对数概率质量。
#[duck_scalar_function(
    description = "Binomial log probability mass at x",
    example = "SELECT sr_binomial_ln_pmf(3.0, 0.5, 10.0)"
)]
fn sr_binomial_ln_pmf(x: f64, p: f64, n: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_binomial_ln_pmf", x)?;
    Ok(Some(binomial("sr_binomial_ln_pmf", p, n)?.ln_pmf(x)))
}

/// 二项累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Binomial cumulative distribution function P(X <= x)",
    example = "SELECT sr_binomial_cdf(3.0, 0.5, 10.0)"
)]
fn sr_binomial_cdf(x: f64, p: f64, n: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_binomial_cdf", x)?;
    Ok(Some(binomial("sr_binomial_cdf", p, n)?.cdf(x)))
}

/// 二项生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Binomial survival function P(X > x)",
    example = "SELECT sr_binomial_sf(3.0, 0.5, 10.0)"
)]
fn sr_binomial_sf(x: f64, p: f64, n: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_binomial_sf", x)?;
    Ok(Some(binomial("sr_binomial_sf", p, n)?.sf(x)))
}

/// 二项分位数函数。
#[duck_scalar_function(
    description = "Binomial quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_binomial_quantile(0.5, 0.5, 10.0)"
)]
fn sr_binomial_quantile(prob: f64, p: f64, n: f64) -> DuckOptionResult<f64> {
    check_probability("sr_binomial_quantile", prob)?;
    Ok(Some(binomial("sr_binomial_quantile", p, n)?.inverse_cdf(prob) as f64))
}

// ---------------------------------------------------------------------------
// DiscreteUniform（statrs::distribution::DiscreteUniform，边界为 i64）
// ---------------------------------------------------------------------------

fn discrete_uniform(name: &str, min: f64, max: f64) -> Result<DiscreteUniform, ExtensionError> {
    DiscreteUniform::new(as_i64(name, min)?, as_i64(name, max)?)
        .map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 离散均匀概率质量 P(X = x)（整数边界 [min, max]）。
#[duck_scalar_function(
    description = "Discrete uniform probability mass P(X = x) on the integer range [min, max]",
    example = "SELECT sr_discrete_uniform_pmf(2.0, 1.0, 6.0)"
)]
fn sr_discrete_uniform_pmf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    let x = as_i64("sr_discrete_uniform_pmf", x)?;
    Ok(Some(discrete_uniform("sr_discrete_uniform_pmf", min, max)?.pmf(x)))
}

/// 离散均匀对数概率质量。
#[duck_scalar_function(
    description = "Discrete uniform log probability mass at x",
    example = "SELECT sr_discrete_uniform_ln_pmf(2.0, 1.0, 6.0)"
)]
fn sr_discrete_uniform_ln_pmf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    let x = as_i64("sr_discrete_uniform_ln_pmf", x)?;
    Ok(Some(discrete_uniform("sr_discrete_uniform_ln_pmf", min, max)?.ln_pmf(x)))
}

/// 离散均匀累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Discrete uniform cumulative distribution function P(X <= x)",
    example = "SELECT sr_discrete_uniform_cdf(3.5, 1.0, 6.0)"
)]
fn sr_discrete_uniform_cdf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    let x = as_i64("sr_discrete_uniform_cdf", x)?;
    Ok(Some(discrete_uniform("sr_discrete_uniform_cdf", min, max)?.cdf(x)))
}

/// 离散均匀生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Discrete uniform survival function P(X > x)",
    example = "SELECT sr_discrete_uniform_sf(3.0, 1.0, 6.0)"
)]
fn sr_discrete_uniform_sf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    let x = as_i64("sr_discrete_uniform_sf", x)?;
    Ok(Some(discrete_uniform("sr_discrete_uniform_sf", min, max)?.sf(x)))
}

/// 离散均匀分位数函数。
#[duck_scalar_function(
    description = "Discrete uniform quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_discrete_uniform_quantile(0.5, 1.0, 6.0)"
)]
fn sr_discrete_uniform_quantile(prob: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    check_probability("sr_discrete_uniform_quantile", prob)?;
    Ok(Some(discrete_uniform("sr_discrete_uniform_quantile", min, max)?.inverse_cdf(prob) as f64))
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
    example = "SELECT sr_geometric_pmf(2.0, 0.5)"
)]
fn sr_geometric_pmf(x: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_geometric_pmf", x)?;
    Ok(Some(geometric("sr_geometric_pmf", p)?.pmf(x)))
}

/// 几何对数概率质量。
#[duck_scalar_function(
    description = "Geometric log probability mass at x",
    example = "SELECT sr_geometric_ln_pmf(2.0, 0.5)"
)]
fn sr_geometric_ln_pmf(x: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_geometric_ln_pmf", x)?;
    Ok(Some(geometric("sr_geometric_ln_pmf", p)?.ln_pmf(x)))
}

/// 几何累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Geometric cumulative distribution function P(X <= x)",
    example = "SELECT sr_geometric_cdf(2.0, 0.5)"
)]
fn sr_geometric_cdf(x: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_geometric_cdf", x)?;
    Ok(Some(geometric("sr_geometric_cdf", p)?.cdf(x)))
}

/// 几何生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Geometric survival function P(X > x)",
    example = "SELECT sr_geometric_sf(2.0, 0.5)"
)]
fn sr_geometric_sf(x: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_geometric_sf", x)?;
    Ok(Some(geometric("sr_geometric_sf", p)?.sf(x)))
}

/// 几何分位数函数。
#[duck_scalar_function(
    description = "Geometric quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_geometric_quantile(0.5, 0.5)"
)]
fn sr_geometric_quantile(prob: f64, p: f64) -> DuckOptionResult<f64> {
    check_probability("sr_geometric_quantile", prob)?;
    Ok(Some(geometric("sr_geometric_quantile", p)?.inverse_cdf(prob) as f64))
}

// ---------------------------------------------------------------------------
// Hypergeometric（statrs::distribution::Hypergeometric，三个计数都是 u64）
// ---------------------------------------------------------------------------

fn hypergeometric(
    name: &str,
    population: f64,
    successes: f64,
    draws: f64,
) -> Result<Hypergeometric, ExtensionError> {
    Hypergeometric::new(
        as_u64(name, population)?,
        as_u64(name, successes)?,
        as_u64(name, draws)?,
    )
    .map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 超几何概率质量 P(X = x)：不放回抽样命中的成功数。
#[duck_scalar_function(
    description = "Hypergeometric probability mass P(X = x): successes drawn without replacement (population, successes, draws as whole-number DOUBLEs)",
    example = "SELECT sr_hypergeometric_pmf(2.0, 10.0, 5.0, 4.0)"
)]
fn sr_hypergeometric_pmf(x: f64, population: f64, successes: f64, draws: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_hypergeometric_pmf", x)?;
    let dist = hypergeometric("sr_hypergeometric_pmf", population, successes, draws)?;
    Ok(Some(dist.pmf(x)))
}

/// 超几何对数概率质量。
#[duck_scalar_function(
    description = "Hypergeometric log probability mass at x",
    example = "SELECT sr_hypergeometric_ln_pmf(2.0, 10.0, 5.0, 4.0)"
)]
fn sr_hypergeometric_ln_pmf(x: f64, population: f64, successes: f64, draws: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_hypergeometric_ln_pmf", x)?;
    let dist = hypergeometric("sr_hypergeometric_ln_pmf", population, successes, draws)?;
    Ok(Some(dist.ln_pmf(x)))
}

/// 超几何累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Hypergeometric cumulative distribution function P(X <= x)",
    example = "SELECT sr_hypergeometric_cdf(2.0, 10.0, 5.0, 4.0)"
)]
fn sr_hypergeometric_cdf(x: f64, population: f64, successes: f64, draws: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_hypergeometric_cdf", x)?;
    let dist = hypergeometric("sr_hypergeometric_cdf", population, successes, draws)?;
    Ok(Some(dist.cdf(x)))
}

/// 超几何生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Hypergeometric survival function P(X > x)",
    example = "SELECT sr_hypergeometric_sf(2.0, 10.0, 5.0, 4.0)"
)]
fn sr_hypergeometric_sf(x: f64, population: f64, successes: f64, draws: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_hypergeometric_sf", x)?;
    let dist = hypergeometric("sr_hypergeometric_sf", population, successes, draws)?;
    Ok(Some(dist.sf(x)))
}

/// 超几何分位数函数。
#[duck_scalar_function(
    description = "Hypergeometric quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_hypergeometric_quantile(0.5, 10.0, 5.0, 4.0)"
)]
fn sr_hypergeometric_quantile(prob: f64, population: f64, successes: f64, draws: f64) -> DuckOptionResult<f64> {
    check_probability("sr_hypergeometric_quantile", prob)?;
    let dist = hypergeometric("sr_hypergeometric_quantile", population, successes, draws)?;
    Ok(Some(dist.inverse_cdf(prob) as f64))
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
    example = "SELECT sr_negative_binomial_pmf(3.0, 2.0, 0.5)"
)]
fn sr_negative_binomial_pmf(x: f64, r: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_negative_binomial_pmf", x)?;
    Ok(Some(negative_binomial("sr_negative_binomial_pmf", r, p)?.pmf(x)))
}

/// 负二项对数概率质量。
#[duck_scalar_function(
    description = "Negative-binomial log probability mass at x",
    example = "SELECT sr_negative_binomial_ln_pmf(3.0, 2.0, 0.5)"
)]
fn sr_negative_binomial_ln_pmf(x: f64, r: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_negative_binomial_ln_pmf", x)?;
    Ok(Some(negative_binomial("sr_negative_binomial_ln_pmf", r, p)?.ln_pmf(x)))
}

/// 负二项累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Negative-binomial cumulative distribution function P(X <= x)",
    example = "SELECT sr_negative_binomial_cdf(3.0, 2.0, 0.5)"
)]
fn sr_negative_binomial_cdf(x: f64, r: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_negative_binomial_cdf", x)?;
    Ok(Some(negative_binomial("sr_negative_binomial_cdf", r, p)?.cdf(x)))
}

/// 负二项生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Negative-binomial survival function P(X > x)",
    example = "SELECT sr_negative_binomial_sf(3.0, 2.0, 0.5)"
)]
fn sr_negative_binomial_sf(x: f64, r: f64, p: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_negative_binomial_sf", x)?;
    Ok(Some(negative_binomial("sr_negative_binomial_sf", r, p)?.sf(x)))
}

/// 负二项分位数函数。
#[duck_scalar_function(
    description = "Negative-binomial quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_negative_binomial_quantile(0.5, 2.0, 0.5)"
)]
fn sr_negative_binomial_quantile(prob: f64, r: f64, p: f64) -> DuckOptionResult<f64> {
    check_probability("sr_negative_binomial_quantile", prob)?;
    Ok(Some(negative_binomial("sr_negative_binomial_quantile", r, p)?.inverse_cdf(prob) as f64))
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
    example = "SELECT sr_poisson_pmf(2.0, 3.0)"
)]
fn sr_poisson_pmf(x: f64, lambda: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_poisson_pmf", x)?;
    Ok(Some(poisson("sr_poisson_pmf", lambda)?.pmf(x)))
}

/// 泊松对数概率质量。
#[duck_scalar_function(
    description = "Poisson log probability mass at x",
    example = "SELECT sr_poisson_ln_pmf(2.0, 3.0)"
)]
fn sr_poisson_ln_pmf(x: f64, lambda: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_poisson_ln_pmf", x)?;
    Ok(Some(poisson("sr_poisson_ln_pmf", lambda)?.ln_pmf(x)))
}

/// 泊松累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Poisson cumulative distribution function P(X <= x)",
    example = "SELECT sr_poisson_cdf(2.0, 3.0)"
)]
fn sr_poisson_cdf(x: f64, lambda: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_poisson_cdf", x)?;
    Ok(Some(poisson("sr_poisson_cdf", lambda)?.cdf(x)))
}

/// 泊松生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Poisson survival function P(X > x)",
    example = "SELECT sr_poisson_sf(2.0, 3.0)"
)]
fn sr_poisson_sf(x: f64, lambda: f64) -> DuckOptionResult<f64> {
    let x = as_u64("sr_poisson_sf", x)?;
    Ok(Some(poisson("sr_poisson_sf", lambda)?.sf(x)))
}

/// 泊松分位数函数。
#[duck_scalar_function(
    description = "Poisson quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_poisson_quantile(0.5, 3.0)"
)]
fn sr_poisson_quantile(prob: f64, lambda: f64) -> DuckOptionResult<f64> {
    check_probability("sr_poisson_quantile", prob)?;
    Ok(Some(poisson("sr_poisson_quantile", lambda)?.inverse_cdf(prob) as f64))
}
