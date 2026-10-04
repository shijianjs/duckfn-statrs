---
title: "Continuous: shape (A)"
sidebar_position: 6
description: Gamma, inverse-gamma and chi-squared distributions — pdf, ln_pdf, cdf, sf, quantile for each.
---

# Continuous distributions: shape (A)

## Gamma

Parameters: `shape` (> 0), `rate` (> 0). Shape/rate parameterisation.

### sr_gamma_pdf(x, shape, rate)

**Signature**: `sr_gamma_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_pdf(1.0, 2.0, 2.0)
-- 0.2706705664732254
```

### sr_gamma_ln_pdf(x, shape, rate)

**Signature**: `sr_gamma_ln_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_ln_pdf(1.0, 2.0, 2.0)
```

### sr_gamma_cdf(x, shape, rate)

**Signature**: `sr_gamma_cdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_cdf(1.0, 2.0, 2.0)
```

### sr_gamma_sf(x, shape, rate)

**Signature**: `sr_gamma_sf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_sf(1.0, 2.0, 2.0)
```

### sr_gamma_quantile(p, shape, rate)

**Signature**: `sr_gamma_quantile(p DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_quantile(0.5, 2.0, 2.0)
```

## Inverse-gamma

Parameters: `shape` (> 0), `scale` (> 0).

### sr_inverse_gamma_pdf(x, shape, scale)

**Signature**: `sr_inverse_gamma_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_pdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_ln_pdf(x, shape, scale)

**Signature**: `sr_inverse_gamma_ln_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_ln_pdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_cdf(x, shape, scale)

**Signature**: `sr_inverse_gamma_cdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_cdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_sf(x, shape, scale)

**Signature**: `sr_inverse_gamma_sf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_sf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_quantile(p, shape, scale)

**Signature**: `sr_inverse_gamma_quantile(p DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_quantile(0.5, 2.0, 2.0)
```

## Chi-squared

Parameter: `freedom` (degrees of freedom, > 0).

### sr_chi_squared_pdf(x, freedom)

**Signature**: `sr_chi_squared_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_pdf(1.0, 2.0)
```

### sr_chi_squared_ln_pdf(x, freedom)

**Signature**: `sr_chi_squared_ln_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_ln_pdf(1.0, 2.0)
```

### sr_chi_squared_cdf(x, freedom)

**Signature**: `sr_chi_squared_cdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_cdf(1.0, 2.0)
```

### sr_chi_squared_sf(x, freedom)

**Signature**: `sr_chi_squared_sf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_sf(1.0, 2.0)
```

### sr_chi_squared_quantile(p, freedom)

**Signature**: `sr_chi_squared_quantile(p DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_quantile(0.95, 2.0)
```
