---
title: Categorical and discrete uniform
sidebar_position: 3
description: Categorical and discrete uniform distributions — pmf, ln_pmf, cdf, sf, quantile, plus moments and support.
---

# Categorical and discrete uniform

Integer-valued slots are UBIGINT/BIGINT arguments; a non-integer literal has no matching signature.

## Categorical

Parameter: `probabilities` — an **unnormalised** probability `LIST(DOUBLE)`; statrs normalises
internally.

### sr_categorical_pmf(x, probabilities)

**Signature**: `sr_categorical_pmf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
-- [1, 2, 1] normalises to [0.25, 0.5, 0.25]; pmf(1) = 0.5
SELECT sr_categorical_pmf(1, [1.0, 2.0, 1.0])
-- 0.5
```

### sr_categorical_ln_pmf(x, probabilities)

**Signature**: `sr_categorical_ln_pmf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_ln_pmf(1, [1.0, 2.0, 1.0])
```

### sr_categorical_cdf(x, probabilities)

**Signature**: `sr_categorical_cdf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_cdf(1, [1.0, 2.0, 1.0])
-- 0.75
```

### sr_categorical_sf(x, probabilities)

**Signature**: `sr_categorical_sf(x UBIGINT, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_sf(1, [1.0, 2.0, 1.0])
```

### sr_categorical_quantile(p, probabilities)

**Signature**: `sr_categorical_quantile(p DOUBLE, probabilities DOUBLE[]) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_quantile(0.5, [1.0, 2.0, 1.0])
-- 1.0
```

Non-integer `x` in the pmf:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_categorical_pmf(1.5, [1.0, 2.0, 1.0])
-- error: no matching signature: x is UBIGINT, so 1.5 does not bind
```

### sr_categorical_entropy(probs)

**Signature**: `sr_categorical_entropy(probs DOUBLE[]) -> DOUBLE`

Categorical entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_entropy([1.0, 2.0, 1.0])
-- 1.0397207708399179
```

### sr_categorical_max(probs)

**Signature**: `sr_categorical_max(probs DOUBLE[]) -> DOUBLE`

Categorical maximum of the support (last category index).

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_max([1.0, 2.0, 1.0])
-- 2.0
```

### sr_categorical_mean(probs)

**Signature**: `sr_categorical_mean(probs DOUBLE[]) -> DOUBLE`

Categorical mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_mean([1.0, 2.0, 1.0])
-- 1.0
```

### sr_categorical_median(probs)

**Signature**: `sr_categorical_median(probs DOUBLE[]) -> DOUBLE`

Categorical median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_median([1.0, 2.0, 1.0])
-- 1.0
```

### sr_categorical_min(probs)

**Signature**: `sr_categorical_min(probs DOUBLE[]) -> DOUBLE`

Categorical minimum of the support (category index 0).

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_min([1.0, 2.0, 1.0])
-- 0.0
```

### sr_categorical_skewness(probs)

**Signature**: `sr_categorical_skewness(probs DOUBLE[]) -> DOUBLE`

Categorical skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_skewness([1.0, 2.0, 1.0])
-- NULL
```

### sr_categorical_std_dev(probs)

**Signature**: `sr_categorical_std_dev(probs DOUBLE[]) -> DOUBLE`

Categorical standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_std_dev([1.0, 2.0, 1.0])
-- 0.7071067811865476
```

### sr_categorical_variance(probs)

**Signature**: `sr_categorical_variance(probs DOUBLE[]) -> DOUBLE`

Categorical variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_categorical_variance([1.0, 2.0, 1.0])
-- 0.5
```

## Discrete uniform

Parameters: `min`, `max` — BIGINTs. Support: integers in `[min, max]`.

### sr_discrete_uniform_pmf(x, min, max)

**Signature**: `sr_discrete_uniform_pmf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_pmf(2, 1, 6)
-- 0.16666666666666666
```

### sr_discrete_uniform_ln_pmf(x, min, max)

**Signature**: `sr_discrete_uniform_ln_pmf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_ln_pmf(2, 1, 6)
```

### sr_discrete_uniform_cdf(x, min, max)

**Signature**: `sr_discrete_uniform_cdf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_cdf(3, 1, 6)
```

### sr_discrete_uniform_sf(x, min, max)

**Signature**: `sr_discrete_uniform_sf(x BIGINT, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_sf(3, 1, 6)
```

### sr_discrete_uniform_quantile(p, min, max)

**Signature**: `sr_discrete_uniform_quantile(p DOUBLE, min BIGINT, max BIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_quantile(0.5, 1, 6)
-- 3.0
```

### sr_discrete_uniform_entropy(min, max)

**Signature**: `sr_discrete_uniform_entropy(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform entropy.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_entropy(1, 6)
-- 1.791759469228055
```

### sr_discrete_uniform_max(min, max)

**Signature**: `sr_discrete_uniform_max(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform maximum of the support (max).

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_max(1, 6)
-- 6.0
```

### sr_discrete_uniform_mean(min, max)

**Signature**: `sr_discrete_uniform_mean(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_mean(1, 6)
-- 3.5
```

### sr_discrete_uniform_median(min, max)

**Signature**: `sr_discrete_uniform_median(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform median.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_median(1, 6)
-- 3.5
```

### sr_discrete_uniform_min(min, max)

**Signature**: `sr_discrete_uniform_min(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform minimum of the support (min).

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_min(1, 6)
-- 1.0
```

### sr_discrete_uniform_mode(min, max)

**Signature**: `sr_discrete_uniform_mode(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform mode.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_mode(1, 6)
-- 3.0
```

### sr_discrete_uniform_skewness(min, max)

**Signature**: `sr_discrete_uniform_skewness(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform skewness.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_skewness(1, 6)
-- 0.0
```

### sr_discrete_uniform_std_dev(min, max)

**Signature**: `sr_discrete_uniform_std_dev(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_std_dev(1, 6)
-- 1.707825127659933
```

### sr_discrete_uniform_variance(min, max)

**Signature**: `sr_discrete_uniform_variance(min BIGINT, max BIGINT) -> DOUBLE`

Discrete uniform variance.

```sql {"type":"duckfn","show":"value"}
SELECT sr_discrete_uniform_variance(1, 6)
-- 2.9166666666666665
```
