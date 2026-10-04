---
title: 简介
sidebar_position: 1
slug: /intro
description: duckfn_statrs 为 DuckDB SQL 带来 252 个统计函数——描述统计量聚合、27 种分布、特殊函数、随机抽样与假设检验。
---

# 简介

`duckfn_statrs` 是一个 DuckDB [loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)，
把 Rust 统计计算库 [statrs](https://crates.io/crates/statrs) 的全部能力包装成可直接在 SQL 里调用的
函数。它在 `sr_` 前缀下注册了 **252 个函数**：

| 类别 | 类型 | 数量 | 示例 |
| --- | --- | --- | --- |
| 描述统计量 | 聚合 | 26 | `sr_mean`、`sr_median`、`sr_variance`、`sr_covariance` |
| 连续分布 | 标量 | 98 | `sr_normal_pdf`、`sr_gamma_cdf`、`sr_beta_quantile` |
| 离散分布 | 标量 | 40 | `sr_poisson_pmf`、`sr_binomial_cdf`、`sr_geometric_quantile` |
| 特殊函数 | 标量 | 29 | `sr_gamma`、`sr_erf`、`sr_ln_choose` |
| 常量 | 标量 | 7 | `sr_ln_pi`、`sr_sqrt_2pi`、`sr_euler_mascheroni` |
| 随机抽样 | 标量 | 30 | `sr_sample_normal`、`sr_sample_beta` |
| 密度估计 | 标量 | 2 | `sr_kde_pdf`、`sr_knn_pdf` |
| 信号生成 | 标量 | 6 | `sr_gen_sinusoidal`、`sr_gen_square` |
| 假设检验 | 标量 | 9 | `sr_ttest_onesample`、`sr_ks_twosample`、`sr_chisquare` |
| 多元分布 | 标量 | 5 | `sr_multivariate_normal_pdf`、`sr_dirichlet_pdf` |

计算全部交给 statrs，本扩展不重新实现任何统计公式。描述统计量是**聚合函数**，分布 / 抽样 /
特殊函数 / 常量是**标量函数**（逐行求值）。

就地试试——这个块会在你的浏览器里真跑（站点自动预加载扩展）：

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS std_dev
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 10.0), (2, 20.0)) t(g, x)
GROUP BY g ORDER BY g;
```

```sql {"type":"duckfn","show":"table"}
SELECT x, sr_normal_pdf(x, 0.0, 1.0) AS pdf, sr_normal_cdf(x, 0.0, 1.0) AS cdf
FROM (VALUES (-1.96::DOUBLE), (0.0), (1.96)) t(x);
```

## NULL 语义

整条规则一句话说完：**statrs 算不出的就是 SQL NULL，NULL 输入也永远不会悄悄变成一个数。**
具体拆开：NULL 行不进聚合（SQL 聚合惯例，与 DuckDB 自带的 mean/stddev 一致）；statrs 对空组、
单值的样本方差、越界的 `tau`、几何/调和平均里的负数返回的 NAN，统一折成 NULL。参数**存在但
非法**（`std_dev <= 0`、概率不在 `[0, 1]` 内）是调用写错了，报查询错误，而不是被静默折成空。

## 接下来去哪

- [安装与加载](./getting-started/quick-start.md) —— 把扩展跑起来。
- [函数参考](./guide/functions/overview.md) —— 按类别浏览全部 252 个函数。
- [开发指南](./getting-started/project-structure.md) —— 从源码构建、添加函数、测试与发版。
