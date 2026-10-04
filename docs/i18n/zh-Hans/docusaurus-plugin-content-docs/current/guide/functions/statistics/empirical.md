---
title: 经验分布
sidebar_position: 2
description: 经验 CDF、生存函数、分位数与各阶矩——按 DOUBLE 列聚合。
---

# 经验分布（聚合函数）

本节所有函数都是**聚合函数**：整列 DOUBLE 作为样本收集，再在其上求一个经验统计量。`cdf`、
`sf` 与 `quantile` 需要常量第二参数；各阶矩（`mean`、`variance`、`std_dev`、`min`、`max` 等）
则直接汇总整列。

## sr_empirical_cdf(v, x)

**签名**：`sr_empirical_cdf(v DOUBLE, x DOUBLE) -> DOUBLE`

所收集列 `v` 的经验累积分布在常量 `x` 处的取值。等价于「`v <= x` 的行占比」。

空组返回 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_cdf(v, 2.0)
FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)
-- 0.5
```

## sr_empirical_sf(v, x)

**签名**：`sr_empirical_sf(v DOUBLE, x DOUBLE) -> DOUBLE`

在常量 `x` 处的经验生存函数。等价于 `1 - cdf(x)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_sf(v, 2.0)
FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)
-- 0.5
```

## sr_empirical_quantile(v, p)

**签名**：`sr_empirical_quantile(v DOUBLE, p DOUBLE) -> DOUBLE`

在常量概率 `p`（范围 [0, 1]）处的经验分位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_quantile(v, 0.5)::DECIMAL(12,6)
FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)
-- 1.999969
```

### sr_empirical_entropy(values)

**签名**：`sr_empirical_entropy(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验熵；statrs 未实现经验分布的熵，恒为 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_entropy(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- NULL
```

### sr_empirical_max(values)

**签名**：`sr_empirical_max(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_max(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 3.0
```

### sr_empirical_mean(values)

**签名**：`sr_empirical_mean(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_mean(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 2.0
```

### sr_empirical_min(values)

**签名**：`sr_empirical_min(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_min(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 1.0
```

### sr_empirical_skewness(values)

**签名**：`sr_empirical_skewness(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验偏度；statrs 未实现经验分布的偏度，恒为 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_skewness(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- NULL
```

### sr_empirical_std_dev(values)

**签名**：`sr_empirical_std_dev(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_std_dev(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 1.0
```

### sr_empirical_variance(values)

**签名**：`sr_empirical_variance(values DOUBLE) -> DOUBLE`

DOUBLE 列的经验方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_variance(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 1.0
```

## NULL 行与空组

NULL 行不进收集器，与 SQL 聚合惯例一致。空组返回 NULL：

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_cdf(v, 2.0)
FROM (VALUES (1.0)) t(v) WHERE 1 = 0
-- NULL
```

## 配合 GROUP BY

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_empirical_cdf(v, 2.0) AS cdf_at_2
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 2.0), (2, 4.0), (2, 6.0)) t(g, v)
GROUP BY g ORDER BY g
```
