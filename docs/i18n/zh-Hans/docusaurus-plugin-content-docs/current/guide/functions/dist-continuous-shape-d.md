---
title: "连续：形状 (D)"
sidebar_position: 9
description: 均匀、Student-t、Fisher-Snedecor 与 Dirac 分布。
---

# 连续分布：形状 (D)

## Uniform (continuous)

参数：`min`, `max` (min < max). 支撑：`[min, max]`.

### sr_uniform_pdf(x, min, max)

**签名**：`sr_uniform_pdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_pdf(0.5, 0.0, 1.0)
-- 1.0
```

### sr_uniform_ln_pdf(x, min, max)

**签名**：`sr_uniform_ln_pdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_ln_pdf(0.5, 0.0, 1.0)
```

### sr_uniform_cdf(x, min, max)

**签名**：`sr_uniform_cdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_cdf(0.5, 0.0, 1.0)
-- 0.5
```

### sr_uniform_sf(x, min, max)

**签名**：`sr_uniform_sf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_sf(0.5, 0.0, 1.0)
```

### sr_uniform_quantile(p, min, max)

**签名**：`sr_uniform_quantile(p DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_uniform_quantile(0.25, 0.0, 1.0)
-- 0.25
```

## Student's t

参数：`location` （任意实数）, `scale`（> 0）, `freedom`（> 0）。

### sr_students_t_pdf(x, location, scale, freedom)

**签名**：`sr_students_t_pdf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_pdf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_ln_pdf(x, location, scale, freedom)

**签名**：`sr_students_t_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_ln_pdf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_cdf(x, location, scale, freedom)

**签名**：`sr_students_t_cdf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_cdf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_sf(x, location, scale, freedom)

**签名**：`sr_students_t_sf(x DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_sf(1.0, 0.0, 1.0, 2.0)
```

### sr_students_t_quantile(p, location, scale, freedom)

**签名**：`sr_students_t_quantile(p DOUBLE, location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Solved numerically (bisection) by statrs.

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_quantile(0.975, 0.0, 1.0, 10.0)
```

## Fisher-Snedecor (F distribution)

参数：`df_num（> 0）、`df_denom`（> 0）。

### sr_fisher_snedecor_pdf(x, df_num, df_denom)

**签名**：`sr_fisher_snedecor_pdf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_pdf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_ln_pdf(x, df_num, df_denom)

**签名**：`sr_fisher_snedecor_ln_pdf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_ln_pdf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_cdf(x, df_num, df_denom)

**签名**：`sr_fisher_snedecor_cdf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_cdf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_sf(x, df_num, df_denom)

**签名**：`sr_fisher_snedecor_sf(x DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_sf(1.0, 2.0, 3.0)
```

### sr_fisher_snedecor_quantile(p, df_num, df_denom)

**签名**：`sr_fisher_snedecor_quantile(p DOUBLE, df_num DOUBLE, df_denom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_quantile(0.95, 2.0, 3.0)
```

## Dirac delta

参数：`location` (v). A degenerate distribution concentrated at a point. Only `cdf`,
`sf`, and `quantile` are exposed (there is no ordinary density).

### sr_dirac_cdf(x, location)

**签名**：`sr_dirac_cdf(x DOUBLE, location DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_cdf(1.0, 1.0)
-- 1.0
```

### sr_dirac_sf(x, location)

**签名**：`sr_dirac_sf(x DOUBLE, location DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_sf(0.5, 1.0)
```

### sr_dirac_quantile(p, location)

**签名**：`sr_dirac_quantile(p DOUBLE, location DOUBLE) -> DOUBLE`

Always returns `location` regardless of `p`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirac_quantile(0.5, 1.0)
-- 1.0
```
