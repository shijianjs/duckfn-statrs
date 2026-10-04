---
title: 均匀、三角与 Dirac
sidebar_position: 7
description: 连续均匀、三角与 Dirac（退化）分布。
---

# 均匀、三角与 Dirac

## 连续均匀

参数：`min`、`max`（min < max）。支撑：`[min, max]`。

### sr_uniform_pdf(x, min, max)

**签名**：`sr_uniform_pdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_pdf(0.5, 0.0, 1.0)
-- 1.0
```

### sr_uniform_ln_pdf(x, min, max)

**签名**：`sr_uniform_ln_pdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_ln_pdf(0.5, 0.0, 1.0)
```

### sr_uniform_cdf(x, min, max)

**签名**：`sr_uniform_cdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_cdf(0.5, 0.0, 1.0)
-- 0.5
```

### sr_uniform_sf(x, min, max)

**签名**：`sr_uniform_sf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_sf(0.5, 0.0, 1.0)
```

### sr_uniform_quantile(p, min, max)

**签名**：`sr_uniform_quantile(p DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_quantile(0.25, 0.0, 1.0)
-- 0.25
```

## Triangular（三角）

参数：`min`、`max`、`mode`。支撑：`[min, max]`。

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

## Dirac 退化

参数：`location`（v）。退化到一个点的分布。只暴露 `cdf`、`sf` 与 `quantile`（没有通常意义上的密度）。

### sr_dirac_cdf(x, location)

**签名**：`sr_dirac_cdf(x DOUBLE, location DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_cdf(1.0, 1.0)
-- 1.0
```

### sr_dirac_sf(x, location)

**签名**：`sr_dirac_sf(x DOUBLE, location DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_sf(0.5, 1.0)
```

### sr_dirac_quantile(p, location)

**签名**：`sr_dirac_quantile(p DOUBLE, location DOUBLE) -> DOUBLE`

无论 `p` 取何值，结果都是 `location`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_quantile(0.5, 1.0)
-- 1.0
```
