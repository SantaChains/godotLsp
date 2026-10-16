# godotlsp 路线图（2026-10-16 重排，晚补 P3 补全段）

> 按"用户体验收益 / 前置依赖 / 风险"三维排序。已完成项标注 [x]。

## P0 — 核心闭环已完成

- [x] LSP 握手 / stdio / 增量文档同步
- [x] 引擎模型（1036 类 + 38 builtin + 114 utility + 41 singletons，内嵌 extension_api.json）
- [x] 项目符号图（class_name / autoload / preload 跨文件解析）
- [x] 语义分析（符号表 / 名称解析 / 类型推断 / 未声明标识符诊断）
- [x] hover（引擎类/内建/全局函数/单例/常量 → 签名 + 文档站链接）
- [x] completion（全局面 + 接收者成员面）
- [x] signatureHelp（括号配平 → 参数列表 + 当前参数索引）
- [x] gotoDefinition（项目 class_name → 脚本文件；当前文件声明；引擎类 → 文档站 URL）
- [x] findReferences（词边界全局搜索）
- [x] workspaceSymbol（class_name 搜索）
- [x] documentLink（.gd 里 res:// + uid:// 可点）
- [x] documentHighlight / inlayHint / foldingRange
- [x] 后台诊断线程（150ms 去抖，Arc 共享 engine/project）
- [x] 诊断 version 回传 / 取消三态（RequestCancelled）
- [x] 五级摄取梯度（workspace .godot/extension_api.json → embedded）
- [x] 版本对齐日志 / UID 映射扫描

## P1 — 文档正文与全局面补全 ✅（2026-10-16 全部落地）

1. [x] **hover 接描述 bin**：`hoverdata.json`（21613 成员 + 16 内建函数 + 36 注解，`include_str!` 内嵌）+ `hoverdata.rs` OnceLock。签名在上、描述居中、文档站链接在尾（三段式 §D）。中文 P0 已入 75 条（50 类 brief + 高频成员 + 内建函数 + 注解），长尾 P2 续推——中文化在 `gen_docs.py` 提取层切换，网站/hover 双端同批。
2. [x] **编译器内建函数**：len/preload/range/print… 16 个走 hoverdata.f（签名+描述），hover 与 completion 双面。
3. [x] **内建常量**：PI/TAU/INF/NAN 走 @GDScript 常量兜底，双面。
4. [x] **注解**：36 个，hover（@ 前缀识别）+ completion（`@` 触发独立注解面，不混全局面）。
5. [x] **关键字**：34 个（langdocs.rs，链接语法篇 `#kw-*` 锚），hover + completion。
6. [x] **枚举值**：hover 十级链（全局/类/内建枚举值 + 静态类名回退 + 继承链锚点宿主）；completion 全局面展开全局枚举值、成员面出类/内建枚举名与值 + 内建常量（Vector2.ZERO）。

附带修复：成员描述与链接沿继承链指向声明宿主页（Sprite2D.position → Node2D 页）；类补全 take(80) HashMap 随机截断改全量稳定排序；recv_type_at 静态回退 + `.` 后切片 panic。

验收待量化：corpus 标识符 hover 命中率抽验、节点路径上下文补全抑制回归（R4 重写后重验）。

验收：corpus 内任一标识符 hover 命中率 ≥95%；补全不出现全局面污染节点路径上下文。

## P2 — 资源文件与工程理解 ✅（2026-10-16 全部落地）

7. [x] **服务器侧 tscn/tres 解析器**（6d3e721）：`gdscript-project/tscn.rs` 行级状态机解析 [gd_scene/gd_resource/ext_resource/sub_resource/node/connection]，宽容容错；Project 扫 .tscn/.tres 建 scenes + uid_paths 索引；scan_uid_files 扩头部自嵌 uid（1KB 快扫）；documentLink uid:// 自指跳过（路径等价比较，盘符大小写无关）、tscn 内 res:// 与 .gd 内 uid:// 双向可点；uidscene 夹具 + smoke_uid.py 端到端冒烟。
8. [x] **rename 跨文件**（4cbdaee）：词法级与 references 同源（collect_spans 共用实现）;prepareRename 保留名闸门（引擎类/单例/内建函数/关键字/注解/@GDScript 函数/全局常量枚举不触发 UI）;新名校验 + 拒绝回 -32602;只改 .gd（tscn 引用靠 path）。已知限制:同名异作用域符号一并改写（语义化作用域收窄属后续打磨）。
9. [x] **documentColor**（e3088b3）：词法扫描 Color(r,g,b[,a]) 全数字字面量 + Color.NAME 命名常量（引擎模型 40 色表 OnceLock）；colorPresentation 回写构造文本。变量/表达式不猜（零误报）。
10. [x] **信号连接 CodeLens**（e3088b3）：tscn [connection]（仅 complete）引用本脚本 → func 声明行 lens "n 个场景连接" → godotlsp.openConnection 命令跳 tscn 连接行（插件已注册）。

## P3 — 补全排序与 snippet（2026-10-16 立项，依据 lsp-design.md §6.8）

> 四个用户诉求映射：常用函数 snippet → C2；文件已输入词条 → C3；频率排序表 → C4；算法/预测 → C1+C5。参照系：rust-analyzer CompletionRelevance（relevance 与 fuzzy 严格分离，`score ^ 0xFFFF_FFFF` 十六进制零填充 sortText）+ clangd 双维分（Quality 符号质量 × Relevance 查询相关，ContextWords/MainFileRefs，扩展字段 `score` 供客户端二次加权）。

11. [x] **C1 sortText 骨架**（6aed282）：四补全面统一评分——fuzzy 七带（完全相等/CI 相等/前缀越短越优/CI 前缀/子序列空隙密度/空前缀中性/不匹配）× 分桶 relevance（LOCAL 预留 C3 / MEMBER / FN_GLOBAL / TYPE / CONST / KEYWORD）；sortText = `{bucket:02x}{(0xFFFF-fuzzy):04x}` 取反编码。附带修复:partial 词归面（`Vector3.no` → 成员面、`@ex` → 注解面,旧逻辑只有紧邻点/裸 `@` 才归）。契约不变:仍全量返回,客户端过滤照旧。桶内频率细分留 C4。
12. [x] **C2 snippet 面**（47a789b）：`snippetSupport` 协商（不声明则整面不出现，不做纯文本降级）。三类模板：① `func _` 上下文出虚方法模板（12 个，按宿主继承链过滤——Node 系不出 _draw、CanvasItem 出 _draw 不出 _gui_input；无 extends 默认 RefCounted 只出 Object 级）；② 语句位复合模板（instantiate_scene/connect_lambda/create_tween/await_timer，仅词面命中项，空前缀不泛滥）；③ 控制流关键字（for/while/if/match）label 不变升级 snippet 插入。模板正文禁裸 `$`（LSP 占位符转义会炸 GDScript 节点路径）。已知限制:宿主为项目 class_name 时只解析一跳 extends,更深项目继承链留 P5 语义层。
13. **C3 本文件词条面 ✅**：词级扫描本文件标识符（出现 ≥2 次的词 + 当前作用域 locals 粗扫），供 nvim/zed/helix（VSCode 自带 wordBasedSuggestions，靠低 sortText 桶 + `godotlsp.completion.localWords` 开关避免重复感）。落地（6860ee2）：locals（var/const/for/参数，空前缀也出，LOCAL 桶）+ 高频词（≥2 次，有前缀才出 + fuzzy>0 过滤）；保留名（引擎类/内建/关键字/注解/@GDScript 函数）排除——已由专门面提供，避免同名重复项；光标半截词不计频次。**配置开关挂起**：当前配置通道为空壳（didChangeConfiguration 接收即忽略），localWords 开关待配置系统立项后补。
14. **C4 频率排序表**：`tools/gen_freq.py` 构建期挖掘——godot-docs 全部代码块 + corpus（feature-showcase + godot-engine-tests pinned）提取标识符频率 → `completion_freq.json`（include_str! 内嵌，OnceLock）；C1 桶内按频率细分。表再生成纳入 gen_docs --check 常设校验。官方无 Godot API 频率数据集，语料挖掘是可复现替代（fetch_corpus.py 同模式）。
15. **C5 类型感知评分**（前置 P5.18 locals_at/类型推断）：rust-analyzer 式 relevance 字段——`exact_name_match`（调用点参数名命中）、`type_match`（赋值目标类型对齐）、作用域深度；clangd 式扩展字段 `score`（纯质量分）随 item 下发，供客户端 fuzzy 重排。**明确不做**：ML/LLM/ghost-text inline completion（零依赖 + 隐私定位冲突，见"永久不做"）。

## P4 — 诊断分级与格式化（原 P3）

16. **warning 分级**：WarningSettings per-code 配置 + diagnostics.ignore 双通道。
17. **格式化**：内建 formatter，对齐 gdformat。**前置：CST trivia**（保留空白/注释 token，否则格式化丢注释）。
18. **codeAction**：未声明变量 → preload/class_name 导入建议；未解析节点路径 → 提示补 $/%。

## P5 — 高级导航与着色（原 P4）

19. **semanticTokens**：类型/函数/变量/字符串语义着色。**前置：CST trivia**。
20. **callHierarchy / typeHierarchy**：函数调用点集合 + 继承链导航。
21. **真正块级作用域查询**：semantic 层 locals 栈暴露 locals_at(offset)，LSP 层不再粗扫文本。——同时是 P3.15 C5 的前置。

## P6 — 架构升级（大仓才需要，原 P5）

22. **World/Snapshot 正式划分**：Arc 克隆是过渡方案；大仓（>1000 文件）需增量索引 + 关闭文件 AST 驱逐。
23. **UTF-8 位置编码协商**：客户端声明 utf-8 时跳过 UTF-16 换算。
24. **$/progress 上报**：大仓冷启动加载进度。
25. **自适应诊断去抖**：按解析耗时动态调整 50-500ms。

## 永久不做（已决策）

- salsa 增量数据库（cancel_others 死锁）
- 内嵌 Godot 引擎运行时（与脱离引擎定位矛盾）
- dagre 图布局（子→父边 bug）
- GDShader 语义类型检查（竞品共同坟场）
- Blender 材质节点转 gdshader（glTF 不存活）
- create_id_for_path 确定性 uid 复刻（无硬需求）
- clobber 配置文件生成
- **ML/LLM/ghost-text inline completion**（零依赖 + 隐私定位冲突；Copilot 类职责不属 LSP，2026-10-16 决策）
- **自建全局语言模型统计**（无官方频率数据集，语料挖掘已覆盖 C4 需求，2026-10-16 决策）

## 关键路径

```
P1.1 中文描述 bin ──→ P1.2-6 全局面补全 ──→ P3.11 sortText 骨架
                                              │
P3.14 频率表 ────────────────────────────────┤
                                              │
P3.12 snippet 面（snippetSupport 协商）←──────┘
                                              │
P5.21 locals_at ──→ P3.15 类型感知评分 ←──────┘
        │
P4.17 格式化（CST trivia）──→ P5.19 semanticTokens
                                              │
P6.22 World/Snapshot ←───────────────────────┘（大仓触发）
```
