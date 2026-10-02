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
// 状态先收齐整列数据、finalize 时把切片交给 statrs 计算：语义与「在本机直接调 statrs」
// 严格一致是第一优先级，median/quantile 这类顺序统计量本来也需要全量数据（DuckDB 自带的
// quantile 同样在状态里攒数据）。
//
// Wrapping statrs::statistics as aggregates: a column in, one value out.
//
// Why aggregates instead of "LIST + scalar": `SELECT sr_mean(x) FROM t GROUP BY g` is the shape
// database users already write; the scalar form would force a `list(x)` in front of every call.
//
// NULL semantics (aligned with statrs, propagated to the SQL side): SQL NULL rows never enter
// the state (the aggregate convention, same as DuckDB's own mean/stddev); whatever statrs cannot
// define (empty group, fewer than two values for the variance family, an out-of-range tau) comes
// back as NAN and is folded into SQL NULL by `nan_to_null` — a NAN never reaches the user as a
// value.
//
// The state collects the column and hands the slice to statrs at finalize: staying strictly
// identical to calling statrs directly is the priority, and the order statistics need the full
// data anyway (DuckDB's own quantile buffers rows too).
// ============================================================================

use duckfn::{DuckAggregateState, DuckOptionResult, duck_aggregate_function};
use statrs::statistics::{Data, OrderStatistics, Statistics};

/// 生成「攒齐一列、finalize 调一次 statrs」的聚合：状态、`result` 的 NAN→NULL、行处理器
/// 三件套都是同一个形状，逐个手写只会重复九遍同样的十行。`$calc` 约定拿名为 `values` 的
/// 切片（`&[f64]` 直接落在 statrs 的 `Statistics`/`IntoIterator` 实现上）。
///
/// Generates the "collect the column, call statrs once at finalize" aggregate: state, the
/// NAN→NULL `result`, and the row handler are the same shape every time. `$calc` receives the
/// column as `values: &[f64]`, which lands directly on statrs' `Statistics` / `IntoIterator`
/// implementations.
macro_rules! collect_aggregate {
    ($state:ident, $fun:ident, $desc:literal, $comment:literal, $example:literal, $calc:expr) => {
        #[derive(Default, Debug, Clone)]
        struct $state {
            values: Vec<f64>,
        }

        impl DuckAggregateState for $state {
            type Output = f64;

            fn simple_combine(&mut self, other: &Self) {
                self.values.extend(other.values.iter().copied());
            }

            fn result(&self) -> DuckOptionResult<f64> {
                let calc: fn(&[f64]) -> f64 = $calc;
                super::nan_to_null(calc(&self.values))
            }
        }

        #[duck_aggregate_function(description = $desc, comment = $comment, example = $example)]
        fn $fun(input: f64, state: &mut $state) {
            state.values.push(input);
        }
    };
}

// 集中趋势：均值族走 `Statistics`（对切片直接算）；中位数走 `OrderStatistics`，
// 它按 `&mut` 就地选择，finalize 时给一份可重排的拷贝即可 —— 状态里的原始列不动。
collect_aggregate!(
    MeanState,
    sr_mean,
    "Arithmetic mean of a DOUBLE column, NULL when no row is non-NULL",
    "SQL NULL rows are skipped; statrs' NAN for an empty group becomes SQL NULL",
    "SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)",
    |values| values.mean()
);

collect_aggregate!(
    GeometricMeanState,
    sr_geometric_mean,
    "Geometric mean of a DOUBLE column, NULL when a value is negative or no row is non-NULL",
    "A negative value makes the statistic undefined, which comes back as NULL",
    "SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)",
    |values| values.geometric_mean()
);

collect_aggregate!(
    HarmonicMeanState,
    sr_harmonic_mean,
    "Harmonic mean of a DOUBLE column, NULL when a value is negative or no row is non-NULL",
    "A negative value makes the statistic undefined, which comes back as NULL",
    "SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)",
    |values| values.harmonic_mean()
);

collect_aggregate!(
    QuadraticMeanState,
    sr_quadratic_mean,
    "Quadratic mean (root mean square) of a DOUBLE column, NULL when no row is non-NULL",
    "SQL NULL rows are skipped; statrs' NAN for an empty group becomes SQL NULL",
    "SELECT sr_quadratic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)",
    |values| values.quadratic_mean()
);

collect_aggregate!(
    MedianState,
    sr_median,
    "Median of a DOUBLE column, NULL when no row is non-NULL",
    "Even-length inputs average the two middle values, statrs' own convention",
    "SELECT sr_median(x) FROM (VALUES (3.0), (1.0), (2.0)) t(x)",
    |values| {
        let mut data = Data::new(values.to_vec());
        data.median()
    }
);

// 离散程度：样本版（Bessel 修正、除以 N-1）与总体版（除以 N）。空组与样本不足在 statrs 里
// 都是 NAN，落到 SQL 侧就是 NULL —— 与 DuckDB 自带 var/stddev 对单行输入给 NULL 一致。
collect_aggregate!(
    VarianceState,
    sr_variance,
    "Sample variance of a DOUBLE column (Bessel-corrected), NULL when fewer than two rows are non-NULL",
    "One value has no sample variance; the result is NULL, not 0",
    "SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)",
    |values| values.variance()
);

collect_aggregate!(
    StdDevState,
    sr_std_dev,
    "Sample standard deviation of a DOUBLE column (Bessel-corrected), NULL when fewer than two rows are non-NULL",
    "One value has no sample standard deviation; the result is NULL, not 0",
    "SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)",
    |values| values.std_dev()
);

collect_aggregate!(
    PopulationVarianceState,
    sr_population_variance,
    "Population variance of a DOUBLE column, NULL when no row is non-NULL",
    "Single-row groups yield a real number here (dividing by N), unlike the sample family",
    "SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)",
    |values| values.population_variance()
);

collect_aggregate!(
    PopulationStdDevState,
    sr_population_std_dev,
    "Population standard deviation of a DOUBLE column, NULL when no row is non-NULL",
    "Single-row groups yield a real number here (dividing by N), unlike the sample family",
    "SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)",
    |values| values.population_std_dev()
);

/// tau 分位数：`sr_quantile(x, 0.5)` 的第二个参数按常量写（聚合参数是逐行的列，
/// `0.975` 这样的字面量每行都是同一个值）。tau 为 NULL 的行整行不进状态（参数读取层
/// 跳过 NULL 行），与「NULL 输入不进函数体」的标量规则同源。
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
