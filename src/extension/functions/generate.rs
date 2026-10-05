// ============================================================================
// statrs::generate 的包装：无限波形/对数间距序列，SQL 侧取前 k 项出 LIST(DOUBLE)
//
// statrs 的生成器是惰性 Iterator（为测试与信号场景设计）；数据库里对应的形态就是
// 「要前 k 个点」，所以每个生成器一个函数、返回 LIST。k 与 duration 类参数用
// BIGINT（statrs 签名里就是 i64/usize），其余是 DOUBLE。
//
// statrs::generate wrapped as "take the first k points" functions returning
// LIST(DOUBLE); the iterators' i64 durations stay BIGINT on the SQL side.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use quack_rs::error::ExtensionError;
use statrs::generate;


/// 长度校验：k 必须是正整数。
///
/// k must be a positive whole number.
fn take_len(fn_name: &str, k: i64) -> Result<usize, ExtensionError> {
    if k <= 0 {
        return Err(duck_error(format!(
            "{fn_name}: k must be a positive whole number, got {k}"
        )));
    }
    Ok(k as usize)
}

/// 对数间距序列（statrs::generate::log_spaced）：length 个点、指数从 start_exp 到 stop_exp。
/// length 是 statrs 的 `usize` → UBIGINT。
#[duck_scalar_function(
    description = "Log-spaced sequence of length points between 10^start_exp and 10^stop_exp, returned as LIST(DOUBLE)",
    example = "SELECT sr_gen_log_spaced(5, 0.0, 2.0)"
)]
fn sr_gen_log_spaced(length: u64, start_exp: f64, stop_exp: f64) -> DuckOptionResult<Vec<f64>> {
    Ok(Some(generate::log_spaced(length as usize, start_exp, stop_exp)))
}

/// 无限方波（InfiniteSquare::new）取前 k 点。
#[duck_scalar_function(
    description = "First k points of statrs' infinite square wave (high/low durations as BIGINT)",
    example = "SELECT sr_gen_square(8, 2, 2, 1.0, 0.0, 0)"
)]
fn sr_gen_square(
    k: i64,
    high_duration: i64,
    low_duration: i64,
    high_value: f64,
    low_value: f64,
    delay: i64,
) -> DuckOptionResult<Vec<f64>> {
    let k = take_len("sr_gen_square", k)?;
    Ok(Some(
        generate::InfiniteSquare::new(high_duration, low_duration, high_value, low_value, delay)
            .take(k)
            .collect(),
    ))
}

/// 无限三角波（InfiniteTriangle::new）取前 k 点。
#[duck_scalar_function(
    description = "First k points of statrs' infinite triangle wave (raise/fall durations as BIGINT)",
    example = "SELECT sr_gen_triangle(8, 2, 2, 1.0, 0.0, 0)"
)]
fn sr_gen_triangle(
    k: i64,
    raise_duration: i64,
    fall_duration: i64,
    high_value: f64,
    low_value: f64,
    delay: i64,
) -> DuckOptionResult<Vec<f64>> {
    let k = take_len("sr_gen_triangle", k)?;
    Ok(Some(
        generate::InfiniteTriangle::new(raise_duration, fall_duration, high_value, low_value, delay)
            .take(k)
            .collect(),
    ))
}

/// 无限锯齿波（InfiniteSawtooth::new）取前 k 点。
#[duck_scalar_function(
    description = "First k points of statrs' infinite sawtooth wave (period as BIGINT)",
    example = "SELECT sr_gen_sawtooth(8, 4, 1.0, 0.0, 0)"
)]
fn sr_gen_sawtooth(
    k: i64,
    period: i64,
    high_value: f64,
    low_value: f64,
    delay: i64,
) -> DuckOptionResult<Vec<f64>> {
    let k = take_len("sr_gen_sawtooth", k)?;
    Ok(Some(
        generate::InfiniteSawtooth::new(period, high_value, low_value, delay)
            .take(k)
            .collect(),
    ))
}

/// 无限周期序列（InfinitePeriodic::new）取前 k 点。
#[duck_scalar_function(
    description = "First k points of statrs' infinite periodic generator (sampling rate, frequency, amplitude, phase, delay)",
    example = "SELECT sr_gen_periodic(8, 8.0, 1.0, 1.0, 0.0, 0)"
)]
fn sr_gen_periodic(
    k: i64,
    sampling_rate: f64,
    frequency: f64,
    amplitude: f64,
    phase: f64,
    delay: i64,
) -> DuckOptionResult<Vec<f64>> {
    let k = take_len("sr_gen_periodic", k)?;
    Ok(Some(
        generate::InfinitePeriodic::new(sampling_rate, frequency, amplitude, phase, delay)
            .take(k)
            .collect(),
    ))
}

/// 无限正弦序列（InfiniteSinusoidal::new）取前 k 点。
#[duck_scalar_function(
    description = "First k points of statrs' infinite sinusoidal generator (sampling rate, frequency, amplitude, mean, phase, delay)",
    example = "SELECT sr_gen_sinusoidal(8, 8.0, 1.0, 1.0, 0.0, 0.0, 0)"
)]
fn sr_gen_sinusoidal(
    k: i64,
    sampling_rate: f64,
    frequency: f64,
    amplitude: f64,
    mean: f64,
    phase: f64,
    delay: i64,
) -> DuckOptionResult<Vec<f64>> {
    let k = take_len("sr_gen_sinusoidal", k)?;
    Ok(Some(
        generate::InfiniteSinusoidal::new(sampling_rate, frequency, amplitude, mean, phase, delay)
            .take(k)
            .collect(),
    ))
}
