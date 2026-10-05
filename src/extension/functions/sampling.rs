// ============================================================================
// statrs 的随机采样能力（rand::distr::Distribution impls + binomial 的 sampler 导出）
//
// statrs 0.19 给每个分布都实现了 rand 的 Distribution trait（清单核对：一元 27 个
// 全部有 f64 或 u64 实例；二项另有 BinomialAlgorithm / BinomialSampler 导出；多元
// 4 个都有 OVector 实例——Dirichlet / Multinomial / MultivariateStudent 也一样）。
//
// SQL 形态：`sr_sample_<分布>(<分布参数...>, k)` 抽 k 个点，k 是 BIGINT；连续分布与
// Bernoulli 出 LIST(DOUBLE)，离散分布用 statrs 的整数 impl、出 LIST(UBIGINT)
// （DiscreteUniform 是 LIST(BIGINT)）；多元分布出 LIST(LIST(DOUBLE))（Multinomial 的
// 计数是 LIST(UBIGINT)）。Empirical 的采样是聚合形态（样本列进、k 为 DuckFirst 常量、
// 随机子样本出 LIST）。随机源是线程级 ThreadRng —— 每次调用独立、不可复现；这是 SQL
// 即席采样的固有语义，statrs 侧同样由调用方提供 rng。因此这些标量函数一律标
// `volatile = true`（`#[duck_scalar_function(volatile = true)]`），注册期调用
// `duckdb_scalar_function_set_volatile`，DuckDB 不缓存、不复用相同参数的调用结果，也不会
// 把常量参数的调用折叠成只执行一次；唯一例外是 sr_sample_dirac —— statrs 的 Dirac 采样
// 忽略 rng、恒返回 v，输出不随调用变化，是确定性的，不需要 volatile。
//
// statrs' sampling surface: every distribution implements rand's Distribution trait
// (all 27 univariate ones have an f64 or u64 instance; Binomial additionally exports
// BinomialAlgorithm / BinomialSampler; all four multivariate ones have OVector
// instances — Dirichlet / Multinomial / MultivariateStudent included).
// `sr_sample_<dist>(params..., k BIGINT)` draws k points; continuous distributions and
// Bernoulli use the f64 instance (LIST(DOUBLE)), discrete ones the integer instance
// (LIST(UBIGINT), DiscreteUniform LIST(BIGINT)). Each call uses the thread rng, so every
// sampling scalar is marked `volatile = true`: registration calls
// `duckdb_scalar_function_set_volatile`, and DuckDB neither caches nor reuses calls with
// the same arguments. The one exception is sr_sample_dirac — statrs' Dirac sampling ignores
// the rng and always returns v, so its output is deterministic and needs no volatile.
// ============================================================================

use duckfn::{DuckFirst, DuckOptionResult, duck_aggregate_function, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use rand::RngExt;
use statrs::distribution::{
    Bernoulli, Beta, Binomial, BinomialAlgorithm, BinomialSampler, Categorical, Cauchy, Chi,
    ChiSquared, Dirac, Dirichlet, DiscreteUniform, Empirical, Erlang, Exp, FisherSnedecor, Gamma,
    Geometric, Gumbel, Hypergeometric, InverseGamma, Laplace, Levy, LogNormal, Multinomial,
    MultivariateNormal, MultivariateStudent, NegativeBinomial, Normal, Pareto, Poisson, StudentsT,
    Triangular, Uniform, Weibull,
};


fn take_len(fn_name: &str, k: i64) -> Result<usize, ExtensionError> {
    if k <= 0 {
        return Err(duck_error(format!(
            "{fn_name}: k must be a positive whole number, got {k}"
        )));
    }
    Ok(k as usize)
}

/// f64 通道的抽样。
fn sample_f64<D: rand::distr::Distribution<f64>>(
    fn_name: &str,
    dist: &D,
    k: i64,
) -> Result<Vec<f64>, ExtensionError> {
    let k = take_len(fn_name, k)?;
    let mut rng = rand::rng();
    Ok((0..k).map(|_| rng.sample(dist)).collect())
}

/// u64 通道的抽样（离散分布与 BinomialSampler），返回 statrs 原生的 u64 列表。
fn sample_u64<D: rand::distr::Distribution<u64>>(
    fn_name: &str,
    dist: &D,
    k: i64,
) -> Result<Vec<u64>, ExtensionError> {
    let k = take_len(fn_name, k)?;
    let mut rng = rand::rng();
    Ok((0..k).map(|_| rng.sample(dist)).collect())
}

/// i64 通道的抽样（DiscreteUniform 在 statrs 里的采样类型是 i64）。
fn sample_i64<D: rand::distr::Distribution<i64>>(
    fn_name: &str,
    dist: &D,
    k: i64,
) -> Result<Vec<i64>, ExtensionError> {
    let k = take_len(fn_name, k)?;
    let mut rng = rand::rng();
    Ok((0..k).map(|_| rng.sample(dist)).collect())
}

// ---------------------------------------------------------------------------
// 连续分布（statrs 的 Distribution<f64> impl）
// ---------------------------------------------------------------------------

/// 从 Beta(shape_a, shape_b) 抽 k 个点。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Beta(shape_a, shape_b) samples into a LIST(DOUBLE) using statrs' rand integration",
    example = "SELECT len(sr_sample_beta(2.0, 3.0, 10))"
)]
fn sr_sample_beta(shape_a: f64, shape_b: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Beta::new(shape_a, shape_b).map_err(|e| duck_error(format!("sr_sample_beta: {e}")))?;
    Ok(Some(sample_f64("sr_sample_beta", &d, k)?))
}

/// 从 Cauchy(location, scale) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Cauchy(location, scale) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_cauchy(0.0, 1.0, 10))"
)]
fn sr_sample_cauchy(location: f64, scale: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Cauchy::new(location, scale).map_err(|e| duck_error(format!("sr_sample_cauchy: {e}")))?;
    Ok(Some(sample_f64("sr_sample_cauchy", &d, k)?))
}

/// 从 Chi(freedom) 抽样（整数自由度）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Chi(UBIGINT freedom) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_chi(2, 10))"
)]
fn sr_sample_chi(freedom: u64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Chi::new(freedom).map_err(|e| duck_error(format!("sr_sample_chi: {e}")))?;
    Ok(Some(sample_f64("sr_sample_chi", &d, k)?))
}

/// 从 ChiSquared(freedom) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Chi-squared(freedom) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_chi_squared(2.0, 10))"
)]
fn sr_sample_chi_squared(freedom: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = ChiSquared::new(freedom).map_err(|e| duck_error(format!("sr_sample_chi_squared: {e}")))?;
    Ok(Some(sample_f64("sr_sample_chi_squared", &d, k)?))
}

/// 从 Dirac(v) 抽样（恒等于 v）。
#[duck_scalar_function(
    description = "Draw k Dirac(v) samples into a LIST(DOUBLE)",
    example = "SELECT sr_sample_dirac(3.0, 2)"
)]
fn sr_sample_dirac(v: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Dirac::new(v).map_err(|e| duck_error(format!("sr_sample_dirac: {e}")))?;
    Ok(Some(sample_f64("sr_sample_dirac", &d, k)?))
}

/// 从 Erlang(shape, rate) 抽样（整数 shape）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Erlang(UBIGINT shape, DOUBLE rate) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_erlang(2, 2.0, 10))"
)]
fn sr_sample_erlang(shape: u64, rate: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Erlang::new(shape, rate).map_err(|e| duck_error(format!("sr_sample_erlang: {e}")))?;
    Ok(Some(sample_f64("sr_sample_erlang", &d, k)?))
}

/// 从 Exp(rate)（指数分布）抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Exponential(rate) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_exp(2.0, 10))"
)]
fn sr_sample_exp(rate: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Exp::new(rate).map_err(|e| duck_error(format!("sr_sample_exp: {e}")))?;
    Ok(Some(sample_f64("sr_sample_exp", &d, k)?))
}

/// 从 FisherSnedecor(f1, f2)（F 分布）抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Fisher-Snedecor(f1, f2) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_fisher_snedecor(2.0, 3.0, 10))"
)]
fn sr_sample_fisher_snedecor(freedom_1: f64, freedom_2: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = FisherSnedecor::new(freedom_1, freedom_2)
        .map_err(|e| duck_error(format!("sr_sample_fisher_snedecor: {e}")))?;
    Ok(Some(sample_f64("sr_sample_fisher_snedecor", &d, k)?))
}

/// 从 Gamma(shape, rate) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Gamma(shape, rate) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_gamma(2.0, 2.0, 10))"
)]
fn sr_sample_gamma(shape: f64, rate: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Gamma::new(shape, rate).map_err(|e| duck_error(format!("sr_sample_gamma: {e}")))?;
    Ok(Some(sample_f64("sr_sample_gamma", &d, k)?))
}

/// 从 Gumbel(location, scale) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Gumbel(location, scale) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_gumbel(0.0, 1.0, 10))"
)]
fn sr_sample_gumbel(location: f64, scale: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Gumbel::new(location, scale).map_err(|e| duck_error(format!("sr_sample_gumbel: {e}")))?;
    Ok(Some(sample_f64("sr_sample_gumbel", &d, k)?))
}

/// 从 InverseGamma(shape, scale) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k InverseGamma(shape, scale) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_inverse_gamma(2.0, 2.0, 10))"
)]
fn sr_sample_inverse_gamma(shape: f64, scale: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = InverseGamma::new(shape, scale)
        .map_err(|e| duck_error(format!("sr_sample_inverse_gamma: {e}")))?;
    Ok(Some(sample_f64("sr_sample_inverse_gamma", &d, k)?))
}

/// 从 Laplace(location, scale) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Laplace(location, scale) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_laplace(0.0, 1.0, 10))"
)]
fn sr_sample_laplace(location: f64, scale: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Laplace::new(location, scale).map_err(|e| duck_error(format!("sr_sample_laplace: {e}")))?;
    Ok(Some(sample_f64("sr_sample_laplace", &d, k)?))
}

/// 从 Levy(mu, c) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Levy(mu, c) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_levy(0.0, 1.0, 10))"
)]
fn sr_sample_levy(mu: f64, c: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Levy::new(mu, c).map_err(|e| duck_error(format!("sr_sample_levy: {e}")))?;
    Ok(Some(sample_f64("sr_sample_levy", &d, k)?))
}

/// 从 LogNormal(location, scale) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k LogNormal(location, scale) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_log_normal(0.0, 1.0, 10))"
)]
fn sr_sample_log_normal(location: f64, scale: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = LogNormal::new(location, scale).map_err(|e| duck_error(format!("sr_sample_log_normal: {e}")))?;
    Ok(Some(sample_f64("sr_sample_log_normal", &d, k)?))
}

/// 从 Normal(mean, std_dev) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Normal(mean, std_dev) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_normal(0.0, 1.0, 10))"
)]
fn sr_sample_normal(mean: f64, std_dev: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Normal::new(mean, std_dev).map_err(|e| duck_error(format!("sr_sample_normal: {e}")))?;
    Ok(Some(sample_f64("sr_sample_normal", &d, k)?))
}

/// 从 Pareto(scale, shape) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Pareto(scale, shape) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_pareto(1.0, 2.0, 10))"
)]
fn sr_sample_pareto(scale: f64, shape: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Pareto::new(scale, shape).map_err(|e| duck_error(format!("sr_sample_pareto: {e}")))?;
    Ok(Some(sample_f64("sr_sample_pareto", &d, k)?))
}

/// 从 StudentsT(location, scale, freedom) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Student's t(location, scale, freedom) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_students_t(0.0, 1.0, 2.0, 10))"
)]
fn sr_sample_students_t(location: f64, scale: f64, freedom: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = StudentsT::new(location, scale, freedom)
        .map_err(|e| duck_error(format!("sr_sample_students_t: {e}")))?;
    Ok(Some(sample_f64("sr_sample_students_t", &d, k)?))
}

/// 从 Triangular(min, max, mode) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Triangular(min, max, mode) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_triangular(0.0, 2.0, 1.0, 10))"
)]
fn sr_sample_triangular(min: f64, max: f64, mode: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Triangular::new(min, max, mode).map_err(|e| duck_error(format!("sr_sample_triangular: {e}")))?;
    Ok(Some(sample_f64("sr_sample_triangular", &d, k)?))
}

/// 从 Uniform(min, max)（连续）抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k continuous Uniform(min, max) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_uniform(0.0, 1.0, 10))"
)]
fn sr_sample_uniform(min: f64, max: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Uniform::new(min, max).map_err(|e| duck_error(format!("sr_sample_uniform: {e}")))?;
    Ok(Some(sample_f64("sr_sample_uniform", &d, k)?))
}

/// 从 Weibull(shape, scale) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Weibull(shape, scale) samples into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_weibull(1.0, 1.0, 10))"
)]
fn sr_sample_weibull(shape: f64, scale: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Weibull::new(shape, scale).map_err(|e| duck_error(format!("sr_sample_weibull: {e}")))?;
    Ok(Some(sample_f64("sr_sample_weibull", &d, k)?))
}

// ---------------------------------------------------------------------------
// 离散分布
// ---------------------------------------------------------------------------

/// 从 Bernoulli(p) 抽样（0/1）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Bernoulli(p) samples (0/1) into a LIST(DOUBLE)",
    example = "SELECT len(sr_sample_bernoulli(0.5, 10))"
)]
fn sr_sample_bernoulli(p: f64, k: i64) -> DuckOptionResult<Vec<f64>> {
    let d = Bernoulli::new(p).map_err(|e| duck_error(format!("sr_sample_bernoulli: {e}")))?;
    Ok(Some(sample_f64("sr_sample_bernoulli", &d, k)?))
}

/// 从 Binomial(p, n) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Binomial(p, UBIGINT n) samples into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_binomial(0.5, 10, 8))"
)]
fn sr_sample_binomial(p: f64, n: u64, k: i64) -> DuckOptionResult<Vec<u64>> {
    let d = Binomial::new(p, n).map_err(|e| duck_error(format!("sr_sample_binomial: {e}")))?;
    Ok(Some(sample_u64("sr_sample_binomial", &d, k)?))
}

/// 从 Binomial 抽样并显式指定 statrs 的采样算法（BinomialAlgorithm：
/// 1 = Automatic、2 = Inversion、3 = Rejection）—— 这是 statrs 的 sampler 导出。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Binomial(p, n) samples through statrs' BinomialSampler with an explicit algorithm (1 automatic, 2 inversion, 3 rejection) into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_binomial_algorithm(0.5, 10, 1.0, 8))"
)]
fn sr_sample_binomial_algorithm(p: f64, n: u64, algorithm: f64, k: i64) -> DuckOptionResult<Vec<u64>> {
    let dist = Binomial::new(p, n).map_err(|e| duck_error(format!("sr_sample_binomial_algorithm: {e}")))?;
    let algo = match algorithm {
        1.0 => BinomialAlgorithm::Automatic,
        2.0 => BinomialAlgorithm::Inversion,
        3.0 => BinomialAlgorithm::Rejection,
        other => {
            return Err(duck_error(format!(
                "sr_sample_binomial_algorithm: the algorithm must be 1 (automatic), 2 (inversion) or 3 (rejection), got {other}"
            )));
        }
    };
    // sampler() 的构造错误（如 Rejection 下均值过小）是调用错误。
    let sampler: BinomialSampler = dist
        .sampler(algo)
        .map_err(|e| duck_error(format!("sr_sample_binomial_algorithm: {e}")))?;
    Ok(Some(sample_u64("sr_sample_binomial_algorithm", &sampler, k)?))
}

/// 从 DiscreteUniform(min, max)（整数边界）抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k DiscreteUniform(BIGINT min, max) samples into a LIST(BIGINT)",
    example = "SELECT len(sr_sample_discrete_uniform(1, 6, 10))"
)]
fn sr_sample_discrete_uniform(min: i64, max: i64, k: i64) -> DuckOptionResult<Vec<i64>> {
    let d = DiscreteUniform::new(min, max)
        .map_err(|e| duck_error(format!("sr_sample_discrete_uniform: {e}")))?;
    Ok(Some(sample_i64("sr_sample_discrete_uniform", &d, k)?))
}

/// 从 Geometric(p) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Geometric(p) samples into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_geometric(0.5, 10))"
)]
fn sr_sample_geometric(p: f64, k: i64) -> DuckOptionResult<Vec<u64>> {
    let d = Geometric::new(p).map_err(|e| duck_error(format!("sr_sample_geometric: {e}")))?;
    Ok(Some(sample_u64("sr_sample_geometric", &d, k)?))
}

/// 从 Hypergeometric(population, successes, draws) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Hypergeometric(population, successes, draws as UBIGINT) samples into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_hypergeometric(10, 5, 4, 8))"
)]
fn sr_sample_hypergeometric(population: u64, successes: u64, draws: u64, k: i64) -> DuckOptionResult<Vec<u64>> {
    let d = Hypergeometric::new(population, successes, draws)
        .map_err(|e| duck_error(format!("sr_sample_hypergeometric: {e}")))?;
    Ok(Some(sample_u64("sr_sample_hypergeometric", &d, k)?))
}

/// 从 NegativeBinomial(r, p) 抽样（u64 通道）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k NegativeBinomial(r, p) samples into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_negative_binomial(2.0, 0.5, 10))"
)]
fn sr_sample_negative_binomial(r: f64, p: f64, k: i64) -> DuckOptionResult<Vec<u64>> {
    let d = NegativeBinomial::new(r, p).map_err(|e| duck_error(format!("sr_sample_negative_binomial: {e}")))?;
    Ok(Some(sample_u64("sr_sample_negative_binomial", &d, k)?))
}

/// 从 Poisson(lambda) 抽样。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Poisson(lambda) samples into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_poisson(3.0, 10))"
)]
fn sr_sample_poisson(lambda: f64, k: i64) -> DuckOptionResult<Vec<u64>> {
    let d = Poisson::new(lambda).map_err(|e| duck_error(format!("sr_sample_poisson: {e}")))?;
    Ok(Some(sample_u64("sr_sample_poisson", &d, k)?))
}

/// 从 Categorical(probs) 抽样（u64 类别索引 → UBIGINT）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Categorical(prob LIST) samples as category indices into a LIST(UBIGINT)",
    example = "SELECT len(sr_sample_categorical([1.0, 2.0, 1.0], 10))"
)]
fn sr_sample_categorical(probs: Vec<f64>, k: i64) -> DuckOptionResult<Vec<u64>> {
    let d = Categorical::new(&probs).map_err(|e| duck_error(format!("sr_sample_categorical: {e}")))?;
    Ok(Some(sample_u64("sr_sample_categorical", &d, k)?))
}

// ---------------------------------------------------------------------------
// 多元与经验分布的抽样
// ---------------------------------------------------------------------------

/// 从 MultivariateNormal(mean, cov 行主序摊平) 抽 k 个向量，返回 LIST(LIST(DOUBLE))。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k MultivariateNormal samples (mean LIST, row-major flattened covariance LIST) as a LIST of point LISTs",
    example = "SELECT len(sr_sample_multivariate_normal([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 4))"
)]
fn sr_sample_multivariate_normal(
    mean: Vec<f64>,
    cov: Vec<f64>,
    k: i64,
) -> DuckOptionResult<Vec<Vec<f64>>> {
    let dim = mean.len();
    if cov.len() != dim * dim {
        return Err(duck_error(format!(
            "sr_sample_multivariate_normal: expected {} entries for a {dim}x{dim} covariance matrix (row-major), got {}",
            dim * dim,
            cov.len()
        )));
    }
    let d = MultivariateNormal::new(mean, cov)
        .map_err(|e| duck_error(format!("sr_sample_multivariate_normal: {e}")))?;
    let k = take_len("sr_sample_multivariate_normal", k)?;
    let mut rng = rand::rng();
    let rows: Vec<Vec<f64>> = (0..k)
        .map(|_| {
            let v: nalgebra::DVector<f64> = rng.sample(&d);
            v.iter().copied().collect()
        })
        .collect();
    Ok(Some(rows))
}

/// 从 MultivariateStudent(location, scale 行主序摊平, freedom) 抽 k 个向量。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k multivariate Student's t samples (location LIST, row-major flattened scale LIST, degrees of freedom) as a LIST of point LISTs",
    example = "SELECT len(sr_sample_multivariate_students_t([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0, 4))"
)]
fn sr_sample_multivariate_students_t(
    location: Vec<f64>,
    scale: Vec<f64>,
    freedom: f64,
    k: i64,
) -> DuckOptionResult<Vec<Vec<f64>>> {
    let dim = location.len();
    if scale.len() != dim * dim {
        return Err(duck_error(format!(
            "sr_sample_multivariate_students_t: expected {} entries for a {dim}x{dim} scale matrix (row-major), got {}",
            dim * dim,
            scale.len()
        )));
    }
    let d = MultivariateStudent::new(location, scale, freedom)
        .map_err(|e| duck_error(format!("sr_sample_multivariate_students_t: {e}")))?;
    let k = take_len("sr_sample_multivariate_students_t", k)?;
    let mut rng = rand::rng();
    let rows: Vec<Vec<f64>> = (0..k)
        .map(|_| {
            let v: nalgebra::DVector<f64> = rng.sample(&d);
            v.iter().copied().collect()
        })
        .collect();
    Ok(Some(rows))
}

/// 从 Dirichlet(alpha) 抽 k 个单纯形上的点（每行分量和为 1）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k Dirichlet samples on the simplex (concentration LIST alpha) as a LIST of point LISTs, each summing to 1",
    example = "SELECT len(sr_sample_dirichlet([1.0, 2.0], 4))"
)]
fn sr_sample_dirichlet(alpha: Vec<f64>, k: i64) -> DuckOptionResult<Vec<Vec<f64>>> {
    let d = Dirichlet::new(alpha).map_err(|e| duck_error(format!("sr_sample_dirichlet: {e}")))?;
    let k = take_len("sr_sample_dirichlet", k)?;
    let mut rng = rand::rng();
    let rows: Vec<Vec<f64>> = (0..k)
        .map(|_| {
            let v: nalgebra::DVector<f64> = rng.sample(&d);
            v.iter().copied().collect()
        })
        .collect();
    Ok(Some(rows))
}

/// 从 Multinomial(p, n) 抽 k 个计数向量（LIST(UBIGINT)，每行分量和恒为 n）。
#[duck_scalar_function(
    volatile = true,
    description = "Draw k multinomial count vectors (UBIGINT LISTs summing to n) given category probabilities and the trial count",
    example = "SELECT len(sr_sample_multinomial([0.3, 0.7], 10, 4))"
)]
fn sr_sample_multinomial(p: Vec<f64>, n: u64, k: i64) -> DuckOptionResult<Vec<Vec<u64>>> {
    let d = Multinomial::new(p, n)
        .map_err(|e| duck_error(format!("sr_sample_multinomial: {e}")))?;
    let k = take_len("sr_sample_multinomial", k)?;
    let mut rng = rand::rng();
    let rows: Vec<Vec<u64>> = (0..k)
        .map(|_| {
            let v: nalgebra::DVector<u64> = rng.sample(&d);
            v.iter().copied().collect()
        })
        .collect();
    Ok(Some(rows))
}

/// 经验分布抽样（聚合形态）：样本列进、k 为第二常量参数，随机子样本出 LIST。
///
/// ```sql
/// SELECT sr_sample_empirical(v, 5) FROM (VALUES (1.0), (2.0), (3.0)) t(v);
/// ```
#[duck_aggregate_function(
    auto_collect = true,
    description = "Draw k samples from the empirical distribution of a DOUBLE column into a LIST(DOUBLE); k is the second (constant) argument",
    example = "SELECT sr_sample_empirical(v, 5) FROM (VALUES (1.0), (2.0), (3.0)) t(v)"
)]
fn sr_sample_empirical(
    values: Vec<f64>,
    k: DuckFirst<f64>,
) -> DuckOptionResult<Vec<f64>> {
    if values.is_empty() {
        return Ok(None);
    }
    let k = k as i64;
    let k = take_len("sr_sample_empirical", k)?;
    let empirical: Empirical = values.into_iter().collect();
    let mut rng = rand::rng();
    Ok(Some((0..k).map(|_| rng.sample(&empirical)).collect()))
}
