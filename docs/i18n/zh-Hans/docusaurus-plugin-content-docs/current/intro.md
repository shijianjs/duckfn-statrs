---
title: 简介
sidebar_position: 1
slug: /intro
description: duckfn_statrs 把 Rust 统计库 statrs 包装成 DuckDB 函数——描述统计量是聚合函数，正态分布是标量函数。
---

# 简介

`duckfn_statrs` 是一个 DuckDB [loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)，
把 Rust 统计计算库 [statrs](https://crates.io/crates/statrs) 包装成可以直接在 SQL 里调用的函数：
描述统计量做成**聚合函数**（`SELECT sr_mean(x) FROM t GROUP BY g` 这样的写法），正态分布的
pdf / cdf / 分位数做成**标量函数**（逐行求值）。代码用 [duckfn](https://crates.io/crates/duckfn)
的属性宏写成，建立在 [duckfn-extension-template](https://github.com/shijianjs/duckfn-extension-template)
之上 —— 仓库里已经装好了构建、测试、文档与发版链路；DuckDB 的 C API 只用到头文件，所以除了扩展
本身，没有任何东西需要编译。

计算全部交给 statrs：本扩展不重新实现任何统计公式。汇总类聚合只差一个
`#[duck_aggregate_function(auto_collect = true)]` 属性 —— 被注解的函数本身*就是* finalize
处理器，`Vec<T>` 参数就是逐行收集的列，状态由宏生成（duckfn 0.0.18+）：

```rust
use duckfn::{DuckOptionResult, duck_aggregate_function};

#[duck_aggregate_function(
    auto_collect = true,
    description = "Arithmetic mean of a DOUBLE column, NULL when no row is non-NULL",
    example = "SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x)"
)]
fn sr_mean(values: Vec<f64>) -> DuckOptionResult<f64> {
    nan_to_null(values.mean())  // statrs 的 NAN 落成 SQL NULL
}
```

同一批函数的 SQL 一面。这个块会在你的浏览器里真跑：站点从仓库的最新 Release 预加载了这个扩展，
所以这里不用写 `LOAD`。

```sql {"type":"duckfn","show":"table"}
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS std_dev
FROM (VALUES (1, 1.0), (1, 2.0), (1, 3.0), (2, 10.0), (2, 20.0)) t(g, x)
GROUP BY g ORDER BY g;
```

## NULL 语义

整条规则一句话说完：**statrs 算不出的就是 SQL NULL，NULL 输入也永远不会悄悄变成一个数。**
具体拆开：NULL 行不进聚合（SQL 聚合惯例，与 DuckDB 自带的 mean/stddev 一致）；statrs 对空组、
单值的样本方差、越界的 `tau`、几何/调和平均里的负数返回的 NAN，统一折成 NULL。参数**存在但
非法**（`std_dev <= 0`、概率不在 `[0, 1]` 内）是调用写错了，报查询错误，而不是被静默折成空。

## 仓库里有什么

| 路径 | 是什么 |
| --- | --- |
| `src/extension/mod.rs` | 入口：`duckfn_entrypoint!("duckfn_statrs")` 与模块树。 |
| `src/extension/functions/` | 注册进 DuckDB 的函数，目录与 statrs 模块树同构：`consts.rs`、`function.rs`、`statistics/`（聚合）、`distribution/`（27 个一元分布，标量）；对应表与有意未包装清单在其 `mod.rs` 头注释里。 |
| `src/extension/types/` | 面向 SQL 的类型放这里（STRUCT / ENUM 定义、`list<struct>` 行类型）。目前是空的。 |
| `test/sql/` | SQLLogicTest 用例，每个函数组一份，外加一份冒烟测试；期望值全部取自 statrs 的实际输出。 |
| `Justfile` | 日常命令：构建、跑一条 SQL、REPL、测试、lint、发版。 |
| `.github/workflows/` | 构建矩阵、版本 tag 上的 GitHub Release、以及这份站点的部署。 |
| `community-extension/` | 注册 [社区扩展](https://duckdb.org/community_extensions/list_of_extensions) 需要的那两份文件。 |
| `docs/` | 这份站点：Docusaurus，中英双语。 |

## 接下来去哪

- [快速开始](./getting-started/quick-start.md) —— 构建、加载、调用。
- [项目结构](./getting-started/project-structure.md) —— 入口、函数、类型各在哪里，把它们拴在一起的命名规则。
- [写函数](./guide/functions.md) —— 已注册的函数与各自用的形状。
- [测试](./guide/testing.md) —— SQLLogicTest 用例与怎么跑。
- [构建与发版](./build-and-release.md) —— 构建路径与发版流程。
- [社区扩展](./community-extension.md) —— 发布到 DuckDB 社区仓库。
