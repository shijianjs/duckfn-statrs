---
title: Constants
sidebar_position: 2
description: Mathematical constants from statrs::consts plus statrs' own floating-point precision thresholds and almost_eq comparator, exposed as zero-argument scalar functions.
---

# Constants

## Mathematical constants

Zero-argument scalar functions returning mathematical constants as DOUBLE.

### sr_ln_pi()

**Signature**: `sr_ln_pi() -> DOUBLE`

The constant ln(PI).

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_pi()
-- 1.1447298858494002
```

### sr_sqrt_2pi()

**Signature**: `sr_sqrt_2pi() -> DOUBLE`

The constant sqrt(2*PI), the normalizing factor for the Gaussian density.

```sql {"type":"duckfn","show":"value"}
SELECT sr_sqrt_2pi()
-- 2.5066282746310002
```

### sr_ln_sqrt_2pi()

**Signature**: `sr_ln_sqrt_2pi() -> DOUBLE`

The constant ln(sqrt(2*PI)).

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_sqrt_2pi()
-- 0.9189385332046727
```

### sr_ln_sqrt_2pie()

**Signature**: `sr_ln_sqrt_2pie() -> DOUBLE`

The constant ln(sqrt(2*PI*e)).

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_sqrt_2pie()
-- 1.4189385332046727
```

### sr_2_sqrt_e_over_pi()

**Signature**: `sr_2_sqrt_e_over_pi() -> DOUBLE`

The constant 2*sqrt(e/PI).

```sql {"type":"duckfn","show":"value"}
SELECT sr_2_sqrt_e_over_pi()
-- 2.2964211229736176
```

### sr_ln_2_sqrt_e_over_pi()

**Signature**: `sr_ln_2_sqrt_e_over_pi() -> DOUBLE`

The constant ln(2*sqrt(e/PI)).

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_2_sqrt_e_over_pi()
-- 0.8313224962216493
```

### sr_euler_mascheroni()

**Signature**: `sr_euler_mascheroni() -> DOUBLE`

The Euler-Mascheroni constant (gamma).

```sql {"type":"duckfn","show":"value"}
SELECT sr_euler_mascheroni()
-- 0.5772156649015329
```

## Precision thresholds

These come from statrs' own `prec` module — the tolerances it uses internally when comparing
floating-point results. They are exposed so that a query can reproduce statrs' notion of "close
enough" instead of hard-coding its own epsilon.

### sr_f64_prec()

**Signature**: `sr_f64_prec() -> DOUBLE`

`F64_PREC`: the maximum relative precision of an IEEE 754 double, i.e. `2^-53`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_f64_prec()
-- 1.1102230246251565e-16
```

### sr_default_f64_acc()

**Signature**: `sr_default_f64_acc() -> DOUBLE`

`DEFAULT_F64_ACC`: statrs' default absolute comparison tolerance for f64, exactly `0.01 * F64_PREC`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_f64_acc()
-- 1.1102230246251565e-15
```

### sr_default_relative_acc()

**Signature**: `sr_default_relative_acc() -> DOUBLE`

`DEFAULT_RELATIVE_ACC`: the default target relative accuracy for statrs' f64 operations.

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_relative_acc()
-- 1e-14
```

### sr_default_eps()

**Signature**: `sr_default_eps() -> DOUBLE`

`DEFAULT_EPS`: the default target absolute accuracy for statrs' f64 operations.

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_eps()
-- 1e-09
```

### sr_default_ulps()

**Signature**: `sr_default_ulps() -> DOUBLE`

`DEFAULT_ULPS`: the default target ULPs accuracy for statrs' f64 operations. It is a `u32` in
statrs; returned as DOUBLE to keep the zero-argument-returns-DOUBLE convention, and 5 is exactly
representable.

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_ulps()
-- 5.0
```

### sr_almost_eq(a, b, acc)

**Signature**: `sr_almost_eq(a DOUBLE, b DOUBLE, acc DOUBLE) -> BOOLEAN`

Whether `a` and `b` are within the absolute tolerance `acc` of each other — statrs' `almost_eq`.
Semantics follow statrs exactly: two infinities match only when they are equal, and a NaN
operand never matches anything (not even itself), so `sr_almost_eq(NaN, NaN, 'inf')` is `false`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_almost_eq(0.1 + 0.2, 0.3, sr_default_relative_acc())
-- true
```

Passing `'inf'` as `acc` turns it into an exact equality test that is NaN-aware.

```sql {"type":"duckfn","show":"value"}
SELECT sr_almost_eq(1.0, 1.0, 'inf'::DOUBLE)
-- true
```
