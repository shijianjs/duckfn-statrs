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

### sr_uniform_entropy(min, max)

**签名**：`sr_uniform_entropy(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_entropy(0.0, 1.0)
-- 0.0
```

### sr_uniform_max(min, max)

**签名**：`sr_uniform_max(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_max(0.0, 1.0)
-- 1.0
```

### sr_uniform_mean(min, max)

**签名**：`sr_uniform_mean(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_mean(0.0, 1.0)
-- 0.5
```

### sr_uniform_median(min, max)

**签名**：`sr_uniform_median(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_median(0.0, 1.0)
-- 0.5
```

### sr_uniform_min(min, max)

**签名**：`sr_uniform_min(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_min(0.0, 1.0)
-- 0.0
```

### sr_uniform_mode(min, max)

**签名**：`sr_uniform_mode(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_mode(0.0, 1.0)
-- 0.5
```

### sr_uniform_skewness(min, max)

**签名**：`sr_uniform_skewness(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_skewness(0.0, 1.0)
-- 0.0
```

### sr_uniform_std_dev(min, max)

**签名**：`sr_uniform_std_dev(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_std_dev(0.0, 1.0)
-- 0.28867513459481287
```

### sr_uniform_variance(min, max)

**签名**：`sr_uniform_variance(min DOUBLE, max DOUBLE) -> DOUBLE`

连续均匀的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_variance(0.0, 1.0)
-- 0.08333333333333333
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

### sr_triangular_entropy(min, max, mode)

**签名**：`sr_triangular_entropy(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_entropy(0.0, 2.0, 1.0)
-- 0.5
```

### sr_triangular_max(min, max, mode)

**签名**：`sr_triangular_max(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_max(0.0, 2.0, 1.0)
-- 2.0
```

### sr_triangular_mean(min, max, mode)

**签名**：`sr_triangular_mean(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_mean(0.0, 2.0, 1.0)
-- 1.0
```

### sr_triangular_median(min, max, mode)

**签名**：`sr_triangular_median(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_median(0.0, 2.0, 1.0)
-- 1.0
```

### sr_triangular_min(min, max, mode)

**签名**：`sr_triangular_min(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_min(0.0, 2.0, 1.0)
-- 0.0
```

### sr_triangular_mode(min, max, mode)

**签名**：`sr_triangular_mode(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_mode(0.0, 2.0, 1.0)
-- 1.0
```

### sr_triangular_skewness(min, max, mode)

**签名**：`sr_triangular_skewness(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_skewness(0.0, 2.0, 1.0)
-- 0.0
```

### sr_triangular_std_dev(min, max, mode)

**签名**：`sr_triangular_std_dev(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_std_dev(0.0, 2.0, 1.0)
-- 0.408248290463863
```

### sr_triangular_variance(min, max, mode)

**签名**：`sr_triangular_variance(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular（三角）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_variance(0.0, 2.0, 1.0)
-- 0.16666666666666666
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

### sr_dirac_entropy(v)

**签名**：`sr_dirac_entropy(v DOUBLE) -> DOUBLE`

Dirac 退化的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_entropy(1.0)
-- 0.0
```

### sr_dirac_max(v)

**签名**：`sr_dirac_max(v DOUBLE) -> DOUBLE`

Dirac 退化的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_max(1.0)
-- 1.0
```

### sr_dirac_mean(v)

**签名**：`sr_dirac_mean(v DOUBLE) -> DOUBLE`

Dirac 退化的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_mean(1.0)
-- 1.0
```

### sr_dirac_median(v)

**签名**：`sr_dirac_median(v DOUBLE) -> DOUBLE`

Dirac 退化的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_median(1.0)
-- 1.0
```

### sr_dirac_min(v)

**签名**：`sr_dirac_min(v DOUBLE) -> DOUBLE`

Dirac 退化的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_min(1.0)
-- 1.0
```

### sr_dirac_mode(v)

**签名**：`sr_dirac_mode(v DOUBLE) -> DOUBLE`

Dirac 退化的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_mode(1.0)
-- 1.0
```

### sr_dirac_skewness(v)

**签名**：`sr_dirac_skewness(v DOUBLE) -> DOUBLE`

Dirac 退化的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_skewness(1.0)
-- 0.0
```

### sr_dirac_std_dev(v)

**签名**：`sr_dirac_std_dev(v DOUBLE) -> DOUBLE`

Dirac 退化的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_std_dev(1.0)
-- 0.0
```

### sr_dirac_variance(v)

**签名**：`sr_dirac_variance(v DOUBLE) -> DOUBLE`

Dirac 退化的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_variance(1.0)
-- 0.0
```
