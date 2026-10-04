---
title: Special functions
sidebar_position: 1
description: Error functions, gamma and beta families, factorials and combinatorial, harmonic numbers, logistic/logit, polynomial evaluation, kernel helpers, and Euclidean modulus.
---

# Special functions

Scalar functions evaluating the mathematical primitives statrs exposes under `function/`.
Every argument is DOUBLE unless the signature says otherwise.

## Error function family

### sr_erf(x)

**Signature**: `sr_erf(x DOUBLE) -> DOUBLE`

The error function `erf(x)`. `erf(0) = 0`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_erf(0.0)
-- 0.0
```

```sql {"type":"duckfn","show":"value"}
SELECT sr_erf(1.0)::DECIMAL(12,6)
-- 0.842701
```

### sr_erfc(x)

**Signature**: `sr_erfc(x DOUBLE) -> DOUBLE`

Complementary error function `erfc(x) = 1 - erf(x)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_erfc(1.0)::DECIMAL(12,6)
-- 0.157299
```

### sr_erf_inv(y)

**Signature**: `sr_erf_inv(y DOUBLE) -> DOUBLE`

Inverse error function: the x with `erf(x) = y`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_erf_inv(sr_erf(0.7))::DECIMAL(12,8)
-- 0.70000000
```

### sr_erfc_inv(y)

**Signature**: `sr_erfc_inv(y DOUBLE) -> DOUBLE`

Inverse complementary error function.

```sql {"type":"duckfn","show":"value"}
SELECT sr_erfc_inv(0.15729920705028513)::DECIMAL(12,6)
-- 1.000000
```

## Gamma family

### sr_gamma(x)

**Signature**: `sr_gamma(x DOUBLE) -> DOUBLE`

Gamma function `Γ(x)`. For whole-number inputs `Γ(n+1) = n!`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma(5.0)
-- 24.0
```

### sr_ln_gamma(x)

**Signature**: `sr_ln_gamma(x DOUBLE) -> DOUBLE`

Natural log of the Gamma function, `ln(Γ(x))`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_gamma(5.0)::DECIMAL(12,6)
-- 3.178054
```

### sr_digamma(x)

**Signature**: `sr_digamma(x DOUBLE) -> DOUBLE`

Digamma function `ψ(x)`, the derivative of `ln(Γ(x))`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_digamma(2.0)::DECIMAL(12,8)
-- 0.42278434
```

### sr_inv_digamma(y)

**Signature**: `sr_inv_digamma(y DOUBLE) -> DOUBLE`

Inverse digamma: the x with `ψ(x) = y`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inv_digamma(0.42278433509846713)::DECIMAL(12,8)
-- 2.00000000
```

### sr_gamma_lower_incomplete(a, x)

**Signature**: `sr_gamma_lower_incomplete(a DOUBLE, x DOUBLE) -> DOUBLE`

Lower incomplete Gamma `γ(a, x)` — integrates from 0 to x.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_lower_incomplete(1.0, 1.0)::DECIMAL(12,8)
-- 0.63212056
```

### sr_gamma_upper_incomplete(a, x)

**Signature**: `sr_gamma_upper_incomplete(a DOUBLE, x DOUBLE) -> DOUBLE`

Upper incomplete Gamma `Γ(a, x)` — integrates from x to infinity.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_upper_incomplete(1.0, 1.0)::DECIMAL(12,8)
-- 0.36787944
```

### sr_gamma_lower_regularized(a, x)

**Signature**: `sr_gamma_lower_regularized(a DOUBLE, x DOUBLE) -> DOUBLE`

Regularized lower incomplete Gamma `P(a, x) = γ(a, x) / Γ(a)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_lower_regularized(1.0, 1.0)::DECIMAL(12,8)
-- 0.63212056
```

### sr_gamma_upper_regularized(a, x)

**Signature**: `sr_gamma_upper_regularized(a DOUBLE, x DOUBLE) -> DOUBLE`

Regularized upper incomplete Gamma `Q(a, x) = Γ(a, x) / Γ(a)`. Complementary to the lower
regularized form: `P(a, x) + Q(a, x) = 1`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_gamma_upper_regularized(1.0, 1.0)::DECIMAL(12,8)
-- 0.36787944
```

## Beta family

### sr_beta(a, b)

**Signature**: `sr_beta(a DOUBLE, b DOUBLE) -> DOUBLE`

Beta function `B(a, b)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta(2.0, 3.0)::DECIMAL(12,8)
-- 0.08333333
```

### sr_ln_beta(a, b)

**Signature**: `sr_ln_beta(a DOUBLE, b DOUBLE) -> DOUBLE`

Natural log of the Beta function.

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_beta(2.0, 3.0)::DECIMAL(12,8)
-- -2.48490665
```

### sr_beta_incomplete(x, a, b)

**Signature**: `sr_beta_incomplete(a DOUBLE, b DOUBLE, x DOUBLE) -> DOUBLE`

Incomplete Beta function `B(x; a, b)`, integrating from 0 to x.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_incomplete(2.0, 3.0, 0.5)
```

### sr_beta_regularized(a, b, x)

**Signature**: `sr_beta_regularized(a DOUBLE, b DOUBLE, x DOUBLE) -> DOUBLE`

Regularized incomplete Beta `I(x; a, b)`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_beta_regularized(2.0, 3.0, 0.5)::DECIMAL(12,8)
-- 0.68750000
```

### sr_inv_beta_regularized(a, b, p)

**Signature**: `sr_inv_beta_regularized(a DOUBLE, b DOUBLE, p DOUBLE) -> DOUBLE`

Inverse of the regularized incomplete Beta: the x where `I(x; a, b) = p`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_inv_beta_regularized(2.0, 3.0, 0.5)::DECIMAL(12,8)
-- 0.38572757
```

## Factorial and combinatorial

### sr_factorial(n)

**Signature**: `sr_factorial(n DOUBLE) -> DOUBLE`

Factorial `n!`. `n` must be a whole-number DOUBLE.

```sql {"type":"duckfn","show":"value"}
SELECT sr_factorial(10.0)
-- 3628800.0
```

### sr_ln_factorial(n)

**Signature**: `sr_ln_factorial(n DOUBLE) -> DOUBLE`

Natural log of `n!`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_ln_factorial(10.0)::DECIMAL(12,8)
-- 15.10441257
```

### sr_choose(n, k)

**Signature**: `sr_choose(n DOUBLE, k DOUBLE) -> DOUBLE`

Binomial coefficient `C(n, k)`. Both arguments must be whole-number DOUBLEs.

```sql {"type":"duckfn","show":"value"}
SELECT sr_choose(10.0, 3.0)
-- 120.0
```

### sr_ln_choose(n, k)

**Signature**: `sr_ln_choose(n DOUBLE, k DOUBLE) -> DOUBLE`

Natural log of the binomial coefficient.

```sql {"type":"duckfn","show":"value"}
SELECT exp(sr_ln_choose(10.0, 3.0))::DECIMAL(12,8)
-- 120.00000000
```

### sr_multinomial_coefficient(total, counts)

**Signature**: `sr_multinomial_coefficient(total DOUBLE, counts BIGINT[]) -> DOUBLE`

Multinomial coefficient n! / (n1! n2! ...) over a whole-number n and a count LIST(BIGINT); NULL when the counts do not sum to n.

```sql {"type":"duckfn","show":"value"}
SELECT sr_multinomial_coefficient(5.0, [2, 2, 1])
-- 30.0
```

## Harmonic numbers

### sr_harmonic(n)

**Signature**: `sr_harmonic(n DOUBLE) -> DOUBLE`

Harmonic number `H(n) = sum_{k=1..n} 1/k`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_harmonic(10.0)::DECIMAL(12,8)
-- 2.92896825
```

### sr_generalized_harmonic(n, m)

**Signature**: `sr_generalized_harmonic(n DOUBLE, m DOUBLE) -> DOUBLE`

Generalized harmonic number `sum_{k=1..n} 1/k^m`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_generalized_harmonic(10.0, 2.0)::DECIMAL(12,8)
-- 1.54976773
```

## Logistic / logit

### sr_logistic(p)

**Signature**: `sr_logistic(p DOUBLE) -> DOUBLE`

Sigmoid `1 / (1 + exp(-p))`.

```sql {"type":"duckfn","show":"value"}
SELECT sr_logistic(0.0)
-- 0.5
```

### sr_logit(p)

**Signature**: `sr_logit(p DOUBLE) -> DOUBLE`

Inverse sigmoid `ln(p / (1 - p))`. Endpoints (`0` and `1`) yield infinity; values outside `[0, 1]`
yield NULL.

```sql {"type":"duckfn","show":"value"}
SELECT sr_logit(sr_logistic(1.3))::DECIMAL(12,8)
-- 1.30000000
```

```sql {"type":"duckfn","show":"value"}
SELECT sr_logit(1.2)
-- NULL (out of [0, 1])
```

## Exponential integral

### sr_exponential_integral(n, x)

**Signature**: `sr_exponential_integral(n DOUBLE, x DOUBLE) -> DOUBLE`

Exponential integral `E_n(x)` for `n >= 0` and `x >= 0`. NULL where statrs leaves it undefined.

```sql {"type":"duckfn","show":"value"}
SELECT sr_exponential_integral(1.0, 0.0)
-- inf
```

## Polynomial

### sr_polynomial(x, coefficients)

**Signature**: `sr_polynomial(x DOUBLE, coefficients LIST(DOUBLE)) -> DOUBLE`

Evaluate `sum coeff[i] * x^i` where `coefficients` is in ascending order.

```sql {"type":"duckfn","show":"value"}
SELECT sr_polynomial(2.0, [1.0, 0.0, 3.0])
-- 13.0
```

## Kernel functions

### sr_kernel_eval(x, kind)

**Signature**: `sr_kernel_eval(x DOUBLE, kind DOUBLE) -> DOUBLE`

Evaluate kernel `K(x)`. Kind codes: 1=gaussian, 2=epanechnikov, 3=triangular, 4=tricube,
5=quartic, 6=uniform, 7=cosine, 8=logistic, 9=sigmoid.

```sql {"type":"duckfn","show":"value"}
SELECT sr_kernel_eval(2.0, 0.0)
```

### sr_kernel_support(kind)

**Signature**: `sr_kernel_support(kind DOUBLE) -> LIST(DOUBLE)`

Compact support interval `[lo, hi]` of the kernel. NULL for kernels with unbounded support
(gaussian, sigmoid, logistic).

```sql {"type":"duckfn","show":"value"}
SELECT sr_kernel_support(2.0)
-- [-1.0, 1.0]
```

### sr_kernel_eval_with_bandwidth(kind, x, bandwidth)

**Signature**: `sr_kernel_eval_with_bandwidth(kind DOUBLE, x DOUBLE, bandwidth DOUBLE) -> DOUBLE`

Kernel function with bandwidth scaling K(x / h) / h (same kind codes as sr_kernel_eval).

```sql {"type":"duckfn","show":"value"}
SELECT sr_kernel_eval_with_bandwidth(1.0, 0.0, 0.5)
-- 0.7978845608028654
```

## Euclidean modulus

### sr_modulus(x, divisor)

**Signature**: `sr_modulus(x DOUBLE, divisor DOUBLE) -> DOUBLE`

Canonical (Euclidean) modulus ((x % divisor) + divisor) % divisor, always in [0, divisor); NULL for a zero divisor.

```sql {"type":"duckfn","show":"value"}
SELECT sr_modulus(-1.0, 5.0)
-- 4.0
```
