---
title: Beta
sidebar_position: 3
description: Beta distribution — pdf, ln_pdf, cdf, sf, quantile, moments and support. Support [0, 1].
---

# Beta

Parameters: `shape_a` (> 0), `shape_b` (> 0). Support: `[0, 1]`.

### sr_beta_pdf(x, shape_a, shape_b)

**Signature**: `sr_beta_pdf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_pdf(0.5, 2.0, 3.0)
-- 1.5
```

### sr_beta_ln_pdf(x, shape_a, shape_b)

**Signature**: `sr_beta_ln_pdf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_ln_pdf(0.5, 2.0, 3.0)
```

### sr_beta_cdf(x, shape_a, shape_b)

**Signature**: `sr_beta_cdf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_cdf(0.5, 2.0, 3.0)
-- 0.6875
```

### sr_beta_sf(x, shape_a, shape_b)

**Signature**: `sr_beta_sf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_sf(0.5, 2.0, 3.0)
```

### sr_beta_quantile(p, shape_a, shape_b)

**Signature**: `sr_beta_quantile(p DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_quantile(0.5, 2.0, 3.0)
```

### sr_beta_entropy(shape_a, shape_b)

**Signature**: `sr_beta_entropy(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta differential entropy, given shape_a and shape_b.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_entropy(2.0, 3.0)
-- -0.2349066497880017
```

### sr_beta_max(shape_a, shape_b)

**Signature**: `sr_beta_max(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta maximum of the support (1).

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_max(2.0, 3.0)
-- 1.0
```

### sr_beta_mean(shape_a, shape_b)

**Signature**: `sr_beta_mean(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta mean, given shape_a and shape_b.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_mean(2.0, 3.0)
-- 0.4
```

### sr_beta_min(shape_a, shape_b)

**Signature**: `sr_beta_min(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_min(2.0, 3.0)
-- 0.0
```

### sr_beta_mode(shape_a, shape_b)

**Signature**: `sr_beta_mode(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta mode, given shape_a and shape_b.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_mode(2.0, 3.0)
-- 0.3333333333333333
```

### sr_beta_skewness(shape_a, shape_b)

**Signature**: `sr_beta_skewness(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta skewness, given shape_a and shape_b.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_skewness(2.0, 3.0)
-- 0.28571428571428575
```

### sr_beta_std_dev(shape_a, shape_b)

**Signature**: `sr_beta_std_dev(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta standard deviation, given shape_a and shape_b.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_std_dev(2.0, 3.0)
-- 0.2
```

### sr_beta_variance(shape_a, shape_b)

**Signature**: `sr_beta_variance(shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

Beta variance, given shape_a and shape_b.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_variance(2.0, 3.0)
-- 0.04
```
