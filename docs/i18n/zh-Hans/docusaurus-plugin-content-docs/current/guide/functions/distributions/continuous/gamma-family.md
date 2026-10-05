---
title: Gamma 族
sidebar_position: 2
description: Gamma、逆 Gamma、卡方、Chi、Erlang 与指数分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile，以及各阶矩与支撑。
---

# Gamma 族

Gamma、逆 Gamma、卡方、Chi、Erlang 与指数分布都通过重参数化关联到同一个 Gamma law。

## Gamma

参数：`shape`（> 0）、`rate`（> 0）。形状 / 率参数化。

### sr_gamma_pdf(x, shape, rate)

**签名**：`sr_gamma_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_pdf(1.0, 2.0, 2.0)
-- 0.2706705664732254
```

### sr_gamma_ln_pdf(x, shape, rate)

**签名**：`sr_gamma_ln_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_ln_pdf(1.0, 2.0, 2.0)
```

### sr_gamma_cdf(x, shape, rate)

**签名**：`sr_gamma_cdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_cdf(1.0, 2.0, 2.0)
```

### sr_gamma_sf(x, shape, rate)

**签名**：`sr_gamma_sf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_sf(1.0, 2.0, 2.0)
```

### sr_gamma_quantile(p, shape, rate)

**签名**：`sr_gamma_quantile(p DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_quantile(0.5, 2.0, 2.0)
```

### sr_gamma_entropy(shape, rate)

**签名**：`sr_gamma_entropy(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_entropy(2.0, 1.0)
-- 1.5772156649015352
```

### sr_gamma_max(shape, rate)

**签名**：`sr_gamma_max(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_max(2.0, 1.0)
-- inf
```

### sr_gamma_mean(shape, rate)

**签名**：`sr_gamma_mean(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_mean(2.0, 1.0)
-- 2.0
```

### sr_gamma_min(shape, rate)

**签名**：`sr_gamma_min(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_min(2.0, 1.0)
-- 0.0
```

### sr_gamma_mode(shape, rate)

**签名**：`sr_gamma_mode(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_mode(2.0, 1.0)
-- 1.0
```

### sr_gamma_skewness(shape, rate)

**签名**：`sr_gamma_skewness(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_skewness(2.0, 1.0)
-- 1.414213562373095
```

### sr_gamma_std_dev(shape, rate)

**签名**：`sr_gamma_std_dev(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_std_dev(2.0, 1.0)
-- 1.4142135623730951
```

### sr_gamma_variance(shape, rate)

**签名**：`sr_gamma_variance(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_variance(2.0, 1.0)
-- 2.0
```

## 逆 Gamma

参数：`shape`（> 0）、`scale`（> 0）。

### sr_inverse_gamma_pdf(x, shape, scale)

**签名**：`sr_inverse_gamma_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_pdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_ln_pdf(x, shape, scale)

**签名**：`sr_inverse_gamma_ln_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_ln_pdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_cdf(x, shape, scale)

**签名**：`sr_inverse_gamma_cdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_cdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_sf(x, shape, scale)

**签名**：`sr_inverse_gamma_sf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_sf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_quantile(p, shape, scale)

**签名**：`sr_inverse_gamma_quantile(p DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_quantile(0.5, 2.0, 2.0)
```

### sr_inverse_gamma_entropy(shape, scale)

**签名**：`sr_inverse_gamma_entropy(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_entropy(2.0, 1.0)
-- 0.7316469947046054
```

### sr_inverse_gamma_max(shape, scale)

**签名**：`sr_inverse_gamma_max(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_max(2.0, 1.0)
-- inf
```

### sr_inverse_gamma_mean(shape, scale)

**签名**：`sr_inverse_gamma_mean(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_mean(2.0, 1.0)
-- 1.0
```

### sr_inverse_gamma_min(shape, scale)

**签名**：`sr_inverse_gamma_min(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_min(2.0, 1.0)
-- 0.0
```

### sr_inverse_gamma_mode(shape, scale)

**签名**：`sr_inverse_gamma_mode(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_mode(2.0, 1.0)
-- 0.3333333333333333
```

### sr_inverse_gamma_skewness(shape, scale)

**签名**：`sr_inverse_gamma_skewness(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_skewness(4.0, 1.0)
-- 5.656854249492381
```

### sr_inverse_gamma_std_dev(shape, scale)

**签名**：`sr_inverse_gamma_std_dev(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_std_dev(3.0, 1.0)
-- 0.5
```

### sr_inverse_gamma_variance(shape, scale)

**签名**：`sr_inverse_gamma_variance(shape DOUBLE, scale DOUBLE) -> DOUBLE`

逆 Gamma 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_variance(3.0, 1.0)
-- 0.25
```

## 卡方（Chi-squared）

参数：`freedom`（自由度，> 0）。

### sr_chi_squared_pdf(x, freedom)

**签名**：`sr_chi_squared_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_pdf(1.0, 2.0)
```

### sr_chi_squared_ln_pdf(x, freedom)

**签名**：`sr_chi_squared_ln_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_ln_pdf(1.0, 2.0)
```

### sr_chi_squared_cdf(x, freedom)

**签名**：`sr_chi_squared_cdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_cdf(1.0, 2.0)
```

### sr_chi_squared_sf(x, freedom)

**签名**：`sr_chi_squared_sf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_sf(1.0, 2.0)
```

### sr_chi_squared_quantile(p, freedom)

**签名**：`sr_chi_squared_quantile(p DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_quantile(0.95, 2.0)
```

### sr_chi_squared_entropy(freedom)

**签名**：`sr_chi_squared_entropy(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_entropy(2.0)
-- 1.693147180559945
```

### sr_chi_squared_max(freedom)

**签名**：`sr_chi_squared_max(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_max(2.0)
-- inf
```

### sr_chi_squared_mean(freedom)

**签名**：`sr_chi_squared_mean(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_mean(2.0)
-- 2.0
```

### sr_chi_squared_median(freedom)

**签名**：`sr_chi_squared_median(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_median(2.0)
-- 1.3333333333333335
```

### sr_chi_squared_min(freedom)

**签名**：`sr_chi_squared_min(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_min(2.0)
-- 0.0
```

### sr_chi_squared_mode(freedom)

**签名**：`sr_chi_squared_mode(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_mode(2.0)
-- 0.0
```

### sr_chi_squared_skewness(freedom)

**签名**：`sr_chi_squared_skewness(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_skewness(2.0)
-- 2.0
```

### sr_chi_squared_std_dev(freedom)

**签名**：`sr_chi_squared_std_dev(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_std_dev(2.0)
-- 2.0
```

### sr_chi_squared_variance(freedom)

**签名**：`sr_chi_squared_variance(freedom DOUBLE) -> DOUBLE`

卡方（Chi-squared）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_variance(2.0)
-- 4.0
```

## Chi（卡方根）

参数：`freedom`（自由度，> 0）。开根号的卡方变量。

### sr_chi_pdf(x, freedom)

**签名**：`sr_chi_pdf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_pdf(1.0, 2)
```

### sr_chi_ln_pdf(x, freedom)

**签名**：`sr_chi_ln_pdf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_ln_pdf(1.0, 2)
```

### sr_chi_cdf(x, freedom)

**签名**：`sr_chi_cdf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_cdf(1.0, 2)
```

### sr_chi_sf(x, freedom)

**签名**：`sr_chi_sf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_sf(1.0, 2)
```

### sr_chi_quantile(p, freedom)

**签名**：`sr_chi_quantile(p DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_quantile(0.5, 2)
```

### sr_chi_entropy(freedom)

**签名**：`sr_chi_entropy(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_entropy(2)
-- 0.9420342421707942
```

### sr_chi_max(freedom)

**签名**：`sr_chi_max(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_max(2)
-- inf
```

### sr_chi_mean(freedom)

**签名**：`sr_chi_mean(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_mean(2)
-- 1.2533141373155032
```

### sr_chi_min(freedom)

**签名**：`sr_chi_min(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_min(2)
-- 0.0
```

### sr_chi_mode(freedom)

**签名**：`sr_chi_mode(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_mode(2)
-- 1.0
```

### sr_chi_skewness(freedom)

**签名**：`sr_chi_skewness(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_skewness(2)
-- 0.6311106578190224
```

### sr_chi_std_dev(freedom)

**签名**：`sr_chi_std_dev(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_std_dev(2)
-- 0.6551363775620278
```

### sr_chi_variance(freedom)

**签名**：`sr_chi_variance(freedom UBIGINT) -> DOUBLE`

Chi（卡方根）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_variance(2)
-- 0.4292036732050959
```

## Erlang

参数：`shape`（UBIGINT > 0）、`rate`（> 0）。

### sr_erlang_pdf(x, shape, rate)

**签名**：`sr_erlang_pdf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_pdf(1.0, 2, 2.0)
```

### sr_erlang_ln_pdf(x, shape, rate)

**签名**：`sr_erlang_ln_pdf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_ln_pdf(1.0, 2, 2.0)
```

### sr_erlang_cdf(x, shape, rate)

**签名**：`sr_erlang_cdf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_cdf(1.0, 2, 2.0)
```

### sr_erlang_sf(x, shape, rate)

**签名**：`sr_erlang_sf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_sf(1.0, 2, 2.0)
```

### sr_erlang_quantile(p, shape, rate)

**签名**：`sr_erlang_quantile(p DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_quantile(0.5, 2, 2.0)
```

### sr_erlang_entropy(shape, rate)

**签名**：`sr_erlang_entropy(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_entropy(2, 1.0)
-- 1.5772156649015352
```

### sr_erlang_max(shape, rate)

**签名**：`sr_erlang_max(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_max(2, 1.0)
-- inf
```

### sr_erlang_mean(shape, rate)

**签名**：`sr_erlang_mean(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_mean(2, 1.0)
-- 2.0
```

### sr_erlang_min(shape, rate)

**签名**：`sr_erlang_min(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_min(2, 1.0)
-- 0.0
```

### sr_erlang_mode(shape, rate)

**签名**：`sr_erlang_mode(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_mode(2, 1.0)
-- 1.0
```

### sr_erlang_skewness(shape, rate)

**签名**：`sr_erlang_skewness(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_skewness(2, 1.0)
-- 1.414213562373095
```

### sr_erlang_std_dev(shape, rate)

**签名**：`sr_erlang_std_dev(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_std_dev(2, 1.0)
-- 1.4142135623730951
```

### sr_erlang_variance(shape, rate)

**签名**：`sr_erlang_variance(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_variance(2, 1.0)
-- 2.0
```

## 指数

参数：`rate`（> 0）。

### sr_exp_pdf(x, rate)

**签名**：`sr_exp_pdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_pdf(1.0, 2.0)
-- 0.2706705664732254
```

### sr_exp_ln_pdf(x, rate)

**签名**：`sr_exp_ln_pdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_ln_pdf(1.0, 2.0)
```

### sr_exp_cdf(x, rate)

**签名**：`sr_exp_cdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_cdf(1.0, 2.0)
-- 0.8646647167633873
```

### sr_exp_sf(x, rate)

**签名**：`sr_exp_sf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_sf(1.0, 2.0)
```

### sr_exp_quantile(p, rate)

**签名**：`sr_exp_quantile(p DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_quantile(0.5, 2.0)
-- 0.34657359027997264
```

### sr_exp_entropy(rate)

**签名**：`sr_exp_entropy(rate DOUBLE) -> DOUBLE`

指数的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_entropy(2.0)
-- 0.3068528194400547
```

### sr_exp_max(rate)

**签名**：`sr_exp_max(rate DOUBLE) -> DOUBLE`

指数的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_max(2.0)
-- inf
```

### sr_exp_mean(rate)

**签名**：`sr_exp_mean(rate DOUBLE) -> DOUBLE`

指数的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_mean(2.0)
-- 0.5
```

### sr_exp_median(rate)

**签名**：`sr_exp_median(rate DOUBLE) -> DOUBLE`

指数的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_median(2.0)
-- 0.34657359027997264
```

### sr_exp_min(rate)

**签名**：`sr_exp_min(rate DOUBLE) -> DOUBLE`

指数的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_min(2.0)
-- 0.0
```

### sr_exp_mode(rate)

**签名**：`sr_exp_mode(rate DOUBLE) -> DOUBLE`

指数的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_mode(2.0)
-- 0.0
```

### sr_exp_skewness(rate)

**签名**：`sr_exp_skewness(rate DOUBLE) -> DOUBLE`

指数的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_skewness(2.0)
-- 2.0
```

### sr_exp_std_dev(rate)

**签名**：`sr_exp_std_dev(rate DOUBLE) -> DOUBLE`

指数的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_std_dev(2.0)
-- 0.5
```

### sr_exp_variance(rate)

**签名**：`sr_exp_variance(rate DOUBLE) -> DOUBLE`

指数的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_variance(2.0)
-- 0.25
```
