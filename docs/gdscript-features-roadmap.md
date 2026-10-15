# GDScript 开发体验路线图(数据层 → 提示 → 交互)

以"数据准确 → 提示更强 → 交互更全"为序,结合官方插件能力面、社区高频诉求与生态工具,规划 godotlsp 的功能演进。服务器侧与编辑器侧分层标注:服务器能力所有 IDE 共享,编辑器侧仅进 VS Code 插件。

> **配套文档**:`gdblue-roadmap.md`(可视化蓝图 GDBlue 的专属路线图,P3 条目已迁出扩写);`rejected-designs.md`(**不做/排除/降级决策汇总**——各条目内的取舍记录保留原语境,该文档是其分类聚合 + 完整理由与复活条件)。

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

**图视图与索引类参照(P2-19/20 设计输入,2026-10-16 调研)**:
- **SourceDive**(tariq10x,VS Code 扩展)— 代码库交互图浏览:文件清单 → 语义索引 → 符号检索 → 关系图与编辑器双向跳转;快捷键范式(Ctrl+G 搜索、Shift+Click 展开/折叠、双击下钻、Alt+Left/Right 历史、F/R 适配与重排)、Whiteboard 模式;用户点名参考,交互规格见 P2-19。
- **JSON Flow**(ManuelGil/vscode-json-flow,2026-10-16 核验)— React Flow + Entitree Flex Tree + Web Worker 布局的实证组合;确定性布局、折叠子树 `(+N hidden)` 命中计数、结构化搜索 token + 三投影模式(highlight/focus-context/focus-strict)、四方向布局、minimap、双向 live sync(100ms 节流)、导出排除 UI;P2-19 交互全集的规格来源。
- **rust-analyzer `viewCrateGraph`** — 自定义 LSP method + webview 渲染图的业界顶级先例;P2-19 B 方案(`godotlsp/graph`)的直接依据,也是「additive capability 不破坏兼容性」论证的支撑。
- **fnMap**(chensuiyi,闭源付费,2026-10-16 README 调研)— 多区块合一侧边栏(符号/区域/TODO/标签/置顶)的交互范式:`todoTags`/`todoColors` 自定义分类、`#region` + Folding Markers 双语法、每区块实时过滤、pin/tag 工作区级持久化、isMergeRegionsAndSymbols 区块融合;大纲增强包(P2.5-21~25)的交互来源。
- **Outline Map**(Gerrnperl/outline-map,开源,2026-10-16 源码核验)— **不做语言解析、纯消费 LSP** 的哲学与本项目同构;源码实证:follow 双模式(cursor 300ms debounce / viewport 100ms throttle)、region/tag 状态机解析器(405 行 token 级+语义高亮)、搜索三模式(`/`正则 `?`模糊 默认子串)+ `@kind` 前缀过滤 + 增量复用、工作区符号池(excludes 三级 target + IncludeSymbols + 不活跃剔除)、诊断旗标进大纲、findRef 点击跳转显示引用;P2.5-21~25 与 P2-19 渐进披露的直接依据。
- **codemap**(FMATheNomad,开源,2026-10-16 README 调研)— D3 force-directed 依赖图:**分析指标进视觉通道**的范式——节点大小=PageRank 中心度、循环依赖红虚线、入口点橙框、孤儿文件半透明、CodeLens 依赖计数内联;P2-19 E1-G 的指标层与孤儿/循环检测来源。
- **CodeCharta**(MaibornWolff,开源)— 3D 城市隐喻(文件=建筑、指标=高度/颜色)宏观架构图;对本项目过重(3D/CI 管线),只取「同一指标映射多重视觉通道」思想。
- **CodeVisualizer**(litong-2017,开源)— Tree-sitter WASM 解析 → IR → Mermaid 渲染,函数级控制流 flowchart + 实时刷新;远期 gd 函数 flowchart 的技术路线参照。
- **dependency-cruiser**(sverweij,开源)— **依赖规则即代码**(validate 规则文件 lint 架构约束);远期「架构规则 lint」(如 UI 场景不得 preload 核心逻辑)参照。
- **godot-vscode-plugin**(官方,已 clone 源码核验 2026-10-16)— gap 审计基准:其 scene_tools 有 mtime 缓存单例解析器(ext/sub_resource/node 三循环正则)、**509 个官方节点类型图标**(MIT,`resources/godot_icons/{light,dark}`,可直接取用进我们的场景树)、FileDecorationProvider(被场景挂载的 .gd 文件徽章)、ExtResource/SubResource 行内 inlay hints、gd↔tscn 切换(alt+o);**缺**:图视图、uid 映射、信号连接图、节点→素材边、跨场景架构视图、孤儿/循环检测——全是 godotlsp 的差异化空间。
- **IntelliGit**(marketplace,MIT,2026-10-16 README 调研)— JetBrains 风格 Git 工作台;**P2-19 工程架构的直接印证**:①四层分离 `GitExecutor(进程)→GitOps(操作层)→ViewProviders(编排)→Webviews(React 渲染)`,与我们 GraphIndex→GraphQuery→ViewProviders→Webview 同构;②**图渲染 = HTML5 Canvas**(分支泳道,React 18 webview 内)——SVG/DOM 图对超大历史的性能升级路径实证;③**undock 工作台**(面板合一 + 编辑器 tab/新窗口双形态 + 工具栏按钮可见性独立开关);④**actionable history**:图节点右键 10+ 操作,风险动作(force push)带上下文守卫与确认;⑤外部状态(远端 CI)进图节点徽章 + popover 展开;⑥TSDoc 棘轮纪律——「注释捕获契约/不变量/副作用/信任边界,拒绝复述类型的注释,禁止削弱 lint 落地未文档化导出」。
- **vscode-git-flow**(开源预览版,2026-10-16 README 调研)— 反面 caution:把特定工作流(git flow 的 master/develop/feature 分支模型)硬编码进 UI,项目组织习惯一变即废;资源图谱设计保持**数据驱动**(边类型来自文件事实,不预设项目组织方式)。
- **hediet/vscode-debug-visualizer** — Webview + vis.js/graphviz 渲染图数据的成熟先例,JSON 图谱协议(nodes/edges)可借鉴为图数据交换形状。
- **VS Graph / Dependency Cruiser Extension** — D3/cytoscape 系 Webview 关系图与文件依赖图的交互基线(缩放/拖拽/点击跳源码)。
- **gdls**(kurushimee/gdls)— `--dump-extension-api-with-docs` 自动 dump 工程学文档级实证(ClassDB 完备性、dump 落点、退出 abort 仍采纳、5 分钟上限、解析梯度),P2-15 引擎模型 v2 的对照参照;其 `doc/classes/*.xml` 否决结论一并采纳。
- **VS Code 接口面**:WebviewView(侧边栏常驻)/ WebviewPanel(标签页);TreeView + TreeDataProvider(结构视图);内建「调用层次/类型层次」UI 消费 LSP callHierarchy/typeHierarchy——图视图的数据与交互一律走这些公开接口,不自造协议。

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

### P1.5 — 插件完备性(编辑器侧,2026-10-15 追加)

**背景**:用户实测报告「已加载 N 个项目文件」等服务器信息日志在 VS Code 输出通道显示为 `[error]`(stderr 一律按 error 级展示),并提出插件独立完备性诉求。

**工程化审计轮(2026-10-16,对标业界:vscode-languageclient v10 规范 / pyrefly 启动日志模式 / CWE-77 注入防护)**:

- ✅ **LSP 通信同构化**:新建 `src/lsp.ts` 统一出口——`isRunning` 守卫 + `documentSymbols`(统一 try/catch 降级为 null + LogOutputChannel.debug)+ `LspSymbol`/`iconForKind` 单份映射;符号树与项目树共用,消除三处重复请求逻辑;协议纪律:只用标准 LSP method,无私有扩展。
- ✅ **死代码清理**:`symbols.ts` 未被消费的 `emptyMessage`/`message` 状态、重复的 `iconFor`(ThemeIcon 版)映射表、`sceneTree` 遗留 `client` 参数,全删;tsc `noUnusedLocals` 强制兜底。
- ✅ **纯解析器抽取 + 零依赖单测**:`parseAutoloads`/`parseScene` 移入 `src/parsers.ts`(零 vscode/LSP 依赖),`npm test` 用 **node:test 内建框架**(不加任何 dev 依赖)跑 5 个用例(autoload 段/容错挂载/行号定位);测试中发现并修正层级挂载语义(根节点名不出现在 parent 路径,`.` 即根)。
- ✅ **命令命名统一**:`godotlsp.symbols.goto` → **`godotlsp.goto`**(三树共用,不再挂在符号树名下);刷新命令保持 `godotlsp.<视图>.<动作>` 同构;`rg` 确认旧名零残留。
- ✅ **终端注入防护(CWE-77)**:`runGodot` 重写引述策略——POSIX 单引号 + 内部 `'` 转义;Windows 拒绝含 shell 元字符(`"` `` ` `` `$` | & ; ( ) < >)的路径并明确报错(与其造不完美的双 shell 转义轮子,不如对配置路径收紧校验);工作区文件名派生参数同过校验;`--path` 从相对 `.` 改为显式引述路径。
- ✅ **LogOutputChannel 分级日志**:`appendLine` ×3 全部改 `info`/`error`;新增**结构化启动日志**(pyrefly 模式):`[mode=configured|bundled|path] + 路径`一行说明二进制选择依据;`trace.server` 补 `"scope": "window"`(v9+ 规范形状)。
- ✅ **打包卫生**:`.vscodeignore` 补 `tests/**`/`tsconfig.test.json`/`out-test/**`;README 命令表同步。

11. **服务器信息日志改走 `window/logMessage`**(R2 恢复构建时生效):初始化后的信息类日志(已加载文件数、引擎模型就绪)经 `MessageType::INFO` 通知送达,警告类(配置拉取失败)走 `WARNING`;stderr 只保留初始化前致命错误与运行期异常。已改 `crates/godotlsp/src/main.rs`(`log_notify`)+ `state.rs`(`load_project` 返回文件数)。
12. ✅ **专属侧边栏符号树**:activitybar 加 godotlsp 视图容器 + 「当前文件符号」TreeView,数据源走 LSP `textDocument/documentSymbol`(服务器早已支持,类/函数/枚举/信号/常量嵌套),点击跳转;VS Code 内建大纲视图同步可用。
13. ✅ **Godot 程序化文件高亮**:新增 `godot-resource`(.tscn/.tres)、`godot-project`(project.godot 与 *.godot)、`godot-uid`(.uid)语言 + TextMate 语法,节头/属性/字符串/数值/颜色/`ExtResource`/`SubResource`/`NodePath` 分类着色,格式对照上游 `resource_format_text.cpp` 核验;文件监视器已含这些类型。
14. **无 Godot 完备运行保证**:LSP 全特性(hover/补全/导航/诊断/符号/格式化)运行期零 Godot 依赖(内嵌引擎模型,见 from-scratch-design.md D2);`godotExecutable` 仅是「运行项目/运行场景」的可选入口,缺失时给出明确指引而非静默失败。每次插件改动回归此项。
15. **documentHighlight / 写入处 hover 标注错误**(用户截图实证,2026-10-16):悬停 `match` 块内写入处(如 `_wait = 60`)时,hover 只回裸类型 `int`(缺声明行 + `##` 注释),且高亮框落在 `match _phase:` 整行而非该变量的第 2/3 行出现处。疑似内核在写入出现处的名称解析错位(`match` 模式行上下文);旧内核属外部 crate(gdscript-ide,R0 已切),本仓库不可修——**R2 重写后以此截图场景为验收探针**:写入处 hover 须带声明行与文档,documentHighlight 须覆盖全部读写出现处且范围精确。

### P2 — 生态与规模

11. ~~**离线中文文档集成**~~ → **并入 P2-16**(范围扩大:在线跳转 + 离线索引 + 双语回退策略)。
12. **`godotlsp check` CLI**:CI 静态检查,与服务器共享内核。
13. **DAP 适配器**:独立子命令 `godotlsp dap`,桥接 Godot 调试协议。
14. **多根工作区**、诊断节流、线程模型(见 lsp-design.md P0/P1,前置项)。
15. **引擎模型摄取 v2(服务器侧,P2 首位)**:按 `from-scratch-design.md` §7 执行——主源改为
    `godot --headless --dump-extension-api-with-docs` 产物(走 ClassDB,完备-by-construction;已核验,gdls 同款结论),
    自研五级解析梯度(显式路径 → 工作区 dump → 自动 dump → 内嵌出厂面 → 动态降级),
    内嵌面只从 Godot 官方源码 pinned commit 摄取,**不再复用任何第三方预构建 blob**;
    版本对齐规则、GDExtension 缺类检测、二进制变更热重载均按 §7.3。验收:无 Godot 环境全特性可用(内嵌面)+ 有 Godot 环境模型版本与引擎一致(自动 dump)。

### P2.5 — 文档与项目理解(编辑器侧 + 服务器数据层)

16. **中文文档集成(hover 双语 + 跳转)**:
    - **数据**:godot-docs 中文语料(官方 CC-BY 3.0,署名「Juan Linietsky、Ariel Manzur 和 Godot 引擎社区」;仓库已收孤辰辰译自 GodotEngine.epub 的语料)。构建期按「符号键 → 中文摘要 + 官方页面路径与锚点」抽取索引(hover 长文不进模型,按需取摘要)。
    - **hover 渲染**:签名/参数/默认值以 `extension_api.json` 模型为权威(数据准确性原则);描述文字**中文优先,缺失回退英文**;尾部附「打开 zh-CN 官方文档」链接(`https://docs.godotengine.org/zh_CN/stable/<页面路径>#<锚点>`)。
    - **离线模式**:配置 `godotlsp.docs.offlinePath` 指向本地 godot-docs HTML/ePub 解压目录时,跳转改为 `vscode.open` 本地文件;未配置走在线。
    - **验收**:类/方法/属性/信号/枚举 hover 中文覆盖率和 zh-CN 链接正确性抽检 ≥95%;语料版本漂移不影响签名正确性(只影响描述文字)。
17. **场景/资源文件语义识别(tscn/tres/godot/uid 从「高亮」升级为「识别」)**:
    - ✅ **tscn 节点树大纲(插件侧,2026-10-16 已实现)**:侧边栏「场景结构」视图(`src/sceneTree.ts`),纯插件侧解析 `[node name= type= parent= instance=]` 层级(parent 路径 "." 为根,孤儿节点容错挂根级),instance 节点 description 显示 `ext_resource` 解析出的资源路径;点击跳转 tscn 声明行;随活动编辑器/文档变更自动刷新。
    - 服务器扩展项目索引:解析 `.tscn`/`.tres` 的节头结构(`ext_resource`/`sub_resource`/`node`/`connection`),建立 **`uid://` ↔ res:// 路径双向映射表**(Godot 4.4+ 的 `.uid` 文件与节头 uid 属性双来源);
    - `documentLink`:`.tscn`/`.tres`/`project.godot` 中的 `res://` 路径与 `uid://` 引用可点击跳转(服务器侧,待 R2);
    - `$NodePath`/`%Unique` 补全联动 uid 映射(场景被改名/移动后仍可解析);
    - 验收:对 `corpus` 内真实工程夹具,uid 映射覆盖 100% 带.uid 文件;documentLink 协议合规(范围精确到字符串字面量)。
18. **项目结构视图(fnmap/outmap,大纲式组织 + 跳转)**✅ 已实现(2026-10-16,`editors/vscode/src/projectTree.ts`):
    - ✅ 「项目结构」TreeView:autoload 单例(解析 project.godot `[autoload]` 段,`*res://` 前缀剥离)→ `class_name` 全局类注册表(工作区 .gd 扫描:只看前 64 行、排除 `.godot`/`.git`、上限 5000 文件)→ 任意脚本惰性展开复用 LSP `documentSymbol`(与符号树同源);
    - ✅ 点击跳转统一走 `godotlsp.symbols.goto`(零基行列,越界钳制);class_name 注册表**仅保存时重建**(防击键级全工程扫描);三个视图各带 view/title 刷新按钮;
    - 后续增强(可选):autoload/类节点引用计数角标、与 P2-19 图视图共用索引。
19. **交互式关系图(v3 执行设计,2026-10-16;参照:IntelliJ 图视图 / SourceDive / JSON Flow)**:
    - **定位**:把代码库当图浏览——**不是代码历史,是语义关系**:文件/符号/类/场景为节点,包含/继承/调用/信号/实例化为边;编辑器 WebviewPanel(主)+ 侧边栏 WebviewView(常驻迷你态)双形态;**undock 工作台(IntelliGit 实证)**:多面板合一可 undock 到编辑器 tab 或新窗口(多显示器工作流),工具栏按钮可见性独立设置。
    - **架构分层(IntelliGit 四层印证)**:`GraphIndex(扫描/缓存:资源边抽取 + uid 归一)→ GraphQuery(查询/物化:两级物化 + revision 失效)→ ViewProviders(扩展宿主编排)→ Webview(React 渲染)`——数据/操作/视图分离,扫描层无 vscode 依赖可单测(与 parsers.ts 同纪律)。
    - **图数据模型**(服务器与插件共用):`node { id, kind: file|symbol|class|scene|resource|engine-class, label, uri?, range? }` + `edge { from, to, kind: contains|extends|calls|signal|preload|instance|link }`;节点 id 稳定(URI+符号路径),保证 JSON Flow 式**确定性布局**(同输入同布局,折叠态/搜索态跨刷新保持)。
    - **分析架构(核心取舍,三选一)**:
      - A. 纯 LSP 聚合(`workspaceSymbol`+`references` 逐符号查询拼图):兼容性满,但 O(符号数) 次请求,千文件工程不可行——只作兜底。
      - B. **自定义请求 `godotlsp/graph`(主选)**:服务器从 R2 内存索引直接物化图 JSON,一次请求。**先例:rust-analyzer `viewCrateGraph`**(同为自定义 LSP method + webview 渲染,业界顶级项目实践)。修订此前「零私有 method」纪律:该纪律本意是**核心语言特性**不分裂(所有 IDE 平权);图视图是 additive capability——能力协商声明 `godotlsp.graph:boolean`,客户端不请求则完全不存在,无兼容性损失。
      - C. 插件侧纯文本解析(contains/extends/preload 边):零服务器依赖,**E1 立即可做**。
      - **决策:B 为主 + C 先行 + A 兜底。**
    - **高效分析结构(服务器侧,E2)**:复用 R2 内存索引(符号表+调用解析),**两级物化**——①小图(≤800 节点,可配):全量物化 + revision 缓存(沿用快照 revision,写后失效,过期重建);②大图:JSON Flow 式**惰性展开**——根=活动文件/符号,`godotlsp/graph?root=<uri>&depth=1` 只取 1-hop 邻域,服务器侧 per-node 邻接缓存,绝不全量物化(防 OOM 与卡顿)。断链(引用不存在目标)以虚线节点呈现而非报错。
    - **渲染技术(E1 选型核验,2026-10-16 修订)**:React Flow(DOM/SVG)+ **Web Worker 算布局**组合不变(参照 JSON Flow 实证);**布局引擎从 Entitree Flex/dagre 修订为 ELK layered(主)+ Entitree Flex(纯树图)**——双源实证:①React Flow 官方布局指南中 ELK 是唯一同时支持动态节点尺寸+子流程布局+边路由的引擎;②wobu spike benchmark:1000 节点 elk 619ms vs dagre 3384ms(5.5×),304 节点图高度少一半,交叉数/对齐偏差(elk skew 110px vs dagre 216px)全面占优;③dagre 对「子→父复合边」有 6 年未修的致命 bug(AzureCraft 评估),资源图谱的 scene→node 包含边正中此雷;④**两者输出均确定性且 byte-identical**,与我们的确定性布局要求吻合(elk `hierarchyHandling: INCLUDE_CHILDREN` + `edgeRouting: ORTHOGONAL`);elkjs gzip ~435KB 须 dynamic import 按需加载,照旧 vendored 离线打进 vsix。**升级路径(IntelliGit 实证)**:全工程大图若超 2000 节点,SVG/DOM 碰壁,切 Canvas/WebGL 渲染层(cytoscape canvas 或自绘泳道/力导向);当前两级物化(≤800 全量 + 1-hop 惰性)使 React Flow 恒在小图域,选型不变,只记升级触发线。
    - **交互全集(JSON Flow 已核验 + SourceDive 对齐 + IntelliGit 增补)**:①任意节点折叠/展开子树,折叠子树内搜索命中显示 `(+N hidden)` 计数;②结构化搜索 token(`kind:calls`、`uri:res://x.gd`、`depth>2`,隐式 AND,250ms 防抖)+ **三投影模式**:highlight(全图可见命中加环)/focus-context(命中+祖先链)/focus-strict(仅命中);③四方向布局 TB/LR/BT/RL 旋转;④minimap + 缩放平移 + Fit;⑤双向 live sync(编辑器光标 ⇄ 图节点互跳,100ms 节流,默认关);⑥快捷键:`Ctrl+G` 搜索、`F` fit、`R` 重排、`Alt+Left/Right` 历史、`Shift+Click` 展开/折叠、双击跳编辑器声明(复用 `godotlsp.goto`)、右键按边类型过滤(对标 IntelliJ);⑦导出 PNG(2×)/SVG,自动排除 UI 控件与小地图;⑧**节点操作菜单(IntelliGit actionable history 范式)**:右键 → 跳转声明/显示引用(L3,条目 22)/以此为中心重排/复制 res:// 路径与 uid/隐藏节点——「图节点即可操作对象」;**诊断徽章**:文件/符号诊断进图节点(红/黄角标,popover 展开明细,IntelliGit CI checks popover 同构)。
    - **执行分期(按依赖排)**:
      - **E1 插件侧包含图(零服务器依赖,可立即做)**:webview shell + React Flow vendored + 边来源=parsers.ts 的 tscn 解析(instance/contains)+ 文本级 `extends`/`preload`/`class_name` 解析 + 上述交互全集(搜索/投影/布局/live sync);
      - **E2 服务器图查询(R2 后)**:`godotlsp/graph` 请求 + capability 协商 + 两级物化 + revision 失效;
      - **E3 调用/信号边(依赖 P2-20)**:`calls`/`signal` 边入图,类型层级着色;
      - **E4 打磨(P3)**:SourceDive Whiteboard 模式(白板标注/自由摆放;技术选型 **Excalidraw/tldraw 嵌入 webview**——两者均 MIT 可 vendored,Excalidraw 元素模型可直接映射图节点/手写标注/箭头,白板快照 JSON 可随工程保存)、图快照导出为文档资产(**新增 Mermaid 文本导出**:图 JSON → mermaid flowchart 语法纯字符串生成,零渲染依赖,快照直接进 PR 描述/README/设计文档——「图即代码」,diff-able;PNG/SVG 导出保留)。
    - **付费/云工具取舍记录(2026-10-16 调研)**:GoJS/yFiles(顶级图布局,商业授权 + vendored 冲突)不采纳,yFiles 层级布局质量作为 ELK 调参的标杆参照;AWS/Azure/GCP 云架构图工具(Application Composer 等)面向基础设施拓扑,与本地代码库资源图谱无关,不采纳;iconify 图标聚合仅作 codicon/Godot 官方图标之外的兜底(图标纪律:VS Code codicon 优先 → Godot 官方图标 → iconify 通用,不引第三方付费图库)。
    - **E1-G Godot 资源图谱(2026-10-16 立项,E1 的 Godot 特化层;参照 codemap 指标层 + 官方插件 scene_tools gap 审计)**:
      > 用户构想落地:把「脚本、节点、素材相互调用」理成一张图。边抽取是纯文本/结构解析(parsers.ts 同构),uid 归一规格见 `engine-analyzer.md` U 节。
      - **完整边类型清单**(官方插件只特判 script/instance 两类,我们泛化到全部资源语义):
        | 边 | 来源 | 抽取方式 |
        |---|---|---|
        | scene→script | tscn 节点 `script = ExtResource("id")` | ext_resource 表 + 节点属性扫描 |
        | scene→scene | tscn 节点 `instance = ExtResource("id")` 指向 .tscn | 同上,按扩展名分流 |
        | scene→asset | tscn **任意属性**的 `ExtResource("id")`(texture/audio/stylebox/…官方未做) | 全属性 ExtResource 泛化扫描,目标按 ext 分类 |
        | node→node | tscn `[connection from= to= signal= method=]` | engine-analyzer.md S 节规格 |
        | script→script | `extends` / `preload()` / `const X = preload(...)` / class_name 引用 | gd 文本扫描 |
        | script→scene/asset | gd `preload("res://x.tscn")` / `load(...)` 按扩展名分流 | 同上 |
        | project→script | project.godot `[autoload]` 段 | parseAutoloads 已有 |
        | tres→script/asset | tres 的 ext_resource(材质脚本/嵌入资源引用) | tscn 同款解析器复用 |
      - **uid 归一层**:所有 path 键先过 uid↔path 双向映射(4.4+ `.uid` 侧写扫描),改名/移动不断链;映射表 = 插件侧扫描 `.uid` 文件 + tscn 节头 uid(与 P2-17 服务器索引同一数据模型);
      - **指标层(codemap 实证)**:节点大小=被引用数(场景被 instance 数/脚本被挂载数);孤儿资源(零入边)半透明呈现——**「删这个场景前看谁引用它」是真实高频需求**;循环 preload 红虚线(Godot 运行时报错,静态检测提前拦截);
      - **视图形态**:默认以活动文件为中心 1-hop(渐进披露 L4,见条目 22),命令面板「显示资源关系图」可切全工程(≤800 节点);节点双击跳 `godotlsp.goto`,tscn 节点显示所在场景+父链;
      - **官方图标复用**:509 个节点类型图标(MIT,官方插件 `resources/godot_icons/{light,dark}`)提取进我们场景树 + 图节点,替换通用 ThemeIcon——按 Godot 蓝标识体系,场景树的视觉专业性一次到位;
      - **官方插件 gap 清单(全部是我们的差异化空间)**:图视图、uid↔path 映射、信号连接图、节点→素材边、跨场景架构视图、孤儿/循环检测——上述全部已在本条覆盖;
      - 验收:corpus 工程全资源扫描无漏边(与手工清单比对);uid 归一后改名场景不断链;孤儿/循环检测在注入夹具上准确。
    - **验收**:千文件工程展开单节点 <100ms(惰性)/ 小图全量 <2s(增量 <200ms);布局确定性(同输入同布局);live sync 往返不丢焦点;导出图无 UI 控件残留;主题明暗两态可读。
20. **callHierarchy / typeHierarchy(服务器侧,P2-19 E3 前置)**:
    - LSP 3.17 `textDocument/prepareCallHierarchy` + `callHierarchy/incomingCalls|outgoingCalls`(VS Code 内建「显示调用层次」UI 直接受益,不依赖 20 的图视图);`prepareTypeHierarchy` + `supertypes|subtypes`(继承树导航);
    - 内核语义层需输出:函数调用点集合(`CallExpr` → callee 解析)、`extends`/`is`/`as` 的类型引用集合;
    - 验收:递归调用、跨文件调用、lambda 内调用三类夹具的 incoming/outgoing 正确;与 references 结果一致性断言(调用点 ⊆ 引用集)。

21. **大纲增强包(2026-10-16 立项;双实证:fnMap README + Outline Map 源码;全部插件侧,零服务器依赖)**:
    > 统一原则:TreeDataProvider 不重写 webview(现有三视图形态保持);Outline Map 哲学——**插件不做语言解析,只消费 LSP/纯文本扫描**。
    - **21a 区域/标签大纲**(`#region Name` / `#endregion` / `#tag Name 描述`,Godot 官方注释语法):parsers.ts 加纯函数扫描器(行级正则起步,Outline Map 状态机是过度设计)→ 符号树顶部加「区域」段,region 容器嵌套子符号(区间包含判断),tag 平铺;`#`/`#tag` 语法着色进 GDScript.tmLanguage(低成本)。fnMap/Outline Map 双实证的核心特性。
    - **21b 大纲搜索语法**(Outline Map 模式):每视图过滤输入,默认子串,前缀 `/` 正则、`?` 模糊(subsequence)、`@kind` 按符号类型过滤(可组合:`?@function render`);TreeDataProvider 内置过滤状态 + refresh,复用现有 providers。
    - **21c Follow(光标 → 树定位)**:`onDidChangeTextEditorSelection`(debounce 300ms,Outline Map 同参数)→ 计算光标所在最深符号(区间包含)→ `tree.reveal(node, {select:true, expand:true})`;viewport 映射高亮 TreeDataProvider 做不到,降级接受(记远期 webview 版)。
    - **21d 诊断旗标进大纲**:`vscode.languages.getDiagnostics` 区间包含符号 → TreeItem 叠加错误/警告角标(description 前缀或 ThemeIcon),Outline Map 实证范式,零协议成本。
    - **21e TODO 扫描与高亮**(fnMap 模式):配置 `godotlsp.todoTags`(默认 TODO/FIXME/NOTE/HACK/XXX)+ `godotlsp.todoColors`(按 tag 着色);插件侧行扫描 → 符号树「TODO」段 + 编辑器 decoration 高亮(ruler + 注释底色);tag 大小写不敏感,格式 `TAG: 描述` 与 `TAG(分类): 描述`。
    - **21f 官方生态补强(官方 godot-vscode-plugin 已实证,MIT 合规)**:① 509 个官方节点类型图标提取进场景树(替换通用 ThemeIcon,见 E1-G);② FileDecorationProvider——被场景挂载的 .gd 文件在资源管理器加徽章(官方 ScriptDecorationProvider 同思想,可升格为引用计数角标);③ ExtResource/SubResource 行内 inlay hints(官方 inlay_hints.ts 实证:显示资源路径与类型,正则级成本);④ gd↔tscn 快速切换命令(官方 alt+o 先例,配对规则:同名文件/`script = ExtResource` 关联);⑤ **Godot 文件图标主题**(2026-10-16 调研确认):`contributes.iconThemes` 贡献点 + icon-theme.json(iconDefinitions/fileExtensions/languageIds/fileNames + light/highContrast 双态),用官方 Godot 素材给 .gd/.tscn/.tres/.uid/.godot/.gdshader/.gdextension/.import 配图标——资源管理器一眼识别 Godot 工程,官方插件没有此贡献,纯增量;输出为可选主题不强制替换用户现有主题。
    - 验收:corpus 夹具的 region 嵌套/搜索三模式/follow 定位/TODO 高亮各一测;三视图共用同一套过滤组件(同构复用)。

22. **渐进披露四层级(P2-19 的交互理论修订,Outline Map findRef + SourceDive 下钻综合)**:
    > 关系图不是孤立视图,而是大纲的「展开到极端」。四级披露,每级保留上下文,用户在任意层级满足即停:
    > **L1 大纲**(符号树,已有)→ **L2 过滤**(21b 搜索,范围内定位)→ **L3 引用展开**(节点级局部关系:点击/右键符号 → references 列表内联展示,Outline Map findRef 实证——对变量/成员这类低引用数目标,引用列表比全图更实用;数据走 LSP references,零服务器新增)→ **L4 全图**(P2-19 godotlsp/graph,以节点为中心 1-hop)。
    - 落地:**L3 进 P2-19 C 方案增强**(大纲节点右键「显示引用」→ QuickPick/树内子节点展示,点击跳转);L4 的图视图入口从大纲节点发起(root=该符号),与 SourceDive 双击下钻同构;
    - **变量识别增益**:变量的关系通常只需 L3(定义 + N 个引用点),L4 图对变量只在其作为跨文件数据流枢纽时才有价值——关系图的节点默认聚合到「类/场景」粒度,变量级展开按需(L3 已覆盖),避免图爆炸。

### P3 — 可视化蓝图 GDBlue(2026-10-16 立项)

> **已迁移至独立文档 `docs/gdblue-roadmap.md`**(含构想、源码依据、38 节点分类学、BIR 规范、V1-V4 分期、状态图/ECS、AI 协同、画布体系取舍)。
> 一句话:把 Godot 3.x Visual Script 以现代化蓝图形态复活在 VS Code(类 ComfyUI / Unity Visual Scripting)——静态类型端口 + JSON IR(.gdblue.json)+ transpile 生成 GDScript 进 gdscript-engine 验证闭环;蓝图是人机共享 IR(AI 写 JSON、人操作编辑器);V4 原生 State Graph 生成组合式状态机(组合大于继承)。依赖 P2-19 E1 webview 基建,图谱先行。

## 三、取舍原则

- **数据一律双源验证**:名单可来自文档语料,签名/值/文档只认 `extension_api.json` 模型;语料与引擎版本漂移不影响正确性。
- **服务器做语言智能,插件做交互**:运行/调试/视图类进插件,协议保持纯 LSP(**修订 2026-10-16**:additive capability 类的自定义请求不违反本原则,见 P2-19 B 方案与 rust-analyzer viewCrateGraph 先例)。
- **构建期提取,运行时零解析**:任何文档管线都在 build 时完成,服务器保持"无文件系统时钟线程"的可嵌入形态。

## 四、X — 远期/条件触发(不承诺实现)

> 触发条件满足才立项;规格先行、代码不预写。

- **X1 — AI 资源管线(uid 生成器 + 资源绑定器)**:为「纯 AI 生成 Godot 工程」铺路——程序化生成合法 uid、写 `.uid` 侧写、绑定 ext_resource、一次 `--import` 注册、四门验证闭环。**规格已完整写在 `docs/engine-analyzer.md` UID 节(U1-U4,源码逐行核验)**,实现是照规格搬运,复杂度低;等有真实「AI 生成工程」消费方再落地。
- **X2 — 确定性路径派生 uid(复刻 `create_id_for_path`)**:需移植 Godot 私有 `String::hash64` + `RandomPCG`(PCG 变体)两个算法才能与引擎 bit 级一致;仅在出现「同路径必须稳定产出同 uid、且不能依赖引擎运行」的硬需求时立项,否则永远用 X1 的随机方案(resource_uid.cpp:134-155)。
