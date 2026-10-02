// ============================================================================
// 协方差聚合：两列成对进、一个值出
//
// 两列参数按行配对：任何一列是 NULL 的行整行不进状态（聚合的 NULL 跳过规则），所以
// 「两条边长度不等」在这个形态下根本构造不出来 —— LIST + 标量的形态才需要手工检查
// 长度并报错（statrs 对不等长是直接 panic 的）。
//
// 形状与 aggregate_summary.rs 相同：一份共享状态 `PairState<P>`，具体算样本还是总体协方差
// 由类型参数在编译期静态派发（marker struct + `Pairwise` trait），不用宏。
//
// NULL 语义同样对齐 statrs：配对后不足 2 行返回 NAN，统一折成 SQL NULL。
//
// Covariance aggregates: two columns in, paired row by row.
//
// A NULL in either column skips the whole row, so the "unequal-length" case that statrs panics on
// is unconstructible in this shape — the row pairing itself takes care of it.
//
// Same structure as aggregate_summary.rs: one shared state (`PairState<P>`), the concrete statistic
// dispatched statically by the type parameter. No macro.
// ============================================================================

use duckfn::{DuckAggregateState, DuckOptionResult, duck_aggregate_function};
use statrs::statistics::Statistics;
use std::marker::PhantomData;

/// 「两条配对切片算一个统计量」的静态派发点。
///
/// The static dispatch point for "compute one statistic over two paired columns".
trait Pairwise {
    fn eval(xs: &[f64], ys: &[f64]) -> f64;
}

#[derive(Default, Debug, Clone, Copy)] struct SampleCovariance;
#[derive(Default, Debug, Clone, Copy)] struct PopulationCovariance;

// 样本版带 Bessel 修正（除以 N-1），总体版除以 N；与单列的方差族一一对应。
impl Pairwise for SampleCovariance {
    fn eval(xs: &[f64], ys: &[f64]) -> f64 {
        xs.covariance(ys)
    }
}

impl Pairwise for PopulationCovariance {
    fn eval(xs: &[f64], ys: &[f64]) -> f64 {
        xs.population_covariance(ys)
    }
}

/// 两条配对列共用的状态：按行成对收集，finalize 时按类型参数派发。
///
/// The state shared by the paired aggregates: collect rows in pairs, dispatch at finalize.
#[derive(Default, Debug, Clone)]
struct PairState<P: Pairwise> {
    xs: Vec<f64>,
    ys: Vec<f64>,
    _marker: PhantomData<P>,
}

impl<P: Pairwise> DuckAggregateState for PairState<P> {
    type Output = f64;

    fn simple_combine(&mut self, other: &Self) {
        self.xs.extend(other.xs.iter().copied());
        self.ys.extend(other.ys.iter().copied());
    }

    fn result(&self) -> DuckOptionResult<f64> {
        super::nan_to_null(P::eval(&self.xs, &self.ys))
    }
}

/// `sr_covariance(x, y)`：样本协方差（`Statistics::covariance`，Bessel 修正）。
///
/// ```sql
/// SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y);  -- -5.5
/// ```
#[duck_aggregate_function(
    description = "Sample covariance of two DOUBLE columns (Bessel-corrected), NULL when fewer than two rows are fully non-NULL",
    comment = "A row with a NULL in either column is skipped entirely, keeping the two columns paired",
    example = "SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)"
)]
fn sr_covariance(x: f64, y: f64, state: &mut PairState<SampleCovariance>) {
    state.xs.push(x);
    state.ys.push(y);
}

/// `sr_population_covariance(x, y)`：总体协方差（除以 N）。
///
/// ```sql
/// SELECT sr_population_covariance(x, y)
/// FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y);  -- -11/3
/// ```
#[duck_aggregate_function(
    description = "Population covariance of two DOUBLE columns, NULL when no row is fully non-NULL",
    comment = "A row with a NULL in either column is skipped entirely, keeping the two columns paired",
    example = "SELECT sr_population_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)"
)]
fn sr_population_covariance(x: f64, y: f64, state: &mut PairState<PopulationCovariance>) {
    state.xs.push(x);
    state.ys.push(y);
}
