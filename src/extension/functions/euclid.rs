// ============================================================================
// statrs::euclid 的包装：规范化取模（Modulus trait）
//
// Modulus::modulus 即 `((self % d) + d) % d`，与 SQL 的 `%`（截断取余，负数会得负数）
// 不同：这里的结果落在 `[0, d)`。严格对齐 statrs 的 6 种实现（f64、f32、i64、i32、
// u64、u32），用 duckfn 的 `overloads_name` 合并到同一个 SQL 函数名 `sr_modulus` 下，
// 按参数类型分派。浮点 divisor 为 0 时结果是 NaN → SQL NULL；整数 divisor 为 0 时
// 显式返回 NULL（避免整数除零 panic）。
//
// statrs::euclid: the canonical modulus (`((self % d) + d) % d`, unlike SQL's truncated
// remainder). The result always lands in `[0, d)`. All six statrs implementations
// (f64, f32, i64, i32, u64, u32) are exported via `overloads_name = "sr_modulus"`,
// dispatched by argument types. A zero divisor yields NaN (floating-point) → NULL,
// or is explicitly caught (integer) → NULL.
// ============================================================================

use duckfn::{DuckOptionResult, duck_scalar_function};
use statrs::euclid::Modulus;

use crate::extension::functions::nan_to_null;

/// `sr_modulus(x, divisor)`：f64 版本（statrs 原生 Modulus for f64）。
/// 结果总落在 `[0, divisor)`；divisor 为 0 → NaN → NULL。
#[duck_scalar_function(
    overloads_name = "sr_modulus",
    description = "Canonical (Euclidean) modulus ((x % divisor) + divisor) % divisor for f64, always in [0, divisor); NULL for a zero divisor",
    example = "SELECT sr_modulus(-1.0, 5.0)"
)]
fn sr_modulus_f64(x: f64, divisor: f64) -> DuckOptionResult<f64> {
    nan_to_null(x.modulus(divisor))
}

/// `sr_modulus(x, divisor)`：f32 版本（statrs 原生 Modulus for f32）。
#[duck_scalar_function(
    overloads_name = "sr_modulus",
    description = "Canonical (Euclidean) modulus for f32, always in [0, divisor); NULL for a zero divisor"
)]
fn sr_modulus_f32(x: f32, divisor: f32) -> DuckOptionResult<f32> {
    let v = x.modulus(divisor);
    if v.is_nan() {
        Ok(None)
    } else {
        Ok(Some(v))
    }
}

/// `sr_modulus(x, divisor)`：i64 版本（statrs 原生 Modulus for i64）。
/// 结果总落在 `[0, divisor)`；divisor 为 0 → 显式返回 NULL（避免整数除零 panic）。
#[duck_scalar_function(
    overloads_name = "sr_modulus",
    description = "Canonical (Euclidean) modulus for i64, always in [0, divisor); NULL for a zero divisor"
)]
fn sr_modulus_i64(x: i64, divisor: i64) -> DuckOptionResult<i64> {
    if divisor == 0 {
        return Ok(None);
    }
    Ok(Some(x.modulus(divisor)))
}

/// `sr_modulus(x, divisor)`：i32 版本（statrs 原生 Modulus for i32）。
#[duck_scalar_function(
    overloads_name = "sr_modulus",
    description = "Canonical (Euclidean) modulus for i32, always in [0, divisor); NULL for a zero divisor"
)]
fn sr_modulus_i32(x: i32, divisor: i32) -> DuckOptionResult<i32> {
    if divisor == 0 {
        return Ok(None);
    }
    Ok(Some(x.modulus(divisor)))
}

/// `sr_modulus(x, divisor)`：u64 版本（statrs 原生 Modulus for u64）。
/// 结果总落在 `[0, divisor)`；divisor 为 0 → 显式返回 NULL。
#[duck_scalar_function(
    overloads_name = "sr_modulus",
    description = "Canonical (Euclidean) modulus for u64, always in [0, divisor); NULL for a zero divisor"
)]
fn sr_modulus_u64(x: u64, divisor: u64) -> DuckOptionResult<u64> {
    if divisor == 0 {
        return Ok(None);
    }
    Ok(Some(x.modulus(divisor)))
}

/// `sr_modulus(x, divisor)`：u32 版本（statrs 原生 Modulus for u32）。
#[duck_scalar_function(
    overloads_name = "sr_modulus",
    description = "Canonical (Euclidean) modulus for u32, always in [0, divisor); NULL for a zero divisor"
)]
fn sr_modulus_u32(x: u32, divisor: u32) -> DuckOptionResult<u32> {
    if divisor == 0 {
        return Ok(None);
    }
    Ok(Some(x.modulus(divisor)))
}