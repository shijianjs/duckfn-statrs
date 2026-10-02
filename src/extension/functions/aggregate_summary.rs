// ============================================================================
// statrs::statistics 的聚合版包装：一列进、一个值出
//
// 为什么是聚合而不是「LIST + 标量」：`SELECT sr_mean(x) FROM t GROUP BY g` 是数据库用户
// 写统计查询的默认形状，换成标量就得先 `list(x)` 再喂给函数，SQL 更长、也更绕。
//
// NULL 语义（与 statrs 对齐、向 SQL 用户侧传播）：
//   - SQL NULL 行不进状态（聚合惯例，与 DuckDB 自带的 mean/stddev 一致）；
//   - statrs 算不出的（空组、方差类不足 2 个值、quantile 的 tau 越界…）返回 NAN，
//     统一经 `nan_to_null` 折成 SQL NULL —— NAN 不会作为值出现在结果里；
//   - 整组没有一行有效数据时，结果同样是 NULL（空 slice 上 statrs 一律给 NAN）。
//
// 结构：状态只有一份（`SummaryState<S>`），具体算哪个统计量由类型参数 `S` 在编译期
// 静态派发（marker struct + `Summary` trait）—— 不用宏：宏生成的代码块类型不安全、
// IDE 也不补全，而这里「收集整列 + 换最后一跳算式」本来就只有一个形状。
// 每个注册函数剩下的一行 push，是 duckfn 宏要求「函数即行处理器」的固有成本。
//
// 状态先收齐整列数据、finalize 时把切片交给 statrs 计算：语义与「在本机直接调 statrs」
// 严格一致是第一优先级，median/quantile 这类顺序统计量本来也需要全量数据（DuckDB 自带的
// quantile 同样在状态里攒数据）。
//
// Wrapping statrs::statistics as aggregates: a column in, one value out.
//
// Structure: there is a single state (`SummaryState<S>`) and the concrete statistic is dispatched
// statically at compile time by the type parameter `S` (marker structs + the `Summary` trait). No
// macro: generated code is type-unsafe and invisible to the IDE, and "collect the column, swap the
// last hop" has exactly one shape. The one-line push per registered function is the inherent cost
// of duckfn's "the function is the row handler".
//
// NULL semantics as before: SQL NULL rows never enter the state; whatever statrs spells NAN
// (empty group, fewer than two samples, out-of-range tau, negatives in the geometric/harmonic
// means) becomes SQL NULL through the shared `nan_to_null`.
// ============================================================================

use duckfn::{DuckAggregateState, DuckOptionResult, duck_aggregate_function};
use statrs::statistics::{Data, OrderStatistics, Statistics};
use std::marker::PhantomData;

/// 「一列切片算一个汇总统计量」的静态派发点。实现体只有委托给 statrs 的一行。
///
/// The static dispatch point for "compute one summary statistic over a column". Each body is the
/// one-line delegation to statrs.
trait Summary {
    fn eval(values: &[f64]) -> f64;
}

// marker 类型本身不带任何逻辑，存在的意义就是让 `SummaryState<ArithmeticMean>` 与
// `SummaryState<Median>` 成为两个可分别注册的类型。
#[derive(Default, Debug, Clone, Copy)] struct ArithmeticMean;
#[derive(Default, Debug, Clone, Copy)] struct GeometricMean;
#[derive(Default, Debug, Clone, Copy)] struct HarmonicMean;
#[derive(Default, Debug, Clone, Copy)] struct QuadraticMean;
#[derive(Default, Debug, Clone, Copy)] struct Median;
#[derive(Default, Debug, Clone, Copy)] struct SampleVariance;
#[derive(Default, Debug, Clone, Copy)] struct SampleStdDev;
#[derive(Default, Debug, Clone, Copy)] struct PopulationVariance;
#[derive(Default, Debug, Clone, Copy)] struct PopulationStdDev;

// 集中趋势：均值族对切片直接算（`Statistics` 落在所有 `IntoIterator<Item = Borrow<f64>>`
// 上，`&[f64]` 即是）；中位数走 `OrderStatistics`，它按 `&mut` 就地选择，finalize 时给一份
// 可重排的拷贝 —— 状态里收集的原始列不动。
impl Summary for ArithmeticMean {
    fn eval(values: &[f64]) -> f64 {
        values.mean()
    }
}

impl Summary for GeometricMean {
    fn eval(values: &[f64]) -> f64 {
        values.geometric_mean()
    }
}

impl Summary for HarmonicMean {
    fn eval(values: &[f64]) -> f64 {
        values.harmonic_mean()
    }
}

impl Summary for QuadraticMean {
    fn eval(values: &[f64]) -> f64 {
        values.quadratic_mean()
    }
}

impl Summary for Median {
    fn eval(values: &[f64]) -> f64 {
        let mut data = Data::new(values.to_vec());
        data.median()
    }
}

// 离散程度：样本版（Bessel 修正、除以 N-1）与总体版（除以 N）。空组与样本不足在 statrs 里
// 都是 NAN，落到 SQL 侧就是 NULL —— 与 DuckDB 自带 var/stddev 对单行输入给 NULL 一致。
impl Summary for SampleVariance {
    fn eval(values: &[f64]) -> f64 {
        values.variance()
    }
}

impl Summary for SampleStdDev {
    fn eval(values: &[f64]) -> f64 {
        values.std_dev()
    }
}

impl Summary for PopulationVariance {
    fn eval(values: &[f64]) -> f64 {
        values.population_variance()
    }
}

impl Summary for PopulationStdDev {
    fn eval(values: &[f64]) -> f64 {
        values.population_std_dev()
    }
}

/// 全部汇总统计量共用的状态：收集整列，finalize 时按类型参数派发。marker 只活在类型里，
/// 字段用 `PhantomData` 占位。
///
/// The state shared by every summary aggregate: collect the column, dispatch on the type parameter
/// at finalize. The marker lives only in the type; the field is a `PhantomData` placeholder.
#[derive(Default, Debug, Clone)]
struct SummaryState<S: Summary> {
    values: Vec<f64>,
    _marker: PhantomData<S>,
}

impl<S: Summary> DuckAggregateState for SummaryState<S> {
    type Output = f64;

    fn simple_combine(&mut self, other: &Self) {
        self.values.extend(other.values.iter().copied());
    }

    fn result(&self) -> DuckOptionResult<f64> {
        super::nan_to_null(S::eval(&self.values))
    }
}

/// `sr_mean(x)`：算术平均（`Statistics::mean`）。
///
/// ```sql
/// SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);  -- 2.0
/// SELECT sr_mean(x) FROM range(0) t(x);                      -- NULL（空组）
/// ```
#[duck_aggregate_function(
    description = "Arithmetic mean of a DOUBLE column, NULL when no row is non-NULL",
    comment = "SQL NULL rows are skipped; statrs' NAN for an empty group becomes SQL NULL",
    example = "SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_mean(input: f64, state: &mut SummaryState<ArithmeticMean>) {
    state.values.push(input);
}

/// `sr_geometric_mean(x)`：几何平均（`Statistics::geometric_mean`），含负数 NULL、含 0 出 0。
///
/// ```sql
/// SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);  -- 1.8171205928321397
/// SELECT sr_geometric_mean(x) FROM (VALUES (-1.0), (2.0)) t(x);        -- NULL
/// ```
#[duck_aggregate_function(
    description = "Geometric mean of a DOUBLE column, NULL when a value is negative or no row is non-NULL",
    comment = "A negative value makes the statistic undefined, which comes back as NULL",
    example = "SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_geometric_mean(input: f64, state: &mut SummaryState<GeometricMean>) {
    state.values.push(input);
}

/// `sr_harmonic_mean(x)`：调和平均（`Statistics::harmonic_mean`）。
///
/// ```sql
/// SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);  -- 1.6363636363636365
/// ```
#[duck_aggregate_function(
    description = "Harmonic mean of a DOUBLE column, NULL when a value is negative or no row is non-NULL",
    comment = "A negative value makes the statistic undefined, which comes back as NULL",
    example = "SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_harmonic_mean(input: f64, state: &mut SummaryState<HarmonicMean>) {
    state.values.push(input);
}

/// `sr_quadratic_mean(x)`：平方均值 / RMS（`Statistics::quadratic_mean`）。
///
/// ```sql
/// SELECT sr_quadratic_mean(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 2.0816659994661326
/// ```
#[duck_aggregate_function(
    description = "Quadratic mean (root mean square) of a DOUBLE column, NULL when no row is non-NULL",
    comment = "SQL NULL rows are skipped; statrs' NAN for an empty group becomes SQL NULL",
    example = "SELECT sr_quadratic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_quadratic_mean(input: f64, state: &mut SummaryState<QuadraticMean>) {
    state.values.push(input);
}

/// `sr_median(x)`：中位数（`OrderStatistics::median`，就地选择算法），偶数个取中间两数平均。
///
/// ```sql
/// SELECT sr_median(x) FROM (VALUES (-1.0), (5.0), (0.0), (-3.0), (10.0), (-0.5), (4.0), (1.0), (6.0)) t(x);  -- 1.0
/// SELECT sr_median(x) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x);  -- 2.5
/// ```
#[duck_aggregate_function(
    description = "Median of a DOUBLE column, NULL when no row is non-NULL",
    comment = "Even-length inputs average the two middle values, statrs' own convention",
    example = "SELECT sr_median(x) FROM (VALUES (3.0), (1.0), (2.0)) t(x)"
)]
fn sr_median(input: f64, state: &mut SummaryState<Median>) {
    state.values.push(input);
}

/// `sr_variance(x)`：样本方差（`Statistics::variance`，Bessel 修正、除以 N-1）。
///
/// ```sql
/// SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 19/3
/// SELECT sr_variance(x) FROM (VALUES (1.0)) t(x);                 -- NULL
/// ```
#[duck_aggregate_function(
    description = "Sample variance of a DOUBLE column (Bessel-corrected), NULL when fewer than two rows are non-NULL",
    comment = "One value has no sample variance; the result is NULL, not 0",
    example = "SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_variance(input: f64, state: &mut SummaryState<SampleVariance>) {
    state.values.push(input);
}

/// `sr_std_dev(x)`：样本标准差。
///
/// ```sql
/// SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- sqrt(19/3)
/// ```
#[duck_aggregate_function(
    description = "Sample standard deviation of a DOUBLE column (Bessel-corrected), NULL when fewer than two rows are non-NULL",
    comment = "One value has no sample standard deviation; the result is NULL, not 0",
    example = "SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_std_dev(input: f64, state: &mut SummaryState<SampleStdDev>) {
    state.values.push(input);
}

/// `sr_population_variance(x)`：总体方差（除以 N）。
///
/// ```sql
/// SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- 38/9
/// ```
#[duck_aggregate_function(
    description = "Population variance of a DOUBLE column, NULL when no row is non-NULL",
    comment = "Single-row groups yield a real number here (dividing by N), unlike the sample family",
    example = "SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_population_variance(input: f64, state: &mut SummaryState<PopulationVariance>) {
    state.values.push(input);
}

/// `sr_population_std_dev(x)`：总体标准差（除以 N）。
///
/// ```sql
/// SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x);  -- sqrt(38/9)
/// ```
#[duck_aggregate_function(
    description = "Population standard deviation of a DOUBLE column, NULL when no row is non-NULL",
    comment = "Single-row groups yield a real number here (dividing by N), unlike the sample family",
    example = "SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)"
)]
fn sr_population_std_dev(input: f64, state: &mut SummaryState<PopulationStdDev>) {
    state.values.push(input);
}

/// `sr_quantile(x, tau)`：tau 分位数（`OrderStatistics::quantile`）。第二个参数按常量写
/// （聚合参数是逐行的列，`0.975` 这样的字面量每行都是同一个值）。tau 为 NULL 的行整行
/// 不进状态，与「NULL 输入不进函数体」的标量规则同源。
///
/// 与均值族不同，状态里要多带一个 tau，所以这个函数不走 `SummaryState`，手写。
///
/// ```sql
/// SELECT sr_quantile(x, 0.5) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x);  -- 2.5
/// SELECT sr_quantile(x, 1.5) FROM (VALUES (1.0), (2.0)) t(x);                -- NULL（tau 越界）
/// ```
#[duck_aggregate_function(
    description = "Tau quantile of a DOUBLE column, tau as the second (constant) argument, NULL when empty or tau is not in [0, 1]",
    comment = "Write the tau argument as a literal; a NULL tau skips the row entirely, like any NULL input",
    example = "SELECT sr_quantile(x, 0.5) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)"
)]
fn sr_quantile(input: f64, tau: f64, state: &mut QuantileState) {
    state.values.push(input);
    state.tau = Some(tau);
}

/// 分位数状态：数据 + 每行覆写的 tau。并行聚合合并分区时任取一份即可（同一条查询里
/// 常量的值处处相同）。
///
/// The quantile state: the data plus the per-row overwrite of tau. Merging partitions keeps
/// whichever copy — the constant is the same on every row of one query.
#[derive(Default, Debug, Clone)]
struct QuantileState {
    values: Vec<f64>,
    tau: Option<f64>,
}

impl DuckAggregateState for QuantileState {
    type Output = f64;

    fn simple_combine(&mut self, other: &Self) {
        self.values.extend(other.values.iter().copied());
        self.tau = self.tau.or(other.tau);
    }

    fn result(&self) -> DuckOptionResult<f64> {
        // 没有一行进过状态（空组，或 tau/输入全是 NULL）——没有结果可报。
        // Nothing ever entered the state (empty group, or every row NULL) — nothing to report.
        let Some(tau) = self.tau else {
            return Ok(None);
        };
        let mut data = Data::new(self.values.clone());
        super::nan_to_null(data.quantile(tau))
    }
}
