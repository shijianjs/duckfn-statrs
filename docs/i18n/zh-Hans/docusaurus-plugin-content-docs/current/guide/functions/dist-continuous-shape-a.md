---
title: "连续：形状 (A)"
sidebar_position: 6
description: Gamma、逆 Gamma、卡方分布——每种都有 pdf / ln_pdf / cdf / sf / quantile。
---

# 连续分布：形状 (A)

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
