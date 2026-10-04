---
title: "Continuous: location-scale (B)"
sidebar_position: 5
description: Laplace, Gumbel, Levy and Logistic distributions — pdf, ln_pdf, cdf, sf, quantile for each.
---

# Continuous distributions: location-scale (B)

Each distribution exposes 5 functions: `pdf`, `ln_pdf`, `cdf`, `sf`, `quantile`. The `scale`
parameter must be > 0; `p` in the quantile must be in [0, 1].

## Laplace

Parameters: `location` (any real), `scale` (must be > 0).

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

## Gumbel (Type-I extreme value)

Parameters: `location`, `scale` (must be > 0).

### sr_gumbel_pdf(x, location, scale)

**Signature**: `sr_gumbel_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_pdf(0.0, 0.0, 1.0)
-- 0.3678794411714424
```

### sr_gumbel_ln_pdf(x, location, scale)

**Signature**: `sr_gumbel_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_ln_pdf(0.0, 0.0, 1.0)
```

### sr_gumbel_cdf(x, location, scale)

**Signature**: `sr_gumbel_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_cdf(1.0, 0.0, 1.0)
```

### sr_gumbel_sf(x, location, scale)

**Signature**: `sr_gumbel_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_sf(1.0, 0.0, 1.0)
```

### sr_gumbel_quantile(p, location, scale)

**Signature**: `sr_gumbel_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_quantile(0.5, 0.0, 1.0)
```

## Levy

Parameters: `mu` (location, any real), `c` (scale, must be > 0).

### sr_levy_pdf(x, mu, c)

**Signature**: `sr_levy_pdf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_pdf(1.0, 0.0, 1.0)
```

### sr_levy_ln_pdf(x, mu, c)

**Signature**: `sr_levy_ln_pdf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_ln_pdf(1.0, 0.0, 1.0)
```

### sr_levy_cdf(x, mu, c)

**Signature**: `sr_levy_cdf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_cdf(1.0, 0.0, 1.0)
```

### sr_levy_sf(x, mu, c)

**Signature**: `sr_levy_sf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_sf(1.0, 0.0, 1.0)
```

### sr_levy_quantile(p, mu, c)

**Signature**: `sr_levy_quantile(p DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

Solved numerically (bisection) by statrs; accuracy is lower than the closed-form quantiles.

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_quantile(0.5, 0.0, 1.0)
```

## Logistic

Parameters: `location`, `scale` (must be > 0). Same functional shape as the sigmoid.

### sr_logistic_dist_pdf(x, location, scale)

The registered SQL name is `sr_logistic_pdf` — distinct from `sr_logistic(p)` (the sigmoid
function on the special-functions page).

**Signature**: `sr_logistic_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_pdf(0.0, 0.0, 1.0)
-- 0.25
```

### sr_logistic_ln_pdf(x, location, scale)

**Signature**: `sr_logistic_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_ln_pdf(0.0, 0.0, 1.0)
```

### sr_logistic_cdf(x, location, scale)

**Signature**: `sr_logistic_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_cdf(1.0, 0.0, 1.0)
```

### sr_logistic_sf(x, location, scale)

**Signature**: `sr_logistic_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_sf(1.0, 0.0, 1.0)
```

### sr_logistic_quantile(p, location, scale)

**Signature**: `sr_logistic_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_quantile(0.5, 0.0, 1.0)
-- 0.0
```
