---
title: 经验分布
sidebar_position: 4
description: 经验 CDF、生存函数与分位数——按 DOUBLE 列聚合。
---

# 经验分布（聚合函数）

本节的三个函数都是**聚合函数**：整列 DOUBLE 作为样本收集，然后在常量第二参数上求一个经验
统计量。

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
