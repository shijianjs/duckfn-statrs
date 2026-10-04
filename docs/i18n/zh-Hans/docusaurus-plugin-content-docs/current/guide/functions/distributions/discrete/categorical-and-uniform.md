---
title: 类别与离散均匀
sidebar_position: 3
description: 类别与离散均匀分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile。
---

# 类别与离散均匀

整数槽位是整数值 DOUBLE 字面量；非整数报查询错误。

## Categorical（类别）

参数：`probabilities` —— **未归一化**的概率 `LIST(DOUBLE)`；statrs 内部会自动归一。

### sr_categorical_pmf(x, probabilities)

**签名**：`sr_categorical_pmf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
-- [1, 2, 1] 归一化为 [0.25, 0.5, 0.25]；pmf(1) = 0.5
SELECT sr_categorical_pmf(1.0, [1.0, 2.0, 1.0])
-- 0.5
```

### sr_categorical_ln_pmf(x, probabilities)

**签名**：`sr_categorical_ln_pmf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_ln_pmf(1.0, [1.0, 2.0, 1.0])
```

### sr_categorical_cdf(x, probabilities)

**签名**：`sr_categorical_cdf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_cdf(1.0, [1.0, 2.0, 1.0])
-- 0.75
```

### sr_categorical_sf(x, probabilities)

**签名**：`sr_categorical_sf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_sf(1.0, [1.0, 2.0, 1.0])
```

### sr_categorical_quantile(p, probabilities)

**签名**：`sr_categorical_quantile(p DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_quantile(0.5, [1.0, 2.0, 1.0])
-- 1.0
```

pmf 中 `x` 不是整数：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_categorical_pmf(1.5, [1.0, 2.0, 1.0])
-- error: expected a non-negative whole number, got 1.5
```

## Discrete uniform（离散均匀）

参数：`min`、`max`——整数值 DOUBLE。支撑：`[min, max]` 内的整数。

### sr_discrete_uniform_pmf(x, min, max)

**签名**：`sr_discrete_uniform_pmf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_pmf(2.0, 1.0, 6.0)::DECIMAL(12,8)
-- 0.16666667
```

### sr_discrete_uniform_ln_pmf(x, min, max)

**签名**：`sr_discrete_uniform_ln_pmf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_ln_pmf(2.0, 1.0, 6.0)
```

### sr_discrete_uniform_cdf(x, min, max)

**签名**：`sr_discrete_uniform_cdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_cdf(3.0, 1.0, 6.0)
```

### sr_discrete_uniform_sf(x, min, max)

**签名**：`sr_discrete_uniform_sf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_sf(3.0, 1.0, 6.0)
```

### sr_discrete_uniform_quantile(p, min, max)

**签名**：`sr_discrete_uniform_quantile(p DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_quantile(0.5, 1.0, 6.0)
-- 3.0
```
