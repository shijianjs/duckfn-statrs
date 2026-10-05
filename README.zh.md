[English](README.md) | [简体中文](README.zh.md)

# duckfn_statrs

DuckDB 的 [loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)，
把 Rust 统计计算库 [statrs](https://crates.io/crates/statrs) 包装成可以直接在 SQL 里调用的函数：
描述统计量做成**聚合函数**（`SELECT sr_mean(x) FROM t GROUP BY g` 这样的写法），正态分布的
pdf / cdf / 分位数做成**标量函数**（逐行求值）。扩展用 [duckfn](https://crates.io/crates/duckfn)
的属性宏写成，全程不碰 C++ 构建。

计算本身全部交给 statrs —— 本扩展不重新实现任何统计公式，只负责把 SQL 的值送进去、把结果按
SQL 的语义送出来。

## 快速上手

```shell
make configure   # 只做一次：建 configure/venv（Python 与 sqllogictest 运行器）
make debug       # -> build/debug/duckfn_statrs.duckdb_extension
```

自己构建的产物没有签名，加载时必须给 DuckDB 加 `-unsigned`：

```shell
duckdb -unsigned -c "
LOAD './build/debug/duckfn_statrs.duckdb_extension';
SELECT sr_mean(x) FROM (VALUES (1.0), (2.0), (3.0)) t(x);
-- 2.0
SELECT sr_normal_cdf(1.96, 0.0, 1.0);
-- 0.9750021048529024
"
```

`Justfile` 把同样的命令包了一层：`just build`、`just sql "SELECT sr_mean(x) FROM range(10) t(x)"`、
`just repl`（已 LOAD 扩展的 REPL）。

## 函数

注册名统一带 `sr_` 前缀，在 `duckdb_functions()` 里可以按前缀整组检索；代码目录与 statrs 的
模块树同构（`functions/{consts,function,statistics,distribution}/`），对应关系见
`src/extension/functions/mod.rs` 头注释。当前 34 个聚合 + 503 个标量 = 537 个函数，按族概括：

**聚合函数**（一列 DOUBLE 进、一个 DOUBLE 出，`auto_collect` 形态）：

| 族 | 函数 |
| --- | --- |
| 集中趋势 | `sr_mean` / `sr_geometric_mean` / `sr_harmonic_mean` / `sr_quadratic_mean` |
| 顺序统计量 | `sr_median` / `sr_quantile(x, tau)` / `sr_order_statistic(x, k)` / `sr_percentile(x, p)` / `sr_lower_quartile` / `sr_upper_quartile` / `sr_interquartile_range` / `sr_ranks(x, method)`（出 LIST） |
| 离散程度 | `sr_variance` / `sr_std_dev` / `sr_population_variance` / `sr_population_std_dev` / `sr_skewness` |
| 极值与配对 | `sr_min` / `sr_max` / `sr_abs_min` / `sr_abs_max`；`sr_covariance(x, y)` / `sr_population_covariance(x, y)` |

**标量函数**（逐行求值；参数用 DOUBLE，向量/计数走 LIST，BIGINT 用于随机抽样点数与波形时长）：

| 族 | 函数 |
| --- | --- |
| 连续分布（20 种） | 每分布 `sr_<分布>_pdf / ln_pdf / cdf / sf / quantile`（normal、log_normal、beta、gamma、chi_squared、students_t、uniform、weibull、pareto…） |
| 离散分布（8 种） | 每分布 `sr_<分布>_pmf / ln_pmf / cdf / sf / quantile`（binomial、poisson、geometric、hypergeometric、categorical…） |
| 多元分布（4 种） | 每种分布的密度（`pdf` / `ln_pdf`）、各阶矩（`mean` / `variance` / `mode`，矩阵行主序摊平）与 `entropy` / `min` / `max` —— `sr_multivariate_normal_*`、`sr_multivariate_students_t_*`、`sr_dirichlet_*`、`sr_multinomial_*`（向量走 LIST，计数向量走 LIST(BIGINT)） |
| 经验分布（聚合） | `sr_empirical_cdf / sf / quantile(x, k)`；抽样 `sr_sample_empirical` |
| 随机采样（34 个） | `sr_sample_<分布>(<参数...>, k BIGINT)` 抽 k 个点出 LIST；二项另有 `sr_sample_binomial_algorithm`（statrs 的 BinomialAlgorithm） |
| 密度估计 | `sr_kde_pdf(x, 样本 LIST, 带宽可 NULL)` / `sr_knn_pdf`（kde feature，k-d tree） |
| 假设检验（9 个） | `sr_ttest_onesample` / `sr_skewtest` / `sr_anderson_darling` / `sr_ks_onesample` / `sr_ks_twosample` / `sr_mannwhitneyu` / `sr_chisquare` / `sr_f_oneway` / `sr_fishers_exact(_with_odds_ratio)`，出 [统计量, p 值] |
| 信号生成器 | `sr_gen_square / triangle / sawtooth / periodic / sinusoidal / log_spaced`（取前 k 点出 LIST） |
| 特殊函数 | `sr_erf` / `sr_erfc` / `sr_gamma` / `sr_ln_gamma` / `sr_digamma` / `sr_beta` / `sr_beta_regularized` / 不完全 Gamma 四变体 / `sr_factorial` / `sr_choose` / `sr_harmonic` / `sr_logistic` / `sr_logit` / `sr_polynomial` / `sr_kernel_eval` / `sr_kernel_support`… |
| 常量 | `sr_sqrt_2pi()` / `sr_euler_mascheroni()` 等（statrs::consts） |

statrs 的全部能力都有 SQL 出口（首要规则，缺口不允许存在）；仅剩的非能力导出
（`prec` 内部精度策略、`euclid::Modulus` trait）在 `src/extension/functions/mod.rs` 头注释里说明。

## NULL 与错误语义

一律对齐 statrs，向 SQL 用户侧传播：

- **NULL 输入**：聚合里 NULL 行不进状态（SQL 聚合惯例，与 DuckDB 自带的 `mean`/`stddev` 一致）；
  标量里任一参数为 NULL 的行短路成 `NULL`。协方差的任何一列为 NULL 的行整行跳过，两列保持配对。
- **statrs 算不出的**（空组、单值的样本方差、越界的 tau、几何/调和平均遇到负数）返回 NAN，
  扩展统一折成 `NULL` —— NAN 永远不会作为值出现在结果里。
- **参数存在但非法**（`std_dev <= 0`、概率不在 `[0, 1]` 内）是调用写错了，报查询错误，
  不会静默折成空。

## 从源码构建

构建走官方 DuckDB `extension-ci-tools` makefile：

```shell
make configure           # 只做一次：建 configure/venv（Python 与 sqllogictest 运行器）
make debug               # 官方路径 -> build/debug/duckfn_statrs.duckdb_extension
```

`make release` 是带优化的同一套流程。Windows 上 `make` 需要在 Git Bash 里跑。
`Justfile` 把它包了一层（`just build` = `make configure && make debug`、`just ci-build`、`just test`、
`just ci-release`）。

## 测试

测试用 SQLLogicTest 格式写在 `test/sql/` 下，按函数组分文件：
`aggregate_summary.test`（统计量聚合）、`aggregate_covariance.test`（协方差）、
`scalar_normal.test`（正态分布）、`duckfn_statrs.test`（冒烟 + 注册清单）。
期望值全部取自 statrs 的实际输出，不是手算的近似。

```shell
just test          # make configure + make debug + make test
just ci-build      # 只做官方构建，不跑测试
```

`make test` 不会自动重新构建，改完 Rust 先跑 `just ci-build`（或 `make debug`）。
更快的迭代方式见 [DEVELOPMENT.zh.md](DEVELOPMENT.zh.md)。

## WebAssembly

```shell
just config_env   # 只做一次：固定工具链版本并装上 wasm target
just build_wasm
```

wasm 构建走 `src/wasm_lib.rs`（`src/lib.rs` 的 `staticlib` 镜像），两个 crate root 必须始终声明同一组
`mod`。statrs 是纯 Rust 实现，不碍这条路。

## 文档站

仓库里带着一个 [Docusaurus](https://docusaurus.io/) 站点（`docs/`），中英双语，并且有工作流在每次版本
tag 时把它发布到 GitHub Pages：

```shell
just docs_install    # 只做一次
just docs_start      # 本地预览 http://localhost:3000
just docs_build      # 真正该跑的那一条：onBrokenLinks 设为 throw，链接断了就构建失败
```

页面里带着可运行的 SQL 块（由 [`duckfn-docs-kit`](https://www.npmjs.com/package/duckfn-docs-kit) 提供），
在浏览器里直接调用本扩展；`cd docs && npm test` 会把它们重跑一遍。约定（目录、命令、翻译流程、部署、
`{{EXTENSION_VERSION}}` 版本占位符）写在 [`docs/README.md`](docs/README.md) 里。

## 安装已发布的扩展

发布物是 GitHub Release 上各平台构建出的 `.duckdb_extension` 文件：

```shell
duckdb -unsigned -c "
LOAD 'https://github.com/shijianjs/duckfn-statrs/releases/latest/download/duckfn_statrs-windows_amd64.duckdb_extension';
"
```

注册进 DuckDB 的[社区扩展](https://duckdb.org/community_extensions/list_of_extensions)之后，就变成
一句 `INSTALL duckfn_statrs FROM community`；那需要提交的两份文件已经备在
[`community-extension/`](community-extension/AGENTS.md)。

## 文档

| 文件 | 里面有什么 |
| --- | --- |
| [AGENTS.md](AGENTS.md) | 约定、duckfn 知识地图、发版流程 |
| [DEVELOPMENT.zh.md](DEVELOPMENT.zh.md) | 目录结构、骨架取舍、构建与测试、函数描述导出 |
| [docs/README.md](docs/README.md) | 文档站：目录、命令、翻译流程、部署 |
| [DEVELOPMENT.md](DEVELOPMENT.md) | 同上，英文 |
| [README.md](README.md) | 本文件，英文 |

本仓库来自 [duckfn-extension-template](https://github.com/shijianjs/duckfn-extension-template)，
仓库约定（扩展名五处一致、`just rename`、共享 justfile、发版流程）见 [AGENTS.md](AGENTS.md)。
