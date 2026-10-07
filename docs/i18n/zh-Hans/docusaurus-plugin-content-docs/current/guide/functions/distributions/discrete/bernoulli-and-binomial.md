---
title: Bernoulli 与二项试验
sidebar_position: 1
description: Bernoulli、二项、负二项与几何分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile，以及各阶矩与支撑。
---

# Bernoulli 与二项试验

这四种分布都建模独立成功 / 失败试验。吃整数的槽位（`x`、试验次数、成功次数）接受 **UBIGINT**——写 `10`，不是 `10.0`；
非整数字面量找不到匹配的签名，查询会直接绑定失败。

## Bernoulli

参数：`p`——成功概率，必须在 [0, 1]。支撑：`{0, 1}`。

### sr_bernoulli_pmf(x, p)

**签名**：`sr_bernoulli_pmf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_pmf(1, 0.7)
-- 0.7
```

### sr_bernoulli_ln_pmf(x, p)

**签名**：`sr_bernoulli_ln_pmf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_ln_pmf(1, 0.7)
```

### sr_bernoulli_cdf(x, p)

**签名**：`sr_bernoulli_cdf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_cdf(0, 0.7)
-- 0.30000000000000004
```

### sr_bernoulli_sf(x, p)

**签名**：`sr_bernoulli_sf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_sf(0, 0.7)
-- 0.7
```

### sr_bernoulli_quantile(p, prob)

**签名**：`sr_bernoulli_quantile(p DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_quantile(0.5, 0.7)
-- 1.0
```

### sr_bernoulli_entropy(p)

**签名**：`sr_bernoulli_entropy(p DOUBLE) -> DOUBLE`

Bernoulli 的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_entropy(0.7)
-- 0.6108643020548935
```

### sr_bernoulli_max(p)

**签名**：`sr_bernoulli_max(p DOUBLE) -> DOUBLE`

Bernoulli 的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_max(0.7)
-- 1.0
```

### sr_bernoulli_mean(p)

**签名**：`sr_bernoulli_mean(p DOUBLE) -> DOUBLE`

Bernoulli 的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_mean(0.7)
-- 0.7
```

### sr_bernoulli_median(p)

**签名**：`sr_bernoulli_median(p DOUBLE) -> DOUBLE`

Bernoulli 的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_median(0.7)
-- 0.0
```

### sr_bernoulli_min(p)

**签名**：`sr_bernoulli_min(p DOUBLE) -> DOUBLE`

Bernoulli 的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_min(0.7)
-- 0.0
```

### sr_bernoulli_mode(p)

**签名**：`sr_bernoulli_mode(p DOUBLE) -> DOUBLE`

Bernoulli 的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_mode(0.7)
-- 1.0
```

### sr_bernoulli_skewness(p)

**签名**：`sr_bernoulli_skewness(p DOUBLE) -> DOUBLE`

Bernoulli 的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_skewness(0.7)
-- -0.8728715609439692
```

### sr_bernoulli_std_dev(p)

**签名**：`sr_bernoulli_std_dev(p DOUBLE) -> DOUBLE`

Bernoulli 的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_std_dev(0.7)
-- 0.45825756949558405
```

### sr_bernoulli_variance(p)

**签名**：`sr_bernoulli_variance(p DOUBLE) -> DOUBLE`

Bernoulli 的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_variance(0.7)
-- 0.21000000000000002
```

## 二项

参数：`p`（成功概率）、`n`（试验次数，UBIGINT）。

### sr_binomial_pmf(x, p, n)

**签名**：`sr_binomial_pmf(x UBIGINT, p DOUBLE, n UBIGINT) -> DOUBLE`

概率质量 `P(X = x)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_pmf(3, 0.5, 10)
-- 0.11718750000000014
```

### sr_binomial_ln_pmf(x, p, n)

**签名**：`sr_binomial_ln_pmf(x UBIGINT, p DOUBLE, n UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_ln_pmf(3, 0.5, 10)
```

### sr_binomial_cdf(x, p, n)

**签名**：`sr_binomial_cdf(x UBIGINT, p DOUBLE, n UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_cdf(3, 0.5, 10)
```

### sr_binomial_sf(x, p, n)

**签名**：`sr_binomial_sf(x UBIGINT, p DOUBLE, n UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_sf(3, 0.5, 10)
```

### sr_binomial_quantile(p, prob, n)

**签名**：`sr_binomial_quantile(p DOUBLE, prob DOUBLE, n UBIGINT) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_quantile(0.5, 0.5, 10)
-- 5.0
```

### sr_binomial_entropy(p, n)

**签名**：`sr_binomial_entropy(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_entropy(0.5, 10)
-- 1.8759536052468009
```

### sr_binomial_max(p, n)

**签名**：`sr_binomial_max(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_max(0.5, 10)
-- 10.0
```

### sr_binomial_mean(p, n)

**签名**：`sr_binomial_mean(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_mean(0.5, 10)
-- 5.0
```

### sr_binomial_median(p, n)

**签名**：`sr_binomial_median(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_median(0.5, 10)
-- 5.0
```

### sr_binomial_min(p, n)

**签名**：`sr_binomial_min(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_min(0.5, 10)
-- 0.0
```

### sr_binomial_mode(p, n)

**签名**：`sr_binomial_mode(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_mode(0.5, 10)
-- 5.0
```

### sr_binomial_skewness(p, n)

**签名**：`sr_binomial_skewness(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_skewness(0.5, 10)
-- 0.0
```

### sr_binomial_std_dev(p, n)

**签名**：`sr_binomial_std_dev(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_std_dev(0.5, 10)
-- 1.5811388300841898
```

### sr_binomial_variance(p, n)

**签名**：`sr_binomial_variance(p DOUBLE, n UBIGINT) -> DOUBLE`

二项的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_variance(0.5, 10)
-- 2.5
```

## 负二项

参数：`r`（成功次数，statrs 里可取实数）、`p`（成功概率）。
支撑：第 r 次成功前的失败次数。

### sr_negative_binomial_pmf(x, r, p)

**签名**：`sr_negative_binomial_pmf(x UBIGINT, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_pmf(3, 2.0, 0.5)
-- 0.12500000000000086
```

### sr_negative_binomial_ln_pmf(x, r, p)

**签名**：`sr_negative_binomial_ln_pmf(x UBIGINT, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_ln_pmf(3, 2.0, 0.5)
```

### sr_negative_binomial_cdf(x, r, p)

**签名**：`sr_negative_binomial_cdf(x UBIGINT, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_cdf(3, 2.0, 0.5)
```

### sr_negative_binomial_sf(x, r, p)

**签名**：`sr_negative_binomial_sf(x UBIGINT, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_sf(3, 2.0, 0.5)
```

### sr_negative_binomial_quantile(p, r, prob)

**签名**：`sr_negative_binomial_quantile(p DOUBLE, r DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_quantile(0.5, 2.0, 0.5)
```

### sr_negative_binomial_entropy(r, p)

**签名**：`sr_negative_binomial_entropy(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_entropy(2.0, 0.5)
-- NULL
```

### sr_negative_binomial_max(r, p)

**签名**：`sr_negative_binomial_max(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_max(2.0, 0.5)
-- 1.8446744073709552e+19
```

### sr_negative_binomial_mean(r, p)

**签名**：`sr_negative_binomial_mean(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_mean(2.0, 0.5)
-- 2.0
```

### sr_negative_binomial_min(r, p)

**签名**：`sr_negative_binomial_min(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_min(2.0, 0.5)
-- 0.0
```

### sr_negative_binomial_mode(r, p)

**签名**：`sr_negative_binomial_mode(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_mode(2.0, 0.5)
-- 1.0
```

### sr_negative_binomial_skewness(r, p)

**签名**：`sr_negative_binomial_skewness(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_skewness(2.0, 0.5)
-- 1.5
```

### sr_negative_binomial_std_dev(r, p)

**签名**：`sr_negative_binomial_std_dev(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_std_dev(2.0, 0.5)
-- 2.0
```

### sr_negative_binomial_variance(r, p)

**签名**：`sr_negative_binomial_variance(r DOUBLE, p DOUBLE) -> DOUBLE`

负二项的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_variance(2.0, 0.5)
-- 4.0
```

## Geometric（几何）

参数：`p`——成功概率。支撑从 1 起（statrs 惯例：首次成功所需的试验数）。

### sr_geometric_pmf(x, p)

**签名**：`sr_geometric_pmf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_pmf(2, 0.5)
-- 0.25
```

### sr_geometric_ln_pmf(x, p)

**签名**：`sr_geometric_ln_pmf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_ln_pmf(2, 0.5)
```

### sr_geometric_cdf(x, p)

**签名**：`sr_geometric_cdf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_cdf(1, 0.5)
-- 0.5
```

### sr_geometric_sf(x, p)

**签名**：`sr_geometric_sf(x UBIGINT, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_sf(1, 0.5)
-- 0.5
```

### sr_geometric_quantile(p, prob)

**签名**：`sr_geometric_quantile(p DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_quantile(0.5, 0.5)
```

### sr_geometric_dist_mean(p)

**签名**：`sr_geometric_dist_mean(p DOUBLE) -> DOUBLE`

Geometric（几何）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_dist_mean(0.5)
-- 2.0
```

### sr_geometric_entropy(p)

**签名**：`sr_geometric_entropy(p DOUBLE) -> DOUBLE`

Geometric（几何）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_entropy(0.5)
-- 1.3862943611198906
```

### sr_geometric_max(p)

**签名**：`sr_geometric_max(p DOUBLE) -> DOUBLE`

Geometric（几何）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_max(0.5)
-- 1.8446744073709552e+19
```

### sr_geometric_median(p)

**签名**：`sr_geometric_median(p DOUBLE) -> DOUBLE`

Geometric（几何）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_median(0.5)
-- 1.0
```

### sr_geometric_min(p)

**签名**：`sr_geometric_min(p DOUBLE) -> DOUBLE`

Geometric（几何）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_min(0.5)
-- 1.0
```

### sr_geometric_mode(p)

**签名**：`sr_geometric_mode(p DOUBLE) -> DOUBLE`

Geometric（几何）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_mode(0.5)
-- 1.0
```

### sr_geometric_skewness(p)

**签名**：`sr_geometric_skewness(p DOUBLE) -> DOUBLE`

Geometric（几何）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_skewness(0.5)
-- 2.1213203435596424
```

### sr_geometric_std_dev(p)

**签名**：`sr_geometric_std_dev(p DOUBLE) -> DOUBLE`

Geometric（几何）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_std_dev(0.5)
-- 1.4142135623730951
```

### sr_geometric_variance(p)

**签名**：`sr_geometric_variance(p DOUBLE) -> DOUBLE`

Geometric（几何）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_variance(0.5)
-- 2.0
```

## 错误与 NULL

```sql {"type":"duckfn","expect":"error"}
SELECT sr_binomial_pmf(2.5, 0.5, 10)
-- error: no matching signature: x is UBIGINT, so 2.5 does not bind
```

```sql {"type":"duckfn","show":"value"}
-- NULL 输入短路成 NULL：
SELECT sr_binomial_pmf(NULL, 0.5, 10)
-- NULL
```
