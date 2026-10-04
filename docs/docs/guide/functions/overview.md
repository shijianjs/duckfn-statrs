---
title: Overview
sidebar_position: 1
description: Index of all function reference pages — 252 statistical functions with executable SQL examples.
---

# Function reference overview

All 252 functions registered by `duckfn_statrs` share the `sr_` prefix. Every function in the
sub-pages below includes at least one executable SQL example that runs directly in your browser.

Search all registered functions at runtime:

```sql
SELECT function_name, function_type, description
FROM duckdb_functions()
WHERE function_name LIKE 'sr_%'
ORDER BY function_name;
```

## How this section is organised

The sidebar mirrors the function families, with distributions split by kind:

- **Summary statistics** — descriptive aggregates and the empirical distribution.
- **Distributions**
  - **Continuous** — normal / log-normal, the gamma family, beta, location-scale, extreme value
    and heavy tail, Student's t and Fisher-Snedecor, uniform / triangular / Dirac.
  - **Discrete** — Bernoulli and binomial trials, Poisson and hypergeometric, categorical and
    discrete uniform.
  - **Multivariate** — multivariate normal, Dirichlet and multinomial.
- **Special functions** — error / gamma / beta families, factorials, harmonic numbers,
  logistic/logit, polynomial and kernel helpers — plus the **Constants**.
- **Random sampling** — continuous, and discrete / multivariate / empirical samplers.
- **Density and signals**, **Hypothesis tests**.

Each page gives every function its signature, parameter types and at least one executable SQL
example that runs in your browser.

## NULL semantics (applies everywhere)

- NULL input rows never enter an aggregate; scalars short-circuit to NULL.
- Results that statrs cannot define (empty group, insufficient samples, out-of-range parameter)
  become SQL NULL — never NAN.
- A parameter that is present but invalid (e.g. `std_dev <= 0`) raises a query error.

## Common patterns

All continuous distributions provide 5 functions per distribution:

```text
sr_<name>_pdf(x, params...)    probability density
sr_<name>_ln_pdf(x, params...) log-density
sr_<name>_cdf(x, params...)    cumulative distribution P(X <= x)
sr_<name>_sf(x, params...)     survival function P(X > x)
sr_<name>_quantile(p, params...) inverse CDF
```

All discrete distributions substitute `pmf`/`ln_pmf` for `pdf`/`ln_pdf`.
