[English](README.md) | [简体中文](README.zh.md)

# duckfn_statrs

A DuckDB [loadable extension](https://duckdb.org/docs/stable/extensions/extension_development) that
wraps the Rust statistical computing library [statrs](https://crates.io/crates/statrs) into functions
callable straight from SQL: the descriptive statistics are **aggregate functions** (written as
`SELECT sr_mean(x) FROM t GROUP BY g`), while the normal distribution's pdf / cdf / quantile are
**scalar functions** (evaluated row by row). The extension is written with
[duckfn](https://crates.io/crates/duckfn) attribute macros, and the C++ build is not involved at all.

Every computation is delegated to statrs — this extension re-implements no statistical formula; it
only feeds SQL values in and hands SQL results back out.

## Quick start

```shell
cargo install cargo-duckdb-ext-tools   # once: a global cargo subcommand, no project dependency
cargo duckdb-ext build                 # -> target/debug/duckfn_statrs.duckdb_extension
```

Self-built artifacts are unsigned, so DuckDB needs `-unsigned` to load them:

```shell
duckdb -unsigned -c "
LOAD './target/debug/duckfn_statrs.duckdb_extension';
SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);
-- 2.0
SELECT sr_normal_cdf(1.96, 0.0, 1.0);
-- 0.9750021048529024
"
```

The `Justfile` wraps the same commands: `just build`, `just sql "SELECT sr_mean(x) FROM range(10) t(x)"`,
`just repl` (a REPL with the extension loaded).

## Functions

Every registered name carries the `sr_` prefix, so the whole set is one `duckdb_functions()` filter away.

**Aggregates** (a DOUBLE column in, one DOUBLE out):

| Function | Notes |
| --- | --- |
| `sr_mean(x)` | arithmetic mean |
| `sr_geometric_mean(x)` | geometric mean (undefined with a negative value → `NULL`) |
| `sr_harmonic_mean(x)` | harmonic mean (undefined with a negative value → `NULL`) |
| `sr_quadratic_mean(x)` | quadratic mean (RMS) |
| `sr_median(x)` | median (even lengths average the two middle values) |
| `sr_quantile(x, tau)` | tau quantile; write tau as the second, constant argument: `sr_quantile(x, 0.975)` |
| `sr_variance(x)` / `sr_std_dev(x)` | sample variance / standard deviation (Bessel-corrected, `NULL` under two values) |
| `sr_population_variance(x)` / `sr_population_std_dev(x)` | population variance / standard deviation (dividing by N) |
| `sr_covariance(x, y)` / `sr_population_covariance(x, y)` | sample / population covariance of two row-paired columns |

**Scalars** (the normal distribution, row by row):

| Function | Notes |
| --- | --- |
| `sr_normal_pdf(x, mean, std_dev)` | probability density |
| `sr_normal_cdf(x, mean, std_dev)` | cumulative distribution P(X ≤ x) |
| `sr_normal_quantile(p, mean, std_dev)` | quantile function (the inverse CDF) |

## NULL and error semantics

Everything follows statrs and propagates to the SQL side:

- **NULL inputs**: a NULL row never enters an aggregate's state (the SQL aggregate convention, same
  as DuckDB's own `mean`/`stddev`); in a scalar, a row with a NULL in any argument short-circuits to
  `NULL`. For the covariances a NULL in either column skips the whole row, keeping the two columns paired.
- **What statrs cannot define** (an empty group, sample variance of one value, an out-of-range tau, a
  negative value in the geometric/harmonic means) comes back as NAN, and the extension folds every NAN
  into `NULL` — a NAN never reaches the user as a value.
- **A parameter that is present but invalid** (`std_dev <= 0`, a probability outside `[0, 1]`) is a bad
  call: it fails the query instead of being silently folded into emptiness.

## Building from source

Two build paths, deliberately kept in sync:

```shell
cargo duckdb-ext build   # day-to-day, no make -> target/debug/duckfn_statrs.duckdb_extension
make configure           # once: the configure/venv (Python + the sqllogictest runner)
make debug               # the official-template path -> build/debug/extension/duckfn_statrs/...
```

`make release` is the same flow with optimizations. On Windows `make` needs Git Bash.
The `Justfile` wraps both (`just build`, `just ci-build`, `just test`, `just ci-release`).

## Testing

The SQLLogicTest files live under `test/sql/`, one per function group:
`aggregate_summary.test` (the statistics aggregates), `aggregate_covariance.test`,
`scalar_normal.test` (the normal distribution), `duckfn_statrs.test` (smoke + the registration
census). Every expected value is statrs' actual output, not a hand-computed approximation.

```shell
just test          # make configure + make debug + make test
just ci-build      # the official build only, no tests
```

`make test` does not rebuild: run `just ci-build` (or `make debug`) after changing Rust.
Faster iteration loops are in [DEVELOPMENT.md](DEVELOPMENT.md).

## WebAssembly

```shell
just config_env   # once: pin the toolchain and add the wasm target
just build_wasm
```

The wasm build goes through `src/wasm_lib.rs` (the `staticlib` mirror of `src/lib.rs`); both crate
roots must always declare the same `mod` list. statrs is pure Rust, so it rides along fine.

## Documentation site

The repository carries a [Docusaurus](https://docusaurus.io/) site (`docs/`, English + Simplified
Chinese), published to GitHub Pages on every version tag:

```shell
just docs_install    # once
just docs_start      # local preview at http://localhost:3000
just docs_build      # the one that matters: onBrokenLinks throws, a broken link fails the build
```

Pages carry runnable SQL blocks (powered by
[`duckfn-docs-kit`](https://www.npmjs.com/package/duckfn-docs-kit)) that call this extension right in
the browser; `cd docs && npm test` re-runs them. Conventions (layout, commands, translation flow,
deployment, the `{{EXTENSION_VERSION}}` placeholder) are in [`docs/README.md`](docs/README.md).

## Installing a released extension

Releases ship the `.duckdb_extension` built for each platform:

```shell
duckdb -unsigned -c "
LOAD 'https://github.com/shijianjs/duckfn-statrs/releases/latest/download/duckfn_statrs-windows_amd64.duckdb_extension';
"
```

Once registered in the [community extensions](https://duckdb.org/community_extensions/list_of_extensions),
this becomes `INSTALL duckfn_statrs FROM community`; the two files that submission needs are staged
under [`community-extension/`](community-extension/AGENTS.md).

## Documentation

| File | Contents |
| --- | --- |
| [AGENTS.md](AGENTS.md) | conventions, the duckfn knowledge map, the release flow |
| [DEVELOPMENT.md](DEVELOPMENT.md) | layout, design trade-offs, build & test, description export |
| [docs/README.md](docs/README.md) | the documentation site: layout, commands, translation, deployment |
| [README.zh.md](README.zh.md) | this file, in Simplified Chinese |

This repository started from [duckfn-extension-template](https://github.com/shijianjs/duckfn-extension-template);
the repository conventions (the five places the extension name must match, `just rename`, the shared
justfile, the release flow) are documented in [AGENTS.md](AGENTS.md).
