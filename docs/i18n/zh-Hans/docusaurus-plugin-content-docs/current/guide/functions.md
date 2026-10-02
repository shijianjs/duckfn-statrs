---
title: 写函数
sidebar_position: 1
description: 已注册的 statrs 包装函数与各自用的形状、duckfn 对参数和返回值的规则，以及加新函数时该抄哪一份。
---

# 写函数

一个注册进 DuckDB 的函数就是「一个普通 Rust 函数 + 一个属性宏」。宏负责写 FFI 包装、读参数列、写
结果列、提交注册；函数体里只有你的业务逻辑。

从普通函数到可调用 SQL 函数的路径：

```mermaid
flowchart LR
    fn["普通 Rust 函数"] --> macro["duck 属性宏"]
    macro --> wrapper["FFI 包装与<br/>注册项"]
    wrapper --> cdylib["扩展二进制<br/>cdylib"]
    cdylib --> load["LOAD 进 DuckDB"]
```

## 这个扩展注册了什么

| 函数 | 类别 | 签名 | 行为 |
| --- | --- | --- | --- |
| `sr_mean` 及均值 / 方差族 | 聚合 | `DOUBLE -> DOUBLE` | 状态收集整列，finalize 时交给 statrs 计算。 |
| `sr_quantile` | 聚合 | `(DOUBLE, DOUBLE) -> DOUBLE` | tau 是第二个逐行但恒定的参数，存进状态。 |
| `sr_covariance` / `sr_population_covariance` | 聚合 | `(DOUBLE, DOUBLE) -> DOUBLE` | 两列按行配对；任一列为 NULL 的行整行跳过。 |
| `sr_normal_pdf` / `sr_normal_cdf` / `sr_normal_quantile` | 标量 | `3 × DOUBLE -> DOUBLE` | 逐行求值；非法参数报查询错误。 |

代码在 `src/extension/functions/`，一个函数组一个文件：`aggregate_summary.rs`、
`aggregate_covariance.rs`、`scalar_normal.rs`。统计量刻意做成聚合 —— `SELECT sr_mean(x) FROM t
GROUP BY g` 就是数据库用户本来就会写的形状；换成 LIST + 标量，每个调用点前都得先 `list(x)`。

## 标量：三种返回形状

宏按返回类型生成不同的代码：

| 签名 | 含义 |
| --- | --- |
| `-> T` | 朴素值，永不为 NULL。 |
| `-> DuckOptionResult<T>` | 可空 + 可报错：`Ok(None)` 落成 SQL `NULL`，`Err` 让整条查询失败。 |
| `-> Option<T>` | 可空，但报不了错。 |

```rust
use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};

#[duck_scalar_function(
    description = "Normal (Gaussian) quantile function: the x whose CDF equals p, for p in [0, 1]",
    comment = "A probability outside [0, 1] is a query error rather than a clamped endpoint",
    example = "SELECT sr_normal_quantile(0.975, 0.0, 1.0)"
)]
fn sr_normal_quantile(p: f64, mean: f64, std_dev: f64) -> DuckOptionResult<f64> {
    if !(0.0..=1.0).contains(&p) {
        return Err(duck_error(format!(
            "sr_normal_quantile: the probability must be within [0, 1], got {p}"
        )));
    }
    let normal = normal("sr_normal_quantile", mean, std_dev)?;
    Ok(Some(normal.inverse_cdf(p)))
}
```

### NULL 参数会怎么样

参数能不能带 NULL 由参数类型决定，聚合也遵守同一条规则：

- **`p: f64`** —— NULL 输入行被 duckfn 的参数读取层短路成 SQL `NULL`，函数体不会执行到那一行；
  在聚合里，这行根本不进状态。大多数时候你要的就是这个。
- **`p: Option<f64>`** —— NULL 以 `None` 的身份进入函数体，语义由你定（返回 NULL、给默认值、
  统计 NULL 数……）。

### 错误与 panic

遇到函数处理不了的值就返回 `Err(duck_error("…"))`，整条查询带着你的文案失败。函数体里的 `panic!`
会被捕获、报成 DuckDB 错误，而不是跨 FFI 边界展开。错误文案是面向用户的：用英文写，前面带上函数名，
让一条报告单独也看得懂。本扩展里「报错」与「NULL」的界线是刻意划的：*statrs 算不出* → NULL
（经 `nan_to_null`）；*调用写错*（std_dev ≤ 0、概率越界）→ 报错。

## 聚合：逐行输入 + 一个状态

聚合的签名是「输入列 + 一个 `&mut` 状态参数」（位置随意）。状态要满足 `Default + Clone + Debug`
（宏生成的包装结构体 derive 了它们），再实现 `DuckAggregateState`：

- `combine` / `simple_combine` 合并两个状态。线程并行与分组合并都走这里，所以它必须满足结合律。
- `result` / `simple_result` 把状态变成这一组的值。`simple_result` 只能给永不为 NULL 的值；
  一组要落成 SQL NULL 时覆盖 `result`、返回 `Ok(None)`。
- `Output` 决定 SQL 返回类型：`i64`、`f64`、`String`、`Vec<…>`（即 `list<…>`）等。

分位数状态把两个活动件都摆出来了 —— 收集的数据，加上每行读一次的常量参数：

```rust
#[derive(Default, Debug, Clone)]
struct QuantileState {
    values: Vec<f64>,
    tau: Option<f64>,
}

impl DuckAggregateState for QuantileState {
    type Output = f64;

    fn simple_combine(&mut self, other: &Self) {
        self.values.extend(other.values.iter().copied());
        self.tau = self.tau.or(other.tau);      // 同一条查询里常量处处相同
    }

    fn result(&self) -> DuckOptionResult<f64> {
        let Some(tau) = self.tau else {
            return Ok(None);                    // 空组 -> SQL NULL
        };
        let mut data = Data::new(self.values.clone());
        super::nan_to_null(data.quantile(tau))  // statrs 的 NAN -> SQL NULL
    }
}

#[duck_aggregate_function(/* description / comment / example */)]
fn sr_quantile(input: f64, tau: f64, state: &mut QuantileState) {
    state.values.push(input);
    state.tau = Some(tau);
}
```

均值 / 方差族就是同一个形状重复九遍、只差最后的算式，所以 `aggregate_summary.rs` 用一个
`collect_aggregate!` 宏把「状态 + NAN→NULL 的 `result` + 行处理器」三件套一次生成。状态里同样可以
放 `String`、`HashMap`、`Vec<…>`，或按分组键一格一位 —— 支持哪些形状见 duckfn 指南的聚合章节。

## 加你自己的函数

1. **选属性。** 标量、聚合、表函数、`COPY`、cast、SQL 宏、replacement scan：各有各的属性，
   也各只接受自己的参数。手册是 [duckfn 用户指南](https://shijianjs.github.io/duckfn/) ——
   直接翻到对应种类那一章。
2. **抄最近的那一个**（`src/extension/functions/` 里），改逻辑，别从零发明签名。一个新的 statrs
   包装通常就是多一个 `collect_aggregate!` 调用，换掉末尾的算式。
3. **挂进模块树**：在 `src/extension/functions/mod.rs` 加 `mod my_function;`。crate root 不用动。
4. **把文档元数据写进属性** —— `description`、`comment`、`example` / `examples`：

   ```rust
   #[duck_scalar_function(
       description = "One line for the function table of the community-extension page",
       comment = "The detail that does not fit the one-liner",
       example = "SELECT sr_normal_pdf(0.0, 0.0, 1.0)"
   )]
   ```

   DuckDB 的 C 扩展 API 没有设置函数描述与示例的接口，所以这段文字是社区扩展文档页
   `Added Functions` 表格的唯一来源。`just docs_csv` 把它导出到 `target/function_descriptions.csv`
   （见[社区扩展](../community-extension.md)）。文案用**英文** —— 它会被原样贴进那个页面。
5. **配一份测试**（见[测试](./testing.md)），跑一遍 `just lint`。期望值取自 statrs 的实际输出，
   不要手算。

### 命名

凡是出现在 SQL 里的名字都带同一个短前缀 `sr_`，前缀之后的部分要能读出函数在干什么。带前缀的名字
也是用户要敲的，所以别写成 `duckfn_statrs_sr_mean`。

属性宏默认拿 **Rust 函数名**当注册名，所以函数就叫 `sr_mean`、`sr_normal_pdf`。同一个名字需要多个
签名（参数类型或个数不同）时，用 `overloads_name = "…"` 并成一个函数集，而不是各注册各的。

宏还会为每个签名生成 `SQL_NAME` 常量。一个名字出现在多处（错误前缀、日志、提示文案）时改读这个
常量，别再抄一份字面量；代价是这种函数得写成 `pub(super)`，因为生成的模块沿用函数的可见性。

### 配置参数

配置类参数（`DuckLazy<T>`）解析一次、函数内读取，逐行解析不会出现在性能剖面里。本扩展没有用到；
这个模式（具名 STRUCT 类型、加载期创建）在 duckfn 指南里有文档，
[duckfn_quantstats](https://github.com/shijianjs/duckfn-quantstats) 通篇在用。
