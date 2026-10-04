---
title: "连续：形状 (B)"
sidebar_position: 7
description: Chi、Erlang、指数与 Beta 分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile。
---

# 连续分布：形状 (B)

## Chi（卡方根）

参数：`freedom`（自由度，> 0）。开根号的卡方变量。

### sr_chi_pdf(x, freedom)

**签名**：`sr_chi_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_pdf(1.0, 2.0)
```

### sr_chi_ln_pdf(x, freedom)

**签名**：`sr_chi_ln_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_ln_pdf(1.0, 2.0)
```

### sr_chi_cdf(x, freedom)

**签名**：`sr_chi_cdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_cdf(1.0, 2.0)
```

### sr_chi_sf(x, freedom)

**签名**：`sr_chi_sf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_sf(1.0, 2.0)
```

### sr_chi_quantile(p, freedom)

**签名**：`sr_chi_quantile(p DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_quantile(0.5, 2.0)
```

## Erlang

参数：`shape`（整数值 DOUBLE > 0）、`rate`（> 0）。

### sr_erlang_pdf(x, shape, rate)

**签名**：`sr_erlang_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_pdf(1.0, 2.0, 2.0)
```

### sr_erlang_ln_pdf(x, shape, rate)

**签名**：`sr_erlang_ln_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_ln_pdf(1.0, 2.0, 2.0)
```

### sr_erlang_cdf(x, shape, rate)

**签名**：`sr_erlang_cdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_cdf(1.0, 2.0, 2.0)
```

### sr_erlang_sf(x, shape, rate)

**签名**：`sr_erlang_sf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_sf(1.0, 2.0, 2.0)
```

### sr_erlang_quantile(p, shape, rate)

**签名**：`sr_erlang_quantile(p DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_erlang_quantile(0.5, 2.0, 2.0)
```

## 指数

参数：`rate`（> 0）。

### sr_exp_pdf(x, rate)

**签名**：`sr_exp_pdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_pdf(1.0, 2.0)
-- 0.2706705664732254
```

### sr_exp_ln_pdf(x, rate)

**签名**：`sr_exp_ln_pdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_ln_pdf(1.0, 2.0)
```

### sr_exp_cdf(x, rate)

**签名**：`sr_exp_cdf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_cdf(1.0, 2.0)
-- 0.8646647167633873
```

### sr_exp_sf(x, rate)

**签名**：`sr_exp_sf(x DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_sf(1.0, 2.0)
```

### sr_exp_quantile(p, rate)

**签名**：`sr_exp_quantile(p DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_exp_quantile(0.5, 2.0)
-- 0.34657359027997264
```

## Beta

参数：`shape_a`（> 0）、`shape_b`（> 0）。支撑：`[0, 1]`。

### sr_beta_pdf(x, shape_a, shape_b)

**签名**：`sr_beta_pdf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_pdf(0.5, 2.0, 3.0)
-- 1.5
```

### sr_beta_ln_pdf(x, shape_a, shape_b)

**签名**：`sr_beta_ln_pdf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_ln_pdf(0.5, 2.0, 3.0)
```

### sr_beta_cdf(x, shape_a, shape_b)

**签名**：`sr_beta_cdf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_cdf(0.5, 2.0, 3.0)
-- 0.6875
```

### sr_beta_sf(x, shape_a, shape_b)

**签名**：`sr_beta_sf(x DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_sf(0.5, 2.0, 3.0)
```

### sr_beta_quantile(p, shape_a, shape_b)

**签名**：`sr_beta_quantile(p DOUBLE, shape_a DOUBLE, shape_b DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_quantile(0.5, 2.0, 3.0)
```
