---
title: 常量
sidebar_position: 2
description: statrs::consts 里的 7 个数学常量，做成零参标量函数。
---

# 常量

零参标量函数，返回 DOUBLE 类型的数学常量。

### sr_ln_pi()

**签名**：`sr_ln_pi() -> DOUBLE`

常量 ln(PI)。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_pi()
-- 1.1447298858494002
```

### sr_sqrt_2pi()

**签名**：`sr_sqrt_2pi() -> DOUBLE`

常量 sqrt(2*PI)，正态密度里的归一化因子。

```sql {"type":"duckfn","show":"value"}
SELECT sr_sqrt_2pi()
-- 2.5066282746310002
```

### sr_ln_sqrt_2pi()

**签名**：`sr_ln_sqrt_2pi() -> DOUBLE`

常量 ln(sqrt(2*PI))。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_sqrt_2pi()
-- 0.9189385332046727
```

### sr_ln_sqrt_2pie()

**签名**：`sr_ln_sqrt_2pie() -> DOUBLE`

常量 ln(sqrt(2*PI*e))。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_sqrt_2pie()
-- 1.4189385332046727
```

### sr_2_sqrt_e_over_pi()

**签名**：`sr_2_sqrt_e_over_pi() -> DOUBLE`

常量 2*sqrt(e/PI)。

```sql {"type":"duckfn","show":"value"}
SELECT sr_2_sqrt_e_over_pi()
-- 2.2964211229736176
```

### sr_ln_2_sqrt_e_over_pi()

**签名**：`sr_ln_2_sqrt_e_over_pi() -> DOUBLE`

常量 ln(2*sqrt(e/PI))。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_2_sqrt_e_over_pi()
-- 0.8313224962216493
```

### sr_euler_mascheroni()

**签名**：`sr_euler_mascheroni() -> DOUBLE`

欧拉-马斯刻若尼常数（γ）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_euler_mascheroni()
-- 0.5772156649015329
```
