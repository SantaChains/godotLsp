# 从零自研 GDScript 分析内核 · 设计文档

> 本文档是 godotlsp **脱离 `gdscript-ide` 外部依赖、从零自研分析内核** 的总设计。
> 配套：`docs/lsp-design.md`（通用 LSP 架构参照）、`REFERENCE.md`（上游参考边界）、`corpus/`（语料）。

---

## 0. 决策记录（decision log）

- **D1 — 放弃 `gdscript-ide` 依赖，从零自研分析内核。**
  纯 Rust、safe（`unsafe_code = "deny"`）；不通过 `[patch.crates-io]` 或 path 引入上游编译单元。
- **D2 — 保留 Godot 引擎 API 模型「数据」，但运行期零引擎。**
  引擎模型 = 静态数据集，**不是**引擎进程 / FFI / 编辑器。
  「脱离引擎」指运行期脱离，不是与 Godot 类体系无关——分析 GDScript 必然要携带类模型。
  **v2 修订(2026-10-16)**:模型源头改为 **Godot 源码导出的 `extension_api.json`**(自研摄取管线，
  见 §7 解析梯度)——默认跟随用户引擎版本自动 dump,内嵌出厂面降为无 Godot 环境的最后兜底；
  彻底移除对任何第三方预构建 blob(gdscript-api 的 engine_api.bin 等)的依赖。
- **D3 — 复用现有 LSP 服务层，不引入 salsa。**
  `lsp-server-tokio 0.3` 异步传输（P1 已完成）+ 单写者 World / 快照 / 信号量 / 取消三态（P2 已完成）全部保留。
  上游 `gdscript-analyzer` 用 salsa 0.27 增量，我们**明确不采用**——salsa 写路径有 `cancel_others` 死锁硬约束（见 `lsp-design.md` §六源码核验），自研快照模型已规避。
- **D4 — 引擎模型摄取管线自研。**
  **v2 修订(2026-10-16)**:不再「借鉴 gdscript-api 设计」,而是从零自研:主源 = `godot --dump-extension-api-with-docs`
  产物(走 ClassDB,完备-by-construction,核验见 §7.1),归一化/codegen 全自研;
  gdls(kurushimee/gdls)的自动 dump 工程学(§7.2/7.3)经其文档实证,作为对照参考。
- **D5 — 词法/语法/语义全自研；CST 采用「无损树 + trivia 一等 token」思路**（借鉴 cstree 思路，可自研轻量实现，不必直接依赖 cstree）。

---

## 1. 目标与范围

| 项 | 目标 |
|---|---|
| 语言 | GDScript 2.0（Godot 4.x）语法全集（见 §2 矩阵） |
| 运行期 | 完全脱离 Godot 引擎：无进程、无 FFI、无编辑器；仅内嵌静态类模型数据 |
| 协议 | LSP 3.17 全特性（渐进交付，见 §7 / §10） |
| 非目标 | 不运行 GDScript（无 VM）、不渲染、不做 Godot 编辑器专属插件 |

---

## 2. GDScript 2.0 语法覆盖矩阵

`★` = 首版必须支持；`○` = 后续阶段补全。

| 特性 | 形态 | 优先级 |
|---|---|---|
| 隐式类 / `class_name` / `extends`（父路径） | `extends Node2D` / `class_name Foo` | ★ |
| 内部类 `class X` | 命名空间 + 自身作用域 | ★ |
| 变量 `var` / 常量 `const` | 含类级 | ★ |
| 类型注解 `: T` 与推断 `:=` | 局部 + 成员 | ★ |
| 注解 `@export*` / `@onready` / `@export_group` / `@export_range` 等 | 成员修饰 | ★ |
| 函数 `func` | 参数类型/默认/`->` 返回/`static`/`varargs` | ★ |
| 内联 lambda / `Callable` | `func():` / `.bind()` / 作为参数 | ★ |
| 信号 `signal` / `emit` / `connect` / `await sig` | 协程挂起 | ★ |
| 枚举 `enum` / 具名枚举 / 枚举值注解 | `enum E { A, B }` | ★ |
| 控制流 `if/elif/else` `for` `while` `match` | match：字面/多候选/守卫/通配/绑定 | ★ |
| 运算符优先级 / `is` / `as` | 表达式 | ★ |
| `$` 节点路径 / `%` 唯一名 / `self` / `super` | 表达式 | ★ |
| `preload` / `load` / `class_name` 引用 | 跨文件符号 | ★（R3 完善） |
| 类型系统 | Variant 顶类型、具体类型、null、推断、类型收窄 | ★（R2） |
| 泛型容器 | `Array[T]` / `Dictionary[K,V]` / `Packed*Array` | ★（R2） |
| 文档注释 `""" """` + `@` 注解 | hover / 补全文档源 | ★（R2） |
| `@rpc` / `@tool` / `@warning_ignore` | 注解语义 | ○ |
| 协程 `await` / `yield` | 控制流语义 | ○（R3） |

---

## 3. 架构总览（crate 布局）

```
crates/
  gdscript-syntax/   纯前端：lexer + parser + CST/AST（无 IO，可 wasm）
  gdscript-engine/   引擎模型摄取：extension_api.json -> 内存模型 + codegen 二进制
  godotlsp/          (现有) LSP 服务层：tokio 传输 + 并发 + 特性路由
                     依赖上面两个 crate，不再依赖 gdscript-ide
  (可选) gdscript-test/  corpus 驱动的解析/语义测试 harness
```

数据流：

```
  .gd 文本
     │
     ▼
  [lexer]  ── trivia 一等 token 流 ──►  [parser]  ──►  无损 CST (+ 派生 AST)
                                                  │
                                                  ▼
                                          [semantic]  符号表 + 类型推断 + 名称解析
                                                  │   （跨文件需 project 索引）
                                                  ▼
                                   [gdscript-engine 模型]  Godot 类/方法/信号/枚举/常量
                                                  │
                                                  ▼
                                     [godotlsp LSP 层]  hover/补全/诊断/导航/...（复用现有）
```

关键：**现有 `godotlsp` 的传输与并发（P1/P2）完全不动**，只把"分析内核插槽"从 `gdscript-ide::Analysis` 换成我们的 `Project` / `Semantic` 模型。

---

## 4. 词法（lexer）

- **token 集**：关键字、标识符、数字（整数/浮点/十六进制/二进制/科学计数）、字符串（含多行 `"""`）、运算符、分隔符、缩进标记。
- **trivia 一等 token**（借鉴 cstree/logos）：空白与注释**不 skip**，作为 CST 节点留存——这是格式化、诊断定位、增量重解析、语义 token 的基础。
- **缩进敏感**：GDScript 靠缩进分块，tokenizer 须产出 `INDENT` / `DEDENT`（或在 parser 内处理），并与括号深度解耦。
- **错误词法**：非法字符 / 未闭合字符串 → 产生 `ERROR` token，**绝不 panic**；错误有界。
- **实现选型**：手写 DFA 或 `logos 0.16`（上游用 logos）。两者均 safe Rust；首版倾向手写以完全掌控错误恢复与 trivia 处理，后续可按需换 logos。

---

## 5. 语法（parser）

- **递归下降 + Pratt（运算符优先级）**：GDScript 表达式运算符优先级明确，Pratt 最贴合。
- **产出无损 CST**（+ 派生 AST 供语义用）：CST 保留全部 trivia 与错误节点，支持增量与精确诊断。
- **错误恢复（panic 自由）**：用同步 token 集（`func`/`class`/`var`/`enum`/`signal`、行尾、缩进边界、`}`/`)`）重置解析状态，保证每个文件错误数有界、不崩溃。
- **增量解析**：结合现有快照模型（P2 的 50ms 事件合并），对局部编辑做字符/行级重解析而非全量。
- **验收**：`corpus/feature-showcase` 全过；`corpus/godot-engine-tests` 解析通过率 ≥95%（其余为已知负向错误夹具，须"零 panic + 有界错误"）。

---

## 6. 语义分析（semantic）

- **符号表**：作用域树（文件级类 → 类成员 → 局部块），区分 `var`/`const`/`func`/`signal`/`enum`/`class`。
- **类型系统**：GDScript 类型格——`Variant` 为顶类型，具体类型、`null`、类型推断、`is`/`as` 收窄。局部 `:=` 推断、函数返回推断、成员类型查询。
- **名称解析**：
  - 成员：自身类 + `extends` 链 + 内建 + 引擎模型（`gdscript-engine`）；
  - 局部 / 全局（`autoload`、`preload`、`class_name` 注册）；
  - 信号连接目标、枚举值、常量。
- **跨文件项目分析（R3）**：扫描工作区全部 `.gd`，建立项目级符号图（`class_name` 注册、`preload` 路径解析、`project.godot` 中 `autoload` 单例）。
- **诊断**：解析错误、类型不匹配、未声明标识符、未使用（warning）、循环继承等。
- **借鉴边界**：参考 `gdscript-ide` 的*成员解析与类型推断思路*，但**自研实现**，不引 salsa——用快照 + 显式依赖图，沿用 P2 已验证的并发模型。

### 6.1 `project.godot` 与 autoload 语义（已核验，R2/R3 实施依据）

`project.godot` 是 Godot `ConfigFile`（INI 式：`[section]` 头、`key=value`、`;`/`#` 注释、值带引号）。分析器只需三个 section（行向最小扫描即可，不必实现完整 Variant 解析）：

| Section | 用途 |
|---|---|
| `[autoload]` | 单例注册表（全局标识符来源） |
| `[application] config/features` | 引擎版本 `(major, minor)`（数组里第一个版本形态项） |
| `[debug] gdscript/warnings/*` | 警告开关与分级（Ignore/Warn/Error） |

**autoload 条目格式**（本机 Godot 4.7 项目实测 + 上游 `project_settings.cpp` 核验）：

```ini
[autoload]
GameConfig="*res://scripts/game_config.gd"   # `*` = 暴露为全局标识符
Music="*res://music.tscn"                    # 场景单例 → 根节点附着脚本
Helper="res://helper.gd"                     # 无 `*`（4.3+ 取消勾选"全局变量"）
```

- **`*` 前缀 = 全局标识符**：4.3+ 编辑器"全局变量"复选框的存储形式。带 `*` 的名字在所有脚本中直接可见（`GameConfig.high_score`）；不带 `*` 仅在 `/root/Name` 作为节点存在，**不是**标识符。
- **路径类型**：`.gd` → 该脚本的类（无 `class_name` 也可，按 res:// 路径解析）；`.tscn`/`.tres` → 场景根节点的附着脚本；其余 → 保守 seam（`Unknown`，不误报）。
- **解析优先级**（Godot `reduce_identifier`）：局部 → 类成员 → 继承链 → 引擎全局 → **`class_name` 注册 ≻ autoload 单例** → 未声明。
- **注册表命中但不解析 ≠ 未声明**：场景单例等落到 seam 的情况仍算"已定义"，禁止报未声明（保守 seam 的核心理由）。
- **热更新是硬约束**：`project.godot` 是活输入——变更后 autoload 注册表 / 引擎版本 / 警告设置全部失效，**必须重算并重发全部已打开文档的诊断**。旧服务曾因"重载了配置但不重发诊断"，导致用户在 Godot 里加好 autoload 后 VS Code 持续误报 `Identifier "X" not declared in the current scope`（2026-10-16 修复：watcher 事件 + didOpen mtime 自愈双路径，回归测试 `tests/autoload_reload.rs`）。
- **借鉴落点**：上游 `gdscript-hir` 的 `project.rs::parse_autoloads` / `resolve.rs::resolve_autoload` 即此语义，可对照重写（不得复制）。

---

## 7. 引擎模型摄取(gdscript-engine)v2 —— 源头是 Godot 源码,内嵌只是兜底

> **v2 修订(2026-10-16,用户决策)**:v1 的「构建期内嵌一份固定模型二进制」降级为**最后一级兜底**。
> 用户要求:完全脱离对任何第三方预构建 blob 的依赖,结合开源项目与 Godot 源码**自研摄取管线**;
> 模型默认**跟随用户实际引擎版本**,而不是我们固化的一版。运行期零引擎原则(D2)不变。

### 7.1 权威数据源(已核验,gdls 同款结论)

- **主源:`godot --headless --dump-extension-api-with-docs` 导出的 `extension_api.json`**。
  该 dump 迭代 `ClassDB`,凡引擎经 `ClassDB::bind_method`/`GDREGISTER_CLASS` 注册的类**完备-by-construction**,
  免解析 C++ 源码;含 name/inherits(全继承链)/methods(参数名、类型、默认值、is_const/is_static/is_virtual/is_vararg)、
  properties、signals、enums、constants、builtin classes、global enums、utility functions,
  以及 `description`/`brief_description` 文档散文——正是 hover/补全/推断所需的全部信息。
- **`doc/classes/*.xml` 不作主源**(doc-oriented,类没有专属 XML 就不出现,不完备);仅用于第三方 GDExtension 类的文档补充。
- **中文文档语料**:godot-docs 中文(官方 CC-BY 3.0;本地 `godot-docs/` 已收孤辰辰译本),
  构建期抽取「符号 → 中文摘要 + 在线页面锚点」索引(见路线图 P2-17);签名正确性仍只认 `extension_api.json`。

### 7.2 摄取梯度(解析阶梯,自研实现)

运行期按序解析,取第一个可用级;逐级降级永不崩溃:

1. **用户显式路径**(配置 `engine.apiPath` / initializationOptions):永远最优先,从不触发自动 dump;
2. **工作区已有 dump**:项目根或 `.godotlsp/api.json` 已存在合法 dump → 直接采用(注意:**绝不覆盖用户手放的 `extension_api.json`**);
3. **自动 dump**(默认,仅当检测到 `project.godot` 真工程 + 找到 Godot 二进制):会话启动后台线程执行
   `godot --headless --dump-extension-api-with-docs`,超时上限 5 分钟;**以产物为准、以 exit code 为准都不看**
   ——Godot 可能在完整写出 dump 后于退出阶段 abort(gdls 在 4.6.3 实证),文件完整即采纳;
   dump 落在项目根(与子进程 cwd 无关,gdls 实证),写临时名再原子改名入 `.godotlsp/`;
4. **内嵌出厂面**(唯一保留「内嵌」语义的一级):构建期从 **Godot 官方源码仓库 pinned commit** 导出的
   `extension_api.json` 摄取(管线同 §7.4,不再使用任何第三方 crate 的预构建 blob),仅作无 Godot 环境(CI/无头/远程)兜底;
5. **空模型 + 动态降级**:什么都没有时类型解析退化为动态,发一条通知,功能不崩。

### 7.3 版本对齐与新鲜度

- dump header 携带版本号 + 内容哈希;`project.godot` 的 `config/features` 声明版本**高于** dump 版本时,
  该 dump 视为陈旧(dump 缺新版本 API 会把合法调用误报为错误),自动换用内嵌对应版本出厂面(gdls 同款规则);
- 监视 Godot 二进制 mtime/size 与 `.gdextension` 文件集变化 → 会话内重 dump(后台,会话中热替换);
- 第三方 GDExtension 类:裸二进制 dump 不含项目安装的 GDExtension(gdls 实证);检测到
  「声明了扩展但其类提示全部解析失败」→ 提示用户在 Godot 编辑器打开一次工程后重 dump。

### 7.4 归一化与 codegen(自研,不依赖上游 rkyv)

- 归一化:去重、`inherits` 继承链扁平化、参数/返回类型映射为内部类型表示;
- 构建(`xtask`)与运行期双形态:**内嵌面** = 构建期序列化零拷贝二进制 `include_bytes!`;
  **外部 dump** = 运行期惰性解析 `extension_api.json`(serde),解析一次后驻留——两形态共用同一套内存模型 `from_*` 构造器;
- 分版本存放(`4.3/4.4/4.5/...`),构建期可选多版本打包,运行期按工程声明切换;
- 中文文档索引独立于模型(按符号键关联),模型换版不重建语料索引。

---

## 8. 与现有 LSP 服务层对接

| 现有组件 | 处置 |
|---|---|
| `lsp-server-tokio 0.3` 异步传输（P1） | **保留** |
| 单写者 World + 快照 + 信号量 + 取消三态 + 诊断节流（P2） | **保留** |
| 特性路由（hover/completion/diagnostics/...） | **保留**，仅把查询后端从 `gdscript-ide::Analysis` 换成我们的 `Project`/`Semantic` |
| `gdscript-ide` / `gdscript-base` / `gdscript-api` 依赖 | **移除**，改依赖 `gdscript-syntax` + `gdscript-engine` |

`server/godotlsp.exe` 的构建与 `vsix` 打包流程不变（重拷二进制 + 打 vsix），但二进制里不再链接任何 `gdscript-ide` 符号。

---

## 9. corpus 驱动的测试策略

- **`corpus/godot-engine-tests/`**（703 文件，Godot 上游 MIT）：解析器必须对**全部**文件**零 panic**、错误有界；按 `analyzer/completion/parser/runtime/lsp` 分类统计解析通过率与诊断正确性。
- **`corpus/feature-showcase/`**（手写 8 文件）：快速冒烟 + 文档示例，覆盖 §2 的 ★ 特性。
- **golden 测试**：对 `completion/` `analyzer/` 夹具用 expect-test（`UPDATE_EXPECT=1` 祝福）比对期望输出。
- **解析 fuzz**：`proptest` / 随机编辑重解析，断言 panic 自由与错误有界。
- **性能基准**：单文件冷解析 <50ms、热解析 <5ms（借用 `criterion`）。

---

## 10. 分阶段路线图（含验收）

| 阶段 | 内容 | 验收标准 |
|---|---|---|
| **R0 脚手架** | 新建 `gdscript-syntax` / `gdscript-engine` crate；从 `godotlsp/Cargo.toml` 移除 gdscript-ide 依赖；接入 corpus 测试 harness | `cargo build` 无 gdscript-ide 依赖；corpus 解析零 panic |
| **R1 前端** | lexer + parser（CST + 错误恢复 + 缩进） | feature-showcase 全过；godot-engine-tests 解析通过率 ≥95% |
| **R2 语义** | 符号表 + 类型推断 + 引擎模型摄取 | hover / 补全基本可用；类型错误能报 |
| **R3 项目** | 跨文件分析 + 诊断完善（preload/autoload/class_name/信号） | 多文件工程诊断正确 |
| **R4 特性+健壮** | 全 LSP 特性 + P3 大输入/UTF-8 边界 + 增量 | vsix 真机测试通过 |
| **R5 收尾** | 文档全汉化 + criterion/fuzz 对标 gdls | 测试全绿、benchmark 达标 |

---

## 11. 风险与权衡

- **工作量**：从零前端是数周级工程；用 corpus 驱动保证每阶段增量可验证、可回滚。
- **Godot 版本漂移**：引擎模型须随 Godot 版本更新（构建期摄取，见 §7）。
- **与上游分叉**：我们选**自研并发（非 salsa）** + **自研异步传输（非 lsp-server 同步）**，换取对死锁/取消的完全掌控（见 `lsp-design.md` §六）。这是经过源码核验的取舍，不是倒退。
- **正确性 oracle**：以 Godot 上游测试夹具为主；可选引入 `tree-sitter-gdscript` 作 dev-only 差分对照（借鉴上游 oracle 思路，但默认关闭，不进发布构建）。

---

## 12. 借鉴清单（来自 `gdscript-analyzer`，详见 `REFERENCE.md`）

✅ 借鉴：引擎模型摄取管线（`extension_api.json` → 二进制 → 零拷贝）；CST 无损树 + trivia 一等 token；特性覆盖矩阵与类型推断思路；测试夹具组织（`completion/` `analyzer/` `lsp/` 分目录 + golden）；Rust 2024 edition / safe 实践。

❌ 不借鉴：salsa 0.27 增量数据库；`lsp-server` 同步传输 + unwind 取消；napi / wasm 绑定层。
