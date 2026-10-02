// ============================================================================
// statrs::statistics 的 LIST 版包装：一组统计量一个标量函数
//
// 输入是 `Vec<f64>`（SQL `LIST(DOUBLE) NOT NULL`）：整个列表为 NULL 时结果 NULL，列表里出现
// NULL 元素时同样整行 NULL（duckfn 的参数读取层短路）。这些函数是「一列进、一个值出」的
// 标量，不承担 SQL 聚合「跳过 NULL」的语义；想跳过就把列先聚成列表时过滤掉 NULL：
// `stat_mean(list(x) FILTER (WHERE x IS NOT NULL))`（DuckDB 的 `list()` 本身是保留 NULL 的）。
//
// statrs 对「算不出」一律返回 NAN（空列表、方差类输入少于 2 个元素、几何/调和平均遇到负数、
// quantile 的 tau 越界……），这里统一折算成 SQL NULL（见 `nan_to_null`）；无穷值是合法结果，
// 原样保留。
//
// The input is `Vec<f64>` (SQL `LIST(DOUBLE) NOT NULL`): a NULL list, or a NULL element inside
// one, yields a NULL row (short-circuited by duckfn's argument reader). These are scalars over a
// whole column, not SQL aggregates, so they do not skip NULLs — and note that DuckDB's `list()`
// keeps NULLs too, so filter while aggregating:
// `stat_mean(list(x) FILTER (WHERE x IS NOT NULL))`.
//
// statrs signals "undefined" with NAN (empty input, fewer than two elements for the variance
// family, negatives in the geometric/harmonic means, an out-of-range tau for quantile, ...);
// those all become SQL NULL here (see `nan_to_null`), while infinities stay as they are.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use statrs::statistics::{Data, OrderStatistics, Statistics};

/// statrs 的 NAN（无定义）折算成 SQL NULL；其余值原样通过。
///
/// Turns statrs' NAN (undefined) into SQL NULL and lets every other value through unchanged.
fn nan_to_null(value: f64) -> DuckOptionResult<f64> {
    if value.is_nan() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

/// 算术平均（`statrs::statistics::Statistics::mean`）。
///
/// ```sql
/// SELECT stat_mean([1.0, 2.0, 3.0]);        -- 2.0
/// SELECT stat_mean([]::DOUBLE[]);           -- NULL（空列表无均值）
/// ```
#[duck_scalar_function(
    description = "Arithmetic mean of a DOUBLE list, NULL when the list is empty",
    example = "SELECT stat_mean([1.0, 2.0, 3.0])"
)]
fn stat_mean(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.mean())
}

/// 几何平均（`Statistics::geometric_mean`）：含负数时 NULL，含 0 时 0。
///
/// ```sql
/// SELECT stat_geometric_mean([1.0, 2.0, 3.0]);  -- 1.8171205928321397
/// SELECT stat_geometric_mean([-1.0, 2.0]);      -- NULL
/// ```
#[duck_scalar_function(
    description = "Geometric mean of a DOUBLE list, NULL when it holds a negative value or is empty",
    example = "SELECT stat_geometric_mean([1.0, 2.0, 3.0])"
)]
fn stat_geometric_mean(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.geometric_mean())
}

/// 调和平均（`Statistics::harmonic_mean`）：含负数时 NULL，含 0 时 0。
///
/// ```sql
/// SELECT stat_harmonic_mean([1.0, 2.0, 3.0]);  -- 1.6363636363636365
/// ```
#[duck_scalar_function(
    description = "Harmonic mean of a DOUBLE list, NULL when it holds a negative value or is empty",
    example = "SELECT stat_harmonic_mean([1.0, 2.0, 3.0])"
)]
fn stat_harmonic_mean(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.harmonic_mean())
}

/// 平方均值 / RMS（`Statistics::quadratic_mean`）。
///
/// ```sql
/// SELECT stat_quadratic_mean([1.0, 2.0, 3.0]);  -- 2.160246899469287
/// ```
#[duck_scalar_function(
    description = "Quadratic mean (root mean square) of a DOUBLE list, NULL when it is empty",
    example = "SELECT stat_quadratic_mean([1.0, 2.0, 3.0])"
)]
fn stat_quadratic_mean(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.quadratic_mean())
}

/// 样本中位数（`statrs::statistics::OrderStatistics::median`，就地选择算法，不整体排序）。
///
/// 与 `Vec<f64>` 上的 `Statistics` 不同，顺序统计量要先把列表包进 `Data`：它按 `&mut` 工作，
/// 归一化时直接重排这份数据，函数返回即释放。
///
/// ```sql
/// SELECT stat_median([3.0, 1.0, 2.0]);  -- 2.0
/// SELECT stat_median([]::DOUBLE[]);     -- NULL
/// ```
#[duck_scalar_function(
    description = "Median of a DOUBLE list, NULL when the list is empty",
    example = "SELECT stat_median([3.0, 1.0, 2.0])"
)]
fn stat_median(xs: Vec<f64>) -> DuckOptionResult<f64> {
    let mut data = Data::new(xs);
    nan_to_null(data.median())
}

/// tau 分位数（`OrderStatistics::quantile`）：tau 须在 `[0, 1]` 内，越界返回 NULL。
///
/// ```sql
/// SELECT stat_quantile([1.0, 2.0, 3.0, 4.0], 0.5);  -- 2.5
/// SELECT stat_quantile([1.0, 2.0], 1.5);             -- NULL（tau 越界）
/// ```
#[duck_scalar_function(
    description = "Tau quantile of a DOUBLE list, NULL when the list is empty or tau is not in [0, 1]",
    example = "SELECT stat_quantile([1.0, 2.0, 3.0, 4.0], 0.5)"
)]
fn stat_quantile(xs: Vec<f64>, tau: f64) -> DuckOptionResult<f64> {
    let mut data = Data::new(xs);
    nan_to_null(data.quantile(tau))
}

/// 样本方差（`Statistics::variance`，Bessel 修正、除以 N-1）：不足 2 个元素时 NULL。
///
/// ```sql
/// SELECT stat_variance([0.0, 3.0, -2.0]);  -- 6.333333333333333
/// SELECT stat_variance([1.0]);             -- NULL
/// ```
#[duck_scalar_function(
    description = "Sample variance of a DOUBLE list (Bessel-corrected), NULL when it holds fewer than two values",
    example = "SELECT stat_variance([0.0, 3.0, -2.0])"
)]
fn stat_variance(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.variance())
}

/// 样本标准差（`Statistics::std_dev`）：不足 2 个元素时 NULL。
///
/// ```sql
/// SELECT stat_std_dev([0.0, 3.0, -2.0]);  -- 2.516611478423583
/// ```
#[duck_scalar_function(
    description = "Sample standard deviation of a DOUBLE list (Bessel-corrected), NULL when it holds fewer than two values",
    example = "SELECT stat_std_dev([0.0, 3.0, -2.0])"
)]
fn stat_std_dev(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.std_dev())
}

/// 总体方差（`Statistics::population_variance`，除以 N）。
///
/// ```sql
/// SELECT stat_population_variance([0.0, 3.0, -2.0]);  -- 4.222222222222222
/// ```
#[duck_scalar_function(
    description = "Population variance of a DOUBLE list, NULL when the list is empty",
    example = "SELECT stat_population_variance([0.0, 3.0, -2.0])"
)]
fn stat_population_variance(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.population_variance())
}

/// 总体标准差（`Statistics::population_std_dev`）。
///
/// ```sql
/// SELECT stat_population_std_dev([0.0, 3.0, -2.0]);  -- 2.0548046676563256
/// ```
#[duck_scalar_function(
    description = "Population standard deviation of a DOUBLE list, NULL when the list is empty",
    example = "SELECT stat_population_std_dev([0.0, 3.0, -2.0])"
)]
fn stat_population_std_dev(xs: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(xs.population_std_dev())
}

/// 样本协方差（`Statistics::covariance`，Bessel 修正）。statrs 对「两列长度不等」是直接 panic
/// 的，所以先自己检查、给一条正常报错；长度相等但为空时返回 NULL。
///
/// ```sql
/// SELECT stat_covariance([0.0, 3.0, -2.0], [-5.0, 4.0, 10.0]);  -- -5.5
/// SELECT stat_covariance([1.0, 2.0], [1.0]);                    -- 报错（长度不等）
/// ```
#[duck_scalar_function(
    description = "Sample covariance of two DOUBLE lists (Bessel-corrected), failing when their lengths differ",
    comment = "Length mismatch is a query error; equal-but-shorter-than-two lists yield NULL",
    example = "SELECT stat_covariance([0.0, 3.0, -2.0], [-5.0, 4.0, 10.0])"
)]
fn stat_covariance(xs: Vec<f64>, ys: Vec<f64>) -> DuckOptionResult<f64> {
    if xs.len() != ys.len() {
        return Err(duck_error(format!(
            "stat_covariance: the two lists must have the same length, got {} and {}",
            xs.len(),
            ys.len()
        )));
    }
    nan_to_null(xs.covariance(ys))
}

/// 总体协方差（`Statistics::population_covariance`）。长度检查同上。
///
/// ```sql
/// SELECT stat_population_covariance([0.0, 3.0, -2.0], [-5.0, 4.0, 10.0]);  -- -3.6666666666666665
/// ```
#[duck_scalar_function(
    description = "Population covariance of two DOUBLE lists, failing when their lengths differ",
    comment = "Length mismatch is a query error; equal-but-empty lists yield NULL",
    example = "SELECT stat_population_covariance([0.0, 3.0, -2.0], [-5.0, 4.0, 10.0])"
)]
fn stat_population_covariance(xs: Vec<f64>, ys: Vec<f64>) -> DuckOptionResult<f64> {
    if xs.len() != ys.len() {
        return Err(duck_error(format!(
            "stat_population_covariance: the two lists must have the same length, got {} and {}",
            xs.len(),
            ys.len()
        )));
    }
    nan_to_null(xs.population_covariance(ys))
}
