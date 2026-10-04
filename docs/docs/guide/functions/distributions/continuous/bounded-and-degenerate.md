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
