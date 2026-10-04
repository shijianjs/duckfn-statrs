---
title: 位置-尺度
sidebar_position: 4
description: 柯西与 Laplace 分布——关于位置对称的位置-尺度族，每种提供 pdf / ln_pdf / cdf / sf / quantile、各阶矩与支撑。
---

# 位置-尺度

柯西与 Laplace 分布关于 `location` 对称，由 `scale`（必须 > 0）缩放。

## 柯西

### sr_cauchy_pdf(x, location, scale)

**签名**：`sr_cauchy_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_pdf(0.0, 0.0, 1.0)
-- 0.3183098861837907
```

### sr_cauchy_ln_pdf(x, location, scale)

**签名**：`sr_cauchy_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_ln_pdf(0.0, 0.0, 1.0)
-- -1.1447298858494002
```

### sr_cauchy_cdf(x, location, scale)

**签名**：`sr_cauchy_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_cdf(1.0, 0.0, 1.0)
-- 0.75
```

### sr_cauchy_sf(x, location, scale)

**签名**：`sr_cauchy_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_sf(1.0, 0.0, 1.0)
-- 0.25
```

### sr_cauchy_quantile(p, location, scale)

**签名**：`sr_cauchy_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_quantile(0.75, 0.0, 1.0)
-- 1.0
```

### sr_cauchy_entropy(location, scale)

**签名**：`sr_cauchy_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_entropy(0.0, 1.0)
-- 2.5310242469692907
```

### sr_cauchy_max(location, scale)

**签名**：`sr_cauchy_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_max(0.0, 1.0)
-- inf
```

### sr_cauchy_mean(location, scale)

**签名**：`sr_cauchy_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_mean(0.0, 1.0)
-- NULL
```

### sr_cauchy_median(location, scale)

**签名**：`sr_cauchy_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_median(0.0, 1.0)
-- 0.0
```

### sr_cauchy_min(location, scale)

**签名**：`sr_cauchy_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_min(0.0, 1.0)
-- -inf
```

### sr_cauchy_mode(location, scale)

**签名**：`sr_cauchy_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_mode(0.0, 1.0)
-- 0.0
```

### sr_cauchy_skewness(location, scale)

**签名**：`sr_cauchy_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_skewness(0.0, 1.0)
-- NULL
```

### sr_cauchy_std_dev(location, scale)

**签名**：`sr_cauchy_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_std_dev(0.0, 1.0)
-- NULL
```

### sr_cauchy_variance(location, scale)

**签名**：`sr_cauchy_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

柯西的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_variance(0.0, 1.0)
-- NULL
```

## Laplace

### sr_laplace_pdf(x, location, scale)

**签名**：`sr_laplace_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_pdf(0.0, 0.0, 1.0)
-- 0.5
```

### sr_laplace_ln_pdf(x, location, scale)

**签名**：`sr_laplace_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_ln_pdf(0.0, 0.0, 1.0)
-- -0.6931471805599453
```

### sr_laplace_cdf(x, location, scale)

**签名**：`sr_laplace_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_cdf(1.0, 0.0, 1.0)
```

### sr_laplace_sf(x, location, scale)

**签名**：`sr_laplace_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_sf(1.0, 0.0, 1.0)
```

### sr_laplace_quantile(p, location, scale)

**签名**：`sr_laplace_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_quantile(0.5, 0.0, 1.0)
-- 0.0
```

### sr_laplace_entropy(location, scale)

**签名**：`sr_laplace_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_entropy(0.0, 1.0)
-- 1.6931471805599454
```

### sr_laplace_max(location, scale)

**签名**：`sr_laplace_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_max(0.0, 1.0)
-- inf
```

### sr_laplace_mean(location, scale)

**签名**：`sr_laplace_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_mean(0.0, 1.0)
-- 0.0
```

### sr_laplace_median(location, scale)

**签名**：`sr_laplace_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_median(0.0, 1.0)
-- 0.0
```

### sr_laplace_min(location, scale)

**签名**：`sr_laplace_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_min(0.0, 1.0)
-- -inf
```

### sr_laplace_mode(location, scale)

**签名**：`sr_laplace_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_mode(0.0, 1.0)
-- 0.0
```

### sr_laplace_skewness(location, scale)

**签名**：`sr_laplace_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_skewness(0.0, 1.0)
-- 0.0
```

### sr_laplace_std_dev(location, scale)

**签名**：`sr_laplace_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_std_dev(0.0, 1.0)
-- 1.4142135623730951
```

### sr_laplace_variance(location, scale)

**签名**：`sr_laplace_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_variance(0.0, 1.0)
-- 2.0
```
