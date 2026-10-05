---
title: Hypothesis tests
sidebar_position: 7
description: Ten statistical hypothesis tests — t-test, chi-square, ANOVA, KS, Mann-Whitney U, skewness, Anderson-Darling, Fisher's exact.
---

# Hypothesis tests

Each test returns a `LIST(DOUBLE)` of the form `[statistic, p-value]` unless noted. Sample
arguments are `LIST(DOUBLE)`. Coded arguments (`alternative`, `nan_policy`, `method`) are DOUBLE
literals matching statrs' enum variants.

## sr_ttest_onesample(sample, popmean, alternative, nan_policy)

**Signature**: `sr_ttest_onesample(sample DOUBLE[], popmean DOUBLE, alternative DOUBLE, nan_policy DOUBLE) -> LIST(DOUBLE)`

One-sample t-test against `popmean`. Returns `[t_statistic, p_value]`.

- `sample`: observations
- `popmean`: hypothesised population mean
- `alternative`: 1.0 = two-sided, 2.0 = less, 3.0 = greater
- `nan_policy`: 1.0 = propagate, 2.0 = omit, 3.0 = raise

```sql {"type":"duckfn","show":"value"}
SELECT sr_ttest_onesample([1.0, 2.0, 3.0, 4.0, 5.0], 3.0, 1.0, 1.0)
-- [0.0, 1.0]
```

## sr_chisquare(observed, expected, ddof)

**Signature**: `sr_chisquare(observed UBIGINT[], expected DOUBLE[], ddof UBIGINT) -> LIST(DOUBLE)`

Chi-square goodness-of-fit test. Pass NULL for `expected` to assume uniform frequencies; pass
NULL for `ddof` to default to 0.

Returns `[chi_statistic, p_value]`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_chisquare([16::UBIGINT, 18::UBIGINT, 16::UBIGINT, 14::UBIGINT, 12::UBIGINT, 12::UBIGINT], NULL, NULL)
-- [2.0, 0.8491450360846101]
```

## sr_f_oneway(groups, nan_policy)

**Signature**: `sr_f_oneway(groups DOUBLE[][], nan_policy DOUBLE) -> LIST(DOUBLE)`

One-way ANOVA across a list of sample lists. Returns `[F_statistic, p_value]`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_f_oneway([[1.0, 3.0, 5.0], [2.0, 4.0, 8.0]], 1.0)
-- [0.6249999999999984, 0.47342736525713647]
```

## sr_fishers_exact(table, alternative)

**Signature**: `sr_fishers_exact(table UBIGINT[], alternative DOUBLE) -> DOUBLE`

Fisher's exact test p-value on a 2x2 contingency table (4 UBIGINTs, row-major).
Returns just the p-value (a DOUBLE, not a LIST).

- `alternative`: 1.0 = two-sided, 2.0 = less, 3.0 = greater

```sql {"type":"duckfn","show":"value"}
SELECT sr_fishers_exact([1::UBIGINT, 2::UBIGINT, 3::UBIGINT, 4::UBIGINT], 1.0)
-- 1.0
```

## sr_fishers_exact_with_odds_ratio(table, alternative)

**Signature**: `sr_fishers_exact_with_odds_ratio(table UBIGINT[], alternative DOUBLE) -> LIST(DOUBLE)`

Same as `sr_fishers_exact`, but returns `[odds_ratio, p_value]`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_fishers_exact_with_odds_ratio([1::UBIGINT, 2::UBIGINT, 3::UBIGINT, 4::UBIGINT], 2.0)
-- [0.6666666666666666, 0.6666666666666666]
```

## sr_ks_twosample(sample1, sample2, alternative, method)

**Signature**: `sr_ks_twosample(sample1 DOUBLE[], sample2 DOUBLE[], alternative DOUBLE, method DOUBLE) -> LIST(DOUBLE)`

Two-sample Kolmogorov-Smirnov test. Returns `[D_statistic, p_value]`.

- `alternative`: 1.0 = less, 2.0 = greater, 3.0 = two-sided-exact, 4.0 = two-sided-asymptotic
- `method`: mode selector; usually 1.0 (auto)

```sql {"type":"duckfn","show":"value"}
SELECT sr_ks_twosample([0.1, 0.2, 0.3], [0.4, 0.5, 0.6], 4.0, 1.0)
-- [1.0, 0.09956184831478034]
```

### sr_ks_onesample(x, dist, params, method, nan)

**Signature**: `sr_ks_onesample(x DOUBLE[], dist VARCHAR, params DOUBLE[], method DOUBLE, nan DOUBLE) -> DOUBLE[]`

One-sample Kolmogorov-Smirnov test of a LIST(DOUBLE) sample against a named distribution (normal / lognormal / exponential / gumbel / weibull / uniform) with its parameter LIST: LIST [KS statistic, p-value]; method 1 less 2 greater 3 two-sided exact 4 two-sided asymptotic 5 two-sided approximate.

```sql {"type":"duckfn","show":"value"}
SELECT sr_ks_onesample([1.0, 2.0, 3.0, 4.0], 'normal', [2.5, 1.0], 4.0, 1.0)
-- [0.19146246127401312, 0.9985479186818191]
```

## sr_mannwhitneyu(sample1, sample2, alternative, method)

**Signature**: `sr_mannwhitneyu(sample1 DOUBLE[], sample2 DOUBLE[], alternative DOUBLE, method DOUBLE) -> LIST(DOUBLE)`

Mann-Whitney U test. Returns `[U_statistic, p_value]`.

- `alternative`: 1.0 = two-sided, 2.0 = less, 3.0 = greater
- `method`: 1.0 = auto, 2.0 = exact, 3.0 = asymptotic with continuity correction, 4.0 = asymptotic without

```sql {"type":"duckfn","show":"value"}
SELECT sr_mannwhitneyu([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 2.0, 2.0)
-- [0.0, 0.95]
```

## sr_skewtest(sample, alternative, nan_policy)

**Signature**: `sr_skewtest(sample DOUBLE[], alternative DOUBLE, nan_policy DOUBLE) -> LIST(DOUBLE)`

Skewness z-test. Returns `[z_statistic, p_value]`.

- `alternative`: 1.0 = two-sided, 2.0 = less (negatively skewed), 3.0 = greater (positively skewed)
- `nan_policy`: same codes as ttest

```sql {"type":"duckfn","show":"value"}
SELECT sr_skewtest([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 1.0, 1.0)
-- [1.018464355396213, 0.30845733197774927]
```

## sr_anderson_darling(sample, distribution, params)

**Signature**: `sr_anderson_darling(sample DOUBLE[], distribution VARCHAR, params DOUBLE[]) -> LIST(DOUBLE)`

Anderson-Darling goodness-of-fit test against a named distribution. Returns
`[A_squared, critical_value_5_percent]`.

- `distribution`: one of `'normal'`, `'lognormal'`, `'exponential'`, `'gumbel'`, `'weibull'`, `'uniform'`
- `params`: distribution parameter list (e.g. `[mean, std_dev]` for normal)

```sql {"type":"duckfn","show":"value"}
SELECT sr_anderson_darling([1.0, 2.0, 3.0, 4.0, 5.0], 'normal', [3.0, 1.5])
-- [0.15491765936161173, 0.8959843708415298]
```

## Building a sample LIST from a column

When the data lives in a table, use `list(x)` to feed a test:

```sql {"type":"duckfn","show":"value"}
SELECT sr_ttest_onesample(list(x), 3.0, 1.0, 1.0)
FROM (VALUES (1.0), (2.0), (3.0), (4.0), (5.0)) t(x)
-- [0.0, 1.0]
```

## Invalid codes raise query errors

```sql {"type":"duckfn","expect":"error"}
SELECT sr_ttest_onesample([1.0, 2.0], 3.0, 9.0, 1.0)
-- error: sr_ttest_onesample: the alternative must be 1 = two-sided, 2 = less, 3 = greater (statrs' Alternative), got 9
```
