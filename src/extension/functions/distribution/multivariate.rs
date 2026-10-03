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

use crate::extension::functions::as_u64;

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

/// 多项分布概率质量 P(X = counts)：counts 为 LIST(BIGINT)，与概率向量 p 等长。
#[duck_scalar_function(
    description = "Multinomial probability mass of a count LIST(BIGINT), given category probabilities and the trial count",
    example = "SELECT sr_multinomial_pmf([1.0, 2.0], 3.0, [1, 2])"
)]
fn sr_multinomial_pmf(p: Vec<f64>, n: f64, counts: Vec<i64>) -> DuckOptionResult<f64> {
    let dist = multinomial("sr_multinomial_pmf", p, n)?;
    let counts: Vec<u64> = counts
        .iter()
        .map(|c| as_u64("sr_multinomial_pmf", *c as f64))
        .collect::<Result<_, _>>()?;
    Ok(Some(dist.pmf(&DVector::from_vec(counts))))
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
