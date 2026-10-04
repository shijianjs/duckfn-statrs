---
title: Function reference
sidebar_position: 1
description: All 252 sr_ functions organized by category — aggregates for summary statistics, scalars for distributions, special functions, sampling, and more.
---

# Function reference

All functions registered by `duckfn_statrs` share the `sr_` prefix. Search them at runtime:

```sql
SELECT function_name, function_type, description
FROM duckdb_functions()
WHERE function_name LIKE 'sr_%'
ORDER BY function_name;
```

## Summary statistics (aggregates)

Accept one or two DOUBLE columns, return a single value per group. NULL rows are skipped automatically.

| Function | Description | Example |
| --- | --- | --- |
| `sr_mean(x)` | Arithmetic mean | `SELECT sr_mean(x) FROM t` |
| `sr_geometric_mean(x)` | Geometric mean (NULL if any value < 0) | |
| `sr_harmonic_mean(x)` | Harmonic mean (NULL if any value < 0) | |
| `sr_quadratic_mean(x)` | Root mean square | |
| `sr_median(x)` | Median (averages two middle values for even-length) | |
| `sr_quantile(x, tau)` | Tau quantile, tau in [0, 1] | `SELECT sr_quantile(x, 0.75) FROM t` |
| `sr_percentile(x, p)` | p-th percentile, p in 0..100 (integer) | |
| `sr_order_statistic(x, k)` | k-th smallest (1-based) | |
| `sr_lower_quartile(x)` | First quartile (Q1) | |
| `sr_upper_quartile(x)` | Third quartile (Q3) | |
| `sr_interquartile_range(x)` | IQR (Q3 - Q1) | |
| `sr_variance(x)` | Sample variance (Bessel-corrected) | |
| `sr_std_dev(x)` | Sample standard deviation | |
| `sr_population_variance(x)` | Population variance (divide by N) | |
| `sr_population_std_dev(x)` | Population standard deviation | |
| `sr_min(x)` / `sr_max(x)` | Minimum / maximum | |
| `sr_abs_min(x)` / `sr_abs_max(x)` | Smallest / largest absolute value | |
| `sr_covariance(x, y)` | Sample covariance of two paired columns | |
| `sr_population_covariance(x, y)` | Population covariance | |
| `sr_ranks(x, method)` | Ranks as LIST(DOUBLE); method 1=avg 2=min 3=max 4=first | |
| `sr_empirical_cdf(x, v)` | Empirical CDF at constant v | |
| `sr_empirical_sf(x, v)` | Empirical survival at constant v | |
| `sr_empirical_quantile(x, p)` | Empirical quantile at constant p | |

```sql {"type":"duckfn","show":"table"}
SELECT g,
       sr_mean(x) AS mean,
       sr_median(x) AS median,
       sr_std_dev(x) AS std,
       sr_skewtest(list(x), 1.0, 1.0)[1] AS skew_z
FROM (VALUES (1, 2.5), (1, 3.1), (1, 1.8), (2, 7.2), (2, 8.1), (2, 6.9)) t(g, x)
GROUP BY g ORDER BY g;
```

## Continuous distributions (scalars)

20 distributions, each with 5 functions: `pdf`, `ln_pdf`, `cdf`, `sf`, `quantile`.

Pattern: `sr_<name>_<function>(x, ...params)`

| Distribution | Parameters | Example |
| --- | --- | --- |
| normal | mean, std_dev | `SELECT sr_normal_pdf(0.0, 0.0, 1.0)` |
| log_normal | location, scale | |
| beta | shape_a, shape_b | |
| gamma | shape, rate | |
| inverse_gamma | shape, scale | |
| chi_squared | freedom | |
| chi | freedom | |
| exponential | rate | |
| uniform | min, max | |
| students_t | location, scale, freedom | |
| fisher_snedecor | df_num, df_den | |
| cauchy | location, scale | |
| laplace | location, scale | |
| logistic | location, scale | |
| weibull | shape, scale | |
| frechet | location, scale | |
| gumbel | location, scale | |
| pareto | scale, shape | |
| levy | mu, c | |
| erlang | shape (integer), rate | |

```sql {"type":"duckfn","show":"table"}
SELECT sr_gamma_pdf(2.0, 3.0, 2.0) AS pdf,
       sr_gamma_cdf(2.0, 3.0, 2.0) AS cdf,
       sr_gamma_quantile(0.95, 3.0, 2.0) AS q95;
```

## Discrete distributions (scalars)

8 distributions, each with: `pmf`, `ln_pmf`, `cdf`, `sf`, `quantile`.

| Distribution | Parameters | Example |
| --- | --- | --- |
| bernoulli | p | `SELECT sr_bernoulli_pmf(1.0, 0.7)` |
| binomial | p, n | |
| negative_binomial | r, p | |
| poisson | lambda | |
| geometric | p | |
| hypergeometric | population, successes, draws | |
| categorical | probs (LIST) | |
| discrete_uniform | min, max | |

Integer-valued slots accept whole-number DOUBLE literals (e.g. `10.0` not `10`); non-integer values
produce an error.

## Multivariate distributions (scalars)

| Function | Description |
| --- | --- |
| `sr_multivariate_normal_pdf(x, mean, covariance)` | Density, row-major flattened covariance LIST |
| `sr_multivariate_students_t_pdf(x, location, scale, freedom)` | Density |
| `sr_dirichlet_pdf(x, alpha)` | Density on the simplex |
| `sr_dirichlet_entropy(alpha)` | Differential entropy |
| `sr_multinomial_pmf(probs, trials, counts)` | Probability mass, counts as LIST(BIGINT) |

## Special functions (scalars)

| Family | Functions |
| --- | --- |
| Error function | `sr_erf`, `sr_erfc`, `sr_erf_inv`, `sr_erfc_inv` |
| Gamma family | `sr_gamma`, `sr_ln_gamma`, `sr_digamma`, `sr_inv_digamma`, `sr_gamma_lower_incomplete`, `sr_gamma_upper_incomplete`, `sr_gamma_lower_regularized`, `sr_gamma_upper_regularized` |
| Beta family | `sr_beta`, `sr_ln_beta`, `sr_beta_incomplete`, `sr_beta_regularized`, `sr_inv_beta_regularized` |
| Factorial / combinatorial | `sr_factorial`, `sr_ln_factorial`, `sr_choose`, `sr_ln_choose` |
| Harmonic numbers | `sr_harmonic`, `sr_generalized_harmonic` |
| Logistic / logit | `sr_logistic`, `sr_logit` |
| Exponential integral | `sr_exponential_integral` |
| Polynomial | `sr_polynomial(x, coefficients)` |
| Kernel functions | `sr_kernel_eval(x, kind)`, `sr_kernel_support(kind)` |

```sql {"type":"duckfn","show":"table"}
SELECT sr_erf(1.0) AS erf_val,
       sr_gamma(5.0) AS gamma_5,
       sr_ln_choose(100.0, 50.0) AS ln_binom;
```

## Constants (zero-argument scalars)

| Function | Value |
| --- | --- |
| `sr_ln_pi()` | ln(PI) |
| `sr_sqrt_2pi()` | sqrt(2*PI) |
| `sr_ln_sqrt_2pi()` | ln(sqrt(2*PI)) |
| `sr_ln_sqrt_2pie()` | ln(sqrt(2*PI*e)) |
| `sr_2_sqrt_e_over_pi()` | 2*sqrt(e/PI) |
| `sr_ln_2_sqrt_e_over_pi()` | ln(2*sqrt(e/PI)) |
| `sr_euler_mascheroni()` | Euler-Mascheroni constant |
| `sr_frac_1_sqrt_pi()` | 1/sqrt(PI) |
| `sr_ln_2()` | ln(2) |

## Random sampling (scalars)

Draw k random samples from any distribution into a LIST(DOUBLE).

Pattern: `sr_sample_<distribution>(...params, k)` where k is a BIGINT.

```sql {"type":"duckfn","show":"table"}
SELECT len(sr_sample_normal(0.0, 1.0, 100)) AS n,
       sr_mean(x) AS sample_mean
FROM (SELECT unnest(sr_sample_normal(5.0, 2.0, 1000)) AS x);
```

All 27 distributions have a sampler. `sr_sample_empirical(x, k)` (aggregate) samples from a column.
`sr_sample_binomial_algorithm(p, n, algorithm, k)` selects the algorithm explicitly.

## Density estimation (scalars)

| Function | Description |
| --- | --- |
| `sr_kde_pdf(x, sample, bandwidth)` | Gaussian kernel density estimate; NULL bandwidth uses auto-selection |
| `sr_knn_pdf(x, sample, bandwidth)` | k-nearest-neighbour density estimate |

## Signal generation (scalars)

| Function | Description |
| --- | --- |
| `sr_gen_sinusoidal(k, sample_rate, freq, amp, mean, phase, delay)` | First k points of a sinusoid |
| `sr_gen_square(k, high, low, amplitude, offset, delay)` | Square wave |
| `sr_gen_triangle(k, raise, fall, amplitude, offset, delay)` | Triangle wave |
| `sr_gen_sawtooth(k, period, amplitude, offset, delay)` | Sawtooth wave |
| `sr_gen_periodic(k, sample_rate, freq, amplitude, phase, delay)` | Periodic signal |
| `sr_gen_log_spaced(n, start_exp, stop_exp)` | Log-spaced sequence as LIST |

## Hypothesis tests (scalars)

| Function | Description |
| --- | --- |
| `sr_ttest_onesample(sample, popmean, alternative, nan_policy)` | One-sample t-test → LIST [t, p-value] |
| `sr_mannwhitneyu(sample1, sample2, alternative, method)` | Mann-Whitney U → LIST [U, p-value] |
| `sr_ks_twosample(sample1, sample2, alternative, method)` | Two-sample KS → LIST [D, p-value] |
| `sr_chisquare(observed, expected, ddof)` | Chi-square GoF → LIST [stat, p-value] |
| `sr_f_oneway(groups, nan_policy)` | One-way ANOVA → LIST [F, p-value] |
| `sr_skewtest(sample, alternative, nan_policy)` | Skewness z-test → LIST [z, p-value] |
| `sr_anderson_darling(sample, distribution, params)` | Anderson-Darling GoF → LIST [A², critical] |
| `sr_fishers_exact(table, alternative)` | Fisher's exact test → p-value |
| `sr_fishers_exact_with_odds_ratio(table, alternative)` | → LIST [odds_ratio, p-value] |

Samples are passed as `LIST(DOUBLE)` literals or built with `list(x)` from a column. The `alternative`
and `nan_policy` codes are DOUBLE literals (e.g. 1.0 = two-sided, 2.0 = less, 3.0 = greater).

## Finding a function

Use `duckdb_functions()` to search programmatically:

```sql
SELECT function_name, function_type, description
FROM duckdb_functions()
WHERE function_name LIKE 'sr_normal%'
ORDER BY function_name;
```
