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
