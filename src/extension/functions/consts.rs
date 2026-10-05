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
use statrs::{consts, prec};

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

// ============================================================================
// statrs::prec 的包装：statrs 内部做浮点比较用的精度阈值（5 个常量）+ almost_eq
//
// 为什么放在 consts.rs 而不是新开文件：它们同样是「statrs 的一组常量 + 一个比较器」，
// 与 consts 同属精度策略，放一起比拆成 prec.rs 更好找（对照 functions/mod.rs 的模块表）。
//
// 取舍：`statrs::prec` 之前被判为「内部精度策略，非计算能力」而整块不包装。复核后改判：
// 它是 `pub mod`，5 个常量与 `almost_eq` 都是公开 API，而本项目的首要规则是 statrs 能力
// 不允许缺失。常量的包装方式照抄上面 7 个数学常量（零参标量），`almost_eq` 则是
// 「两个浮点 + 阈值 → BOOLEAN」的标量。
//
// statrs 0.19 把 `almost_eq` 标成了 deprecated（建议改用 approx 宏），但它仍是公开 API 且
// 本项目要的是 statrs 的能力面，所以照包；下面这一处 allow 就是为这个 deprecated 调用，
// 删掉包装就没有理由保留了。
//
// Why these live in consts.rs: they are a constant set plus a comparator, same shape as consts
// itself. Reconsidering the earlier "internal precision strategy, not a capability" verdict —
// statrs::prec is a pub mod whose five constants and almost_eq are all public API, and the
// project's first rule is that no statrs capability may be missing.
// ============================================================================

/// IEEE 754 双精度的最大相对精度 ε = 2⁻⁵³。
#[duck_scalar_function(
    description = "F64_PREC: the maximum relative precision 2^-53 of an IEEE 754 double",
    example = "SELECT sr_f64_prec()"
)]
fn sr_f64_prec() -> f64 {
    prec::F64_PREC
}

/// statrs 对 f64 的默认绝对比较容差（= 0.01 · F64_PREC）。
#[duck_scalar_function(
    description = "statrs' default absolute comparison tolerance for f64 (0.01 * F64_PREC)",
    example = "SELECT sr_default_f64_acc()"
)]
fn sr_default_f64_acc() -> f64 {
    prec::DEFAULT_F64_ACC
}

/// statrs f64 运算的默认相对精度目标 1e-14。
#[duck_scalar_function(
    description = "statrs' default target relative accuracy for f64 operations, 1e-14",
    example = "SELECT sr_default_relative_acc()"
)]
fn sr_default_relative_acc() -> f64 {
    prec::DEFAULT_RELATIVE_ACC
}

/// statrs f64 运算的默认绝对精度目标 1e-9。
#[duck_scalar_function(
    description = "statrs' default target absolute accuracy for f64 operations, 1e-9",
    example = "SELECT sr_default_eps()"
)]
fn sr_default_eps() -> f64 {
    prec::DEFAULT_EPS
}

/// statrs f64 运算的默认 ULP 精度目标（5 个 ULP）。
///
/// statrs 里是 u32；这里返回 DOUBLE，让「零参标量返回 DOUBLE」这条约定保持一致
/// （见 functions/mod.rs 的类型面），值本身是个小整数，DOUBLE 装得下且无损。
#[duck_scalar_function(
    description = "statrs' default target ULPs accuracy for f64 operations (5), as a DOUBLE",
    example = "SELECT sr_default_ulps()"
)]
fn sr_default_ulps() -> f64 {
    f64::from(prec::DEFAULT_ULPS)
}

/// statrs 的 `almost_eq(a, b, acc)`：按绝对阈值 acc 比较两个浮点是否足够接近。
///
/// 语义照 statrs 原文：双方都是无穷时只有同号（相等）才算接近 —— NaN 不等于任何值，
/// 含 NaN 的比较一律给 false；其余走 approx 的绝对差比较。
#[duck_scalar_function(
    description = "Whether two DOUBLEs are within the absolute tolerance acc of each other (statrs' almost_eq); infinities only match themselves and NaN never matches",
    example = "SELECT sr_almost_eq(0.1 + 0.2, 0.3, 1e-9)"
)]
fn sr_almost_eq(a: f64, b: f64, acc: f64) -> bool {
    #[allow(deprecated)] // statrs 0.19 marks almost_eq deprecated; wrapping it is the point here.
    prec::almost_eq(a, b, acc)
}
