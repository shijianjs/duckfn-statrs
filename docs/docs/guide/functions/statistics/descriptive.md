---
title: Summary statistics
sidebar_position: 1
description: 27 aggregate functions — mean, median, variance, covariance and more. One column in, one value per group.
---

# Summary statistics (aggregates)

All functions on this page are **aggregates**: pass a DOUBLE column (optionally a constant), get
one value per group. NULL rows are skipped automatically.

## Central tendency

### sr_mean(x)

**Signature**: `sr_mean(x DOUBLE[]) -> DOUBLE`

Arithmetic mean, NULL when no row is non-NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_mean(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 0.3333333333333333
```

### sr_geometric_mean(x)

**Signature**: `sr_geometric_mean(x DOUBLE[]) -> DOUBLE`

Geometric mean. NULL when any value is negative.

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)
-- 1.8171205928321397
```

### sr_harmonic_mean(x)

**Signature**: `sr_harmonic_mean(x DOUBLE[]) -> DOUBLE`

Harmonic mean. NULL when any value is negative.

```sql {"type":"duckfn","show":"value"}
SELECT sr_harmonic_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)
-- 1.6363636363636365
```

### sr_quadratic_mean(x)

**Signature**: `sr_quadratic_mean(x DOUBLE[]) -> DOUBLE`

Quadratic mean (root mean square).

```sql {"type":"duckfn","show":"value"}
SELECT sr_quadratic_mean(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 2.0816659994661326
```

## Order statistics

### sr_median(x)

**Signature**: `sr_median(x DOUBLE[]) -> DOUBLE`

Median. Even-length inputs average the two middle values.

```sql {"type":"duckfn","show":"value"}
SELECT sr_median(x) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)
-- 2.5
```

### sr_quantile(x, tau)

**Signature**: `sr_quantile(x DOUBLE[], tau DOUBLE) -> DOUBLE`

Tau quantile. `tau` must be in [0, 1]; otherwise NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_quantile(x, 0.5) FROM (VALUES (-1.0), (5.0), (0.0), (-3.0), (10.0), (-0.5), (4.0), (0.2), (1.0), (6.0)) t(x)
-- 0.6
```

### sr_order_statistic(x, k)

**Signature**: `sr_order_statistic(x DOUBLE[], k UBIGINT) -> DOUBLE`

k-th smallest value (1-based). NULL when k is outside the data range. An aggregate's constant argument needs an explicit `::UBIGINT` cast.

```sql {"type":"duckfn","show":"value"}
SELECT sr_order_statistic(x, 2::UBIGINT) FROM (VALUES (3.0), (1.0), (2.0)) t(x)
-- 2.0
```

### sr_percentile(x, p)

**Signature**: `sr_percentile(x DOUBLE[], p UBIGINT) -> DOUBLE`

p-th percentile. `p` is a UBIGINT in [0, 100]; an aggregate's constant argument needs an explicit `::UBIGINT` cast.

```sql {"type":"duckfn","show":"value"}
SELECT sr_percentile(x, 50::UBIGINT) FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(x)
-- 2.5
```

### sr_lower_quartile(x)

**Signature**: `sr_lower_quartile(x DOUBLE[]) -> DOUBLE`

First quartile (lower hinge).

```sql {"type":"duckfn","show":"value"}
SELECT sr_lower_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)
-- 1.4166666666666665
```

### sr_upper_quartile(x)

**Signature**: `sr_upper_quartile(x DOUBLE[]) -> DOUBLE`

Third quartile (upper hinge).

```sql {"type":"duckfn","show":"value"}
SELECT sr_upper_quartile(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)
-- 3.5833333333333335
```

### sr_interquartile_range(x)

**Signature**: `sr_interquartile_range(x DOUBLE[]) -> DOUBLE`

IQR = upper_quartile - lower_quartile.

```sql {"type":"duckfn","show":"value"}
SELECT sr_interquartile_range(x) FROM (VALUES (2.0), (1.0), (3.0), (4.0)) t(x)
-- 2.166666666666667
```

### sr_ranks(x, method)

**Signature**: `sr_ranks(x DOUBLE[], method DOUBLE) -> LIST(DOUBLE)`

Ranks of each value. Method: 1=average, 2=min, 3=max, 4=first.

```sql {"type":"duckfn","show":"value"}
SELECT sr_ranks(x, 1.0) FROM (VALUES (1.0), (3.0), (2.0), (2.0)) t(x)
-- [1.0, 4.0, 2.5, 2.5]
```

## Dispersion

### sr_variance(x)

**Signature**: `sr_variance(x DOUBLE[]) -> DOUBLE`

Sample variance (Bessel-corrected). NULL when fewer than two rows.

```sql {"type":"duckfn","show":"value"}
SELECT sr_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 6.333333333333333
```

### sr_std_dev(x)

**Signature**: `sr_std_dev(x DOUBLE[]) -> DOUBLE`

Sample standard deviation. NULL when fewer than two rows.

```sql {"type":"duckfn","show":"value"}
SELECT sr_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 2.5166114784690383
```

### sr_population_variance(x)

**Signature**: `sr_population_variance(x DOUBLE[]) -> DOUBLE`

Population variance (divides by N, not N-1).

```sql {"type":"duckfn","show":"value"}
SELECT sr_population_variance(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 4.222222222222222
```

### sr_population_std_dev(x)

**Signature**: `sr_population_std_dev(x DOUBLE[]) -> DOUBLE`

Population standard deviation.

```sql {"type":"duckfn","show":"value"}
SELECT sr_population_std_dev(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 2.0548046670007003
```

### sr_skewness(x)

**Signature**: `sr_skewness(x DOUBLE[]) -> DOUBLE`

Sample skewness (statrs' `OnlineMoments<3>`: the third central moment divided by the 3/2 power
of the population second moment). NULL when fewer than two rows are non-NULL; a constant column
(zero variance) yields 0, following statrs.

```sql {"type":"duckfn","show":"value"}
SELECT sr_skewness(x)
FROM (VALUES (2.0), (4.0), (4.0), (4.0), (5.0), (5.0), (7.0), (9.0)) t(x)
-- 0.65625
```

## Extremes

### sr_min(x)

**Signature**: `sr_min(x DOUBLE[]) -> DOUBLE`

Minimum value.

```sql {"type":"duckfn","show":"value"}
SELECT sr_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- -2.0
```

### sr_max(x)

**Signature**: `sr_max(x DOUBLE[]) -> DOUBLE`

Maximum value.

```sql {"type":"duckfn","show":"value"}
SELECT sr_max(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 3.0
```

### sr_abs_min(x)

**Signature**: `sr_abs_min(x DOUBLE[]) -> DOUBLE`

Smallest absolute value.

```sql {"type":"duckfn","show":"value"}
SELECT sr_abs_min(x) FROM (VALUES (0.0), (3.0), (-2.0)) t(x)
-- 0.0
```

### sr_abs_max(x)

**Signature**: `sr_abs_max(x DOUBLE[]) -> DOUBLE`

Largest absolute value.

```sql {"type":"duckfn","show":"value"}
SELECT sr_abs_max(x) FROM (VALUES (0.0), (3.0), (-8.0)) t(x)
-- 8.0
```

## Covariance (two-column)

### sr_covariance(x, y)

**Signature**: `sr_covariance(x DOUBLE[], y DOUBLE[]) -> DOUBLE`

Sample covariance (Bessel-corrected). A row with NULL in either column is skipped entirely.
NULL when fewer than two fully non-NULL rows.

```sql {"type":"duckfn","show":"value"}
SELECT sr_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)
-- -11.5
```

### sr_population_covariance(x, y)

**Signature**: `sr_population_covariance(x DOUBLE[], y DOUBLE[]) -> DOUBLE`

Population covariance (divides by N). NULL when no fully non-NULL row exists.

```sql {"type":"duckfn","show":"value"}
SELECT sr_population_covariance(x, y) FROM (VALUES (0.0, -5.0), (3.0, 4.0), (-2.0, 10.0)) t(x, y)
-- -7.666666666666667
```

## Using with GROUP BY

```sql {"type":"duckfn","show":"table"}
SELECT g,
       sr_mean(x) AS mean,
       sr_median(x) AS median,
       sr_std_dev(x) AS std_dev,
       sr_min(x) AS min,
       sr_max(x) AS max
FROM (VALUES (1, 2.5), (1, 3.1), (1, 1.8), (2, 7.2), (2, 8.1), (2, 6.9)) t(g, x)
GROUP BY g ORDER BY g
```

## NULL handling

```sql {"type":"duckfn","show":"table"}
-- NULL rows are skipped; the mean of [1, 3] is 2.0, not affected by the NULL
SELECT sr_mean(x) AS mean, sr_variance(x) AS var, sr_std_dev(x) AS sd
FROM (VALUES (1.0), (NULL), (3.0)) t(x)
```

```sql {"type":"duckfn","show":"value"}
-- Single value has no sample variance: result is NULL
SELECT sr_variance(x) FROM (VALUES (42.0)) t(x)
-- NULL
```
