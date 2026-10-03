// ============================================================================
// statrs::function 的包装：特殊函数（erf / gamma / beta / 阶乘 / 调和数 / logistic）
//
// 与 statrs 的对应关系逐条写在每个函数的注释里（statrs 原名）。有意不包装的：
//   - checked_* 变体：SQL 侧统一「算不出 → NULL」，checked 的错误路径与直接版的 NAN 收敛到
//     同一个出口，包两份只是把同一个函数注册两次；
//   - factorial::multinomial：参数是 &[u64]（向量），不匹配「对外只有 DOUBLE 标量」的形状；
//   - kernel（KDE 带宽用）与 evaluate::polynomial（内部求值宏的支撑件）：不是用户可算的函数。
//
// Wrapping statrs::function: the special functions (erf / gamma / beta / factorial / harmonic /
// logistic). Each note below names the statrs item it delegates to. Deliberately not wrapped:
// the checked_* variants (the SQL side folds "undefined" into NULL either way), multinomial
// (vector argument — does not fit the DOUBLE-only scalar surface), kernel and evaluate::polynomial
// (support pieces, not user-facing computations).
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
// 阶乘与二项系数（statrs::function::factorial；multinomial 因向量参数不包装）
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
// 核函数族（statrs::function::kernel）：9 个核的 evaluate 与 support，code 表：
// 1 Gaussian 2 Epanechnikov 3 Triangular 4 Tricube 5 Quartic 6 Uniform
// 7 Cosine 8 Logistic 9 Sigmoid
// ---------------------------------------------------------------------------

use statrs::function::kernel::{
    Cosine, Epanechnikov, Gaussian, Kernel, Logistic as LogisticKernel, Quartic, Sigmoid,
    Triangular as TriangularKernel, Tricube, Uniform as UniformKernel,
};

/// 核函数 K(x)（按 code 选核）。
#[duck_scalar_function(
    description = "Kernel function evaluation K(x); kind 1 gaussian 2 epanechnikov 3 triangular 4 tricube 5 quartic 6 uniform 7 cosine 8 logistic 9 sigmoid",
    example = "SELECT sr_kernel_eval(1.0, 0.0)"
)]
fn sr_kernel_eval(kind: f64, x: f64) -> DuckOptionResult<f64> {
    let value = match kind {
        1.0 => Gaussian.evaluate(x),
        2.0 => Epanechnikov.evaluate(x),
        3.0 => TriangularKernel.evaluate(x),
        4.0 => Tricube.evaluate(x),
        5.0 => Quartic.evaluate(x),
        6.0 => UniformKernel.evaluate(x),
        7.0 => Cosine.evaluate(x),
        8.0 => LogisticKernel.evaluate(x),
        9.0 => Sigmoid.evaluate(x),
        other => {
            return Err(duck_error(format!(
                "sr_kernel_eval: the kernel kind must be 1..9 (gaussian, epanechnikov, triangular, tricube, quartic, uniform, cosine, logistic, sigmoid), got {other}"
            )));
        }
    };
    nan_to_null(value)
}

/// 核的紧支撑区间 [lo, hi]；非紧支撑核（gaussian / logistic / sigmoid）返回 NULL。
#[duck_scalar_function(
    description = "Compact support [lo, hi] of a kernel (same kind codes as sr_kernel_eval); NULL for kernels with unbounded support",
    example = "SELECT sr_kernel_support(2.0)"
)]
fn sr_kernel_support(kind: f64) -> DuckOptionResult<Vec<f64>> {
    let support = match kind {
        1.0 => Gaussian.support(),
        2.0 => Epanechnikov.support(),
        3.0 => TriangularKernel.support(),
        4.0 => Tricube.support(),
        5.0 => Quartic.support(),
        6.0 => UniformKernel.support(),
        7.0 => Cosine.support(),
        8.0 => LogisticKernel.support(),
        9.0 => Sigmoid.support(),
        other => {
            return Err(duck_error(format!(
                "sr_kernel_support: the kernel kind must be 1..9 (see sr_kernel_eval), got {other}"
            )));
        }
    };
    Ok(support.map(|(lo, hi)| vec![lo, hi]))
}
