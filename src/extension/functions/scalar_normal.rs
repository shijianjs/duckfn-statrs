// ============================================================================
// statrs::distribution 的首个包装：正态分布的 pdf / cdf / 分位数
//
// 分布类函数与统计量不同：参数本身可以非法（std_dev <= 0 时 `Normal::new` 返回 Err），
// 这属于「调用写错了」而不是「结果为空」，所以报成查询错误而不是 NULL。
// 每个标量行都要重新构造一次分布 —— statrs 的分布对象是廉价的结构体（几个 f64），
// 直接在建好的对象上求值即可，不需要额外的缓存层。
//
// The first wrapping of statrs::distribution: the normal pdf / cdf / quantile.
//
// Unlike the summary statistics, a distribution can be handed invalid parameters (std_dev <= 0
// makes `Normal::new` return an Err) — that is a bad call rather than an empty result, so it
// fails the query instead of yielding NULL. Constructing the distribution per row is fine: the
// statrs objects are cheap structs of a few f64s.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

/// 由 (mean, std_dev) 构造正态分布，非法参数报成查询错误。
///
/// Builds the normal distribution, surfacing invalid parameters as a query error.
fn normal(fn_name: &str, mean: f64, std_dev: f64) -> Result<Normal, ExtensionError> {
    Normal::new(mean, std_dev).map_err(|error| duck_error(format!("{fn_name}: {error}")))
}

/// 正态概率密度 `N(mean, std_dev)` 在 x 处的取值。
///
/// ```sql
/// SELECT stat_normal_pdf(0.0, 0.0, 1.0);  -- 0.3989422804014327
/// SELECT stat_normal_pdf(0.0, 0.0, -1.0); -- 报错（std_dev 必须为正）
/// ```
#[duck_scalar_function(
    description = "Normal (Gaussian) probability density at x, given mean and standard deviation",
    example = "SELECT stat_normal_pdf(0.0, 0.0, 1.0)"
)]
fn stat_normal_pdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    let normal = normal("stat_normal_pdf", mean, std_dev)?;
    Ok(Some(normal.pdf(x)))
}

/// 正态累积分布函数 P(X <= x)。
///
/// ```sql
/// SELECT stat_normal_cdf(0.0, 0.0, 1.0);  -- 0.5
/// SELECT stat_normal_cdf(1.96, 0.0, 1.0); -- 0.9750021048517796
/// ```
#[duck_scalar_function(
    description = "Normal (Gaussian) cumulative distribution function P(X <= x)",
    example = "SELECT stat_normal_cdf(1.96, 0.0, 1.0)"
)]
fn stat_normal_cdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    let normal = normal("stat_normal_cdf", mean, std_dev)?;
    Ok(Some(normal.cdf(x)))
}

/// 正态分位数函数（CDF 的反函数）：给概率 p，返回 x 使 `cdf(x) = p`。
/// p 须在 `[0, 1]` 内，越界报查询错误（statrs 会把它钳到端点，掩盖写错的参数）。
///
/// ```sql
/// SELECT stat_normal_quantile(0.975, 0.0, 1.0);  -- 1.959963984540054
/// ```
#[duck_scalar_function(
    description = "Normal (Gaussian) quantile function: the x whose CDF equals p, for p in [0, 1]",
    comment = "A probability outside [0, 1] is a query error rather than a clamped endpoint",
    example = "SELECT stat_normal_quantile(0.975, 0.0, 1.0)"
)]
fn stat_normal_quantile(p: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    if !(0.0..=1.0).contains(&p) {
        return Err(duck_error(format!(
            "stat_normal_quantile: the probability must be within [0, 1], got {p}"
        )));
    }
    let normal = normal("stat_normal_quantile", mean, std_dev)?;
    Ok(Some(normal.inverse_cdf(p)))
}
