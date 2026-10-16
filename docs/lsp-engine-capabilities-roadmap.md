# godotlsp 路线图（2026-10-16 重排）

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

## P2 — 资源文件与工程理解

7. [x] **服务器侧 tscn/tres 解析器**（6d3e721）：`gdscript-project/tscn.rs` 行级状态机解析 [gd_scene/gd_resource/ext_resource/sub_resource/node/connection]，宽容容错；Project 扫 .tscn/.tres 建 scenes + uid_paths 索引；scan_uid_files 扩头部自嵌 uid（1KB 快扫）；documentLink uid:// 自指跳过（路径等价比较，盘符大小写无关）、tscn 内 res:// 与 .gd 内 uid:// 双向可点；uidscene 夹具 + smoke_uid.py 端到端冒烟。
8. **rename 跨文件**：符号表驱动，重命名 var/func/class 声明与所有引用点。
9. **documentColor**：Color(...) 字面量行内色块装饰。
10. **信号连接 CodeLens**：tscn [connection] 段 → 脚本里 connect() 行上方显示连接数。

## P3 — 诊断分级与格式化

11. **warning 分级**：WarningSettings per-code 配置 + diagnostics.ignore 双通道。
12. **格式化**：内建 formatter，对齐 gdformat。**前置：CST trivia**（保留空白/注释 token，否则格式化丢注释）。
13. **codeAction**：未声明变量 → preload/class_name 导入建议；未解析节点路径 → 提示补 $/%。

## P4 — 高级导航与着色

14. **semanticTokens**：类型/函数/变量/字符串语义着色。**前置：CST trivia**。
15. **callHierarchy / typeHierarchy**：函数调用点集合 + 继承链导航。
16. **真正块级作用域查询**：semantic 层 locals 栈暴露 locals_at(offset)，LSP 层不再粗扫文本。

## P5 — 架构升级（大仓才需要）

17. **World/Snapshot 正式划分**：Arc 克隆是过渡方案；大仓（>1000 文件）需增量索引 + 关闭文件 AST 驱逐。
18. **UTF-8 位置编码协商**：客户端声明 utf-8 时跳过 UTF-16 换算。
19. **$/progress 上报**：大仓冷启动加载进度。
20. **自适应诊断去抖**：按解析耗时动态调整 50-500ms。

## 永久不做（已决策）

- salsa 增量数据库（cancel_others 死锁）
- 内嵌 Godot 引擎运行时（与脱离引擎定位矛盾）
- dagre 图布局（子→父边 bug）
- GDShader 语义类型检查（竞品共同坟场）
- Blender 材质节点转 gdshader（glTF 不存活）
- create_id_for_path 确定性 uid 复刻（无硬需求）
- clobber 配置文件生成

## 关键路径

```
P1.1 中文描述 bin ──→ P1.2-6 全局面补全
                         │
P2.7 tscn 解析 ────────┤
                         │
P3.12 格式化 ──→ P3.11 诊断分级
                         │
P4.14 semanticTokens ──→ P4.15 callHierarchy
                         │
P5.17 World/Snapshot ←──┘（大仓触发）
```
