---
title: 密度估计与信号生成
sidebar_position: 6
description: KDE 与 kNN 密度估计；正弦、方波、三角、锯齿、周期与对数间距生成器。
---

# 密度估计与信号生成

## 密度估计

### sr_kde_pdf(x, sample, bandwidth)

**签名**：`sr_kde_pdf(x DOUBLE, sample LIST(DOUBLE), bandwidth DOUBLE) -> DOUBLE`

在 `x` 处的高斯核密度估计，样本是 `sample` LIST，用 k-d tree 加速。`bandwidth` 传 NULL 让
statrs 自动选择带宽。

空样本是 statrs 的 `EmptySample` 错误（不是 NULL）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_kde_pdf(0.5, [0.0, 0.2, 0.7, 1.0], 0.5)
-- 0.5927180961167049
```

```sql {"type":"duckfn","show":"value"}
-- NULL 带宽走 statrs 的自动带宽（与手动指定的结果不同）：
SELECT sr_kde_pdf(0.5, [0.0, 0.2, 0.7, 1.0], NULL)
-- 0.6336441730224942
```

### sr_knn_pdf(x, sample, bandwidth)

**签名**：`sr_knn_pdf(x DOUBLE, sample LIST(DOUBLE), bandwidth DOUBLE) -> DOUBLE`

在 `x` 处的 k 近邻密度估计。`bandwidth` 传 NULL 使用自动 k。

```sql {"type":"duckfn","show":"value"}
SELECT sr_knn_pdf(0.5, [0.0, 0.2, 0.7, 1.0], 0.5)
-- 1.0000000000000018
```

## 信号生成

所有生成器都返回 `LIST(DOUBLE)`。`k` 是 **BIGINT**（从无限序列里取多少个点）。`delay` 也是 BIGINT。

### sr_gen_sinusoidal(k, sample_rate, frequency, amplitude, mean, phase, delay)

**签名**：`sr_gen_sinusoidal(k BIGINT, sample_rate DOUBLE, frequency DOUBLE, amplitude DOUBLE, mean DOUBLE, phase DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

正弦波的前 k 个点。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_sinusoidal(4, 4.0, 1.0, 1.0, 0.0, 0.0, 0)
-- [0.0, 1.0, 1.2246467991473532e-16, -1.0]
```

### sr_gen_square(k, high, low, amplitude, offset, delay)

**签名**：`sr_gen_square(k BIGINT, high BIGINT, low BIGINT, amplitude DOUBLE, offset DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

方波的前 k 个点。`high` 与 `low` 是两个电平各持续多少采样。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_square(8, 2, 2, 1.0, 0.0, 0)
-- [1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]
```

### sr_gen_triangle(k, raise, fall, amplitude, offset, delay)

**签名**：`sr_gen_triangle(k BIGINT, raise BIGINT, fall BIGINT, amplitude DOUBLE, offset DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

三角波的前 k 个点。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_triangle(6, 1, 1, 1.0, 0.0, 0)
```

### sr_gen_sawtooth(k, period, amplitude, offset, delay)

**签名**：`sr_gen_sawtooth(k BIGINT, period BIGINT, amplitude DOUBLE, offset DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

锯齿波的前 k 个点，`period` 是周期（采样数）。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_sawtooth(5, 4, 1.0, 0.0, 0)
-- [0.0, 0.3333333333333333, 0.6666666666666666, 1.0, 0.0]
```

### sr_gen_periodic(k, sample_rate, frequency, amplitude, phase, delay)

**签名**：`sr_gen_periodic(k BIGINT, sample_rate DOUBLE, frequency DOUBLE, amplitude DOUBLE, phase DOUBLE, delay BIGINT) -> LIST(DOUBLE)`

通用周期信号的前 k 个点。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_periodic(10, 8.0, 1.0, 1.0, 0.0, 0)
```

### sr_gen_log_spaced(n, start_exp, stop_exp)

**签名**：`sr_gen_log_spaced(n DOUBLE, start_exp DOUBLE, stop_exp DOUBLE) -> LIST(DOUBLE)`

在 `10^start_exp` 与 `10^stop_exp` 之间取 n 个对数间距的点。`n` 是整数值 DOUBLE。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gen_log_spaced(3.0, 0.0, 2.0)
-- [1.0, 10.0, 100.0]
```

## 错误

非正的 `k` 是调用错误（不是 NULL）：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_gen_square(0, 2, 2, 1.0, 0.0, 0)
-- error: sr_gen_square: k must be a positive whole number, got 0
```
