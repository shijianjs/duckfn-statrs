---
title: Empirical distribution
sidebar_position: 2
description: Empirical CDF, survival function, quantile and moments — aggregates over a DOUBLE column.
---

# Empirical distribution (aggregates)

Every function on this page is an **aggregate**: it collects a whole DOUBLE column as the sample
and evaluates an empirical statistic over it. `cdf`, `sf` and `quantile` take a constant second
argument; the moments (`mean`, `variance`, `std_dev`, `min`, `max`, …) summarise the column on
their own.

## sr_empirical_cdf(v, x)

**Signature**: `sr_empirical_cdf(v DOUBLE, x DOUBLE) -> DOUBLE`

Empirical cumulative distribution function of the collected column `v`, evaluated at the constant
`x`. Equivalent to the fraction of rows with `v <= x`.

An empty group yields NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_cdf(v, 2.0)
FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)
-- 0.5
```

## sr_empirical_sf(v, x)

**Signature**: `sr_empirical_sf(v DOUBLE, x DOUBLE) -> DOUBLE`

Empirical survival function of `v` evaluated at `x`. Equivalent to `1 - cdf(x)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_sf(v, 2.0)
FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)
-- 0.5
```

## sr_empirical_quantile(v, p)

**Signature**: `sr_empirical_quantile(v DOUBLE, p DOUBLE) -> DOUBLE`

Empirical quantile function of `v` at the constant probability `p` in [0, 1].

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_quantile(v, 0.5)::DECIMAL(12,6)
FROM (VALUES (1.0), (2.0), (3.0), (4.0)) t(v)
-- 1.999969
```

### sr_empirical_entropy(values)

**Signature**: `sr_empirical_entropy(values DOUBLE) -> DOUBLE`

Empirical entropy of a DOUBLE column; always NULL, since statrs implements no entropy for the empirical distribution.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_entropy(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- NULL
```

### sr_empirical_max(values)

**Signature**: `sr_empirical_max(values DOUBLE) -> DOUBLE`

Empirical maximum of a DOUBLE column.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_max(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 3.0
```

### sr_empirical_mean(values)

**Signature**: `sr_empirical_mean(values DOUBLE) -> DOUBLE`

Empirical mean of a DOUBLE column.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_mean(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 2.0
```

### sr_empirical_min(values)

**Signature**: `sr_empirical_min(values DOUBLE) -> DOUBLE`

Empirical minimum of a DOUBLE column.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_min(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 1.0
```

### sr_empirical_skewness(values)

**Signature**: `sr_empirical_skewness(values DOUBLE) -> DOUBLE`

Empirical skewness of a DOUBLE column; always NULL, since statrs implements no skewness for the empirical distribution.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_skewness(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- NULL
```

### sr_empirical_std_dev(values)

**Signature**: `sr_empirical_std_dev(values DOUBLE) -> DOUBLE`

Empirical standard deviation of a DOUBLE column.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_std_dev(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 1.0
```

### sr_empirical_variance(values)

**Signature**: `sr_empirical_variance(values DOUBLE) -> DOUBLE`

Empirical variance of a DOUBLE column.

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_variance(v) FROM (VALUES (1.0), (2.0), (3.0)) t(v)
-- 1.0
```

## NULL rows and empty groups

NULL rows are skipped by the collector, matching the SQL aggregate convention. An empty group
reports NULL:

```sql {"type":"duckfn","show":"value"}
SELECT sr_empirical_cdf(v, 2.0)
FROM (VALUES (1.0)) t(v) WHERE 1 = 0
-- NULL
```

## With GROUP BY

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_empirical_cdf(v, 2.0) AS cdf_at_2
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 2.0), (2, 4.0), (2, 6.0)) t(g, v)
GROUP BY g ORDER BY g
```
