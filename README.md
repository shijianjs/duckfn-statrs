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
make configure   # once: the configure/venv (Python + the sqllogictest runner)
make debug       # -> build/debug/duckfn_statrs.duckdb_extension
```

Self-built artifacts are unsigned, so DuckDB needs `-unsigned` to load them:

```shell
duckdb -unsigned -c "
LOAD './build/debug/duckfn_statrs.duckdb_extension';
SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);
-- 2.0
SELECT sr_normal_cdf(1.96, 0.0, 1.0);
-- 0.9750021048529024
"
```

The `Justfile` wraps the same commands: `just build`, `just sql "SELECT sr_mean(x) FROM range(10) t(x)"`,
`just repl` (a REPL with the extension loaded).

## Functions

Every registered name carries the `sr_` prefix, so the whole set is one `duckdb_functions()` filter
away; the code tree mirrors statrs' module tree (`functions/{consts,function,statistics,distribution}/`)
and the correspondence table is in `src/extension/functions/mod.rs`. Currently 22 aggregates +
166 scalars, by family:

**Aggregates** (a DOUBLE column in, one DOUBLE out, all `auto_collect`):

| Family | Functions |
| --- | --- |
| Central tendency | `sr_mean` / `sr_geometric_mean` / `sr_harmonic_mean` / `sr_quadratic_mean` |
| Order statistics | `sr_median` / `sr_quantile(x, tau)` / `sr_order_statistic(x, k)` / `sr_percentile(x, p)` / `sr_lower_quartile` / `sr_upper_quartile` / `sr_interquartile_range` / `sr_ranks(x, method)` (returns LIST) |
| Dispersion | `sr_variance` / `sr_std_dev` / `sr_population_variance` / `sr_population_std_dev` |
| Extremes & pairing | `sr_min` / `sr_max` / `sr_abs_min` / `sr_abs_max`; `sr_covariance(x, y)` / `sr_population_covariance(x, y)` |

**Scalars** (row by row, every parameter and result is DOUBLE):

| Family | Functions |
| --- | --- |
| Continuous distributions (20) | per distribution `sr_<dist>_pdf / ln_pdf / cdf / sf / quantile` (normal, log_normal, beta, gamma, chi_squared, students_t, uniform, weibull, pareto, …) |
| Discrete distributions (7) | per distribution `sr_<dist>_pmf / ln_pmf / cdf / sf / quantile` (binomial, poisson, geometric, hypergeometric, …) |
| Special functions | `sr_erf` / `sr_erfc` / `sr_gamma` / `sr_ln_gamma` / `sr_digamma` / `sr_beta` / `sr_beta_regularized` / the four incomplete-Gamma variants / `sr_factorial` / `sr_choose` / `sr_harmonic` / `sr_logistic` / `sr_logit` … |
| Constants | `sr_sqrt_2pi()` / `sr_euler_mascheroni()` and friends (statrs::consts) |

What statrs deliberately does not wrap (multivariate / Categorical, samplers, KDE, the hypothesis
test module) and why are recorded in the headers of `src/extension/functions/mod.rs` and
`distribution/mod.rs`.

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

The official DuckDB `extension-ci-tools` makefiles are the build path:

```shell
make configure           # once: the configure/venv (Python + the sqllogictest runner)
make debug               # the official path -> build/debug/duckfn_statrs.duckdb_extension
```

`make release` is the same flow with optimizations. On Windows `make` needs Git Bash.
The `Justfile` wraps it (`just build` = `make configure && make debug`, `just ci-build`, `just test`,
`just ci-release`).

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
