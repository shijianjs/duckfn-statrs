---
title: Multivariate distributions
sidebar_position: 3
description: Multivariate normal, Student-t, Dirichlet and multinomial densities, log-densities, moments and supports — vectors as LIST(DOUBLE), matrices as row-major flattened LIST.
---

# Multivariate distributions

Vectors are `LIST(DOUBLE)`; matrices (covariance, scale) are `LIST(DOUBLE)` in **row-major**
flattened form. A 2x2 identity matrix is written `[1.0, 0.0, 0.0, 1.0]`.

## sr_multivariate_normal_pdf(x, mean, covariance)

**Signature**: `sr_multivariate_normal_pdf(x DOUBLE[], mean DOUBLE[], covariance DOUBLE[]) -> DOUBLE`

Probability density of `x` under a multivariate normal distribution with the given mean vector
and covariance matrix.

The size of `covariance` must equal `len(mean)²`. A mismatch is a query error (statrs' internal
`panic` is caught at the SQL boundary).

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- 0.15915494309189535
```

The value is `1 / (2π)` — the density of a standard bivariate normal at the origin.

### sr_multivariate_normal_ln_pdf(x, mean, cov)

**Signature**: `sr_multivariate_normal_ln_pdf(x DOUBLE[], mean DOUBLE[], covariance DOUBLE[]) -> DOUBLE`

Log probability density of `x` — equivalent to `ln(sr_multivariate_normal_pdf(...))` but computed
without the intermediate exponential, so it stays finite far into the tails where the density
underflows to 0.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_ln_pdf([1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- -2.8378770664093453
```

### sr_multivariate_normal_min(mean, cov)

**Signature**: `sr_multivariate_normal_min(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

Lower bound of the support: a vector of `-inf` with the dimension of the mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_min([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [-inf, -inf]
```

### sr_multivariate_normal_max(mean, cov)

**Signature**: `sr_multivariate_normal_max(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

Upper bound of the support: a vector of `+inf` with the dimension of the mean.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_max([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [inf, inf]
```

### sr_multivariate_normal_entropy(mean, cov)

**Signature**: `sr_multivariate_normal_entropy(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE`

Differential entropy of the multivariate normal distribution given the mean and a row-major flattened covariance matrix.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_entropy([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- 2.8378770664093453
```

### sr_multivariate_normal_mean(mean, cov)

**Signature**: `sr_multivariate_normal_mean(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

Mean vector of the multivariate normal distribution given the mean and a row-major flattened covariance matrix.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_mean([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [0.0, 0.0]
```

### sr_multivariate_normal_mode(mean, cov)

**Signature**: `sr_multivariate_normal_mode(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

Mode vector of the multivariate normal distribution given the mean and a row-major flattened covariance matrix.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_mode([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [0.0, 0.0]
```

### sr_multivariate_normal_variance(mean, cov)

**Signature**: `sr_multivariate_normal_variance(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

Covariance matrix of the multivariate normal distribution (row-major flattened LIST) given the mean and a row-major flattened covariance matrix.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_variance([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [1.0, 0.0, 0.0, 1.0]
```

### sr_multivariate_normal_precision(mean, cov)

**Signature**: `sr_multivariate_normal_precision(mean DOUBLE[], cov DOUBLE[]) -> DOUBLE[]`

The **precision matrix** `Σ⁻¹` — the inverse of the covariance matrix, row-major flattened.

Unlike `variance`, which returns the covariance you passed in, this returns a matrix statrs
actually computes (from the Cholesky factorisation at construction time) — it is the `Σ⁻¹` in
the density's exponent term `-(1/2)·(x-μ)ᵀ·Σ⁻¹·(x-μ)`. DuckDB has no matrix inverse, so there
is no way to derive it on the SQL side.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_precision([0.0, 0.0], [1.0, 0.0, 0.0, 1.0])
-- [1.0, 0.0, 0.0, 1.0]
```

For the identity covariance the inverse is the identity again. A diagonal case shows the
inversion directly: `diag(2, 4)⁻¹ = diag(1/2, 1/4)` — `1/2` prints as `0.49999999999999994`
in binary floating point.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_precision([0.0, 0.0], [2.0, 0.0, 0.0, 4.0])
-- [0.49999999999999994, 0.0, 0.0, 0.25]
```

## sr_multivariate_students_t_pdf(x, location, scale, freedom)

**Signature**: `sr_multivariate_students_t_pdf(x DOUBLE[], location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE`

Density under a multivariate Student's t distribution. At `location = 0`, `scale = I` and
symmetric `x = 0`, the density matches the multivariate normal at the origin.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- 0.1591549430918955
```

### sr_multivariate_students_t_ln_pdf(x, location, scale, freedom)

**Signature**: `sr_multivariate_students_t_ln_pdf(x DOUBLE[], location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE`

Log probability density under a multivariate Student's t distribution. As `freedom` grows the
values converge to the multivariate normal's `ln_pdf`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_ln_pdf([1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 4.0)
-- -3.0542723907338383
```

### sr_multivariate_students_t_mean(location, scale, freedom)

**Signature**: `sr_multivariate_students_t_mean(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

Mean vector of the multivariate Student's t distribution (equal to `location`). Only defined for
`freedom > 1`; otherwise the result is NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_mean([-1.0, 1.0, 3.0], [1.0, 0.0, 0.5, 0.0, 2.0, 0.0, 0.5, 0.0, 3.0], 2.0)
-- [-1.0, 1.0, 3.0]
```

### sr_multivariate_students_t_variance(location, scale, freedom)

**Signature**: `sr_multivariate_students_t_variance(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

Covariance matrix `scale · ν / (ν − 2)` as a row-major flattened LIST. Only defined for
`freedom > 2`; otherwise the result is NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_variance([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [3.0, 0.0, 0.0, 3.0]
```

### sr_multivariate_students_t_precision(location, scale, freedom)

**Signature**: `sr_multivariate_students_t_precision(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

The **precision matrix** `scale⁻¹` — the inverse of the scale matrix, row-major flattened.

Do not confuse it with `variance`: that one is `scale · ν / (ν − 2)` (a genuine covariance that
scales with the degrees of freedom), this one is exactly `scale⁻¹` and does **not** depend on
`freedom` at all — only `scale` matters. statrs computes it at construction time from the
Cholesky factorisation, and DuckDB has no matrix inverse, so it is not derivable on the SQL side.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_precision([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [1.0, 0.0, 0.0, 1.0]
```

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_precision([0.0, 0.0], [2.0, 0.0, 0.0, 4.0], 3.0)
-- [0.49999999999999994, 0.0, 0.0, 0.25]
```

### sr_multivariate_students_t_min(location, scale, freedom)

**Signature**: `sr_multivariate_students_t_min(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

Lower bound of the support: a vector of `-inf` with the dimension of `location`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_min([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [-inf, -inf]
```

### sr_multivariate_students_t_max(location, scale, freedom)

**Signature**: `sr_multivariate_students_t_max(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

Upper bound of the support: a vector of `+inf` with the dimension of `location`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_max([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [inf, inf]
```

### sr_multivariate_students_t_mode(location, scale, freedom)

**Signature**: `sr_multivariate_students_t_mode(location DOUBLE[], scale DOUBLE[], freedom DOUBLE) -> DOUBLE[]`

Mode vector of the multivariate Student's t distribution given location, a row-major flattened scale matrix and the degrees of freedom.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_mode([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)
-- [0.0, 0.0]
```

## sr_dirichlet_pdf(x, alpha)

**Signature**: `sr_dirichlet_pdf(x DOUBLE[], alpha DOUBLE[]) -> DOUBLE`

Density of `x` on the simplex under a Dirichlet distribution with concentration vector `alpha`.
`x` and `alpha` must have the same length.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_pdf([0.5, 0.5], [1.0, 1.0])
-- 1.0000000000000009
```

For `alpha = (1, 1)` the Dirichlet is uniform on the simplex, so the density is 1 everywhere
on `{(x1, x2) : x1 + x2 = 1, xi >= 0}`.

### sr_dirichlet_ln_pdf(x, alpha)

**Signature**: `sr_dirichlet_ln_pdf(x DOUBLE[], alpha DOUBLE[]) -> DOUBLE`

Log density of `x` on the simplex. Every component of `x` must lie in `(0, 1)` and the components
must sum to 1 (within `1e-4`) — violations are statrs assertion failures, which surface as query
errors.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_ln_pdf([0.1, 0.2, 0.3, 0.4], [0.1, 0.3, 0.5, 0.8])
-- -0.18456529434757482
```

### sr_dirichlet_mean(alpha)

**Signature**: `sr_dirichlet_mean(alpha DOUBLE[]) -> DOUBLE[]`

Mean vector of the Dirichlet distribution: `alpha_i / alpha_0` where `alpha_0` is the sum of the
concentration parameters.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_mean([1.0, 2.0, 3.0, 4.0])
-- [0.1, 0.2, 0.3, 0.4]
```

### sr_dirichlet_variance(alpha)

**Signature**: `sr_dirichlet_variance(alpha DOUBLE[]) -> DOUBLE[]`

Covariance matrix of the Dirichlet distribution as a row-major flattened LIST of length
`len(alpha)²`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_variance([1.0, 2.0])
-- [0.05555555555555556, -0.05555555555555556, -0.05555555555555556, 0.05555555555555556]
```

## sr_dirichlet_entropy(alpha)

**Signature**: `sr_dirichlet_entropy(alpha DOUBLE[]) -> DOUBLE`

Differential entropy of `Dir(alpha)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_entropy([1.0, 1.0])
-- -8.881784197001252e-16
```

For `alpha = (1, 1)` the entropy is 0; the tiny negative value is floating-point noise.

## sr_multinomial_pmf(probs, trials, counts)

**Signature**: `sr_multinomial_pmf(probs DOUBLE[], trials UBIGINT, counts UBIGINT[]) -> DOUBLE`

Probability mass of a count vector under a multinomial distribution.

- `probs`: category probabilities (must sum to 1 in effect; statrs normalises internally)
- `trials`: total number of trials, a UBIGINT
- `counts`: observations per category, `LIST(BIGINT)`

The counts must sum to `trials`. A non-integer `trials` is a query error.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_pmf([0.5, 0.5], 4, [2::UBIGINT, 2::UBIGINT])
-- 0.3750000000000001
```

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multinomial_pmf([0.5, 0.5], 4.5, [2::UBIGINT, 2::UBIGINT])
-- error: no matching signature: trials is UBIGINT, so 4.5 does not bind
```

### sr_multinomial_ln_pmf(probs, trials, counts)

**Signature**: `sr_multinomial_ln_pmf(probs DOUBLE[], trials UBIGINT, counts UBIGINT[]) -> DOUBLE`

Log probability mass of a count vector. Unlike `pmf`, a count vector that does not sum to
`trials` is not an error but a legitimate `-inf` (the log of probability 0). For large `trials`
the log form stays finite where the mass itself would underflow.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_ln_pmf([0.5, 0.5], 2000, [1000::UBIGINT, 1000::UBIGINT])
-- -4.026367582410558
```

### sr_multinomial_mean(probs, trials)

**Signature**: `sr_multinomial_mean(probs DOUBLE[], trials UBIGINT) -> DOUBLE[]`

Mean vector of the multinomial distribution: `n · p_i` per category.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_mean([0.3, 0.7], 5)
-- [1.5, 3.5]
```

### sr_multinomial_variance(probs, trials)

**Signature**: `sr_multinomial_variance(probs DOUBLE[], trials UBIGINT) -> DOUBLE[]`

Covariance matrix of the multinomial distribution as a row-major flattened LIST of length
`len(probs)²`: diagonal `n · p_i · (1 − p_i)`, off-diagonal `−n · p_i · p_j`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_variance([0.1, 0.3, 0.6], 10)
-- [0.9, -0.3, -0.6, -0.3, 2.1, -1.8, -0.6, -1.8, 2.4]
```

## Shape validation

A matrix argument is validated against the vector length:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0])
-- error: sr_multivariate_normal_pdf: expected 4 entries for a 2x2 covariance matrix (row-major), got 2
```
