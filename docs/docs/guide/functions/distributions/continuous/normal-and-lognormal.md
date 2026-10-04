---
title: Normal and log-normal
sidebar_position: 1
description: Normal (Gaussian) and log-normal distributions — pdf, ln_pdf, cdf, sf, quantile for each.
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
