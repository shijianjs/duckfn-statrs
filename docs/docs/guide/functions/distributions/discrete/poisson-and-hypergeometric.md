---
title: Poisson and hypergeometric
sidebar_position: 2
description: Poisson and hypergeometric distributions — pmf, ln_pmf, cdf, sf, quantile, plus moments and support.
---

# Poisson and hypergeometric

Integer-valued slots are UBIGINT/BIGINT arguments; a non-integer literal has no matching signature.

## Poisson

Parameter: `lambda` (rate, > 0).

### sr_poisson_pmf(x, lambda)

**Signature**: `sr_poisson_pmf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(2, 3.0)
-- 0.22404180765538775
```

### sr_poisson_ln_pmf(x, lambda)

**Signature**: `sr_poisson_ln_pmf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT exp(sr_poisson_ln_pmf(2, 3.0))
-- 0.22404180765538775
```

### sr_poisson_cdf(x, lambda)

**Signature**: `sr_poisson_cdf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_cdf(2, 3.0)
```

### sr_poisson_sf(x, lambda)

**Signature**: `sr_poisson_sf(x UBIGINT, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_sf(2, 3.0)
```

### sr_poisson_quantile(p, lambda)

**Signature**: `sr_poisson_quantile(p DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_quantile(0.5, 3.0)
```

Out-of-range probability:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_poisson_quantile(-0.1, 3.0)
-- error: sr_poisson_quantile: the probability must be within [0, 1], got -0.1
```

### sr_poisson_entropy(lambda)

**Signature**: `sr_poisson_entropy(lambda DOUBLE) -> DOUBLE`

Poisson entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_entropy(3.0)
-- 1.9338825376210322
```

### sr_poisson_max(lambda)

**Signature**: `sr_poisson_max(lambda DOUBLE) -> DOUBLE`

Poisson maximum of the support (u64::MAX, shown as 1.8e19).

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_max(3.0)
-- 1.8446744073709552e+19
```

### sr_poisson_mean(lambda)

**Signature**: `sr_poisson_mean(lambda DOUBLE) -> DOUBLE`

Poisson mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_mean(3.0)
-- 3.0
```

### sr_poisson_median(lambda)

**Signature**: `sr_poisson_median(lambda DOUBLE) -> DOUBLE`

Poisson median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_median(3.0)
-- 3.0
```

### sr_poisson_min(lambda)

**Signature**: `sr_poisson_min(lambda DOUBLE) -> DOUBLE`

Poisson minimum of the support (0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_min(3.0)
-- 0.0
```

### sr_poisson_mode(lambda)

**Signature**: `sr_poisson_mode(lambda DOUBLE) -> DOUBLE`

Poisson mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_mode(3.0)
-- 3.0
```

### sr_poisson_skewness(lambda)

**Signature**: `sr_poisson_skewness(lambda DOUBLE) -> DOUBLE`

Poisson skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_skewness(3.0)
-- 0.5773502691896258
```

### sr_poisson_std_dev(lambda)

**Signature**: `sr_poisson_std_dev(lambda DOUBLE) -> DOUBLE`

Poisson standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_std_dev(3.0)
-- 1.7320508075688772
```

### sr_poisson_variance(lambda)

**Signature**: `sr_poisson_variance(lambda DOUBLE) -> DOUBLE`

Poisson variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_variance(3.0)
-- 3.0
```

## Hypergeometric

Parameters: `population` (N, UBIGINT), `successes` (K, UBIGINT),
`draws` (n, UBIGINT). Represents drawing n items without replacement from a
population of size N with K successes.

### sr_hypergeometric_pmf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_pmf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_pmf(2, 10, 5, 4)
-- 0.47619047619047616
```

### sr_hypergeometric_ln_pmf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_ln_pmf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_ln_pmf(2, 10, 5, 4)
```

### sr_hypergeometric_cdf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_cdf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_cdf(2, 10, 5, 4)
```

### sr_hypergeometric_sf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_sf(x UBIGINT, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_sf(2, 10, 5, 4)
```

### sr_hypergeometric_quantile(p, population, successes, draws)

**Signature**: `sr_hypergeometric_quantile(p DOUBLE, population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_quantile(0.5, 10, 5, 4)
```

### sr_hypergeometric_entropy(population, successes, draws)

**Signature**: `sr_hypergeometric_entropy(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_entropy(10, 5, 4)
-- NULL
```

### sr_hypergeometric_max(population, successes, draws)

**Signature**: `sr_hypergeometric_max(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric maximum of the support.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_max(10, 5, 4)
-- 4.0
```

### sr_hypergeometric_mean(population, successes, draws)

**Signature**: `sr_hypergeometric_mean(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_mean(10, 5, 4)
-- 2.0
```

### sr_hypergeometric_min(population, successes, draws)

**Signature**: `sr_hypergeometric_min(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric minimum of the support.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_min(10, 5, 4)
-- 0.0
```

### sr_hypergeometric_mode(population, successes, draws)

**Signature**: `sr_hypergeometric_mode(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_mode(10, 5, 4)
-- 2.0
```

### sr_hypergeometric_skewness(population, successes, draws)

**Signature**: `sr_hypergeometric_skewness(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_skewness(10, 5, 4)
-- 0.0
```

### sr_hypergeometric_std_dev(population, successes, draws)

**Signature**: `sr_hypergeometric_std_dev(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_std_dev(10, 5, 4)
-- 0.816496580927726
```

### sr_hypergeometric_variance(population, successes, draws)

**Signature**: `sr_hypergeometric_variance(population UBIGINT, successes UBIGINT, draws UBIGINT) -> DOUBLE`

Hypergeometric variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_variance(10, 5, 4)
-- 0.6666666666666666
```

## NULL argument short-circuit

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(NULL, 3.0)
-- NULL
```
