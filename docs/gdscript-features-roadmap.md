# GDScript 开发体验路线图(数据层 → 提示 → 交互)

以"数据准确 → 提示更强 → 交互更全"为序,结合官方插件能力面、社区高频诉求与生态工具,规划 godotlsp 的功能演进。服务器侧与编辑器侧分层标注:服务器能力所有 IDE 共享,编辑器侧仅进 VS Code 插件。

## 零、已完成:数据与准确性基线(本次)

**实测缺口 → 修复**(探针数据:按名补全 41 项,引擎全局面 0 命中):

| 缺口 | 修复 |
|---|---|
| 按名补全缺引擎全局面(类/内建类型/单例/工具函数/全局枚举及值/常量) | `globals.rs` 数据层:名单来自官方文档表,签名/值/文档以 `extension_api.json` 模型逐名验证,`z` 前缀降权排序 |
| `sin`/`preload`/`print` 等全局函数 hover 错误(`Callable`)或无文档 | 全局面 hover 兜底:签名(模型渲染)+ 文档 + 官方链接 |
| `extends Node` 类名、单例名 hover 无文档跳转 | 内核有文档 → 追加 zh-CN 官方文档链接;无文档 → 全局面合成 hover |
| 36 个注解(`@export_range` 等)无 hover 文档 | 构建期从官方文档提取(36/36),`tools/extract_docs.py` 可复跑 |
| 内建函数(`preload`/`range`/…)无文档 | 同上,含官方签名行 |

验证:cargo test 15/15(含探针转化而来的全局面覆盖断言与成员上下文隔离断言),clippy 干净。

## 一、参照系

**官方 godot-vscode-plugin 能力面**(用户视角):语法高亮、连接 Godot 编辑器内建 LSP(补全/悬停/诊断)、调试器(断点/单步/变量)、运行项目/场景、场景跳转。其 LSP 依赖**打开着的 Godot 编辑器**——godotlsp 的差异化正是"脱离引擎 + 内嵌模型",调试/运行类交互则属 DAP 域(见 `lsp-design.md` P2-18,不混入 LSP)。

**生态工具参考**:
- `gdtoolkit`(gdformat/gdlint)— 社区格式化与 lint 事实标准;服务器格式化器当前自研,行为对齐 gdformat 是兼容性关键
- Godot Files(VS Code)— 纯语法包;我们的语法文件已覆盖
- Godot 4.x 官方文档 — 数据源(godot-docs 语料已入库为本地语料,构建期提取)

**社区高频诉求**(godot-vscode-plugin issues 与 GDScript 社区常年重复的主题):
断点调试、不开编辑器运行、gdformat 一键格式化、`$NodePath` 场景节点补全与跳转、信号连接可视化、悬浮文档中文、实时的类型错误提示、类/文件模板。

## 二、路线图

### P0 — 提示与准确性(服务器侧)✅ 已完成

1. ✅ **成员补全 detail/doc 增强**:`recv.<TAB>` 经接收者 hover 定位类型,引擎模型回填完整签名与文档(方法 `(…) -> R`、属性 `name: T`、信号/常量);自有成员兜底为声明行原文 + `##` 文档注释。
2. ✅ **签名帮助文档填充**:`sin(` 等全局调用回填模型文档;`recv.method(` 经接收者类型定位类取成员文档;均附 zh-CN 链接。
3. ✅ **关键字 hover**:35 个关键字/伪常量别名一行说明(`extract_docs.py` 从 GDScript reference Keywords 表提取)。
4. ✅ **注解 detail 回填**:36 个注解补全带一行说明 + 完整文档 + 链接。
5. ✅ **诊断分级/降噪**:分级由内核 `WarningSettings`(Godot 编辑器同款 per-code)+ 严格模式开关承担;新增 `diagnostics.ignore` 按码忽略配置(双通道下发)。

### P1 — 交互(服务器侧场景感知 + 编辑器侧入口)

6. ✅ **运行/调试入口(运行类)**:插件提供「运行项目/运行当前场景」命令(集成终端调 `godot --path`,可配 `godotlsp.godotExecutable`);DAP 调试仍走 P2-13 独立适配器,**不进服务器协议**(域隔离准则)。
7. ✅ **`$NodePath` / `%Unique` 场景感知增强**:内核 M2 链路已具备(补全带节点 `type=` detail、hover 类型推断、跳转定义);服务器补节点路径上下文识别,`$`/`%` 上下文下不再合并全局面(探针实测 1662 项污染 → 纯节点列表);取模 `%` 经操作数前缀启发式排除。
8. **格式化桥接**:优先内建格式化器;提供 `godotlsp.format.engine: gdformat` 开关,外部工具存在时委托(行为兼容诉求)。
9. ✅ **snippet 集**:`@export`/`@onready`/信号声明、`_ready`/`_process`/`_physics_process`/`_input` 回调、`match`、lambda、定时器、信号连接、场景实例化等 16 组模板(插件 contributes,中文描述)。
10. **信号连接视图**:列出脚本内 `connect`/`@rpc` 与场景信号连接(读 .tscn `connection` 段),CodeLens 或面板。

### P2 — 生态与规模

11. **离线中文文档集成**:godot-docs 语料构建期压缩索引,hover 长文可切中文(P2-17 of lsp-design.md,同管线扩展)。
12. **`godotlsp check` CLI**:CI 静态检查,与服务器共享内核。
13. **DAP 适配器**:独立子命令 `godotlsp dap`,桥接 Godot 调试协议。
14. **多根工作区**、诊断节流、线程模型(见 lsp-design.md P0/P1,前置项)。

## 三、取舍原则

- **数据一律双源验证**:名单可来自文档语料,签名/值/文档只认 `extension_api.json` 模型;语料与引擎版本漂移不影响正确性。
- **服务器做语言智能,插件做交互**:运行/调试/视图类进插件,协议保持纯 LSP。
- **构建期提取,运行时零解析**:任何文档管线都在 build 时完成,服务器保持"无文件系统时钟线程"的可嵌入形态。
