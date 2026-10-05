---
title: 类别与离散均匀
sidebar_position: 3
description: 类别与离散均匀分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile，以及各阶矩与支撑。
---

# 类别与离散均匀

整数槽位是 UBIGINT/BIGINT 参数；非整数字面量找不到匹配的签名。

## Categorical（类别）

参数：`probabilities` —— **未归一化**的概率 `LIST(DOUBLE)`；statrs 内部会自动归一。

### sr_categorical_pmf(x, probabilities)

**签名**：`sr_categorical_pmf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
-- [1, 2, 1] 归一化为 [0.25, 0.5, 0.25]；pmf(1) = 0.5
SELECT sr_categorical_pmf(1, [1.0, 2.0, 1.0])
-- 0.5
```

### sr_categorical_ln_pmf(x, probabilities)

**签名**：`sr_categorical_ln_pmf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_ln_pmf(1, [1.0, 2.0, 1.0])
```

### sr_categorical_cdf(x, probabilities)

**签名**：`sr_categorical_cdf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_cdf(1, [1.0, 2.0, 1.0])
-- 0.75
```

### sr_categorical_sf(x, probabilities)

**签名**：`sr_categorical_sf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_sf(1, [1.0, 2.0, 1.0])
```

### sr_categorical_quantile(p, probabilities)

**签名**：`sr_categorical_quantile(p DOUBLE, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_quantile(0.5, [1.0, 2.0, 1.0])
-- 1.0
```

pmf 中 `x` 不是整数：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_categorical_pmf(1.5, [1.0, 2.0, 1.0])
-- error: expected a non-negative whole number, got 1.5
```

### sr_categorical_entropy(probs)

**签名**：`sr_categorical_entropy(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_entropy([1.0, 2.0, 1.0])
-- 1.0397207708399179
```

### sr_categorical_max(probs)

**签名**：`sr_categorical_max(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_max([1.0, 2.0, 1.0])
-- 2.0
```

### sr_categorical_mean(probs)

**签名**：`sr_categorical_mean(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_mean([1.0, 2.0, 1.0])
-- 1.0
```

### sr_categorical_median(probs)

**签名**：`sr_categorical_median(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_median([1.0, 2.0, 1.0])
-- 1.0
```

### sr_categorical_min(probs)

**签名**：`sr_categorical_min(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_min([1.0, 2.0, 1.0])
-- 0.0
```

### sr_categorical_skewness(probs)

**签名**：`sr_categorical_skewness(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_skewness([1.0, 2.0, 1.0])
-- NULL
```

### sr_categorical_std_dev(probs)

**签名**：`sr_categorical_std_dev(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_std_dev([1.0, 2.0, 1.0])
-- 0.7071067811865476
```

### sr_categorical_variance(probs)

**签名**：`sr_categorical_variance(probs DOUBLE[]) -> DOUBLE`

Categorical（类别）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_variance([1.0, 2.0, 1.0])
-- 0.5
```

## Discrete uniform（离散均匀）

参数：`min`、`max`——BIGINT。支撑：`[min, max]` 内的整数。

### sr_discrete_uniform_pmf(x, min, max)

**签名**：`sr_discrete_uniform_pmf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_pmf(2, 1, 6)::DECIMAL(12,8)
-- 0.16666667
```

### sr_discrete_uniform_ln_pmf(x, min, max)

**签名**：`sr_discrete_uniform_ln_pmf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_ln_pmf(2, 1, 6)
```

### sr_discrete_uniform_cdf(x, min, max)

**签名**：`sr_discrete_uniform_cdf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_cdf(3, 1, 6)
```

### sr_discrete_uniform_sf(x, min, max)

**签名**：`sr_discrete_uniform_sf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_sf(3, 1, 6)
```

### sr_discrete_uniform_quantile(p, min, max)

**签名**：`sr_discrete_uniform_quantile(p DOUBLE, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_quantile(0.5, 1, 6)
-- 3.0
```

### sr_discrete_uniform_entropy(min, max)

**签名**：`sr_discrete_uniform_entropy(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_entropy(1, 6)
-- 1.791759469228055
```

### sr_discrete_uniform_max(min, max)

**签名**：`sr_discrete_uniform_max(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_max(1, 6)
-- 6.0
```

### sr_discrete_uniform_mean(min, max)

**签名**：`sr_discrete_uniform_mean(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_mean(1, 6)
-- 3.5
```

### sr_discrete_uniform_median(min, max)

**签名**：`sr_discrete_uniform_median(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_median(1, 6)
-- 3.5
```

### sr_discrete_uniform_min(min, max)

**签名**：`sr_discrete_uniform_min(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_min(1, 6)
-- 1.0
```

### sr_discrete_uniform_mode(min, max)

**签名**：`sr_discrete_uniform_mode(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_mode(1, 6)
-- 3.0
```

### sr_discrete_uniform_skewness(min, max)

**签名**：`sr_discrete_uniform_skewness(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_skewness(1, 6)
-- 0.0
```

### sr_discrete_uniform_std_dev(min, max)

**签名**：`sr_discrete_uniform_std_dev(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_std_dev(1, 6)
-- 1.707825127659933
```

### sr_discrete_uniform_variance(min, max)

**签名**：`sr_discrete_uniform_variance(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform（离散均匀）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_variance(1, 6)
-- 2.9166666666666665
```
