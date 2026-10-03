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
