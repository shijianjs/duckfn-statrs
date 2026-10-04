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

## 按类别浏览

用左边的侧边栏跳到各函数族。每个分页为每个函数写明签名、参数类型，并至少配一个可执行示例。

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
