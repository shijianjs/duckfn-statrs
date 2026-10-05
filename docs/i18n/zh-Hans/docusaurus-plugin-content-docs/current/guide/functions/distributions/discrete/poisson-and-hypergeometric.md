---
title: Poisson 与超几何
sidebar_position: 2
description: Poisson 与超几何分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile，以及各阶矩与支撑。
---

# Poisson 与超几何

整数槽位是 UBIGINT/BIGINT 参数；非整数字面量找不到匹配的签名。

## Poisson（泊松）

参数：`lambda`（率，> 0）。

### sr_poisson_pmf(x, lambda)

**签名**：`sr_poisson_pmf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(2, 3.0)::DECIMAL(12,8)
-- 0.22404181
```

### sr_poisson_ln_pmf(x, lambda)

**签名**：`sr_poisson_ln_pmf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT exp(sr_poisson_ln_pmf(2, 3.0))::DECIMAL(12,8)
-- 0.22404181
```

### sr_poisson_cdf(x, lambda)

**签名**：`sr_poisson_cdf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_cdf(2, 3.0)
```

### sr_poisson_sf(x, lambda)

**签名**：`sr_poisson_sf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_sf(2, 3.0)
```

### sr_poisson_quantile(p, lambda)

**签名**：`sr_poisson_quantile(p DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_quantile(0.5, 3.0)
```

概率越界：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_poisson_quantile(-0.1, 3.0)
-- error: sr_poisson_quantile: the probability must be within [0, 1], got -0.1
```

### sr_poisson_entropy(lambda)

**签名**：`sr_poisson_entropy(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_entropy(3.0)
-- 1.9338825376210322
```

### sr_poisson_max(lambda)

**签名**：`sr_poisson_max(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_max(3.0)
-- 1.8446744073709552e+19
```

### sr_poisson_mean(lambda)

**签名**：`sr_poisson_mean(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_mean(3.0)
-- 3.0
```

### sr_poisson_median(lambda)

**签名**：`sr_poisson_median(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_median(3.0)
-- 3.0
```

### sr_poisson_min(lambda)

**签名**：`sr_poisson_min(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_min(3.0)
-- 0.0
```

### sr_poisson_mode(lambda)

**签名**：`sr_poisson_mode(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_mode(3.0)
-- 3.0
```

### sr_poisson_skewness(lambda)

**签名**：`sr_poisson_skewness(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_skewness(3.0)
-- 0.5773502691896258
```

### sr_poisson_std_dev(lambda)

**签名**：`sr_poisson_std_dev(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_std_dev(3.0)
-- 1.7320508075688772
```

### sr_poisson_variance(lambda)

**签名**：`sr_poisson_variance(lambda DOUBLE) -> DOUBLE`

Poisson（泊松）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_variance(3.0)
-- 3.0
```

## Hypergeometric（超几何）

参数：`population`（N，UBIGINT）、`successes`（K，UBIGINT）、
`draws`（n，UBIGINT）。从共 N 个、此中 K 个成功的总体里无放回抽 n 个。

### sr_hypergeometric_pmf(x, population, successes, draws)

**签名**：`sr_hypergeometric_pmf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_pmf(2, 10, 5, 4)::DECIMAL(12,8)
-- 0.47619048
```

### sr_hypergeometric_ln_pmf(x, population, successes, draws)

**签名**：`sr_hypergeometric_ln_pmf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_ln_pmf(2, 10, 5, 4)
```

### sr_hypergeometric_cdf(x, population, successes, draws)

**签名**：`sr_hypergeometric_cdf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_cdf(2, 10, 5, 4)
```

### sr_hypergeometric_sf(x, population, successes, draws)

**签名**：`sr_hypergeometric_sf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_sf(2, 10, 5, 4)
```

### sr_hypergeometric_quantile(p, population, successes, draws)

**签名**：`sr_hypergeometric_quantile(p DOUBLE, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_quantile(0.5, 10, 5, 4)
```

### sr_hypergeometric_entropy(population, successes, draws)

**签名**：`sr_hypergeometric_entropy(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_entropy(10, 5, 4)
-- NULL
```

### sr_hypergeometric_max(population, successes, draws)

**签名**：`sr_hypergeometric_max(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_max(10, 5, 4)
-- 4.0
```

### sr_hypergeometric_mean(population, successes, draws)

**签名**：`sr_hypergeometric_mean(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_mean(10, 5, 4)
-- 2.0
```

### sr_hypergeometric_min(population, successes, draws)

**签名**：`sr_hypergeometric_min(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_min(10, 5, 4)
-- 0.0
```

### sr_hypergeometric_mode(population, successes, draws)

**签名**：`sr_hypergeometric_mode(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_mode(10, 5, 4)
-- 2.0
```

### sr_hypergeometric_skewness(population, successes, draws)

**签名**：`sr_hypergeometric_skewness(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_skewness(10, 5, 4)
-- 0.0
```

### sr_hypergeometric_std_dev(population, successes, draws)

**签名**：`sr_hypergeometric_std_dev(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_std_dev(10, 5, 4)
-- 0.816496580927726
```

### sr_hypergeometric_variance(population, successes, draws)

**签名**：`sr_hypergeometric_variance(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric（超几何）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_variance(10, 5, 4)
-- 0.6666666666666666
```

## NULL 参数短路

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(NULL, 3.0)
-- NULL
```
