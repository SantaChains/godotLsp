# godotlsp 服务器设计参照与路线图

以一线 LSP 的公开设计为参照系,记录 godotlsp 的取舍依据与演进路径。每项结论标注来源项目;条目分「已落地」与「路线图」两档,路线图按 P0(稳定)/ P1(性能与体验)/ P2(规模)排序。

## 一、参照系:七个项目的核心设计

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

### gopls(Go 官方)

- **快照层次**:Cache(进程级)→ Session(每连接,持 overlay 未保存文本)→ View(工作区)→ Snapshot(不可变);didChange 应用增量后 `snapshot.clone()` 结构共享,未失效的包图与 type-check 结果按 PackageID memoize 复用,只重算脏包;旧快照引用计数回收。
- **keyed single-flight**:`memoize.Promise` 按 key 去重并发计算,同 key 只算一次。
- **诊断分级**:打开文件语法/类型 pass 几十 ms 内更新,全 workspace idle ~1s 重算;publish 带 `version`,每次修改先重发旧诊断防闪烁;单调 modification ID + `cancelPrevDiagnostics` 取消上一轮诊断。
- **缓存分层**:overlay → parseCache(LRU + 内容 hash 校验 + 时间过期)→ type-check memoize → metadata 包图;磁盘 filecache 按输入 hash 持久化,重启秒级恢复;CPU 按 GOMAXPROCS 限流。

### ZLS(Zig 官方)

- **双轨调度**:`isBlockingMessage`(initialize/shutdown 与全部 `textDocument/*` 请求)等积压 job 清空后**同步执行**,其余消息并发(`std.Io.Group`);每消息 `ArenaAllocator`。
- **惰性失效**:分析产物挂 Handle、`Lazy(T)` 缓存;文档变更只把 `document_scope`/trigram 置 `.unset`,访问时重建,**无级联失效广播**;imports 依赖图 + `Future` 合并并发加载。
- **诊断集合独立管理**:`DiagnosticsCollection` 按代数过期,后台任务完成前检查 `running_but_already_invalidated` 丢弃过期结果。
- **工程结构**:`features/` 每能力一文件(2025-09 重构);协议分帧/类型层独立为 `lsp-kit` crate;InternPool 统一类型 + MultiArrayList(SoA)。

## 二、godotlsp 现状基线

- **协议层已异步化**(0eda519,`lsp-server-tokio` 0.3):tokio 异步 stdio + `route()` 显式路由 + 每请求 `CancellationToken`(尚未接入计算);请求仍在主循环内同步处理——并发化是 P2 主题;salsa 增量分析(`gdscript-ide`)。
- **纯脱离 Godot 客户端运行**(已验证):分析内核源码无进程派生、无网络依赖;引擎模型与 API 文档内嵌于 `gdscript-api`(`engine_docs.bin`),场景感知靠直接解析 `.tscn`/`.tres` 文本。
- 增量文本同步(UTF-16 位置,夹紧容错)、文件监视兜底 `didOpen` 前的项目感知。
- 配置:推送(`didChangeConfiguration`)+ 拉取(`workspace/configuration`)双通道。
- 诊断带 `source`/结构化 `code`/`version`;请求级 panic 隔离;`lifecycle_over_duplex` 内存协议测试锁定生命周期语义。
- 已知边界:通知路径无 panic 防护(状态变更中途捕获会留下半更新状态,宁可崩溃重启);无取消传播(token 已挂载未接入计算)、无进度、无后台线程。

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

## 六、参照系深挖:并发 · 取消 · 诊断 · IO(P2-P3 实施依据)

> 本节为 §四 路线图中 P0-2(取消)、P1-5(线程模型)、P1-8(诊断节流)、P2-13(内存分层)的实施依据。全部结论经各项目 master 源码逐段核验(2026-10),标注来源文件。

### 6.1 快照与状态划分(写者独占可变,读者拿不可变快照)

- **rust-analyzer**:`GlobalState`(全可变:sender、req_queue、analysis_host、mem_docs、OpQueue)vs `GlobalStateSnapshot`(仅 Arc clone,廉价;global_state.rs:214)。快照先于 spawn 生成,重跑天然用新状态。
- **gopls**:didChange 后 `snapshot.clone()` 结构共享,未失效的包图/type-check 结果按 PackageID memoize 复用,只重算脏包;旧快照引用计数回收(snapshot.go)。
- **pyrefly**:事务二选一(TransactionManager,transaction_manager.rs:30-74)——后台 recheck 运行时,IDE 请求拿「非可提交事务」(overlay + save/restore)先答,不阻塞等待重检完成。
- **ZLS**:产物挂 Handle、`Lazy(T)`(mutex + ?T);变更只把 `document_scope`/trigram 置 `.unset`,访问时惰性重建,无级联失效广播(DocumentStore.zig `Handle.refresh`)。
- **godotlsp 对应**:World 持注册表与待发消息通道(可变),Snapshot = `Arc` clone(`gdscript-ide` 的 `Analysis` 句柄本已廉价);handler 全部改为只读快照后,通知路径的崩溃语义(P0-1)随之重评。

### 6.2 主循环与任务队列

- **rust-analyzer**:单线程 `Event` 枚举 + crossbeam `select!` 复用 9 个事件源(main_loop.rs:283);`TaskPool` + `ThreadIntent` 区分 Worker / LatencySensitive(补全、semanticTokens)/ Format(独立单线程池);每类事件处理后 `while elapsed < 50ms { try_recv() }` 批量合并积压。
- **pyrefly**:双通道 crossbeam + biased select——priority 队列(RecheckFinished / CancelRequest)> normal(Mutation / Query);`has_subsequent_mutation` 直接丢弃被后续编辑作废的查询(event_loop.rs:36)。
- **ruff**:`Task::Sync`(主线程可变)vs `Task::Background`(主线程构建闭包并拍只读快照,再移交后台,schedule/task.rs);后台三级 LatencySensitive / Worker / Fmt,配 OS 线程优先级。
- **ZLS**:阻塞消息等积压 job 清空后同步执行,其余并发;每消息 ArenaAllocator(Server.zig:1428)。
- **clangd**:每文件双线程——ASTWorker 独占队列 + PreambleThread;全局 `Semaphore` 限并发解析;已取消读请求移队首快速失败,「死更新」(后跟另一更新且无人等其诊断)直接丢弃(`shouldSkipHeadLocked`)。

### 6.3 取消(协作式,绝不杀线程)

- **rust-analyzer 三态响应**:`$/cancelRequest` 先查队列(未开始 → RequestCancelled);在途请求在 salsa 查询边界 `Cancelled::throw`,catch_unwind 后:可重试 → `Task::Retry`(快照已新);非延迟敏感 → ContentModified(-32801);诊断类 → ServerCancelled(-32802)+ retrigger(dispatch.rs)。
- **clangd**:Context 携带 CancellationFlag,调度点 + 任务内协作检查;官方注释明示「cooperative、racy、必须容忍取消后仍返回结果」(support/Cancellation.h)。
- **ruff**:`Arc<AtomicBool>` 每请求 token,响应发送阶段静默丢弃(request_queue.rs:141)——零运行时依赖,与单写者模型零冲突。
- **gopls**:连接断开自动取消全部在途请求;`$/cancelRequest` 经 Preempter 在入队前拦截。
- **godotlsp 对应**:lsp-server-tokio 的 `route()` 已为每请求发 `CancellationToken` 且 `$/cancelRequest` 自动取消;P2 只需在 salsa 查询边界轮询 token + 回三态错误码,复用现有 catch_unwind。

### 6.4 诊断管线(最终一致性)

- **分级**:gopls 打开文件语法/类型 pass 几十 ms 内,全 workspace idle ~1s(`diagnosticsDelay`);快速 pass 每按键即发。
- **自适应去抖**:clangd `DebouncePolicy{Min:50ms, Max:500ms, RebuildRatio:1}`——取最近 15 次重建耗时**中位数 × ratio** clamp 到区间(TUScheduler.cpp:1835);解析越慢去抖越长。
- **过期抑制**:RA `DiagnosticCollection` 分代 + `clear_check_older_than(generation)`;ZLS 完成前检查失效位;gopls 单调 modification ID;pyrefly 编辑作废旧 query。
- **防闪烁**:gopls 每次修改先立即重发旧诊断再算新的;publishedHash 去重。
- **退避**:basedpyright 用户交互 250ms backoff + 配置重载 100ms debounce(service.ts:630)。
- **godotlsp 对应**:publishDiagnostics 已带 version;P2 后加:快速语法 pass(内核容错解析产物)+ 全量 pass idle 延迟 + 代际清理;去抖窗口用 GDScript 解析耗时自适应。

### 6.5 IO 取舍(为什么主流 LSP 大多不用 async runtime)

- **rust-analyzer / clangd / ruff / ZLS 全部同步顺序读 stdio**(阻塞读线程);RA 社区结论:单管道顺序 IO 无可复用流,异步只增运行时开销;ruff 明确拒绝 async/tower-lsp(乱序风险,PR #10158)。
- **gopls** 异步分发(每请求 goroutine)+ Preempter;**godotlsp** 已在 P1 选定 tokio——补偿措施:P3 重点不在「更多异步」,而在**背压**(出站有界通道)、大读缓冲、UTF-8 多字节边界;请求处理保持「循环内同步 / 快照后台」两轨,不把 handler 全异步化(吸取 tower-lsp 通知乱序教训)。

### 6.6 内存分层

- **gopls** parseCache:LRU + 内容 hash 校验 + 时间过期,保底 100 文件;磁盘 filecache 按 hash 持久化,重启秒级恢复。
- **pyrefly**:关闭文件生成 interface 后驱逐完整 AST,内存与打开文件数而非项目规模挂钩;进程内 Arc 共享,无 LRU。
- **ZLS**:InternPool 统一类型 + MultiArrayList(SoA)+ 每消息 arena;TrigramStore 惰性建。
- **godotlsp 对应**:扩展 P2-13——salsa interface 保留、完整 AST 驱逐(P2);磁盘缓存可后置(P2 末期评估)。

### 6.7 P2 落地五步(综合七家,按依赖序)

1. **World/Snapshot 划分**:World 可变(注册表 + 待发消息通道),Snapshot = Arc clone;handler 全改只读快照——参照 RA `global_state.rs:214` 字段划分。
2. **Task 回传 + 事件合并**:worker 结果枚举(Task::Response / Diagnostics / Retry)经通道回主循环;每类事件 50ms 批量合并——参照 RA `main_loop.rs`。
3. **取消三态**:salsa 查询边界轮询 token → Cancelled 时 catch_unwind → Retry / ContentModified / ServerCancelled+retrigger——参照 RA `dispatch.rs`;token 已由 route() 就位。
4. **双通道优先级 + 编辑作废**:priority(诊断完成/取消)> normal(变更/查询);后续编辑使在途查询作废——参照 pyrefly `queue.rs`。
5. **诊断节流 + 代际**:自适应去抖(解析耗时中位数 × ratio,clamp 50-500ms)+ 分级 pass + 代际清理——参照 clangd + gopls。
