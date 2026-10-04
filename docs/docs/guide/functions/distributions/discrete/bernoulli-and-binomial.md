---
title: Bernoulli and binomial trials
sidebar_position: 1
description: Bernoulli, binomial, negative binomial and geometric — pmf, ln_pmf, cdf, sf, quantile, plus moments and support.
---

# Bernoulli and binomial trials

These four distributions all model independent success/failure trials. Integer-valued slots
(`x`, trial counts, success counts) take **whole-number DOUBLE** literals like `10.0`, not `10`
or `3.5` — a non-integer is a query error, never a silent round.

## Bernoulli

Parameters: `p` — success probability, must be in [0, 1]. Support: `{0, 1}`.

### sr_bernoulli_pmf(x, p)

**Signature**: `sr_bernoulli_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_pmf(1.0, 0.7)
-- 0.7
```

### sr_bernoulli_ln_pmf(x, p)

**Signature**: `sr_bernoulli_ln_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_ln_pmf(1.0, 0.7)
```

### sr_bernoulli_cdf(x, p)

**Signature**: `sr_bernoulli_cdf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_cdf(0.0, 0.7)
-- 0.30000000000000004
```

### sr_bernoulli_sf(x, p)

**Signature**: `sr_bernoulli_sf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_sf(0.0, 0.7)
-- 0.7
```

### sr_bernoulli_quantile(p, prob)

**Signature**: `sr_bernoulli_quantile(p DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_quantile(0.5, 0.7)
-- 1.0
```

### sr_bernoulli_entropy(p)

**Signature**: `sr_bernoulli_entropy(p DOUBLE) -> DOUBLE`

Bernoulli entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_entropy(0.7)
-- 0.6108643020548935
```

### sr_bernoulli_max(p)

**Signature**: `sr_bernoulli_max(p DOUBLE) -> DOUBLE`

Bernoulli maximum of the support (1).

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_max(0.7)
-- 1.0
```

### sr_bernoulli_mean(p)

**Signature**: `sr_bernoulli_mean(p DOUBLE) -> DOUBLE`

Bernoulli mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_mean(0.7)
-- 0.7
```

### sr_bernoulli_median(p)

**Signature**: `sr_bernoulli_median(p DOUBLE) -> DOUBLE`

Bernoulli median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_median(0.7)
-- 0.0
```

### sr_bernoulli_min(p)

**Signature**: `sr_bernoulli_min(p DOUBLE) -> DOUBLE`

Bernoulli minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_min(0.7)
-- 0.0
```

### sr_bernoulli_mode(p)

**Signature**: `sr_bernoulli_mode(p DOUBLE) -> DOUBLE`

Bernoulli mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_mode(0.7)
-- 1.0
```

### sr_bernoulli_skewness(p)

**Signature**: `sr_bernoulli_skewness(p DOUBLE) -> DOUBLE`

Bernoulli skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_skewness(0.7)
-- -0.8728715609439692
```

### sr_bernoulli_std_dev(p)

**Signature**: `sr_bernoulli_std_dev(p DOUBLE) -> DOUBLE`

Bernoulli standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_std_dev(0.7)
-- 0.45825756949558405
```

### sr_bernoulli_variance(p)

**Signature**: `sr_bernoulli_variance(p DOUBLE) -> DOUBLE`

Bernoulli variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_variance(0.7)
-- 0.21000000000000002
```

## Binomial

Parameters: `p` (success probability), `n` (number of trials, whole-number DOUBLE).

### sr_binomial_pmf(x, p, n)

**Signature**: `sr_binomial_pmf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

Probability mass `P(X = x)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_pmf(3.0, 0.5, 10.0)::DECIMAL(12,8)
-- 0.11718750
```

### sr_binomial_ln_pmf(x, p, n)

**Signature**: `sr_binomial_ln_pmf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_ln_pmf(3.0, 0.5, 10.0)
```

### sr_binomial_cdf(x, p, n)

**Signature**: `sr_binomial_cdf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_cdf(3.0, 0.5, 10.0)
```

### sr_binomial_sf(x, p, n)

**Signature**: `sr_binomial_sf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_sf(3.0, 0.5, 10.0)
```

### sr_binomial_quantile(p, prob, n)

**Signature**: `sr_binomial_quantile(p DOUBLE, prob DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_quantile(0.5, 0.5, 10.0)
-- 5.0
```

### sr_binomial_entropy(p, n)

**Signature**: `sr_binomial_entropy(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_entropy(0.5, 10.0)
-- 1.8759536052468009
```

### sr_binomial_max(p, n)

**Signature**: `sr_binomial_max(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial maximum of the support (n).

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_max(0.5, 10.0)
-- 10.0
```

### sr_binomial_mean(p, n)

**Signature**: `sr_binomial_mean(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_mean(0.5, 10.0)
-- 5.0
```

### sr_binomial_median(p, n)

**Signature**: `sr_binomial_median(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_median(0.5, 10.0)
-- 5.0
```

### sr_binomial_min(p, n)

**Signature**: `sr_binomial_min(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_min(0.5, 10.0)
-- 0.0
```

### sr_binomial_mode(p, n)

**Signature**: `sr_binomial_mode(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_mode(0.5, 10.0)
-- 5.0
```

### sr_binomial_skewness(p, n)

**Signature**: `sr_binomial_skewness(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_skewness(0.5, 10.0)
-- 0.0
```

### sr_binomial_std_dev(p, n)

**Signature**: `sr_binomial_std_dev(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_std_dev(0.5, 10.0)
-- 1.5811388300841898
```

### sr_binomial_variance(p, n)

**Signature**: `sr_binomial_variance(p DOUBLE, n DOUBLE) -> DOUBLE`

Binomial variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_variance(0.5, 10.0)
-- 2.5
```

## Negative binomial

Parameters: `r` (number of successes; real-valued in statrs), `p` (success probability).
Support: number of failures before the r-th success.

### sr_negative_binomial_pmf(x, r, p)

**Signature**: `sr_negative_binomial_pmf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_pmf(3.0, 2.0, 0.5)::DECIMAL(12,8)
-- 0.12500000
```

### sr_negative_binomial_ln_pmf(x, r, p)

**Signature**: `sr_negative_binomial_ln_pmf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_ln_pmf(3.0, 2.0, 0.5)
```

### sr_negative_binomial_cdf(x, r, p)

**Signature**: `sr_negative_binomial_cdf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_cdf(3.0, 2.0, 0.5)
```

### sr_negative_binomial_sf(x, r, p)

**Signature**: `sr_negative_binomial_sf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_sf(3.0, 2.0, 0.5)
```

### sr_negative_binomial_quantile(p, r, prob)

**Signature**: `sr_negative_binomial_quantile(p DOUBLE, r DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_quantile(0.5, 2.0, 0.5)
```

### sr_negative_binomial_entropy(r, p)

**Signature**: `sr_negative_binomial_entropy(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_entropy(2.0, 0.5)
-- NULL
```

### sr_negative_binomial_max(r, p)

**Signature**: `sr_negative_binomial_max(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial maximum of the support (u64::MAX, shown as 1.8e19).

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_max(2.0, 0.5)
-- 1.8446744073709552e+19
```

### sr_negative_binomial_mean(r, p)

**Signature**: `sr_negative_binomial_mean(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_mean(2.0, 0.5)
-- 2.0
```

### sr_negative_binomial_min(r, p)

**Signature**: `sr_negative_binomial_min(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_min(2.0, 0.5)
-- 0.0
```

### sr_negative_binomial_mode(r, p)

**Signature**: `sr_negative_binomial_mode(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial mode (statrs returns a real-valued `Option<f64>`).

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_mode(2.0, 0.5)
-- 1.0
```

### sr_negative_binomial_skewness(r, p)

**Signature**: `sr_negative_binomial_skewness(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_skewness(2.0, 0.5)
-- 1.5
```

### sr_negative_binomial_std_dev(r, p)

**Signature**: `sr_negative_binomial_std_dev(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_std_dev(2.0, 0.5)
-- 2.0
```

### sr_negative_binomial_variance(r, p)

**Signature**: `sr_negative_binomial_variance(r DOUBLE, p DOUBLE) -> DOUBLE`

Negative-binomial variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_variance(2.0, 0.5)
-- 4.0
```

## Geometric

Parameter: `p` — success probability. Support starts at 1 (statrs' convention: number of
trials until the first success).

### sr_geometric_pmf(x, p)

**Signature**: `sr_geometric_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_pmf(2.0, 0.5)
-- 0.25
```

### sr_geometric_ln_pmf(x, p)

**Signature**: `sr_geometric_ln_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_ln_pmf(2.0, 0.5)
```

### sr_geometric_cdf(x, p)

**Signature**: `sr_geometric_cdf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_cdf(1.0, 0.5)
-- 0.5
```

### sr_geometric_sf(x, p)

**Signature**: `sr_geometric_sf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_sf(1.0, 0.5)::DECIMAL(12,8)
-- 0.50000000
```

### sr_geometric_quantile(p, prob)

**Signature**: `sr_geometric_quantile(p DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_quantile(0.5, 0.5)
```

### sr_geometric_dist_mean(p)

**Signature**: `sr_geometric_dist_mean(p DOUBLE) -> DOUBLE`

Mean of the Geometric distribution, 1/p (named _dist to avoid clashing with the sr_geometric_mean aggregate over a column).

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_dist_mean(0.5)
-- 2.0
```

### sr_geometric_entropy(p)

**Signature**: `sr_geometric_entropy(p DOUBLE) -> DOUBLE`

Geometric entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_entropy(0.5)
-- 1.3862943611198906
```

### sr_geometric_max(p)

**Signature**: `sr_geometric_max(p DOUBLE) -> DOUBLE`

Geometric maximum of the support (u64::MAX, shown as 1.8e19).

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_max(0.5)
-- 1.8446744073709552e+19
```

### sr_geometric_median(p)

**Signature**: `sr_geometric_median(p DOUBLE) -> DOUBLE`

Geometric median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_median(0.5)
-- 1.0
```

### sr_geometric_min(p)

**Signature**: `sr_geometric_min(p DOUBLE) -> DOUBLE`

Geometric minimum of the support (1).

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_min(0.5)
-- 1.0
```

### sr_geometric_mode(p)

**Signature**: `sr_geometric_mode(p DOUBLE) -> DOUBLE`

Geometric mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_mode(0.5)
-- 1.0
```

### sr_geometric_skewness(p)

**Signature**: `sr_geometric_skewness(p DOUBLE) -> DOUBLE`

Geometric skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_skewness(0.5)
-- 2.1213203435596424
```

### sr_geometric_std_dev(p)

**Signature**: `sr_geometric_std_dev(p DOUBLE) -> DOUBLE`

Geometric standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_std_dev(0.5)
-- 1.4142135623730951
```

### sr_geometric_variance(p)

**Signature**: `sr_geometric_variance(p DOUBLE) -> DOUBLE`

Geometric variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_variance(0.5)
-- 2.0
```

## Errors and NULL

```sql {"type":"duckfn","expect":"error"}
SELECT sr_binomial_pmf(2.5, 0.5, 10.0)
-- error: expected a non-negative whole number, got 2.5
```

```sql {"type":"duckfn","show":"value"}
-- NULL input is short-circuited to NULL:
SELECT sr_binomial_pmf(NULL, 0.5, 10.0)
-- NULL
```
