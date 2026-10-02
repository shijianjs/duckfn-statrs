---
title: Introduction
sidebar_position: 1
slug: /intro
description: duckfn_statrs wraps the Rust statrs crate as DuckDB functions — aggregates for summary statistics, scalars for the normal distribution.
---

# Introduction

`duckfn_statrs` is a DuckDB [loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)
that wraps the Rust statistical computing library [statrs](https://crates.io/crates/statrs) into SQL
functions. The summary statistics are **aggregates** over a DOUBLE column (`SELECT sr_mean(x) FROM t
GROUP BY g`), and the normal distribution's pdf / cdf / quantile are **scalars**. The code is written
with [duckfn](https://crates.io/crates/duckfn) attribute macros on top of
[duckfn-extension-template](https://github.com/shijianjs/duckfn-extension-template) — the repository
already contains the build, test, documentation and release tooling, and DuckDB's C API is used
headers-only, so nothing has to be built except the extension itself.

Every computation is delegated to statrs: the extension re-implements no statistical formula. The
summary aggregates share one state — which statistic to compute is dispatched statically by a type
parameter (no code generation, no `macro_rules!`):

```rust
trait Summary {
    fn eval(values: &[f64]) -> f64;
}

#[derive(Default, Debug, Clone)]
struct SummaryState<S: Summary> {
    values: Vec<f64>,
    _marker: PhantomData<S>,
}

impl<S: Summary> DuckAggregateState for SummaryState<S> {
    type Output = f64;

    fn simple_combine(&mut self, other: &Self) {
        self.values.extend(other.values.iter().copied());
    }

    fn result(&self) -> DuckOptionResult<f64> {
        nan_to_null(S::eval(&self.values))  // statrs' NAN becomes SQL NULL
    }
}

#[duck_aggregate_function(/* description / example */)]
fn sr_mean(input: f64, state: &mut SummaryState<ArithmeticMean>) {
    state.values.push(input);
}
```

The same functions from SQL. This block runs in your browser: the site preloads the extension from the
repository's latest release, so there is no `LOAD` to write here.

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS std_dev
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 10.0), (2, 20.0)) t(g, x)
GROUP BY g ORDER BY g;
```

## NULL semantics

One rule covers the whole extension: **whatever statrs cannot define comes back as SQL NULL, and a
NULL input never silently produces a number.** Concretely: NULL rows never enter an aggregate (the
SQL aggregate convention); the NAN statrs returns for an empty group, sample variance of a single
value, an out-of-range `tau`, or a negative value in the geometric / harmonic means is folded into
NULL. A parameter that is present but *invalid* (`std_dev <= 0`, a probability outside `[0, 1]`) is a
bad call, so it fails the query instead of being hidden as emptiness.

## What is in the box

| Path | What it is |
| --- | --- |
| `src/extension/mod.rs` | The entry point: `duckfn_entrypoint!("duckfn_statrs")` plus the module tree. |
| `src/extension/functions/` | The registered functions: `aggregate_summary.rs` (the statistics), `aggregate_covariance.rs`, `scalar_normal.rs` (the distribution). |
| `src/extension/types/` | Where SQL-facing types go (STRUCT/ENUM definitions, `list<struct>` row types). Empty for now. |
| `test/sql/` | SQLLogicTest files, one per function group plus a smoke test; expectations are statrs' actual output. |
| `Justfile` | The everyday commands: build, run SQL, repl, test, lint, release. |
| `.github/workflows/` | The build matrix, the GitHub Release on a version tag, and this site's deployment. |
| `community-extension/` | The two files a [community extension](https://duckdb.org/community_extensions/list_of_extensions) registration needs. |
| `docs/` | This site: Docusaurus, English and Simplified Chinese. |

## Where to go next

- [Quick start](./getting-started/quick-start.md) — build it, load it, call it.
- [Project structure](./getting-started/project-structure.md) — where the entry point, the functions and
  the types live, and the naming rules that hold them together.
- [Writing functions](./guide/functions.md) — the registered functions and the shapes they use.
- [Testing](./guide/testing.md) — the SQLLogicTest files and how to run them.
- [Build and release](./build-and-release.md) — the build paths and the release flow.
- [Community extensions](./community-extension.md) — publishing to DuckDB's community repository.
