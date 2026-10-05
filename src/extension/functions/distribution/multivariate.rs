// ============================================================================
// 多元分布（statrs::distribution 的 multivariate_normal / dirichlet / multinomial /
// multivariate_students_t）
//
// statrs 的向量构造入口（new(mean: Vec<f64>, cov: Vec<f64>) 这类）落在 Dyn 维度上，
// 一个包装就覆盖任意维：mean/x/alpha 是 LIST(DOUBLE)，协方差/尺度矩阵是行主序摊平的
// LIST(DOUBLE)（长度必须等于 dim²，由 statrs 自查并报构造错误）。multinomial 的计数
// 向量是 LIST(BIGINT)。
//
// The multivariate distributions: statrs' Vec-based constructors hand back the Dyn
// dimension, so one wrapper covers every dimension — vectors are LIST(DOUBLE),
// matrices are LIST(DOUBLE) flattened row-major (length dim², checked by statrs),
// multinomial counts are LIST(BIGINT).
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use nalgebra::DVector;
use quack_rs::error::ExtensionError;
use statrs::distribution::{
    Continuous, Dirichlet, Discrete, Multinomial, MultivariateNormal, MultivariateStudent,
};
use statrs::statistics::{Max, MeanN, Min, Mode, VarianceN};

use crate::extension::functions::as_u64;

/// 协方差/尺度/计数矩阵按行主序摊平成 LIST(DOUBLE)。statrs 返回的是 nalgebra 矩阵，
/// SQL 侧的统一形状（与构造参数同构）。
fn row_major(m: nalgebra::DMatrix<f64>) -> Vec<f64> {
    let mut out = Vec::with_capacity(m.nrows() * m.ncols());
    for i in 0..m.nrows() {
        for j in 0..m.ncols() {
            out.push(m[(i, j)]);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// MultivariateNormal（statrs::distribution::MultivariateNormal）
// ---------------------------------------------------------------------------

fn multivariate_normal(
    name: &str,
    mean: Vec<f64>,
    cov: Vec<f64>,
) -> Result<MultivariateNormal<nalgebra::Dyn>, ExtensionError> {
    // 同 multivariate_student：cov 长度不等于 dim² 时 nalgebra 的 from_vec 会 panic，
    // 先在 SQL 边界拦下来。
    let dim = mean.len();
    if cov.len() != dim * dim {
        return Err(duck_error(format!(
            "{name}: expected {} entries for a {dim}x{dim} covariance matrix (row-major), got {}",
            dim * dim,
            cov.len()
        )));
    }
    MultivariateNormal::new(mean, cov).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 多元正态概率密度：x 与 mean 等长，cov 是行主序摊平的 dim² 向量。
#[duck_scalar_function(
    description = "Multivariate normal probability density of x (LIST) given the mean and a row-major flattened covariance matrix",
    example = "SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_pdf(
    x: Vec<f64>,
    mean: Vec<f64>,
    cov: Vec<f64>,
) -> DuckOptionResult<f64> {
    let dist = multivariate_normal("sr_multivariate_normal_pdf", mean, cov)?;
    Ok(Some(dist.pdf(&DVector::from_vec(x))))
}

/// 多元正态对数密度：与 pdf(x).ln() 等价（statrs 的实现走 pdf_const 的 ln，避免中间溢出）。
#[duck_scalar_function(
    description = "Log probability density of x under a multivariate normal distribution with the given mean and a row-major flattened covariance matrix",
    example = "SELECT sr_multivariate_normal_ln_pdf([1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_ln_pdf(
    x: Vec<f64>,
    mean: Vec<f64>,
    cov: Vec<f64>,
) -> DuckOptionResult<f64> {
    let dist = multivariate_normal("sr_multivariate_normal_ln_pdf", mean, cov)?;
    Ok(Some(dist.ln_pdf(&DVector::from_vec(x))))
}

/// 多元正态定义域下界（全 -inf 向量；无参数构造错误之外恒有定义）。
#[duck_scalar_function(
    description = "Lower bound of the multivariate normal support: a vector of -inf with the dimension of the mean",
    example = "SELECT sr_multivariate_normal_min([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_min(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_normal("sr_multivariate_normal_min", mean, cov)?;
    Ok(Some(dist.min().iter().copied().collect()))
}

/// 多元正态定义域上界（全 +inf 向量）。
#[duck_scalar_function(
    description = "Upper bound of the multivariate normal support: a vector of +inf with the dimension of the mean",
    example = "SELECT sr_multivariate_normal_max([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_max(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_normal("sr_multivariate_normal_max", mean, cov)?;
    Ok(Some(dist.max().iter().copied().collect()))
}

// ---------------------------------------------------------------------------
// Dirichlet（statrs::distribution::Dirichlet）
// ---------------------------------------------------------------------------

fn dirichlet(name: &str, alpha: Vec<f64>) -> Result<Dirichlet<nalgebra::Dyn>, ExtensionError> {
    Dirichlet::new(alpha).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// Dirichlet 概率密度（单纯形上的点 x，与 alpha 等长）。
#[duck_scalar_function(
    description = "Dirichlet probability density of x on the simplex, given the concentration vector alpha",
    example = "SELECT sr_dirichlet_pdf([0.5, 0.5], [1.0, 1.0])"
)]
fn sr_dirichlet_pdf(x: Vec<f64>, alpha: Vec<f64>) -> DuckOptionResult<f64> {
    let dist = dirichlet("sr_dirichlet_pdf", alpha)?;
    Ok(Some(dist.pdf(&DVector::from_vec(x))))
}

/// Dirichlet 熵。
#[duck_scalar_function(
    description = "Differential entropy of the Dirichlet distribution with concentrations alpha",
    example = "SELECT sr_dirichlet_entropy([1.0, 1.0])"
)]
fn sr_dirichlet_entropy(alpha: Vec<f64>) -> DuckOptionResult<f64> {
    let dist = dirichlet("sr_dirichlet_entropy", alpha)?;
    Ok(dist.entropy())
}

/// Dirichlet 对数密度：x 的每个分量须在 (0, 1) 且和为 1（容差 1e-4），否则 statrs
/// 断言失败、经 duckfn 的 panic 处理变成查询错误。
#[duck_scalar_function(
    description = "Log probability density of x on the simplex, given the concentration vector alpha; x must lie in (0,1)^k and sum to 1",
    example = "SELECT sr_dirichlet_ln_pdf([0.1, 0.2, 0.3, 0.4], [0.1, 0.3, 0.5, 0.8])"
)]
fn sr_dirichlet_ln_pdf(x: Vec<f64>, alpha: Vec<f64>) -> DuckOptionResult<f64> {
    let dist = dirichlet("sr_dirichlet_ln_pdf", alpha)?;
    Ok(Some(dist.ln_pdf(&DVector::from_vec(x))))
}

/// Dirichlet 均值向量：alpha_i / alpha_0（alpha_0 为浓度和）。
#[duck_scalar_function(
    description = "Mean vector of the Dirichlet distribution (alpha_i / alpha_0) given the concentration vector alpha",
    example = "SELECT sr_dirichlet_mean([1.0, 2.0, 3.0, 4.0])"
)]
fn sr_dirichlet_mean(alpha: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = dirichlet("sr_dirichlet_mean", alpha)?;
    Ok(dist.mean().map(|v| v.iter().copied().collect()))
}

/// Dirichlet 协方差矩阵（行主序摊平成 LIST(DOUBLE)）。
#[duck_scalar_function(
    description = "Covariance matrix of the Dirichlet distribution (row-major flattened LIST) given the concentration vector alpha",
    example = "SELECT sr_dirichlet_variance([1.0, 2.0])"
)]
fn sr_dirichlet_variance(alpha: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = dirichlet("sr_dirichlet_variance", alpha)?;
    Ok(dist.variance().map(row_major))
}

// ---------------------------------------------------------------------------
// Multinomial（statrs::distribution::Multinomial）
// ---------------------------------------------------------------------------

fn multinomial(
    name: &str,
    p: Vec<f64>,
    n: f64,
) -> Result<Multinomial<nalgebra::Dyn>, ExtensionError> {
    let n = as_u64(name, n)?;
    Multinomial::new(p, n).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// LIST(BIGINT) 计数向量 → u64 向量（pmf / ln_pmf 共用；负数或非整数是调用错误）。
fn counts_to_u64(name: &str, counts: &[i64]) -> Result<Vec<u64>, ExtensionError> {
    counts
        .iter()
        .map(|c| as_u64(name, *c as f64))
        .collect()
}

/// 多项分布概率质量 P(X = counts)：counts 为 LIST(BIGINT)，与概率向量 p 等长。
#[duck_scalar_function(
    description = "Multinomial probability mass of a count LIST(BIGINT), given category probabilities and the trial count",
    example = "SELECT sr_multinomial_pmf([1.0, 2.0], 3.0, [1, 2])"
)]
fn sr_multinomial_pmf(p: Vec<f64>, n: f64, counts: Vec<i64>) -> DuckOptionResult<f64> {
    let dist = multinomial("sr_multinomial_pmf", p, n)?;
    let counts = counts_to_u64("sr_multinomial_pmf", &counts)?;
    Ok(Some(dist.pmf(&DVector::from_vec(counts))))
}

/// 多项分布对数概率质量：counts 之和 ≠ n 时是合法结果 -inf（概率 0 的对数）。
#[duck_scalar_function(
    description = "Log probability mass of a count LIST(BIGINT) under the multinomial distribution; -inf when the counts do not sum to n",
    example = "SELECT sr_multinomial_ln_pmf([0.5, 0.5], 2000.0, [1000, 1000])"
)]
fn sr_multinomial_ln_pmf(p: Vec<f64>, n: f64, counts: Vec<i64>) -> DuckOptionResult<f64> {
    let dist = multinomial("sr_multinomial_ln_pmf", p, n)?;
    let counts = counts_to_u64("sr_multinomial_ln_pmf", &counts)?;
    Ok(Some(dist.ln_pmf(&DVector::from_vec(counts))))
}

/// 多项分布均值向量：n * p_i。
#[duck_scalar_function(
    description = "Mean vector of the multinomial distribution (n * p_i) given category probabilities and the trial count",
    example = "SELECT sr_multinomial_mean([0.3, 0.7], 5.0)"
)]
fn sr_multinomial_mean(p: Vec<f64>, n: f64) -> DuckOptionResult<Vec<f64>> {
    let dist = multinomial("sr_multinomial_mean", p, n)?;
    Ok(dist.mean().map(|v| v.iter().copied().collect()))
}

/// 多项分布协方差矩阵（行主序摊平成 LIST(DOUBLE)，长度为 k²）。
#[duck_scalar_function(
    description = "Covariance matrix of the multinomial distribution (row-major flattened LIST) given category probabilities and the trial count",
    example = "SELECT sr_multinomial_variance([0.1, 0.3, 0.6], 10.0)"
)]
fn sr_multinomial_variance(p: Vec<f64>, n: f64) -> DuckOptionResult<Vec<f64>> {
    let dist = multinomial("sr_multinomial_variance", p, n)?;
    Ok(dist.variance().map(row_major))
}

// ---------------------------------------------------------------------------
// MultivariateStudent（statrs::distribution::MultivariateStudent）
// ---------------------------------------------------------------------------

fn multivariate_student(
    name: &str,
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> Result<MultivariateStudent<nalgebra::Dyn>, ExtensionError> {
    // statrs 把摊平的 scale 交给 nalgebra 的 from_vec，长度不等于 dim² 会 panic；
    // 先在 SQL 边界拦下来，换成带函数名前缀的查询错误。
    let dim = location.len();
    if scale.len() != dim * dim {
        return Err(duck_error(format!(
            "{name}: expected {} entries for a {dim}x{dim} scale matrix (row-major), got {}",
            dim * dim,
            scale.len()
        )));
    }
    MultivariateStudent::new(location, scale, freedom).map_err(|e| duck_error(format!("{name}: {e}")))
}

/// 多元 t 概率密度：location 给维度，scale 是行主序摊平的 dim² 向量，freedom 是自由度。
#[duck_scalar_function(
    description = "Multivariate Student's t probability density of x, given location, a row-major flattened scale matrix and the degrees of freedom",
    example = "SELECT sr_multivariate_students_t_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)"
)]
fn sr_multivariate_students_t_pdf(
    x: Vec<f64>,
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<f64> {
    let dist = multivariate_student("sr_multivariate_students_t_pdf", location, scale, freedom)?;
    Ok(Some(dist.pdf(&DVector::from_vec(x))))
}

// ---------------------------------------------------------------------------
// 补充：MultivariateNormal / MultivariateStudent 的矩、熵、众数
// ---------------------------------------------------------------------------

/// 多元正态微分熵。
#[duck_scalar_function(
    description = "Differential entropy of the multivariate normal distribution given the mean and a row-major flattened covariance matrix",
    example = "SELECT sr_multivariate_normal_entropy([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_entropy(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<f64> {
    let dist = multivariate_normal("sr_multivariate_normal_entropy", mean, cov)?;
    Ok(dist.entropy())
}

/// 多元正态均值向量（LIST(DOUBLE)）。
#[duck_scalar_function(
    description = "Mean vector of the multivariate normal distribution given the mean and a row-major flattened covariance matrix",
    example = "SELECT sr_multivariate_normal_mean([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_mean(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_normal("sr_multivariate_normal_mean", mean, cov)?;
    Ok(dist.mean().map(|v| v.iter().copied().collect::<Vec<f64>>()))
}

/// 多元正态协方差矩阵（行主序摊平成 LIST(DOUBLE)）。
#[duck_scalar_function(
    description = "Covariance matrix of the multivariate normal distribution (row-major flattened LIST) given the mean and a row-major flattened covariance matrix",
    example = "SELECT sr_multivariate_normal_variance([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_variance(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_normal("sr_multivariate_normal_variance", mean, cov)?;
    Ok(dist.variance().map(row_major))
}

/// 多元正态精度矩阵（协方差的逆，行主序摊平成 LIST(DOUBLE)）。
///
/// statrs 在构造时用 Cholesky 分解的逆算出它并缓存（`MultivariateNormal::precision`），
/// 正是密度指数项 `-(1/2)·(x-μ)ᵀ·Σ⁻¹·(x-μ)` 里的那个 Σ⁻¹。与 `variance` 不同，这不是把
/// 构造参数原样读回：DuckDB 没有矩阵求逆，SQL 侧算不出来，所以是独立出口。
#[duck_scalar_function(
    description = "Precision matrix (inverse of the covariance matrix, row-major flattened LIST) of the multivariate normal distribution",
    example = "SELECT sr_multivariate_normal_precision([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_precision(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_normal("sr_multivariate_normal_precision", mean, cov)?;
    Ok(Some(row_major(dist.precision().clone_owned())))
}

/// 多元正态众数向量（LIST(DOUBLE)）。
#[duck_scalar_function(
    description = "Mode vector of the multivariate normal distribution given the mean and a row-major flattened covariance matrix",
    example = "SELECT sr_multivariate_normal_mode([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])"
)]
fn sr_multivariate_normal_mode(mean: Vec<f64>, cov: Vec<f64>) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_normal("sr_multivariate_normal_mode", mean, cov)?;
    Ok(Some(dist.mode().iter().copied().collect::<Vec<f64>>()))
}

/// 多元 t 众数向量（LIST(DOUBLE)）。
#[duck_scalar_function(
    description = "Mode vector of the multivariate Student's t distribution given location, a row-major flattened scale matrix and the degrees of freedom",
    example = "SELECT sr_multivariate_students_t_mode([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)"
)]
fn sr_multivariate_students_t_mode(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_student("sr_multivariate_students_t_mode", location, scale, freedom)?;
    Ok(Some(dist.mode().iter().copied().collect()))
}

/// 多元 t 对数密度：自由度 → ∞ 时与多元正态的 ln_pdf 收敛到同一值。
#[duck_scalar_function(
    description = "Log probability density of x under a multivariate Student's t distribution, given location, a row-major flattened scale matrix and the degrees of freedom",
    example = "SELECT sr_multivariate_students_t_ln_pdf([1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 4.0)"
)]
fn sr_multivariate_students_t_ln_pdf(
    x: Vec<f64>,
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<f64> {
    let dist = multivariate_student("sr_multivariate_students_t_ln_pdf", location, scale, freedom)?;
    Ok(Some(dist.ln_pdf(&DVector::from_vec(x))))
}

/// 多元 t 均值向量：自由度 ≤ 1 时无定义（statrs 返回 None → SQL NULL）。
#[duck_scalar_function(
    description = "Mean vector of the multivariate Student's t distribution; NULL when the degrees of freedom is at most 1",
    example = "SELECT sr_multivariate_students_t_mean([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 2.0)"
)]
fn sr_multivariate_students_t_mean(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_student("sr_multivariate_students_t_mean", location, scale, freedom)?;
    Ok(dist.mean().map(|v| v.iter().copied().collect()))
}

/// 多元 t 协方差矩阵：scale · ν/(ν-2)，自由度 ≤ 2 时无定义 → NULL；行主序摊平。
#[duck_scalar_function(
    description = "Covariance matrix (row-major flattened LIST) of the multivariate Student's t distribution, scale * nu/(nu-2); NULL when the degrees of freedom is at most 2",
    example = "SELECT sr_multivariate_students_t_variance([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)"
)]
fn sr_multivariate_students_t_variance(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_student(
        "sr_multivariate_students_t_variance",
        location,
        scale,
        freedom,
    )?;
    Ok(dist.variance().map(row_major))
}

/// 多元 t 定义域下界（全 -inf 向量，支撑与多元正态相同）。
#[duck_scalar_function(
    description = "Lower bound of the multivariate Student's t support: a vector of -inf with the dimension of the location",
    example = "SELECT sr_multivariate_students_t_min([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)"
)]
fn sr_multivariate_students_t_min(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_student("sr_multivariate_students_t_min", location, scale, freedom)?;
    Ok(Some(dist.min().iter().copied().collect()))
}

/// 多元 t 定义域上界（全 +inf 向量）。
#[duck_scalar_function(
    description = "Upper bound of the multivariate Student's t support: a vector of +inf with the dimension of the location",
    example = "SELECT sr_multivariate_students_t_max([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)"
)]
fn sr_multivariate_students_t_max(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_student("sr_multivariate_students_t_max", location, scale, freedom)?;
    Ok(Some(dist.max().iter().copied().collect()))
}

/// 多元 t 精度矩阵（尺度矩阵的逆，行主序摊平成 LIST(DOUBLE)）。
///
/// 与 `sr_multivariate_normal_precision` 同理，是 statrs 构造时算好缓存的 Σ⁻¹
/// （`MultivariateStudent::precision`），不是把 scale 参数读回。注意它与
/// `variance` 不同：variance 是 `scale·ν/(ν-2)`，精度矩阵就是 `scale⁻¹`，两者互为倒数关系
/// 的不同对象 —— 想复原尺度矩阵用 variance，想直接拿逆用本函数。
#[duck_scalar_function(
    description = "Precision matrix (inverse of the scale matrix, row-major flattened LIST) of the multivariate Student's t distribution",
    example = "SELECT sr_multivariate_students_t_precision([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)"
)]
fn sr_multivariate_students_t_precision(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
) -> DuckOptionResult<Vec<f64>> {
    let dist = multivariate_student(
        "sr_multivariate_students_t_precision",
        location,
        scale,
        freedom,
    )?;
    Ok(Some(row_major(dist.precision().clone_owned())))
}
