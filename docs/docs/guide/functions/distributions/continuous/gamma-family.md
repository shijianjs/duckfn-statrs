---
title: Gamma family
sidebar_position: 2
description: Gamma, inverse-gamma, chi-squared, chi, Erlang and exponential distributions — pdf, ln_pdf, cdf, sf, quantile, plus moments and support.
---

# Gamma family

The gamma, inverse-gamma, chi-squared, chi, Erlang and exponential distributions are all
related by reparameterisation of the same underlying gamma law.

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

### sr_gamma_entropy(shape, rate)

**Signature**: `sr_gamma_entropy(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma differential entropy (shape/rate parameterization).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_entropy(2.0, 1.0)
-- 1.5772156649015352
```

### sr_gamma_max(shape, rate)

**Signature**: `sr_gamma_max(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_max(2.0, 1.0)
-- inf
```

### sr_gamma_mean(shape, rate)

**Signature**: `sr_gamma_mean(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma mean (shape/rate parameterization).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_mean(2.0, 1.0)
-- 2.0
```

### sr_gamma_min(shape, rate)

**Signature**: `sr_gamma_min(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_min(2.0, 1.0)
-- 0.0
```

### sr_gamma_mode(shape, rate)

**Signature**: `sr_gamma_mode(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma mode (shape/rate parameterization).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_mode(2.0, 1.0)
-- 1.0
```

### sr_gamma_skewness(shape, rate)

**Signature**: `sr_gamma_skewness(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma skewness (shape/rate parameterization).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_skewness(2.0, 1.0)
-- 1.414213562373095
```

### sr_gamma_std_dev(shape, rate)

**Signature**: `sr_gamma_std_dev(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma standard deviation (shape/rate parameterization).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_std_dev(2.0, 1.0)
-- 1.4142135623730951
```

### sr_gamma_variance(shape, rate)

**Signature**: `sr_gamma_variance(shape DOUBLE, rate DOUBLE) -> DOUBLE`

Gamma variance (shape/rate parameterization).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_variance(2.0, 1.0)
-- 2.0
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

### sr_inverse_gamma_entropy(shape, scale)

**Signature**: `sr_inverse_gamma_entropy(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma differential entropy, given shape and scale.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_entropy(2.0, 1.0)
-- 0.7316469947046054
```

### sr_inverse_gamma_max(shape, scale)

**Signature**: `sr_inverse_gamma_max(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_max(2.0, 1.0)
-- inf
```

### sr_inverse_gamma_mean(shape, scale)

**Signature**: `sr_inverse_gamma_mean(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma mean, given shape and scale.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_mean(2.0, 1.0)
-- 1.0
```

### sr_inverse_gamma_min(shape, scale)

**Signature**: `sr_inverse_gamma_min(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_min(2.0, 1.0)
-- 0.0
```

### sr_inverse_gamma_mode(shape, scale)

**Signature**: `sr_inverse_gamma_mode(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma mode, given shape and scale.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_mode(2.0, 1.0)
-- 0.3333333333333333
```

### sr_inverse_gamma_skewness(shape, scale)

**Signature**: `sr_inverse_gamma_skewness(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma skewness, given shape and scale.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_skewness(4.0, 1.0)
-- 5.656854249492381
```

### sr_inverse_gamma_std_dev(shape, scale)

**Signature**: `sr_inverse_gamma_std_dev(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma standard deviation, given shape and scale.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_std_dev(3.0, 1.0)
-- 0.5
```

### sr_inverse_gamma_variance(shape, scale)

**Signature**: `sr_inverse_gamma_variance(shape DOUBLE, scale DOUBLE) -> DOUBLE`

Inverse-gamma variance, given shape and scale.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_variance(3.0, 1.0)
-- 0.25
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

### sr_chi_squared_entropy(freedom)

**Signature**: `sr_chi_squared_entropy(freedom DOUBLE) -> DOUBLE`

Chi-squared differential entropy, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_entropy(2.0)
-- 1.693147180559945
```

### sr_chi_squared_max(freedom)

**Signature**: `sr_chi_squared_max(freedom DOUBLE) -> DOUBLE`

Chi-squared maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_max(2.0)
-- inf
```

### sr_chi_squared_mean(freedom)

**Signature**: `sr_chi_squared_mean(freedom DOUBLE) -> DOUBLE`

Chi-squared mean, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_mean(2.0)
-- 2.0
```

### sr_chi_squared_median(freedom)

**Signature**: `sr_chi_squared_median(freedom DOUBLE) -> DOUBLE`

Chi-squared median, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_median(2.0)
-- 1.3333333333333335
```

### sr_chi_squared_min(freedom)

**Signature**: `sr_chi_squared_min(freedom DOUBLE) -> DOUBLE`

Chi-squared minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_min(2.0)
-- 0.0
```

### sr_chi_squared_mode(freedom)

**Signature**: `sr_chi_squared_mode(freedom DOUBLE) -> DOUBLE`

Chi-squared mode, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_mode(2.0)
-- 0.0
```

### sr_chi_squared_skewness(freedom)

**Signature**: `sr_chi_squared_skewness(freedom DOUBLE) -> DOUBLE`

Chi-squared skewness, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_skewness(2.0)
-- 2.0
```

### sr_chi_squared_std_dev(freedom)

**Signature**: `sr_chi_squared_std_dev(freedom DOUBLE) -> DOUBLE`

Chi-squared standard deviation, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_std_dev(2.0)
-- 2.0
```

### sr_chi_squared_variance(freedom)

**Signature**: `sr_chi_squared_variance(freedom DOUBLE) -> DOUBLE`

Chi-squared variance, given degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_variance(2.0)
-- 4.0
```

## Chi

Parameter: `freedom` (degrees of freedom, > 0). This is the sqrt of a chi-squared variable.

### sr_chi_pdf(x, freedom)

**Signature**: `sr_chi_pdf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_pdf(1.0, 2)
```

### sr_chi_ln_pdf(x, freedom)

**Signature**: `sr_chi_ln_pdf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_ln_pdf(1.0, 2)
```

### sr_chi_cdf(x, freedom)

**Signature**: `sr_chi_cdf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_cdf(1.0, 2)
```

### sr_chi_sf(x, freedom)

**Signature**: `sr_chi_sf(x DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_sf(1.0, 2)
```

### sr_chi_quantile(p, freedom)

**Signature**: `sr_chi_quantile(p DOUBLE, freedom UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_quantile(0.5, 2)
```

### sr_chi_entropy(freedom)

**Signature**: `sr_chi_entropy(freedom UBIGINT) -> DOUBLE`

Chi distribution differential entropy, given UBIGINT freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_entropy(2)
-- 0.9420342421707942
```

### sr_chi_max(freedom)

**Signature**: `sr_chi_max(freedom UBIGINT) -> DOUBLE`

Chi distribution maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_max(2)
-- inf
```

### sr_chi_mean(freedom)

**Signature**: `sr_chi_mean(freedom UBIGINT) -> DOUBLE`

Chi distribution mean, given UBIGINT freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_mean(2)
-- 1.2533141373155032
```

### sr_chi_min(freedom)

**Signature**: `sr_chi_min(freedom UBIGINT) -> DOUBLE`

Chi distribution minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_min(2)
-- 0.0
```

### sr_chi_mode(freedom)

**Signature**: `sr_chi_mode(freedom UBIGINT) -> DOUBLE`

Chi distribution mode, given UBIGINT freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_mode(2)
-- 1.0
```

### sr_chi_skewness(freedom)

**Signature**: `sr_chi_skewness(freedom UBIGINT) -> DOUBLE`

Chi distribution skewness, given UBIGINT freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_skewness(2)
-- 0.6311106578190224
```

### sr_chi_std_dev(freedom)

**Signature**: `sr_chi_std_dev(freedom UBIGINT) -> DOUBLE`

Chi distribution standard deviation, given UBIGINT freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_std_dev(2)
-- 0.6551363775620278
```

### sr_chi_variance(freedom)

**Signature**: `sr_chi_variance(freedom UBIGINT) -> DOUBLE`

Chi distribution variance, given UBIGINT freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_variance(2)
-- 0.4292036732050959
```

## Erlang

Parameters: `shape` (UBIGINT > 0), `rate` (> 0).

### sr_erlang_pdf(x, shape, rate)

**Signature**: `sr_erlang_pdf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_pdf(1.0, 2, 2.0)
```

### sr_erlang_ln_pdf(x, shape, rate)

**Signature**: `sr_erlang_ln_pdf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_ln_pdf(1.0, 2, 2.0)
```

### sr_erlang_cdf(x, shape, rate)

**Signature**: `sr_erlang_cdf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_cdf(1.0, 2, 2.0)
```

### sr_erlang_sf(x, shape, rate)

**Signature**: `sr_erlang_sf(x DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_sf(1.0, 2, 2.0)
```

### sr_erlang_quantile(p, shape, rate)

**Signature**: `sr_erlang_quantile(p DOUBLE, shape UBIGINT, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_quantile(0.5, 2, 2.0)
```

### sr_erlang_entropy(shape, rate)

**Signature**: `sr_erlang_entropy(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang differential entropy (UBIGINT shape, DOUBLE rate).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_entropy(2, 1.0)
-- 1.5772156649015352
```

### sr_erlang_max(shape, rate)

**Signature**: `sr_erlang_max(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_max(2, 1.0)
-- inf
```

### sr_erlang_mean(shape, rate)

**Signature**: `sr_erlang_mean(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang mean (UBIGINT shape, DOUBLE rate).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_mean(2, 1.0)
-- 2.0
```

### sr_erlang_min(shape, rate)

**Signature**: `sr_erlang_min(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_min(2, 1.0)
-- 0.0
```

### sr_erlang_mode(shape, rate)

**Signature**: `sr_erlang_mode(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang mode (UBIGINT shape, DOUBLE rate).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_mode(2, 1.0)
-- 1.0
```

### sr_erlang_skewness(shape, rate)

**Signature**: `sr_erlang_skewness(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang skewness (UBIGINT shape, DOUBLE rate).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_skewness(2, 1.0)
-- 1.414213562373095
```

### sr_erlang_std_dev(shape, rate)

**Signature**: `sr_erlang_std_dev(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang standard deviation (UBIGINT shape, DOUBLE rate).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_std_dev(2, 1.0)
-- 1.4142135623730951
```

### sr_erlang_variance(shape, rate)

**Signature**: `sr_erlang_variance(shape UBIGINT, rate DOUBLE) -> DOUBLE`

Erlang variance (UBIGINT shape, DOUBLE rate).

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_variance(2, 1.0)
-- 2.0
```

## Exponential

Parameter: `rate` (> 0).

### sr_exp_pdf(x, rate)

**Signature**: `sr_exp_pdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_pdf(1.0, 2.0)
-- 0.2706705664732254
```

### sr_exp_ln_pdf(x, rate)

**Signature**: `sr_exp_ln_pdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_ln_pdf(1.0, 2.0)
```

### sr_exp_cdf(x, rate)

**Signature**: `sr_exp_cdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_cdf(1.0, 2.0)
-- 0.8646647167633873
```

### sr_exp_sf(x, rate)

**Signature**: `sr_exp_sf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_sf(1.0, 2.0)
```

### sr_exp_quantile(p, rate)

**Signature**: `sr_exp_quantile(p DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_quantile(0.5, 2.0)
-- 0.34657359027997264
```

### sr_exp_entropy(rate)

**Signature**: `sr_exp_entropy(rate DOUBLE) -> DOUBLE`

Exponential differential entropy, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_entropy(2.0)
-- 0.3068528194400547
```

### sr_exp_max(rate)

**Signature**: `sr_exp_max(rate DOUBLE) -> DOUBLE`

Exponential maximum of the support (positive infinity).

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_max(2.0)
-- inf
```

### sr_exp_mean(rate)

**Signature**: `sr_exp_mean(rate DOUBLE) -> DOUBLE`

Exponential mean, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_mean(2.0)
-- 0.5
```

### sr_exp_median(rate)

**Signature**: `sr_exp_median(rate DOUBLE) -> DOUBLE`

Exponential median, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_median(2.0)
-- 0.34657359027997264
```

### sr_exp_min(rate)

**Signature**: `sr_exp_min(rate DOUBLE) -> DOUBLE`

Exponential minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_min(2.0)
-- 0.0
```

### sr_exp_mode(rate)

**Signature**: `sr_exp_mode(rate DOUBLE) -> DOUBLE`

Exponential mode, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_mode(2.0)
-- 0.0
```

### sr_exp_skewness(rate)

**Signature**: `sr_exp_skewness(rate DOUBLE) -> DOUBLE`

Exponential skewness, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_skewness(2.0)
-- 2.0
```

### sr_exp_std_dev(rate)

**Signature**: `sr_exp_std_dev(rate DOUBLE) -> DOUBLE`

Exponential standard deviation, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_std_dev(2.0)
-- 0.5
```

### sr_exp_variance(rate)

**Signature**: `sr_exp_variance(rate DOUBLE) -> DOUBLE`

Exponential variance, given the rate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_variance(2.0)
-- 0.25
```
