// ============================================================================
// statrs::euclid 的包装：规范化取模（Modulus trait）
//
// Modulus::modulus 即 `((self % d) + d) % d`，与 SQL 的 `%`（截断取余，负数会得负数）
// 不同：这里的结果落在 `[0, d)`。只出口 f64 标量版本 —— statrs 对 f32 / i64 / i32 /
// u64 / u32 的同义实现是「同一能力在别的类型上的重复实现」，DuckDB 的隐式转换已覆盖
// （与 functions/mod.rs 的口径一致）。divisor 为 0 时结果是 NaN → SQL NULL。
//
// statrs::euclid: the canonical modulus (`((self % d) + d) % d`, unlike SQL's truncated
// remainder). Only the f64 scalar surface is exported; statrs' f32 / integer impls are the
// same capability repeated on other types, which DuckDB's implicit conversions already
// cover. A zero divisor yields NaN, folded to SQL NULL.
// ============================================================================

use duckfn::{DuckOptionResult, duck_scalar_function};
use statrs::euclid::Modulus;

use crate::extension::functions::nan_to_null;

/// `sr_modulus(x, divisor)`：规范化取模 `((x % divisor) + divisor) % divisor`
/// （statrs::euclid::Modulus），结果总落在 `[0, divisor)`；divisor 为 0 → NULL。
#[duck_scalar_function(
    description = "Canonical (Euclidean) modulus ((x % divisor) + divisor) % divisor, always in [0, divisor); NULL for a zero divisor",
    example = "SELECT sr_modulus(-1.0, 5.0)"
)]
fn sr_modulus(x: f64, divisor: f64) -> DuckOptionResult<f64> {
    nan_to_null(x.modulus(divisor))
}