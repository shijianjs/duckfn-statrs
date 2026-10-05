---
title: 常量
sidebar_position: 2
description: statrs::consts 里的数学常量，加上 statrs 自己比较浮点结果用的精度阈值与 almost_eq 比较器，都做成零参标量函数。
---

# 常量

## 数学常量

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

## 精度阈值

下面这组来自 statrs 自己的 `prec` 模块 —— statrs 内部比较浮点结果时用的容差。暴露出来是为了
让查询能沿用 statrs 的「算相等」口径，而不必自己另定一个 epsilon。

### sr_f64_prec()

**签名**：`sr_f64_prec() -> DOUBLE`

`F64_PREC`：IEEE 754 双精度的最大相对精度，即 `2^-53`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_f64_prec()
-- 1.1102230246251565e-16
```

### sr_default_f64_acc()

**签名**：`sr_default_f64_acc() -> DOUBLE`

`DEFAULT_F64_ACC`：statrs 对 f64 的默认绝对比较容差，恰为 `0.01 * F64_PREC`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_f64_acc()
-- 1.1102230246251565e-15
```

### sr_default_relative_acc()

**签名**：`sr_default_relative_acc() -> DOUBLE`

`DEFAULT_RELATIVE_ACC`：statrs 的 f64 运算默认相对精度目标。

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_relative_acc()
-- 1e-14
```

### sr_default_eps()

**签名**：`sr_default_eps() -> DOUBLE`

`DEFAULT_EPS`：statrs 的 f64 运算默认绝对精度目标。

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_eps()
-- 1e-09
```

### sr_default_ulps()

**签名**：`sr_default_ulps() -> DOUBLE`

`DEFAULT_ULPS`：statrs 的 f64 运算默认 ULP 精度目标。它在 statrs 里是 `u32`，这里返回 DOUBLE
以保持「零参标量返回 DOUBLE」的约定；5 本身可精确表示。

```sql {"type":"duckfn","show":"value"}
SELECT sr_default_ulps()
-- 5.0
```

### sr_almost_eq(a, b, acc)

**签名**：`sr_almost_eq(a DOUBLE, b DOUBLE, acc DOUBLE) -> BOOLEAN`

`a` 与 `b` 的绝对差是否在 `acc` 之内 —— 即 statrs 的 `almost_eq`。语义完全照 statrs：两边都
是无穷时只有相等才算接近；出现 NaN 则永不匹配（连与自身也不匹配），所以
`sr_almost_eq(NaN, NaN, 'inf')` 是 `false`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_almost_eq(0.1 + 0.2, 0.3, sr_default_relative_acc())
-- true
```

把 `acc` 传 `'inf'` 就得到一个能正确处理 NaN 的精确相等判断。

```sql {"type":"duckfn","show":"value"}
SELECT sr_almost_eq(1.0, 1.0, 'inf'::DOUBLE)
-- true
```
