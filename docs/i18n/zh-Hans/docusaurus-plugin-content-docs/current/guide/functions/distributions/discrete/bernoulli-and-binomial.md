---
title: Bernoulli 与二项试验
sidebar_position: 1
description: Bernoulli、二项、负二项与几何分布——每种都提供 pmf / ln_pmf / cdf / sf / quantile。
---

# Bernoulli 与二项试验

这四种分布都建模独立成功 / 失败试验。吃整数的槽位（`x`、试验次数、成功次数）接受
**整数值 DOUBLE** 字面量（如 `10.0`），不是 `10` 也不是 `3.5`——非整数会报查询错误，不会四舍五入。

## Bernoulli

参数：`p`——成功概率，必须在 [0, 1]。支撑：`{0, 1}`。

### sr_bernoulli_pmf(x, p)

**签名**：`sr_bernoulli_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_pmf(1.0, 0.7)
-- 0.7
```

### sr_bernoulli_ln_pmf(x, p)

**签名**：`sr_bernoulli_ln_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_ln_pmf(1.0, 0.7)
```

### sr_bernoulli_cdf(x, p)

**签名**：`sr_bernoulli_cdf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_cdf(0.0, 0.7)
-- 0.30000000000000004
```

### sr_bernoulli_sf(x, p)

**签名**：`sr_bernoulli_sf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_sf(0.0, 0.7)
-- 0.7
```

### sr_bernoulli_quantile(p, prob)

**签名**：`sr_bernoulli_quantile(p DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_bernoulli_quantile(0.5, 0.7)
-- 1.0
```

## 二项

参数：`p`（成功概率）、`n`（试验次数，整数值 DOUBLE）。

### sr_binomial_pmf(x, p, n)

**签名**：`sr_binomial_pmf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

概率质量 `P(X = x)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_pmf(3.0, 0.5, 10.0)::DECIMAL(12,8)
-- 0.11718750
```

### sr_binomial_ln_pmf(x, p, n)

**签名**：`sr_binomial_ln_pmf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_ln_pmf(3.0, 0.5, 10.0)
```

### sr_binomial_cdf(x, p, n)

**签名**：`sr_binomial_cdf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_cdf(3.0, 0.5, 10.0)
```

### sr_binomial_sf(x, p, n)

**签名**：`sr_binomial_sf(x DOUBLE, p DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_sf(3.0, 0.5, 10.0)
```

### sr_binomial_quantile(p, prob, n)

**签名**：`sr_binomial_quantile(p DOUBLE, prob DOUBLE, n DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_binomial_quantile(0.5, 0.5, 10.0)
-- 5.0
```

## 负二项

参数：`r`（成功次数，statrs 里可取实数）、`p`（成功概率）。
支撑：第 r 次成功前的失败次数。

### sr_negative_binomial_pmf(x, r, p)

**签名**：`sr_negative_binomial_pmf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_pmf(3.0, 2.0, 0.5)::DECIMAL(12,8)
-- 0.12500000
```

### sr_negative_binomial_ln_pmf(x, r, p)

**签名**：`sr_negative_binomial_ln_pmf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_ln_pmf(3.0, 2.0, 0.5)
```

### sr_negative_binomial_cdf(x, r, p)

**签名**：`sr_negative_binomial_cdf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_cdf(3.0, 2.0, 0.5)
```

### sr_negative_binomial_sf(x, r, p)

**签名**：`sr_negative_binomial_sf(x DOUBLE, r DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_sf(3.0, 2.0, 0.5)
```

### sr_negative_binomial_quantile(p, r, prob)

**签名**：`sr_negative_binomial_quantile(p DOUBLE, r DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_negative_binomial_quantile(0.5, 2.0, 0.5)
```

## Geometric（几何）

参数：`p`——成功概率。支撑从 1 起（statrs 惯例：首次成功所需的试验数）。

### sr_geometric_pmf(x, p)

**签名**：`sr_geometric_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_pmf(2.0, 0.5)
-- 0.25
```

### sr_geometric_ln_pmf(x, p)

**签名**：`sr_geometric_ln_pmf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_ln_pmf(2.0, 0.5)
```

### sr_geometric_cdf(x, p)

**签名**：`sr_geometric_cdf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_cdf(1.0, 0.5)
-- 0.5
```

### sr_geometric_sf(x, p)

**签名**：`sr_geometric_sf(x DOUBLE, p DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_sf(1.0, 0.5)::DECIMAL(12,8)
-- 0.50000000
```

### sr_geometric_quantile(p, prob)

**签名**：`sr_geometric_quantile(p DOUBLE, prob DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_geometric_quantile(0.5, 0.5)
```

## 错误与 NULL

```sql {"type":"duckfn","expect":"error"}
SELECT sr_binomial_pmf(2.5, 0.5, 10.0)
-- error: expected a non-negative whole number, got 2.5
```

```sql {"type":"duckfn","show":"value"}
-- NULL 输入短路成 NULL：
SELECT sr_binomial_pmf(NULL, 0.5, 10.0)
-- NULL
```
