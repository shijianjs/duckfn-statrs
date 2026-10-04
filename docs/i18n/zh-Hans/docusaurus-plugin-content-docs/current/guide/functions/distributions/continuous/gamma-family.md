---
title: Gamma 族
sidebar_position: 2
description: Gamma、逆 Gamma、卡方、Chi、Erlang 与指数分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile。
---

# Gamma 族

Gamma、逆 Gamma、卡方、Chi、Erlang 与指数分布都通过重参数化关联到同一个 Gamma law。

## Gamma

参数：`shape`（> 0）、`rate`（> 0）。形状 / 率参数化。

### sr_gamma_pdf(x, shape, rate)

**签名**：`sr_gamma_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_pdf(1.0, 2.0, 2.0)
-- 0.2706705664732254
```

### sr_gamma_ln_pdf(x, shape, rate)

**签名**：`sr_gamma_ln_pdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_ln_pdf(1.0, 2.0, 2.0)
```

### sr_gamma_cdf(x, shape, rate)

**签名**：`sr_gamma_cdf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_cdf(1.0, 2.0, 2.0)
```

### sr_gamma_sf(x, shape, rate)

**签名**：`sr_gamma_sf(x DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_sf(1.0, 2.0, 2.0)
```

### sr_gamma_quantile(p, shape, rate)

**签名**：`sr_gamma_quantile(p DOUBLE, shape DOUBLE, rate DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_quantile(0.5, 2.0, 2.0)
```

## 逆 Gamma

参数：`shape`（> 0）、`scale`（> 0）。

### sr_inverse_gamma_pdf(x, shape, scale)

**签名**：`sr_inverse_gamma_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_pdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_ln_pdf(x, shape, scale)

**签名**：`sr_inverse_gamma_ln_pdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_ln_pdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_cdf(x, shape, scale)

**签名**：`sr_inverse_gamma_cdf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_cdf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_sf(x, shape, scale)

**签名**：`sr_inverse_gamma_sf(x DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_sf(1.0, 2.0, 2.0)
```

### sr_inverse_gamma_quantile(p, shape, scale)

**签名**：`sr_inverse_gamma_quantile(p DOUBLE, shape DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_inverse_gamma_quantile(0.5, 2.0, 2.0)
```

## 卡方（Chi-squared）

参数：`freedom`（自由度，> 0）。

### sr_chi_squared_pdf(x, freedom)

**签名**：`sr_chi_squared_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_pdf(1.0, 2.0)
```

### sr_chi_squared_ln_pdf(x, freedom)

**签名**：`sr_chi_squared_ln_pdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_ln_pdf(1.0, 2.0)
```

### sr_chi_squared_cdf(x, freedom)

**签名**：`sr_chi_squared_cdf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_cdf(1.0, 2.0)
```

### sr_chi_squared_sf(x, freedom)

**签名**：`sr_chi_squared_sf(x DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_sf(1.0, 2.0)
```

### sr_chi_squared_quantile(p, freedom)

**签名**：`sr_chi_squared_quantile(p DOUBLE, freedom DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_chi_squared_quantile(0.95, 2.0)
```

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
