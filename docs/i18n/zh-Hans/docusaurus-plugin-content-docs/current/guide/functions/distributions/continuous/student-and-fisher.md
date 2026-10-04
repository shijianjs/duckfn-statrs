---
title: "Student's t 与 Fisher-Snedecor"
sidebar_position: 6
description: Student's t 与 Fisher-Snedecor（F）分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile。
---

# Student's t 与 Fisher-Snedecor

## Student's t

参数：`location`（任意实数）、`scale`（> 0）、`freedom`（> 0）。

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

statrs 用二分法数值求解。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_quantile(0.975, 0.0, 1.0, 10.0)
```

## Fisher-Snedecor（F 分布）

参数：`df_num`（> 0）、`df_denom`（> 0）。

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
