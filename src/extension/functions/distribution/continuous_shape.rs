// ============================================================================
// 连续分布 · 形状/速率族（statrs::distribution 的 gamma / erlang / chi /
// chi_squared / exponential(Exp) / inverse_gamma / pareto / weibull / beta /
// fisher_snedecor）
//
// 方法集与位置-尺度族相同：pdf / ln_pdf / cdf / sf / quantile。Erlang 的 shape、
// Chi 的自由度在 statrs 里是 u64：SQL 侧写整数值的DOUBLE 字面量，转换与校验走
// `as_u64`（3.5 报错，不四舍五入）。
//
// Shape/rate continuous distributions; same five methods each. Erlang's shape and Chi's
// freedom are u64 in statrs — validated whole-number DOUBLEs on the SQL side.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{
    Beta, Chi, ChiSquared, Continuous, ContinuousCDF, Erlang, Exp, FisherSnedecor, Gamma,
    InverseGamma, Pareto, Weibull,
};

use super::check_probability;
use crate::extension::functions::as_u64;
// 分布的矩与域：Distribution 提供 mean/variance/std_dev/entropy/skewness，
// Min/Max/Median/Mode 提供支撑集端点与典型值。刻意不导入 DiscreteDistribution ——
// 它与 Distribution 的同名方法（mean 等）会造成方法解析歧义。
use statrs::statistics::{Distribution, Max, Median, Min, Mode};

// ---------------------------------------------------------------------------
// Gamma（statrs::distribution::Gamma）
// ---------------------------------------------------------------------------

fn gamma(name: &str, shape: f64, rate: f64) -> Result<Gamma, ExtensionError> {
    Gamma::new(shape, rate).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Gamma 概率密度（shape / rate 参数化）。
#[duck_scalar_function(
    description = "Gamma probability density at x (shape/rate parameterization)",
    example = "SELECT sr_gamma_pdf(1.0, 2.0, 2.0)"
)]
fn sr_gamma_pdf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(gamma("sr_gamma_pdf", shape, rate)?.pdf(x)))
}

/// Gamma 对数密度。
#[duck_scalar_function(
    description = "Gamma log-density at x",
    example = "SELECT sr_gamma_ln_pdf(1.0, 2.0, 2.0)"
)]
fn sr_gamma_ln_pdf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(gamma("sr_gamma_ln_pdf", shape, rate)?.ln_pdf(x)))
}

/// Gamma 累积分布（即正则化下不完全 Gamma 函数 P(shape, rate·x)）。
#[duck_scalar_function(
    description = "Gamma cumulative distribution function P(X <= x)",
    example = "SELECT sr_gamma_cdf(1.0, 2.0, 2.0)"
)]
fn sr_gamma_cdf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(gamma("sr_gamma_cdf", shape, rate)?.cdf(x)))
}

/// Gamma 生存函数。
#[duck_scalar_function(
    description = "Gamma survival function P(X > x)",
    example = "SELECT sr_gamma_sf(1.0, 2.0, 2.0)"
)]
fn sr_gamma_sf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(gamma("sr_gamma_sf", shape, rate)?.sf(x)))
}

/// Gamma 分位数函数。
#[duck_scalar_function(
    description = "Gamma quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_gamma_quantile(0.5, 2.0, 2.0)"
)]
fn sr_gamma_quantile(p: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    check_probability("sr_gamma_quantile", p)?;
    Ok(Some(gamma("sr_gamma_quantile", shape, rate)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Erlang（statrs::distribution::Erlang，shape 为整数）
// ---------------------------------------------------------------------------

/// Erlang 的 shape 是 u64：先过 `as_u64` 的整数校验再构造。
fn erlang(name: &str, shape: f64, rate: f64) -> Result<Erlang, ExtensionError> {
    Erlang::new(as_u64(name, shape)?, rate).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Erlang 概率密度（整数 shape / rate）。
#[duck_scalar_function(
    description = "Erlang probability density at x (whole-number shape, rate)",
    example = "SELECT sr_erlang_pdf(1.0, 2.0, 2.0)"
)]
fn sr_erlang_pdf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(erlang("sr_erlang_pdf", shape, rate)?.pdf(x)))
}

/// Erlang 对数密度。
#[duck_scalar_function(
    description = "Erlang log-density at x",
    example = "SELECT sr_erlang_ln_pdf(1.0, 2.0, 2.0)"
)]
fn sr_erlang_ln_pdf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(erlang("sr_erlang_ln_pdf", shape, rate)?.ln_pdf(x)))
}

/// Erlang 累积分布。
#[duck_scalar_function(
    description = "Erlang cumulative distribution function P(X <= x)",
    example = "SELECT sr_erlang_cdf(1.0, 2.0, 2.0)"
)]
fn sr_erlang_cdf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(erlang("sr_erlang_cdf", shape, rate)?.cdf(x)))
}

/// Erlang 生存函数。
#[duck_scalar_function(
    description = "Erlang survival function P(X > x)",
    example = "SELECT sr_erlang_sf(1.0, 2.0, 2.0)"
)]
fn sr_erlang_sf(x: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(erlang("sr_erlang_sf", shape, rate)?.sf(x)))
}

/// Erlang 分位数函数。
#[duck_scalar_function(
    description = "Erlang quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_erlang_quantile(0.5, 2.0, 2.0)"
)]
fn sr_erlang_quantile(p: f64, shape: f64, rate: f64) -> DuckOptionResult<f64> {
    check_probability("sr_erlang_quantile", p)?;
    Ok(Some(erlang("sr_erlang_quantile", shape, rate)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Chi（statrs::distribution::Chi，自由度为整数）
// ---------------------------------------------------------------------------

fn chi(name: &str, freedom: f64) -> Result<Chi, ExtensionError> {
    Chi::new(as_u64(name, freedom)?).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Chi 分布概率密度 —— statrs 的 Chi 是 **√(χ²) 的分布**（自由度为整数的根卡方分布），
/// 与下面的 ChiSquared 是两个不同的分布，别弄混。
#[duck_scalar_function(
    description = "Chi distribution probability density at x (the sqrt-chi-squared distribution), whole-number freedom",
    example = "SELECT sr_chi_pdf(1.0, 2.0)"
)]
fn sr_chi_pdf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi("sr_chi_pdf", freedom)?.pdf(x)))
}

/// Chi 分布对数密度。
#[duck_scalar_function(
    description = "Chi distribution log-density at x",
    example = "SELECT sr_chi_ln_pdf(1.0, 2.0)"
)]
fn sr_chi_ln_pdf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi("sr_chi_ln_pdf", freedom)?.ln_pdf(x)))
}

/// Chi 分布累积分布。
#[duck_scalar_function(
    description = "Chi distribution cumulative distribution function P(X <= x)",
    example = "SELECT sr_chi_cdf(1.0, 2.0)"
)]
fn sr_chi_cdf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi("sr_chi_cdf", freedom)?.cdf(x)))
}

/// Chi 分布生存函数。
#[duck_scalar_function(
    description = "Chi distribution survival function P(X > x)",
    example = "SELECT sr_chi_sf(1.0, 2.0)"
)]
fn sr_chi_sf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi("sr_chi_sf", freedom)?.sf(x)))
}

/// Chi 分布分位数函数。
#[duck_scalar_function(
    description = "Chi distribution quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_chi_quantile(0.5, 2.0)"
)]
fn sr_chi_quantile(p: f64, freedom: f64) -> DuckOptionResult<f64> {
    check_probability("sr_chi_quantile", p)?;
    Ok(Some(chi("sr_chi_quantile", freedom)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// ChiSquared（statrs::distribution::ChiSquared）
// ---------------------------------------------------------------------------

fn chi_squared(name: &str, freedom: f64) -> Result<ChiSquared, ExtensionError> {
    ChiSquared::new(freedom).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 卡方概率密度（自由度可以是小数）。
#[duck_scalar_function(
    description = "Chi-squared probability density at x, given degrees of freedom",
    example = "SELECT sr_chi_squared_pdf(1.0, 2.0)"
)]
fn sr_chi_squared_pdf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_pdf", freedom)?.pdf(x)))
}

/// 卡方对数密度。
#[duck_scalar_function(
    description = "Chi-squared log-density at x",
    example = "SELECT sr_chi_squared_ln_pdf(1.0, 2.0)"
)]
fn sr_chi_squared_ln_pdf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_ln_pdf", freedom)?.ln_pdf(x)))
}

/// 卡方累积分布。
#[duck_scalar_function(
    description = "Chi-squared cumulative distribution function P(X <= x)",
    example = "SELECT sr_chi_squared_cdf(1.0, 2.0)"
)]
fn sr_chi_squared_cdf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_cdf", freedom)?.cdf(x)))
}

/// 卡方生存函数。
#[duck_scalar_function(
    description = "Chi-squared survival function P(X > x)",
    example = "SELECT sr_chi_squared_sf(1.0, 2.0)"
)]
fn sr_chi_squared_sf(x: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_sf", freedom)?.sf(x)))
}

/// 卡方分位数函数。
#[duck_scalar_function(
    description = "Chi-squared quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_chi_squared_quantile(0.95, 2.0)"
)]
fn sr_chi_squared_quantile(p: f64, freedom: f64) -> DuckOptionResult<f64> {
    check_probability("sr_chi_squared_quantile", p)?;
    Ok(Some(chi_squared("sr_chi_squared_quantile", freedom)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Exp（statrs::distribution::Exp，指数分布）
// ---------------------------------------------------------------------------

fn exp(name: &str, rate: f64) -> Result<Exp, ExtensionError> {
    Exp::new(rate).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 指数分布概率密度（rate 参数化）。
#[duck_scalar_function(
    description = "Exponential probability density at x, given the rate",
    example = "SELECT sr_exp_pdf(1.0, 2.0)"
)]
fn sr_exp_pdf(x: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_pdf", rate)?.pdf(x)))
}

/// 指数分布对数密度。
#[duck_scalar_function(
    description = "Exponential log-density at x",
    example = "SELECT sr_exp_ln_pdf(1.0, 2.0)"
)]
fn sr_exp_ln_pdf(x: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_ln_pdf", rate)?.ln_pdf(x)))
}

/// 指数分布累积分布。
#[duck_scalar_function(
    description = "Exponential cumulative distribution function P(X <= x)",
    example = "SELECT sr_exp_cdf(1.0, 2.0)"
)]
fn sr_exp_cdf(x: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_cdf", rate)?.cdf(x)))
}

/// 指数分布生存函数。
#[duck_scalar_function(
    description = "Exponential survival function P(X > x)",
    example = "SELECT sr_exp_sf(1.0, 2.0)"
)]
fn sr_exp_sf(x: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_sf", rate)?.sf(x)))
}

/// 指数分布分位数函数。
#[duck_scalar_function(
    description = "Exponential quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_exp_quantile(0.5, 2.0)"
)]
fn sr_exp_quantile(p: f64, rate: f64) -> DuckOptionResult<f64> {
    check_probability("sr_exp_quantile", p)?;
    Ok(Some(exp("sr_exp_quantile", rate)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// InverseGamma（statrs::distribution::InverseGamma）
// ---------------------------------------------------------------------------

fn inverse_gamma(name: &str, shape: f64, scale: f64) -> Result<InverseGamma, ExtensionError> {
    InverseGamma::new(shape, scale).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 逆 Gamma 概率密度（statrs 用 shape/scale）。
#[duck_scalar_function(
    description = "Inverse-gamma probability density at x, given shape and scale",
    example = "SELECT sr_inverse_gamma_pdf(1.0, 2.0, 2.0)"
)]
fn sr_inverse_gamma_pdf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(inverse_gamma("sr_inverse_gamma_pdf", shape, scale)?.pdf(x)))
}

/// 逆 Gamma 对数密度。
#[duck_scalar_function(
    description = "Inverse-gamma log-density at x",
    example = "SELECT sr_inverse_gamma_ln_pdf(1.0, 2.0, 2.0)"
)]
fn sr_inverse_gamma_ln_pdf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(inverse_gamma("sr_inverse_gamma_ln_pdf", shape, scale)?.ln_pdf(x)))
}

/// 逆 Gamma 累积分布。
#[duck_scalar_function(
    description = "Inverse-gamma cumulative distribution function P(X <= x)",
    example = "SELECT sr_inverse_gamma_cdf(1.0, 2.0, 2.0)"
)]
fn sr_inverse_gamma_cdf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(inverse_gamma("sr_inverse_gamma_cdf", shape, scale)?.cdf(x)))
}

/// 逆 Gamma 生存函数。
#[duck_scalar_function(
    description = "Inverse-gamma survival function P(X > x)",
    example = "SELECT sr_inverse_gamma_sf(1.0, 2.0, 2.0)"
)]
fn sr_inverse_gamma_sf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(inverse_gamma("sr_inverse_gamma_sf", shape, scale)?.sf(x)))
}

/// 逆 Gamma 分位数函数。
#[duck_scalar_function(
    description = "Inverse-gamma quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_inverse_gamma_quantile(0.5, 2.0, 2.0)"
)]
fn sr_inverse_gamma_quantile(p: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    check_probability("sr_inverse_gamma_quantile", p)?;
    Ok(Some(inverse_gamma("sr_inverse_gamma_quantile", shape, scale)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Pareto（statrs::distribution::Pareto）
// ---------------------------------------------------------------------------

fn pareto(name: &str, scale: f64, shape: f64) -> Result<Pareto, ExtensionError> {
    Pareto::new(scale, shape).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Pareto（I 型）概率密度（scale = x_m，shape = α）。
#[duck_scalar_function(
    description = "Pareto (type-I) probability density at x, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_pdf(1.0, 1.0, 2.0)"
)]
fn sr_pareto_pdf(x: f64, scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_pdf", scale, shape)?.pdf(x)))
}

/// Pareto 对数密度。
#[duck_scalar_function(
    description = "Pareto log-density at x",
    example = "SELECT sr_pareto_ln_pdf(1.0, 1.0, 2.0)"
)]
fn sr_pareto_ln_pdf(x: f64, scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_ln_pdf", scale, shape)?.ln_pdf(x)))
}

/// Pareto 累积分布。
#[duck_scalar_function(
    description = "Pareto cumulative distribution function P(X <= x)",
    example = "SELECT sr_pareto_cdf(2.0, 1.0, 2.0)"
)]
fn sr_pareto_cdf(x: f64, scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_cdf", scale, shape)?.cdf(x)))
}

/// Pareto 生存函数。
#[duck_scalar_function(
    description = "Pareto survival function P(X > x)",
    example = "SELECT sr_pareto_sf(2.0, 1.0, 2.0)"
)]
fn sr_pareto_sf(x: f64, scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_sf", scale, shape)?.sf(x)))
}

/// Pareto 分位数函数。
#[duck_scalar_function(
    description = "Pareto quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_pareto_quantile(0.5, 1.0, 2.0)"
)]
fn sr_pareto_quantile(p: f64, scale: f64, shape: f64) -> DuckOptionResult<f64> {
    check_probability("sr_pareto_quantile", p)?;
    Ok(Some(pareto("sr_pareto_quantile", scale, shape)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Weibull（statrs::distribution::Weibull）
// ---------------------------------------------------------------------------

fn weibull(name: &str, shape: f64, scale: f64) -> Result<Weibull, ExtensionError> {
    Weibull::new(shape, scale).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Weibull 概率密度（shape k / scale λ）。
#[duck_scalar_function(
    description = "Weibull probability density at x, given shape and scale",
    example = "SELECT sr_weibull_pdf(1.0, 1.0, 1.0)"
)]
fn sr_weibull_pdf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_pdf", shape, scale)?.pdf(x)))
}

/// Weibull 对数密度。
#[duck_scalar_function(
    description = "Weibull log-density at x",
    example = "SELECT sr_weibull_ln_pdf(1.0, 1.0, 1.0)"
)]
fn sr_weibull_ln_pdf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_ln_pdf", shape, scale)?.ln_pdf(x)))
}

/// Weibull 累积分布。
#[duck_scalar_function(
    description = "Weibull cumulative distribution function P(X <= x)",
    example = "SELECT sr_weibull_cdf(1.0, 1.0, 1.0)"
)]
fn sr_weibull_cdf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_cdf", shape, scale)?.cdf(x)))
}

/// Weibull 生存函数。
#[duck_scalar_function(
    description = "Weibull survival function P(X > x)",
    example = "SELECT sr_weibull_sf(1.0, 1.0, 1.0)"
)]
fn sr_weibull_sf(x: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_sf", shape, scale)?.sf(x)))
}

/// Weibull 分位数函数。
#[duck_scalar_function(
    description = "Weibull quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_weibull_quantile(0.5, 1.0, 1.0)"
)]
fn sr_weibull_quantile(p: f64, shape: f64, scale: f64) -> DuckOptionResult<f64> {
    check_probability("sr_weibull_quantile", p)?;
    Ok(Some(weibull("sr_weibull_quantile", shape, scale)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Beta（statrs::distribution::Beta）
// ---------------------------------------------------------------------------

fn beta(name: &str, shape_a: f64, shape_b: f64) -> Result<Beta, ExtensionError> {
    Beta::new(shape_a, shape_b).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Beta 概率密度（shape_a / shape_b）。
#[duck_scalar_function(
    description = "Beta probability density at x, given shape_a and shape_b",
    example = "SELECT sr_beta_pdf(0.5, 2.0, 3.0)"
)]
fn sr_beta_pdf(x: f64, shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(Some(beta("sr_beta_pdf", shape_a, shape_b)?.pdf(x)))
}

/// Beta 对数密度。
#[duck_scalar_function(
    description = "Beta log-density at x",
    example = "SELECT sr_beta_ln_pdf(0.5, 2.0, 3.0)"
)]
fn sr_beta_ln_pdf(x: f64, shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(Some(beta("sr_beta_ln_pdf", shape_a, shape_b)?.ln_pdf(x)))
}

/// Beta 累积分布（即正则化不完全 Beta 函数 I(x; a, b)）。
#[duck_scalar_function(
    description = "Beta cumulative distribution function P(X <= x)",
    example = "SELECT sr_beta_cdf(0.5, 2.0, 3.0)"
)]
fn sr_beta_cdf(x: f64, shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(Some(beta("sr_beta_cdf", shape_a, shape_b)?.cdf(x)))
}

/// Beta 生存函数。
#[duck_scalar_function(
    description = "Beta survival function P(X > x)",
    example = "SELECT sr_beta_sf(0.5, 2.0, 3.0)"
)]
fn sr_beta_sf(x: f64, shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(Some(beta("sr_beta_sf", shape_a, shape_b)?.sf(x)))
}

/// Beta 分位数函数。
#[duck_scalar_function(
    description = "Beta quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_beta_quantile(0.5, 2.0, 3.0)"
)]
fn sr_beta_quantile(p: f64, shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    check_probability("sr_beta_quantile", p)?;
    Ok(Some(beta("sr_beta_quantile", shape_a, shape_b)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// FisherSnedecor（statrs::distribution::FisherSnedecor，F 分布）
// ---------------------------------------------------------------------------

fn fisher_snedecor(name: &str, freedom_1: f64, freedom_2: f64) -> Result<FisherSnedecor, ExtensionError> {
    FisherSnedecor::new(freedom_1, freedom_2).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// F 分布（Fisher–Snedecor）概率密度。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) probability density at x, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_pdf(1.0, 2.0, 3.0)"
)]
fn sr_fisher_snedecor_pdf(x: f64, freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_pdf", freedom_1, freedom_2)?.pdf(x)))
}

/// F 分布对数密度。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) log-density at x",
    example = "SELECT sr_fisher_snedecor_ln_pdf(1.0, 2.0, 3.0)"
)]
fn sr_fisher_snedecor_ln_pdf(x: f64, freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_ln_pdf", freedom_1, freedom_2)?.ln_pdf(x)))
}

/// F 分布累积分布。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) cumulative distribution function P(X <= x)",
    example = "SELECT sr_fisher_snedecor_cdf(1.0, 2.0, 3.0)"
)]
fn sr_fisher_snedecor_cdf(x: f64, freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_cdf", freedom_1, freedom_2)?.cdf(x)))
}

/// F 分布生存函数。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) survival function P(X > x)",
    example = "SELECT sr_fisher_snedecor_sf(1.0, 2.0, 3.0)"
)]
fn sr_fisher_snedecor_sf(x: f64, freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_sf", freedom_1, freedom_2)?.sf(x)))
}

/// F 分布分位数函数。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_fisher_snedecor_quantile(0.95, 2.0, 3.0)"
)]
fn sr_fisher_snedecor_quantile(p: f64, freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    check_probability("sr_fisher_snedecor_quantile", p)?;
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_quantile", freedom_1, freedom_2)?.inverse_cdf(p)))
}

// ============================================================================
// 分布的矩 / 域 / 中位数 / 众数（statrs::statistics 的 Distribution / Min / Max /
// Median / Mode）。每个分布一组：mean / variance / std_dev / entropy / skewness，
// 再按 statrs 的实现情况补 min / max / median / mode。均无 x 参数，参数顺序与上文
// 各构造函数（gamma / erlang / chi / …）完全一致。
//
// Distribution moments, support endpoints, median and mode for the shape/rate
// family. No x argument; parameter order matches the constructors above.
// ============================================================================

// ---------------------------------------------------------------------------
// Gamma（shape, rate）
// ---------------------------------------------------------------------------

/// Gamma 均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Gamma mean (shape/rate parameterization)",
    example = "SELECT sr_gamma_mean(2.0, 1.0)"
)]
fn sr_gamma_mean(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(gamma("sr_gamma_mean", shape, rate)?.mean())
}

/// Gamma 方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Gamma variance (shape/rate parameterization)",
    example = "SELECT sr_gamma_variance(2.0, 1.0)"
)]
fn sr_gamma_variance(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(gamma("sr_gamma_variance", shape, rate)?.variance())
}

/// Gamma 标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Gamma standard deviation (shape/rate parameterization)",
    example = "SELECT sr_gamma_std_dev(2.0, 1.0)"
)]
fn sr_gamma_std_dev(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(gamma("sr_gamma_std_dev", shape, rate)?.std_dev())
}

/// Gamma 微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Gamma differential entropy (shape/rate parameterization)",
    example = "SELECT sr_gamma_entropy(2.0, 1.0)"
)]
fn sr_gamma_entropy(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(gamma("sr_gamma_entropy", shape, rate)?.entropy())
}

/// Gamma 偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Gamma skewness (shape/rate parameterization)",
    example = "SELECT sr_gamma_skewness(2.0, 1.0)"
)]
fn sr_gamma_skewness(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(gamma("sr_gamma_skewness", shape, rate)?.skewness())
}

/// Gamma 支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Gamma minimum of the support (0)",
    example = "SELECT sr_gamma_min(2.0, 1.0)"
)]
fn sr_gamma_min(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(gamma("sr_gamma_min", shape, rate)?.min()))
}

/// Gamma 支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Gamma maximum of the support (positive infinity)",
    example = "SELECT sr_gamma_max(2.0, 1.0)"
)]
fn sr_gamma_max(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(gamma("sr_gamma_max", shape, rate)?.max()))
}

/// Gamma 众数，对应 `Mode::mode`（shape < 1 时无众数，返回 NULL）。
#[duck_scalar_function(
    description = "Gamma mode (shape/rate parameterization)",
    example = "SELECT sr_gamma_mode(2.0, 1.0)"
)]
fn sr_gamma_mode(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(gamma("sr_gamma_mode", shape, rate)?.mode())
}

// ---------------------------------------------------------------------------
// Erlang（shape, rate；shape 为整数）
// ---------------------------------------------------------------------------

/// Erlang 均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Erlang mean (whole-number shape, rate)",
    example = "SELECT sr_erlang_mean(2.0, 1.0)"
)]
fn sr_erlang_mean(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(erlang("sr_erlang_mean", shape, rate)?.mean())
}

/// Erlang 方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Erlang variance (whole-number shape, rate)",
    example = "SELECT sr_erlang_variance(2.0, 1.0)"
)]
fn sr_erlang_variance(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(erlang("sr_erlang_variance", shape, rate)?.variance())
}

/// Erlang 标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Erlang standard deviation (whole-number shape, rate)",
    example = "SELECT sr_erlang_std_dev(2.0, 1.0)"
)]
fn sr_erlang_std_dev(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(erlang("sr_erlang_std_dev", shape, rate)?.std_dev())
}

/// Erlang 微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Erlang differential entropy (whole-number shape, rate)",
    example = "SELECT sr_erlang_entropy(2.0, 1.0)"
)]
fn sr_erlang_entropy(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(erlang("sr_erlang_entropy", shape, rate)?.entropy())
}

/// Erlang 偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Erlang skewness (whole-number shape, rate)",
    example = "SELECT sr_erlang_skewness(2.0, 1.0)"
)]
fn sr_erlang_skewness(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(erlang("sr_erlang_skewness", shape, rate)?.skewness())
}

/// Erlang 支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Erlang minimum of the support (0)",
    example = "SELECT sr_erlang_min(2.0, 1.0)"
)]
fn sr_erlang_min(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(erlang("sr_erlang_min", shape, rate)?.min()))
}

/// Erlang 支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Erlang maximum of the support (positive infinity)",
    example = "SELECT sr_erlang_max(2.0, 1.0)"
)]
fn sr_erlang_max(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(erlang("sr_erlang_max", shape, rate)?.max()))
}

/// Erlang 众数，对应 `Mode::mode`。
#[duck_scalar_function(
    description = "Erlang mode (whole-number shape, rate)",
    example = "SELECT sr_erlang_mode(2.0, 1.0)"
)]
fn sr_erlang_mode(shape: f64, rate: f64) -> DuckOptionResult<f64> {
    Ok(erlang("sr_erlang_mode", shape, rate)?.mode())
}

// ---------------------------------------------------------------------------
// Chi（freedom；自由度为整数）
// ---------------------------------------------------------------------------

/// Chi 分布均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Chi distribution mean, given whole-number freedom",
    example = "SELECT sr_chi_mean(2.0)"
)]
fn sr_chi_mean(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi("sr_chi_mean", freedom)?.mean())
}

/// Chi 分布方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Chi distribution variance, given whole-number freedom",
    example = "SELECT sr_chi_variance(2.0)"
)]
fn sr_chi_variance(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi("sr_chi_variance", freedom)?.variance())
}

/// Chi 分布标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Chi distribution standard deviation, given whole-number freedom",
    example = "SELECT sr_chi_std_dev(2.0)"
)]
fn sr_chi_std_dev(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi("sr_chi_std_dev", freedom)?.std_dev())
}

/// Chi 分布微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Chi distribution differential entropy, given whole-number freedom",
    example = "SELECT sr_chi_entropy(2.0)"
)]
fn sr_chi_entropy(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi("sr_chi_entropy", freedom)?.entropy())
}

/// Chi 分布偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Chi distribution skewness, given whole-number freedom",
    example = "SELECT sr_chi_skewness(2.0)"
)]
fn sr_chi_skewness(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi("sr_chi_skewness", freedom)?.skewness())
}

/// Chi 分布支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Chi distribution minimum of the support (0)",
    example = "SELECT sr_chi_min(2.0)"
)]
fn sr_chi_min(freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi("sr_chi_min", freedom)?.min()))
}

/// Chi 分布支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Chi distribution maximum of the support (positive infinity)",
    example = "SELECT sr_chi_max(2.0)"
)]
fn sr_chi_max(freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi("sr_chi_max", freedom)?.max()))
}

/// Chi 分布众数，对应 `Mode::mode`。
#[duck_scalar_function(
    description = "Chi distribution mode, given whole-number freedom",
    example = "SELECT sr_chi_mode(2.0)"
)]
fn sr_chi_mode(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi("sr_chi_mode", freedom)?.mode())
}

// ---------------------------------------------------------------------------
// ChiSquared（freedom）
// ---------------------------------------------------------------------------

/// 卡方均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Chi-squared mean, given degrees of freedom",
    example = "SELECT sr_chi_squared_mean(2.0)"
)]
fn sr_chi_squared_mean(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi_squared("sr_chi_squared_mean", freedom)?.mean())
}

/// 卡方方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Chi-squared variance, given degrees of freedom",
    example = "SELECT sr_chi_squared_variance(2.0)"
)]
fn sr_chi_squared_variance(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi_squared("sr_chi_squared_variance", freedom)?.variance())
}

/// 卡方标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Chi-squared standard deviation, given degrees of freedom",
    example = "SELECT sr_chi_squared_std_dev(2.0)"
)]
fn sr_chi_squared_std_dev(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi_squared("sr_chi_squared_std_dev", freedom)?.std_dev())
}

/// 卡方微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Chi-squared differential entropy, given degrees of freedom",
    example = "SELECT sr_chi_squared_entropy(2.0)"
)]
fn sr_chi_squared_entropy(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi_squared("sr_chi_squared_entropy", freedom)?.entropy())
}

/// 卡方偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Chi-squared skewness, given degrees of freedom",
    example = "SELECT sr_chi_squared_skewness(2.0)"
)]
fn sr_chi_squared_skewness(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi_squared("sr_chi_squared_skewness", freedom)?.skewness())
}

/// 卡方支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Chi-squared minimum of the support (0)",
    example = "SELECT sr_chi_squared_min(2.0)"
)]
fn sr_chi_squared_min(freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_min", freedom)?.min()))
}

/// 卡方支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Chi-squared maximum of the support (positive infinity)",
    example = "SELECT sr_chi_squared_max(2.0)"
)]
fn sr_chi_squared_max(freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_max", freedom)?.max()))
}

/// 卡方中位数，对应 `Median::median`。
#[duck_scalar_function(
    description = "Chi-squared median, given degrees of freedom",
    example = "SELECT sr_chi_squared_median(2.0)"
)]
fn sr_chi_squared_median(freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(chi_squared("sr_chi_squared_median", freedom)?.median()))
}

/// 卡方众数，对应 `Mode::mode`（freedom < 2 时无众数，返回 NULL）。
#[duck_scalar_function(
    description = "Chi-squared mode, given degrees of freedom",
    example = "SELECT sr_chi_squared_mode(2.0)"
)]
fn sr_chi_squared_mode(freedom: f64) -> DuckOptionResult<f64> {
    Ok(chi_squared("sr_chi_squared_mode", freedom)?.mode())
}

// ---------------------------------------------------------------------------
// Exp（rate）
// ---------------------------------------------------------------------------

/// 指数分布均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Exponential mean, given the rate",
    example = "SELECT sr_exp_mean(2.0)"
)]
fn sr_exp_mean(rate: f64) -> DuckOptionResult<f64> {
    Ok(exp("sr_exp_mean", rate)?.mean())
}

/// 指数分布方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Exponential variance, given the rate",
    example = "SELECT sr_exp_variance(2.0)"
)]
fn sr_exp_variance(rate: f64) -> DuckOptionResult<f64> {
    Ok(exp("sr_exp_variance", rate)?.variance())
}

/// 指数分布标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Exponential standard deviation, given the rate",
    example = "SELECT sr_exp_std_dev(2.0)"
)]
fn sr_exp_std_dev(rate: f64) -> DuckOptionResult<f64> {
    Ok(exp("sr_exp_std_dev", rate)?.std_dev())
}

/// 指数分布微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Exponential differential entropy, given the rate",
    example = "SELECT sr_exp_entropy(2.0)"
)]
fn sr_exp_entropy(rate: f64) -> DuckOptionResult<f64> {
    Ok(exp("sr_exp_entropy", rate)?.entropy())
}

/// 指数分布偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Exponential skewness, given the rate",
    example = "SELECT sr_exp_skewness(2.0)"
)]
fn sr_exp_skewness(rate: f64) -> DuckOptionResult<f64> {
    Ok(exp("sr_exp_skewness", rate)?.skewness())
}

/// 指数分布支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Exponential minimum of the support (0)",
    example = "SELECT sr_exp_min(2.0)"
)]
fn sr_exp_min(rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_min", rate)?.min()))
}

/// 指数分布支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Exponential maximum of the support (positive infinity)",
    example = "SELECT sr_exp_max(2.0)"
)]
fn sr_exp_max(rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_max", rate)?.max()))
}

/// 指数分布中位数，对应 `Median::median`。
#[duck_scalar_function(
    description = "Exponential median, given the rate",
    example = "SELECT sr_exp_median(2.0)"
)]
fn sr_exp_median(rate: f64) -> DuckOptionResult<f64> {
    Ok(Some(exp("sr_exp_median", rate)?.median()))
}

/// 指数分布众数，对应 `Mode::mode`（恒为 0）。
#[duck_scalar_function(
    description = "Exponential mode, given the rate",
    example = "SELECT sr_exp_mode(2.0)"
)]
fn sr_exp_mode(rate: f64) -> DuckOptionResult<f64> {
    Ok(exp("sr_exp_mode", rate)?.mode())
}

// ---------------------------------------------------------------------------
// InverseGamma（shape, scale）
// ---------------------------------------------------------------------------

/// 逆 Gamma 均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Inverse-gamma mean, given shape and scale",
    example = "SELECT sr_inverse_gamma_mean(2.0, 1.0)"
)]
fn sr_inverse_gamma_mean(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(inverse_gamma("sr_inverse_gamma_mean", shape, scale)?.mean())
}

/// 逆 Gamma 方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Inverse-gamma variance, given shape and scale",
    example = "SELECT sr_inverse_gamma_variance(3.0, 1.0)"
)]
fn sr_inverse_gamma_variance(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(inverse_gamma("sr_inverse_gamma_variance", shape, scale)?.variance())
}

/// 逆 Gamma 标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Inverse-gamma standard deviation, given shape and scale",
    example = "SELECT sr_inverse_gamma_std_dev(3.0, 1.0)"
)]
fn sr_inverse_gamma_std_dev(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(inverse_gamma("sr_inverse_gamma_std_dev", shape, scale)?.std_dev())
}

/// 逆 Gamma 微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Inverse-gamma differential entropy, given shape and scale",
    example = "SELECT sr_inverse_gamma_entropy(2.0, 1.0)"
)]
fn sr_inverse_gamma_entropy(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(inverse_gamma("sr_inverse_gamma_entropy", shape, scale)?.entropy())
}

/// 逆 Gamma 偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Inverse-gamma skewness, given shape and scale",
    example = "SELECT sr_inverse_gamma_skewness(4.0, 1.0)"
)]
fn sr_inverse_gamma_skewness(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(inverse_gamma("sr_inverse_gamma_skewness", shape, scale)?.skewness())
}

/// 逆 Gamma 支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Inverse-gamma minimum of the support (0)",
    example = "SELECT sr_inverse_gamma_min(2.0, 1.0)"
)]
fn sr_inverse_gamma_min(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(inverse_gamma("sr_inverse_gamma_min", shape, scale)?.min()))
}

/// 逆 Gamma 支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Inverse-gamma maximum of the support (positive infinity)",
    example = "SELECT sr_inverse_gamma_max(2.0, 1.0)"
)]
fn sr_inverse_gamma_max(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(inverse_gamma("sr_inverse_gamma_max", shape, scale)?.max()))
}

/// 逆 Gamma 众数，对应 `Mode::mode`。
#[duck_scalar_function(
    description = "Inverse-gamma mode, given shape and scale",
    example = "SELECT sr_inverse_gamma_mode(2.0, 1.0)"
)]
fn sr_inverse_gamma_mode(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(inverse_gamma("sr_inverse_gamma_mode", shape, scale)?.mode())
}

// ---------------------------------------------------------------------------
// Pareto（scale, shape）
// ---------------------------------------------------------------------------

/// Pareto（I 型）均值，对应 `Distribution::mean`（shape <= 1 时不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Pareto (type-I) mean, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_mean(1.0, 2.0)"
)]
fn sr_pareto_mean(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(pareto("sr_pareto_mean", scale, shape)?.mean())
}

/// Pareto 方差，对应 `Distribution::variance`（shape <= 2 时不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Pareto (type-I) variance, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_variance(1.0, 3.0)"
)]
fn sr_pareto_variance(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(pareto("sr_pareto_variance", scale, shape)?.variance())
}

/// Pareto 标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Pareto (type-I) standard deviation, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_std_dev(1.0, 3.0)"
)]
fn sr_pareto_std_dev(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(pareto("sr_pareto_std_dev", scale, shape)?.std_dev())
}

/// Pareto 微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Pareto (type-I) differential entropy, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_entropy(1.0, 2.0)"
)]
fn sr_pareto_entropy(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(pareto("sr_pareto_entropy", scale, shape)?.entropy())
}

/// Pareto 偏度，对应 `Distribution::skewness`（shape <= 3 时不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Pareto (type-I) skewness, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_skewness(1.0, 4.0)"
)]
fn sr_pareto_skewness(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(pareto("sr_pareto_skewness", scale, shape)?.skewness())
}

/// Pareto 支撑集下确界，对应 `Min::min`（等于 scale）。
#[duck_scalar_function(
    description = "Pareto (type-I) minimum of the support (the scale)",
    example = "SELECT sr_pareto_min(1.0, 2.0)"
)]
fn sr_pareto_min(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_min", scale, shape)?.min()))
}

/// Pareto 支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Pareto (type-I) maximum of the support (positive infinity)",
    example = "SELECT sr_pareto_max(1.0, 2.0)"
)]
fn sr_pareto_max(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_max", scale, shape)?.max()))
}

/// Pareto 中位数，对应 `Median::median`。
#[duck_scalar_function(
    description = "Pareto (type-I) median, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_median(1.0, 2.0)"
)]
fn sr_pareto_median(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(Some(pareto("sr_pareto_median", scale, shape)?.median()))
}

/// Pareto 众数，对应 `Mode::mode`。
#[duck_scalar_function(
    description = "Pareto (type-I) mode, given scale x_m and shape alpha",
    example = "SELECT sr_pareto_mode(1.0, 2.0)"
)]
fn sr_pareto_mode(scale: f64, shape: f64) -> DuckOptionResult<f64> {
    Ok(pareto("sr_pareto_mode", scale, shape)?.mode())
}

// ---------------------------------------------------------------------------
// Weibull（shape, scale）
// ---------------------------------------------------------------------------

/// Weibull 均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Weibull mean, given shape k and scale lambda",
    example = "SELECT sr_weibull_mean(1.0, 1.0)"
)]
fn sr_weibull_mean(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(weibull("sr_weibull_mean", shape, scale)?.mean())
}

/// Weibull 方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Weibull variance, given shape k and scale lambda",
    example = "SELECT sr_weibull_variance(1.0, 1.0)"
)]
fn sr_weibull_variance(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(weibull("sr_weibull_variance", shape, scale)?.variance())
}

/// Weibull 标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Weibull standard deviation, given shape k and scale lambda",
    example = "SELECT sr_weibull_std_dev(1.0, 1.0)"
)]
fn sr_weibull_std_dev(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(weibull("sr_weibull_std_dev", shape, scale)?.std_dev())
}

/// Weibull 微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Weibull differential entropy, given shape k and scale lambda",
    example = "SELECT sr_weibull_entropy(1.0, 1.0)"
)]
fn sr_weibull_entropy(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(weibull("sr_weibull_entropy", shape, scale)?.entropy())
}

/// Weibull 偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Weibull skewness, given shape k and scale lambda",
    example = "SELECT sr_weibull_skewness(1.0, 1.0)"
)]
fn sr_weibull_skewness(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(weibull("sr_weibull_skewness", shape, scale)?.skewness())
}

/// Weibull 支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Weibull minimum of the support (0)",
    example = "SELECT sr_weibull_min(1.0, 1.0)"
)]
fn sr_weibull_min(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_min", shape, scale)?.min()))
}

/// Weibull 支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Weibull maximum of the support (positive infinity)",
    example = "SELECT sr_weibull_max(1.0, 1.0)"
)]
fn sr_weibull_max(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_max", shape, scale)?.max()))
}

/// Weibull 中位数，对应 `Median::median`。
#[duck_scalar_function(
    description = "Weibull median, given shape k and scale lambda",
    example = "SELECT sr_weibull_median(1.0, 1.0)"
)]
fn sr_weibull_median(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(weibull("sr_weibull_median", shape, scale)?.median()))
}

/// Weibull 众数，对应 `Mode::mode`（shape <= 1 时返回 0）。
#[duck_scalar_function(
    description = "Weibull mode, given shape k and scale lambda",
    example = "SELECT sr_weibull_mode(2.0, 1.0)"
)]
fn sr_weibull_mode(shape: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(weibull("sr_weibull_mode", shape, scale)?.mode())
}

// ---------------------------------------------------------------------------
// Beta（shape_a, shape_b）
// ---------------------------------------------------------------------------

/// Beta 均值，对应 `Distribution::mean`。
#[duck_scalar_function(
    description = "Beta mean, given shape_a and shape_b",
    example = "SELECT sr_beta_mean(2.0, 3.0)"
)]
fn sr_beta_mean(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(beta("sr_beta_mean", shape_a, shape_b)?.mean())
}

/// Beta 方差，对应 `Distribution::variance`。
#[duck_scalar_function(
    description = "Beta variance, given shape_a and shape_b",
    example = "SELECT sr_beta_variance(2.0, 3.0)"
)]
fn sr_beta_variance(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(beta("sr_beta_variance", shape_a, shape_b)?.variance())
}

/// Beta 标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Beta standard deviation, given shape_a and shape_b",
    example = "SELECT sr_beta_std_dev(2.0, 3.0)"
)]
fn sr_beta_std_dev(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(beta("sr_beta_std_dev", shape_a, shape_b)?.std_dev())
}

/// Beta 微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Beta differential entropy, given shape_a and shape_b",
    example = "SELECT sr_beta_entropy(2.0, 3.0)"
)]
fn sr_beta_entropy(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(beta("sr_beta_entropy", shape_a, shape_b)?.entropy())
}

/// Beta 偏度，对应 `Distribution::skewness`。
#[duck_scalar_function(
    description = "Beta skewness, given shape_a and shape_b",
    example = "SELECT sr_beta_skewness(2.0, 3.0)"
)]
fn sr_beta_skewness(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(beta("sr_beta_skewness", shape_a, shape_b)?.skewness())
}

/// Beta 支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Beta minimum of the support (0)",
    example = "SELECT sr_beta_min(2.0, 3.0)"
)]
fn sr_beta_min(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(Some(beta("sr_beta_min", shape_a, shape_b)?.min()))
}

/// Beta 支撑集上确界，对应 `Max::max`（恒为 1）。
#[duck_scalar_function(
    description = "Beta maximum of the support (1)",
    example = "SELECT sr_beta_max(2.0, 3.0)"
)]
fn sr_beta_max(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(Some(beta("sr_beta_max", shape_a, shape_b)?.max()))
}

/// Beta 众数，对应 `Mode::mode`（shape_a <= 1 或 shape_b <= 1 时无众数，返回 NULL）。
#[duck_scalar_function(
    description = "Beta mode, given shape_a and shape_b",
    example = "SELECT sr_beta_mode(2.0, 3.0)"
)]
fn sr_beta_mode(shape_a: f64, shape_b: f64) -> DuckOptionResult<f64> {
    Ok(beta("sr_beta_mode", shape_a, shape_b)?.mode())
}

// ---------------------------------------------------------------------------
// FisherSnedecor（freedom_1, freedom_2）
// ---------------------------------------------------------------------------

/// F 分布均值，对应 `Distribution::mean`（freedom_2 <= 2 时不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) mean, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_mean(3.0, 5.0)"
)]
fn sr_fisher_snedecor_mean(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(fisher_snedecor("sr_fisher_snedecor_mean", freedom_1, freedom_2)?.mean())
}

/// F 分布方差，对应 `Distribution::variance`（freedom_2 <= 4 时不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) variance, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_variance(3.0, 5.0)"
)]
fn sr_fisher_snedecor_variance(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(fisher_snedecor("sr_fisher_snedecor_variance", freedom_1, freedom_2)?.variance())
}

/// F 分布标准差，对应 `Distribution::std_dev`。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) standard deviation, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_std_dev(3.0, 5.0)"
)]
fn sr_fisher_snedecor_std_dev(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(fisher_snedecor("sr_fisher_snedecor_std_dev", freedom_1, freedom_2)?.std_dev())
}

/// F 分布微分熵，对应 `Distribution::entropy`。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) differential entropy, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_entropy(3.0, 5.0)"
)]
fn sr_fisher_snedecor_entropy(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(fisher_snedecor("sr_fisher_snedecor_entropy", freedom_1, freedom_2)?.entropy())
}

/// F 分布偏度，对应 `Distribution::skewness`（freedom_2 <= 6 时不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) skewness, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_skewness(3.0, 7.0)"
)]
fn sr_fisher_snedecor_skewness(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(fisher_snedecor("sr_fisher_snedecor_skewness", freedom_1, freedom_2)?.skewness())
}

/// F 分布支撑集下确界，对应 `Min::min`（恒为 0）。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) minimum of the support (0)",
    example = "SELECT sr_fisher_snedecor_min(3.0, 5.0)"
)]
fn sr_fisher_snedecor_min(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_min", freedom_1, freedom_2)?.min()))
}

/// F 分布支撑集上确界，对应 `Max::max`（无界，返回 +inf）。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) maximum of the support (positive infinity)",
    example = "SELECT sr_fisher_snedecor_max(3.0, 5.0)"
)]
fn sr_fisher_snedecor_max(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(Some(fisher_snedecor("sr_fisher_snedecor_max", freedom_1, freedom_2)?.max()))
}

/// F 分布众数，对应 `Mode::mode`（freedom_1 <= 2 时无众数，返回 NULL）。
#[duck_scalar_function(
    description = "Fisher-Snedecor (F) mode, given the two degrees of freedom",
    example = "SELECT sr_fisher_snedecor_mode(3.0, 5.0)"
)]
fn sr_fisher_snedecor_mode(freedom_1: f64, freedom_2: f64) -> DuckOptionResult<f64> {
    Ok(fisher_snedecor("sr_fisher_snedecor_mode", freedom_1, freedom_2)?.mode())
}
