---
title: Categorical and discrete uniform
sidebar_position: 3
description: Categorical and discrete uniform distributions — pmf, ln_pmf, cdf, sf, quantile for each.
---

# Categorical and discrete uniform

Integer-valued slots are whole-number DOUBLE literals; non-integers are query errors.

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
