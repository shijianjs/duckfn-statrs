---
title: 函数参考
sidebar_position: 1
description: 按类别组织的 252 个 sr_ 函数——聚合统计量、分布、特殊函数、抽样、假设检验等。
---

# 函数参考

`duckfn_statrs` 注册的全部函数共享 `sr_` 前缀。在 SQL 中检索：

```sql
SELECT function_name, function_type, description
FROM duckdb_functions()
WHERE function_name LIKE 'sr_%'
ORDER BY function_name;
```

## 描述统计量（聚合函数）

接受一或两个 DOUBLE 列，每组返回一个值。NULL 行自动跳过。

| 函数 | 说明 | 示例 |
| --- | --- | --- |
| `sr_mean(x)` | 算术均值 | `SELECT sr_mean(x) FROM t` |
| `sr_geometric_mean(x)` | 几何均值（含负值则 NULL） | |
| `sr_harmonic_mean(x)` | 调和均值（含负值则 NULL） | |
| `sr_quadratic_mean(x)` | 均方根 | |
| `sr_median(x)` | 中位数（偶数个取两中间均值） | |
| `sr_quantile(x, tau)` | tau 分位数，tau 在 [0, 1] | `SELECT sr_quantile(x, 0.75) FROM t` |
| `sr_percentile(x, p)` | 第 p 百分位，p 为 0–100 整数 | |
| `sr_order_statistic(x, k)` | 第 k 小值（从 1 起） | |
| `sr_lower_quartile(x)` | 下四分位 Q1 | |
| `sr_upper_quartile(x)` | 上四分位 Q3 | |
| `sr_interquartile_range(x)` | 四分位距 IQR | |
| `sr_variance(x)` | 样本方差（Bessel 校正） | |
| `sr_std_dev(x)` | 样本标准差 | |
| `sr_population_variance(x)` | 总体方差（除以 N） | |
| `sr_population_std_dev(x)` | 总体标准差 | |
| `sr_min(x)` / `sr_max(x)` | 最小值 / 最大值 | |
| `sr_abs_min(x)` / `sr_abs_max(x)` | 绝对值最小 / 最大 | |
| `sr_covariance(x, y)` | 两列样本协方差 | |
| `sr_population_covariance(x, y)` | 两列总体协方差 | |
| `sr_ranks(x, method)` | 秩（出 LIST），method 1=均值 2=min 3=max 4=首位 | |
| `sr_empirical_cdf(x, v)` | 经验 CDF 在常量 v 处 | |
| `sr_empirical_sf(x, v)` | 经验生存函数在常量 v 处 | |
| `sr_empirical_quantile(x, p)` | 经验分位数在常量 p 处 | |

```sql {"type":"duckfn","show":"table"}
SELECT g,
       sr_mean(x) AS mean,
       sr_median(x) AS median,
       sr_std_dev(x) AS std
FROM (VALUES (1, 2.5), (1, 3.1), (1, 1.8), (2, 7.2), (2, 8.1), (2, 6.9)) t(g, x)
GROUP BY g ORDER BY g;
```

## 连续分布（标量函数）

20 种分布，每种 5 个函数：`pdf`、`ln_pdf`、`cdf`、`sf`、`quantile`。

命名模式：`sr_<分布名>_<函数>(x, ...参数)`

| 分布 | 参数 | 示例 |
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
| erlang | shape（整数）, rate | |

```sql {"type":"duckfn","show":"table"}
SELECT sr_gamma_pdf(2.0, 3.0, 2.0) AS pdf,
       sr_gamma_cdf(2.0, 3.0, 2.0) AS cdf,
       sr_gamma_quantile(0.95, 3.0, 2.0) AS q95;
```

## 离散分布（标量函数）

8 种分布，每种：`pmf`、`ln_pmf`、`cdf`、`sf`、`quantile`。

| 分布 | 参数 | 示例 |
| --- | --- | --- |
| bernoulli | p | `SELECT sr_bernoulli_pmf(1.0, 0.7)` |
| binomial | p, n | |
| negative_binomial | r, p | |
| poisson | lambda | |
| geometric | p | |
| hypergeometric | population, successes, draws | |
| categorical | probs（LIST） | |
| discrete_uniform | min, max | |

整数槽位接受整数值 DOUBLE 字面量（如 `10.0`），非整数会报错。

## 多元分布（标量函数）

| 函数 | 说明 |
| --- | --- |
| `sr_multivariate_normal_pdf(x, mean, covariance)` | 密度，协方差矩阵行主序摊平 LIST |
| `sr_multivariate_students_t_pdf(x, location, scale, freedom)` | 密度 |
| `sr_dirichlet_pdf(x, alpha)` | 单纯形上的密度 |
| `sr_dirichlet_entropy(alpha)` | 微分熵 |
| `sr_multinomial_pmf(probs, trials, counts)` | 概率质量，counts 为 LIST(BIGINT) |

## 特殊函数（标量函数）

| 族 | 函数 |
| --- | --- |
| 误差函数 | `sr_erf`、`sr_erfc`、`sr_erf_inv`、`sr_erfc_inv` |
| Gamma 族 | `sr_gamma`、`sr_ln_gamma`、`sr_digamma`、`sr_inv_digamma`、`sr_gamma_lower_incomplete`、`sr_gamma_upper_incomplete`、`sr_gamma_lower_regularized`、`sr_gamma_upper_regularized` |
| Beta 族 | `sr_beta`、`sr_ln_beta`、`sr_beta_incomplete`、`sr_beta_regularized`、`sr_inv_beta_regularized` |
| 阶乘 / 组合 | `sr_factorial`、`sr_ln_factorial`、`sr_choose`、`sr_ln_choose` |
| 调和数 | `sr_harmonic`、`sr_generalized_harmonic` |
| Logistic / logit | `sr_logistic`、`sr_logit` |
| 指数积分 | `sr_exponential_integral` |
| 多项式 | `sr_polynomial(x, coefficients)` |
| 核函数 | `sr_kernel_eval(x, kind)`、`sr_kernel_support(kind)` |

```sql {"type":"duckfn","show":"table"}
SELECT sr_erf(1.0) AS erf_val,
       sr_gamma(5.0) AS gamma_5,
       sr_ln_choose(100.0, 50.0) AS ln_binom;
```

## 常量（零参标量函数）

| 函数 | 值 |
| --- | --- |
| `sr_ln_pi()` | ln(PI) |
| `sr_sqrt_2pi()` | sqrt(2*PI) |
| `sr_ln_sqrt_2pi()` | ln(sqrt(2*PI)) |
| `sr_ln_sqrt_2pie()` | ln(sqrt(2*PI*e)) |
| `sr_2_sqrt_e_over_pi()` | 2*sqrt(e/PI) |
| `sr_ln_2_sqrt_e_over_pi()` | ln(2*sqrt(e/PI)) |
| `sr_euler_mascheroni()` | 欧拉-马斯刻若尼常数 |
| `sr_frac_1_sqrt_pi()` | 1/sqrt(PI) |
| `sr_ln_2()` | ln(2) |

## 随机抽样（标量函数）

从任意分布抽取 k 个随机样本，返回 LIST(DOUBLE)。

命名模式：`sr_sample_<分布>(...参数, k)`，k 为 BIGINT。

```sql {"type":"duckfn","show":"table"}
SELECT len(sr_sample_normal(0.0, 1.0, 100)) AS n,
       sr_mean(x) AS sample_mean
FROM (SELECT unnest(sr_sample_normal(5.0, 2.0, 1000)) AS x);
```

全部 27 种分布都有对应的采样器。`sr_sample_empirical(x, k)`（聚合）从列中抽样。
`sr_sample_binomial_algorithm(p, n, algorithm, k)` 可显式选算法。

## 密度估计（标量函数）

| 函数 | 说明 |
| --- | --- |
| `sr_kde_pdf(x, sample, bandwidth)` | 高斯核密度估计；带宽 NULL 时自动选择 |
| `sr_knn_pdf(x, sample, bandwidth)` | k 近邻密度估计 |

## 信号生成（标量函数）

| 函数 | 说明 |
| --- | --- |
| `sr_gen_sinusoidal(k, sample_rate, freq, amp, mean, phase, delay)` | 前 k 点正弦波 |
| `sr_gen_square(k, high, low, amplitude, offset, delay)` | 方波 |
| `sr_gen_triangle(k, raise, fall, amplitude, offset, delay)` | 三角波 |
| `sr_gen_sawtooth(k, period, amplitude, offset, delay)` | 锯齿波 |
| `sr_gen_periodic(k, sample_rate, freq, amplitude, phase, delay)` | 周期信号 |
| `sr_gen_log_spaced(n, start_exp, stop_exp)` | 对数间距序列（LIST） |

## 假设检验（标量函数）

| 函数 | 说明 |
| --- | --- |
| `sr_ttest_onesample(sample, popmean, alternative, nan_policy)` | 单样本 t 检验 → LIST [t, p-value] |
| `sr_mannwhitneyu(sample1, sample2, alternative, method)` | Mann-Whitney U → LIST [U, p-value] |
| `sr_ks_twosample(sample1, sample2, alternative, method)` | 双样本 KS → LIST [D, p-value] |
| `sr_chisquare(observed, expected, ddof)` | 卡方拟合优度 → LIST [stat, p-value] |
| `sr_f_oneway(groups, nan_policy)` | 单因素 ANOVA → LIST [F, p-value] |
| `sr_skewtest(sample, alternative, nan_policy)` | 偏度 z 检验 → LIST [z, p-value] |
| `sr_anderson_darling(sample, distribution, params)` | Anderson-Darling → LIST [A², critical] |
| `sr_fishers_exact(table, alternative)` | Fisher 精确检验 → p-value |
| `sr_fishers_exact_with_odds_ratio(table, alternative)` | → LIST [odds_ratio, p-value] |

样本用 `LIST(DOUBLE)` 字面量或 `list(x)` 构建。`alternative` 和 `nan_policy` 用 DOUBLE 字面量编码
（1.0 = 双侧，2.0 = 小于，3.0 = 大于）。

## 查找函数

用 `duckdb_functions()` 按名称搜索：

```sql
SELECT function_name, function_type, description
FROM duckdb_functions()
WHERE function_name LIKE 'sr_normal%'
ORDER BY function_name;
```
