---
title: Uniform, triangular and Dirac
sidebar_position: 7
description: Continuous uniform, triangular and Dirac (degenerate) distributions.
---

# Uniform, triangular and Dirac

## Uniform (continuous)

Parameters: `min`, `max` (min < max). Support: `[min, max]`.

### sr_uniform_pdf(x, min, max)

**Signature**: `sr_uniform_pdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_pdf(0.5, 0.0, 1.0)
-- 1.0
```

### sr_uniform_ln_pdf(x, min, max)

**Signature**: `sr_uniform_ln_pdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_ln_pdf(0.5, 0.0, 1.0)
```

### sr_uniform_cdf(x, min, max)

**Signature**: `sr_uniform_cdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_cdf(0.5, 0.0, 1.0)
-- 0.5
```

### sr_uniform_sf(x, min, max)

**Signature**: `sr_uniform_sf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_sf(0.5, 0.0, 1.0)
```

### sr_uniform_quantile(p, min, max)

**Signature**: `sr_uniform_quantile(p DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_quantile(0.25, 0.0, 1.0)
-- 0.25
```

### sr_uniform_entropy(min, max)

**Signature**: `sr_uniform_entropy(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_entropy(0.0, 1.0)
-- 0.0
```

### sr_uniform_max(min, max)

**Signature**: `sr_uniform_max(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution support maximum (max).

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_max(0.0, 1.0)
-- 1.0
```

### sr_uniform_mean(min, max)

**Signature**: `sr_uniform_mean(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_mean(0.0, 1.0)
-- 0.5
```

### sr_uniform_median(min, max)

**Signature**: `sr_uniform_median(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_median(0.0, 1.0)
-- 0.5
```

### sr_uniform_min(min, max)

**Signature**: `sr_uniform_min(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution support minimum (min).

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_min(0.0, 1.0)
-- 0.0
```

### sr_uniform_mode(min, max)

**Signature**: `sr_uniform_mode(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_mode(0.0, 1.0)
-- 0.5
```

### sr_uniform_skewness(min, max)

**Signature**: `sr_uniform_skewness(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution skewness (always 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_skewness(0.0, 1.0)
-- 0.0
```

### sr_uniform_std_dev(min, max)

**Signature**: `sr_uniform_std_dev(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_std_dev(0.0, 1.0)
-- 0.28867513459481287
```

### sr_uniform_variance(min, max)

**Signature**: `sr_uniform_variance(min DOUBLE, max DOUBLE) -> DOUBLE`

Continuous uniform distribution variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_variance(0.0, 1.0)
-- 0.08333333333333333
```

## Triangular

Parameters: `min`, `max`, `mode`. Support: `[min, max]`.

### sr_triangular_pdf(x, min, max, mode)

**Signature**: `sr_triangular_pdf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_pdf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_ln_pdf(x, min, max, mode)

**Signature**: `sr_triangular_ln_pdf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_ln_pdf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_cdf(x, min, max, mode)

**Signature**: `sr_triangular_cdf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_cdf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_sf(x, min, max, mode)

**Signature**: `sr_triangular_sf(x DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_sf(1.0, 0.0, 2.0, 1.0)
```

### sr_triangular_quantile(p, min, max, mode)

**Signature**: `sr_triangular_quantile(p DOUBLE, min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_quantile(0.5, 0.0, 2.0, 1.0)
```

### sr_triangular_entropy(min, max, mode)

**Signature**: `sr_triangular_entropy(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_entropy(0.0, 2.0, 1.0)
-- 0.5
```

### sr_triangular_max(min, max, mode)

**Signature**: `sr_triangular_max(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution support maximum (max).

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_max(0.0, 2.0, 1.0)
-- 2.0
```

### sr_triangular_mean(min, max, mode)

**Signature**: `sr_triangular_mean(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_mean(0.0, 2.0, 1.0)
-- 1.0
```

### sr_triangular_median(min, max, mode)

**Signature**: `sr_triangular_median(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_median(0.0, 2.0, 1.0)
-- 1.0
```

### sr_triangular_min(min, max, mode)

**Signature**: `sr_triangular_min(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution support minimum (min).

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_min(0.0, 2.0, 1.0)
-- 0.0
```

### sr_triangular_mode(min, max, mode)

**Signature**: `sr_triangular_mode(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_mode(0.0, 2.0, 1.0)
-- 1.0
```

### sr_triangular_skewness(min, max, mode)

**Signature**: `sr_triangular_skewness(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_skewness(0.0, 2.0, 1.0)
-- 0.0
```

### sr_triangular_std_dev(min, max, mode)

**Signature**: `sr_triangular_std_dev(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_std_dev(0.0, 2.0, 1.0)
-- 0.408248290463863
```

### sr_triangular_variance(min, max, mode)

**Signature**: `sr_triangular_variance(min DOUBLE, max DOUBLE, mode DOUBLE) -> DOUBLE`

Triangular distribution variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_triangular_variance(0.0, 2.0, 1.0)
-- 0.16666666666666666
```

## Dirac delta

Parameter: `location` (v). A degenerate distribution concentrated at a point. Only `cdf`,
`sf`, and `quantile` are exposed (there is no ordinary density).

### sr_dirac_cdf(x, location)

**Signature**: `sr_dirac_cdf(x DOUBLE, location DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_cdf(1.0, 1.0)
-- 1.0
```

### sr_dirac_sf(x, location)

**Signature**: `sr_dirac_sf(x DOUBLE, location DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_sf(0.5, 1.0)
```

### sr_dirac_quantile(p, location)

**Signature**: `sr_dirac_quantile(p DOUBLE, location DOUBLE) -> DOUBLE`

Always returns `location` regardless of `p`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_quantile(0.5, 1.0)
-- 1.0
```

### sr_dirac_entropy(v)

**Signature**: `sr_dirac_entropy(v DOUBLE) -> DOUBLE`

Dirac delta distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_entropy(1.0)
-- 0.0
```

### sr_dirac_max(v)

**Signature**: `sr_dirac_max(v DOUBLE) -> DOUBLE`

Dirac delta distribution support maximum (v).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_max(1.0)
-- 1.0
```

### sr_dirac_mean(v)

**Signature**: `sr_dirac_mean(v DOUBLE) -> DOUBLE`

Dirac delta distribution mean (always v).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_mean(1.0)
-- 1.0
```

### sr_dirac_median(v)

**Signature**: `sr_dirac_median(v DOUBLE) -> DOUBLE`

Dirac delta distribution median (always v).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_median(1.0)
-- 1.0
```

### sr_dirac_min(v)

**Signature**: `sr_dirac_min(v DOUBLE) -> DOUBLE`

Dirac delta distribution support minimum (v).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_min(1.0)
-- 1.0
```

### sr_dirac_mode(v)

**Signature**: `sr_dirac_mode(v DOUBLE) -> DOUBLE`

Dirac delta distribution mode (always v).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_mode(1.0)
-- 1.0
```

### sr_dirac_skewness(v)

**Signature**: `sr_dirac_skewness(v DOUBLE) -> DOUBLE`

Dirac delta distribution skewness (always 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_skewness(1.0)
-- 0.0
```

### sr_dirac_std_dev(v)

**Signature**: `sr_dirac_std_dev(v DOUBLE) -> DOUBLE`

Dirac delta distribution standard deviation (always 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_std_dev(1.0)
-- 0.0
```

### sr_dirac_variance(v)

**Signature**: `sr_dirac_variance(v DOUBLE) -> DOUBLE`

Dirac delta distribution variance (always 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_variance(1.0)
-- 0.0
```
