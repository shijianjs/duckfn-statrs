---
title: 总览
sidebar_position: 1
description: 函数参考各分页的索引——252 个统计函数的可执行 SQL 示例，按类别组织。
---

# 函数参考总览

`duckfn_statrs` 注册的 252 个函数都带 `sr_` 前缀。下面每个分页里的每个函数都至少配一个可在
浏览器里真跑的 SQL 示例。

在 SQL 里检索所有注册函数：

```sql
SELECT function_name, function_type, description
FROM duckdb_functions()
WHERE function_name LIKE 'sr_%'
ORDER BY function_name;
```

## 本节如何组织

侧边栏与函数族一致，分布再按种类细分：

- **描述统计量** —— 描述统计聚合与经验分布。
- **分布**
  - **连续** —— 正态 / 对数正态、Gamma 族、Beta、位置-尺度、极值与重尾、Student's t 与
    Fisher-Snedecor、均匀 / 三角 / Dirac。
  - **离散** —— Bernoulli 与二项试验、Poisson 与超几何、类别与离散均匀。
  - **多元** —— 多元正态、Dirichlet 与多项式。
- **特殊函数** —— 误差 / Gamma / Beta 族、阶乘、调和数、logistic/logit、多项式与核函数，
  外加**常量**。
- **随机抽样** —— 连续采样，以及离散 / 多元 / 经验采样。
- **密度与信号**、**假设检验**。

每个分页为每个函数写明签名、参数类型，并至少配一个可在浏览器里真跑的 SQL 示例。

## NULL 语义（全局规则）

- NULL 输入行不进聚合；标量函数遇到 NULL 参数短路返回 NULL。
- statrs 无法定义的结果（空组、样本不足、参数越界）一律返回 SQL NULL——绝不是 NAN。
- 参数**存在但非法**（如 `std_dev <= 0`）报查询错误。

## 常见命名模式

连续分布每种都提供 5 个函数：

```text
sr_<name>_pdf(x, params...)       概率密度
sr_<name>_ln_pdf(x, params...)    对数密度
sr_<name>_cdf(x, params...)       累积分布 P(X <= x)
sr_<name>_sf(x, params...)        生存函数 P(X > x)
sr_<name>_quantile(p, params...)  反 CDF
```

离散分布用 `pmf` / `ln_pmf` 替代 `pdf` / `ln_pdf`，其余同名。
