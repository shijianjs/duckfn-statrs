---
title: "Discrete distributions (A)"
sidebar_position: 10
description: Bernoulli, Binomial, Negative binomial and Geometric — pmf, ln_pmf, cdf, sf, quantile for each.
---

# Discrete distributions (A)

For discrete distributions, integer-valued slots (`x`, trial counts, success counts) take
**whole-number DOUBLE** literals like `10.0`, not `10` or `3.5` — a non-integer is a query
error, never a silent round. Each distribution exposes 5 functions:

```
sr_<name>_pmf(x, params...)     probability mass P(X = x)
sr_<name>_ln_pmf(x, params...)  log probability mass
sr_<name>_cdf(x, params...)     P(X <= x)
sr_<name>_sf(x, params...)      P(X > x)
sr_<name>_quantile(p, params...) inverse CDF
```

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
