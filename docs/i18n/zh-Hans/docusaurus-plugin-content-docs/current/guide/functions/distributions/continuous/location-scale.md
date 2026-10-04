---
title: 位置-尺度
sidebar_position: 4
description: 柯西与 Laplace 分布——关于位置对称的位置-尺度族，每种提供 pdf / ln_pdf / cdf / sf / quantile。
---

# 位置-尺度

柯西与 Laplace 分布关于 `location` 对称，由 `scale`（必须 > 0）缩放。

## 柯西

### sr_cauchy_pdf(x, location, scale)

**签名**：`sr_cauchy_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_pdf(0.0, 0.0, 1.0)
-- 0.3183098861837907
```

### sr_cauchy_ln_pdf(x, location, scale)

**签名**：`sr_cauchy_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_ln_pdf(0.0, 0.0, 1.0)
-- -1.1447298858494002
```

### sr_cauchy_cdf(x, location, scale)

**签名**：`sr_cauchy_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_cdf(1.0, 0.0, 1.0)
-- 0.75
```

### sr_cauchy_sf(x, location, scale)

**签名**：`sr_cauchy_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_sf(1.0, 0.0, 1.0)
-- 0.25
```

### sr_cauchy_quantile(p, location, scale)

**签名**：`sr_cauchy_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_cauchy_quantile(0.75, 0.0, 1.0)
-- 1.0
```

## Laplace

### sr_laplace_pdf(x, location, scale)

**签名**：`sr_laplace_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_pdf(0.0, 0.0, 1.0)
-- 0.5
```

### sr_laplace_ln_pdf(x, location, scale)

**签名**：`sr_laplace_ln_pdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_ln_pdf(0.0, 0.0, 1.0)
-- -0.6931471805599453
```

### sr_laplace_cdf(x, location, scale)

**签名**：`sr_laplace_cdf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_cdf(1.0, 0.0, 1.0)
```

### sr_laplace_sf(x, location, scale)

**签名**：`sr_laplace_sf(x DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_sf(1.0, 0.0, 1.0)
```

### sr_laplace_quantile(p, location, scale)

**签名**：`sr_laplace_quantile(p DOUBLE, location DOUBLE, scale DOUBLE) -> DOUBLE`

```sql {"type":"duckfn","show":"value"}
SELECT sr_laplace_quantile(0.5, 0.0, 1.0)
-- 0.0
```
