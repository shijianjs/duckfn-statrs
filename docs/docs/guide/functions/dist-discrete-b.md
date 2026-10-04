---
title: "Discrete distributions (B)"
sidebar_position: 11
description: Poisson, hypergeometric, categorical and discrete uniform — pmf, ln_pmf, cdf, sf, quantile for each.
---

# Discrete distributions (B)

Same 5-function pattern as [Discrete distributions (A)](./dist-discrete-a.md). Integer slots
are whole-number DOUBLE literals; non-integers are query errors.

## Poisson

Parameter: `lambda` (rate, > 0).

### sr_poisson_pmf(x, lambda)

**Signature**: `sr_poisson_pmf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(2.0, 3.0)::DECIMAL(12,8)
-- 0.22404181
```

### sr_poisson_ln_pmf(x, lambda)

**Signature**: `sr_poisson_ln_pmf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT exp(sr_poisson_ln_pmf(2.0, 3.0))::DECIMAL(12,8)
-- 0.22404181
```

### sr_poisson_cdf(x, lambda)

**Signature**: `sr_poisson_cdf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_cdf(2.0, 3.0)
```

### sr_poisson_sf(x, lambda)

**Signature**: `sr_poisson_sf(x DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_sf(2.0, 3.0)
```

### sr_poisson_quantile(p, lambda)

**Signature**: `sr_poisson_quantile(p DOUBLE, lambda DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_quantile(0.5, 3.0)
```

Out-of-range probability:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_poisson_quantile(-0.1, 3.0)
-- error: the probability must be within [0, 1], got -0.1
```

## Hypergeometric

Parameters: `population` (N, whole-number DOUBLE), `successes` (K, whole-number DOUBLE),
`draws` (n, whole-number DOUBLE). Represents drawing n items without replacement from a
population of size N with K successes.

### sr_hypergeometric_pmf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_pmf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_pmf(2.0, 10.0, 5.0, 4.0)::DECIMAL(12,8)
-- 0.47619048
```

### sr_hypergeometric_ln_pmf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_ln_pmf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_ln_pmf(2.0, 10.0, 5.0, 4.0)
```

### sr_hypergeometric_cdf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_cdf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_cdf(2.0, 10.0, 5.0, 4.0)
```

### sr_hypergeometric_sf(x, population, successes, draws)

**Signature**: `sr_hypergeometric_sf(x DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_sf(2.0, 10.0, 5.0, 4.0)
```

### sr_hypergeometric_quantile(p, population, successes, draws)

**Signature**: `sr_hypergeometric_quantile(p DOUBLE, population DOUBLE, successes DOUBLE, draws DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_hypergeometric_quantile(0.5, 10.0, 5.0, 4.0)
```

## Categorical

Parameter: `probabilities` — an **unnormalised** probability `LIST(DOUBLE)`; statrs normalises
internally.

### sr_categorical_pmf(x, probabilities)

**Signature**: `sr_categorical_pmf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
-- [1, 2, 1] normalises to [0.25, 0.5, 0.25]; pmf(1) = 0.5
SELECT sr_categorical_pmf(1.0, [1.0, 2.0, 1.0])
-- 0.5
```

### sr_categorical_ln_pmf(x, probabilities)

**Signature**: `sr_categorical_ln_pmf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_ln_pmf(1.0, [1.0, 2.0, 1.0])
```

### sr_categorical_cdf(x, probabilities)

**Signature**: `sr_categorical_cdf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_cdf(1.0, [1.0, 2.0, 1.0])
-- 0.75
```

### sr_categorical_sf(x, probabilities)

**Signature**: `sr_categorical_sf(x DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_sf(1.0, [1.0, 2.0, 1.0])
```

### sr_categorical_quantile(p, probabilities)

**Signature**: `sr_categorical_quantile(p DOUBLE, probabilities LIST(DOUBLE)) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_quantile(0.5, [1.0, 2.0, 1.0])
-- 1.0
```

Non-integer `x` in the pmf:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_categorical_pmf(1.5, [1.0, 2.0, 1.0])
-- error: expected a non-negative whole number, got 1.5
```

## Discrete uniform

Parameters: `min`, `max` — whole-number DOUBLEs. Support: integers in `[min, max]`.

### sr_discrete_uniform_pmf(x, min, max)

**Signature**: `sr_discrete_uniform_pmf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_pmf(2.0, 1.0, 6.0)::DECIMAL(12,8)
-- 0.16666667
```

### sr_discrete_uniform_ln_pmf(x, min, max)

**Signature**: `sr_discrete_uniform_ln_pmf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_ln_pmf(2.0, 1.0, 6.0)
```

### sr_discrete_uniform_cdf(x, min, max)

**Signature**: `sr_discrete_uniform_cdf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_cdf(3.0, 1.0, 6.0)
```

### sr_discrete_uniform_sf(x, min, max)

**Signature**: `sr_discrete_uniform_sf(x DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_sf(3.0, 1.0, 6.0)
```

### sr_discrete_uniform_quantile(p, min, max)

**Signature**: `sr_discrete_uniform_quantile(p DOUBLE, min DOUBLE, max DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_quantile(0.5, 1.0, 6.0)
-- 3.0
```

## NULL argument short-circuit

```sql {"type":"duckfn","show":"value"}
SELECT sr_poisson_pmf(NULL, 3.0)
-- NULL
```
