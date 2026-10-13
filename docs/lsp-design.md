# godotlsp 服务器设计参照与路线图

以一线 LSP 的公开设计为参照系,记录 godotlsp 的取舍依据与演进路径。每项结论标注来源项目;条目分「已落地」与「路线图」两档,路线图按 P0(稳定)/ P1(性能与体验)/ P2(规模)排序。

## 一、参照系:五个项目的核心设计

### rust-analyzer

- **线程模型**:主循环只做消息分发,分析任务跑在工作线程;请求带取消令牌,`$/cancelRequest` 通过 salsa 的 cancellation 机制中断在途查询,被打断的请求以 `RequestCancelled` 回应。
- **panic 隔离**:每个请求在线程池中执行,单请求 panic 只回 `RequestFailed`,服务器不死。
- **位置编码**:协商优先 UTF-8(`positionEncoding`),省去 UTF-16 转换开销。
- **快照语义**:任何请求拿到的是某一修订版的不可变快照,处理期间编辑照常进入。

### clangd

- **后台索引**:AST 在请求线程,Preamble/Index 在后台线程池,读写分离。
- **进度上报**:`$/progress` 汇报索引进度(`window/workDoneProgress/create`),用户可见"正在建索引"。
- **诊断节流**:按文件去抖,避免每次击键全量重推。
- **协议严谨性**:对未知请求严格回 `MethodNotFound`,对未知通知静默;`version` 字段贯穿诊断。

### ruff(lsp 侧)

- **与 CLI 共享内核**:LSP 只是内核之上的一个前端,同一引擎服务 `ruff check` 与编辑器,行为零漂移。
- **server 朴素化**:`lsp-server`(与 godotlsp 同栈)+ 同步处理起家,先保证正确再谈并发;客户端能力全部按声明降级。
- **textDocument 同步**:增量同步 + `didSave` 显式兜底。

### basedpyright

- **配置双通道**:初始化后主动 `workspace/configuration` 拉取,之后 `didChangeConfiguration` 推送增量;两端汇合同一套 settings 解析。
- **能力协商**:按客户端声明裁剪能力面(nvim/VS Code/Zed 各得其所),不做隐性假设。
- **诊断来源标注**:每条诊断带 `source` 与结构化 `code`,供下游 fix 关联。

### pyrefly(Meta)

- **事务化快照**:`State` 持不可变快照,`Transaction` 提供可提交的可变视图;读事务并发、写事务至多一个,IDE 查询永不阻塞后台重检。
- **取消感知**:事务创建即携带取消令牌,后台重检完成时在途查询自然失效重算。
- **内存分层**:关闭文件的完整 AST/bindings 在生成 interface 后即驱逐,只保留接口层;完整数据只留打开的文件 —— 大仓内存与打开文件数而非项目规模挂钩。
- **IDE 优先**:按下击键后毫秒级更新为目标,而非"批检查器附带 LSP"。

## 二、godotlsp 现状基线

- 单线程 stdio 主循环(`lsp-server`),19 个请求同步处理;salsa 增量分析(`gdscript-ide`)。
- **纯脱离 Godot 客户端运行**(已验证):分析内核源码无进程派生、无网络依赖;引擎模型与 API 文档内嵌于 `gdscript-api`(`engine_docs.bin`),场景感知靠直接解析 `.tscn`/`.tres` 文本。
- 增量文本同步(UTF-16 位置,夹紧容错)、文件监视兜底 `didOpen` 前的项目感知。
- 配置:推送(`didChangeConfiguration`)+ 拉取(`workspace/configuration`)双通道。
- 诊断带 `source`/结构化 `code`/`version`;请求级 panic 隔离。
- 已知边界:通知路径无 panic 防护(状态变更中途捕获会留下半更新状态,宁可崩溃重启);无取消、无进度、无后台线程。

## 三、已落地(本轮)

| 项 | 参照 | 说明 |
|---|---|---|
| 请求级 panic 隔离 | rust-analyzer | `catch_unwind` 包裹全部请求处理,单请求崩溃回 `RequestFailed(-32803)`,进程存活 |
| 诊断版本回传 | clangd | `publishDiagnostics.version` = 编辑器文档版本,客户端可丢弃迟到诊断 |
| 配置拉取 | basedpyright | 声明 `workspace.configuration` 的客户端(zed/helix)初始化后主动拉一次,与推送同路径生效 |
| workspace/symbol 零冗余 | — | 去掉逐目标克隆全文 + 重建行索引;直接读注册表 |
| inject 路径去克隆 | — | `didOpen`/watched 注入不再二次克隆文档文本 |
| 插件汉化收口 | — | displayName、命令标题、untrustedWorkspaces 描述补全为中文 |

## 四、路线图

### P0 — 稳定与正确性

1. **通知路径的崩溃语义**(决策:保持崩溃)。`didChange` 中途 panic 会留下"文档已更新、分析世界未同步"的半状态,捕获后继续服务比重启更危险。待 §P1-1 的快照化完成后重新评估。
2. **`$ /cancelRequest` 语义化**。当前同步处理下取消天然无效(请求处理完才读下一条消息);引入后台线程后必须接上:salsa cancellation + `RequestCancelled(-32802)`。
3. **畸形输入 fuzz**。`document_colors`/`document_links`/`apply_content_changes` 已有夹紧与容错,补 fuzz(targeted:`arbitrary` + libfuzzer)锁住越界类 panic。
4. **多根工作区**。`initialize` 只取第一个 folder;其余 folder 静默丢弃。至少要 `window/showMessage` 告知,长期按 clangd 方案做多根注册表。

### P1 — 通信与性能

5. **线程模型:主循环 + 请求线程**(参照 rust-analyzer)。`handle` 全部只读 `&State`,天然适合"主线程独占写、工作线程拿快照读"。这是取消、诊断节流、进度上报的共同前置。
6. **`$/progress` 上报**。项目加载(大仓冷启动可达秒级)应发 `window/workDoneProgress/create` + begin/end;需要先落 5 的线程模型以便加载移出初始化路径。
7. **位置编码协商**。客户端声明 `utf-8` 时直通字节偏移,跳过 `LineIndex` 的 UTF-16 换算(`position()`/`offset()` 是每个请求的热路径)。
8. **诊断节流**。同步模型下每击键全量诊断是串行延迟大头;线程模型后按 clangd 去抖 + 合并中间版本(配合已有的 `version` 字段)。
9. **语义 token `range` 支持**。声明了 `range: false`,编辑器大文件滚动时会退化为全量;分析器已可裁剪,补 range 查询。
10. **位置解析零拷贝**。`convert.rs` 的 `DocResolver` 每次调用克隆全文;改为返回 `(uri, &LineIndex)` 借用视图或请求级 memo,definition/references/rename 在大文件上收益明显。
11. **文件监视 IO 增强**。当前每个 `didChangeWatchedFiles` 事件都同步重读磁盘,`didOpen` 风暴(如恢复会话打开几十个文件)会串行放大:事件合并去抖 + 同 URI 去重。`read_to_string` 对非 UTF-8 文件直接丢弃,改有损解码(`from_utf8_lossy`)保注释高亮可用。
12. **异步部分结果**。`workspace/symbol` 大仓可返回数千条;LSP 的 `partialResultToken` 允许流式返回,先出先显。排在 5 之后。

### P2 — 规模与生态

13. **内存分层**(参照 pyrefly):关闭文件只留 salsa interface,完整 AST 驱逐;上限由打开文件数决定。
14. **resultId 增量诊断**(`textDocument/diagnostic` pull 模式)。
15. **动态注册/反注册** inlay hint 等能力,按配置生效面裁剪。
16. **watch 模式 CLI 子命令**(参照 ruff 共享内核):`godotlsp check` 服务 CI,与服务器共用分析内核,行为零漂移。
17. **离线中文文档集成**。语料:`godot-docs/`(1592 个 md,源自《GodotEngine.epub》中文翻译,译者孤辰辰)。方案:**构建期**转换管线(`gdd_*.md` → 按类名/方法名/信号索引的压缩 bin),运行时零依赖、零解析开销。分层策略:API 签名与参数文档沿用 `gdscript-api` 内嵌的英文 `engine_docs.bin`(权威、精确到方法);教程类长文(信号使用、节点生命周期等)走 godot-docs 检索,以"参见"链接或折叠段注入 hover。悬停文档保持中英分离可配置。
18. **Godot 引擎交互通道**(调试/运行)。定位判断:这是**正交能力,不进服务器核心**。LSP 只管语言智能;运行场景、断点调试属 DAP 域(参照官方 godot-vscode-plugin 的 GDScript debug adapter),引擎内建能力对接走编辑器扩展的可选命令。若做:独立子命令(`godotlsp dap`)或独立扩展,与 LSP 服务器进程分离,保持"纯 LSP、无编辑器特设"的多 IDE 准则不被污染。

## 五、多 IDE 兼容准则(服务器侧)

插件暂不设计,但服务器必须保持"纯 LSP 3.17、无编辑器特设":

- **能力全协商**:不假设客户端支持任何能力;配置走拉/推双通道,任何一端缺失都可用。
- **配置 schema 平坦**:只依赖 `didChangeConfiguration` 与 `workspace/configuration` 两种标准载荷,不自造初始化选项。
- **stdio 纪律**:日志只走 stderr,stdout 只走协议帧;`--version` 可独立调用(各编辑器启动脚本的事实标准)。
- **URI 归一化**:接受任意合法 `file:` URI(含百分号编码、Windows 盘符),内部统一转路径再回 URI,不依赖客户端大小写/转义习惯。
- **协议域隔离**:调试/运行等非语言智能能力不得混入 LSP 协议(DAP 归 DAP),服务器进程保持单一职责。
