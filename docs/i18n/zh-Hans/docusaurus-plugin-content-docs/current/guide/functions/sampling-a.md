---
title: "随机抽样 (A)"
sidebar_position: 15
description: 从连续分布抽 k 个样本，输出 LIST(DOUBLE)。
---

# 随机抽样 (A)：连续分布

每个采样器返回长度为 `k` 的 `LIST(DOUBLE)`。`k` 是 **BIGINT**（正的整数量）。随机输出没有固定的
点值，示例断言的是结构不变量（`len(...)` 或区间检查）。参数与对应分布函数签名里除求值点 `x` 
以外的部分一致。

## sr_sample_normal(mean, std_dev, k)

**签名**：`sr_sample_normal(mean DOUBLE, std_dev DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_normal(0.0, 1.0, 10))
-- 10
```

## sr_sample_log_normal(location, scale, k)

**签名**：`sr_sample_log_normal(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_log_normal(0.0, 1.0, 10))
```

## sr_sample_gamma(shape, rate, k)

**签名**：`sr_sample_gamma(shape DOUBLE, rate DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_gamma(2.0, 2.0, 10))
```

## sr_sample_inverse_gamma(shape, scale, k)

**签名**：`sr_sample_inverse_gamma(shape DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_inverse_gamma(2.0, 2.0, 10))
```

## sr_sample_chi_squared(freedom, k)

**签名**：`sr_sample_chi_squared(freedom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_chi_squared(2.0, 10))
```

## sr_sample_chi(freedom, k)

**签名**：`sr_sample_chi(freedom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_chi(2.0, 10))
```

## sr_sample_erlang(shape, rate, k)

**签名**：`sr_sample_erlang(shape DOUBLE, rate DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_erlang(2.0, 2.0, 10))
```

## sr_sample_exp(rate, k)

**签名**：`sr_sample_exp(rate DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_exp(2.0, 5))
-- 5
```

## sr_sample_uniform(min, max, k)

**签名**：`sr_sample_uniform(min DOUBLE, max DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT count(*) FILTER (WHERE s < 2.0 OR s > 3.0) = 0
FROM (SELECT unnest(sr_sample_uniform(2.0, 3.0, 300)) AS s)
-- true
```

## sr_sample_students_t(location, scale, freedom, k)

**签名**：`sr_sample_students_t(location DOUBLE, scale DOUBLE, freedom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_students_t(0.0, 1.0, 2.0, 10))
```

## sr_sample_fisher_snedecor(df_num, df_denom, k)

**签名**：`sr_sample_fisher_snedecor(df_num DOUBLE, df_denom DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_fisher_snedecor(2.0, 3.0, 10))
```

## sr_sample_cauchy(location, scale, k)

**签名**：`sr_sample_cauchy(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_cauchy(0.0, 1.0, 10))
```

## sr_sample_laplace(location, scale, k)

**签名**：`sr_sample_laplace(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_laplace(0.0, 1.0, 10))
```

## sr_sample_gumbel(location, scale, k)

**签名**：`sr_sample_gumbel(location DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_gumbel(0.0, 1.0, 10))
```

## sr_sample_levy(mu, c, k)

**签名**：`sr_sample_levy(mu DOUBLE, c DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_levy(0.0, 1.0, 10))
```

## sr_sample_pareto(scale, shape, k)

**签名**：`sr_sample_pareto(scale DOUBLE, shape DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_pareto(1.0, 2.0, 10))
```

## sr_sample_triangular(min, max, mode, k)

**签名**：`sr_sample_triangular(min DOUBLE, max DOUBLE, mode DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_triangular(0.0, 2.0, 1.0, 10))
```

## sr_sample_weibull(shape, scale, k)

**签名**：`sr_sample_weibull(shape DOUBLE, scale DOUBLE, k BIGINT) -> LIST(DOUBLE)`

```sql {"type":"duckfn","show":"value"}
SELECT len(sr_sample_weibull(1.0, 1.0, 10))
```

## sr_sample_dirac(v, k)

**签名**：`sr_sample_dirac(v DOUBLE, k BIGINT) -> LIST(DOUBLE)`

所有样本都是 `v`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sample_dirac(3.0, 2)
-- [3.0, 3.0]
```

## 错误

`k` 必须是正的整数量：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_sample_normal(0.0, 1.0, 0)
-- error: sr_sample_normal: k must be a positive whole number, got 0
```
