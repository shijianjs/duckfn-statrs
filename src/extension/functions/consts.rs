// ============================================================================
// statrs::consts 的包装：每个常量一个零参标量函数
//
// SQL 里没有这些现成的名字（DuckDB 只给 pi()/e()/tau()），做成函数后可以在表达式里
// 引用，也保证取值与 statrs 内部用的是同一份 —— 不自己重打数字，直接引用常量本身。
//
// Wrapping statrs::consts: one zero-argument scalar function per constant. The bodies return
// statrs' own constants, so the SQL side can never drift from what statrs computes with.
// ============================================================================

use duckfn::duck_scalar_function;
use statrs::consts;

/// √(2π) —— 正态密度的归一化因子。
#[duck_scalar_function(
    description = "The constant sqrt(2*PI), statrs' normalizing factor for the Gaussian density",
    example = "SELECT sr_sqrt_2pi()"
)]
fn sr_sqrt_2pi() -> f64 {
    consts::SQRT_2PI
}

/// ln(π)。
#[duck_scalar_function(
    description = "The constant ln(PI)",
    example = "SELECT sr_ln_pi()"
)]
fn sr_ln_pi() -> f64 {
    consts::LN_PI
}

/// √(2π) 的自然对数。
#[duck_scalar_function(
    description = "The constant ln(sqrt(2*PI))",
    example = "SELECT sr_ln_sqrt_2pi()"
)]
fn sr_ln_sqrt_2pi() -> f64 {
    consts::LN_SQRT_2PI
}

/// √(2πe) 的自然对数。
#[duck_scalar_function(
    description = "The constant ln(sqrt(2*PI*e))",
    example = "SELECT sr_ln_sqrt_2pie()"
)]
fn sr_ln_sqrt_2pie() -> f64 {
    consts::LN_SQRT_2PIE
}

/// 2√(e/π) 的自然对数。
#[duck_scalar_function(
    description = "The constant ln(2*sqrt(e/PI))",
    example = "SELECT sr_ln_2_sqrt_e_over_pi()"
)]
fn sr_ln_2_sqrt_e_over_pi() -> f64 {
    consts::LN_2_SQRT_E_OVER_PI
}

/// 2√(e/π)。
#[duck_scalar_function(
    description = "The constant 2*sqrt(e/PI)",
    example = "SELECT sr_2_sqrt_e_over_pi()"
)]
fn sr_2_sqrt_e_over_pi() -> f64 {
    consts::TWO_SQRT_E_OVER_PI
}

/// 欧拉-马斯刻若尼常数 γ。
#[duck_scalar_function(
    description = "The Euler-Mascheroni constant",
    example = "SELECT sr_euler_mascheroni()"
)]
fn sr_euler_mascheroni() -> f64 {
    consts::EULER_MASCHERONI
}
