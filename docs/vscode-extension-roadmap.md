# VS Code 插件能力与路线（editors/vscode）

> 单一事实源：本文只管**编辑器侧**；服务器协议与补全能力见
> `completion-roadmap.md`（oracle 106/140 权威）、功能路线见
> `gdscript-features-roadmap.md`（P1.5 插件完备性等）、架构约束见 `lsp-design.md`。
> 2026-10-18 对接审计轮建立。

## 一、现状盘点（2026-10-18，实证）

### 1.1 组件清单

| 组件 | 文件 | 规模 | 说明 |
|---|---|---|---|
| 扩展入口 | `src/extension.ts` | 640 行 | 启动/重启/信任工作区/结构化启动日志 |
| LSP 统一入口 | `src/lsp.ts` | 70 行 | 只用标准 LSP method，树视图同构复用 |
| 树视图 ×4 | symbols / projectTree / sceneTree / connections | ~620 行 | 符号/项目结构/场景结构/信号连接，过滤+复制 |
| configAssist | `src/configAssist.ts` + `configFile.ts` | 936 行 | project.godot 等配置文件的补全/hover Provider |
| 语法高亮 ×4 | `syntaxes/` | — | gdscript / tscn·tres / project·cfg / uid |
| snippets | `snippets/gdscript.json` | — | 静态片段（服务器 C2 动态虚方法模板独立） |
| 命令 ×9 | — | — | 重启/输出/运行项目/运行场景/跳转/刷新×4/过滤×4 |

### 1.2 配置对接矩阵（逐键核对 server 消费面）

| 插件配置键 | server 端消费 | 状态 |
|---|---|---|
| `godotlsp.completion.snippets` | `Config.snippets`（C2 双闸门） | ✅ |
| `godotlsp.completion.fileWords` | `Config.file_words`（C3） | ✅ |
| `godotlsp.completion.frequencyBoost` | `Config.freq_boost`（C4） | ✅ |
| `godotlsp.inlayHints.typeHints` | `Config.inlay_type_hints` | ✅ |
| `godotlsp.path` / `trace.server` / `godotExecutable` | 客户端侧 | ✅ |
| `godotlsp.completion.useSingleQuotes` 等 **quote 三键** | `settings.gdscript` 段（P8） | ✅ V1a 接通（原缺口：server 就绪、插件不推送） |
| `godotlsp.diagnostics.strict` / `ignore` | **无**（server 无警告诊断面） | ⚠️ 预留标注（V1b），实现归 V2-W |

审计通过无需动：snippetSupport 协商（VS Code 默认 true，C2 双闸门生效）、
P9 构造 signatureHelp（VS Code 自动可用）。

## 二、V1 — 对接修复 ✅（2026-10-18 本轮）

1. **V1a quote 三键**：新增 `godotlsp.completion.{useSingleQuotes,addStringNameLiterals,addNodePathLiterals}`
   （默认 false，与 server 默认一致）。`extension.ts` 在 `client.start()` 完成后与配置变化时
   **手动合成 `settings.gdscript` 段** `didChangeConfiguration` 推送。
   决策记录：不贡献全局 `gdscript.*` 配置命名空间——避免与官方 godot-tools 等插件冲突，
   命名空间纯净性 > 配置共享便利；server 协议不变（oracle `settings.gdscript` 通道依赖）。
2. **V1b 悬空诊断键**：`diagnostics.strict` / `diagnostics.ignore` 两键 markdownDescription
   标注「预留(当前不生效)」并指向本文 V2-W。诚实原则：不承诺不存在的能力（server 目前
   只产出 ERROR 级诊断，warning 体系尚不存在，无码可滤）。
3. 版本 0.1.8 重打包。

## 三、V2 — 增强面（按优先级，动工前先对照 features-roadmap 锚点）

| # | 项 | 依赖/锚点 | 说明 |
|---|---|---|---|
| W | **警告诊断面**（`diagnostics.strict`/`ignore` 兑现） | semantic 加 warning 通道 + 码表（对齐 gdscript_warning.h） | server 侧工作为主；插件侧只等码过滤语义。这是两键从"预留"转正的唯一路径 |
| 1 | **21d 诊断旗标进大纲** | features-roadmap 186 行 | `vscode.languages.getDiagnostics` 区间含符号 → TreeItem 角标。零协议成本，纯编辑器侧 |
| 2 | **脚本侧信号连接** | features-roadmap 86 行 | connections 视图现只有场景侧（tscn 解析）；`connect()` 调用扫描与 `@rpc` 列举待增量 |
| 3 | **tscn 节点属性键补全** | configAssist.ts 头注「不做半成品」 | 属性键全集依赖引擎类数据库（节点类成员）——数据链路就绪后才做 |
| 4 | **格式化桥**（P1-8） | features-roadmap 84 行 | `godotlsp.format.engine: gdformat` 开关，外部工具存在时委托；先还自研格式化器对齐 gdformat 的债 |
| 5 | **DAP 适配器**（P2-13） | features-roadmap 112 行 | 独立子命令 `godotlsp dap`；launch.json schema 参照 godot-tools 全集 + 三档目标；**配置生成必须 merge-not-clobber**（godot-rust-vscode 反面教训） |
| 6 | **N6 场景树节点补全** | completion-roadmap P10 | server 侧 tscn 解析落地后，`$NodePath` 补全/跳转面在 VS Code 免配置可用 |

## 三·五、场景/资源引用面（tscn 侧专项，2026-10-18 立项）

> 用户实测发现：场景树只显示 `instance`（实例化场景）一种引用形态，
> 「绑定的脚本、多种纹理/音频素材全部不可见」。根因 + 设计如下。
> 参照系：官方 Godot Tools 的 GDResource 功能面（res:// 链接 Ctrl+点击、
> ExtResource/SubResource hover 预览、inlay 可视化、Alt+O 互跳、Scene Preview）
> + rust-analyzer 细节（lens 分开关、hover actions）+ VS Code 生态小工具
> （file nesting、image preview hover、documentLink）。

### 3.5.0 根因（实证 2026-10-18）

`parsers.ts parseScene` 只抓 `[node]`/`[ext_resource]`/`[connection]` 标签行——
**节点下的属性行零解析**：`script = ExtResource("1")`、`texture = ExtResource("2")`
等绑定关系全部丢失。`SceneFile.ext` 虽全量注册 ext_resource（含 type/path），
但树视图只消费 `instanceId`。引擎绑定判定锚点（`.refs/godot/scene/resources/
resource_format_text.cpp`）：**:1985** `is_script = name == CoreStringName(script)`
（属性行的 `script` 键即脚本绑定）；**:234-235** `instance=ExtResource("id")` 即
PackedScene 实例化；**:1634** `ExtResource("id")` 序列化格式；**:541** ext_resources 注册表。

### 3.5.1 A 系列（数据基础 → 展示 → 跨文件跳转）

| # | 项 | 设计要点 |
|---|---|---|
| **A1** | **属性行解析**（parsers.ts，数据基础，无它全免谈） | node 标签行**之后的缩进属性行**解析为 `bindings: Map<attr, extId>`（复用 `parseAttrs` 三形态正则）；`script`/`texture`/`stream`/`theme` 等任意 attr 不做白名单——引擎侧任意 OBJECT 属性都可能是 ExtResource。SNode 增 `bindings`；单测用真实形状夹具（script 绑定 + texture + audio 三类） |
| **A2** | **节点树绑定展示 + 打开对应文件** | 绑定脚本/实例化场景的节点：description 显示资源文件名（`main.gd`），图标区分（脚本 `symbol-file`、场景 `symbol-reference`）；**点击 jump 指向目标文件 uri**（`godotlsp.goto` 已支持任意 uri 的 showTextDocument+reveal，基建现成）；同节点多绑定（script + texture）时子节点展开各资源条目 |
| **A3** | **外部资源分组（根级，按类去重）** | 场景树根级新增「外部资源」分组：按 kind 分类——脚本 Script / 场景 PackedScene / 纹理 Texture2D / 音频 AudioStream / 字体 Font / 材质与其他。**kind 推断：ext.type 字段优先，path 扩展名兜底**（.png/.svg→纹理、.ogg/.wav/.mp3→音频）；**同 path 多 ext id 合并去重**（用户明确要求「不重复显示」）；点击行为分流：文本资源（.gd/.tscn/.tres/.cfg）goto 跳转，二进制（png/ogg 等）`vscode.open` 原生编辑器打开（图片有内置预览）。条目描述显示引用次数（`×2 节点引用`） |
| **A4** | **res:// 链接 Ctrl+点击**（DocumentLinkProvider，gd + tscn 双语言） | gd 里 `res://...` 字符串、tscn 里 `path="res://..."` 变可点击链接；res:// → workspace folder fsPath 映射；纯客户端零协议成本。官方插件同款，成本最低价值最高 |
| **A5** ✅ 已实施(2026-10-18,0.1.10) | **ExtResource hover 预览 + 使用侧行尾 inlay** | hover `ExtResource("1")`：显示资源 type/path（后续可扩：图片 hover 显示缩略图 data-URI img，Image preview 插件模式）；inlay hint：ext_resource 行尾 `← Texture2D · icon.png` 类注释可视化（官方插件「inlay hints visualize External/Sub Resources」同款）；两者共用 A1 的 ext 注册表 |
| **A6** ✅ 已实施(2026-10-18,0.1.10) | **脚本↔场景互跳命令**（Alt+O） | 打开 .gd → 扫描 workspace `*.tscn` 的 script 绑定反向索引（与 projectTree class_name 缓存同模式：保存时重建），多命中 quick pick；打开 .tscn → 跳根节点绑定脚本 |

### 3.5.2 B 系列（借鉴细节，后排）

| # | 项 | 来源 | 说明 |
|---|---|---|---|
| B1 | **拖拽树节点进脚本生成 NodePath** | 官方插件 Scene Preview 独有 | TreeItem dragAndDrop API（vscode 1.66+）；按上下文生成 `$Name` / `get_node("...")` / `%UniqueName`；成本中价值高 |
| B2 | **explorer file nesting** | VS Code 1.67 原生 | `configurationDefaults` 塞 `explorer.fileNesting.patterns`：`*.tscn` 嵌套同名 `.gd`。声明式零代码 |
| B3 | **rust-analyzer 式细节** | features.html | lens 分开关风格（已有 P2.10 基础）、hover actions（references 按钮）、join lines；GDScript postfix 补全（`node.onready` → `onready var`）视需求 |

### 3.5.3 明确不做（取舍记录）

- **active scene tree inspector**（运行时场景树）：DAP 域，归 P2-13，不混入静态视图。
- **tscn semantic tokens**：TextMate 语法已覆盖，语义着色收益低。
- **join lines / 结构化编辑 assist**：GDScript 场景收益低，暂缓。
- **资源缩略图 hover**：并入 A5 后续增量（先文本预览，后图片 data-URI）。

## 四、V3 — 远期/条件触发（不承诺）

- **多 IDE 配置样例**（nvim/zed/helix）：服务器已天然兼容（无私有 method 纪律），
  样例配置文档成本低时随文档站出；**不做 IDE 专属插件**（既定方向）。
- **P2-6 GDShader 语言支持**：独立 language id + 语法，视 features-roadmap P2.6 进展。
- **P6 汉化**：设置 UI 与命令标题已全中文；文档站/README 的中英分版视需求。
- **Publish vsix 到 marketplace**：vsce package 已闭环（0.1.2→0.1.8 全流程实证），
  发布账号与渠道属运营决策。

## 五、工程纪律（沿用既有，汇总本文相关项）

- **协议纪律**：只用标准 LSP method，不自造私有 method（lsp.ts 头注）。
- **版本号纪律**：同版本号重打包 vsix 必须升号。
- **验证**：`npm run typecheck` + `npm run test`（node:test）→ vsce package → 装机冒烟。
- **server 二进制**：不入 git，随 vsix 分发；`resolveServer()` 三级回退
  （配置 → 打包 `server/` → PATH）。
