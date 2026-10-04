---
title: Constants
sidebar_position: 13
description: Nine mathematical constants from statrs::consts, exposed as zero-argument scalar functions.
---

# Constants

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

### sr_frac_1_sqrt_pi()

**Signature**: `sr_frac_1_sqrt_pi() -> DOUBLE`

The constant 1/sqrt(PI).

```sql {"type":"duckfn","show":"value"}
SELECT sr_frac_1_sqrt_pi()
-- 0.5641895835477563
```

### sr_ln_2()

**Signature**: `sr_ln_2() -> DOUBLE`

The constant ln(2).

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_2()
-- 0.6931471805599453
```
