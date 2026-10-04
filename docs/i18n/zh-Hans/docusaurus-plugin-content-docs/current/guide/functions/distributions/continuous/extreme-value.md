---
title: 极值与重尾
sidebar_position: 5
description: Gumbel、Weibull、Levy 与 Pareto 分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile，以及各阶矩与支撑。
---

# 极值与重尾

Gumbel 与 Weibull 是极值分布（有界 / 无界尾部）；Levy 与 Pareto 是重尾分布。

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

### sr_gumbel_entropy(location, scale)

**签名**：`sr_gumbel_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_entropy(0.0, 1.0)
-- 1.5772156649015328
```

### sr_gumbel_max(location, scale)

**签名**：`sr_gumbel_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_max(0.0, 1.0)
-- inf
```

### sr_gumbel_mean(location, scale)

**签名**：`sr_gumbel_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_mean(0.0, 1.0)
-- 0.5772156649015329
```

### sr_gumbel_median(location, scale)

**签名**：`sr_gumbel_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_median(0.0, 1.0)
-- 0.36651292058166435
```

### sr_gumbel_min(location, scale)

**签名**：`sr_gumbel_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_min(0.0, 1.0)
-- -inf
```

### sr_gumbel_mode(location, scale)

**签名**：`sr_gumbel_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_mode(0.0, 1.0)
-- 0.0
```

### sr_gumbel_skewness(location, scale)

**签名**：`sr_gumbel_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_skewness(0.0, 1.0)
-- 1.13955
```

### sr_gumbel_std_dev(location, scale)

**签名**：`sr_gumbel_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_std_dev(0.0, 1.0)
-- 1.282549830161864
```

### sr_gumbel_variance(location, scale)

**签名**：`sr_gumbel_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

Gumbel（I 型极值分布）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gumbel_variance(0.0, 1.0)
-- 1.6449340668482264
```

## Weibull

参数：`shape`（> 0）、`scale`（> 0）。

### sr_weibull_pdf(x, shape, scale)

**签名**：`sr_weibull_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_pdf(1.0, 1.0, 1.0)
-- 1.0
```

### sr_weibull_ln_pdf(x, shape, scale)

**签名**：`sr_weibull_ln_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_ln_pdf(1.0, 1.0, 1.0)
```

### sr_weibull_cdf(x, shape, scale)

**签名**：`sr_weibull_cdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_cdf(1.0, 1.0, 1.0)
-- 0.6321205588285577
```

### sr_weibull_sf(x, shape, scale)

**签名**：`sr_weibull_sf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_sf(1.0, 1.0, 1.0)
```

### sr_weibull_quantile(p, shape, scale)

**签名**：`sr_weibull_quantile(p DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_quantile(0.5, 1.0, 1.0)
```

### sr_weibull_entropy(shape, scale)

**签名**：`sr_weibull_entropy(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_entropy(1.0, 1.0)
-- 1.0
```

### sr_weibull_max(shape, scale)

**签名**：`sr_weibull_max(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_max(1.0, 1.0)
-- inf
```

### sr_weibull_mean(shape, scale)

**签名**：`sr_weibull_mean(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_mean(1.0, 1.0)
-- 1.0
```

### sr_weibull_median(shape, scale)

**签名**：`sr_weibull_median(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_median(1.0, 1.0)
-- 0.6931471805599453
```

### sr_weibull_min(shape, scale)

**签名**：`sr_weibull_min(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_min(1.0, 1.0)
-- 0.0
```

### sr_weibull_mode(shape, scale)

**签名**：`sr_weibull_mode(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_mode(2.0, 1.0)
-- 0.7071067811865476
```

### sr_weibull_skewness(shape, scale)

**签名**：`sr_weibull_skewness(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_skewness(1.0, 1.0)
-- 1.9999999999999476
```

### sr_weibull_std_dev(shape, scale)

**签名**：`sr_weibull_std_dev(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_std_dev(1.0, 1.0)
-- 1.0000000000000033
```

### sr_weibull_variance(shape, scale)

**签名**：`sr_weibull_variance(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Weibull 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_weibull_variance(1.0, 1.0)
-- 1.0000000000000067
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

### sr_levy_entropy(mu, c)

**签名**：`sr_levy_entropy(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_entropy(0.0, 1.0)
-- 3.32448280139689
```

### sr_levy_max(mu, c)

**签名**：`sr_levy_max(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_max(0.0, 1.0)
-- inf
```

### sr_levy_mean(mu, c)

**签名**：`sr_levy_mean(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_mean(0.0, 1.0)
-- inf
```

### sr_levy_median(mu, c)

**签名**：`sr_levy_median(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_median(0.0, 1.0)
-- 2.1981093383177326
```

### sr_levy_min(mu, c)

**签名**：`sr_levy_min(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_min(0.0, 1.0)
-- 0.0
```

### sr_levy_mode(mu, c)

**签名**：`sr_levy_mode(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_mode(0.0, 1.0)
-- 0.3333333333333333
```

### sr_levy_skewness(mu, c)

**签名**：`sr_levy_skewness(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_skewness(0.0, 1.0)
-- NULL
```

### sr_levy_std_dev(mu, c)

**签名**：`sr_levy_std_dev(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_std_dev(0.0, 1.0)
-- inf
```

### sr_levy_variance(mu, c)

**签名**：`sr_levy_variance(mu DOUBLE, c DOUBLE) -> DOUBLE`

Levy 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_levy_variance(0.0, 1.0)
-- inf
```

## Pareto（I 型）

参数：`scale`（x_m，> 0）、`shape`（alpha，> 0）。支撑：`[x_m, infinity)`。

### sr_pareto_pdf(x, scale, shape)

**签名**：`sr_pareto_pdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_pdf(1.0, 1.0, 2.0)
-- 2.0
```

### sr_pareto_ln_pdf(x, scale, shape)

**签名**：`sr_pareto_ln_pdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_ln_pdf(1.0, 1.0, 2.0)
```

### sr_pareto_cdf(x, scale, shape)

**签名**：`sr_pareto_cdf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_cdf(2.0, 1.0, 2.0)
```

### sr_pareto_sf(x, scale, shape)

**签名**：`sr_pareto_sf(x DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_sf(2.0, 1.0, 2.0)
```

### sr_pareto_quantile(p, scale, shape)

**签名**：`sr_pareto_quantile(p DOUBLE, scale DOUBLE, shape DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_quantile(0.5, 1.0, 2.0)
```

### sr_pareto_entropy(scale, shape)

**签名**：`sr_pareto_entropy(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_entropy(1.0, 2.0)
-- 0.8068528194400547
```

### sr_pareto_max(scale, shape)

**签名**：`sr_pareto_max(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_max(1.0, 2.0)
-- inf
```

### sr_pareto_mean(scale, shape)

**签名**：`sr_pareto_mean(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_mean(1.0, 2.0)
-- 2.0
```

### sr_pareto_median(scale, shape)

**签名**：`sr_pareto_median(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_median(1.0, 2.0)
-- 1.4142135623730951
```

### sr_pareto_min(scale, shape)

**签名**：`sr_pareto_min(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_min(1.0, 2.0)
-- 1.0
```

### sr_pareto_mode(scale, shape)

**签名**：`sr_pareto_mode(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_mode(1.0, 2.0)
-- 1.0
```

### sr_pareto_skewness(scale, shape)

**签名**：`sr_pareto_skewness(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_skewness(1.0, 4.0)
-- 7.0710678118654755
```

### sr_pareto_std_dev(scale, shape)

**签名**：`sr_pareto_std_dev(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_std_dev(1.0, 3.0)
-- 0.8660254037844386
```

### sr_pareto_variance(scale, shape)

**签名**：`sr_pareto_variance(scale DOUBLE, shape DOUBLE) -> DOUBLE`

Pareto（I 型）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_pareto_variance(1.0, 3.0)
-- 0.75
```
