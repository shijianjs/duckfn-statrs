---
title: 多元分布
sidebar_position: 3
description: 多元正态、Dirichlet、多项式与多元 t 的密度、对数密度、各阶矩与定义域——向量走 LIST(DOUBLE)，矩阵走行主序摊平 LIST。
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

### sr_multivariate_normal_ln_pdf(x, mean, cov)

**签名**：`sr_multivariate_normal_ln_pdf(x LIST(DOUBLE), mean LIST(DOUBLE), covariance LIST(DOUBLE)) -> DOUBLE`

x 的多元正态对数密度——等价于 `ln(sr_multivariate_normal_pdf(...))`，但不经过中间的指数运算，
密度下溢成 0 的深尾区域里它仍然是有限值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_ln_pdf([1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- -2.8378770664093453
```

### sr_multivariate_normal_min(mean, cov)

**签名**：`sr_multivariate_normal_min(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

支撑集下界：与均值向量同维的全 `-inf` 向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_min([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [-inf, -inf]
```

### sr_multivariate_normal_max(mean, cov)

**签名**：`sr_multivariate_normal_max(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

支撑集上界：与均值向量同维的全 `+inf` 向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_max([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [inf, inf]
```

### sr_multivariate_normal_entropy(mean, cov)

**签名**：`sr_multivariate_normal_entropy(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE`

多元正态分布的微分熵（由均值向量与行主序展平的协方差矩阵给出）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_entropy([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- 2.8378770664093453
```

### sr_multivariate_normal_mean(mean, cov)

**签名**：`sr_multivariate_normal_mean(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

多元正态分布的均值向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_mean([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [0.0, 0.0]
```

### sr_multivariate_normal_mode(mean, cov)

**签名**：`sr_multivariate_normal_mode(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

多元正态分布的众数向量（等于均值向量）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_mode([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [0.0, 0.0]
```

### sr_multivariate_normal_variance(mean, cov)

**签名**：`sr_multivariate_normal_variance(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

多元正态分布的协方差矩阵（行主序展平为 LIST）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_variance([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [1.0, 0.0, 0.0, 1.0]
```

## sr_multivariate_students_t_pdf(x, location, scale, freedom)

**签名**：`sr_multivariate_students_t_pdf(x LIST(DOUBLE), location LIST(DOUBLE), scale LIST(DOUBLE), freedom DOUBLE) -> DOUBLE`

多元 Student's t 密度。当 `location = 0`、`scale = I`、`x = 0` 时与多元正态同形。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)::DECIMAL(12,8)
-- 0.15915494
```

### sr_multivariate_students_t_ln_pdf(x, location, scale, freedom)

**签名**：`sr_multivariate_students_t_ln_pdf(x LIST(DOUBLE), location LIST(DOUBLE), scale LIST(DOUBLE), freedom DOUBLE) -> DOUBLE`

多元 Student's t 的对数密度。`freedom` 增大时取值收敛到多元正态的 `ln_pdf`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_ln_pdf([1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 4.0)
-- -3.0542723907338383
```

### sr_multivariate_students_t_mean(location, scale, freedom)

**签名**：`sr_multivariate_students_t_mean(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

多元 Student's t 分布的均值向量（等于 `location`）。仅当 `freedom > 1` 有定义，否则结果为 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_mean([-1.0, 1.0, 3.0], [1.0, 0.0, 0.5, 0.0, 2.0, 0.0, 0.5, 0.0, 3.0], 2.0)
-- [-1.0, 1.0, 3.0]
```

### sr_multivariate_students_t_variance(location, scale, freedom)

**签名**：`sr_multivariate_students_t_variance(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

协方差矩阵 `scale · ν / (ν − 2)`，行主序展平为 LIST。仅当 `freedom > 2` 有定义，否则结果为 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_variance([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [3.0, 0.0, 0.0, 3.0]
```

### sr_multivariate_students_t_min(location, scale, freedom)

**签名**：`sr_multivariate_students_t_min(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

支撑集下界：与 `location` 同维的全 `-inf` 向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_min([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [-inf, -inf]
```

### sr_multivariate_students_t_max(location, scale, freedom)

**签名**：`sr_multivariate_students_t_max(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

支撑集上界：与 `location` 同维的全 `+inf` 向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_max([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [inf, inf]
```

### sr_multivariate_students_t_mode(location, scale, freedom)

**签名**：`sr_multivariate_students_t_mode(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

多元 Student's t 分布的众数向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_mode([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [0.0, 0.0]
```

## sr_dirichlet_pdf(x, alpha)

**签名**：`sr_dirichlet_pdf(x LIST(DOUBLE), alpha LIST(DOUBLE)) -> DOUBLE`

x 在单纯形上、Dirichlet(alpha) 分布下的密度。`x` 与 `alpha` 长度必须一致。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_pdf([0.5, 0.5], [1.0, 1.0])::DECIMAL(12,8)
-- 1.00000000
```

`alpha = (1, 1)` 时 Dirichlet 就是单纯形上的均匀分布，密度处处为 1。

### sr_dirichlet_ln_pdf(x, alpha)

**签名**：`sr_dirichlet_ln_pdf(x LIST(DOUBLE), alpha LIST(DOUBLE)) -> DOUBLE`

x 在单纯形上的对数密度。`x` 的每个分量都必须落在 `(0, 1)` 内且分量和为 1（容差 `1e-4`）——
违反是 statrs 的断言失败，会以查询错误的形式浮上来。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_ln_pdf([0.1, 0.2, 0.3, 0.4], [0.1, 0.3, 0.5, 0.8])
-- -0.18456529434757482
```

### sr_dirichlet_mean(alpha)

**签名**：`sr_dirichlet_mean(alpha LIST(DOUBLE)) -> DOUBLE[]`

Dirichlet 分布的均值向量：`alpha_i / alpha_0`（`alpha_0` 为浓度参数之和）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_mean([1.0, 2.0, 3.0, 4.0])
-- [0.1, 0.2, 0.3, 0.4]
```

### sr_dirichlet_variance(alpha)

**签名**：`sr_dirichlet_variance(alpha LIST(DOUBLE)) -> DOUBLE[]`

Dirichlet 分布的协方差矩阵，行主序展平为长度 `len(alpha)²` 的 LIST。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_variance([1.0, 2.0])
-- [0.05555555555555556, -0.05555555555555556, -0.05555555555555556, 0.05555555555555556]
```

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

### sr_multinomial_ln_pmf(probs, trials, counts)

**签名**：`sr_multinomial_ln_pmf(probs LIST(DOUBLE), trials DOUBLE, counts LIST(BIGINT)) -> DOUBLE`

计数向量的对数概率质量。与 `pmf` 不同，`counts` 之和不为 `trials` 不是错误，而是合法的
`-inf`（概率 0 的对数）。`trials` 很大时对数形式保持有限，概率质量本身会下溢成 0。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_ln_pmf([0.5, 0.5], 2000.0, [1000, 1000])
-- -4.026367582410558
```

### sr_multinomial_mean(probs, trials)

**签名**：`sr_multinomial_mean(probs LIST(DOUBLE), trials DOUBLE) -> DOUBLE[]`

多项式分布的均值向量：每个类别 `n · p_i`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_mean([0.3, 0.7], 5.0)
-- [1.5, 3.5]
```

### sr_multinomial_variance(probs, trials)

**签名**：`sr_multinomial_variance(probs LIST(DOUBLE), trials DOUBLE) -> DOUBLE[]`

多项式分布的协方差矩阵，行主序展平为长度 `len(probs)²` 的 LIST：对角线 `n · p_i · (1 − p_i)`，
非对角线 `−n · p_i · p_j`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_variance([0.1, 0.3, 0.6], 10.0)
-- [0.9, -0.3, -0.6, -0.3, 2.1, -1.8, -0.6, -1.8, 2.4]
```

## 形状校验

矩阵参数的长度会与向量长度对齐检查：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0])
-- error: expected 4 entries for a 2x2 covariance matrix
```
