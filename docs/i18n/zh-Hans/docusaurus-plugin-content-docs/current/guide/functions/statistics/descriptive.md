---
title: 描述统计量
sidebar_position: 1
description: 27 个聚合函数——均值、中位数、方差、协方差等。一列 DOUBLE 进，每组一个值出。
---

# 描述统计量（聚合函数）

本节所有函数都是**聚合函数**：接受一个（或两个）DOUBLE 列，每个分组返回一个值。
NULL 行自动跳过。

## 集中趋势

### sr_mean(x)

**签名**：`sr_mean(x DOUBLE) -> DOUBLE`

算术平均。所有行都是 NULL 时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_mean(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 0.3333333333333333
```

### sr_geometric_mean(x)

**签名**：`sr_geometric_mean(x DOUBLE) -> DOUBLE`

几何平均。任一值为负时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)
-- 1.8171205928321397
```

### sr_harmonic_mean(x)

**签名**：`sr_harmonic_mean(x DOUBLE) -> DOUBLE`

调和平均。任一值为负时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)
-- 1.6363636363636365
```

### sr_quadratic_mean(x)

**签名**：`sr_quadratic_mean(x DOUBLE) -> DOUBLE`

均方根（二次均值）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_quadratic_mean(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 2.0816659994661326
```

## 顺序统计量

### sr_median(x)

**签名**：`sr_median(x DOUBLE) -> DOUBLE`

中位数。偶数个值时取中间两个的平均。

```sql {"type":"duckfn","show":"value"}
SELECT sr_median(x) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)
-- 2.5
```

### sr_quantile(x, tau)

**签名**：`sr_quantile(x DOUBLE, tau DOUBLE) -> DOUBLE`

tau 分位数。`tau` 必须在 [0, 1]，否则返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_quantile(x, 0.5) FROM (VALUES (-1.0), (5.0), (0.0), (-3.0), (10.0), (-0.5), (4.0), (0.2), (1.0), (6.0)) t(x)
-- 0.6
```

### sr_order_statistic(x, k)

**签名**：`sr_order_statistic(x DOUBLE, k DOUBLE) -> DOUBLE`

第 k 小的值（1 起）。k 超出数据范围时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_order_statistic(x, 2.0) FROM (VALUES (3.0), (1.0), (2.0)) t(x)
-- 2.0
```

### sr_percentile(x, p)

**签名**：`sr_percentile(x DOUBLE, p DOUBLE) -> DOUBLE`

第 p 百分位。`p` 必须是 0–100 的整数值 DOUBLE。

```sql {"type":"duckfn","show":"value"}
SELECT sr_percentile(x, 50.0) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)
-- 2.5
```

### sr_lower_quartile(x)

**签名**：`sr_lower_quartile(x DOUBLE) -> DOUBLE`

下四分位 Q1。

```sql {"type":"duckfn","show":"value"}
SELECT sr_lower_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)
-- 1.4166666666666665
```

### sr_upper_quartile(x)

**签名**：`sr_upper_quartile(x DOUBLE) -> DOUBLE`

上四分位 Q3。

```sql {"type":"duckfn","show":"value"}
SELECT sr_upper_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)
-- 3.5833333333333335
```

### sr_interquartile_range(x)

**签名**：`sr_interquartile_range(x DOUBLE) -> DOUBLE`

四分位距 IQR = 上四分位 - 下四分位。

```sql {"type":"duckfn","show":"value"}
SELECT sr_interquartile_range(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)
-- 2.166666666666667
```

### sr_ranks(x, method)

**签名**：`sr_ranks(x DOUBLE, method DOUBLE) -> LIST(DOUBLE)`

每行的秩。method：1=average 2=min 3=max 4=first。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ranks(x, 1.0) FROM (VALUES (1.0), (3.0), (2.0), (2.0)) t(x)
-- [1.0, 4.0, 2.5, 2.5]
```

## 离散程度

### sr_variance(x)

**签名**：`sr_variance(x DOUBLE) -> DOUBLE`

样本方差（Bessel 校正）。少于两行非 NULL 时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 6.333333333333333
```

### sr_std_dev(x)

**签名**：`sr_std_dev(x DOUBLE) -> DOUBLE`

样本标准差。少于两行非 NULL 时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 2.5166114784690383
```

### sr_population_variance(x)

**签名**：`sr_population_variance(x DOUBLE) -> DOUBLE`

总体方差（除以 N，而非 N-1）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 4.222222222222222
```

### sr_population_std_dev(x)

**签名**：`sr_population_std_dev(x DOUBLE) -> DOUBLE`

总体标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 2.0548046670007003
```

### sr_skewness(x)

**签名**：`sr_skewness(x DOUBLE) -> DOUBLE`

样本偏度（statrs 的 `OnlineMoments<3>`：三阶中心矩除以总体二阶矩的 3/2 次幂）。
非 NULL 行少于两个时为 NULL；常数列（零方差）按 statrs 的约定给 0。

```sql {"type":"duckfn","show":"value"}
SELECT sr_skewness(x)
FROM (VALUES (2.0), (4.0), (4.0), (4.0), (5.0), (5.0), (7.0), (9.0)) t(x)
-- 0.65625
```

## 极值

### sr_min(x)

**签名**：`sr_min(x DOUBLE) -> DOUBLE`

最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- -2.0
```

### sr_max(x)

**签名**：`sr_max(x DOUBLE) -> DOUBLE`

最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_max(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 3.0
```

### sr_abs_min(x)

**签名**：`sr_abs_min(x DOUBLE) -> DOUBLE`

绝对值最小。

```sql {"type":"duckfn","show":"value"}
SELECT sr_abs_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 0.0
```

### sr_abs_max(x)

**签名**：`sr_abs_max(x DOUBLE) -> DOUBLE`

绝对值最大。

```sql {"type":"duckfn","show":"value"}
SELECT sr_abs_max(x) FROM (VALUES (0.0), (3.0), (-8.0)) t(x)
-- 8.0
```

## 协方差（两列）

### sr_covariance(x, y)

**签名**：`sr_covariance(x DOUBLE, y DOUBLE) -> DOUBLE`

两列的样本协方差（Bessel 校正）。任一列为 NULL 的行整对跳过。少于两对完整非 NULL 时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)
-- -11.5
```

### sr_population_covariance(x, y)

**签名**：`sr_population_covariance(x DOUBLE, y DOUBLE) -> DOUBLE`

总体协方差（除以 N）。没有完整非 NULL 行时返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_population_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)
-- -7.666666666666667
```

## 配合 GROUP BY

```sql {"type":"duckfn","show":"table"}
SELECT g,
       sr_mean(x) AS mean,
       sr_median(x) AS median,
       sr_std_dev(x) AS std_dev,
       sr_min(x) AS min,
       sr_max(x) AS max
FROM (VALUES (1, 2.5), (1, 3.1), (1, 1.8), (2, 7.2), (2, 8.1), (2, 6.9)) t(g, x)
GROUP BY g ORDER BY g
```

## NULL 处理

```sql {"type":"duckfn","show":"table"}
-- NULL 行不进收集：[1, NULL, 3] 算的是 [1, 3]
SELECT sr_mean(x) AS mean, sr_variance(x) AS var, sr_std_dev(x) AS sd
FROM (VALUES (1.0), (NULL), (3.0)) t(x)
```

```sql {"type":"duckfn","show":"value"}
-- 单值没有样本方差：返回 NULL
SELECT sr_variance(x) FROM (VALUES (42.0)) t(x)
-- NULL
```
