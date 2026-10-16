# VS Code 插件功能衔接规划

> 定位:回答「插件每个功能连到哪里、靠谁供数、何时能动」。rs 侧(`gdscript-features-roadmap.md`)已立项的编辑器条目,本文不重复规格,只做**衔接映射与执行排序**。
> 背景:rs 侧测试暂时冻结(用户指示),本文所有「立即队列」条目均零 rs 代码改动。

## 一、现状三面盘点

### rs 服务器已声明的能力面(R4 最小真实版,main.rs 470 行)

| 能力 | 状态 | 插件侧衔接现状 |
|---|---|---|
| hover(尾部链接→自建文档站,W2 起) | ✅ | languageclient 自动渲染,零插件代码 |
| completion / signatureHelp / definition / references / documentHighlight | ✅ | 自动接线 |
| documentSymbol / workspace/symbol / foldingRange | ✅ | 符号树消费 documentSymbol;其余自动 |
| **documentLink(res:// 与 uid:// 字面量)** | ✅ | **仅对 gdscript 生效**(documentSelector 只声明 gdscript)——tscn/tres/project.godot 中的资源链接实际收不到 |
| inlayHint | ✅ | 已接线,`godotlsp.inlayHints.typeHints` 双通道 |
| publishDiagnostics + didChangeWatchedFiles | ✅ | 自动 |

### 插件 0.1.3 已实现

三树视图(符号/项目结构/场景结构)共用 `godotlsp.goto`;运行项目/场景(含 CWE-77 注入防护);configAssist/configFile(project.godot/gdextension/.import 的补全与 hover,数据自 `tools/extract_project_settings.py`,源码核验);四语言 TextMate;16 组 snippet;parsers.ts(autoload/scene 纯解析,node:test 5 用例);highlightClampMiddleware。

### 与文档站的契约(W2 定稿)

`BASE = santachains.github.io/godotlsp/`;类页 `class_<小写去@>.html`;成员锚 `#<原名>`/`#enum-<名>`/`#<值名>`;`@GDScript → class_at_gdscript`。rs 侧 `doclinks.rs` 有 ~2 万锚点遍历测试锁定;**TS 侧无任何镜像**。

## 二、衔接映射总表

| VS 功能(roadmap 编号) | 数据来源 | 依赖侧 | 状态 |
|---|---|---|---|
| 三树视图(12/18) | LSP documentSymbol + parsers.ts | 双侧已在位 | ✅ |
| 运行项目/场景(6) | `godotExecutable` 配置 | 纯插件 | ✅ |
| project.godot/gdextension/.import 助手 | project-settings.json | 纯插件 | ✅ |
| 信号连接视图(P1-10) | parsers.ts 扩展(connection 段) | 纯插件 | ✅ 插件侧 tscn 连接已实现(2026-10-16);gd 侧 connect() 扫描待增量 |
| 大纲增强包(P2.5-21a~f) | parsers.ts + LSP diagnostics/references | 纯插件 | ✅ 21b/21c/21d 已实现(2026-10-16,三视图共用);21a/21e/21f 待做 |
| 资源图谱 E1/E1-G(P2-19) | parsers.ts 泛化 + webview | 纯插件 | 待做 |
| gdshader S1(P2.6) | TextMate + 正则 | 纯插件 | 待做 |
| gdformat 桥(P1-8) | 外部命令 + 配置开关 | 纯插件 | 待做 |
| tscn/tres documentLink(P2-17) | rs document_links | **rs 扩容+分流验证** | 🧊 冻结 |
| uid→path inlay 替换/hover 资源预览(P2-17) | uid 映射 | 纯插件可先行 | 待做 |
| 中文文档集成(P2-16) | 中文语料构建期提取 | **rs(hover 数据)** | 🧊 冻结 |
| 图谱 E2 `godotlsp/graph`(P2-19) | 自定义 method | **rs(R2 后)** | 🧊 冻结 |
| DAP(P2-13) | 独立适配器 | 独立轨道 | 远期 |

## 三、衔接决策

**D1 · documentLink 覆盖面(🧊 rs 解冻后第一批)**
服务器 `document_links` 是文本级 `res://`/`uid://` 字面量扫描,对 tscn/tres 天然可工作,但 `documentSelector` 只含 gdscript——扩容不是一行改动那么简单:tscn 进入 selector 后 didChange/诊断链路也会激活,而服务器诊断器按 GDScript 解析,会向 .tscn 发垃圾诊断。**正确路径:rs 侧先做按文件类型分流(诊断只对 .gd,link/folding 对资源文件开放),再扩 documentSelector 三语言**。属 rs 改动,冻结期不做;插件侧唯一可先行的是把三类资源文件的 `res://` 可点击性用插件侧 documentLinkProvider 补上(与 21f-③ 同管线)——但那是重复实现,等分流一次到位更好。**决策:整体冻结,分流方案已记入 rs 队列。**

**D2 · 文档站命令 `godotlsp.openDocs`(✅ 立即,轻量)**
W2 后 hover 链接已指向自建站,但只有 hover 一条路。加命令:光标处符号(类/成员)→ 按 doclinks 契约拼 URL → `vscode.open`。TS 镜像 slug 逻辑 ~40 行;**漂移风险对策:node:test 复用插件测试基建,用同一份 `extension_api.json` 跑与 rs `doclinks.rs` 同构的断言(slug 消歧/成员锚形态),契约双侧互锁**。hover 链接是兜底,命令是快捷路径(Ctrl+Q 式),并给键盘流补齐。

**D3 · 大纲增强包 21b/21c/21d 先行(✅ 已实现 2026-10-16)**
落地形态:21b 过滤 = 纯逻辑 `treefilter.ts`(node:test 直测)+ `PTreeDataProvider` 层统一套用(项目/场景/连接三视图)+ symbols 自带;过滤词显示在视图描述行,保留命中后代祖先链。21c 跟随 = 选择事件防抖 250ms,`locate()` 最深区间命中 → `reveal`(尽力而为,过滤隐藏/折叠竞态静默放弃);symbols/场景/连接三视图生效。21d 旗标 = `diagSuffix` 区间求交计数,severity 并入 description(`✗n ⚠n`)、消息进 tooltip,类别图标保留;诊断事件驱动刷新。21a region / 21e TODO / 21f 图标主题待做,全部纯插件,不碰 rs。

**D4 · 信号连接视图(P1-10,✅ 立即,最小闭环)**
parsers.ts `parseScene` 已解析节点树,`[connection]` 段解析是 ~15 行扩展(engine-analyzer.md S 节四字段规格已有);新 TreeView「信号连接」列出 from/signal/to/method,点击双向跳转(tscn 行 / gd 方法行)。这是官方插件 gap 清单里**成本最低**的差异化项。

**D5 · E1 资源图谱(✅ 立即可启动,大件)**
webview 基建 + parsers 泛化边抽取,规格在 roadmap P2-19 已定死(React Flow + ELK vendored、两级物化)。工作量大,排在 D3/D4 之后作为插件侧主战役;E2 的 `godotlsp/graph` 是 rs 冻结项,但 E1 的纯插件形态(文本边)本就是决策 C 方案。

**D6 · P2.6 S1 gdshader 插件侧(✅ 立即,独立小战役)**
TextMate 语法/文件图标/documentColor/snippet/`#include` documentLink——五件全部零 rs 依赖,且与 Blender 交集(用户 X3 收敛)直接相关。

**D7 · gdformat 桥(P1-8,✅ 立即)**
`godotlsp.format.engine` 开关 + 外部命令执行;注入防护复用 runGodot 的引述策略;检测 gdformat 不存在时明确指引(条目 14 无 Godot 完备运行保证的姊妹纪律)。注意:服务器内建格式化器到位后此开关语义变为 fallback,**配置键现在就定名,避免未来 breaking**。

## 四、执行序

**VA(rs 冻结期,全纯插件)**:~~D4 信号连接视图~~ ✅ → ~~D3 大纲增强包 21b/c/d~~ ✅(余 21a/21e/21f 可随 D2 前穿插)→ **D2 openDocs 命令(下一项)** → D7 gdformat 桥 → D6 gdshader S1 → D5 E1 图谱。

**VB(rs 解冻后)**:① documentSelector 分流扩容(D1)→ ② P2-16 中文文档集成 → ③ E2 `godotlsp/graph` → ④ P1-15 documentHighlight/hover 写入处探针验收(R2 探针)。

**冻结项带**:W3 收尾时 cargo test 出现 2 个编译错误,按指示挂起未排查——rs 解冻时**先清这笔账**再进 VB。

## 五、纪律(沿用 roadmap §三,补两条)

- 插件不重写服务器已有的语言智能;新建衔接先查本文映射表确认没有重复实现(如插件侧 documentLinkProvider 与 D1 的关系)。
- TS 侧复刻 rs 契约(slug/锚点)必须带同构 node:test,数据源同一份,双侧互锁;契约变更时两处同改(生成器 gen_docs.py 是最终事实源)。
