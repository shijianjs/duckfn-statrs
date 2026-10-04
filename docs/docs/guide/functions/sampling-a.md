---
title: "Random sampling (A)"
sidebar_position: 15
description: Draw k samples from continuous distributions into a LIST(DOUBLE).
---

# Random sampling (A): continuous distributions

Every sampler returns a `LIST(DOUBLE)` of length `k`. `k` is a **BIGINT** (positive whole
number). Random output has no fixed point value, so examples assert a structural invariant
(`len(...)` or a range check). Parameters mirror the corresponding distribution function's
signature minus the evaluation point `x`.

## sr_sample_normal(mean, std_dev, k)

**Signature**: `sr_sample_normal(mean DOUBLE, std_dev DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_normal(0.0, 1.0, 10))
-- 10
```

## sr_sample_log_normal(location, scale, k)

**Signature**: `sr_sample_log_normal(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_log_normal(0.0, 1.0, 10))
```

## sr_sample_gamma(shape, rate, k)

**Signature**: `sr_sample_gamma(shape DOUBLE, rate DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_gamma(2.0, 2.0, 10))
```

## sr_sample_inverse_gamma(shape, scale, k)

**Signature**: `sr_sample_inverse_gamma(shape DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_inverse_gamma(2.0, 2.0, 10))
```

## sr_sample_chi_squared(freedom, k)

**Signature**: `sr_sample_chi_squared(freedom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_chi_squared(2.0, 10))
```

## sr_sample_chi(freedom, k)

**Signature**: `sr_sample_chi(freedom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_chi(2.0, 10))
```

## sr_sample_erlang(shape, rate, k)

**Signature**: `sr_sample_erlang(shape DOUBLE, rate DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_erlang(2.0, 2.0, 10))
```

## sr_sample_exp(rate, k)

**Signature**: `sr_sample_exp(rate DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_exp(2.0, 5))
-- 5
```

## sr_sample_uniform(min, max, k)

**Signature**: `sr_sample_uniform(min DOUBLE, max DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT count(*) FILTER (WHERE s < 2.0 OR s > 3.0) = 0
FROM (SELECT unnest(sr_sample_uniform(2.0, 3.0, 300)) AS s)
-- true
```

## sr_sample_students_t(location, scale, freedom, k)

**Signature**: `sr_sample_students_t(location DOUBLE, scale DOUBLE, freedom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_students_t(0.0, 1.0, 2.0, 10))
```

## sr_sample_fisher_snedecor(df_num, df_denom, k)

**Signature**: `sr_sample_fisher_snedecor(df_num DOUBLE, df_denom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_fisher_snedecor(2.0, 3.0, 10))
```

## sr_sample_cauchy(location, scale, k)

**Signature**: `sr_sample_cauchy(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_cauchy(0.0, 1.0, 10))
```

## sr_sample_laplace(location, scale, k)

**Signature**: `sr_sample_laplace(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_laplace(0.0, 1.0, 10))
```

## sr_sample_gumbel(location, scale, k)

**Signature**: `sr_sample_gumbel(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_gumbel(0.0, 1.0, 10))
```

## sr_sample_levy(mu, c, k)

**Signature**: `sr_sample_levy(mu DOUBLE, c DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_levy(0.0, 1.0, 10))
```

## sr_sample_pareto(scale, shape, k)

**Signature**: `sr_sample_pareto(scale DOUBLE, shape DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_pareto(1.0, 2.0, 10))
```

## sr_sample_triangular(min, max, mode, k)

**Signature**: `sr_sample_triangular(min DOUBLE, max DOUBLE, mode DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_triangular(0.0, 2.0, 1.0, 10))
```

## sr_sample_weibull(shape, scale, k)

**Signature**: `sr_sample_weibull(shape DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_weibull(1.0, 1.0, 10))
```

## sr_sample_dirac(v, k)

**Signature**: `sr_sample_dirac(v DOUBLE, k BIGINT) -> LIST(DOUBLE)`

All samples are exactly `v`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_dirac(3.0, 2)
-- [3.0, 3.0]
```

## Errors

`k` must be a positive whole number:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_sample_normal(0.0, 1.0, 0)
-- error: sr_sample_normal: k must be a positive whole number, got 0
```
