---
title: "连续：位置-尺度 (B)"
sidebar_position: 5
description: Laplace、Gumbel、Levy、Logistic 分布——每种都有 pdf / ln_pdf / cdf / sf / quantile。
---

# 连续分布：位置-尺度 (B)

每种分布 5 个函数：`pdf`、`ln_pdf`、`cdf`、`sf`、`quantile`。`scale` 必须 > 0；分位数的 `p` 
必须在 [0, 1]。

## Laplace

参数：`location`（任意实数）、`scale`（> 0）。

### sr_laplace_pdf(x, location, scale)

**签名**：`sr_laplace_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_pdf(0.0, 0.0, 1.0)
-- 0.5
```

### sr_laplace_ln_pdf(x, location, scale)

**签名**：`sr_laplace_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_ln_pdf(0.0, 0.0, 1.0)
-- -0.6931471805599453
```

### sr_laplace_cdf(x, location, scale)

**签名**：`sr_laplace_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_cdf(1.0, 0.0, 1.0)
```

### sr_laplace_sf(x, location, scale)

**签名**：`sr_laplace_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_sf(1.0, 0.0, 1.0)
```

### sr_laplace_quantile(p, location, scale)

**签名**：`sr_laplace_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_quantile(0.5, 0.0, 1.0)
-- 0.0
```

## Gumbel（I 型极值分布）

参数：`location`、`scale`（> 0）。

### sr_gumbel_pdf(x, location, scale)

**签名**：`sr_gumbel_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_pdf(0.0, 0.0, 1.0)
-- 0.3678794411714424
```

### sr_gumbel_ln_pdf(x, location, scale)

**签名**：`sr_gumbel_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_ln_pdf(0.0, 0.0, 1.0)
```

### sr_gumbel_cdf(x, location, scale)

**签名**：`sr_gumbel_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_cdf(1.0, 0.0, 1.0)
```

### sr_gumbel_sf(x, location, scale)

**签名**：`sr_gumbel_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_sf(1.0, 0.0, 1.0)
```

### sr_gumbel_quantile(p, location, scale)

**签名**：`sr_gumbel_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_quantile(0.5, 0.0, 1.0)
```

## Levy

参数：`mu`（位置）、`c`（尺度，> 0）。

### sr_levy_pdf(x, mu, c)

**签名**：`sr_levy_pdf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_pdf(1.0, 0.0, 1.0)
```

### sr_levy_ln_pdf(x, mu, c)

**签名**：`sr_levy_ln_pdf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_ln_pdf(1.0, 0.0, 1.0)
```

### sr_levy_cdf(x, mu, c)

**签名**：`sr_levy_cdf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_cdf(1.0, 0.0, 1.0)
```

### sr_levy_sf(x, mu, c)

**签名**：`sr_levy_sf(x DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_sf(1.0, 0.0, 1.0)
```

### sr_levy_quantile(p, mu, c)

**签名**：`sr_levy_quantile(p DOUBLE, mu DOUBLE, c DOUBLE) -> DOUBLE`

statrs 用二分法数值求解，精度低于闭式解的分布。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_quantile(0.5, 0.0, 1.0)
```

## Logistic（分布）

参数：`location`、`scale`（> 0）。

### sr_logistic_pdf(x, location, scale)

注册的 SQL 名是 `sr_logistic_pdf` —— 与特殊函数页的 `sr_logistic(p)`（sigmoid 函数）不同。

**签名**：`sr_logistic_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_pdf(0.0, 0.0, 1.0)
-- 0.25
```

### sr_logistic_ln_pdf(x, location, scale)

**签名**：`sr_logistic_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_ln_pdf(0.0, 0.0, 1.0)
```

### sr_logistic_cdf(x, location, scale)

**签名**：`sr_logistic_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_cdf(1.0, 0.0, 1.0)
```

### sr_logistic_sf(x, location, scale)

**签名**：`sr_logistic_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_sf(1.0, 0.0, 1.0)
```

### sr_logistic_quantile(p, location, scale)

**签名**：`sr_logistic_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic_quantile(0.5, 0.0, 1.0)
-- 0.0
```
