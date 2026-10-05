// ============================================================================
// statrs::function 的包装：特殊函数（erf / gamma / beta / 阶乘 / 调和数 / logistic）
//
// 与 statrs 的对应关系逐条写在每个函数的注释里（statrs 原名）。有意不包装的：
//   - checked_* 变体：SQL 侧统一「算不出 → NULL」，checked 的错误路径与直接版的 NAN 收敛到
//     同一个出口，包两份只是把同一个函数注册两次。
//
// Wrapping statrs::function: the special functions (erf / gamma / beta / factorial / harmonic /
// logistic). Each note below names the statrs item it delegates to. Deliberately not wrapped:
// the checked_* variants (the SQL side folds "undefined" into NULL either way).
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use statrs::function::{beta, erf, evaluate, exponential, factorial, gamma, harmonic, logistic};

use super::{as_u64, nan_to_null};

// ---------------------------------------------------------------------------
// 误差函数族（statrs::function::erf）
// ---------------------------------------------------------------------------

/// 误差函数 `erf(x)`。
#[duck_scalar_function(
    description = "Error function erf(x)",
    example = "SELECT sr_erf(1.0)"
)]
fn sr_erf(x: f64) -> DuckOptionResult<f64> {
    nan_to_null(erf::erf(x))
}

/// `erf` 的反函数。
#[duck_scalar_function(
    description = "Inverse error function, erf(x) = y solved for x",
    example = "SELECT sr_erf_inv(0.8427007929497149)"
)]
fn sr_erf_inv(y: f64) -> DuckOptionResult<f64> {
    nan_to_null(erf::erf_inv(y))
}

/// 互补误差函数 `erfc(x) = 1 - erf(x)`。
#[duck_scalar_function(
    description = "Complementary error function erfc(x) = 1 - erf(x)",
    example = "SELECT sr_erfc(1.0)"
)]
fn sr_erfc(x: f64) -> DuckOptionResult<f64> {
    nan_to_null(erf::erfc(x))
}

/// `erfc` 的反函数。
#[duck_scalar_function(
    description = "Inverse complementary error function, erfc(x) = y solved for x",
    example = "SELECT sr_erfc_inv(0.15729920705028513)"
)]
fn sr_erfc_inv(y: f64) -> DuckOptionResult<f64> {
    nan_to_null(erf::erfc_inv(y))
}

// ---------------------------------------------------------------------------
// Gamma 函数族（statrs::function::gamma）
// ---------------------------------------------------------------------------

/// Gamma 函数 `Γ(x)`。
#[duck_scalar_function(
    description = "Gamma function Γ(x)",
    example = "SELECT sr_gamma(5.0)"
)]
fn sr_gamma(x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::gamma(x))
}

/// Gamma 函数的自然对数。
#[duck_scalar_function(
    description = "Natural logarithm of the Gamma function, ln(Γ(x))",
    example = "SELECT sr_ln_gamma(5.0)"
)]
fn sr_ln_gamma(x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::ln_gamma(x))
}

/// 上不完全 Gamma 函数 `Γ(a, x)`（从 x 到 ∞ 的积分）。
#[duck_scalar_function(
    description = "Upper incomplete Gamma function Γ(a, x), integrating from x to infinity",
    example = "SELECT sr_gamma_upper_incomplete(2.0, 1.0)"
)]
fn sr_gamma_upper_incomplete(a: f64, x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::gamma_ui(a, x))
}

/// 下不完全 Gamma 函数 `γ(a, x)`（从 0 到 x 的积分）。
#[duck_scalar_function(
    description = "Lower incomplete Gamma function γ(a, x), integrating from 0 to x",
    example = "SELECT sr_gamma_lower_incomplete(2.0, 1.0)"
)]
fn sr_gamma_lower_incomplete(a: f64, x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::gamma_li(a, x))
}

/// 正则化上不完全 Gamma 函数 `Q(a, x) = Γ(a,x)/Γ(a)`。
#[duck_scalar_function(
    description = "Regularized upper incomplete Gamma function Q(a, x) = Γ(a,x)/Γ(a)",
    example = "SELECT sr_gamma_upper_regularized(2.0, 1.0)"
)]
fn sr_gamma_upper_regularized(a: f64, x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::gamma_ur(a, x))
}

/// 正则化下不完全 Gamma 函数 `P(a, x) = γ(a,x)/Γ(a)`。
#[duck_scalar_function(
    description = "Regularized lower incomplete Gamma function P(a, x) = γ(a,x)/Γ(a)",
    example = "SELECT sr_gamma_lower_regularized(2.0, 1.0)"
)]
fn sr_gamma_lower_regularized(a: f64, x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::gamma_lr(a, x))
}

/// digamma 函数 `ψ(x) = d/dx ln Γ(x)`。
#[duck_scalar_function(
    description = "Digamma function ψ(x), the derivative of ln(Γ(x))",
    example = "SELECT sr_digamma(2.0)"
)]
fn sr_digamma(x: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::digamma(x))
}

/// digamma 的反函数。
#[duck_scalar_function(
    description = "Inverse digamma function, ψ(x) = y solved for x",
    example = "SELECT sr_inv_digamma(0.42278433509846713)"
)]
fn sr_inv_digamma(y: f64) -> DuckOptionResult<f64> {
    nan_to_null(gamma::inv_digamma(y))
}

// ---------------------------------------------------------------------------
// Beta 函数族（statrs::function::beta）
// ---------------------------------------------------------------------------

/// Beta 函数 `B(a, b)`。
#[duck_scalar_function(
    description = "Beta function B(a, b)",
    example = "SELECT sr_beta(2.0, 3.0)"
)]
fn sr_beta(a: f64, b: f64) -> DuckOptionResult<f64> {
    nan_to_null(beta::beta(a, b))
}

/// Beta 函数的自然对数。
#[duck_scalar_function(
    description = "Natural logarithm of the Beta function, ln(B(a, b))",
    example = "SELECT sr_ln_beta(2.0, 3.0)"
)]
fn sr_ln_beta(a: f64, b: f64) -> DuckOptionResult<f64> {
    nan_to_null(beta::ln_beta(a, b))
}

/// 不完全 Beta 函数 `B(x; a, b)`（从 0 到 x 的积分）。
#[duck_scalar_function(
    description = "Incomplete Beta function B(x; a, b), integrating from 0 to x",
    example = "SELECT sr_beta_incomplete(2.0, 3.0, 0.5)"
)]
fn sr_beta_incomplete(a: f64, b: f64, x: f64) -> DuckOptionResult<f64> {
    nan_to_null(beta::beta_inc(a, b, x))
}

/// 正则化不完全 Beta 函数 `I(x; a, b)`。
#[duck_scalar_function(
    description = "Regularized incomplete Beta function I(x; a, b)",
    example = "SELECT sr_beta_regularized(2.0, 3.0, 0.5)"
)]
fn sr_beta_regularized(a: f64, b: f64, x: f64) -> DuckOptionResult<f64> {
    nan_to_null(beta::beta_reg(a, b, x))
}

/// 正则化不完全 Beta 函数的反函数（求 x）。
#[duck_scalar_function(
    description = "Inverse of the regularized incomplete Beta function: the x where I(x; a, b) = p",
    example = "SELECT sr_inv_beta_regularized(2.0, 3.0, 0.6875)"
)]
fn sr_inv_beta_regularized(a: f64, b: f64, p: f64) -> DuckOptionResult<f64> {
    nan_to_null(beta::inv_beta_reg(a, b, p))
}

// ---------------------------------------------------------------------------
// 阶乘与二项/多项式系数（statrs::function::factorial）
// ---------------------------------------------------------------------------

/// n 的阶乘（`factorial::factorial`）。
#[duck_scalar_function(
    description = "Factorial of n, given as a whole-number DOUBLE",
    example = "SELECT sr_factorial(10.0)"
)]
fn sr_factorial(n: f64) -> DuckOptionResult<f64> {
    nan_to_null(factorial::factorial(as_u64("sr_factorial", n)?))
}

/// ln(n!)。
#[duck_scalar_function(
    description = "Natural logarithm of n!, given as a whole-number DOUBLE",
    example = "SELECT sr_ln_factorial(100.0)"
)]
fn sr_ln_factorial(n: f64) -> DuckOptionResult<f64> {
    nan_to_null(factorial::ln_factorial(as_u64("sr_ln_factorial", n)?))
}

/// 二项系数 `C(n, k)`（statrs 原名 `factorial::binomial`；取 SQL 侧更常见的 choose 一词，
/// 避免与二项分布撞名）。
#[duck_scalar_function(
    description = "Binomial coefficient C(n, k) (statrs' factorial::binomial), n and k whole-number DOUBLEs",
    example = "SELECT sr_choose(10.0, 3.0)"
)]
fn sr_choose(n: f64, k: f64) -> DuckOptionResult<f64> {
    nan_to_null(factorial::binomial(
        as_u64("sr_choose", n)?,
        as_u64("sr_choose", k)?,
    ))
}

/// ln(C(n, k))。
#[duck_scalar_function(
    description = "Natural logarithm of the binomial coefficient C(n, k), n and k whole-number DOUBLEs",
    example = "SELECT sr_ln_choose(100.0, 50.0)"
)]
fn sr_ln_choose(n: f64, k: f64) -> DuckOptionResult<f64> {
    nan_to_null(factorial::ln_binomial(
        as_u64("sr_ln_choose", n)?,
        as_u64("sr_ln_choose", k)?,
    ))
}

/// 多项式系数 `n! / (n1! n2! …)`（statrs 原名 `factorial::multinomial`，用其 checked 版：
/// 各 ni 之和 ≠ n 时给 None → SQL NULL）。counts 为 LIST(BIGINT)。
#[duck_scalar_function(
    description = "Multinomial coefficient n! / (n1! n2! ...) over a whole-number n and a count LIST(BIGINT); NULL when the counts do not sum to n",
    example = "SELECT sr_multinomial_coefficient(5.0, [2, 2, 1])"
)]
fn sr_multinomial_coefficient(total: f64, counts: Vec<i64>) -> DuckOptionResult<f64> {
    let n = as_u64("sr_multinomial_coefficient", total)?;
    let ni = counts
        .iter()
        .map(|c| as_u64("sr_multinomial_coefficient", *c as f64))
        .collect::<Result<Vec<u64>, _>>()?;
    Ok(factorial::checked_multinomial(n, &ni))
}

// ---------------------------------------------------------------------------
// 调和数（statrs::function::harmonic）
// ---------------------------------------------------------------------------

/// 第 n 个调和数 `H(n) = Σ 1/k`。
#[duck_scalar_function(
    description = "Harmonic number H(n) = sum of 1/k for k = 1..n, n a whole-number DOUBLE",
    example = "SELECT sr_harmonic(10.0)"
)]
fn sr_harmonic(n: f64) -> DuckOptionResult<f64> {
    nan_to_null(harmonic::harmonic(as_u64("sr_harmonic", n)?))
}

/// 广义调和数 `H(n, m) = Σ 1/k^m`。
#[duck_scalar_function(
    description = "Generalized harmonic number sum of 1/k^m for k = 1..n, n a whole-number DOUBLE",
    example = "SELECT sr_generalized_harmonic(10.0, 2.0)"
)]
fn sr_generalized_harmonic(n: f64, m: f64) -> DuckOptionResult<f64> {
    nan_to_null(harmonic::gen_harmonic(as_u64("sr_generalized_harmonic", n)?, m))
}

// ---------------------------------------------------------------------------
// Logistic 函数（statrs::function::logistic）
// ---------------------------------------------------------------------------

/// Logistic（logit）函数的 sigmoid，即标准 Logistic CDF。
#[duck_scalar_function(
    description = "Logistic (sigmoid) function 1 / (1 + exp(-p))",
    example = "SELECT sr_logistic(0.0)"
)]
fn sr_logistic(p: f64) -> DuckOptionResult<f64> {
    nan_to_null(logistic::logistic(p))
}

/// sigmoid 的反函数 `logit(p) = ln(p / (1 - p))`；端点 p = 0 / 1 给 ∓inf（合法极限），
/// p 越出 `[0, 1]` 时 statrs 的 `checked_logit` 给 None → SQL NULL。
#[duck_scalar_function(
    description = "Logit, the inverse sigmoid ln(p / (1 - p)); NULL unless p is in [0, 1], endpoints give infinity",
    example = "SELECT sr_logit(0.5)"
)]
fn sr_logit(p: f64) -> DuckOptionResult<f64> {
    match logistic::checked_logit(p) {
        Some(v) => Ok(Some(v)),
        None => Ok(None),
    }
}

// ---------------------------------------------------------------------------
// 指数积分（statrs::function::exponential）
// ---------------------------------------------------------------------------

/// 指数积分 `E_n(x)`；statrs 对无定义的输入返回 None → SQL NULL。
#[duck_scalar_function(
    description = "Exponential integral E_n(x) for n >= 0 and x >= 0; NULL where statrs leaves it undefined",
    example = "SELECT sr_exponential_integral(1.0, 0.0)"
)]
fn sr_exponential_integral(x: f64, n: f64) -> DuckOptionResult<f64> {
    Ok(exponential::integral(x, as_u64("sr_exponential_integral", n)?))
}

// ---------------------------------------------------------------------------
// 多项式求值（statrs::function::evaluate::polynomial）
// ---------------------------------------------------------------------------

/// 多项式 Σ coeff[i] · x^i（coeffs 按升幂排列的 LIST(DOUBLE)）。
#[duck_scalar_function(
    description = "Polynomial evaluation sum coeff[i] * x^i with the coefficient LIST in ascending order",
    example = "SELECT sr_polynomial(2.0, [1.0, 0.0, 3.0])"
)]
fn sr_polynomial(x: f64, coeffs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(evaluate::polynomial(x, &coeffs))
}

// ---------------------------------------------------------------------------
// 核函数族（statrs::function::kernel）：9 个核的 evaluate / evaluate_with_bandwidth / support
//
// kind 是核名的字符串（'gaussian'、'epanechnikov' …），不是编号 —— 调用点写
// `sr_kernel_eval('gaussian', 0.0)` 一眼可读，写 `sr_kernel_eval(1.0, 0.0)` 只能回去查表。
// ---------------------------------------------------------------------------

use statrs::function::kernel::{
    Cosine, Epanechnikov, Gaussian, Kernel, Logistic as LogisticKernel, Quartic, Sigmoid,
    Triangular as TriangularKernel, Tricube, Uniform as UniformKernel,
};
use quack_rs::error::ExtensionError;

/// 9 个核的共同形状：按名字选出 statrs 的那个核（大小写不敏感），交回调用方求值。
///
/// 原来这里是三段几乎一样的 `match 1.0 => ...`，每加一个核要改三处；收成一个函数后
/// 「有哪些核」只有下面这张表一处。未知核名在这里就报错，三个出口的错误文案因此一致。
fn kernel_by_name(name: &str, fn_name: &str) -> Result<Box<dyn Kernel>, ExtensionError> {
    // 9 个核都是零大小类型，装 Box 只是为了让三个出口共用一份 dyn Kernel。
    let kernel: Box<dyn Kernel> = match name.to_ascii_lowercase().as_str() {
        "gaussian" => Box::new(Gaussian),
        "epanechnikov" => Box::new(Epanechnikov),
        "triangular" => Box::new(TriangularKernel),
        "tricube" => Box::new(Tricube),
        "quartic" => Box::new(Quartic),
        "uniform" => Box::new(UniformKernel),
        "cosine" => Box::new(Cosine),
        "logistic" => Box::new(LogisticKernel),
        "sigmoid" => Box::new(Sigmoid),
        other => {
            return Err(duck_error(format!(
                "{fn_name}: unknown kernel {other:?}; expected one of gaussian, epanechnikov, \
                 triangular, tricube, quartic, uniform, cosine, logistic, sigmoid"
            )));
        }
    };
    Ok(kernel)
}

/// 核函数 K(x)（按名字选核，大小写不敏感）。
#[duck_scalar_function(
    description = "Kernel function evaluation K(x) for a named kernel (gaussian, epanechnikov, triangular, tricube, quartic, uniform, cosine, logistic, sigmoid; case-insensitive)",
    example = "SELECT sr_kernel_eval('gaussian', 0.0)"
)]
fn sr_kernel_eval(kind: String, x: f64) -> DuckOptionResult<f64> {
    let kernel = kernel_by_name(&kind, "sr_kernel_eval")?;
    nan_to_null(kernel.evaluate(x))
}

/// 带宽缩放后的核函数 `K(x / h) / h`（statrs::Kernel::evaluate_with_bandwidth），
/// 确保缩放后仍积分为 1；核名语义与 sr_kernel_eval 完全一致。
#[duck_scalar_function(
    description = "Kernel function with bandwidth scaling K(x / h) / h, for a named kernel (same names as sr_kernel_eval)",
    example = "SELECT sr_kernel_eval_with_bandwidth('gaussian', 0.0, 0.5)"
)]
fn sr_kernel_eval_with_bandwidth(kind: String, x: f64, bandwidth: f64) -> DuckOptionResult<f64> {
    let kernel = kernel_by_name(&kind, "sr_kernel_eval_with_bandwidth")?;
    nan_to_null(kernel.evaluate_with_bandwidth(x, bandwidth))
}

/// 核的紧支撑区间 [lo, hi]；非紧支撑核（gaussian / logistic / sigmoid）返回 NULL。
#[duck_scalar_function(
    description = "Compact support [lo, hi] of a named kernel (same names as sr_kernel_eval); NULL for kernels with unbounded support",
    example = "SELECT sr_kernel_support('epanechnikov')"
)]
fn sr_kernel_support(kind: String) -> DuckOptionResult<Vec<f64>> {
    let kernel = kernel_by_name(&kind, "sr_kernel_support")?;
    Ok(kernel.support().map(|(lo, hi)| vec![lo, hi]))
}
