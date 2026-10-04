---
title: 特殊函数
sidebar_position: 14
description: 误差函数、Gamma 与 Beta 族、阶乘、调和数、Logistic/logit、多项式求值与核函数。
---

# 特殊函数

计算 statrs 在 `function/` 下暴露的数学本原的标量函数。除签名另有声明，参数一律是 DOUBLE。

## 误差函数族

### sr_erf(x)

**签名**：`sr_erf(x DOUBLE) -> DOUBLE`

误差函数 `erf(x)`。`erf(0) = 0`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erf(0.0)
-- 0.0
```

```sql {"type":"duckfn","show":"value"}
SELECT sr_erf(1.0)::DECIMAL(12,6)
-- 0.842701
```

### sr_erfc(x)

**签名**：`sr_erfc(x DOUBLE) -> DOUBLE`

互补误差函数 `erfc(x) = 1 - erf(x)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erfc(1.0)::DECIMAL(12,6)
-- 0.157299
```

### sr_erf_inv(y)

**签名**：`sr_erf_inv(y DOUBLE) -> DOUBLE`

反误差函数：使 `erf(x) = y` 的那个 x。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erf_inv(sr_erf(0.7))::DECIMAL(12,8)
-- 0.70000000
```

### sr_erfc_inv(y)

**签名**：`sr_erfc_inv(y DOUBLE) -> DOUBLE`

反互补误差函数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_erfc_inv(0.15729920705028513)::DECIMAL(12,6)
-- 1.000000
```

## Gamma 族

### sr_gamma(x)

**签名**：`sr_gamma(x DOUBLE) -> DOUBLE`

Gamma 函数 `Γ(x)`。整数入参时 `Γ(n+1) = n!`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma(5.0)
-- 24.0
```

### sr_ln_gamma(x)

**签名**：`sr_ln_gamma(x DOUBLE) -> DOUBLE`

Gamma 函数的自然对数 `ln(Γ(x))`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_gamma(5.0)::DECIMAL(12,6)
-- 3.178054
```

### sr_digamma(x)

**签名**：`sr_digamma(x DOUBLE) -> DOUBLE`

digamma 函数 `ψ(x)`，`ln(Γ(x))` 的导数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_digamma(2.0)::DECIMAL(12,8)
-- 0.42278434
```

### sr_inv_digamma(y)

**签名**：`sr_inv_digamma(y DOUBLE) -> DOUBLE`

反 digamma：使 `ψ(x) = y` 的那个 x。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inv_digamma(0.42278433509846713)::DECIMAL(12,8)
-- 2.00000000
```

### sr_gamma_lower_incomplete(a, x)

**签名**：`sr_gamma_lower_incomplete(a DOUBLE, x DOUBLE) -> DOUBLE`

下不完全 Gamma `γ(a, x)` —— 从 0 到 x 的积分。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_lower_incomplete(1.0, 1.0)::DECIMAL(12,8)
-- 0.63212056
```

### sr_gamma_upper_incomplete(a, x)

**签名**：`sr_gamma_upper_incomplete(a DOUBLE, x DOUBLE) -> DOUBLE`

上不完全 Gamma `Γ(a, x)` —— 从 x 到无穷大的积分。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_upper_incomplete(1.0, 1.0)::DECIMAL(12,8)
-- 0.36787944
```

### sr_gamma_lower_regularized(a, x)

**签名**：`sr_gamma_lower_regularized(a DOUBLE, x DOUBLE) -> DOUBLE`

正则化下不完全 Gamma `P(a, x) = γ(a, x) / Γ(a)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_lower_regularized(1.0, 1.0)::DECIMAL(12,8)
-- 0.63212056
```

### sr_gamma_upper_regularized(a, x)

**签名**：`sr_gamma_upper_regularized(a DOUBLE, x DOUBLE) -> DOUBLE`

正则化上不完全 Gamma `Q(a, x) = Γ(a, x) / Γ(a)`，与下正则化式互补：`P(a, x) + Q(a, x) = 1`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_upper_regularized(1.0, 1.0)::DECIMAL(12,8)
-- 0.36787944
```

## Beta 族

### sr_beta(a, b)

**签名**：`sr_beta(a DOUBLE, b DOUBLE) -> DOUBLE`

Beta 函数 `B(a, b)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta(2.0, 3.0)::DECIMAL(12,8)
-- 0.08333333
```

### sr_ln_beta(a, b)

**签名**：`sr_ln_beta(a DOUBLE, b DOUBLE) -> DOUBLE`

Beta 函数的自然对数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_beta(2.0, 3.0)::DECIMAL(12,8)
-- -2.48490665
```

### sr_beta_incomplete(x, a, b)

**签名**：`sr_beta_incomplete(a DOUBLE, b DOUBLE, x DOUBLE) -> DOUBLE`

不完全 Beta 函数 `B(x; a, b)`，从 0 到 x 积分。

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_incomplete(2.0, 3.0, 0.5)
```

### sr_beta_regularized(a, b, x)

**签名**：`sr_beta_regularized(a DOUBLE, b DOUBLE, x DOUBLE) -> DOUBLE`

正则化不完全 Beta `I(x; a, b)`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_regularized(2.0, 3.0, 0.5)::DECIMAL(12,8)
-- 0.68750000
```

### sr_inv_beta_regularized(a, b, p)

**签名**：`sr_inv_beta_regularized(a DOUBLE, b DOUBLE, p DOUBLE) -> DOUBLE`

正则化不完全 Beta 的反函数：使 `I(x; a, b) = p` 的那个 x。

```sql {"type":"duckfn","show":"value"}
SELECT sr_inv_beta_regularized(2.0, 3.0, 0.5)::DECIMAL(12,8)
-- 0.38572757
```

## 阶乘与组合

### sr_factorial(n)

**签名**：`sr_factorial(n DOUBLE) -> DOUBLE`

阶乘 `n!`。`n` 必须是整数值 DOUBLE。

```sql {"type":"duckfn","show":"value"}
SELECT sr_factorial(10.0)
-- 3628800.0
```

### sr_ln_factorial(n)

**签名**：`sr_ln_factorial(n DOUBLE) -> DOUBLE`

`n!` 的自然对数。

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_factorial(10.0)::DECIMAL(12,8)
-- 15.10441257
```

### sr_choose(n, k)

**签名**：`sr_choose(n DOUBLE, k DOUBLE) -> DOUBLE`

二项系数 `C(n, k)`。两个参数都必须是整数值 DOUBLE。

```sql {"type":"duckfn","show":"value"}
SELECT sr_choose(10.0, 3.0)
-- 120.0
```

### sr_ln_choose(n, k)

**签名**：`sr_ln_choose(n DOUBLE, k DOUBLE) -> DOUBLE`

二项系数的自然对数。

```sql {"type":"duckfn","show":"value"}
SELECT exp(sr_ln_choose(10.0, 3.0))::DECIMAL(12,8)
-- 120.00000000
```

## 调和数

### sr_harmonic(n)

**签名**：`sr_harmonic(n DOUBLE) -> DOUBLE`

调和数 `H(n) = sum_{k=1..n} 1/k`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_harmonic(10.0)::DECIMAL(12,8)
-- 2.92896825
```

### sr_generalized_harmonic(n, m)

**签名**：`sr_generalized_harmonic(n DOUBLE, m DOUBLE) -> DOUBLE`

广义调和数 `sum_{k=1..n} 1/k^m`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_generalized_harmonic(10.0, 2.0)::DECIMAL(12,8)
-- 1.54976773
```

## Logistic / logit

### sr_logistic(p)

**签名**：`sr_logistic(p DOUBLE) -> DOUBLE`

sigmoid `1 / (1 + exp(-p))`。

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic(0.0)
-- 0.5
```

### sr_logit(p)

**签名**：`sr_logit(p DOUBLE) -> DOUBLE`

反 sigmoid `ln(p / (1 - p))`。端点（`0` 与 `1`）给无穷大；`[0, 1]` 之外的值给 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_logit(sr_logistic(1.3))::DECIMAL(12,8)
-- 1.30000000
```

```sql {"type":"duckfn","show":"value"}
SELECT sr_logit(1.2)
-- NULL (out of [0, 1])
```

## 指数积分

### sr_exponential_integral(n, x)

**签名**：`sr_exponential_integral(n DOUBLE, x DOUBLE) -> DOUBLE`

指数积分 `E_n(x)`，要求 `n >= 0` 且 `x >= 0`。statrs 算不出的地方给 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_exponential_integral(1.0, 0.0)
-- inf
```

## 多项式

### sr_polynomial(x, coefficients)

**签名**：`sr_polynomial(x DOUBLE, coefficients LIST(DOUBLE)) -> DOUBLE`

求 `sum coeff[i] * x^i`，`coefficients` 按幂次升序。

```sql {"type":"duckfn","show":"value"}
SELECT sr_polynomial(2.0, [1.0, 0.0, 3.0])
-- 13.0
```

## 核函数

### sr_kernel_eval(x, kind)

**签名**：`sr_kernel_eval(x DOUBLE, kind DOUBLE) -> DOUBLE`

核函数 `K(x)` 求值。kind 代码：1=gaussian、2=epanechnikov、3=triangular、4=tricube、
5=quartic、6=uniform、7=cosine、8=logistic、9=sigmoid。

```sql {"type":"duckfn","show":"value"}
SELECT sr_kernel_eval(2.0, 0.0)
```

### sr_kernel_support(kind)

**签名**：`sr_kernel_support(kind DOUBLE) -> LIST(DOUBLE)`

核的紧支撑区间 `[lo, hi]`。非紧支撑核（gaussian、sigmoid、logistic）给 NULL。

```sql {"type":"duckfn","show":"value"}
SELECT sr_kernel_support(2.0)
-- [-1.0, 1.0]
```
