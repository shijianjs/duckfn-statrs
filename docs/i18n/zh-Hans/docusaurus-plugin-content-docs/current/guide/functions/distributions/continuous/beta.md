---
title: Beta
sidebar_position: 3
description: Beta 分布——pdf / ln_pdf / cdf / sf / quantile。支撑 [0, 1]。
---

# Beta

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
