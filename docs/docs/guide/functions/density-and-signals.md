---
title: Density estimation and signal generation
sidebar_position: 6
description: KDE and k-NN density estimation; sinusoidal, square, triangle, sawtooth, periodic and log-spaced generators.
---

# Density estimation and signal generation

## Density estimation

### sr_kde_pdf(x, sample, bandwidth)

**Signature**: `sr_kde_pdf(x DOUBLE, sample DOUBLE[], bandwidth DOUBLE) -> DOUBLE`

Kernel density estimate at `x` using a Gaussian kernel over a k-d tree built from `sample`.
Pass `NULL` for `bandwidth` to select statrs' automatic bandwidth.

An empty sample is a query error (`EmptySample`), not NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_kde_pdf(0.5, [0.0, 0.2, 0.7, 1.0], 0.5)
-- 0.5927180961167049
```

```sql {"type":"duckfn","show":"value"}
-- NULL bandwidth selects statrs' automatic bandwidth (a different value than the manual case):
SELECT sr_kde_pdf(0.5, [0.0, 0.2, 0.7, 1.0], NULL)
-- 0.6336441730224942
```

### sr_knn_pdf(x, sample, bandwidth)

**Signature**: `sr_knn_pdf(x DOUBLE, sample DOUBLE[], bandwidth DOUBLE) -> DOUBLE`

k-nearest-neighbour density estimate at `x`. Pass `NULL` for `bandwidth` to select the
automatic k.

```sql {"type":"duckfn","show":"value"}
SELECT sr_knn_pdf(0.5, [0.0, 0.2, 0.7, 1.0], 0.5)
-- 1.0000000000000018
```

## Signal generation

Every generator returns `LIST(DOUBLE)`. The `k` parameter is a **BIGINT** (the count of points
to take from the infinite sequence). `delay` is also BIGINT.

### sr_gen_sinusoidal(k, sample_rate, frequency, amplitude, mean, phase, delay)

**Signature**: `sr_gen_sinusoidal(k BIGINT, sample_rate DOUBLE, frequency DOUBLE, amplitude DOUBLE, mean DOUBLE, phase DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

First `k` points of a sinusoidal wave.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_sinusoidal(4, 4.0, 1.0, 1.0, 0.0, 0.0, 0)
-- [0.0, 1.0, 1.2246467991473532e-16, -1.0]
```

### sr_gen_square(k, high, low, amplitude, offset, delay)

**Signature**: `sr_gen_square(k BIGINT, high BIGINT, low BIGINT, amplitude DOUBLE, offset DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

First `k` points of a square wave. `high` and `low` are the number of samples at each level.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_square(8, 2, 2, 1.0, 0.0, 0)
-- [1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]
```

### sr_gen_triangle(k, raise, fall, amplitude, offset, delay)

**Signature**: `sr_gen_triangle(k BIGINT, raise BIGINT, fall BIGINT, amplitude DOUBLE, offset DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

First `k` points of a triangle wave.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_triangle(6, 1, 1, 1.0, 0.0, 0)
```

### sr_gen_sawtooth(k, period, amplitude, offset, delay)

**Signature**: `sr_gen_sawtooth(k BIGINT, period BIGINT, amplitude DOUBLE, offset DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

First `k` points of a sawtooth wave with the given period (in samples).

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_sawtooth(5, 4, 1.0, 0.0, 0)
-- [0.0, 0.3333333333333333, 0.6666666666666666, 1.0, 0.0]
```

### sr_gen_periodic(k, sample_rate, frequency, amplitude, phase, delay)

**Signature**: `sr_gen_periodic(k BIGINT, sample_rate DOUBLE, frequency DOUBLE, amplitude DOUBLE, phase DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

First `k` points of the general periodic generator.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_periodic(10, 8.0, 1.0, 1.0, 0.0, 0)
```

### sr_gen_log_spaced(n, start_exp, stop_exp)

**Signature**: `sr_gen_log_spaced(n UBIGINT, start_exp DOUBLE, stop_exp DOUBLE) -> LIST(DOUBLE)`

`n` points log-spaced between `10^start_exp` and `10^stop_exp`. `n` is a UBIGINT.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_log_spaced(3, 0.0, 2.0)
-- [1.0, 10.0, 100.0]
```

## Errors

A non-positive `k` is a call error (not NULL):

```sql {"type":"duckfn","expect":"error"}
SELECT sr_gen_square(0, 2, 2, 1.0, 0.0, 0)
-- error: sr_gen_square: k must be a positive whole number, got 0
```
