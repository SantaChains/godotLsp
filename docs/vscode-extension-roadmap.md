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
