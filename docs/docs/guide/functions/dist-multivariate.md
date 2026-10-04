---
title: Multivariate distributions
sidebar_position: 12
description: Multivariate normal, Student-t, Dirichlet and multinomial densities — vectors as LIST(DOUBLE), matrices as row-major flattened LIST.
---

# Multivariate distributions

Vectors are `LIST(DOUBLE)`; matrices (covariance, scale) are `LIST(DOUBLE)` in **row-major**
flattened form. A 2x2 identity matrix is written `[1.0, 0.0, 0.0, 1.0]`.

## sr_multivariate_normal_pdf(x, mean, covariance)

**Signature**: `sr_multivariate_normal_pdf(x LIST(DOUBLE), mean LIST(DOUBLE), covariance LIST(DOUBLE)) -> DOUBLE`

Probability density of `x` under a multivariate normal distribution with the given mean vector
and covariance matrix.

The size of `covariance` must equal `len(mean)²`. A mismatch is a query error (statrs' internal
`panic` is caught at the SQL boundary).

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0])::DECIMAL(12,8)
-- 0.15915494
```

The value is `1 / (2π)` — the density of a standard bivariate normal at the origin.

## sr_multivariate_students_t_pdf(x, location, scale, freedom)

**Signature**: `sr_multivariate_students_t_pdf(x LIST(DOUBLE), location LIST(DOUBLE), scale LIST(DOUBLE), freedom DOUBLE) -> DOUBLE`

Density under a multivariate Student's t distribution. At `location = 0`, `scale = I` and
symmetric `x = 0`, the density matches the multivariate normal at the origin.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multivariate_students_t_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0)::DECIMAL(12,8)
-- 0.15915494
```

## sr_dirichlet_pdf(x, alpha)

**Signature**: `sr_dirichlet_pdf(x LIST(DOUBLE), alpha LIST(DOUBLE)) -> DOUBLE`

Density of `x` on the simplex under a Dirichlet distribution with concentration vector `alpha`.
`x` and `alpha` must have the same length.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_pdf([0.5, 0.5], [1.0, 1.0])::DECIMAL(12,8)
-- 1.00000000
```

For `alpha = (1, 1)` the Dirichlet is uniform on the simplex, so the density is 1 everywhere
on `{(x1, x2) : x1 + x2 = 1, xi >= 0}`.

## sr_dirichlet_entropy(alpha)

**Signature**: `sr_dirichlet_entropy(alpha LIST(DOUBLE)) -> DOUBLE`

Differential entropy of `Dir(alpha)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_dirichlet_entropy([1.0, 1.0])::DECIMAL(12,8)
-- 0.00000000
```

## sr_multinomial_pmf(probs, trials, counts)

**Signature**: `sr_multinomial_pmf(probs LIST(DOUBLE), trials DOUBLE, counts LIST(BIGINT)) -> DOUBLE`

Probability mass of a count vector under a multinomial distribution.

- `probs`: category probabilities (must sum to 1 in effect; statrs normalises internally)
- `trials`: total number of trials, a whole-number DOUBLE
- `counts`: observations per category, `LIST(BIGINT)`

The counts must sum to `trials`. A non-integer `trials` is a query error.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_pmf([0.5, 0.5], 4.0, [2, 2])::DECIMAL(12,8)
-- 0.37500000
```

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multinomial_pmf([0.5, 0.5], 4.5, [2, 2])
-- error: expected a non-negative whole number, got 4.5
```

## Shape validation

A matrix argument is validated against the vector length:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_multivariate_normal_pdf([0.0, 0.0], [0.0, 0.0], [1.0, 0.0])
-- error: expected 4 entries for a 2x2 covariance matrix
```
