---
title: Poisson 与超几何
sidebar_position: 2
description: Poisson 与超几何分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile。
---

# Poisson 与超几何

整数槽位是整数值 DOUBLE 字面量；非整数报查询错误。

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

## NULL 参数短路

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(NULL, 3.0)
-- NULL
```
