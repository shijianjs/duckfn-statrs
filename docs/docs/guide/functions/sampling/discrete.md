---
title: Discrete, multivariate and empirical
sidebar_position: 2
description: Draw k samples from discrete, multivariate, categorical, and empirical distributions.
---

# Discrete, multivariate and empirical sampling

## Bernoulli

### sr_sample_bernoulli(p, k)

**Signature**: `sr_sample_bernoulli(p DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
-- Every sample is 0 or 1:
SELECT count(*) FILTER (WHERE s NOT IN (0.0, 1.0)) = 0
FROM (SELECT unnest(sr_sample_bernoulli(0.5, 200)) AS s)
-- true
```

## Binomial

### sr_sample_binomial(p, n, k)

**Signature**: `sr_sample_binomial(p DOUBLE, n UBIGINT, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_binomial(0.5, 10, 12))
-- 12
```

### sr_sample_binomial_algorithm(p, n, algorithm, k)

**Signature**: `sr_sample_binomial_algorithm(p DOUBLE, n UBIGINT, algorithm DOUBLE, k BIGINT) -> LIST(UBIGINT)`

Explicit algorithm selection: `algorithm` is 1.0 = automatic, 2.0 = inversion, 3.0 = rejection.

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_binomial_algorithm(0.5, 10, 2.0, 4))
-- 4
```

## Negative binomial

### sr_sample_negative_binomial(r, p, k)

**Signature**: `sr_sample_negative_binomial(r DOUBLE, p DOUBLE, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_negative_binomial(2.0, 0.5, 10))
```

## Poisson

### sr_sample_poisson(lambda, k)

**Signature**: `sr_sample_poisson(lambda DOUBLE, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_poisson(3.0, 10))
```

## Geometric

### sr_sample_geometric(p, k)

**Signature**: `sr_sample_geometric(p DOUBLE, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_geometric(0.5, 10))
```

## Hypergeometric

### sr_sample_hypergeometric(population, successes, draws, k)

**Signature**: `sr_sample_hypergeometric(population UBIGINT, successes UBIGINT, draws UBIGINT, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_hypergeometric(10, 5, 4, 8))
```

## Categorical

### sr_sample_categorical(probabilities, k)

**Signature**: `sr_sample_categorical(probabilities DOUBLE[], k BIGINT) -> LIST(UBIGINT)`

Each sample is a category index in `0 .. len(probabilities) - 1`.

```sql {"type":"duckfn","show":"value"}
SELECT count(*) FILTER (WHERE s NOT IN (0.0, 1.0, 2.0)) = 0
FROM (SELECT unnest(sr_sample_categorical([1.0, 2.0, 1.0], 100)) AS s)
-- true
```

## Discrete uniform

### sr_sample_discrete_uniform(min, max, k)

**Signature**: `sr_sample_discrete_uniform(min BIGINT, max BIGINT, k BIGINT) -> LIST(BIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_discrete_uniform(1, 6, 10))
```

## Multivariate normal

### sr_sample_multivariate_normal(mean, covariance, k)

**Signature**: `sr_sample_multivariate_normal(mean DOUBLE[], covariance DOUBLE[], k BIGINT) -> LIST(LIST(DOUBLE))`

Returns a list of `k` point vectors, each of the same length as `mean`.

```sql {"type":"duckfn","show":"value"}
SELECT count(*) = 25
FROM (SELECT unnest(sr_sample_multivariate_normal([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 25)) AS p)
-- true
```

## Multivariate Student's t

### sr_sample_multivariate_students_t(location, scale, freedom, k)

**Signature**: `sr_sample_multivariate_students_t(location DOUBLE[], scale DOUBLE[], freedom DOUBLE, k BIGINT) -> LIST(LIST(DOUBLE))`

Returns a list of `k` point vectors, each of the same length as `location`. `scale` is the
row-major flattened scale matrix (`len(location)²` entries); a mismatch is a query error.

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_multivariate_students_t([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0, 6))
-- 6
```

## Dirichlet

### sr_sample_dirichlet(alpha, k)

**Signature**: `sr_sample_dirichlet(alpha DOUBLE[], k BIGINT) -> LIST(LIST(DOUBLE))`

Returns `k` points on the simplex: each draw is a vector of the same length as `alpha` whose
components sum to 1.

```sql {"type":"duckfn","show":"value"}
SELECT min(s) > 0.999999, max(s) < 1.000001
FROM (SELECT list_sum(x) AS s FROM (SELECT unnest(sr_sample_dirichlet([1.0, 2.0], 8)) AS x))
-- true	true
```

## Multinomial

### sr_sample_multinomial(probs, trials, k)

**Signature**: `sr_sample_multinomial(probs DOUBLE[], trials UBIGINT, k BIGINT) -> LIST(LIST(UBIGINT))`

Returns `k` count vectors of the same length as `probs`; the components of every count vector
sum to `trials` exactly.

```sql {"type":"duckfn","show":"value"}
SELECT count(*) FROM (SELECT unnest(sr_sample_multinomial([0.3, 0.7], 10, 8)) AS x)
WHERE list_sum(x) <> 10
-- 0
```

## Empirical (aggregate)

### sr_sample_empirical(v, k)

**Signature**: `sr_sample_empirical(v DOUBLE[], k DOUBLE) -> LIST(DOUBLE)`

The only **aggregate** sampler: it collects the whole column `v` and resamples `k` points from
its empirical distribution.

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_empirical(v, 6))
FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 6
```

## Errors

Unknown `algorithm` code:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_sample_binomial_algorithm(0.5, 10, 9.0, 4)
-- error: sr_sample_binomial_algorithm: the algorithm must be 1 (automatic), 2 (inversion) or 3 (rejection), got 9
```
