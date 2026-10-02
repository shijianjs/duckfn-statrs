// ============================================================================
// statrs::distribution 的首个包装：正态分布的 pdf / cdf / 分位数
//
// 分布函数保持标量形态：它们本来就是逐行求值的（一个 x 进、一个概率/密度出），
// 聚合形态反而不符合语义。
//
// NULL 语义：任一参数为 NULL 的行在参数读取层短路成 NULL —— 没有 x 或没有参数，
// 就没有可算的值。参数**存在但非法**（std_dev <= 0、概率越界）不走 NULL：那是调用
// 写错了，报成查询错误，而不是把错误静默折成空。
//
// The first wrapping of statrs::distribution: the normal pdf / cdf / quantile.
//
// Distribution functions stay scalar: they are per-row evaluations by nature (an x in, a
// probability or density out) — an aggregate shape would not fit their semantics.
//
// NULL semantics: a row with a NULL in any argument is short-circuited to NULL by the argument
// reader — no x or no parameters, nothing to evaluate. A parameter that is *present but invalid*
// (std_dev <= 0, an out-of-range probability) is not NULL either: that is a bad call, so it fails
// the query instead of being silently folded into emptiness.
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
/// SELECT sr_normal_pdf(0.0, 0.0, 1.0);   -- 0.3989422804014327
/// SELECT sr_normal_pdf(0.0, 0.0, -1.0);  -- 报错（std_dev 必须为正）
/// ```
#[duck_scalar_function(
    description = "Normal (Gaussian) probability density at x, given mean and standard deviation",
    example = "SELECT sr_normal_pdf(0.0, 0.0, 1.0)"
)]
fn sr_normal_pdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    let normal = normal("sr_normal_pdf", mean, std_dev)?;
    Ok(Some(normal.pdf(x)))
}

/// 正态累积分布函数 P(X <= x)。
///
/// ```sql
/// SELECT sr_normal_cdf(0.0, 0.0, 1.0);    -- 0.5
/// SELECT sr_normal_cdf(1.96, 0.0, 1.0);   -- 0.9750021048529024
/// ```
#[duck_scalar_function(
    description = "Normal (Gaussian) cumulative distribution function P(X <= x)",
    example = "SELECT sr_normal_cdf(1.96, 0.0, 1.0)"
)]
fn sr_normal_cdf(x: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    let normal = normal("sr_normal_cdf", mean, std_dev)?;
    Ok(Some(normal.cdf(x)))
}

/// 正态分位数函数（CDF 的反函数）：给概率 p，返回 x 使 `cdf(x) = p`。
/// p 须在 `[0, 1]` 内，越界报查询错误（statrs 会把它钳到端点，掩盖写错的参数）。
///
/// ```sql
/// SELECT sr_normal_quantile(0.975, 0.0, 1.0);  -- 1.9599639845400538
/// ```
#[duck_scalar_function(
    description = "Normal (Gaussian) quantile function: the x whose CDF equals p, for p in [0, 1]",
    comment = "A probability outside [0, 1] is a query error rather than a clamped endpoint",
    example = "SELECT sr_normal_quantile(0.975, 0.0, 1.0)"
)]
fn sr_normal_quantile(p: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    if !(0.0..=1.0).contains(&p) {
        return Err(duck_error(format!(
            "sr_normal_quantile: the probability must be within [0, 1], got {p}"
        )));
    }
    let normal = normal("sr_normal_quantile", mean, std_dev)?;
    Ok(Some(normal.inverse_cdf(p)))
}
