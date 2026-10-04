---
title: Install and load
sidebar_position: 1
description: How to install the duckfn_statrs extension into DuckDB and start calling statistical functions.
---

# Install and load

## Prerequisites

- **DuckDB** version 1.3 or newer (the [CLI](https://duckdb.org/docs/stable/why_overview), the app, or
  any language client).

## Install from the community repository

Once the extension is registered in DuckDB's [community
extensions](https://duckdb.org/community_extensions/list_of_extensions), installing is two statements:

```sql
INSTALL duckfn_statrs FROM community;
LOAD duckfn_statrs;
```

No extra flags needed — the community build is signed and matches your DuckDB version.

## Install from a GitHub Release

If the community route is not yet available for your platform, download the pre-built binary from the
[releases page](https://github.com/shijianjs/duckfn-statrs/releases):

```sql
-- Load directly from the URL (replace the platform suffix with yours)
LOAD 'https://github.com/shijianjs/duckfn-statrs/releases/latest/download/duckfn_statrs-windows_amd64.duckdb_extension';
```

:::note

Released binaries are not signed by DuckDB's distribution key, so start the CLI with the `-unsigned`
flag: `duckdb -unsigned`. The community route above does not have this limitation.

:::

## Verify the installation

```sql
SELECT function_name, function_type
FROM duckdb_functions()
WHERE function_name LIKE 'sr_%'
ORDER BY function_name;
-- 511 rows
```

## First queries

Summary statistics are **aggregates** — pass a column, get one value per group:

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS std_dev, sr_median(x) AS median
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 10.0), (2, 20.0), (2, NULL)) t(g, x)
GROUP BY g
ORDER BY g;
```

Distribution functions are **scalars** — evaluated row by row:

```sql {"type":"duckfn","show":"table"}
SELECT x, sr_normal_cdf(x, 0.0, 1.0) AS cdf, sr_normal_pdf(x, 0.0, 1.0) AS pdf
FROM (VALUES (-1.96::DOUBLE), (0.0), (1.96)) t(x);
```

Errors on invalid parameters are clear and specific:

```sql {"type":"duckfn","expect":"error"}
SELECT sr_normal_pdf(0.0, 0.0, -1.0);    -- error: std_dev must be positive
```

## NULL behaviour

- NULL input rows never enter an aggregate (standard SQL convention).
- When statrs cannot define a result (empty group, insufficient samples, out-of-range parameter),
  the return value is SQL NULL — never NAN.
- A parameter that is *present but invalid* (e.g. `std_dev <= 0`) raises a query error.

```sql {"type":"duckfn","show":"table"}
-- statrs cannot define the sample variance of one value: the result is NULL, not 0.
SELECT grp, sr_variance(x) AS variance
FROM (VALUES ('two', 1.5::DOUBLE), ('two', 2.5), ('one', NULL::DOUBLE), ('one', 3.0)) t(grp, x)
GROUP BY grp
ORDER BY grp;
```

## What to do next

- Browse the full [function reference](../guide/functions/overview.md) organized by category.
- Need to build from source or contribute? See the [development
guide](./project-structure.md).
