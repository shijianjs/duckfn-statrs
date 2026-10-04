---
title: Introduction
sidebar_position: 1
slug: /intro
description: duckfn_statrs brings 252 statistical functions to DuckDB SQL — summary aggregates, 27 distributions, special functions, sampling, and hypothesis tests.
---

# Introduction

`duckfn_statrs` is a DuckDB [loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)
that brings the full power of the Rust [statrs](https://crates.io/crates/statrs) statistical computing
library into SQL. It registers **252 functions** under the `sr_` prefix:

| Category | Kind | Count | Examples |
| --- | --- | --- | --- |
| Summary statistics | Aggregate | 26 | `sr_mean`, `sr_median`, `sr_variance`, `sr_covariance` |
| Continuous distributions | Scalar | 98 | `sr_normal_pdf`, `sr_gamma_cdf`, `sr_beta_quantile` |
| Discrete distributions | Scalar | 40 | `sr_poisson_pmf`, `sr_binomial_cdf`, `sr_geometric_quantile` |
| Special functions | Scalar | 29 | `sr_gamma`, `sr_erf`, `sr_ln_choose` |
| Constants | Scalar | 7 | `sr_ln_pi`, `sr_sqrt_2pi`, `sr_euler_mascheroni` |
| Random sampling | Scalar | 30 | `sr_sample_normal`, `sr_sample_beta` |
| Density estimation | Scalar | 2 | `sr_kde_pdf`, `sr_knn_pdf` |
| Signal generation | Scalar | 6 | `sr_gen_sinusoidal`, `sr_gen_square` |
| Hypothesis tests | Scalar | 9 | `sr_ttest_onesample`, `sr_ks_twosample`, `sr_chisquare` |
| Multivariate distributions | Scalar | 5 | `sr_multivariate_normal_pdf`, `sr_dirichlet_pdf` |

Every computation is delegated to statrs — the extension re-implements no statistical formula.
Summary statistics are **aggregates** over a DOUBLE column, distribution / sampling / special
functions / constants are **scalars** evaluated row by row.

Try it right here — this block runs in your browser (the site preloads the extension automatically):

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS std_dev
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 10.0), (2, 20.0)) t(g, x)
GROUP BY g ORDER BY g;
```

```sql {"type":"duckfn","show":"table"}
SELECT x, sr_normal_pdf(x, 0.0, 1.0) AS pdf, sr_normal_cdf(x, 0.0, 1.0) AS cdf
FROM (VALUES (-1.96::DOUBLE), (0.0), (1.96)) t(x);
```

## NULL semantics

One rule covers the whole extension: **whatever statrs cannot define comes back as SQL NULL, and a
NULL input never silently produces a number.** Concretely: NULL rows never enter an aggregate (the
SQL aggregate convention); the NAN statrs returns for an empty group, sample variance of a single
value, an out-of-range `tau`, or a negative value in the geometric / harmonic means is folded into
NULL. A parameter that is present but *invalid* (`std_dev <= 0`, a probability outside `[0, 1]`) is a
bad call, so it fails the query instead of being hidden as emptiness.

## Where to go next

- [Install and load](./getting-started/quick-start.md) — get the extension running in DuckDB.
- [Function reference](./guide/functions/overview.md) — browse all 252 functions by category.
- [Development guide](./getting-started/project-structure.md) — build from source, add functions, test, release.
