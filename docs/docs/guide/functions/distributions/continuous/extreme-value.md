---
title: Extreme value and heavy tail
sidebar_position: 5
description: Gumbel, Weibull, Levy and Pareto distributions — pdf, ln_pdf, cdf, sf, quantile for each.
---

# Extreme value and heavy tail

Gumbel and Weibull are extreme-value distributions (bounded / unbounded tails); Levy and
Pareto are heavy-tailed.

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

## Weibull

Parameters: `shape` (> 0), `scale` (> 0).

### sr_weibull_pdf(x, shape, scale)

**Signature**: `sr_weibull_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_pdf(1.0, 1.0, 1.0)
-- 1.0
```

### sr_weibull_ln_pdf(x, shape, scale)

**Signature**: `sr_weibull_ln_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_ln_pdf(1.0, 1.0, 1.0)
```

### sr_weibull_cdf(x, shape, scale)

**Signature**: `sr_weibull_cdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_cdf(1.0, 1.0, 1.0)
-- 0.6321205588285577
```

### sr_weibull_sf(x, shape, scale)

**Signature**: `sr_weibull_sf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_sf(1.0, 1.0, 1.0)
```

### sr_weibull_quantile(p, shape, scale)

**Signature**: `sr_weibull_quantile(p DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_quantile(0.5, 1.0, 1.0)
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

## Pareto (Type-I)

Parameters: `scale` (x_m, > 0), `shape` (alpha, > 0). Support: `[x_m, infinity)`.

### sr_pareto_pdf(x, scale, shape)

**Signature**: `sr_pareto_pdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_pdf(1.0, 1.0, 2.0)
-- 2.0
```

### sr_pareto_ln_pdf(x, scale, shape)

**Signature**: `sr_pareto_ln_pdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_ln_pdf(1.0, 1.0, 2.0)
```

### sr_pareto_cdf(x, scale, shape)

**Signature**: `sr_pareto_cdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_cdf(2.0, 1.0, 2.0)
```

### sr_pareto_sf(x, scale, shape)

**Signature**: `sr_pareto_sf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_sf(2.0, 1.0, 2.0)
```

### sr_pareto_quantile(p, scale, shape)

**Signature**: `sr_pareto_quantile(p DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_quantile(0.5, 1.0, 2.0)
```
