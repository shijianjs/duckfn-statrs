---
title: 正态与对数正态
sidebar_position: 1
description: 正态（高斯）与对数正态分布——每种都提供 pdf / ln_pdf / cdf / sf / quantile，以及各阶矩与支撑。
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

### sr_normal_entropy(mean, std_dev)

**签名**：`sr_normal_entropy(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_entropy(0.0, 1.0)
-- 1.4189385332046727
```

### sr_normal_max(mean, std_dev)

**签名**：`sr_normal_max(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_max(0.0, 1.0)
-- inf
```

### sr_normal_mean(mean, std_dev)

**签名**：`sr_normal_mean(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_mean(0.0, 1.0)
-- 0.0
```

### sr_normal_median(mean, std_dev)

**签名**：`sr_normal_median(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_median(0.0, 1.0)
-- 0.0
```

### sr_normal_min(mean, std_dev)

**签名**：`sr_normal_min(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_min(0.0, 1.0)
-- -inf
```

### sr_normal_mode(mean, std_dev)

**签名**：`sr_normal_mode(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_mode(0.0, 1.0)
-- 0.0
```

### sr_normal_skewness(mean, std_dev)

**签名**：`sr_normal_skewness(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_skewness(0.0, 1.0)
-- 0.0
```

### sr_normal_std_dev(mean, std_dev)

**签名**：`sr_normal_std_dev(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_std_dev(0.0, 1.0)
-- 1.0
```

### sr_normal_variance(mean, std_dev)

**签名**：`sr_normal_variance(mean DOUBLE, std_dev DOUBLE) -> DOUBLE`

正态（高斯）的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_normal_variance(0.0, 1.0)
-- 1.0
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

### sr_log_normal_entropy(location, scale)

**签名**：`sr_log_normal_entropy(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的熵。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_entropy(0.0, 1.0)
-- 1.4189385332046727
```

### sr_log_normal_max(location, scale)

**签名**：`sr_log_normal_max(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的最大值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_max(0.0, 1.0)
-- inf
```

### sr_log_normal_mean(location, scale)

**签名**：`sr_log_normal_mean(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的均值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_mean(0.0, 1.0)
-- 1.6487212707001282
```

### sr_log_normal_median(location, scale)

**签名**：`sr_log_normal_median(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的中位数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_median(0.0, 1.0)
-- 1.0
```

### sr_log_normal_min(location, scale)

**签名**：`sr_log_normal_min(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的最小值。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_min(0.0, 1.0)
-- 0.0
```

### sr_log_normal_mode(location, scale)

**签名**：`sr_log_normal_mode(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的众数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_mode(0.0, 1.0)
-- 0.36787944117144233
```

### sr_log_normal_skewness(location, scale)

**签名**：`sr_log_normal_skewness(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的偏度。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_skewness(0.0, 1.0)
-- 6.184877138632554
```

### sr_log_normal_std_dev(location, scale)

**签名**：`sr_log_normal_std_dev(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的标准差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_std_dev(0.0, 1.0)
-- 2.1611974158950877
```

### sr_log_normal_variance(location, scale)

**签名**：`sr_log_normal_variance(location DOUBLE, scale DOUBLE) -> DOUBLE`

对数正态的方差。

```sql {"type":"duckfn","show":"value"}
SELECT sr_log_normal_variance(0.0, 1.0)
-- 4.670774270471604
```
