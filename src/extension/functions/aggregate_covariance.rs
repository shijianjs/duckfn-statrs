// ============================================================================
// 协方差聚合：两列成对进、一个值出
//
// 两列参数按行配对：任何一列是 NULL 的行整行不进状态（聚合的 NULL 跳过规则），所以
// 「两条边长度不等」在这个形态下根本构造不出来 —— 之前 LIST + 标量的版本需要手工检查
// 长度并报错，statrs 对不等长是直接 panic 的；聚合把这件事交给了行配对本身。
//
// NULL 语义与 aggregate_summary.rs 相同：statrs 算不出（配对后不足 2 行）返回 NAN，
// 统一折成 SQL NULL。
//
// Covariance aggregates: two columns in, paired row by row.
//
// A NULL in either column skips the whole row (the aggregate's NULL-skip rule), so the
// "unequal-length lists" case that the LIST + scalar version had to check by hand (statrs
// panics on it) is unconstructible here — the row pairing itself takes care of it.
//
// NULL semantics as in aggregate_summary.rs: whatever statrs cannot define (fewer than two
// paired rows) comes back as NAN and is folded into SQL NULL.
// ============================================================================

use duckfn::{DuckAggregateState, DuckOptionResult, duck_aggregate_function};
use statrs::statistics::Statistics;

/// 生成「成对攒两列、finalize 把两条切片交给 statrs」的协方差聚合，形状同
/// `aggregate_summary.rs` 的 `collect_aggregate!`，只是状态多一条边。
///
/// Generates the "pair two columns, hand both slices to statrs at finalize" covariance
/// aggregate; same shape as `collect_aggregate!` in `aggregate_summary.rs`, one more leg.
macro_rules! collect_covariance {
    ($state:ident, $fun:ident, $desc:literal, $comment:literal, $example:literal, $calc:expr) => {
        #[derive(Default, Debug, Clone)]
        struct $state {
            xs: Vec<f64>,
            ys: Vec<f64>,
        }

        impl DuckAggregateState for $state {
            type Output = f64;

            fn simple_combine(&mut self, other: &Self) {
                self.xs.extend(other.xs.iter().copied());
                self.ys.extend(other.ys.iter().copied());
            }

            fn result(&self) -> DuckOptionResult<f64> {
                let calc: fn(&[f64], &[f64]) -> f64 = $calc;
                super::nan_to_null(calc(&self.xs, &self.ys))
            }
        }

        #[duck_aggregate_function(description = $desc, comment = $comment, example = $example)]
        fn $fun(x: f64, y: f64, state: &mut $state) {
            state.xs.push(x);
            state.ys.push(y);
        }
    };
}

// 样本版带 Bessel 修正（除以 N-1），总体版除以 N；与单列的方差族一一对应。
collect_covariance!(
    CovarianceState,
    sr_covariance,
    "Sample covariance of two DOUBLE columns (Bessel-corrected), NULL when fewer than two rows are fully non-NULL",
    "A row with a NULL in either column is skipped entirely, keeping the two columns paired",
    "SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)",
    |xs, ys| xs.covariance(ys)
);

collect_covariance!(
    PopulationCovarianceState,
    sr_population_covariance,
    "Population covariance of two DOUBLE columns, NULL when no row is fully non-NULL",
    "A row with a NULL in either column is skipped entirely, keeping the two columns paired",
    "SELECT sr_population_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)",
    |xs, ys| xs.population_covariance(ys)
);
