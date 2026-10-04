// statrs::distribution 的包装层（目录对应 statrs 的 src/distribution/）。
//
// 对应关系按 statrs 的文件名逐条落在下面的文件里，函数名 = `sr_` + statrs 类型的
// snake_case 名 + 方法名（pdf / ln_pdf / cdf / sf / quantile；离散为 pmf / ln_pmf /
// cdf / sf / quantile；矩与域为 mean / variance / std_dev / entropy / skewness /
// min / max / median / mode）。statrs 的 inverse_cdf 在 SQL 侧叫 quantile（数据库
// 的通用词），行为一致。
//
// 能力覆盖到 statrs 的全部导出：
//   - 一元连续 20 种、离散 7 种 → continuous_*.rs / discrete.rs（含 Distribution /
//     Min / Max / Median / Mode 四组 trait 的矩与域）；
//   - Categorical（概率向量 LIST）→ categorical.rs；
//   - Empirical（数据驱动，聚合形态）→ empirical.rs；
//   - 多元四类（Dyn 维度，LIST 向量/摊平矩阵）→ multivariate.rs（含 MVN 的
//     MeanN / VarianceN / Mode 与熵，MultivariateStudent 的 Mode）；
//   - 二项随机采样（statrs 的 BinomialAlgorithm/BinomialSampler）→ functions/sampling.rs。
// 唯一不经这里的导出是 `sample` 家族里的内部工具（见 sampling.rs 头注释）。
//
// The tree mirrors statrs::distribution; a wrapper name is `sr_` + the statrs type in
// snake_case + the method. Full export coverage: 20 continuous and 7 discrete univariate
// distributions, Categorical (probability LIST), Empirical (data-driven aggregates), the four
// multivariate ones (Dyn dimension over LIST vectors and row-major flattened matrices), and
// binomial sampling (see functions/sampling.rs).

mod categorical;
mod continuous_location_scale;
mod continuous_shape;
mod discrete;
mod empirical;
mod multivariate;

use quack_rs::error::ExtensionError;

/// 分位数函数的概率参数必须落在 `[0, 1]`：越界是调用错误，不做钳位 —— 把 1.5 静默钳成 1
/// 会掩盖写错的参数（statrs 自己的 `inverse_cdf` 假定 p 在区间内，行为未定义）。
///
/// A quantile's probability must sit in [0, 1]: out of range is a bad call, and clamping it
/// would hide the mistake (statrs' own inverse_cdf assumes an in-range p).
pub(crate) fn check_probability(fn_name: &str, p: f64) -> Result<(), ExtensionError> {
    if !(0.0..=1.0).contains(&p) {
        return Err(duckfn::duck_error(format!(
            "{fn_name}: the probability must be within [0, 1], got {p}"
        )));
    }
    Ok(())
}
