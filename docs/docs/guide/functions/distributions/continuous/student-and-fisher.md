---
title: "Student's t and Fisher-Snedecor"
sidebar_position: 6
description: Student's t and Fisher-Snedecor (F) distributions — pdf, ln_pdf, cdf, sf, quantile for each.
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
