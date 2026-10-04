---
title: "离散分布 (B)"
sidebar_position: 11
description: Poisson、超几何、类别与离散均匀分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile。
---

# 离散分布 (B)

与 [离散分布 (A)](./dist-discrete-a.md) 同一 5 函数模式。整数槽位是整数值 DOUBLE 字面量；非整数报查询错误。

## Poisson（泊松）

参数：`lambda`（率，> 0）。

### sr_poisson_pmf(x, lambda)

**签名**：`sr_poisson_pmf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(2.0, 3.0)::DECIMAL(12,8)
-- 0.22404181
```

### sr_poisson_ln_pmf(x, lambda)

**签名**：`sr_poisson_ln_pmf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT exp(sr_poisson_ln_pmf(2.0, 3.0))::DECIMAL(12,8)
-- 0.22404181
```

### sr_poisson_cdf(x, lambda)

**签名**：`sr_poisson_cdf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_cdf(2.0, 3.0)
```

### sr_poisson_sf(x, lambda)

**签名**：`sr_poisson_sf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_sf(2.0, 3.0)
```

### sr_poisson_quantile(p, lambda)

**签名**：`sr_poisson_quantile(p DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_quantile(0.5, 3.0)
```

概率越界：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_poisson_quantile(-0.1, 3.0)
-- error: the probability must be within [0, 1], got -0.1
```

## Hypergeometric（超几何）

参数：`population`（N，整数值 DOUBLE）、`successes`（K，整数值 DOUBLE）、
`draws`（n，整数值 DOUBLE）。从共 N 个、此中 K 个成功的总体里无放回抽 n 个。

### sr_hypergeometric_pmf(x, population, successes, draws)

**签名**：`sr_hypergeometric_pmf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_pmf(2.0, 10.0, 5.0, 4.0)::DECIMAL(12,8)
-- 0.47619048
```

### sr_hypergeometric_ln_pmf(x, population, successes, draws)

**签名**：`sr_hypergeometric_ln_pmf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_ln_pmf(2.0, 10.0, 5.0, 4.0)
```

### sr_hypergeometric_cdf(x, population, successes, draws)

**签名**：`sr_hypergeometric_cdf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_cdf(2.0, 10.0, 5.0, 4.0)
```

### sr_hypergeometric_sf(x, population, successes, draws)

**签名**：`sr_hypergeometric_sf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_sf(2.0, 10.0, 5.0, 4.0)
```

### sr_hypergeometric_quantile(p, population, successes, draws)

**签名**：`sr_hypergeometric_quantile(p DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_quantile(0.5, 10.0, 5.0, 4.0)
```

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

Non-integer `x` in the pmf:

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
SELECT sr_discrete_uniform_cdf(3.5, 1.0, 6.0)
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

## NULL argument short-circuit

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(NULL, 3.0)
-- NULL
```
