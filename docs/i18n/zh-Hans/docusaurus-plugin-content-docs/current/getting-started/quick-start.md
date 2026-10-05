---
title: 安装与加载
sidebar_position: 1
description: 如何将 duckfn_statrs 扩展安装到 DuckDB 并开始调用统计函数。
---

# 安装与加载

## 前置条件

- **DuckDB** 1.3 或更新版本（[CLI](https://duckdb.org/docs/stable/why_overview)、App 或任何语言客户端）。

## 从社区仓库安装

扩展注册进 DuckDB [社区扩展](https://duckdb.org/community_extensions/list_of_extensions) 之后，
安装只需两条语句：

```sql
INSTALL duckfn_statrs FROM community;
LOAD duckfn_statrs;
```

不需要额外参数——社区构建的产物已签名，并与你的 DuckDB 版本严格匹配。

## 从 GitHub Release 安装

如果社区路线尚不可用，从 [Releases 页面](https://github.com/shijianjs/duckfn-statrs/releases)
下载对应平台的产物：

```sql
-- 直接从 URL 加载（把平台后缀换成你自己的）
LOAD 'https://github.com/shijianjs/duckfn-statrs/releases/latest/download/duckfn_statrs-windows_amd64.duckdb_extension';
```

:::note

Release 产物没有 DuckDB 分发密钥的签名，启动 CLI 时需要加 `-unsigned` 参数：`duckdb -unsigned`。
上面的社区路线没有这个限制。

:::

## 验证安装

```sql
SELECT function_name, function_type
FROM duckdb_functions()
WHERE function_name LIKE 'sr_%'
ORDER BY function_name;
-- 511 行
```

## 第一批查询

描述统计量是**聚合函数**——传入一列，每个分组一个值：

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS std_dev, sr_median(x) AS median
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 10.0), (2, 20.0), (2, NULL)) t(g, x)
GROUP BY g
ORDER BY g;
```

分布函数是**标量**——逐行求值：

```sql {"type":"duckfn","show":"table"}
SELECT x, sr_normal_cdf(x, 0.0, 1.0) AS cdf, sr_normal_pdf(x, 0.0, 1.0) AS pdf
FROM (VALUES (-1.96::DOUBLE), (0.0), (1.96)) t(x);
```

参数非法时报错清晰明确：

```sql {"type":"duckfn","expect":"error"}
SELECT sr_normal_pdf(0.0, 0.0, -1.0);    -- 报错：sr_normal_pdf: Standard deviation is NaN, zero or less than zero
```

## NULL 行为

- NULL 输入行不进聚合（标准 SQL 惯例）。
- statrs 算不出结果时（空组、样本不足、参数越界），返回 SQL NULL——绝不是 NAN。
- 参数**存在但非法**（如 `std_dev <= 0`）报查询错误。

```sql {"type":"duckfn","show":"table"}
-- statrs 算不出单值的样本方差：结果是 NULL 而不是 0。
SELECT grp, sr_variance(x) AS variance
FROM (VALUES ('two', 1.5::DOUBLE), ('two', 2.5), ('one', NULL::DOUBLE), ('one', 3.0)) t(grp, x)
GROUP BY grp
ORDER BY grp;
```

## 下一步

- 按类别浏览完整的[函数参考](../guide/functions/overview.md)。
- 需要从源码构建或参与贡献？见[开发指南](./project-structure.md)。
