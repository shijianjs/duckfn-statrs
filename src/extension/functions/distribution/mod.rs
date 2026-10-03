// statrs::distribution 的包装层（目录对应 statrs 的 src/distribution/）。
//
// 对应关系按 statrs 的文件名逐条落在下面三个文件里，函数名 = `sr_` + statrs 类型的
// snake_case 名 + 方法名（pdf / ln_pdf / cdf / sf / quantile；离散为 pmf / ln_pmf /
// cdf / sf / quantile）。statrs 的 inverse_cdf 在 SQL 侧叫 quantile（数据库的通用词），
// 行为一致。
//
// 有意不包装的（与 statrs 的差额，核对时看这里）：
//   - Categorical：参数是概率向量 &[f64]，不是标量 DOUBLE；
//   - Dirichlet / Multinomial / MultivariateNormal / MultivariateStudent：多元分布，
//     参数是向量/矩阵，不匹配「对外只有 DOUBLE」的约定；
//   - Empirical：吃整份数据集，等样本类接口和聚合形态一起设计；
//   - binomial 的 sampler / sampling 模块：扩展是确定性求值，不做随机采样。
//
// The tree mirrors statrs::distribution; a wrapper name is `sr_` + the statrs type in
// snake_case + the method. Deliberately not wrapped: vector/matrix-parameter distributions
// (Categorical, Dirichlet, Multinomial, the multivariate ones), Empirical (takes a whole
// dataset), and the samplers (this extension evaluates, it does not draw).

mod continuous_location_scale;
mod continuous_shape;
mod discrete;

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
