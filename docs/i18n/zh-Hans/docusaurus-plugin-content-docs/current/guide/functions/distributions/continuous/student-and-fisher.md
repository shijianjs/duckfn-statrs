---
title: "Student's t 与 Fisher-Snedecor"
sidebar_position: 6
description: Student's t 与 Fisher-Snedecor（F）分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile，以及各阶矩与支撑。
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

### sr_students_t_entropy(location, scale, freedom)

**签名**：`sr_students_t_entropy(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_entropy(0.0, 1.0, 10.0)
-- 1.5212624929756853
```

### sr_students_t_max(location, scale, freedom)

**签名**：`sr_students_t_max(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_max(0.0, 1.0, 10.0)
-- inf
```

### sr_students_t_mean(location, scale, freedom)

**签名**：`sr_students_t_mean(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_mean(0.0, 1.0, 2.0)
-- 0.0
```

### sr_students_t_median(location, scale, freedom)

**签名**：`sr_students_t_median(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_median(0.0, 1.0, 10.0)
-- 0.0
```

### sr_students_t_min(location, scale, freedom)

**签名**：`sr_students_t_min(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_min(0.0, 1.0, 10.0)
-- -inf
```

### sr_students_t_mode(location, scale, freedom)

**签名**：`sr_students_t_mode(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_mode(0.0, 1.0, 10.0)
-- 0.0
```

### sr_students_t_skewness(location, scale, freedom)

**签名**：`sr_students_t_skewness(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_skewness(0.0, 1.0, 10.0)
-- 0.0
```

### sr_students_t_std_dev(location, scale, freedom)

**签名**：`sr_students_t_std_dev(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_std_dev(0.0, 1.0, 10.0)
-- 1.118033988749895
```

### sr_students_t_variance(location, scale, freedom)

**签名**：`sr_students_t_variance(location DOUBLE, scale DOUBLE, freedom DOUBLE) -> DOUBLE`

Student's t 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_students_t_variance(0.0, 1.0, 10.0)
-- 1.25
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

### sr_fisher_snedecor_entropy(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_entropy(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_entropy(3.0, 5.0)
-- NULL
```

### sr_fisher_snedecor_max(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_max(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_max(3.0, 5.0)
-- inf
```

### sr_fisher_snedecor_mean(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_mean(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_mean(3.0, 5.0)
-- 1.6666666666666667
```

### sr_fisher_snedecor_min(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_min(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_min(3.0, 5.0)
-- 0.0
```

### sr_fisher_snedecor_mode(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_mode(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_mode(3.0, 5.0)
-- 0.23809523809523808
```

### sr_fisher_snedecor_skewness(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_skewness(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_skewness(3.0, 7.0)
-- 11.0
```

### sr_fisher_snedecor_std_dev(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_std_dev(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_std_dev(3.0, 5.0)
-- 3.3333333333333335
```

### sr_fisher_snedecor_variance(freedom_1, freedom_2)

**签名**：`sr_fisher_snedecor_variance(freedom_1 DOUBLE, freedom_2 DOUBLE) -> DOUBLE`

Fisher-Snedecor（F 分布）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fisher_snedecor_variance(3.0, 5.0)
-- 11.11111111111111
```
