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

- **协议层已异步化**(0eda519,`lsp-server-tokio` **0.4**,2026-10-16 自 0.3 迁移):tokio 异步 stdio + `route()` 显式路由 + 每请求 `CancellationToken`。0.4 迁移面实录:`Connection.receiver` 转私有(读侧改 `receiver_mut()`/`into_receiver()`,3 处调用点);`Response::ok/err`、`Connection<T, I=()>`、`ClientSender`、`request_queue.outgoing.register(-1)`、`IncomingMessage::CancelHandled` 均未变;`ErrorCode`/`LifecycleState` 转 `#[non_exhaustive]`,MSRV 1.91。34/34 测试(含 duplex e2e/smoke)全绿验证。
- **P2 并发模型已落地**(9dabec4):单写者 `State`(主循环独占)+ 线程池快照(请求解析后拍 `Snapshot`,分析计算 `spawn_blocking` 下沉,结果经双通道回传,响应优先于诊断);50ms 预算事件合并;取消三态(`RequestCancelled`/完成后取消静默丢弃/revision 过期 → `ContentModified`);诊断去抖调度器(自适应延迟 = 最近 15 次耗时中位数 clamp [50,500]ms)+ 代际编辑作废 + 取消重排。
- **salsa 0.27 写路径硬约束(源码核验)**:`zalsa_mut` → `cancel_others` 以 Condvar 等待 storage clone 计数归 1 —— 快照句柄存活期间写世界会死锁(官方注释明言单线程双句柄死锁)。因此采用「**写者等待在飞读者(有界)**」而非 RA 式中途取消:所有写前 `quiesce()` 等在飞任务归零;unbounded 任务通道消除任务端阻塞环;`MAX_INFLIGHT=4` 约束写等待上界;过期结果靠 revision 比对识别,内核 `Cancellable` 防御性传播。
- **纯脱离 Godot 客户端运行**(已验证):分析内核源码无进程派生、无网络依赖;引擎模型与 API 文档内嵌于 `gdscript-api`(`engine_docs.bin`),场景感知靠直接解析 `.tscn`/`.tres` 文本。
- 增量文本同步(UTF-16 位置,夹紧容错)、文件监视兜底 `didOpen` 前的项目感知。
- 配置:推送(`didChangeConfiguration`)+ 拉取(`workspace/configuration`)双通道,同批就绪消息中配置响应按序吸收(drain 内每消息前吸收)。
- 诊断带 `source`/结构化 `code`/`version`;请求级 panic 隔离;`lifecycle_over_duplex` 内存协议测试锁定生命周期语义。
- 已知边界:通知路径无 panic 防护(状态变更中途捕获会留下半更新状态,宁可崩溃重启);内核不区分语法/语义 pass,分级诊断(P2 第 5 步的分级部分)待内核暴露 fast-pass 后补;进度上报未做。

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
- **自适应去抖**:clangd `DebouncePolicy{Min:50ms, Max:500ms, RebuildRatio:1}`——取最近 15 次重建耗时**中位数 × ratio** clamp 到区间(clangd `TUScheduler.cpp` 第 1835 行——外部引用,本地无 clangd 仓不可验);解析越慢去抖越长。
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

### 6.8 补全排序与 snippet(2026-10-16 深挖,实施依据)

- **rust-analyzer(`ide-completion/src/item.rs` + `to_proto.rs`)**:相关性建模为 `CompletionRelevance` 13 布尔/枚举字段(`exact_name_match` 参数名精确命中、`type_match` 类型对齐、`is_local` 局部变量、`requires_import` 等),`score()` 合成 u32;**fuzzy 匹配分与 relevance 严格分离**("Relevance ignores fuzzy match score"),最终序 = relevance × fuzzy。sortText 编码:`score ^ 0xFFFF_FFFF` 后 `{score:08x}` 十六进制零填充(客户端升序,取反即降序);仅 relevance 最高者置 `preselect`。
- **clangd(`Quality.h` `SymbolRelevanceSignals` + extensions)**:双维分 `finalScore = fuzzy × (Quality × Relevance)`。Quality 是**与查询无关的符号质量**(引用数/usages);Relevance 是查询相关信号:NameMatch fuzzy、作用域邻近(FunctionScope>ClassScope>FileScope>GlobalScope)、文件邻近(URI 距离)、**ContextWords(补全点附近的词)**、`MainFileRefs`(本文件引用数)。sortText 内嵌总分,另发非标扩展字段 `score`(纯质量分、不含 fuzzy),供客户端自做 fuzzy 过滤时二次加权——**服务端分桶 + 客户端 fuzzy 的混合排序契约**。
- **VSCode `wordBasedSuggestions`**:本文件已输入词条做补全,是编辑器客户端行为(off/currentDocument/allDocuments)。服务器侧实现同面,对 nvim/zed/helix(默认无此能力)才有增量价值;VSCode 下需避免与客户端词条重复(低 sortText 桶 + 配置开关)。
- **snippet**:能力经 `completionItem.snippetSupport` 协商;rust-analyzer 模式补全(pattern snippet)与 clangd 函数括号 snippet 均按客户端声明降级为纯文本。上下文感知(如 `func _` 后才出虚方法模板)靠服务端判断,比编辑器静态 snippet 文件精准。
- **无现成 Godot API 频率数据集**:官方无遥测统计 API 使用频率;可行替代 = 构建期语料挖掘(godot-docs 代码块 + 上游测试语料,pinned commit 可复现),与既有 drift-table/fetch_corpus.py 模式同构。
- **godotlsp 落地序**(P3,详见 capabilities-roadmap):sortText 分桶骨架 → snippet 面 → 本文件词条面 → 频率表 → 类型感知评分(前置 locals_at/类型推断)。**不做**:ML/LLM/ghost-text inline completion(零依赖 + 隐私定位冲突,Copilot 类职责不属 LSP)、postfix 补全(GDScript 无点链宏生态)。

### 6.9 引擎内部补全数据源与官方 snippet 借鉴(2026-10-16 深挖,实施依据)

**A. 官方引擎已有完整 LSP——但它不在引擎外,也不可复用为我们的运行时**(源码锚点:godotengine/godot master,2026-10 核验):

- **协议层** `modules/gdscript/language_server/gdscript_language_protocol.cpp`:TCP JSON-RPC 服务,**跑在编辑器进程内**(`LSP_MAX_CLIENTS=8`、`LSP_MAX_BUFFER_SIZE=4MB`、Content-Length 分帧);端口 6005(Godot 4.x)/6008(3.x),开关在 EditorSettings `network/language_server/*`;Godot 3.6+/4.2+ 支持 `--headless --no-window --lsp-port <port>` 无头模式(官方 godot-vscode-plugin `ClientConnectionManager` 实证)。
- **补全语义层不在 LSP 模块**,而在编辑器共用引擎 `GDScriptEditorLanguage::complete_code`(`modules/gdscript/gdscript_editor.cpp:3554`):LSP 路径 = `GDScriptWorkspace::completion` → `parser.get_text_for_completion(position)` → complete_code,并把**场景缓存 owner 节点**(`language_server/scene_cache.cpp`,didOpen/didClose 时按依赖图加载 .tscn)作 `p_owner` 传入——节点路径补全因此是"真场景树"而非文本猜测。
- **18 个补全上下文**(`GDScriptParser::CompletionContext`):IDENTIFIER / METHOD / ATTRIBUTE / ATTRIBUTE_METHOD / SUBSCRIPT / TYPE_ATTRIBUTE / ASSIGN / CALL_ARGUMENTS / ANNOTATION / ANNOTATION_ARGUMENTS / BUILT_IN_TYPE_CONSTANT_OR_STATIC_METHOD / INHERIT_TYPE / TYPE_NAME(_OR_VOID) / PROPERTY_DECLARATION(_OR_TYPE) / PROPERTY_METHOD / OVERRIDE_METHOD / GET_NODE / RESOURCE_PATH / SUPER(_METHOD)。

**B. 引擎内部可复用表(全部有源码锚点,均与 extension_api.json 同源或可手搬)**:

- **虚方法表**:`COMPLETION_OVERRIDE_METHOD` 用 `ClassDB::get_virtual_methods(class_name)` 出带完整签名的 override 提示(静态另出 `_static_init`)——与 extension_api.json 方法字段 `is_virtual` **同源**(testdata 4.7.0 实测 1437 个虚方法)。
- **关键字三表**(`modules/gdscript/gdscript_editor.cpp:1741-1773`):裸关键字(true/false/PI/TAU/INF/NAN/null/self/super/break/breakpoint/continue/pass/return)、带空格后缀(and/not/or/in/as/class/class_name/extends/is/func/signal/await/const/enum/static/var/if/elif/else/for/match/when/while)、带参数(assert/preload)。我方 KEYWORD_DOCS 已全覆盖(缺 true/false/null 三个字面量)。
- **注解参数表**(`_find_annotation_arguments`,`modules/gdscript/gdscript_editor.cpp:1028`):`@export_range` 第 4 参 slider 串(or_greater/or_less/prefer_slider/hide_control/exp/radians_as_degrees/degrees/suffix:unit)、`@export_exp_easing`(attenuation/positive_only)、`@rpc`(mode/sync/transfer 三组 + 已用去重)、`@warning_ignore*`(GDScriptWarning 全名小写)、`@export_custom`(arg0=PropertyHint / arg2=PropertyUsageFlags 枚举)、`@export_node_path`(Node 全继承树 + 工程全局类)。
- **内建静态面**:`Variant::get_constants_for_type` + `get_builtin_method_list`(仅静态)——builtin_classes.constants/methods 同源(我方已摄取)。
- **全局面合成**(`_find_identifiers`):locals(含参数)→ 当前类成员(变量/常量/枚举/信号/函数/内部类)→ GDScriptUtilityFunctions 13 个编译器内建 → Variant 内建类型 → 关键字三表 → Variant utility functions 114 个 → autoload → ClassDB 全局映射(类/单例/常量)→ 全局枚举 → 工程全局类(class_name)。
- **insertText 约定**:函数补全 `insert_text += "("`,display 追加 `(…)`/`()`;LSP 层按 `snippetSupport` 决定补 `"$1)"` 还是裁掉 `(`(上游 2026-03 commit 7ab735f 才补 closing-brace 模式——**我方 C2 只做了模板类 snippet,函数名括号 snippet 是缺口**)。
- **工程/资源面**:`p_owner->get_argument_options("get_node", 0)`(场景实树 + autoload `/root/<名>`);`EditorFileSystem` 目录树补 res:// 路径;`dict[<|>` 对常量字典 property_list 出键。

**C. 我方模型已在吞咽却丢弃的字段**(testdata `extension_api.json` = Godot 4.7.0 stable 实测):

| 字段 | 规模 | 现状 | 用途 |
|---|---|---|---|
| `classes[].signals` | 503 条 | RawClass 未解析 | 接收者信号补全(引擎有,我方成员面缺) |
| `methods[].is_virtual` | 1437 个 | RawFunction 未解析 | override 面模型化(替代手维护 VIRTUALS 12 条) |
| `builtin_classes.operators/constructors/is_keyed/indexing_return_type` | — | 未解析 | 内建面完备性(运算符/构造/按键类型) |
| `classes[].api_type`(NONE/CORE/EDITOR/EXTENSION) | — | 未解析 | editor-only 类过滤(可选) |
| `is_refcounted` / `global_constants[].is_bitfield` / `header.version_full_name` | — | 未解析 | 语义标注 / 位域标志 / 版本展示 |

> **2026-10-16 晚审计补记(以 `int` 为样本)**:语料 `gdd_1559_int.md` 的 Constructors/Constructor
> Descriptions/Operators/Operator Descriptions 四区块**数据完整**(5 构造 + 64 运算符,与官方中文文档
> 同构);但 `tools/gen_docs.py` 与 `hoverdata.json` 生成面**只解析 Description/Method/Property/
> Enumerations**,904 个内建/类页零"构造函数/运算符"节——第二消费管线的缺口与 RawApi 丢弃同源。
> 已升级为 `completion-roadmap.md` C6.5 工作面(a 模型解析 / b 文档生成 / c 构造补全+类型名 hover)。
> **P9 已实施(2026-10-18)**:缺口闭合——`Constructors`/`Operators`(+Descriptions)四区块并入
> gen_docs 统一管线(网站 37 内建页两节 + hoverdata `k` 键 154c+351o);C6.5c① 按引擎锚点
> (gdscript_editor.cpp:3399-3416)修正为 **signatureHelp** 多重载(引擎 `Vector2(` 内不出补全项)。

**D. 社区 snippet 源(可克隆,已核对内容)**:

- **官方 godot-vscode-plugin `configurations/snippets.json`**(MIT,~45 条):ready/init/process/physics/input/**inpute2D/inpute3D**/uinput/draw/guii/for/if/while/func/match/signal/export/var/onready/is/in/gdscript/loadres/preloadres/**gs/vargetset**/xp/tween/wait。维护方 = Godot 官方组织。
- **loyal-disloyal/gdscript-zed-snippets**(MIT,Zed 向,~45 条):**cn**(class_name)/**stfunc/stvar**/lamb/clamp/lerp/onvar/loadres/preloadres/**waitsec/waitframe**/**prwarn/prerr**/signal/sc/class——与我方多 IDE(zed/nvim/helix)定位直接相关。
- **官方 script_templates**(`modules/gdscript/editor/script_templates/`,9 个:Node/CharacterBody2D/CharacterBody3D/EditorPlugin/EditorScenePostImport/EditorScript/Object/RichTextEffect/VisualShaderNodeCustom):`_BASE_`/`_TS_`/`%CLASS%` 占位 + `# meta-description` 头,`extends CharacterBody2D` 的 basic_movement 即出自此处。
- **已过时勿用**:Calinou/godot-snippets(CC0,但是 Godot 3 API:instance()/get_node("StreamPlayer"))、Leleat/AGodotCodeSnippetPlugin(3.2 时代)。

**E. R4 回归(诚实记录,非新设计)**:fa7d645 实现的 `$`/`%` 节点路径上下文抑制在 22882f7(R4 服务层重写)中丢失,gdscript-features-roadmap 第 7 项仍标 ✅,smoke 无覆盖——C7 一并修复并补回归测试。

**F. godotlsp 设计(C6-C9,合理取舍)——排期与验收见 `docs/completion-roadmap.md`**:

- **oracle 基建已建(2026-10-17,V0)**:`tools/completion_oracle.py` 对引擎自带 146 夹具跑 LSP 补全,分桶通过率 = 量化验收。已按本表落地:C6.4(函数 `name(…)` label + `name($1)` insertText 约定)、C6.6(`Class.Enum.<|>` 枚举值面)、C7.3($/% 上下文抑制)、**C6.1/C6.2(信号入成员面,oracle +9 全为此项战绩,24→33/140)**、**C7.1/C7.2(class_name/autoload 全局面 + `preload("res://` 路径面(textEdit 替换内容区,不依赖客户端 wordPattern))与 N2 字符串抑制(`string_literal_at` 状态机,字符串内返回空、路径面例外)**、**C8 一期(参数选项面,33→36/140,argument_options 0/11→3/11;harness 补语料根 project.godot 采用,输入动作面数据源到位)**、**C6.6b 枚举面入口扩展(2026-10-17,36→58/140,+22:builtin_enum 5/5 新增、global_enum 3/6→6/6、enum_values_in_match 3/7→7/7、全局枚举类型名 +22 入全局面、单点枚举名三态解析 + 字典键/类型标注门控消除 2 回归)**。**2026-10-17 资料盘点轮**:oracle 剩余 104 失败按共同阻塞归并为 5 簇(脚本成员面 ~30 / 枚举入口 ~16 / 语义推断 ~9 / callable 6 / 场景树 ~25),剩余工作重排为 P4-P11;**P4(C6.6b)已实施;P5(C10 脚本成员面)已实施(2026-10-17,58→75/140,+17 零回归)**——三层成员合并(live AST 本脚本自声明 → `Project::script_chain_members` 脚本父链 → `script_engine_base_by_path` 引擎链;`receiver_word` self./super. 短路;override 签名面源码原样含多行 lambda(`signature_text_raw`);`FuncDecl.signature_text` 系 + **`Param.span` 解析器修复**(原取下一 token 区间);`SELF` sort 桶;oracle harness 布局修复(语料镜像 `ws/completion/` 子目录对齐夹具 `res://completion/…` 引用,此前脚本链/场景路径全断链);oracle 运行须 `--server target-bench/...`(默认 target/debug 被锁,曾致 0 改善假象);详见 `completion-roadmap.md` §P5 详节);**P6(C5 语义层类型推断)已实施(2026-10-17,75→99/140,+24 零回归)**——新模块 `gdscript-syntax/src/infer.rs` 静默推断器(`InferTy` 三态 Class/ScriptPath/Unknown,失败一律 Unknown 降级,与诊断分析解耦);`resolve_name_type` 局部→类级→preload 常量递归→内部类→全局 class_name→引擎类解析链;D1 子类收窄(`is_subtype` root 闭包统一 ScriptPath/Class 根解析,初始值 is-a 注解时用初始值);D2 764 类枚举名入类型标注位(`: word$`/`-> word$`,冒号箭头到词首须**同行**——防 `func f():` 块冒号跨行误判);D3 `type_member_items` 类型成员面逐段下钻(内部类/枚举/const + 引擎链 + 全局/类枚举名,失败回落原链保 P4 枚举值面门控);D4 N5 Callable 面(`argopts::callable_arg_items`,实参位形参 callable → 裸名无括号,置于 argument_options 之后兜底 + `site.literal` 先退防抢字符串面);internal 属性排除(`gen_internal_props.py` → `internal_props.json` 45 类/80 属性,PROPERTY_USAGE_INTERNAL 入库过滤);**教训**:no_parenthesis builtin/native 3 夹具依赖词法器括号计数修复(未闭合调用吞后续 `func` 声明,AST 层恢复无效已回滚)——独立债记 §3.6;详见 `completion-roadmap.md` §P6 详节);**P7(C6.3 虚方法模型化)已实施(2026-10-17,99→101/140,+2 零回归)**——`Function.is_virtual`(extension_api 透传,1437 计数锚定)+ `EngineApi::chain_virtuals`(链上同名先到先得,镜像引擎 `options.has(mi.name)`)+ `OBJECT_CORE_VIRTUALS` 12 条手表(object.cpp:1919-1979 `add_virtual_method` 手工注册、不在 api;原记 9 个漏 `_iter_*` 3);`push_engine_virtuals` 接 override 面(签名渲染镜像 `_get_visual_datatype`;static 位只出 `_static_init() -> void:`;**光标所在函数排除** = `member != function_node` 子句,`func _get➡` 时 _get 不算已声明——full_name 卡 absent 的根因);详见 `completion-roadmap.md` §P7 详节);**P8(C8 二期参数选项面)已实施(2026-10-18,101→106/140,+5 零回归)**——`State.prefs` + `didChangeConfiguration` 的 `settings.gdscript` 段三键(oracle 单 Client 全夹具 → **per-fixture didChangeConfiguration** 镜像上游每夹具 set_setting;Python `bool("false")`=True 坑须显式 as_bool);`quoted_option` 前缀化(`QuoteKind{Str,StringName,NodePath}` + `QuoteStyle` 聚合,镜像 `_calculate_string_insertion`:`&` 优先 `^`、insertText 仅 should&&!has 带前缀、单引号换引号字符);tween_property 属性面(argidx==1 && Tween 系 → 实参 0 裸标识符 `first_arg_text` → P6 resolve → 脚本链仅 var + 引擎链 properties 排除 internal;argopts 先于 N2,inside_string 夹具连带);**infer.rs 方法链推断**(Attribute callee 泛化 + Ident 分支 names 回退 + names 闭包扩 self 方法——`get_tree().create_tween()` 全链 → Tween);InputMap 单例面**不做**(rg 引擎全文仅 InputEvent contains("action") 一处,无锚点,修正 roadmap 原判);详见 `completion-roadmap.md` §P8 详节);盘点同时纠正两处历史假设并验证两处数据源(引擎仓无 snippet 表/script_templates;`doc/classes/*.xml` 带完整构造/运算符签名+描述)。

- **C6 模型层补齐**(gdscript-engine,附加式 serde,零行为风险):解析 signals / is_virtual / api_type / is_refcounted / is_bitfield / builtin operators+constructors+is_keyed+indexing_return_type / version_full_name;配套查询方法(类信号表、链上虚方法表)。消费侧:① 成员面补信号(kind Event,detail 带参数签名);② 函数补全 insertText `(` + display `(…)`,snippetSupport 时 `($1)`;③ `func _<|>` 虚方法面改**模型驱动**(沿 ancestor_chain 收 is_virtual,签名取模型),仅保留 9 个非 ClassDB 回调手表(_init/_notification/_get/_set/_to_string/_property_can_revert/_property_get_revert/_get_property_list/_validate_property)。**builtin 构造/运算符数据源定论(2026-10-17 实测)**:`extension_api.json` 的 `constructors` 只有 `index`、`operators` 只有 `name/right_type/return_type`(均无签名/描述);`.refs/godot/doc/classes/*.xml`(810 个)带完整 `param name/type`+`return`+`description` → C6.5 = XML(签名+描述)× JSON(结构)双源互补,英文数据,中文走 P2-16。
- **C7 工程面接入**(godotlsp,数据已在 gdscript-project 索引,只补消费):① 全局面补 class_name + autoload(引擎同等行为,保留名去重扩面);② `preload("res://` / `load("res://` 字符串字面量内补全项目文件(scripts/scenes/resources + uid 映射,服务端按已输入路径前缀过滤);③ 恢复 $/% 节点路径上下文(镜像内核向后字节扫描,% 取模操作数启发式)+ smoke 回归。
- **C8 参数选项面** ✅ 一期已实施(2026-10-17,`argopts.rs`):前向括号帧扫描定位调用/注解实参站点(argidx 跨嵌套/括号字面量正确归属),独占面语义对齐引擎(`r_forced = size > 0`:调用面零项回落、注解参数面 forced)。三面落地:① 信号名面(`connect` 等首参,self extends 链 + 脚本链自声明);② 输入动作面(InputEvent 系 `*action*` 方法,`[input]` 表,gdscript-project 新增解析);③ 注解参数表(slider/rpc/easing 手搬 + `@warning_ignore*` 46 名**取引擎 `gdscript_warning.h`**——本地 gdd_0310 语料页无警告名表,不可作源;`@export_custom` 两参取模型全局枚举;`@export_node_path` 取 Node 继承树 + 工程脚本类)。既有闭合字面量走 textEdit 整串替换(保留 `&`/`^` 前缀,镜像 `_calculate_string_insertion`);引擎默认设置(false)侧一律双引号。oracle `argument_options` 0/11→3/11(connect/dont_add_string_name/prepended);tween_property 属性面与 AnimationPlayer 动画面缓行(需局部变量类型推断/场景解析)。
- **C9 snippet 对齐(官方模板改为来源考证)**:以官方 vscode snippets + zed snippets 为基准,交叉核对现有 16 条插件 snippet 与 C2 模板,补齐高价值缺口(_input_event 2D/3D、inner class、is/in、loadres/preloadres、vargetset、cn、waitsec/waitframe、prwarn/prerr、stfunc/stvar、enum decl)。**官方 script_templates 入库计划的前提不成立**——2026-10-17 实测 `.refs/godot` 全量克隆内无任何模板 `.gd`(全仓唯一 `.gd` 是测试夹具,`editor/` 源码无 `script_templates` 引用;Godot 4 模板走编辑器安装目录/用户目录);改为 fetch 时先做来源考证,能 pinned 复验才入库(判据不变:clone 即可用),不可复验则明确不收并记录来源。
- **明确不做**:① **不外挂 godot --headless LSP**:需用户自备引擎 + 全量 import(秒级)+ 编辑器缓存语义,违背"零引擎依赖"定位;只借鉴其数据契约(snippet 括号、kind 映射、scene cache 概念)。② **不移植 `_guess_*` 千行推断引擎**:现有 semantic + 模型覆盖务实面;类型未知时降级不出成员(与引擎行为一致)。③ **字典键补全**:需字面常量折叠,性价比低。④ **@export_tool_button 图标表**:编辑器主题数据,无头不可得。⑤ **不全文克隆引擎仓库**:目标文件 raw 拉取 + pinned 引用;复用表随消费点入库并注源。
- **验收**:C6/C7/C8 各带单元测试 + smoke 断言(成员面含信号 / override 面签名正确 / class_name 全局面 / res:// 路径面 / `$Panel/` 回归);每阶段跑 bench_load 负载基准不回退;docs/lsp-engine-capabilities-roadmap.md 同步打勾。
