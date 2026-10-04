---
title: Location-scale
sidebar_position: 4
description: Cauchy and Laplace distributions — symmetric location-scale families with pdf, ln_pdf, cdf, sf, quantile.
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
