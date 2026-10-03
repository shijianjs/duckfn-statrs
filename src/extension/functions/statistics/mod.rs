// statrs::statistics 的包装层（目录对应 statrs 的 src/statistics/）：
//
//   statrs                                duckfn_statrs
//   Statistics trait（mean…covariance）    summary.rs / covariance.rs（聚合）
//   OrderStatistics trait                 order.rs（聚合）
//   Median/Mode trait、OnlineMoments、
//   Accumulate                            有意不重复包装（见各文件头注释）
//
// 形状统一：`auto_collect` 聚合 —— 收集整列，finalize 一次性委托给 statrs。

mod covariance;
mod order;
mod summary;
