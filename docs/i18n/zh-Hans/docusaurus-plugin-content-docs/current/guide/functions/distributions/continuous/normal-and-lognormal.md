---
title: 正态与对数正态
sidebar_position: 1
description: 正态（高斯）与对数正态分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile。
---

# 正态与对数正态

## 正态（高斯）

### sr_normal_pdf(x, mean, std_dev)

**签名**：`sr_normal_pdf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

- `x`：求值点
- `mean`：均值（任意实数）
- `std_dev`：标准差（必须 > 0，否则报查询错误）

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_pdf(0.0, 0.0, 1.0)
-- 0.3989422804014327
```

### sr_normal_ln_pdf(x, mean, std_dev)

**签名**：`sr_normal_ln_pdf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

x 处的对数密度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_ln_pdf(0.0, 0.0, 1.0)
-- -0.9189385332046727
```

### sr_normal_cdf(x, mean, std_dev)

**签名**：`sr_normal_cdf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

累积分布 `P(X <= x)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_cdf(1.96, 0.0, 1.0)
-- 0.9750021048529024
```

### sr_normal_sf(x, mean, std_dev)

**签名**：`sr_normal_sf(x DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

生存函数 `P(X > x) = 1 - CDF`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_sf(1.96, 0.0, 1.0)
-- 0.024997895147097634
```

### sr_normal_quantile(p, mean, std_dev)

**签名**：`sr_normal_quantile(p DOUBLE, mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

反 CDF：使 CDF 等于 p 的那个 x。`p` 必须在 [0, 1]，否则报查询错误。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_quantile(0.975, 0.0, 1.0)
-- 1.959963984540054
```

## 对数正态

### sr_log_normal_pdf(x, location, scale)

**签名**：`sr_log_normal_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

x > 0 处的密度。`location` 与 `scale` 是 ln(X) 那个正态分布的参数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_pdf(1.0, 0.0, 1.0)
-- 0.3989422804014327
```

### sr_log_normal_ln_pdf(x, location, scale)

**签名**：`sr_log_normal_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_ln_pdf(1.0, 0.0, 1.0)
-- -0.9189385332046727
```

### sr_log_normal_cdf(x, location, scale)

**签名**：`sr_log_normal_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_cdf(1.0, 0.0, 1.0)
-- 0.5
```

### sr_log_normal_sf(x, location, scale)

**签名**：`sr_log_normal_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_sf(1.0, 0.0, 1.0)
-- 0.5
```

### sr_log_normal_quantile(p, location, scale)

**签名**：`sr_log_normal_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_quantile(0.5, 0.0, 1.0)
-- 1.0
```
