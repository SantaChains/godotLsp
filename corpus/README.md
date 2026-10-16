# GDScript 语料库（corpus）

本目录收录用于**从零自研 GDScript 分析内核**的真实 `.gd` 样本与测试夹具。

## 来源与许可证

- `godot-engine-tests/`：来自官方 **Godot 引擎**仓库
  `github.com/godotengine/godot` 的 `modules/gdscript/tests/scripts`
  （抓取于 `master`，commit 约 `9a05d575f9f5b9dc8f0d5681028089900a73aa80`）。
  **许可证：MIT**（与本项目一致，可自由收录、修改、再分发）。
- 这批文件是 Godot 引擎**自带的 GDScript 测试夹具**，是业界最权威、覆盖最全的
  GDScript 2.0 真实样本，包含：正常语法、静态类型、类型推断、警告、以及大量
  **故意错误（负向）夹具**——后者对检验解析器的错误恢复能力极有价值。

## 目录结构（godot-engine-tests/）

| 子目录 | 文件数 | 内容 | 对本项目的用途 |
|---|---:|---|---|
| `analyzer/` | 235 | 语义/类型推断/名称解析测试 | 类型系统、成员解析、诊断的 golden 对照 |
| `completion/` | 146 | 补全测试夹具（含期望输出） | 补全引擎的 oracle / 回归基准 |
| `parser/` | 183 | 解析测试（含语法错误夹具） | 解析器健壮性、错误恢复、panic 自由 |
| `runtime/` | 126 | 运行时行为测试 | 语义正确性对照（部分需引擎，仅作参考） |
| `lsp/` | 12 | LSP 层测试夹具 | hover/诊断/符号的集成参照 |

## 其他子目录

- `feature-showcase/`：本项目**手写**的、按特性组织的 GDScript 2.0 覆盖样本，
  用于快速冒烟与文档示例（非 Godot 上游）。
- `godot-demo-projects/`：来自官方 demo 仓库
  `github.com/godotengine/godot-demo-projects`（**只解出 .gd 文件**）。
  **许可证：MIT**。拉取用 `uv run tools/fetch_demo.py`（pinned tag
  `4.7-6ad6167`，2026-10 收录，456 个 .gd）。
  消费方：`tools/gen_freq.py` 的补全频率表挖掘（权重 ×2,真实工程）。
- `bench/`：**大型真实工程**（完整 tarball,含 project.godot/.tscn/.tres），
  专供 LSP 能力与负载基准（`crates/godotlsp/tests/bench_load.py`）。
  **许可证：均为 MIT**。不入库，拉取用 `uv run tools/fetch_bench.py`
  （pinned commit,2026-10-16 收录）：

  | 工程 | 仓库 | pinned commit | 规模（收录时实测） |
  |---|---|---|---|
  | `pixelorama` | Orama-Interactive/Pixelorama | `e64b476` | 249 .gd / 59,717 行 / 120 .tscn |
  | `dialogic` | dialogic-godot/dialogic | `701b67b` | 270 .gd / 35,202 行 / 84 .tscn |
  | `gdunit4` | MikeSchulze/gdUnit4 | `306eb30` | 235 .gd / 27,181 行 / 16 .tscn |

  合计 754 个 .gd / ~122,100 行,是官方 demo 语料（456 个小文件）之外的
  「大文件 + 深目录 + 真实 UI 工程」负载维度。

## 使用约定

1. **只读参考 / 测试输入**：这些文件是参照系与测试数据，不应被解析器输出改写。
2. **负向夹具**：含 `parser/` 下的错误样本，解析器必须**不产生 panic**、给出
   有界错误，而非崩溃——这是 corpus 驱动测试的首要验收项。
3. **不随构建发布**：语料仅用于开发期测试与 benchmark，不进 `server/` 也不进 vsix。
4. **更新上游**：如需刷新，重新稀疏克隆 `modules/gdscript/tests/scripts` 并覆盖本目录，
   同步更新上面的 commit 哈希与计数。
5. **分发策略**：`feature-showcase/`（手写）随仓库入库；`godot-engine-tests/`
   （上游未修改）不入库——`cargo test` 缺它时自动跳过，拉取用
   `uv run tools/fetch_corpus.py`（按 pinned commit 稀疏克隆）。

## 统计

- 总 `.gd` 数：703（截至收录时）
- 总大小：约 2.3 MB
- 单文件最大：约 11 KB；绝大多数为小到中型片段（契合"单文件快速解析"基准）。
