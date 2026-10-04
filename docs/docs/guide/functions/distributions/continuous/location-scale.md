---
title: Location-scale
sidebar_position: 4
description: Cauchy and Laplace distributions — symmetric location-scale families with pdf, ln_pdf, cdf, sf, quantile, plus moments and support.
---

# Location-scale

The Cauchy and Laplace distributions are symmetric about their `location`, scaled by `scale`
(must be > 0).

## Cauchy

### sr_cauchy_pdf(x, location, scale)

**Signature**: `sr_cauchy_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_pdf(0.0, 0.0, 1.0)
-- 0.3183098861837907
```

### sr_cauchy_ln_pdf(x, location, scale)

**Signature**: `sr_cauchy_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_ln_pdf(0.0, 0.0, 1.0)
-- -1.1447298858494002
```

### sr_cauchy_cdf(x, location, scale)

**Signature**: `sr_cauchy_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_cdf(1.0, 0.0, 1.0)
-- 0.75
```

### sr_cauchy_sf(x, location, scale)

**Signature**: `sr_cauchy_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_sf(1.0, 0.0, 1.0)
-- 0.25
```

### sr_cauchy_quantile(p, location, scale)

**Signature**: `sr_cauchy_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_quantile(0.75, 0.0, 1.0)
-- 1.0
```

### sr_cauchy_entropy(location, scale)

**Signature**: `sr_cauchy_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_entropy(0.0, 1.0)
-- 2.5310242469692907
```

### sr_cauchy_max(location, scale)

**Signature**: `sr_cauchy_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution support maximum (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_max(0.0, 1.0)
-- inf
```

### sr_cauchy_mean(location, scale)

**Signature**: `sr_cauchy_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution mean (undefined).

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_mean(0.0, 1.0)
-- NULL
```

### sr_cauchy_median(location, scale)

**Signature**: `sr_cauchy_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_median(0.0, 1.0)
-- 0.0
```

### sr_cauchy_min(location, scale)

**Signature**: `sr_cauchy_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution support minimum (negative infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_min(0.0, 1.0)
-- -inf
```

### sr_cauchy_mode(location, scale)

**Signature**: `sr_cauchy_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_mode(0.0, 1.0)
-- 0.0
```

### sr_cauchy_skewness(location, scale)

**Signature**: `sr_cauchy_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution skewness (undefined).

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_skewness(0.0, 1.0)
-- NULL
```

### sr_cauchy_std_dev(location, scale)

**Signature**: `sr_cauchy_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution standard deviation (undefined).

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_std_dev(0.0, 1.0)
-- NULL
```

### sr_cauchy_variance(location, scale)

**Signature**: `sr_cauchy_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

Cauchy distribution variance (undefined).

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_variance(0.0, 1.0)
-- NULL
```

## Laplace

### sr_laplace_pdf(x, location, scale)

**Signature**: `sr_laplace_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_pdf(0.0, 0.0, 1.0)
-- 0.5
```

### sr_laplace_ln_pdf(x, location, scale)

**Signature**: `sr_laplace_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_ln_pdf(0.0, 0.0, 1.0)
-- -0.6931471805599453
```

### sr_laplace_cdf(x, location, scale)

**Signature**: `sr_laplace_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_cdf(1.0, 0.0, 1.0)
```

### sr_laplace_sf(x, location, scale)

**Signature**: `sr_laplace_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_sf(1.0, 0.0, 1.0)
```

### sr_laplace_quantile(p, location, scale)

**Signature**: `sr_laplace_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_quantile(0.5, 0.0, 1.0)
-- 0.0
```

### sr_laplace_entropy(location, scale)

**Signature**: `sr_laplace_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_entropy(0.0, 1.0)
-- 1.6931471805599454
```

### sr_laplace_max(location, scale)

**Signature**: `sr_laplace_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution support maximum (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_max(0.0, 1.0)
-- inf
```

### sr_laplace_mean(location, scale)

**Signature**: `sr_laplace_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_mean(0.0, 1.0)
-- 0.0
```

### sr_laplace_median(location, scale)

**Signature**: `sr_laplace_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_median(0.0, 1.0)
-- 0.0
```

### sr_laplace_min(location, scale)

**Signature**: `sr_laplace_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution support minimum (negative infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_min(0.0, 1.0)
-- -inf
```

### sr_laplace_mode(location, scale)

**Signature**: `sr_laplace_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_mode(0.0, 1.0)
-- 0.0
```

### sr_laplace_skewness(location, scale)

**Signature**: `sr_laplace_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution skewness (always 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_skewness(0.0, 1.0)
-- 0.0
```

### sr_laplace_std_dev(location, scale)

**Signature**: `sr_laplace_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_std_dev(0.0, 1.0)
-- 1.4142135623730951
```

### sr_laplace_variance(location, scale)

**Signature**: `sr_laplace_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

Laplace distribution variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_variance(0.0, 1.0)
-- 2.0
```
