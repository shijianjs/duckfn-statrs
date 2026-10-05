---
title: Overview
sidebar_position: 1
description: Index of all function reference pages — 537 statistical functions with executable SQL examples.
---

# Function reference overview

All 537 functions registered by `duckfn_statrs` share the `sr_` prefix. Every function in the
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
  logistic/logit, polynomial and kernel helpers (kernels are selected by name, e.g.
  `sr_kernel_eval('gaussian', 0.0)`) — plus the **Constants**: the mathematical constants and
  statrs' own floating-point precision thresholds with `sr_almost_eq`.
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

On top of those five, each distribution also exposes the quantities statrs can define — mean,
variance, standard deviation, entropy, skewness, median, mode and the support bounds:

```text
sr_<name>_mean(params...)      distribution mean
sr_<name>_variance(params...)  variance
sr_<name>_std_dev(params...)   standard deviation
sr_<name>_entropy(params...)   differential / Shannon entropy
sr_<name>_skewness(params...)  skewness
sr_<name>_median(params...)    median
sr_<name>_mode(params...)      mode
sr_<name>_min(params...)       lower support bound
sr_<name>_max(params...)       upper support bound
```

Where statrs has no closed form (or none exists), the function is still registered and returns
SQL NULL — for example the mean and variance of a Cauchy distribution.
