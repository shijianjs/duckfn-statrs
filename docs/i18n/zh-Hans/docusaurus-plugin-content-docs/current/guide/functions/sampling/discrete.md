---
title: 离散、多元与经验
sidebar_position: 2
description: 从离散、多元、类别与经验分布抽 k 个样本。
---

# 离散、多元与经验采样

## Bernoulli

### sr_sample_bernoulli(p, k)

**签名**：`sr_sample_bernoulli(p DOUBLE, k BIGINT) -> LIST(DOUBLE)`

每个样本都是 0 或 1：

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_bernoulli(0.5, 5)
```

## Binomial（二项）

### sr_sample_binomial(p, n, k)

**签名**：`sr_sample_binomial(p DOUBLE, n UBIGINT, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_binomial(0.5, 10, 12)
```

### sr_sample_binomial_algorithm(p, n, algorithm, k)

**签名**：`sr_sample_binomial_algorithm(p DOUBLE, n UBIGINT, algorithm DOUBLE, k BIGINT) -> LIST(UBIGINT)`

显式选择算法：`algorithm` 为 1.0 = automatic，2.0 = inversion，3.0 = rejection。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_binomial_algorithm(0.5, 10, 2.0, 4)
```

## Negative binomial（负二项）

### sr_sample_negative_binomial(r, p, k)

**签名**：`sr_sample_negative_binomial(r DOUBLE, p DOUBLE, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_negative_binomial(2.0, 0.5, 10)
```

## Poisson（泊松）

### sr_sample_poisson(lambda, k)

**签名**：`sr_sample_poisson(lambda DOUBLE, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_poisson(3.0, 10)
```

## Geometric（几何）

### sr_sample_geometric(p, k)

**签名**：`sr_sample_geometric(p DOUBLE, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_geometric(0.5, 10)
```

## Hypergeometric（超几何）

### sr_sample_hypergeometric(population, successes, draws, k)

**签名**：`sr_sample_hypergeometric(population UBIGINT, successes UBIGINT, draws UBIGINT, k BIGINT) -> LIST(UBIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_hypergeometric(10, 5, 4, 8)
```

## Categorical（类别）

### sr_sample_categorical(probabilities, k)

**签名**：`sr_sample_categorical(probabilities DOUBLE[], k BIGINT) -> LIST(UBIGINT)`

每个样本是 `0 .. len(probabilities) - 1` 内的一个类别下标。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_categorical([1.0, 2.0, 1.0], 5)
```

## Discrete uniform（离散均匀）

### sr_sample_discrete_uniform(min, max, k)

**签名**：`sr_sample_discrete_uniform(min BIGINT, max BIGINT, k BIGINT) -> LIST(BIGINT)`

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_discrete_uniform(1, 6, 10)
```

## Multivariate normal（多元正态）

### sr_sample_multivariate_normal(mean, covariance, k)

**签名**：`sr_sample_multivariate_normal(mean DOUBLE[], covariance DOUBLE[], k BIGINT) -> LIST(LIST(DOUBLE))`

返回 `k` 个点的列表，每个点的长度与 `mean` 一致。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_multivariate_normal([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 5)
```

## Multivariate Student's t（多元 t）

### sr_sample_multivariate_students_t(location, scale, freedom, k)

**签名**：`sr_sample_multivariate_students_t(location DOUBLE[], scale DOUBLE[], freedom DOUBLE, k BIGINT) -> LIST(LIST(DOUBLE))`

返回 `k` 个点的列表，每个点的长度与 `location` 一致。`scale` 是行主序展平的尺度矩阵
（`len(location)²` 个元素），长度不符报查询错误。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_multivariate_students_t([0.0, 0.0], [1.0, 0.0, 0.0, 1.0], 3.0, 6)
```

## Dirichlet

### sr_sample_dirichlet(alpha, k)

**签名**：`sr_sample_dirichlet(alpha DOUBLE[], k BIGINT) -> LIST(LIST(DOUBLE))`

返回单纯形上的 `k` 个点：每次抽样都是与 `alpha` 等长、分量和为 1 的向量。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_dirichlet([1.0, 2.0], 8)
```

## Multinomial（多项式）

### sr_sample_multinomial(probs, trials, k)

**签名**：`sr_sample_multinomial(probs DOUBLE[], trials UBIGINT, k BIGINT) -> LIST(LIST(UBIGINT))`

返回 `k` 个计数向量，长度与 `probs` 一致；每个计数向量的分量和恒等于 `trials`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_multinomial([0.3, 0.7], 10, 8)
```

## Empirical（经验分布，聚合）

### sr_sample_empirical(v, k)

**签名**：`sr_sample_empirical(v DOUBLE[], k DOUBLE) -> LIST(DOUBLE)`

唯一的一个**聚合**采样器：它把整列 `v` 收集起来，从它的经验分布里重抽 `k` 个点。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_empirical(v, 6)
FROM (VALUES (1.0), (2.0), (3.0)) t(v)
```

## 错误

未知的 `algorithm` code：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_sample_binomial_algorithm(0.5, 10, 9.0, 4)
-- error: sr_sample_binomial_algorithm: the algorithm must be 1 (automatic), 2 (inversion) or 3 (rejection), got 9
```