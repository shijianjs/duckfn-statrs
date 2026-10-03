// ============================================================================
// statrs::density 的包装：点估计密度（kde_pdf / knn_pdf，需要 statrs 的 kde feature）
//
// statrs 的签名对「点」与「样本集」都要求 Container + AsRef<[f64]>：一维下把
// SQL 的 x 包成 [f64; 1]、samples 的 LIST(DOUBLE) 转成 Vec<[f64; 1]>，k-d tree 即
// 跑在一维点上。bandwidth 为 NULL 时走 statrs 的自动带宽（orava_optimal_k）。
// 两者返回 (estimate, radius?) 的估计值部分；knn_pdf 是同一入口的 k-近邻变体。
//
// statrs::density: kernel and k-nearest-neighbour density estimation. One-dimensional
// points are wrapped as [f64; 1] so the k-d tree gets statrs' Container types; a NULL
// bandwidth selects statrs' automatic choice.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};
use statrs::density::{kde::kde_pdf, knn::knn_pdf};

/// 一维点集：LIST(DOUBLE) → Vec<[f64; 1]>（density 的 Container 形状）。
///
/// A one-dimensional point set: the LIST becomes Vec<[f64; 1]>, the shape density's
/// Container bounds want.
fn points(samples: Vec<f64>) -> Vec<[f64; 1]> {
    samples.into_iter().map(|s| [s]).collect()
}

/// 高斯核密度估计 f̂(x)；样本为空或邻域为空是 statrs 的错误，报查询错误。
/// bandwidth 为 NULL 时走 statrs 自动带宽 —— special_null_handling 让常量 NULL 进到函数体
/// 而不是被 binder 折叠成 NULL。
#[duck_scalar_function(
    special_null_handling = true,
    description = "Kernel density estimate at x from a LIST(DOUBLE) sample; NULL bandwidth selects statrs' automatic bandwidth",
    comment = "Gaussian kernel over a k-d tree; errors (empty sample, empty neighbourhood) fail the query",
    example = "SELECT sr_kde_pdf(0.5, [0.0, 0.2, 0.7, 1.0], 0.3)"
)]
fn sr_kde_pdf(
    x: f64,
    samples: Vec<f64>,
    bandwidth: Option<f64>,
) -> DuckOptionResult<f64> {
    let samples = points(samples);
    kde_pdf(&[x], &samples, bandwidth)
        .map(Some)
        .map_err(|e| duck_error(format!("sr_kde_pdf: {e}")))
}

/// k-近邻密度估计 f̂(x)；bandwidth 为 NULL 时由样本量自动选 k。
#[duck_scalar_function(
    special_null_handling = true,
    description = "K-nearest-neighbour density estimate at x from a LIST(DOUBLE) sample; NULL bandwidth selects the automatic k",
    example = "SELECT sr_knn_pdf(0.5, [0.0, 0.2, 0.7, 1.0], 0.3)"
)]
fn sr_knn_pdf(
    x: f64,
    samples: Vec<f64>,
    bandwidth: Option<f64>,
) -> DuckOptionResult<f64> {
    let samples = points(samples);
    knn_pdf(&[x], &samples, bandwidth)
        .map(Some)
        .map_err(|e| duck_error(format!("sr_knn_pdf: {e}")))
}
