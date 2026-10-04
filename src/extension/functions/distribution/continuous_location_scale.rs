// ============================================================================
// 连续分布 · 位置-尺度族（statrs::distribution 的 normal / log_normal / cauchy /
// laplace / gumbel / levy / uniform / triangular / students_t / dirac）
//
// 每个分布五个标量函数：`sr_<分布>_pdf / ln_pdf / cdf / sf / quantile`
// （statrs 的 inverse_cdf 在 SQL 侧叫 quantile）。参数构造失败（尺度非正、区间倒挂…）
// 报查询错误 —— 那是调用写错，不是「结果为空」；quantile 的 p 越出 [0,1] 同理。
// 任一参数为 NULL 的行短路成 NULL（duckfn 默认）。
//
// Location-scale continuous distributions; five scalars each: pdf / ln_pdf / cdf / sf /
// quantile (statrs' inverse_cdf). Invalid parameters fail the query; a NULL argument
// short-circuits the row to NULL.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{
    Continuous, ContinuousCDF, Cauchy, Dirac, Gumbel, Laplace, Levy, LogNormal, Normal,
    StudentsT, Triangular, Uniform,
};
use statrs::statistics::{Distribution, Max, Median, Min, Mode};

use super::check_probability;

// ---------------------------------------------------------------------------
// Normal（statrs::distribution::Normal）
// ---------------------------------------------------------------------------

fn normal(name: &str, mean: f64, std_dev: f64) -> Result<Normal, ExtensionError> {
    Normal::new(mean, std_dev).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 正态概率密度 `N(mean, std_dev)` 在 x 处的取值。
#[duck_scalar_function(
    description = "Normal (Gaussian) probability density at x, given mean and standard deviation",
    example = "SELECT sr_normal_pdf(0.0, 0.0, 1.0)"
)]
fn sr_normal_pdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_pdf", mean, std_dev)?.pdf(x)))
}

/// 正态对数密度。
#[duck_scalar_function(
    description = "Normal (Gaussian) log-density at x, given mean and standard deviation",
    example = "SELECT sr_normal_ln_pdf(0.0, 0.0, 1.0)"
)]
fn sr_normal_ln_pdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_ln_pdf", mean, std_dev)?.ln_pdf(x)))
}

/// 正态累积分布 P(X <= x)。
#[duck_scalar_function(
    description = "Normal (Gaussian) cumulative distribution function P(X <= x)",
    example = "SELECT sr_normal_cdf(1.96, 0.0, 1.0)"
)]
fn sr_normal_cdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_cdf", mean, std_dev)?.cdf(x)))
}

/// 正态生存函数 P(X > x) = 1 - cdf。
#[duck_scalar_function(
    description = "Normal (Gaussian) survival function P(X > x)",
    example = "SELECT sr_normal_sf(1.96, 0.0, 1.0)"
)]
fn sr_normal_sf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_sf", mean, std_dev)?.sf(x)))
}

/// 正态分位数函数（CDF 的反函数）。
#[duck_scalar_function(
    description = "Normal (Gaussian) quantile function: the x whose CDF equals p, for p in [0, 1]",
    comment = "A probability outside [0, 1] is a query error rather than a clamped endpoint",
    example = "SELECT sr_normal_quantile(0.975, 0.0, 1.0)"
)]
fn sr_normal_quantile(p: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    check_probability("sr_normal_quantile", p)?;
    Ok(Some(normal("sr_normal_quantile", mean, std_dev)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// LogNormal（statrs::distribution::LogNormal）
// ---------------------------------------------------------------------------

fn log_normal(name: &str, location: f64, scale: f64) -> Result<LogNormal, ExtensionError> {
    LogNormal::new(location, scale).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 对数正态概率密度（location/scale 是 ln X 的正态参数）。
#[duck_scalar_function(
    description = "Log-normal probability density at x (location and scale of ln X)",
    example = "SELECT sr_log_normal_pdf(1.0, 0.0, 1.0)"
)]
fn sr_log_normal_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_pdf", location, scale)?.pdf(x)))
}

/// 对数正态对数密度。
#[duck_scalar_function(
    description = "Log-normal log-density at x",
    example = "SELECT sr_log_normal_ln_pdf(1.0, 0.0, 1.0)"
)]
fn sr_log_normal_ln_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_ln_pdf", location, scale)?.ln_pdf(x)))
}

/// 对数正态累积分布。
#[duck_scalar_function(
    description = "Log-normal cumulative distribution function P(X <= x)",
    example = "SELECT sr_log_normal_cdf(1.0, 0.0, 1.0)"
)]
fn sr_log_normal_cdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_cdf", location, scale)?.cdf(x)))
}

/// 对数正态生存函数。
#[duck_scalar_function(
    description = "Log-normal survival function P(X > x)",
    example = "SELECT sr_log_normal_sf(1.0, 0.0, 1.0)"
)]
fn sr_log_normal_sf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_sf", location, scale)?.sf(x)))
}

/// 对数正态分位数函数。
#[duck_scalar_function(
    description = "Log-normal quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_log_normal_quantile(0.5, 0.0, 1.0)"
)]
fn sr_log_normal_quantile(p: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    check_probability("sr_log_normal_quantile", p)?;
    Ok(Some(log_normal("sr_log_normal_quantile", location, scale)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Cauchy（statrs::distribution::Cauchy）
// ---------------------------------------------------------------------------

fn cauchy(name: &str, location: f64, scale: f64) -> Result<Cauchy, ExtensionError> {
    Cauchy::new(location, scale).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 柯西概率密度。
#[duck_scalar_function(
    description = "Cauchy probability density at x, given location and scale",
    example = "SELECT sr_cauchy_pdf(1.0, 0.0, 1.0)"
)]
fn sr_cauchy_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_pdf", location, scale)?.pdf(x)))
}

/// 柯西对数密度。
#[duck_scalar_function(
    description = "Cauchy log-density at x",
    example = "SELECT sr_cauchy_ln_pdf(1.0, 0.0, 1.0)"
)]
fn sr_cauchy_ln_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_ln_pdf", location, scale)?.ln_pdf(x)))
}

/// 柯西累积分布。
#[duck_scalar_function(
    description = "Cauchy cumulative distribution function P(X <= x)",
    example = "SELECT sr_cauchy_cdf(1.0, 0.0, 1.0)"
)]
fn sr_cauchy_cdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_cdf", location, scale)?.cdf(x)))
}

/// 柯西生存函数。
#[duck_scalar_function(
    description = "Cauchy survival function P(X > x)",
    example = "SELECT sr_cauchy_sf(1.0, 0.0, 1.0)"
)]
fn sr_cauchy_sf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_sf", location, scale)?.sf(x)))
}

/// 柯西分位数函数。
#[duck_scalar_function(
    description = "Cauchy quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_cauchy_quantile(0.5, 0.0, 1.0)"
)]
fn sr_cauchy_quantile(p: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    check_probability("sr_cauchy_quantile", p)?;
    Ok(Some(cauchy("sr_cauchy_quantile", location, scale)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Laplace（statrs::distribution::Laplace）
// ---------------------------------------------------------------------------

fn laplace(name: &str, location: f64, scale: f64) -> Result<Laplace, ExtensionError> {
    Laplace::new(location, scale).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 拉普拉斯概率密度。
#[duck_scalar_function(
    description = "Laplace probability density at x, given location and scale",
    example = "SELECT sr_laplace_pdf(1.0, 0.0, 1.0)"
)]
fn sr_laplace_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_pdf", location, scale)?.pdf(x)))
}

/// 拉普拉斯对数密度。
#[duck_scalar_function(
    description = "Laplace log-density at x",
    example = "SELECT sr_laplace_ln_pdf(1.0, 0.0, 1.0)"
)]
fn sr_laplace_ln_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_ln_pdf", location, scale)?.ln_pdf(x)))
}

/// 拉普拉斯累积分布。
#[duck_scalar_function(
    description = "Laplace cumulative distribution function P(X <= x)",
    example = "SELECT sr_laplace_cdf(1.0, 0.0, 1.0)"
)]
fn sr_laplace_cdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_cdf", location, scale)?.cdf(x)))
}

/// 拉普拉斯生存函数。
#[duck_scalar_function(
    description = "Laplace survival function P(X > x)",
    example = "SELECT sr_laplace_sf(1.0, 0.0, 1.0)"
)]
fn sr_laplace_sf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_sf", location, scale)?.sf(x)))
}

/// 拉普拉斯分位数函数。
#[duck_scalar_function(
    description = "Laplace quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_laplace_quantile(0.5, 0.0, 1.0)"
)]
fn sr_laplace_quantile(p: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    check_probability("sr_laplace_quantile", p)?;
    Ok(Some(laplace("sr_laplace_quantile", location, scale)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Gumbel（statrs::distribution::Gumbel）
// ---------------------------------------------------------------------------

fn gumbel(name: &str, location: f64, scale: f64) -> Result<Gumbel, ExtensionError> {
    Gumbel::new(location, scale).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Gumbel（I 型极值）概率密度。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) probability density at x, given location and scale",
    example = "SELECT sr_gumbel_pdf(1.0, 0.0, 1.0)"
)]
fn sr_gumbel_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_pdf", location, scale)?.pdf(x)))
}

/// Gumbel 对数密度。
#[duck_scalar_function(
    description = "Gumbel log-density at x",
    example = "SELECT sr_gumbel_ln_pdf(1.0, 0.0, 1.0)"
)]
fn sr_gumbel_ln_pdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_ln_pdf", location, scale)?.ln_pdf(x)))
}

/// Gumbel 累积分布。
#[duck_scalar_function(
    description = "Gumbel cumulative distribution function P(X <= x)",
    example = "SELECT sr_gumbel_cdf(1.0, 0.0, 1.0)"
)]
fn sr_gumbel_cdf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_cdf", location, scale)?.cdf(x)))
}

/// Gumbel 生存函数。
#[duck_scalar_function(
    description = "Gumbel survival function P(X > x)",
    example = "SELECT sr_gumbel_sf(1.0, 0.0, 1.0)"
)]
fn sr_gumbel_sf(x: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_sf", location, scale)?.sf(x)))
}

/// Gumbel 分位数函数。
#[duck_scalar_function(
    description = "Gumbel quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_gumbel_quantile(0.5, 0.0, 1.0)"
)]
fn sr_gumbel_quantile(p: f64, location: f64, scale: f64) -> DuckOptionResult<f64> {
    check_probability("sr_gumbel_quantile", p)?;
    Ok(Some(gumbel("sr_gumbel_quantile", location, scale)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Levy（statrs::distribution::Levy）
// ---------------------------------------------------------------------------

fn levy(name: &str, mu: f64, c: f64) -> Result<Levy, ExtensionError> {
    Levy::new(mu, c).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Lévy 概率密度（位置 mu、尺度 c > 0）。
#[duck_scalar_function(
    description = "Lévy probability density at x, given location mu and scale c",
    example = "SELECT sr_levy_pdf(1.0, 0.0, 1.0)"
)]
fn sr_levy_pdf(x: f64, mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_pdf", mu, c)?.pdf(x)))
}

/// Lévy 对数密度。
#[duck_scalar_function(
    description = "Lévy log-density at x",
    example = "SELECT sr_levy_ln_pdf(1.0, 0.0, 1.0)"
)]
fn sr_levy_ln_pdf(x: f64, mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_ln_pdf", mu, c)?.ln_pdf(x)))
}

/// Lévy 累积分布。
#[duck_scalar_function(
    description = "Lévy cumulative distribution function P(X <= x)",
    example = "SELECT sr_levy_cdf(1.0, 0.0, 1.0)"
)]
fn sr_levy_cdf(x: f64, mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_cdf", mu, c)?.cdf(x)))
}

/// Lévy 生存函数。
#[duck_scalar_function(
    description = "Lévy survival function P(X > x)",
    example = "SELECT sr_levy_sf(1.0, 0.0, 1.0)"
)]
fn sr_levy_sf(x: f64, mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_sf", mu, c)?.sf(x)))
}

/// Lévy 分位数函数（statrs 走默认二分法，精度受限）。
#[duck_scalar_function(
    description = "Lévy quantile function: the x whose CDF equals p, for p in [0, 1]",
    comment = "statrs solves this one numerically (bisection), accuracy is lower than the closed forms",
    example = "SELECT sr_levy_quantile(0.5, 0.0, 1.0)"
)]
fn sr_levy_quantile(p: f64, mu: f64, c: f64) -> DuckOptionResult<f64> {
    check_probability("sr_levy_quantile", p)?;
    Ok(Some(levy("sr_levy_quantile", mu, c)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Uniform（statrs::distribution::Uniform，连续）
// ---------------------------------------------------------------------------

fn uniform(name: &str, min: f64, max: f64) -> Result<Uniform, ExtensionError> {
    Uniform::new(min, max).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 连续均匀概率密度 U(min, max)。
#[duck_scalar_function(
    description = "Continuous uniform probability density at x on [min, max]",
    example = "SELECT sr_uniform_pdf(0.5, 0.0, 1.0)"
)]
fn sr_uniform_pdf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_pdf", min, max)?.pdf(x)))
}

/// 连续均匀对数密度。
#[duck_scalar_function(
    description = "Continuous uniform log-density at x",
    example = "SELECT sr_uniform_ln_pdf(0.5, 0.0, 1.0)"
)]
fn sr_uniform_ln_pdf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_ln_pdf", min, max)?.ln_pdf(x)))
}

/// 连续均匀累积分布。
#[duck_scalar_function(
    description = "Continuous uniform cumulative distribution function P(X <= x)",
    example = "SELECT sr_uniform_cdf(0.5, 0.0, 1.0)"
)]
fn sr_uniform_cdf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_cdf", min, max)?.cdf(x)))
}

/// 连续均匀生存函数。
#[duck_scalar_function(
    description = "Continuous uniform survival function P(X > x)",
    example = "SELECT sr_uniform_sf(0.5, 0.0, 1.0)"
)]
fn sr_uniform_sf(x: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_sf", min, max)?.sf(x)))
}

/// 连续均匀分位数函数。
#[duck_scalar_function(
    description = "Continuous uniform quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_uniform_quantile(0.25, 0.0, 1.0)"
)]
fn sr_uniform_quantile(p: f64, min: f64, max: f64) -> DuckOptionResult<f64> {
    check_probability("sr_uniform_quantile", p)?;
    Ok(Some(uniform("sr_uniform_quantile", min, max)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Triangular（statrs::distribution::Triangular）
// ---------------------------------------------------------------------------

fn triangular(name: &str, min: f64, max: f64, mode: f64) -> Result<Triangular, ExtensionError> {
    Triangular::new(min, max, mode).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 三角分布概率密度（min <= mode <= max）。
#[duck_scalar_function(
    description = "Triangular probability density at x, given min, max and mode",
    example = "SELECT sr_triangular_pdf(1.0, 0.0, 2.0, 1.0)"
)]
fn sr_triangular_pdf(x: f64, min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_pdf", min, max, mode)?.pdf(x)))
}

/// 三角分布对数密度。
#[duck_scalar_function(
    description = "Triangular log-density at x",
    example = "SELECT sr_triangular_ln_pdf(1.0, 0.0, 2.0, 1.0)"
)]
fn sr_triangular_ln_pdf(x: f64, min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_ln_pdf", min, max, mode)?.ln_pdf(x)))
}

/// 三角分布累积分布。
#[duck_scalar_function(
    description = "Triangular cumulative distribution function P(X <= x)",
    example = "SELECT sr_triangular_cdf(1.0, 0.0, 2.0, 1.0)"
)]
fn sr_triangular_cdf(x: f64, min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_cdf", min, max, mode)?.cdf(x)))
}

/// 三角分布生存函数。
#[duck_scalar_function(
    description = "Triangular survival function P(X > x)",
    example = "SELECT sr_triangular_sf(1.0, 0.0, 2.0, 1.0)"
)]
fn sr_triangular_sf(x: f64, min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_sf", min, max, mode)?.sf(x)))
}

/// 三角分布分位数函数。
#[duck_scalar_function(
    description = "Triangular quantile function: the x whose CDF equals p, for p in [0, 1]",
    example = "SELECT sr_triangular_quantile(0.5, 0.0, 2.0, 1.0)"
)]
fn sr_triangular_quantile(p: f64, min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    check_probability("sr_triangular_quantile", p)?;
    Ok(Some(triangular("sr_triangular_quantile", min, max, mode)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// StudentsT（statrs::distribution::StudentsT）
// ---------------------------------------------------------------------------

fn students_t(name: &str, location: f64, scale: f64, freedom: f64) -> Result<StudentsT, ExtensionError> {
    StudentsT::new(location, scale, freedom).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 学生 t 分布概率密度。
#[duck_scalar_function(
    description = "Student's t probability density at x, given location, scale and degrees of freedom",
    example = "SELECT sr_students_t_pdf(1.0, 0.0, 1.0, 2.0)"
)]
fn sr_students_t_pdf(x: f64, location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_pdf", location, scale, freedom)?.pdf(x)))
}

/// 学生 t 分布对数密度。
#[duck_scalar_function(
    description = "Student's t log-density at x",
    example = "SELECT sr_students_t_ln_pdf(1.0, 0.0, 1.0, 2.0)"
)]
fn sr_students_t_ln_pdf(x: f64, location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_ln_pdf", location, scale, freedom)?.ln_pdf(x)))
}

/// 学生 t 分布累积分布。
#[duck_scalar_function(
    description = "Student's t cumulative distribution function P(X <= x)",
    example = "SELECT sr_students_t_cdf(1.0, 0.0, 1.0, 2.0)"
)]
fn sr_students_t_cdf(x: f64, location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_cdf", location, scale, freedom)?.cdf(x)))
}

/// 学生 t 分布生存函数。
#[duck_scalar_function(
    description = "Student's t survival function P(X > x)",
    example = "SELECT sr_students_t_sf(1.0, 0.0, 1.0, 2.0)"
)]
fn sr_students_t_sf(x: f64, location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_sf", location, scale, freedom)?.sf(x)))
}

/// 学生 t 分布分位数函数（statrs 走数值二分法）。
#[duck_scalar_function(
    description = "Student's t quantile function: the x whose CDF equals p, for p in [0, 1]",
    comment = "statrs solves this one numerically (bisection), accuracy is lower than the closed forms",
    example = "SELECT sr_students_t_quantile(0.975, 0.0, 1.0, 10.0)"
)]
fn sr_students_t_quantile(p: f64, location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    check_probability("sr_students_t_quantile", p)?;
    Ok(Some(students_t("sr_students_t_quantile", location, scale, freedom)?.inverse_cdf(p)))
}

// ---------------------------------------------------------------------------
// Dirac（statrs::distribution::Dirac，退化分布）
//
// 只包 ContinuousCDF 提供的三个方法：statrs 没给 Dirac 实现 Continuous —— 退化分布
// 没有通常意义的密度，我们不替它发明。
// ---------------------------------------------------------------------------

fn dirac(name: &str, v: f64) -> Result<Dirac, ExtensionError> {
    Dirac::new(v).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Dirac 退化分布累积分布（x >= v 后为 1）。
#[duck_scalar_function(
    description = "Dirac delta cumulative distribution function P(X <= x)",
    example = "SELECT sr_dirac_cdf(1.0, 1.0)"
)]
fn sr_dirac_cdf(x: f64, v: f64) -> DuckOptionResult<f64> {
    Ok(Some(dirac("sr_dirac_cdf", v)?.cdf(x)))
}

/// Dirac 生存函数。
#[duck_scalar_function(
    description = "Dirac delta survival function P(X > x)",
    example = "SELECT sr_dirac_sf(0.5, 1.0)"
)]
fn sr_dirac_sf(x: f64, v: f64) -> DuckOptionResult<f64> {
    Ok(Some(dirac("sr_dirac_sf", v)?.sf(x)))
}

/// Dirac 分位数函数：恒等于位置 v。
#[duck_scalar_function(
    description = "Dirac delta quantile function: always the location v",
    example = "SELECT sr_dirac_quantile(0.5, 1.0)"
)]
fn sr_dirac_quantile(p: f64, v: f64) -> DuckOptionResult<f64> {
    check_probability("sr_dirac_quantile", p)?;
    Ok(Some(dirac("sr_dirac_quantile", v)?.inverse_cdf(p)))
}

// ============================================================================
// 分布矩与域 · 十个位置-尺度分布
//
// 每个分布补九个标量函数：mean / variance / std_dev / entropy / skewness（返回 Option）
// 与 min / max / median / mode（域端点与中心位置）。min/max 是支撑域端点：无界分布返回
// ±inf，保留为 inf 而不是转 NULL。mode 一般为 Option；Gumbel 例外（statrs 给它裸 f64）。
//
// Moments and domain of the ten location-scale distributions: mean / variance /
// std_dev / entropy / skewness (Option-valued) plus min / max / median / mode.
// Endpoints of unbounded distributions stay ±inf rather than NULL.
// ============================================================================

// ---------------------------------------------------------------------------
// Normal（均矩与域）
// ---------------------------------------------------------------------------

/// 正态分布均值。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution mean",
    example = "SELECT sr_normal_mean(0.0, 1.0)"
)]
fn sr_normal_mean(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(normal("sr_normal_mean", mean, std_dev)?.mean())
}

/// 正态分布方差。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution variance",
    example = "SELECT sr_normal_variance(0.0, 1.0)"
)]
fn sr_normal_variance(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(normal("sr_normal_variance", mean, std_dev)?.variance())
}

/// 正态分布标准差。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution standard deviation",
    example = "SELECT sr_normal_std_dev(0.0, 1.0)"
)]
fn sr_normal_std_dev(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(normal("sr_normal_std_dev", mean, std_dev)?.std_dev())
}

/// 正态分布微分熵。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution differential entropy",
    example = "SELECT sr_normal_entropy(0.0, 1.0)"
)]
fn sr_normal_entropy(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(normal("sr_normal_entropy", mean, std_dev)?.entropy())
}

/// 正态分布偏度（恒为 0）。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution skewness (always 0)",
    example = "SELECT sr_normal_skewness(0.0, 1.0)"
)]
fn sr_normal_skewness(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(normal("sr_normal_skewness", mean, std_dev)?.skewness())
}

/// 正态分布支撑域下确界（-inf，无界）。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution support minimum (negative infinity)",
    example = "SELECT sr_normal_min(0.0, 1.0)"
)]
fn sr_normal_min(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_min", mean, std_dev)?.min()))
}

/// 正态分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution support maximum (positive infinity)",
    example = "SELECT sr_normal_max(0.0, 1.0)"
)]
fn sr_normal_max(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_max", mean, std_dev)?.max()))
}

/// 正态分布中位数。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution median",
    example = "SELECT sr_normal_median(0.0, 1.0)"
)]
fn sr_normal_median(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(Some(normal("sr_normal_median", mean, std_dev)?.median()))
}

/// 正态分布众数。
#[duck_scalar_function(
    description = "Normal (Gaussian) distribution mode",
    example = "SELECT sr_normal_mode(0.0, 1.0)"
)]
fn sr_normal_mode(mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    Ok(normal("sr_normal_mode", mean, std_dev)?.mode())
}

// ---------------------------------------------------------------------------
// LogNormal（均矩与域）
// ---------------------------------------------------------------------------

/// 对数正态分布均值。
#[duck_scalar_function(
    description = "Log-normal distribution mean",
    example = "SELECT sr_log_normal_mean(0.0, 1.0)"
)]
fn sr_log_normal_mean(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(log_normal("sr_log_normal_mean", location, scale)?.mean())
}

/// 对数正态分布方差。
#[duck_scalar_function(
    description = "Log-normal distribution variance",
    example = "SELECT sr_log_normal_variance(0.0, 1.0)"
)]
fn sr_log_normal_variance(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(log_normal("sr_log_normal_variance", location, scale)?.variance())
}

/// 对数正态分布标准差。
#[duck_scalar_function(
    description = "Log-normal distribution standard deviation",
    example = "SELECT sr_log_normal_std_dev(0.0, 1.0)"
)]
fn sr_log_normal_std_dev(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(log_normal("sr_log_normal_std_dev", location, scale)?.std_dev())
}

/// 对数正态分布微分熵。
#[duck_scalar_function(
    description = "Log-normal distribution differential entropy",
    example = "SELECT sr_log_normal_entropy(0.0, 1.0)"
)]
fn sr_log_normal_entropy(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(log_normal("sr_log_normal_entropy", location, scale)?.entropy())
}

/// 对数正态分布偏度。
#[duck_scalar_function(
    description = "Log-normal distribution skewness",
    example = "SELECT sr_log_normal_skewness(0.0, 1.0)"
)]
fn sr_log_normal_skewness(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(log_normal("sr_log_normal_skewness", location, scale)?.skewness())
}

/// 对数正态分布支撑域下确界（0）。
#[duck_scalar_function(
    description = "Log-normal distribution support minimum (0)",
    example = "SELECT sr_log_normal_min(0.0, 1.0)"
)]
fn sr_log_normal_min(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_min", location, scale)?.min()))
}

/// 对数正态分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Log-normal distribution support maximum (positive infinity)",
    example = "SELECT sr_log_normal_max(0.0, 1.0)"
)]
fn sr_log_normal_max(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_max", location, scale)?.max()))
}

/// 对数正态分布中位数。
#[duck_scalar_function(
    description = "Log-normal distribution median",
    example = "SELECT sr_log_normal_median(0.0, 1.0)"
)]
fn sr_log_normal_median(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(log_normal("sr_log_normal_median", location, scale)?.median()))
}

/// 对数正态分布众数。
#[duck_scalar_function(
    description = "Log-normal distribution mode",
    example = "SELECT sr_log_normal_mode(0.0, 1.0)"
)]
fn sr_log_normal_mode(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(log_normal("sr_log_normal_mode", location, scale)?.mode())
}

// ---------------------------------------------------------------------------
// Cauchy（均矩与域）
// ---------------------------------------------------------------------------

/// 柯西分布均值（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Cauchy distribution mean (undefined)",
    example = "SELECT sr_cauchy_mean(0.0, 1.0)"
)]
fn sr_cauchy_mean(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(cauchy("sr_cauchy_mean", location, scale)?.mean())
}

/// 柯西分布方差（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Cauchy distribution variance (undefined)",
    example = "SELECT sr_cauchy_variance(0.0, 1.0)"
)]
fn sr_cauchy_variance(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(cauchy("sr_cauchy_variance", location, scale)?.variance())
}

/// 柯西分布标准差（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Cauchy distribution standard deviation (undefined)",
    example = "SELECT sr_cauchy_std_dev(0.0, 1.0)"
)]
fn sr_cauchy_std_dev(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(cauchy("sr_cauchy_std_dev", location, scale)?.std_dev())
}

/// 柯西分布微分熵。
#[duck_scalar_function(
    description = "Cauchy distribution differential entropy",
    example = "SELECT sr_cauchy_entropy(0.0, 1.0)"
)]
fn sr_cauchy_entropy(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(cauchy("sr_cauchy_entropy", location, scale)?.entropy())
}

/// 柯西分布偏度（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Cauchy distribution skewness (undefined)",
    example = "SELECT sr_cauchy_skewness(0.0, 1.0)"
)]
fn sr_cauchy_skewness(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(cauchy("sr_cauchy_skewness", location, scale)?.skewness())
}

/// 柯西分布支撑域下确界（-inf，无界）。
#[duck_scalar_function(
    description = "Cauchy distribution support minimum (negative infinity)",
    example = "SELECT sr_cauchy_min(0.0, 1.0)"
)]
fn sr_cauchy_min(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_min", location, scale)?.min()))
}

/// 柯西分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Cauchy distribution support maximum (positive infinity)",
    example = "SELECT sr_cauchy_max(0.0, 1.0)"
)]
fn sr_cauchy_max(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_max", location, scale)?.max()))
}

/// 柯西分布中位数。
#[duck_scalar_function(
    description = "Cauchy distribution median",
    example = "SELECT sr_cauchy_median(0.0, 1.0)"
)]
fn sr_cauchy_median(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(cauchy("sr_cauchy_median", location, scale)?.median()))
}

/// 柯西分布众数。
#[duck_scalar_function(
    description = "Cauchy distribution mode",
    example = "SELECT sr_cauchy_mode(0.0, 1.0)"
)]
fn sr_cauchy_mode(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(cauchy("sr_cauchy_mode", location, scale)?.mode())
}

// ---------------------------------------------------------------------------
// Laplace（均矩与域）
// ---------------------------------------------------------------------------

/// 拉普拉斯分布均值。
#[duck_scalar_function(
    description = "Laplace distribution mean",
    example = "SELECT sr_laplace_mean(0.0, 1.0)"
)]
fn sr_laplace_mean(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(laplace("sr_laplace_mean", location, scale)?.mean())
}

/// 拉普拉斯分布方差。
#[duck_scalar_function(
    description = "Laplace distribution variance",
    example = "SELECT sr_laplace_variance(0.0, 1.0)"
)]
fn sr_laplace_variance(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(laplace("sr_laplace_variance", location, scale)?.variance())
}

/// 拉普拉斯分布标准差。
#[duck_scalar_function(
    description = "Laplace distribution standard deviation",
    example = "SELECT sr_laplace_std_dev(0.0, 1.0)"
)]
fn sr_laplace_std_dev(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(laplace("sr_laplace_std_dev", location, scale)?.std_dev())
}

/// 拉普拉斯分布微分熵。
#[duck_scalar_function(
    description = "Laplace distribution differential entropy",
    example = "SELECT sr_laplace_entropy(0.0, 1.0)"
)]
fn sr_laplace_entropy(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(laplace("sr_laplace_entropy", location, scale)?.entropy())
}

/// 拉普拉斯分布偏度（恒为 0）。
#[duck_scalar_function(
    description = "Laplace distribution skewness (always 0)",
    example = "SELECT sr_laplace_skewness(0.0, 1.0)"
)]
fn sr_laplace_skewness(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(laplace("sr_laplace_skewness", location, scale)?.skewness())
}

/// 拉普拉斯分布支撑域下确界（-inf，无界）。
#[duck_scalar_function(
    description = "Laplace distribution support minimum (negative infinity)",
    example = "SELECT sr_laplace_min(0.0, 1.0)"
)]
fn sr_laplace_min(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_min", location, scale)?.min()))
}

/// 拉普拉斯分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Laplace distribution support maximum (positive infinity)",
    example = "SELECT sr_laplace_max(0.0, 1.0)"
)]
fn sr_laplace_max(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_max", location, scale)?.max()))
}

/// 拉普拉斯分布中位数。
#[duck_scalar_function(
    description = "Laplace distribution median",
    example = "SELECT sr_laplace_median(0.0, 1.0)"
)]
fn sr_laplace_median(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(laplace("sr_laplace_median", location, scale)?.median()))
}

/// 拉普拉斯分布众数。
#[duck_scalar_function(
    description = "Laplace distribution mode",
    example = "SELECT sr_laplace_mode(0.0, 1.0)"
)]
fn sr_laplace_mode(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(laplace("sr_laplace_mode", location, scale)?.mode())
}

// ---------------------------------------------------------------------------
// Gumbel（均矩与域）
// ---------------------------------------------------------------------------

/// Gumbel（I 型极值）分布均值。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution mean",
    example = "SELECT sr_gumbel_mean(0.0, 1.0)"
)]
fn sr_gumbel_mean(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(gumbel("sr_gumbel_mean", location, scale)?.mean())
}

/// Gumbel 分布方差。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution variance",
    example = "SELECT sr_gumbel_variance(0.0, 1.0)"
)]
fn sr_gumbel_variance(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(gumbel("sr_gumbel_variance", location, scale)?.variance())
}

/// Gumbel 分布标准差。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution standard deviation",
    example = "SELECT sr_gumbel_std_dev(0.0, 1.0)"
)]
fn sr_gumbel_std_dev(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(gumbel("sr_gumbel_std_dev", location, scale)?.std_dev())
}

/// Gumbel 分布微分熵。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution differential entropy",
    example = "SELECT sr_gumbel_entropy(0.0, 1.0)"
)]
fn sr_gumbel_entropy(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(gumbel("sr_gumbel_entropy", location, scale)?.entropy())
}

/// Gumbel 分布偏度。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution skewness",
    example = "SELECT sr_gumbel_skewness(0.0, 1.0)"
)]
fn sr_gumbel_skewness(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(gumbel("sr_gumbel_skewness", location, scale)?.skewness())
}

/// Gumbel 分布支撑域下确界（-inf，无界）。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution support minimum (negative infinity)",
    example = "SELECT sr_gumbel_min(0.0, 1.0)"
)]
fn sr_gumbel_min(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_min", location, scale)?.min()))
}

/// Gumbel 分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution support maximum (positive infinity)",
    example = "SELECT sr_gumbel_max(0.0, 1.0)"
)]
fn sr_gumbel_max(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_max", location, scale)?.max()))
}

/// Gumbel 分布中位数。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution median",
    example = "SELECT sr_gumbel_median(0.0, 1.0)"
)]
fn sr_gumbel_median(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_median", location, scale)?.median()))
}

/// Gumbel 分布众数（statrs 返回裸 f64）。
#[duck_scalar_function(
    description = "Gumbel (type-I extreme value) distribution mode",
    example = "SELECT sr_gumbel_mode(0.0, 1.0)"
)]
fn sr_gumbel_mode(location: f64, scale: f64) -> DuckOptionResult<f64> {
    Ok(Some(gumbel("sr_gumbel_mode", location, scale)?.mode()))
}

// ---------------------------------------------------------------------------
// Levy（均矩与域）
// ---------------------------------------------------------------------------

/// Lévy 分布均值（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Lévy distribution mean (undefined)",
    example = "SELECT sr_levy_mean(0.0, 1.0)"
)]
fn sr_levy_mean(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(levy("sr_levy_mean", mu, c)?.mean())
}

/// Lévy 分布方差（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Lévy distribution variance (undefined)",
    example = "SELECT sr_levy_variance(0.0, 1.0)"
)]
fn sr_levy_variance(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(levy("sr_levy_variance", mu, c)?.variance())
}

/// Lévy 分布标准差（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Lévy distribution standard deviation (undefined)",
    example = "SELECT sr_levy_std_dev(0.0, 1.0)"
)]
fn sr_levy_std_dev(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(levy("sr_levy_std_dev", mu, c)?.std_dev())
}

/// Lévy 分布微分熵。
#[duck_scalar_function(
    description = "Lévy distribution differential entropy",
    example = "SELECT sr_levy_entropy(0.0, 1.0)"
)]
fn sr_levy_entropy(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(levy("sr_levy_entropy", mu, c)?.entropy())
}

/// Lévy 分布偏度（不存在，返回 NULL）。
#[duck_scalar_function(
    description = "Lévy distribution skewness (undefined)",
    example = "SELECT sr_levy_skewness(0.0, 1.0)"
)]
fn sr_levy_skewness(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(levy("sr_levy_skewness", mu, c)?.skewness())
}

/// Lévy 分布支撑域下确界（mu）。
#[duck_scalar_function(
    description = "Lévy distribution support minimum (mu)",
    example = "SELECT sr_levy_min(0.0, 1.0)"
)]
fn sr_levy_min(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_min", mu, c)?.min()))
}

/// Lévy 分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Lévy distribution support maximum (positive infinity)",
    example = "SELECT sr_levy_max(0.0, 1.0)"
)]
fn sr_levy_max(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_max", mu, c)?.max()))
}

/// Lévy 分布中位数。
#[duck_scalar_function(
    description = "Lévy distribution median",
    example = "SELECT sr_levy_median(0.0, 1.0)"
)]
fn sr_levy_median(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(Some(levy("sr_levy_median", mu, c)?.median()))
}

/// Lévy 分布众数。
#[duck_scalar_function(
    description = "Lévy distribution mode",
    example = "SELECT sr_levy_mode(0.0, 1.0)"
)]
fn sr_levy_mode(mu: f64, c: f64) -> DuckOptionResult<f64> {
    Ok(levy("sr_levy_mode", mu, c)?.mode())
}

// ---------------------------------------------------------------------------
// Uniform（均矩与域）
// ---------------------------------------------------------------------------

/// 连续均匀分布均值。
#[duck_scalar_function(
    description = "Continuous uniform distribution mean",
    example = "SELECT sr_uniform_mean(0.0, 1.0)"
)]
fn sr_uniform_mean(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(uniform("sr_uniform_mean", min, max)?.mean())
}

/// 连续均匀分布方差。
#[duck_scalar_function(
    description = "Continuous uniform distribution variance",
    example = "SELECT sr_uniform_variance(0.0, 1.0)"
)]
fn sr_uniform_variance(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(uniform("sr_uniform_variance", min, max)?.variance())
}

/// 连续均匀分布标准差。
#[duck_scalar_function(
    description = "Continuous uniform distribution standard deviation",
    example = "SELECT sr_uniform_std_dev(0.0, 1.0)"
)]
fn sr_uniform_std_dev(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(uniform("sr_uniform_std_dev", min, max)?.std_dev())
}

/// 连续均匀分布微分熵。
#[duck_scalar_function(
    description = "Continuous uniform distribution differential entropy",
    example = "SELECT sr_uniform_entropy(0.0, 1.0)"
)]
fn sr_uniform_entropy(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(uniform("sr_uniform_entropy", min, max)?.entropy())
}

/// 连续均匀分布偏度（恒为 0）。
#[duck_scalar_function(
    description = "Continuous uniform distribution skewness (always 0)",
    example = "SELECT sr_uniform_skewness(0.0, 1.0)"
)]
fn sr_uniform_skewness(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(uniform("sr_uniform_skewness", min, max)?.skewness())
}

/// 连续均匀分布支撑域下确界（min）。
#[duck_scalar_function(
    description = "Continuous uniform distribution support minimum (min)",
    example = "SELECT sr_uniform_min(0.0, 1.0)"
)]
fn sr_uniform_min(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_min", min, max)?.min()))
}

/// 连续均匀分布支撑域上确界（max）。
#[duck_scalar_function(
    description = "Continuous uniform distribution support maximum (max)",
    example = "SELECT sr_uniform_max(0.0, 1.0)"
)]
fn sr_uniform_max(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_max", min, max)?.max()))
}

/// 连续均匀分布中位数。
#[duck_scalar_function(
    description = "Continuous uniform distribution median",
    example = "SELECT sr_uniform_median(0.0, 1.0)"
)]
fn sr_uniform_median(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(Some(uniform("sr_uniform_median", min, max)?.median()))
}

/// 连续均匀分布众数。
#[duck_scalar_function(
    description = "Continuous uniform distribution mode",
    example = "SELECT sr_uniform_mode(0.0, 1.0)"
)]
fn sr_uniform_mode(min: f64, max: f64) -> DuckOptionResult<f64> {
    Ok(uniform("sr_uniform_mode", min, max)?.mode())
}

// ---------------------------------------------------------------------------
// Triangular（均矩与域）
// ---------------------------------------------------------------------------

/// 三角分布均值。
#[duck_scalar_function(
    description = "Triangular distribution mean",
    example = "SELECT sr_triangular_mean(0.0, 2.0, 1.0)"
)]
fn sr_triangular_mean(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(triangular("sr_triangular_mean", min, max, mode)?.mean())
}

/// 三角分布方差。
#[duck_scalar_function(
    description = "Triangular distribution variance",
    example = "SELECT sr_triangular_variance(0.0, 2.0, 1.0)"
)]
fn sr_triangular_variance(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(triangular("sr_triangular_variance", min, max, mode)?.variance())
}

/// 三角分布标准差。
#[duck_scalar_function(
    description = "Triangular distribution standard deviation",
    example = "SELECT sr_triangular_std_dev(0.0, 2.0, 1.0)"
)]
fn sr_triangular_std_dev(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(triangular("sr_triangular_std_dev", min, max, mode)?.std_dev())
}

/// 三角分布微分熵。
#[duck_scalar_function(
    description = "Triangular distribution differential entropy",
    example = "SELECT sr_triangular_entropy(0.0, 2.0, 1.0)"
)]
fn sr_triangular_entropy(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(triangular("sr_triangular_entropy", min, max, mode)?.entropy())
}

/// 三角分布偏度。
#[duck_scalar_function(
    description = "Triangular distribution skewness",
    example = "SELECT sr_triangular_skewness(0.0, 2.0, 1.0)"
)]
fn sr_triangular_skewness(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(triangular("sr_triangular_skewness", min, max, mode)?.skewness())
}

/// 三角分布支撑域下确界（min）。
#[duck_scalar_function(
    description = "Triangular distribution support minimum (min)",
    example = "SELECT sr_triangular_min(0.0, 2.0, 1.0)"
)]
fn sr_triangular_min(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_min", min, max, mode)?.min()))
}

/// 三角分布支撑域上确界（max）。
#[duck_scalar_function(
    description = "Triangular distribution support maximum (max)",
    example = "SELECT sr_triangular_max(0.0, 2.0, 1.0)"
)]
fn sr_triangular_max(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_max", min, max, mode)?.max()))
}

/// 三角分布中位数。
#[duck_scalar_function(
    description = "Triangular distribution median",
    example = "SELECT sr_triangular_median(0.0, 2.0, 1.0)"
)]
fn sr_triangular_median(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(Some(triangular("sr_triangular_median", min, max, mode)?.median()))
}

/// 三角分布众数。
#[duck_scalar_function(
    description = "Triangular distribution mode",
    example = "SELECT sr_triangular_mode(0.0, 2.0, 1.0)"
)]
fn sr_triangular_mode(min: f64, max: f64, mode: f64) -> DuckOptionResult<f64> {
    Ok(triangular("sr_triangular_mode", min, max, mode)?.mode())
}

// ---------------------------------------------------------------------------
// StudentsT（均矩与域）
// ---------------------------------------------------------------------------

/// 学生 t 分布均值。
#[duck_scalar_function(
    description = "Student's t distribution mean",
    example = "SELECT sr_students_t_mean(0.0, 1.0, 2.0)"
)]
fn sr_students_t_mean(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(students_t("sr_students_t_mean", location, scale, freedom)?.mean())
}

/// 学生 t 分布方差。
#[duck_scalar_function(
    description = "Student's t distribution variance",
    example = "SELECT sr_students_t_variance(0.0, 1.0, 10.0)"
)]
fn sr_students_t_variance(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(students_t("sr_students_t_variance", location, scale, freedom)?.variance())
}

/// 学生 t 分布标准差。
#[duck_scalar_function(
    description = "Student's t distribution standard deviation",
    example = "SELECT sr_students_t_std_dev(0.0, 1.0, 10.0)"
)]
fn sr_students_t_std_dev(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(students_t("sr_students_t_std_dev", location, scale, freedom)?.std_dev())
}

/// 学生 t 分布微分熵。
#[duck_scalar_function(
    description = "Student's t distribution differential entropy",
    example = "SELECT sr_students_t_entropy(0.0, 1.0, 10.0)"
)]
fn sr_students_t_entropy(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(students_t("sr_students_t_entropy", location, scale, freedom)?.entropy())
}

/// 学生 t 分布偏度。
#[duck_scalar_function(
    description = "Student's t distribution skewness",
    example = "SELECT sr_students_t_skewness(0.0, 1.0, 10.0)"
)]
fn sr_students_t_skewness(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(students_t("sr_students_t_skewness", location, scale, freedom)?.skewness())
}

/// 学生 t 分布支撑域下确界（-inf，无界）。
#[duck_scalar_function(
    description = "Student's t distribution support minimum (negative infinity)",
    example = "SELECT sr_students_t_min(0.0, 1.0, 10.0)"
)]
fn sr_students_t_min(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_min", location, scale, freedom)?.min()))
}

/// 学生 t 分布支撑域上确界（+inf，无界）。
#[duck_scalar_function(
    description = "Student's t distribution support maximum (positive infinity)",
    example = "SELECT sr_students_t_max(0.0, 1.0, 10.0)"
)]
fn sr_students_t_max(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_max", location, scale, freedom)?.max()))
}

/// 学生 t 分布中位数。
#[duck_scalar_function(
    description = "Student's t distribution median",
    example = "SELECT sr_students_t_median(0.0, 1.0, 10.0)"
)]
fn sr_students_t_median(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(Some(students_t("sr_students_t_median", location, scale, freedom)?.median()))
}

/// 学生 t 分布众数。
#[duck_scalar_function(
    description = "Student's t distribution mode",
    example = "SELECT sr_students_t_mode(0.0, 1.0, 10.0)"
)]
fn sr_students_t_mode(location: f64, scale: f64, freedom: f64) -> DuckOptionResult<f64> {
    Ok(students_t("sr_students_t_mode", location, scale, freedom)?.mode())
}

// ---------------------------------------------------------------------------
// Dirac（均矩与域）
// ---------------------------------------------------------------------------

/// Dirac 退化分布均值（恒为 v）。
#[duck_scalar_function(
    description = "Dirac delta distribution mean (always v)",
    example = "SELECT sr_dirac_mean(1.0)"
)]
fn sr_dirac_mean(v: f64) -> DuckOptionResult<f64> {
    Ok(dirac("sr_dirac_mean", v)?.mean())
}

/// Dirac 退化分布方差（恒为 0）。
#[duck_scalar_function(
    description = "Dirac delta distribution variance (always 0)",
    example = "SELECT sr_dirac_variance(1.0)"
)]
fn sr_dirac_variance(v: f64) -> DuckOptionResult<f64> {
    Ok(dirac("sr_dirac_variance", v)?.variance())
}

/// Dirac 退化分布标准差（恒为 0）。
#[duck_scalar_function(
    description = "Dirac delta distribution standard deviation (always 0)",
    example = "SELECT sr_dirac_std_dev(1.0)"
)]
fn sr_dirac_std_dev(v: f64) -> DuckOptionResult<f64> {
    Ok(dirac("sr_dirac_std_dev", v)?.std_dev())
}

/// Dirac 退化分布微分熵。
#[duck_scalar_function(
    description = "Dirac delta distribution differential entropy",
    example = "SELECT sr_dirac_entropy(1.0)"
)]
fn sr_dirac_entropy(v: f64) -> DuckOptionResult<f64> {
    Ok(dirac("sr_dirac_entropy", v)?.entropy())
}

/// Dirac 退化分布偏度（恒为 0）。
#[duck_scalar_function(
    description = "Dirac delta distribution skewness (always 0)",
    example = "SELECT sr_dirac_skewness(1.0)"
)]
fn sr_dirac_skewness(v: f64) -> DuckOptionResult<f64> {
    Ok(dirac("sr_dirac_skewness", v)?.skewness())
}

/// Dirac 退化分布支撑域下确界（v）。
#[duck_scalar_function(
    description = "Dirac delta distribution support minimum (v)",
    example = "SELECT sr_dirac_min(1.0)"
)]
fn sr_dirac_min(v: f64) -> DuckOptionResult<f64> {
    Ok(Some(dirac("sr_dirac_min", v)?.min()))
}

/// Dirac 退化分布支撑域上确界（v）。
#[duck_scalar_function(
    description = "Dirac delta distribution support maximum (v)",
    example = "SELECT sr_dirac_max(1.0)"
)]
fn sr_dirac_max(v: f64) -> DuckOptionResult<f64> {
    Ok(Some(dirac("sr_dirac_max", v)?.max()))
}

/// Dirac 退化分布中位数（恒为 v）。
#[duck_scalar_function(
    description = "Dirac delta distribution median (always v)",
    example = "SELECT sr_dirac_median(1.0)"
)]
fn sr_dirac_median(v: f64) -> DuckOptionResult<f64> {
    Ok(Some(dirac("sr_dirac_median", v)?.median()))
}

/// Dirac 退化分布众数（恒为 v）。
#[duck_scalar_function(
    description = "Dirac delta distribution mode (always v)",
    example = "SELECT sr_dirac_mode(1.0)"
)]
fn sr_dirac_mode(v: f64) -> DuckOptionResult<f64> {
    Ok(dirac("sr_dirac_mode", v)?.mode())
}
