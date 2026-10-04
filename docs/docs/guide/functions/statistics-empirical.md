---
title: Empirical distribution
sidebar_position: 4
description: Empirical CDF, survival function and quantile — aggregates over a DOUBLE column.
---

# Empirical distribution (aggregates)

The three functions on this page are **aggregates**: they collect a whole DOUBLE column as the
sample, then evaluate an empirical statistic at a constant second argument.

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
