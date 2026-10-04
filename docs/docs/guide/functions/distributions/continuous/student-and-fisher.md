---
title: "Student's t and Fisher-Snedecor"
sidebar_position: 6
description: Student's t and Fisher-Snedecor (F) distributions — pdf, ln_pdf, cdf, sf, quantile, plus moments and support.
---

# Student's t and Fisher-Snedecor

## Student's t

Parameters: `location` (any real), `scale` (> 0), `freedom` (> 0).

### sr_students_t_pdf(x, location, scale, freedom)

**Signature**: `sr_students_t_pdf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_pdf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_ln_pdf(x, location, scale, freedom)

**Signature**: `sr_students_t_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_ln_pdf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_cdf(x, location, scale, freedom)

**Signature**: `sr_students_t_cdf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_cdf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_sf(x, location, scale, freedom)

**Signature**: `sr_students_t_sf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_sf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_quantile(p, location, scale, freedom)

**Signature**: `sr_students_t_quantile(p DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Solved numerically (bisection) by statrs.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_quantile(0.975, 0.0, 1.0, 10.0)
```

### sr_students_t_entropy(location, scale, freedom)

**Signature**: `sr_students_t_entropy(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution differential entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_entropy(0.0, 1.0, 10.0)
-- 1.5212624929756853
```

### sr_students_t_max(location, scale, freedom)

**Signature**: `sr_students_t_max(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution support maximum (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_max(0.0, 1.0, 10.0)
-- inf
```

### sr_students_t_mean(location, scale, freedom)

**Signature**: `sr_students_t_mean(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_mean(0.0, 1.0, 2.0)
-- 0.0
```

### sr_students_t_median(location, scale, freedom)

**Signature**: `sr_students_t_median(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_median(0.0, 1.0, 10.0)
-- 0.0
```

### sr_students_t_min(location, scale, freedom)

**Signature**: `sr_students_t_min(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution support minimum (negative infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_min(0.0, 1.0, 10.0)
-- -inf
```

### sr_students_t_mode(location, scale, freedom)

**Signature**: `sr_students_t_mode(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_mode(0.0, 1.0, 10.0)
-- 0.0
```

### sr_students_t_skewness(location, scale, freedom)

**Signature**: `sr_students_t_skewness(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_skewness(0.0, 1.0, 10.0)
-- 0.0
```

### sr_students_t_std_dev(location, scale, freedom)

**Signature**: `sr_students_t_std_dev(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_std_dev(0.0, 1.0, 10.0)
-- 1.118033988749895
```

### sr_students_t_variance(location, scale, freedom)

**Signature**: `sr_students_t_variance(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t distribution variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_variance(0.0, 1.0, 10.0)
-- 1.25
```

## Fisher-Snedecor (F distribution)

Parameters: `df_num` (> 0), `df_denom` (> 0).

### sr_fisher_snedecor_pdf(x, df_num, df_denom)

**Signature**: `sr_fisher_snedecor_pdf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_pdf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_ln_pdf(x, df_num, df_denom)

**Signature**: `sr_fisher_snedecor_ln_pdf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_ln_pdf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_cdf(x, df_num, df_denom)

**Signature**: `sr_fisher_snedecor_cdf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_cdf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_sf(x, df_num, df_denom)

**Signature**: `sr_fisher_snedecor_sf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_sf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_quantile(p, df_num, df_denom)

**Signature**: `sr_fisher_snedecor_quantile(p DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_quantile(0.95, 2.0, 3.0)
```

### sr_fisher_snedecor_entropy(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_entropy(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) differential entropy, given the two degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_entropy(3.0, 5.0)
-- NULL
```

### sr_fisher_snedecor_max(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_max(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_max(3.0, 5.0)
-- inf
```

### sr_fisher_snedecor_mean(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_mean(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) mean, given the two degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_mean(3.0, 5.0)
-- 1.6666666666666667
```

### sr_fisher_snedecor_min(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_min(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_min(3.0, 5.0)
-- 0.0
```

### sr_fisher_snedecor_mode(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_mode(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) mode, given the two degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_mode(3.0, 5.0)
-- 0.23809523809523808
```

### sr_fisher_snedecor_skewness(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_skewness(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) skewness, given the two degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_skewness(3.0, 7.0)
-- 11.0
```

### sr_fisher_snedecor_std_dev(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_std_dev(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) standard deviation, given the two degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_std_dev(3.0, 5.0)
-- 3.3333333333333335
```

### sr_fisher_snedecor_variance(freedom_1, freedom_2)

**Signature**: `sr_fisher_snedecor_variance(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor (F) variance, given the two degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_variance(3.0, 5.0)
-- 11.11111111111111
```
