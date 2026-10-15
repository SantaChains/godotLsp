# 参考源码（非构建依赖）

`gdscript-analyzer/` 是一个 git submodule，**仅用于从零重写时借鉴实现思路**，**不是本项目的构建依赖**。

- 上游仓库：`github.com/yanivkalfa/gdscript-analyzer`（crates.io 的 `gdscript-ide/base/api` 发布版即来源于此；`github.com/reactive-ui-toolkit/gdscript-analyzer` 是 GitHub 重定向别名，submodule 用它拉的）。
- 钉在 tag `v0.6.1`（`f5f70e1`），与 crates.io 发布版 `0.6.1` 一致。
- 许可证：MIT OR Apache-2.0（与本项目兼容）。

## 为什么保留它但不依赖它

用户决策：godotlsp 的分析内核要从零自研（纯 Rust、运行期脱离 Godot 引擎），**不再依赖 `gdscript-ide` 等外部 crate**。
保留 submodule 是为了能随时对照上游的成熟设计，但任何借鉴都须**重写**进本仓库的 crate，禁止以 path/patch 形式把上游编译进构建。

## 上游 crate 布局（借鉴地图）

| Crate | 职责 | 本项目是否借鉴 / 怎么借鉴 |
|---|---|---|
| `gdscript-base` | 底层原语（TextSize、interned 字符串等） | 自研等价物，不照搬 |
| `gdscript-syntax` | **logos DFA 词法 + cstree 无损 CST + 递归下降解析** | 借鉴 CST/词法/解析的*结构*，自研实现 |
| `gdscript-api` | 把 Godot `extension_api.json` 归一化为 `EngineApi`，codegen 期序列化为 rkyv 二进制，运行期零拷贝加载 | **重点借鉴**：引擎模型摄取管线（数据，非引擎进程） |
| `gdscript-db` | salsa 0.27 增量数据库（单写多读） | **不采用**：本项目已有自研单写者 World + 快照 + 信号量并发模型，规避 salsa 写路径死锁（见 `docs/lsp-design.md` §六） |
| `gdscript-hir` | 高层 IR | 自研语义模型 |
| `gdscript-ide` | 语义分析 + LSP 特性（hover/补全/导航/语义 token） | 借鉴*特性覆盖*与*类型推断思路*，自研实现 |
| `gdscript-lsp` | LSP 服务（lsp-server 0.8 同步 + unwind 取消） | **不采用**：本项目已有 lsp-server-tokio 0.3 异步传输层（P1 已完成） |
| `gdscript-fmt` / `gdscript-scene` / `gdscript-cli` / `gdscript-ffi` / `gdscript-wasm` | 格式化 / 场景 / CLI / napi / wasm | 暂不在范围内 |

## 借鉴边界红线

1. 不得把 `gdscript-analyzer/` 中的 `.rs` 直接复制进 `crates/`。
2. 不得通过 `[patch.crates-io]` 或 path 依赖把上游编入构建（根 `Cargo.toml` 已移除 patch）。
3. 引擎模型**数据**（`extension_api.json` + 类文档）可摄取、可内嵌——这是静态数据集，不是"依赖引擎"；运行期仍零 Godot 进程、零 FFI。
4. 上游的并发/增量方案（salsa）明确不采用，沿用本项目已验证的快照模型。
