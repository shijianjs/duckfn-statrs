---
title: 假设检验
sidebar_position: 18
description: 9 个统计假设检验——t 检验、卡方、ANOVA、KS、Mann-Whitney U、偏度、Anderson-Darling、Fisher 精确。
---

# 假设检验

除非另有说明，每个检验返回 `LIST(DOUBLE)` 形如 `[统计量, p 值]`。样本参数是 `LIST(DOUBLE)`。
代码类参数（`alternative`、`nan_policy`、`method`）用 DOUBLE 字面量，与 statrs 的枚举对应。

## sr_ttest_onesample(sample, popmean, alternative, nan_policy)

**签名**：`sr_ttest_onesample(sample LIST(DOUBLE), popmean DOUBLE, alternative DOUBLE, nan_policy DOUBLE) -> LIST(DOUBLE)`

单样本 t 检验，与 `popmean` 比较。返回 `[t 统计量, p 值]`。

- `sample`：观测值列表
- `popmean`：待检验的总体均值
- `alternative`：1.0 = 双侧，2.0 = less，3.0 = greater
- `nan_policy`：1.0 = propagate，2.0 = omit，3.0 = raise（对应 statrs 的枚举变体）

```sql {"type":"duckfn","show":"value"}
SELECT sr_ttest_onesample([1.0, 2.0, 3.0, 4.0, 5.0], 3.0, 1.0, 1.0)
-- [0.0, 1.0]
```

## sr_chisquare(observed, expected, ddof)

**签名**：`sr_chisquare(observed LIST(DOUBLE), expected LIST(DOUBLE), ddof DOUBLE) -> LIST(DOUBLE)`

卡方拟合优度检验。`expected` 传 NULL 表示均匀期望；`ddof` 传 NULL 默认为 0。

返回 `[卡方统计量, p 值]`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_chisquare([16.0, 18.0, 16.0, 14.0, 12.0, 12.0], NULL, NULL)
-- [2.0, 0.8491450360846101]
```

## sr_f_oneway(groups, nan_policy)

**签名**：`sr_f_oneway(groups LIST(LIST(DOUBLE)), nan_policy DOUBLE) -> LIST(DOUBLE)`

单因素 ANOVA，多组样本。返回 `[F 统计量, p 值]`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_f_oneway([[1.0, 3.0, 5.0], [2.0, 4.0, 8.0]], 1.0)
-- [0.6249999999999984, 0.47342736525713647]
```

## sr_fishers_exact(table, alternative)

**签名**：`sr_fishers_exact(table LIST(DOUBLE), alternative DOUBLE) -> DOUBLE`

2x2 列联表（4 个整数值 DOUBLE，行主序）的 Fisher 精确检验 p 值。返回单个 DOUBLE，不是 LIST。

- `alternative`：1.0 = 双侧，2.0 = less，3.0 = greater

```sql {"type":"duckfn","show":"value"}
SELECT sr_fishers_exact([1.0, 2.0, 3.0, 4.0], 1.0)
-- 1.0
```

## sr_fishers_exact_with_odds_ratio(table, alternative)

**签名**：`sr_fishers_exact_with_odds_ratio(table LIST(DOUBLE), alternative DOUBLE) -> LIST(DOUBLE)`

与 `sr_fishers_exact` 同，返回 `[odds_ratio, p 值]`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_fishers_exact_with_odds_ratio([1.0, 2.0, 3.0, 4.0], 2.0)
-- [0.6666666666666666, 0.6666666666666666]
```

## sr_ks_twosample(sample1, sample2, alternative, method)

**签名**：`sr_ks_twosample(sample1 LIST(DOUBLE), sample2 LIST(DOUBLE), alternative DOUBLE, method DOUBLE) -> LIST(DOUBLE)`

两样本 Kolmogorov-Smirnov 检验。返回 `[D 统计量, p 值]`。

- `alternative`：1.0 = less，2.0 = greater，3.0 = 双侧精确，4.0 = 双侧渐近
- `method`：模式选择，一般用 1.0（自动）

```sql {"type":"duckfn","show":"value"}
SELECT sr_ks_twosample([0.1, 0.2, 0.3], [0.4, 0.5, 0.6], 4.0, 1.0)
-- [1.0, 0.09956184831478034]
```

## sr_mannwhitneyu(sample1, sample2, alternative, method)

**签名**：`sr_mannwhitneyu(sample1 LIST(DOUBLE), sample2 LIST(DOUBLE), alternative DOUBLE, method DOUBLE) -> LIST(DOUBLE)`

Mann-Whitney U 检验。返回 `[U 统计量, p 值]`。

- `alternative`：1.0 = 双侧，2.0 = less，3.0 = greater
- `method`：1.0 = 自动，2.0 = 精确，3.0 = 渐近含连续性校正，4.0 = 渐近不含

```sql {"type":"duckfn","show":"value"}
SELECT sr_mannwhitneyu([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 2.0, 2.0)
-- [0.0, 0.95]
```

## sr_skewtest(sample, alternative, nan_policy)

**签名**：`sr_skewtest(sample LIST(DOUBLE), alternative DOUBLE, nan_policy DOUBLE) -> LIST(DOUBLE)`

偏度 z 检验。返回 `[z 统计量, p 值]`。

- `alternative`：1.0 = 双侧，2.0 = less（负偏），3.0 = greater（正偏）
- `nan_policy`：与 t 检验同

```sql {"type":"duckfn","show":"value"}
SELECT sr_skewtest([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 1.0, 1.0)
-- [1.018464355396213, 0.30845733197774927]
```

## sr_anderson_darling(sample, distribution, params)

**签名**：`sr_anderson_darling(sample LIST(DOUBLE), distribution VARCHAR, params LIST(DOUBLE)) -> LIST(DOUBLE)`

对指定分布的 Anderson-Darling 拟合优度检验。返回 `[A², 5% 临界值]`。

- `distribution`：`'normal'`、`'lognormal'`、`'exponential'`、`'gumbel'`、`'weibull'`、`'uniform'` 之一
- `params`：分布参数列表（正态是 `[mean, std_dev]`）

```sql {"type":"duckfn","show":"value"}
SELECT sr_anderson_darling([1.0, 2.0, 3.0, 4.0, 5.0], 'normal', [3.0, 1.5])
-- [0.15491765936161173, 0.8959843708415298]
```

## 从列构造样本 LIST

数据在表里时用 `list(x)` 喂给检验：

```sql {"type":"duckfn","show":"value"}
SELECT sr_ttest_onesample(list(x), 3.0, 1.0, 1.0)
FROM (VALUES (1.0), (2.0), (3.0), (4.0), (5.0)) t(x)
-- [0.0, 1.0]
```

## 非法 code 报查询错误

```sql {"type":"duckfn","expect":"error"}
SELECT sr_ttest_onesample([1.0, 2.0], 3.0, 9.0, 1.0)
-- error: the alternative must be 1 = two-sided, 2 = less, 3 = greater
```
