// ============================================================================
// Categorical（statrs::distribution::Categorical）：概率向量定义的有限类别分布
//
// probs 是 LIST(DOUBLE) 参数（statrs 的 new(&[f64]) 会归一化，各分量之和必须 > 0
// 且无 NaN —— 否则构造错误报查询错误）。取值 x 是 0..probs.len()-1 的整数槽位。
//
// Categorical: a finite categorical distribution defined by a probability vector
// (LIST(DOUBLE)); x indexes the categories from 0.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{Categorical, Discrete, DiscreteCDF};

use super::check_probability;
use crate::extension::functions::as_u64;

fn categorical(name: &str, probs: Vec<f64>) -> Result<Categorical, ExtensionError> {
    Categorical::new(&probs).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 类别分布的概率质量 P(X = x)（probs 为 LIST(DOUBLE)，会被归一化）。
#[duck_scalar_function(
    description = "Categorical probability mass P(X = x) for a distribution given by an unnormalised probability LIST (normalised by statrs)",
    example = "SELECT sr_categorical_pmf(1.0, [1.0, 2.0, 1.0])"
)]
fn sr_categorical_pmf(x: f64, probs: Vec<f64>) -> DuckOptionResult<f64> {
    let x = as_u64("sr_categorical_pmf", x)?;
    Ok(Some(categorical("sr_categorical_pmf", probs)?.pmf(x)))
}

/// 类别分布的对数概率质量。
#[duck_scalar_function(
    description = "Categorical log probability mass at x",
    example = "SELECT sr_categorical_ln_pmf(1.0, [1.0, 2.0, 1.0])"
)]
fn sr_categorical_ln_pmf(x: f64, probs: Vec<f64>) -> DuckOptionResult<f64> {
    let x = as_u64("sr_categorical_ln_pmf", x)?;
    Ok(Some(categorical("sr_categorical_ln_pmf", probs)?.ln_pmf(x)))
}

/// 类别分布累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Categorical cumulative distribution function P(X <= x)",
    example = "SELECT sr_categorical_cdf(1.0, [1.0, 2.0, 1.0])"
)]
fn sr_categorical_cdf(x: f64, probs: Vec<f64>) -> DuckOptionResult<f64> {
    let x = as_u64("sr_categorical_cdf", x)?;
    Ok(Some(categorical("sr_categorical_cdf", probs)?.cdf(x)))
}

/// 类别分布生存函数 P(X > x)。
#[duck_scalar_function(
    description = "Categorical survival function P(X > x)",
    example = "SELECT sr_categorical_sf(1.0, [1.0, 2.0, 1.0])"
)]
fn sr_categorical_sf(x: f64, probs: Vec<f64>) -> DuckOptionResult<f64> {
    let x = as_u64("sr_categorical_sf", x)?;
    Ok(Some(categorical("sr_categorical_sf", probs)?.sf(x)))
}

/// 类别分布分位数函数。
#[duck_scalar_function(
    description = "Categorical quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_categorical_quantile(0.5, [1.0, 2.0, 1.0])"
)]
fn sr_categorical_quantile(prob: f64, probs: Vec<f64>) -> DuckOptionResult<f64> {
    check_probability("sr_categorical_quantile", prob)?;
    Ok(Some(categorical("sr_categorical_quantile", probs)?.inverse_cdf(prob) as f64))
}
