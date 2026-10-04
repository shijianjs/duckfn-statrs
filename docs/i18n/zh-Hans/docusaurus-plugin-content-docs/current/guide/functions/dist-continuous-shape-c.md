---
title: "连续：形状 (C)"
sidebar_position: 8
description: Weibull、Pareto 与 Triangular 分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile。
---

# 连续分布：形状 (C)

## Weibull

参数：`shape（> 0）、`scale`（> 0）。

### sr_weibull_pdf(x, shape, scale)

**签名**：`sr_weibull_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_pdf(1.0, 1.0, 1.0)
-- 1.0
```

### sr_weibull_ln_pdf(x, shape, scale)

**签名**：`sr_weibull_ln_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_ln_pdf(1.0, 1.0, 1.0)
```

### sr_weibull_cdf(x, shape, scale)

**签名**：`sr_weibull_cdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_cdf(1.0, 1.0, 1.0)
-- 0.6321205588285577
```

### sr_weibull_sf(x, shape, scale)

**签名**：`sr_weibull_sf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_sf(1.0, 1.0, 1.0)
```

### sr_weibull_quantile(p, shape, scale)

**签名**：`sr_weibull_quantile(p DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_quantile(0.5, 1.0, 1.0)
```

## Pareto（I 型）

参数：`scale` （x_m > 0）, `shape` （alpha > 0）。 支撑：`[x_m, infinity)]`。

### sr_pareto_pdf(x, scale, shape)

**签名**：`sr_pareto_pdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_pdf(1.0, 1.0, 2.0)
-- 2.0
```

### sr_pareto_ln_pdf(x, scale, shape)

**签名**：`sr_pareto_ln_pdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_ln_pdf(1.0, 1.0, 2.0)
```

### sr_pareto_cdf(x, scale, shape)

**签名**：`sr_pareto_cdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_cdf(2.0, 1.0, 2.0)
```

### sr_pareto_sf(x, scale, shape)

**签名**：`sr_pareto_sf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_sf(2.0, 1.0, 2.0)
```

### sr_pareto_quantile(p, scale, shape)

**签名**：`sr_pareto_quantile(p DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_quantile(0.5, 1.0, 2.0)
```

## Triangular

参数：`min`, `max`, `mode`. 支撑：`[min, max]`.

### sr_triangular_pdf(x, min, max, mode)

**签名**：`sr_triangular_pdf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_pdf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_ln_pdf(x, min, max, mode)

**签名**：`sr_triangular_ln_pdf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_ln_pdf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_cdf(x, min, max, mode)

**签名**：`sr_triangular_cdf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_cdf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_sf(x, min, max, mode)

**签名**：`sr_triangular_sf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_sf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_quantile(p, min, max, mode)

**签名**：`sr_triangular_quantile(p DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_quantile(0.5, 0.0, 2.0, 1.0)
```
