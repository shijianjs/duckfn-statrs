---
title: 多元分布
sidebar_position: 12
description: 多元正态、Dirichlet、多项式与多元 t 密度——向量走 LIST(DOUBLE)，矩阵走行主序摊平 LIST。
---

# 多元分布

向量一律是 `LIST(DOUBLE)`；矩阵（协方差、尺度）是**行主序**摊平的 `LIST(DOUBLE)`。
2x2 单位矩阵写成 `[1.0, 0.0, 0.0, 1.0]`。

## sr_multivariate_normal_pdf(x, mean, covariance)

**签名**：`sr_multivariate_normal_pdf(x LIST(DOUBLE), mean LIST(DOUBLE), covariance LIST(DOUBLE)) -> DOUBLE`

x 在给定均值向量与协方差矩阵下的多元正态密度。

`covariance` 的长度必须是 `len(mean)²`。不匹配会报查询错误（statrs 内部 `nalgebra::from_vec`
的 panic 已在 SQL 边界拦下）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])::DECIMAL(12,8)
-- 0.15915494
```

数值 `1/(2π)` 就是标准二元正态在原点的密度。

## sr_multivariate_students_t_pdf(x, location, scale, freedom)

**签名**：`sr_multivariate_students_t_pdf(x LIST(DOUBLE), location LIST(DOUBLE), scale LIST(DOUBLE), freedom DOUBLE) -> DOUBLE`

多元 Student's t 密度。当 `location = 0`、`scale = I`、`x = 0` 时与多元正态同形。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)::DECIMAL(12,8)
-- 0.15915494
```

## sr_dirichlet_pdf(x, alpha)

**签名**：`sr_dirichlet_pdf(x LIST(DOUBLE), alpha LIST(DOUBLE)) -> DOUBLE`

x 在单纯形上、Dirichlet(alpha) 分布下的密度。`x` 与 `alpha` 长度必须一致。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_pdf([0.5, 0.5], [1.0, 1.0])::DECIMAL(12,8)
-- 1.00000000
```

`alpha = (1, 1)` 时 Dirichlet 就是单纯形上的均匀分布，密度处处为 1。

## sr_dirichlet_entropy(alpha)

**签名**：`sr_dirichlet_entropy(alpha LIST(DOUBLE)) -> DOUBLE`

`Dir(alpha)` 的微分熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_entropy([1.0, 1.0])::DECIMAL(12,8)
-- 0.00000000
```

## sr_multinomial_pmf(probs, trials, counts)

**签名**：`sr_multinomial_pmf(probs LIST(DOUBLE), trials DOUBLE, counts LIST(BIGINT)) -> DOUBLE`

多项式分布下计数向量的概率质量。

- `probs`：各类别概率（statrs 内部会归一化）
- `trials`：总试验次数，整数值 DOUBLE
- `counts`：各类别的观测数，`LIST(BIGINT)`

`counts` 之和必须等于 `trials`。`trials` 不是整数会报查询错误。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_pmf([0.5, 0.5], 4.0, [2, 2])::DECIMAL(12,8)
-- 0.37500000
```

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multinomial_pmf([0.5, 0.5], 4.5, [2, 2])
-- error: expected a non-negative whole number, got 4.5
```

## 形状校验

矩阵参数的长度会与向量长度对齐检查：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0])
-- error: expected 4 entries for a 2x2 covariance matrix
```
