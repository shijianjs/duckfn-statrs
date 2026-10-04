---
title: Normal and log-normal
sidebar_position: 1
description: Normal (Gaussian) and log-normal distributions — pdf, ln_pdf, cdf, sf, quantile, plus moments and support.
---

# Normal and log-normal

## Normal (Gaussian)

### sr_normal_pdf(x, mean, std_dev)

**Signature**: `sr_normal_pdf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

- `x`: evaluation point
- `mean`: distribution mean (any real number)
- `std_dev`: standard deviation (must be > 0, otherwise query error)

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_pdf(0.0, 0.0, 1.0)
-- 0.3989422804014327
```

### sr_normal_ln_pdf(x, mean, std_dev)

**Signature**: `sr_normal_ln_pdf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Log-density at x.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_ln_pdf(0.0, 0.0, 1.0)
-- -0.9189385332046727
```

### sr_normal_cdf(x, mean, std_dev)

**Signature**: `sr_normal_cdf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Cumulative distribution function `P(X <= x)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_cdf(1.96, 0.0, 1.0)
-- 0.9750021048529024
```

### sr_normal_sf(x, mean, std_dev)

**Signature**: `sr_normal_sf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Survival function `P(X > x) = 1 - CDF`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_sf(1.96, 0.0, 1.0)
-- 0.024997895147097634
```

### sr_normal_quantile(p, mean, std_dev)

**Signature**: `sr_normal_quantile(p DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Inverse CDF: the x whose CDF equals p. `p` must be in [0, 1]; otherwise query error.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_quantile(0.975, 0.0, 1.0)
-- 1.959963984540054
```

### sr_normal_entropy(mean, std_dev)

**Signature**: `sr_normal_entropy(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_entropy(0.0, 1.0)
-- 1.4189385332046727
```

### sr_normal_max(mean, std_dev)

**Signature**: `sr_normal_max(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution support maximum (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_max(0.0, 1.0)
-- inf
```

### sr_normal_mean(mean, std_dev)

**Signature**: `sr_normal_mean(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_mean(0.0, 1.0)
-- 0.0
```

### sr_normal_median(mean, std_dev)

**Signature**: `sr_normal_median(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_median(0.0, 1.0)
-- 0.0
```

### sr_normal_min(mean, std_dev)

**Signature**: `sr_normal_min(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution support minimum (negative infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_min(0.0, 1.0)
-- -inf
```

### sr_normal_mode(mean, std_dev)

**Signature**: `sr_normal_mode(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_mode(0.0, 1.0)
-- 0.0
```

### sr_normal_skewness(mean, std_dev)

**Signature**: `sr_normal_skewness(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution skewness (always 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_skewness(0.0, 1.0)
-- 0.0
```

### sr_normal_std_dev(mean, std_dev)

**Signature**: `sr_normal_std_dev(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_std_dev(0.0, 1.0)
-- 1.0
```

### sr_normal_variance(mean, std_dev)

**Signature**: `sr_normal_variance(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

Normal (Gaussian) distribution variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_variance(0.0, 1.0)
-- 1.0
```

## Log-normal

### sr_log_normal_pdf(x, location, scale)

**Signature**: `sr_log_normal_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

Density at x > 0. `location` and `scale` parameterize the underlying normal of ln(X).

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_pdf(1.0, 0.0, 1.0)
-- 0.3989422804014327
```

### sr_log_normal_ln_pdf(x, location, scale)

**Signature**: `sr_log_normal_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_ln_pdf(1.0, 0.0, 1.0)
-- -0.9189385332046727
```

### sr_log_normal_cdf(x, location, scale)

**Signature**: `sr_log_normal_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_cdf(1.0, 0.0, 1.0)
-- 0.5
```

### sr_log_normal_sf(x, location, scale)

**Signature**: `sr_log_normal_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_sf(1.0, 0.0, 1.0)
-- 0.5
```

### sr_log_normal_quantile(p, location, scale)

**Signature**: `sr_log_normal_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_quantile(0.5, 0.0, 1.0)
-- 1.0
```

### sr_log_normal_entropy(location, scale)

**Signature**: `sr_log_normal_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_entropy(0.0, 1.0)
-- 1.4189385332046727
```

### sr_log_normal_max(location, scale)

**Signature**: `sr_log_normal_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution support maximum (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_max(0.0, 1.0)
-- inf
```

### sr_log_normal_mean(location, scale)

**Signature**: `sr_log_normal_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_mean(0.0, 1.0)
-- 1.6487212707001282
```

### sr_log_normal_median(location, scale)

**Signature**: `sr_log_normal_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_median(0.0, 1.0)
-- 1.0
```

### sr_log_normal_min(location, scale)

**Signature**: `sr_log_normal_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution support minimum (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_min(0.0, 1.0)
-- 0.0
```

### sr_log_normal_mode(location, scale)

**Signature**: `sr_log_normal_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_mode(0.0, 1.0)
-- 0.36787944117144233
```

### sr_log_normal_skewness(location, scale)

**Signature**: `sr_log_normal_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_skewness(0.0, 1.0)
-- 6.184877138632554
```

### sr_log_normal_std_dev(location, scale)

**Signature**: `sr_log_normal_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_std_dev(0.0, 1.0)
-- 2.1611974158950877
```

### sr_log_normal_variance(location, scale)

**Signature**: `sr_log_normal_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

Log-normal distribution variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_variance(0.0, 1.0)
-- 4.670774270471604
```
