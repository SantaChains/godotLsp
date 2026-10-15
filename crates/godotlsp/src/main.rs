//! `godotlsp` — 独立的、脱离引擎的 GDScript 语言服务器,走 stdio(LSP 3.17)。
//!
//! 分析能力来自 `gdscript-ide`(salsa 增量、场景感知);本二进制负责 JSON-RPC
//! 循环、位置编码转换、项目加载与能力分发。
//!
//! 协议层基于 `lsp-server-tokio`:tokio 异步 stdio、显式路由 `conn.route()`、
//! 每请求 `CancellationToken`、`duplex_transport()` 内存协议测试。
//!
//! P2 并发模型(单写者 + 线程池快照,参照 rust-analyzer / pyrefly / clangd):
//! - **单写者**:`State` 由主循环独占,通知(状态变更)就地处理;
//! - **线程池快照**:请求解析参数后拍 [`Snapshot`],`spawn_blocking` 上计算,
//!   结果经双通道回传主循环 —— 响应通道(`resp_rx`)优先于诊断通道(`diag_rx`);
//! - **写者等待读者**(salsa 0.27 硬约束):写路径要求全部快照句柄消亡,
//!   因此任何写(编辑应用、配置)前主循环先 `quiesce()` 等在飞任务归零,
//!   并发上限 `MAX_INFLIGHT` 进一步约束写等待的上界;
//! - **取消三态**(RA `dispatch.rs` 范式):`$/cancelRequest` 在启动前拦截 →
//!   `RequestCancelled(-32800)`;计算完成后客户端已取消 → 静默丢弃(ruff 式);
//!   结果回到主循环时快照 revision 已过期(期间发生过写)→ `ContentModified(-32801)`
//!   (内核防御性 `Err(Cancelled)` 同样映射于此);
//! - **编辑作废**:诊断任务带代际,结果返回时若已被更新的编辑取代则丢弃
//!   (pyrefly 式);被取消的诊断自动重排;
//! - **自适应去抖**(clangd `DebouncePolicy` 范式):诊断延迟 = 最近 15 次计算
//!   耗时中位数,clamp 到 [50ms, 500ms]。

mod convert;
mod gdscript_docs;
mod globals;
mod state;
mod text;
mod uri;

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures::{FutureExt, StreamExt};
use gdscript_base::{Cancelled, FileId, FilePosition, TextRange};
use gdscript_ide::WarningOverride;
use lsp_server_tokio::{
    ClientSender, Connection, ErrorCode, IncomingMessage, Message, Notification,
    Request, RequestId, Response, ResponseError,
};
use lsp_types::{
    CodeActionParams, ColorPresentationParams, CompletionOptions, CompletionParams,
    ConfigurationItem, ConfigurationParams, DocumentColorParams, DocumentFormattingParams,
    DocumentHighlightParams, DocumentLinkParams, DocumentRangeFormattingParams,
    DocumentSymbolParams, FoldingRangeParams, GotoDefinitionParams, HoverParams, InitializeParams,
    InlayHintParams, PublishDiagnosticsParams, ReferenceParams, RenameParams,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams,
    SemanticTokensServerCapabilities, ServerCapabilities, SignatureHelpOptions,
    SignatureHelpParams, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
    WorkspaceSymbolParams,
};
// `<Notification>::METHOD` 关联常量由该 trait 提供;匿名导入避免与
// `lsp_server_tokio::Notification` 类型名冲突。
use lsp_types::notification::Notification as _;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{Notify, Semaphore};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Instant;

use crate::convert as cv;
use crate::state::{SECTION, Snapshot, State};

fn main() {
    // `--stdio` 是 vscode-languageclient 等客户端附加的标准传输参数,等价于默认行为。
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("godotlsp {}", env!("CARGO_PKG_VERSION"));
        }
        Some(other) if other != "--stdio" => {
            eprintln!("[godotlsp] 未知参数:{other}");
            std::process::exit(2);
        }
        _ => {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("[godotlsp] 无法启动 tokio 运行时");
            if let Err(e) = rt.block_on(run()) {
                eprintln!("[godotlsp] 致命错误:{e}");
                std::process::exit(1);
            }
        }
    }
}

/// 二进制入口:stdio 传输交给异步主循环。
async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    run_on(Connection::stdio()).await
}

/// 服务器→客户端日志通知。信息/警告类日志经 `window/logMessage` 送达,
/// 客户端按消息级别写入输出通道(stderr 会被 vscode-languageclient
/// 一律按 error 级展示,故仅致命错误才走 stderr)。发送失败静默忽略。
fn log_notify<T>(conn: &mut Connection<T, ()>, typ: lsp_types::MessageType, message: String)
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let params = lsp_types::LogMessageParams { typ, message };
    let _ = conn.send(Message::Notification(Notification::new(
        lsp_types::notification::LogMessage::METHOD,
        Some(cv::to_value(&params)),
    )));
}

/// 阻塞线程池上一次消息处理的结果。
struct RespMsg {
    id: RequestId,
    /// 计算完成后客户端是否已 `$/cancelRequest`:取消的响应静默丢弃(ruff 式)。
    client_cancelled: bool,
    /// 派发请求时的世界修订号;与当前 `State::revision` 比对判定过期。
    snapshot_revision: u64,
    outcome: Result<serde_json::Value, HandleError>,
}

/// 线程池上一次诊断计算的结果。
struct DiagMsg {
    uri: String,
    /// 调度代际:结果返回时与最新代际比对,过期即弃(编辑作废)。
    generation: u64,
    version: Option<i32>,
    duration_ms: u64,
    /// `Err(Cancelled)` = 并发编辑使快照失效 → 主循环重排诊断(等价 RA 重试)。
    result: Result<Vec<lsp_types::Diagnostic>, Cancelled>,
}

/// 诊断调度器:代际 + 单一最早截止时间 + 自适应去抖。
struct DiagScheduler {
    /// uri → (代际, 截止时间)。
    pending: HashMap<String, (u64, Instant)>,
    /// uri → 最新代际( spawn 后仍保留,供结果比对)。
    gens: HashMap<String, u64>,
    next_gen: u64,
    /// 最近诊断计算耗时(自适应去抖样本)。
    durations: VecDeque<u64>,
}

const DIAG_HISTORY: usize = 15;
const DIAG_DEBOUNCE_MIN_MS: u64 = 50;
const DIAG_DEBOUNCE_MAX_MS: u64 = 500;

/// 并发在飞任务上限。约束「写者等待读者」的最坏等待时间(见模块文档)。
const MAX_INFLIGHT: usize = 4;

/// 在飞任务计数:写者(quiesce)等待读者退出,读者结束即唤醒。
struct Inflight {
    count: AtomicUsize,
    notify: Notify,
}

impl Inflight {
    fn new() -> Self {
        Self { count: AtomicUsize::new(0), notify: Notify::new() }
    }

    fn enter(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// 任务结束:计数回落并唤醒等待中的写者。
    fn leave(&self) {
        if self.count.fetch_sub(1, Ordering::Release) == 1 {
            self.notify.notify_one();
        }
    }
}

/// 写前静默:等待全部在飞任务结束(salsa 0.27 要求写时无存活快照句柄)。
/// 任务通道为 unbounded,任务端永不阻塞,本等待必然有界。
async fn quiesce(inflight: &Inflight) {
    while inflight.count.load(Ordering::Acquire) > 0 {
        inflight.notify.notified().await;
    }
}

/// 非阻塞吸收配置拉取响应:oneshot 已被 route() 填充则应用配置。
/// 在主循环顶部与 drain 的每次分发前调用 —— 同批就绪的
/// [配置响应, 依赖配置的消息] 必须按序生效。
async fn absorb_config(
    cfg_rx: &mut Option<tokio::sync::oneshot::Receiver<Response>>,
    state: &mut State,
    inflight: &Inflight,
) {
    let Some(rx) = cfg_rx.as_mut() else { return };
    let Ok(resp) = rx.try_recv() else { return };
    *cfg_rx = None;
    let first = resp
        .into_result()
        .and_then(|r| r.as_array().and_then(|a| a.first().cloned()));
    if let Some(first) = first {
        // apply_settings 内含 set_warning_override(一次写):先等在飞任务。
        quiesce(inflight).await;
        apply_settings(state, &first);
    }
}

impl DiagScheduler {
    fn new() -> Self {
        Self {
            pending: HashMap::new(),
            gens: HashMap::new(),
            next_gen: 0,
            durations: VecDeque::with_capacity(DIAG_HISTORY),
        }
    }

    /// 登记一次内容变更:代际推进,截止时间 = 现在 + 自适应去抖。
    /// 同一 uri 的连续编辑天然合并(只保留最新代际与最新截止时间)。
    fn touch(&mut self, uri: &str, now: Instant) {
        let generation = self.next_gen;
        self.next_gen += 1;
        self.gens.insert(uri.to_string(), generation);
        let deadline = now + self.debounce();
        self.pending.insert(uri.to_string(), (generation, deadline));
    }

    /// clangd `DebouncePolicy` 范式:最近 `DIAG_HISTORY` 次计算耗时的中位数,
    /// clamp 到 [50, 500]ms —— 解析越慢去抖越长;无样本时取下限。
    fn debounce(&self) -> std::time::Duration {
        let median = if self.durations.is_empty() {
            DIAG_DEBOUNCE_MIN_MS
        } else {
            let mut v: Vec<u64> = self.durations.iter().copied().collect();
            v.sort_unstable();
            v[v.len() / 2]
        };
        std::time::Duration::from_millis(median.clamp(DIAG_DEBOUNCE_MIN_MS, DIAG_DEBOUNCE_MAX_MS))
    }

    fn next_deadline(&self) -> Option<Instant> {
        self.pending.values().map(|(_, d)| *d).min()
    }

    /// 取出到期的 (uri, 代际),从待发集合移除。
    fn take_due(&mut self, now: Instant) -> Vec<(String, u64)> {
        let due: Vec<String> = self
            .pending
            .iter()
            .filter(|(_, (_, d))| *d <= now)
            .map(|(u, _)| u.clone())
            .collect();
        due.into_iter()
            .filter_map(|u| self.pending.remove(&u).map(|(g, _)| (u, g)))
            .collect()
    }

    /// 该代际是否仍是此 uri 的最新调度(否则结果过期,丢弃)。
    fn is_current(&self, uri: &str, generation: u64) -> bool {
        self.gens.get(uri) == Some(&generation)
    }

    fn record_duration(&mut self, ms: u64) {
        if self.durations.len() == DIAG_HISTORY {
            self.durations.pop_front();
        }
        self.durations.push_back(ms);
    }
}

/// 单条入站消息的处理结论。
enum Flow {
    /// 继续主循环。
    Continue,
    /// `exit`(shutdown 之后):干净终止。
    Exit,
}

/// 在传入的 `Connection` 上运行协议循环(泛型以便测试用 `duplex_transport` 注入)。
async fn run_on<T>(mut conn: Connection<T, ()>) -> Result<(), Box<dyn Error + Send + Sync>>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let init_value = conn
        .initialize(capabilities())
        .await
        .map_err(|e| io::Error::other(format!("initialize 失败:{e}")))?;
    let init: InitializeParams =
        serde_json::from_value(init_value).map_err(|e| e.to_string())?;
    let mut state = State::new();
    let loaded = if let Some(root) = params_root(&init) {
        state.load_project(root)
    } else {
        0
    };
    // 信息类日志走 `window/logMessage`(客户端按对应级别入输出通道);
    // stderr 会被 vscode-languageclient 一律按 error 展示,只保留真正的致命错误。
    log_notify(
        &mut conn,
        lsp_types::MessageType::INFO,
        format!("已加载 {loaded} 个项目文件"),
    );
    log_notify(
        &mut conn,
        lsp_types::MessageType::INFO,
        format!("{} 引擎模型就绪", gdscript_api::godot_version()),
    );

    // 拉取式配置:zed/helix 等客户端只回 `workspace/configuration`,不主动推
    // `didChangeConfiguration`。声明了 `workspace.configuration` 能力即主动拉一次,
    // 结果与后续推送走同一条 apply_settings 路径。
    //
    // 实现:`ClientSender::request` 的 future 依赖主循环 route() 投递响应,不能在
    // 循环外直接 await(死锁),也不能跨 select! 重建存活(drop 竞态)。因此走
    // 出站队列:注册 -1 号请求(server→client 用负 id,与客户端正 id 空间隔离),
    // fire-and-forget 发出;route() 同步投递响应到 oneshot,主循环在处理每条
    // 消息前 try_recv——FIFO 保证响应先于后续消息被消费,配置必然先于一切
    // 依赖它的请求应用。
    let sender = conn.client_sender();
    let mut cfg_rx = if init
        .capabilities
        .workspace
        .as_ref()
        .and_then(|w| w.configuration)
        .unwrap_or(false)
    {
        const CFG_PULL_ID: i32 = -1;
        let params = ConfigurationParams {
            items: vec![ConfigurationItem {
                scope_uri: None,
                section: Some(SECTION.into()),
            }],
        };
        let rx = conn
            .request_queue
            .outgoing
            .register(RequestId::from(CFG_PULL_ID));
        if let Err(e) = conn.send(Message::Request(Request::new(
            RequestId::from(CFG_PULL_ID),
            "workspace/configuration",
            Some(cv::to_value(&params)),
        ))) {
            log_notify(
                &mut conn,
                lsp_types::MessageType::WARNING,
                format!("配置拉取请求发送失败:{e}"),
            );
            None
        } else {
            Some(rx)
        }
    } else {
        None
    };

    // 任务回传双通道(§6.7 第 2/4 步):响应优先于诊断(biased select),
    // 用户可见结果不被后台诊断流量阻塞。通道无界:任务端发送永不阻塞,
    // `quiesce` 的写前等待因此必然有界(不存在「任务等发送 + 写者等任务」环)。
    let (resp_tx, mut resp_rx) = tokio::sync::mpsc::unbounded_channel::<RespMsg>();
    let (diag_tx, mut diag_rx) = tokio::sync::mpsc::unbounded_channel::<DiagMsg>();
    let inflight = Arc::new(Inflight::new());
    let permits = Arc::new(Semaphore::new(MAX_INFLIGHT));
    let mut sched = DiagScheduler::new();

    // LSP 3.17 生命周期。`handle_shutdown` 仅回包并把连接置为 ShuttingDown;
    // 随后客户端发 `exit` 通知,循环据此干净退出。`exit` 在 `shutdown` 之前到达
    // 属异常终止,按规范以非零码退出。
    let mut shutdown_requested = false;
    'main: loop {
        // 吸收已到达的配置拉取响应(若有),再进入本轮分发——
        // 保证配置先于一切依赖它的请求应用。
        absorb_config(&mut cfg_rx, &mut state, &inflight).await;

        // 诊断调度:单一定时器指向最早的待发截止时间。
        let mut sleep = sched
            .next_deadline()
            .map(|d| Box::pin(tokio::time::sleep_until(d)));
        tokio::select! {
            biased;
            // 用户可见响应:最高优先。
            Some(resp) = resp_rx.recv() => {
                // 线程池任务结束,结算其入站队列槽位(取消路由随之上限解除)。
                conn.request_queue.incoming.complete(&resp.id);
                if resp.client_cancelled {
                    // 计算完成后客户端已取消:静默丢弃(ruff 式)。
                    continue;
                }
                let response = match resp.outcome {
                    // 计算期间发生过写(快照 revision 落后):结果过期,
                    // RA 的非延迟敏感路径 —— ContentModified,客户端会重试。
                    Ok(value) if resp.snapshot_revision == state.revision => {
                        Response::ok(resp.id, value)
                    }
                    Ok(_) => Response::err(
                        resp.id,
                        ResponseError::new(
                            ErrorCode::ContentModified,
                            "文档在计算期间被修改,结果已过期,请重试",
                        ),
                    ),
                    Err(HandleError::Msg(msg)) => {
                        Response::err(resp.id, ResponseError::new(ErrorCode::InternalError, msg))
                    }
                    // 内核防御性取消(单写者 + 写前 quiesce 下不应发生,兜底)。
                    Err(HandleError::Cancelled) => Response::err(
                        resp.id,
                        ResponseError::new(
                            ErrorCode::ContentModified,
                            "文档在计算期间被修改,结果已过期,请重试",
                        ),
                    ),
                };
                let _ = sender.respond(response);
            }
            // 诊断结果:过期即弃;被取消则重排。
            Some(msg) = diag_rx.recv() => {
                if msg.result.is_err() {
                    // 并发编辑使诊断快照失效:重排(等价 rust-analyzer 的诊断重试)。
                    sched.touch(&msg.uri, Instant::now());
                    continue;
                }
                sched.record_duration(msg.duration_ms);
                let Ok(diags) = msg.result else { unreachable!("错误分支已在上方处理") };
                // 代际仍是最新且文件仍在注册表中,才发布(编辑作废 + 删除不复活)。
                if sched.is_current(&msg.uri, msg.generation) && state.id_of(&msg.uri).is_some() {
                    publish(&sender, &msg.uri, diags, msg.version);
                }
            }
            // 诊断去抖到期:把到期的 uri 派发到线程池。
            _ = async {
                if let Some(s) = sleep.as_mut() {
                    s.as_mut().await;
                } else {
                    std::future::pending::<()>().await;
                }
            } => {
                for (uri, generation) in sched.take_due(Instant::now()) {
                    if let Some(file) = state.id_of(&uri) {
                        // 快照在此拍(写者不并发:主循环单任务内顺序执行);
                        // spawn_diagnostics 只收 Send 的 Snapshot,不持 `&State`。
                        let snap = state.snapshot();
                        spawn_diagnostics(snap, generation, uri, file, &diag_tx, &inflight, &permits).await;
                    }
                }
            }
            // 入站消息:单写者就地处理。
            item = conn.receiver.next() => {
                let Some(result) = item else { break };
                let msg = result
                    .map_err(|e| io::Error::other(format!("接收失败:{e}")))?;
                match dispatch(msg, &mut conn, &sender, &mut state, &mut sched, &resp_tx, &inflight, &permits, &mut shutdown_requested).await {
                    Flow::Continue => {}
                    Flow::Exit => return Ok(()),
                }
                // 事件合并(RA main_loop 50ms 批处理范式):把已就绪的消息
                // 非阻塞排空再统一触发诊断调度;预算仅作防线,永不主动等待。
                // 每次分发前先吸收配置 —— 同批就绪的 [配置响应, 依赖配置的消息]
                // 必须按序应用。
                let budget = Instant::now() + std::time::Duration::from_millis(50);
                loop {
                    absorb_config(&mut cfg_rx, &mut state, &inflight).await;
                    match conn.receiver.next().now_or_never() {
                        None => break,
                        Some(None) => break 'main,
                        Some(Some(Ok(m))) => {
                            match dispatch(m, &mut conn, &sender, &mut state, &mut sched, &resp_tx, &inflight, &permits, &mut shutdown_requested).await {
                                Flow::Continue => {}
                                Flow::Exit => return Ok(()),
                            }
                        }
                        Some(Some(Err(e))) => {
                            return Err(io::Error::other(format!("接收失败:{e}")).into());
                        }
                    }
                    if Instant::now() >= budget {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

/// 分发一条入站消息:路由、生命周期、请求派发、通知应用。
// 参数虽多但均为同层循环上下文的显式传递;引入上下文结构体反而模糊借用边界。
#[allow(clippy::too_many_arguments)]
async fn dispatch<T>(
    msg: Message,
    conn: &mut Connection<T, ()>,
    sender: &ClientSender,
    state: &mut State,
    sched: &mut DiagScheduler,
    resp_tx: &UnboundedSender<RespMsg>,
    inflight: &Arc<Inflight>,
    permits: &Arc<Semaphore>,
    shutdown_requested: &mut bool,
) -> Flow
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    match conn.route(msg) {
        IncomingMessage::Request(req, token) => {
            if req.method == "shutdown" {
                if let Err(e) = conn.handle_shutdown(req.id) {
                    eprintln!("[godotlsp] shutdown 失败:{e}");
                }
                *shutdown_requested = true;
                return Flow::Continue;
            }
            // 取消三态之一(启动前):客户端已取消 → 不进入线程池。
            if token.is_cancelled() {
                let _ = sender.respond(Response::err(
                    req.id.clone(),
                    ResponseError::new(
                        ErrorCode::RequestCancelled,
                        "请求已被客户端取消",
                    ),
                ));
                conn.request_queue.incoming.complete(&req.id);
                return Flow::Continue;
            }
            // 参数解析在主循环完成(廉价);分析计算携带快照进线程池,
            // 主循环立即回到事件处理 —— 写者(编辑应用)只需等本轮在飞任务,
            // 不再被长请求串行阻塞。
            let parsed: Result<RequestKind, serde_json::Error> = parse_request(&req);
            let kind = match parsed {
                Ok(kind) => kind,
                Err(e) => {
                    let _ = sender.respond(Response::err(
                        req.id.clone(),
                        ResponseError::new(ErrorCode::InvalidParams, format!("参数无效:{e}")),
                    ));
                    conn.request_queue.incoming.complete(&req.id);
                    return Flow::Continue;
                }
            };
            // 并发上限:await 一个空位。任务不依赖主循环推进(unbounded 通道),
            // 因此等待有界,不构成死锁。
            let Ok(permit) = permits.clone().acquire_owned().await else {
                eprintln!("[godotlsp] 任务池已关闭,忽略请求 {}", req.method);
                return Flow::Continue;
            };
            let snap = state.snapshot();
            let id = req.id.clone();
            let method = req.method.clone();
            let resp_tx = resp_tx.clone();
            let inflight = Arc::clone(inflight);
            inflight.enter();
            // 注意:入站队列槽位不在此结算 —— 线程池任务结束、RespMsg 回达主循环时
            // 才 complete,保证期间到达的 $/cancelRequest 仍能取消该 token。
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                // panic 隔离:单个请求的分析崩溃只回错误响应,不拖垮整个服务器
                // (rust-analyzer 同款防线;分析世界通常仍可服务后续请求)。
                let outcome = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handle(&snap, &kind)
                })) {
                    Ok(outcome) => outcome,
                    Err(_) => {
                        eprintln!("[godotlsp] 处理 {method} 时 panic,已隔离");
                        Err(HandleError::Msg(format!(
                            "内部错误:处理 {method} 时服务器异常(已恢复)"
                        )))
                    }
                };
                let client_cancelled = token.is_cancelled();
                let snapshot_revision = snap.revision;
                // unbounded:发送不阻塞,任务必然结束并释放句柄与计数。
                let _ = resp_tx.send(RespMsg { id, client_cancelled, snapshot_revision, outcome });
                inflight.leave();
            });
            Flow::Continue
        }
        IncomingMessage::Notification(notif) => {
            if notif.method == "exit" {
                if *shutdown_requested {
                    return Flow::Exit;
                }
                eprintln!("[godotlsp] 未收到 shutdown 即 exit,以异常码终止");
                std::process::exit(1);
            }
            on_notification(sender, state, sched, inflight, notif).await;
            Flow::Continue
        }
        // `$/cancelRequest` 已被 route() 自动处理(取消对应 token)。
        IncomingMessage::CancelHandled => Flow::Continue,
        // 服务器发起请求的响应已交付给等待方(配置拉取走出站队列)。
        IncomingMessage::ResponseRouted => Flow::Continue,
        IncomingMessage::ResponseUnknown(resp) => {
            eprintln!("[godotlsp] 收到未知响应 id={:?}", resp.id);
            Flow::Continue
        }
        _ => Flow::Continue,
    }
}

fn capabilities() -> serde_json::Value {
    let (token_types, token_modifiers) = cv::semantic_tokens_legend();
    let token_types: Vec<lsp_types::SemanticTokenType> = token_types
        .iter()
        .copied()
        .map(lsp_types::SemanticTokenType::new)
        .collect();
    let token_modifiers: Vec<lsp_types::SemanticTokenModifier> = token_modifiers
        .iter()
        .copied()
        .map(lsp_types::SemanticTokenModifier::new)
        .collect();
    let caps = ServerCapabilities {
        position_encoding: Some(lsp_types::PositionEncodingKind::UTF16),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        references_provider: Some(lsp_types::OneOf::Left(true)),
        document_highlight_provider: Some(lsp_types::OneOf::Left(true)),
        document_link_provider: Some(lsp_types::DocumentLinkOptions {
            resolve_provider: Some(false),
            work_done_progress_options: Default::default(),
        }),
        color_provider: Some(lsp_types::ColorProviderCapability::Simple(true)),
        document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        workspace_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        rename_provider: Some(lsp_types::OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some([".", "@", "$", "%", "/"].into_iter().map(Into::into).collect()),
            ..CompletionOptions::default()
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some([",", "("].into_iter().map(Into::into).collect()),
            retrigger_characters: None,
            work_done_progress_options: Default::default(),
        }),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: SemanticTokensLegend {
                    token_types,
                    token_modifiers,
                },
                range: Some(false),
                full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
            },
        )),
        inlay_hint_provider: Some(lsp_types::OneOf::Left(true)),
        folding_range_provider: Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
        code_action_provider: Some(lsp_types::CodeActionProviderCapability::Simple(true)),
        document_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        document_range_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        ..ServerCapabilities::default()
    };
    cv::to_value(&caps)
}

fn params_root(init: &InitializeParams) -> Option<std::path::PathBuf> {
    let to_path = |u: &Uri| crate::uri::uri_to_path(u);
    init.workspace_folders
        .as_ref()
        .and_then(|f| f.first())
        .and_then(|f| to_path(&f.uri))
        .or_else(|| {
            #[allow(deprecated)]
            init.root_uri.as_ref().and_then(to_path)
        })
}

/// 请求内部处理错误:`Cancelled` 由并发编辑引发,协议层映射为
/// `ContentModified`;`Msg` 为用户可见的失败说明(InternalError)。
enum HandleError {
    Msg(String),
    Cancelled,
}

impl From<Cancelled> for HandleError {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}

/// 每个支持的请求一个变体,分发表因此收敛为单个 match。
enum RequestKind {
    Hover(HoverParams),
    Definition(GotoDefinitionParams),
    Completion(CompletionParams),
    DocumentSymbol(DocumentSymbolParams),
    References(ReferenceParams),
    DocumentHighlight(DocumentHighlightParams),
    DocumentLink(DocumentLinkParams),
    DocumentColor(DocumentColorParams),
    ColorPresentation(ColorPresentationParams),
    Rename(RenameParams),
    PrepareRename(lsp_types::TextDocumentPositionParams),
    WorkspaceSymbol(WorkspaceSymbolParams),
    SemanticTokens(SemanticTokensParams),
    InlayHint(InlayHintParams),
    FoldingRange(FoldingRangeParams),
    SignatureHelp(SignatureHelpParams),
    CodeAction(CodeActionParams),
    Formatting(DocumentFormattingParams),
    RangeFormatting(DocumentRangeFormattingParams),
}

/// 把协议请求反序列化为内部求和类型(主循环内完成,廉价)。
fn parse_request(req: &Request) -> Result<RequestKind, serde_json::Error> {
    let p = || req.params.clone().unwrap_or_default();
    match req.method.as_str() {
        "textDocument/hover" => serde_json::from_value(p()).map(RequestKind::Hover),
        "textDocument/definition" => serde_json::from_value(p()).map(RequestKind::Definition),
        "textDocument/completion" => serde_json::from_value(p()).map(RequestKind::Completion),
        "textDocument/documentSymbol" => serde_json::from_value(p()).map(RequestKind::DocumentSymbol),
        "textDocument/references" => serde_json::from_value(p()).map(RequestKind::References),
        "textDocument/documentHighlight" => {
            serde_json::from_value(p()).map(RequestKind::DocumentHighlight)
        }
        "textDocument/documentLink" => serde_json::from_value(p()).map(RequestKind::DocumentLink),
        "textDocument/documentColor" => serde_json::from_value(p()).map(RequestKind::DocumentColor),
        "textDocument/colorPresentation" => {
            serde_json::from_value(p()).map(RequestKind::ColorPresentation)
        }
        "textDocument/rename" => serde_json::from_value(p()).map(RequestKind::Rename),
        "textDocument/prepareRename" => serde_json::from_value(p()).map(RequestKind::PrepareRename),
        "workspace/symbol" => serde_json::from_value(p()).map(RequestKind::WorkspaceSymbol),
        "textDocument/semanticTokens/full" => {
            serde_json::from_value(p()).map(RequestKind::SemanticTokens)
        }
        "textDocument/inlayHint" => serde_json::from_value(p()).map(RequestKind::InlayHint),
        "textDocument/foldingRange" => serde_json::from_value(p()).map(RequestKind::FoldingRange),
        "textDocument/signatureHelp" => serde_json::from_value(p()).map(RequestKind::SignatureHelp),
        "textDocument/codeAction" => serde_json::from_value(p()).map(RequestKind::CodeAction),
        "textDocument/formatting" => serde_json::from_value(p()).map(RequestKind::Formatting),
        "textDocument/rangeFormatting" => {
            serde_json::from_value(p()).map(RequestKind::RangeFormatting)
        }
        _ => Err(serde::de::Error::custom(format!("未支持的方法 {}", req.method))),
    }
}

/// 处理请求(线程池上执行)。`null` 结果渲染为协议层的 `null`。
///
/// 内核查询是 `Cancellable`:并发编辑使快照失效时经 `?` 传播为
/// [`HandleError::Cancelled`] —— 绝不把过期计算当作有效结果返回。
#[allow(deprecated)] // SymbolInformation.deprecated 仍是 LSP 3.17 的组成部分
fn handle(snap: &Snapshot, req: &RequestKind) -> Result<serde_json::Value, HandleError> {
    let analysis = &snap.analysis;
    let result: Option<serde_json::Value> = match req {
        RequestKind::Hover(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let doc = snap.doc(file).expect("hover 文件已注册");
            // 内核 hover;引擎无文档时,回扫声明名上方的 `##` 文档注释(与 Godot 4.4+ 悬停行为对齐)。
            let mut kernel = analysis.hover(FilePosition { file, offset })?;
            if let Some(h) = kernel.as_mut()
                && h.doc.is_empty()
            {
                let line = doc.index.position(&doc.text, h.range.start).line;
                if let Some(md) = text::doc_comment_above(&doc.text, &doc.index, line) {
                    h.doc = md;
                }
            }
            // 内核无结果或仍无文档:引擎全局面兜底(类名/单例/工具函数/常量/枚举值/注解),
            // 再退语言关键字;内核有文档时,追加官方 zh-CN 文档链接(文档跳转)。
            if kernel.as_ref().is_none_or(|h| h.doc.is_empty()) {
                let fallback = globals_hover_at(&doc.text, offset)
                    .or_else(|| keyword_hover_at(&doc.text, offset))
                    .or_else(|| node_path_hover_at(&doc.text, offset))
                    .or_else(|| operator_hover_at(&doc.text, offset));
                if let Some(md) = fallback {
                    let word = cv::word_range_at(&doc.text, offset);
                    return Ok(cv::to_value(&lsp_types::Hover {
                        contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: md,
                        }),
                        range: word.map(|r| doc.index.range(&doc.text, r)),
                    }));
                }
            } else if let Some(h) = kernel.as_mut()
                && let Some(link) = globals_hover_link(&doc.text, offset)
            {
                h.doc.push_str(&format!("\n\n[📖 中文文档]({link})"));
            }
            let Some(h) = kernel else {
                return Ok(serde_json::Value::Null);
            };
            Some(cv::to_value(&cv::hover(&doc.text, &doc.index, h)))
        }
        RequestKind::Definition(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let targets = analysis.goto_definition(FilePosition { file, offset })?;
            Some(cv::to_value(&cv::nav_targets(targets, &snap.resolver())))
        }
        RequestKind::Completion(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            let items = analysis.completions(FilePosition { file, offset })?;
            let mut items = cv::completions(items);
            // 按名上下文(前字符非 `.`/`@`)合并引擎全局面:
            // 类、内建类型、单例、工具/内建函数、全局枚举及值、常量。
            let doc = snap.doc(file).expect("completion 文件已注册");
            if by_name_context(&doc.text, offset) {
                items.extend(globals::completion_items().iter().cloned());
            }
            enrich_completions(snap, file, &doc.text, &doc.index, offset, &mut items)?;
            Some(cv::to_value(&lsp_types::CompletionResponse::Array(items)))
        }
        RequestKind::DocumentSymbol(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let syms = analysis.document_symbols(file)?;
            let doc = snap.doc(file).expect("symbol 文件已注册");
            Some(cv::to_value(&lsp_types::DocumentSymbolResponse::Nested(
                cv::document_symbols(&doc.text, &doc.index, syms),
            )))
        }
        RequestKind::References(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            let refs = analysis.find_references(FilePosition { file, offset })?;
            Some(cv::to_value(&cv::references(
                refs,
                p.context.include_declaration,
                &snap.resolver(),
            )))
        }
        RequestKind::DocumentHighlight(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = snap.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let refs = analysis.find_references(FilePosition { file, offset })?;
            // 只取本文档内的出现位置,编辑器据此高亮光标符号。
            // 旧内核 TextRange 起点锚行首(吞缩进),经 clamp_word_range
            // 收缩回词段——修在服务器,所有 IDE 客户端同时受益。
            let highlights: Vec<lsp_types::DocumentHighlight> = refs
                .iter()
                .filter(|r| r.file == file)
                .map(|r| lsp_types::DocumentHighlight {
                    range: cv::clamp_word_range(&doc.text, &doc.index, doc.index.range(&doc.text, r.range)),
                    kind: Some(lsp_types::DocumentHighlightKind::TEXT),
                })
                .collect();
            Some(cv::to_value(&highlights))
        }
        RequestKind::DocumentLink(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = snap.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(root) = snap.root.clone() else {
                return Ok(serde_json::Value::Null);
            };
            Some(cv::to_value(&text::document_links(&doc.text, &root)))
        }
        RequestKind::DocumentColor(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = snap.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            Some(cv::to_value(&text::document_colors(&doc.text)))
        }
        RequestKind::ColorPresentation(p) => Some(cv::to_value(&text::color_presentations(&p.color))),
        RequestKind::Rename(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            match analysis.rename(FilePosition { file, offset }, &p.new_name) {
                Ok(Ok(change)) => match cv::workspace_edit(change, &snap.resolver()) {
                    Some(edit) => Some(cv::to_value(&edit)),
                    None => return Err(HandleError::Msg("重命名编辑无法映射".into())),
                },
                Ok(Err(e)) => return Err(HandleError::Msg(format!("无法重命名:{e:?}"))),
                Err(c) => return Err(HandleError::from(c)),
            }
        }
        RequestKind::PrepareRename(p) => {
            let Some((file, offset)) = position(snap, p) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = snap.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            cv::word_range_at(&doc.text, offset)
                .map(|r| {
                    let placeholder = doc.text[r.start as usize..r.end as usize].to_string();
                    cv::to_value(&lsp_types::PrepareRenameResponse::RangeWithPlaceholder {
                        range: doc.index.range(&doc.text, r),
                        placeholder,
                    })
                })
                .or(Some(serde_json::Value::Null))
        }
        RequestKind::WorkspaceSymbol(p) => {
            let targets = analysis.workspace_symbols(&p.query)?;
            // 直接读快照注册表:按 FileId 逐个克隆全文再建行索引是 O(n·文件大小) 的浪费。
            let infos: Vec<lsp_types::SymbolInformation> = targets
                .iter()
                .filter_map(|t| {
                    let uri = snap.uri_of(t.file)?.parse::<Uri>().ok()?;
                    let doc = snap.doc(t.file)?;
                    Some(lsp_types::SymbolInformation {
                        name: t.name.clone(),
                        kind: symbol_kind_of(t.kind),
                        tags: None,
                        deprecated: None,
                        location: lsp_types::Location {
                            uri,
                            range: doc.index.range(&doc.text, t.focus_range),
                        },
                        container_name: None,
                    })
                })
                .collect();
            Some(cv::to_value(&infos))
        }
        RequestKind::SemanticTokens(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let tokens = analysis.semantic_tokens(file)?;
            let doc = snap.doc(file).expect("token 文件已注册");
            Some(cv::to_value(&cv::semantic_tokens(&doc.text, &doc.index, tokens)))
        }
        RequestKind::InlayHint(p) => {
            if !snap.inlay_hints {
                return Ok(serde_json::Value::Null);
            }
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let hints = analysis.inlay_hints(file)?;
            let doc = snap.doc(file).expect("hint 文件已注册");
            Some(cv::to_value(&cv::inlay_hints(&doc.text, &doc.index, hints)))
        }
        RequestKind::FoldingRange(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let ranges = analysis.folding_ranges(file)?;
            let doc = snap.doc(file).expect("fold 文件已注册");
            Some(cv::to_value(&cv::folding_ranges(&doc.text, &doc.index, ranges)))
        }
        RequestKind::SignatureHelp(p) => {
            let Some((file, offset)) = position(snap, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(mut help) = analysis.signature_help(FilePosition { file, offset })? else {
                return Ok(serde_json::Value::Null);
            };
            let doc = snap.doc(file).expect("signature help 文件已注册");
            enrich_signature_docs(snap, file, &doc.text, offset, &mut help)?;
            Some(cv::to_value(&cv::signature_help(help)))
        }
        RequestKind::CodeAction(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = snap.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let offset = doc.index.offset(&doc.text, p.range.start);
            let actions = analysis.code_actions(FilePosition { file, offset })?;
            Some(cv::to_value(&cv::code_actions(actions, &snap.resolver())))
        }
        RequestKind::Formatting(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(new_text) = analysis.format(file)? else {
                return Ok(serde_json::Value::Null);
            };
            let doc = snap.doc(file).expect("format 文件已注册");
            Some(cv::to_value(&cv::format_edits(&doc.text, &doc.index, new_text)))
        }
        RequestKind::RangeFormatting(p) => {
            let Some(file) = file_id(snap, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = snap.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let start = doc.index.offset(&doc.text, p.range.start);
            let end = doc.index.offset(&doc.text, p.range.end);
            let Some((s, e, new_text)) = analysis.format_range(file, start, end)? else {
                return Ok(serde_json::Value::Null);
            };
            let edits = vec![lsp_types::TextEdit {
                range: doc.index.range(&doc.text, TextRange::new(s, e)),
                new_text,
            }];
            Some(cv::to_value(&edits))
        }
    };
    Ok(result.unwrap_or(serde_json::Value::Null))
}

/// 派发一次诊断计算到线程池:快照 → 计算(计时)→ 结果经通道回主循环。
async fn spawn_diagnostics(
    snap: Snapshot,
    generation: u64,
    uri: String,
    file: FileId,
    diag_tx: &UnboundedSender<DiagMsg>,
    inflight: &Arc<Inflight>,
    permits: &Arc<Semaphore>,
) {
    // 先用快照(同步),再等并发许可:跨 await 的只有 Send 的 Snapshot 与 permit。
    let version = snap.doc(file).and_then(|d| d.version);
    // 并发上限与请求共用同一池(等待有界,理由同 dispatch)。
    let Ok(permit) = permits.clone().acquire_owned().await else {
        return;
    };
    inflight.enter();
    let diag_tx = diag_tx.clone();
    let inflight = Arc::clone(inflight);
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        let t0 = std::time::Instant::now();
        let result = snap.analysis.diagnostics(file).map(|diags| {
            // 客户端忽略表:按 Godot 编辑器同款诊断码(`UNUSED_VARIABLE` 等)过滤。
            let diags: Vec<_> = if snap.diag_ignore.is_empty() {
                diags
            } else {
                diags
                    .into_iter()
                    .filter(|d| !snap.diag_ignore.iter().any(|c| c == &d.code))
                    .collect()
            };
            let doc = snap.doc(file).expect("诊断文件已注册");
            cv::diagnostics(&doc.text, &doc.index, &diags)
        });
        let duration_ms = t0.elapsed().as_millis() as u64;
        let _ = diag_tx.send(DiagMsg { uri, generation, version, duration_ms, result });
        inflight.leave();
    });
}

#[allow(clippy::needless_pass_by_value)]
fn symbol_kind_of(k: gdscript_base::SymbolKind) -> lsp_types::SymbolKind {
    match k {
        gdscript_base::SymbolKind::Class => lsp_types::SymbolKind::CLASS,
        gdscript_base::SymbolKind::Function => lsp_types::SymbolKind::FUNCTION,
        gdscript_base::SymbolKind::Method => lsp_types::SymbolKind::METHOD,
        gdscript_base::SymbolKind::Variable => lsp_types::SymbolKind::VARIABLE,
        gdscript_base::SymbolKind::Constant => lsp_types::SymbolKind::CONSTANT,
        gdscript_base::SymbolKind::Enum => lsp_types::SymbolKind::ENUM,
        gdscript_base::SymbolKind::EnumMember => lsp_types::SymbolKind::ENUM_MEMBER,
        gdscript_base::SymbolKind::Signal => lsp_types::SymbolKind::EVENT,
    }
}

fn file_id(snap: &Snapshot, uri: &Uri) -> Option<FileId> {
    snap.id_of(uri.as_str())
}

/// 文本文档位置对应的 `(FileId, 字节偏移)`,未注册则 `None`。
fn position(snap: &Snapshot, p: &lsp_types::TextDocumentPositionParams) -> Option<(FileId, u32)> {
    let file = file_id(snap, &p.text_document.uri)?;
    let doc = snap.doc(file)?;
    let offset = doc.index.offset(&doc.text, p.position);
    Some((file, offset))
}

/// 光标是否处于"按名补全"上下文:标识符之前不是 `.`(成员)、`@`(注解),
/// 也不在 `$`/`%` 取节点路径内(节点路径补全由内核独占,合并全局面会污染)。
fn by_name_context(text: &str, offset: u32) -> bool {
    let b = text.as_bytes();
    let mut ws = (offset as usize).min(b.len());
    while ws > 0 && (b[ws - 1].is_ascii_alphanumeric() || b[ws - 1] == b'_') {
        ws -= 1;
    }
    !matches!(ws.checked_sub(1).map(|i| b[i]), Some(b'.') | Some(b'@'))
        && !in_node_path_context(text, offset)
}

/// 光标是否在 `$Path/Seg` / `%Unique/Seg` 取节点路径内。镜像内核
/// `dollar_path_prefix`/`unique_path_prefix` 的向后字节扫描(无词法器感知,
/// 字符串/注释内的误报与内核同级,内核侧还会再以解析树复核)。
fn in_node_path_context(text: &str, offset: u32) -> bool {
    let b = text.as_bytes();
    let is_seg = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = (offset as usize).min(b.len());
    while i > 0 && is_seg(b[i - 1]) {
        i -= 1;
    }
    loop {
        match i.checked_sub(1).map(|k| b[k]) {
            Some(b'$') => return true,
            // `%` 兼作取模运算符:前一字符是操作数(标识符/右括号)时按取模处理,
            // 不视为路径(内核侧由解析树精确判定,此处仅为合并抑制的保守近似)。
            Some(b'%') => {
                return !i
                    .checked_sub(2)
                    .map(|k| matches!(b[k], b')' | b']') || is_seg(b[k]))
                    .unwrap_or(false);
            }
            Some(b'/') => {
                let mut s = i - 1;
                while s > 0 && is_seg(b[s - 1]) {
                    s -= 1;
                }
                i = s;
            }
            _ => return false,
        }
    }
}

/// 光标处裸标识符的引擎全局面 hover(单例/类/工具函数/常量/枚举值/注解)。
fn globals_hover_at(text: &str, offset: u32) -> Option<String> {
    let range = cv::word_range_at(text, offset)?;
    globals::hover_markdown(&text[range.start as usize..range.end as usize])
}

/// 光标处裸标识符的官方文档链接(命中引擎全局面时)。
fn globals_hover_link(text: &str, offset: u32) -> Option<String> {
    let range = cv::word_range_at(text, offset)?;
    globals::doc_link(&text[range.start as usize..range.end as usize])
}

/// 光标处裸标识符的语言关键字 hover(内核与全局面均未命中时兜底)。
fn keyword_hover_at(text: &str, offset: u32) -> Option<String> {
    let range = cv::word_range_at(text, offset)?;
    globals::keyword_hover(&text[range.start as usize..range.end as usize])
}

/// 节点路径操作符 hover:`$NodePath` / `%Name`(或光标落在 `$`/`%` 前缀本身)。
/// `%` 后紧跟标识符视为唯一节点,否则当作取模运算符交给 `operator_hover_at`。
fn node_path_hover_at(text: &str, offset: u32) -> Option<String> {
    let b = text.as_bytes();
    let o = offset as usize;
    if o == 0 || o > b.len() {
        return None;
    }
    let at = b.get(o).copied();
    let prev = b.get(o.wrapping_sub(1)).copied();
    if at == Some(b'%') || prev == Some(b'%') {
        let pct = if at == Some(b'%') { o } else { o - 1 };
        let next = b.get(pct + 1).copied();
        if next.is_some_and(|c| c == b'_' || c.is_ascii_alphabetic()) {
            return globals::node_path_hover('%');
        }
        return globals::operator_hover("%");
    }
    if at == Some(b'$') || prev == Some(b'$') {
        return globals::node_path_hover('$');
    }
    None
}

/// 符号操作符 hover:`??`/`==`/`**`/`%`/…(光标落在运算符标点本身时)。
fn operator_hover_at(text: &str, offset: u32) -> Option<String> {
    let b = text.as_bytes();
    let o = offset as usize;
    if o == 0 || o > b.len() {
        return None;
    }
    let mut s = o;
    while s > 0 && is_op_char(b[s - 1]) {
        s -= 1;
    }
    let mut e = o;
    while e < b.len() && is_op_char(b[e]) {
        e += 1;
    }
    if s == e {
        return None;
    }
    globals::operator_hover(&text[s..e])
}

/// 运算符标点字符(用于 `operator_hover_at` 的连续标点切分;不含结构性 `.()[]`,`)。
fn is_op_char(c: u8) -> bool {
    matches!(c, b'+' | b'-' | b'*' | b'/' | b'%' | b'<' | b'>' | b'=' | b'!' | b'&' | b'|' | b'^' | b'~' | b'?')
}

/// 补全项增强(服务器侧,不动内核):
/// - 注解项:一行说明 + 完整官方文档;
/// - 成员上下文:接收者类型经引擎模型回填完整签名/类型与文档;
/// - 仍缺 detail 的项(自有成员):声明行原文 + 声明上方 `##` 文档注释。
fn enrich_completions(
    snap: &Snapshot,
    file: FileId,
    text: &str,
    index: &cv::LineIndex,
    offset: u32,
    items: &mut [lsp_types::CompletionItem],
) -> Result<(), Cancelled> {
    // 注解(内核 label 不带 `@`,kind 映射为 PROPERTY 且 detail 恒空)。
    for it in items.iter_mut() {
        if it.detail.is_none()
            && it.kind == Some(lsp_types::CompletionItemKind::PROPERTY)
            && let Some((detail, doc)) = globals::annotation_info(&it.label)
        {
            it.detail = Some(detail.to_owned());
            it.documentation = Some(markdown_doc(doc));
        }
    }

    // 成员上下文:接收者类型 → 完整签名/类型 + 文档。
    if let Some(ty) = member_receiver_type(snap, file, text, offset)? {
        for it in items.iter_mut() {
            if let Some((detail, doc)) = globals::member_info(&ty, &it.label) {
                it.detail = Some(detail);
                it.documentation = Some(markdown_doc(doc));
            }
        }
    }

    // 自有成员兜底:声明行原文 + 上方 `##` 文档注释。
    if items.iter().any(|it| it.detail.is_none()) {
        let syms = snap.analysis.document_symbols(file)?;
        let mut decls: Vec<(&str, u32)> = Vec::new();
        collect_decls(&syms, &mut decls);
        for it in items.iter_mut() {
            if it.detail.is_some() {
                continue;
            }
            if let Some(&(_, start)) = decls.iter().find(|(n, _)| *n == it.label) {
                // 用 selection_range(名字)而非 range:内核 decl range 可能回吞脚本头。
                let line = index.position(text, start).line;
                let (s, e) = index.line_span(line);
                let raw = text[s..e].trim();
                // 函数/类声明行以 `:` 收尾,去掉以呈现签名本体。
                it.detail = Some(raw.strip_suffix(':').unwrap_or(raw).trim_end().to_owned());
                if let Some(md) = text::doc_comment_above(text, index, line) {
                    it.documentation = Some(markdown_doc(md));
                }
            }
        }
    }
    Ok(())
}

/// 递归收集文档符号的 (名字, 声明行锚点=名字起点)。
fn collect_decls<'a>(syms: &'a [gdscript_base::DocumentSymbol], out: &mut Vec<(&'a str, u32)>) {
    for s in syms {
        out.push((&s.name, s.selection_range.start));
        collect_decls(&s.children, out);
    }
}

fn markdown_doc(value: String) -> lsp_types::Documentation {
    lsp_types::Documentation::MarkupContent(lsp_types::MarkupContent {
        kind: lsp_types::MarkupKind::Markdown,
        value,
    })
}

/// 成员上下文 `recv.` 的接收者类型名(内核 hover 的 `ty_label`)。
/// `offset` 为补全位置(紧跟 `.` 之后)。
fn member_receiver_type(
    snap: &Snapshot,
    file: FileId,
    text: &str,
    offset: u32,
) -> Result<Option<String>, Cancelled> {
    let bytes = text.as_bytes();
    let mut i = (offset as usize).min(bytes.len());
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    // 须紧跟 `.` 且点前有表达式字符(排除行首点与 `..`)。
    if i < 2 || bytes[i - 1] != b'.' || bytes[i - 2] == b'.' {
        return Ok(None);
    }
    // hover 接收者表达式的末字符,取其类型标签。
    let h = snap
        .analysis
        .hover(gdscript_base::FilePosition {
            file,
            offset: i as u32 - 2,
        })?;
    Ok(h.and_then(|h| h.ty_label))
}

/// 签名帮助文档回填(P0-2):内核签名恒无 doc;按调用名回填模型/文档表,
/// 成员调用(`recv.method(`)经接收者 hover 定位类后取成员文档。
fn enrich_signature_docs(
    snap: &Snapshot,
    file: FileId,
    text: &str,
    offset: u32,
    help: &mut gdscript_base::SignatureHelp,
) -> Result<(), Cancelled> {
    for sig in &mut help.signatures {
        if !sig.doc.is_empty() {
            continue;
        }
        let Some(name) = sig.label.split('(').next().map(str::trim) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        if let Some(md) =
            callee_member_doc(snap, file, text, offset, name)?.or_else(|| globals::signature_doc(name))
        {
            sig.doc = md;
        }
    }
    Ok(())
}

/// `recv.name(` 形态的成员调用文档;非成员调用返回 None。
fn callee_member_doc(
    snap: &Snapshot,
    file: FileId,
    text: &str,
    offset: u32,
    name: &str,
) -> Result<Option<String>, Cancelled> {
    let bytes = text.as_bytes();
    let cur = (offset as usize).min(bytes.len());
    // 回溯光标前最内层未闭合的 `(`。
    let mut depth = 0i32;
    let mut open = None;
    for i in (0..cur).rev() {
        match bytes[i] {
            b')' => depth += 1,
            b'(' => {
                if depth == 0 {
                    open = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let Some(open) = open else {
        return Ok(None);
    };
    // `(` 前恰为调用名,且调用名前是 `.`(成员调用)。
    if !text[..open].ends_with(name) {
        return Ok(None);
    }
    let Some(dot) = open.checked_sub(name.len() + 1) else {
        return Ok(None);
    };
    if dot == 0 || bytes[dot] != b'.' {
        return Ok(None);
    }
    // 接收者末字符 hover → 类型 → 成员文档。
    let Some(h) = snap
        .analysis
        .hover(gdscript_base::FilePosition {
            file,
            offset: dot as u32 - 1,
        })?
    else {
        return Ok(None);
    };
    let Some(ty) = h.ty_label else {
        return Ok(None);
    };
    Ok(globals::member_method_doc(&ty, name))
}

async fn on_notification(
    sender: &ClientSender,
    state: &mut State,
    sched: &mut DiagScheduler,
    inflight: &Inflight,
    notif: Notification,
) {
    match notif.method.as_str() {
        "textDocument/didOpen" => {
            if let Ok(p) =
                serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(notif.params.unwrap_or_default())
            {
                let uri = p.text_document.uri.to_string();
                // 写前静默:salsa 0.27 要求写时无存活快照句柄。
                quiesce(inflight).await;
                // 项目配置自愈:客户端的文件监视可能漏推 project.godot 变更
                // (典型:会话启动后才在 Godot 里添加 autoload)。打开文档时按
                // mtime 比对补刀,保证全局解析面(autoload 注册表)及时生效。
                let config_changed = state.refresh_project_config_if_stale();
                state.open(&uri, p.text_document.text, Some(p.text_document.version));
                sched.touch(&uri, Instant::now());
                if config_changed {
                    // 全局面变更:其余已打开文档也要重算(配置对所有人生效)。
                    for u in state.open_uris() {
                        if u != uri {
                            sched.touch(&u, Instant::now());
                        }
                    }
                }
            }
        }
        "textDocument/didChange" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(
                notif.params.unwrap_or_default(),
            )
            {
                let uri = p.text_document.uri.to_string();
                let version = Some(p.text_document.version);
                if p.content_changes.is_empty() {
                    return;
                }
                quiesce(inflight).await;
                if let Some(file) = state.id_of(&uri) {
                    // 已注册:增量应用 range 编辑。
                    if !state.apply_changes(file, p.content_changes, version) {
                        return;
                    }
                } else {
                    // 未注册的兜底:取最后一个全文变更按打开处理。
                    let Some(text) = p.content_changes.into_iter().next_back().map(|c| c.text)
                    else {
                        return;
                    };
                    state.open(&uri, text, version);
                }
                // 去抖调度:同一 uri 的连续编辑天然合并到最新代际。
                sched.touch(&uri, Instant::now());
            }
        }
        "textDocument/didClose" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(
                notif.params.unwrap_or_default(),
            )
            {
                let uri = p.text_document.uri.to_string();
                // 关闭后以磁盘内容为准刷新(未保存的编辑丢弃),保持项目视图一致;
                // 文件已从磁盘消失则移出分析世界。
                quiesce(inflight).await;
                state.watched(&uri);
                if state.id_of(&uri).is_some() {
                    sched.touch(&uri, Instant::now());
                }
            }
        }
        "workspace/didChangeConfiguration" => {
            // set_warning_override 是一次写:先等在飞任务退出。
            quiesce(inflight).await;
            apply_settings(state, &notif.params.unwrap_or_default());
        }
        "workspace/didChangeWatchedFiles" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidChangeWatchedFilesParams>(
                notif.params.unwrap_or_default(),
            )
            {
                // 每个事件都可能写(watched/inject/remove)。
                quiesce(inflight).await;
                let mut config_changed = false;
                for event in p.changes {
                    let uri = event.uri.to_string();
                    // project.godot 重载改变全局解析面(autoload 注册表/引擎版本/
                    // 警告设置):仅重载不重发会让陈旧诊断挂到下次编辑 —— 合法的
                    // autoload 单例标识符会被持续误报为未声明。
                    if state.watched(&uri) {
                        config_changed = true;
                    }
                    // 磁盘变化同步诊断:仍注册的 .gd 去抖重算;已删除的推空清单,
                    // 让客户端清掉残留标记。
                    if uri.ends_with(".gd") {
                        if state.id_of(&uri).is_some() {
                            sched.touch(&uri, Instant::now());
                        } else {
                            publish(sender, &uri, Vec::new(), None);
                        }
                    }
                }
                if config_changed {
                    // 全局面变更:重算**全部**已打开文档(不只是触发事件的文件)。
                    for uri in state.open_uris() {
                        sched.touch(&uri, Instant::now());
                    }
                }
            }
        }
        _ => {}
    }
}

/// 客户端配置(与 VSCode `godotlsp.*` 注册项一一对应)。
#[derive(serde::Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ClientSettings {
    diagnostics: Option<DiagSettings>,
    inlay_hints: Option<InlayHintSettings>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct DiagSettings {
    strict: Option<bool>,
    /// 忽略的诊断码列表(Godot 编辑器同款,如 `UNUSED_VARIABLE`)。
    ignore: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct InlayHintSettings {
    type_hints: Option<bool>,
}

/// 应用 `didChangeConfiguration` 载荷。`vscode-languageclient` 以
/// `{ godotlsp: {…} }` 形态推送;兼容把配置节直接内嵌的客户端。
/// 注意:警告严格度变化不立即重推全项目诊断,由后续编辑/打开自然刷新。
fn apply_settings(state: &mut State, params: &serde_json::Value) {
    let settings = params.get("settings").unwrap_or(params);
    let scoped = settings.get(SECTION).unwrap_or(settings);
    let Ok(s) = serde_json::from_value::<ClientSettings>(scoped.clone()) else {
        return;
    };
    if let Some(d) = s.diagnostics {
        if let Some(strict) = d.strict {
            state.host.set_warning_override(if strict {
                WarningOverride::Strict
            } else {
                WarningOverride::None
            });
        }
        if let Some(ignore) = d.ignore {
            state.diag_ignore = Arc::new(ignore);
        }
    }
    if let Some(i) = s.inlay_hints
        && let Some(t) = i.type_hints
    {
        state.inlay_hints = t;
    }
}

fn publish(
    sender: &ClientSender,
    uri: &str,
    diags: Vec<lsp_types::Diagnostic>,
    version: Option<i32>,
) {
    let Ok(url) = uri.parse::<Uri>() else {
        return;
    };
    let params = PublishDiagnosticsParams {
        uri: url,
        diagnostics: diags,
        version,
    };
    let _ = sender.notify(
        "textDocument/publishDiagnostics",
        Some(cv::to_value(&params)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_server_tokio::duplex_transport;

    #[test]
    fn node_path_context_detection() {
        let t = "var p = $Panel/
var q = %Sprite
var r = 7 % 3
var s = a%b
";
        let l2 = t.find("%").unwrap();
        let l3 = t.find("7 % 3").unwrap();
        let l4 = t.find("a%b").unwrap();
        // $ 路径:"var p = $Panel/"
        assert!(in_node_path_context(t, 9), "$ 后");
        assert!(in_node_path_context(t, 12), "$Pa 中");
        assert!(in_node_path_context(t, 14), "$Panel 尾");
        assert!(in_node_path_context(t, 15), "$Panel/ 后");
        assert!(!in_node_path_context(t, 8), "$ 之前");
        assert!(!in_node_path_context(t, 4), "普通标识符 var");
        // % unique:"var q = %Sprite"
        assert!(in_node_path_context(t, (l2 + 1) as u32), "% 后");
        // 取模:"7 % 3" — 光标在结果操作数之后,不得视为路径
        assert!(!in_node_path_context(t, (l3 + 5) as u32), "取模结果后");
        // a%b:% 前是操作数 → 取模
        assert!(!in_node_path_context(t, (l4 + 3) as u32), "a%b 的 b 后");
    }

    #[test]
    fn node_path_and_operator_hover_wiring() {
        // 光标落在 `$` 本身 → 节点路径(get_node)文档。
        let s1 = "var p = $Panel";
        let h = node_path_hover_at(s1, 8).unwrap(); // 8 是 `$` 的位置
        assert!(h.contains("get_node"), "{h}");

        // 光标落在 `$` 后的首字符(其前为 `$`)→ 仍识别为节点路径。
        assert!(node_path_hover_at(s1, 9).unwrap().contains("get_node"));

        // 光标落在唯一节点 `%Name` 的 `%` 上 → 唯一节点文档。
        let s2 = "var q = %Sprite";
        let h = node_path_hover_at(s2, 8).unwrap();
        assert!(h.contains("唯一节点"), "{h}");

        // 取模 `7 % 3`:`%` 后是操作数(数字)→ 当作取模运算符,非节点路径。
        let s3 = "var r = 7 % 3";
        let h = node_path_hover_at(s3, 10).unwrap(); // 10 是 `%` 的位置
        assert!(h.contains("取模"), "{h}");

        // 符号操作符:`??` / `==` 等。
        let s4 = "var x = a ?? b";
        let h = operator_hover_at(s4, 10).unwrap(); // 10 是第一个 `?`
        assert!(h.contains("??") && h.contains("空合并"), "{h}");

        let s5 = "if a == b";
        assert!(operator_hover_at(s5, 5).unwrap().contains("=="));

        // 普通标识符不触发操作符 hover。
        assert!(operator_hover_at("var x", 4).is_none());
    }

    /// 诊断调度器:去抖合并、自适应下限/上限、代际过期。
    #[test]
    fn diag_scheduler_debounce_and_generations() {
        let mut s = DiagScheduler::new();
        let t0 = Instant::now();

        // 无样本 → 下限 50ms。
        assert_eq!(s.debounce(), std::time::Duration::from_millis(50));

        s.touch("a", t0);
        s.touch("b", t0);
        // 未到期:不派发。
        assert!(s.take_due(t0).is_empty());
        // 到期(50ms 后):两个都派发,且各自携带调度时的代际。
        let due = s.take_due(t0 + std::time::Duration::from_millis(51));
        let mut got: Vec<(String, u64)> = due;
        got.sort();
        assert_eq!(got, vec![("a".into(), 0), ("b".into(), 1)]);

        // 派发后同一 uri 再编辑 → 代际推进,旧代际结果过期。
        s.touch("a", t0);
        assert!(s.is_current("a", 2));
        assert!(!s.is_current("a", 0), "旧代际应被视为过期");
        assert!(s.is_current("b", 1));

        // 自适应去抖:中位数 clamp 到 [50, 500]。
        for ms in [10, 20, 30] {
            s.record_duration(ms);
        }
        assert_eq!(s.debounce(), std::time::Duration::from_millis(50), "中位数 20 → 下限");
        for ms in [400, 600, 800, 1000] {
            s.record_duration(ms);
        }
        // 中位数 400 → 400ms(区间内)。
        assert_eq!(s.debounce(), std::time::Duration::from_millis(400));
        for ms in [900; 12] {
            s.record_duration(ms);
        }
        assert_eq!(s.debounce(), std::time::Duration::from_millis(500), "中位数 900 → 上限");
    }

    /// 协议层生命周期闭环(异步 stdio + `duplex_transport` 内存测试):
    /// 客户端发起 initialize → initialized → shutdown → exit,服务端应干净终止。
    ///
    /// 注意:`ClientSender::request` 的响应经 **route()** 投递给 pending oneshot,
    /// 因此客户端侧必须有一个后台泵循环调用 `route()`,否则请求永远挂起。
    #[tokio::test]
    async fn lifecycle_over_duplex() {
        let (client_t, server_t) = duplex_transport(4096);
        let server = Connection::from_transport(server_t);
        let task = tokio::spawn(async move { run_on(server).await });

        // 客户端:后台泵 receiver,把响应路由给等待中的 request future。
        let mut client: Connection<_, ()> = Connection::from_transport(client_t);
        let cs = client.client_sender();
        let pump = tokio::spawn(async move {
            while let Some(Ok(msg)) = client.receiver.next().await {
                let _ = client.route(msg);
            }
        });

        let caps = serde_json::json!({ "capabilities": {}, "rootUri": null });
        let t = std::time::Duration::from_secs(10);
        tokio::time::timeout(t, cs.request("initialize", Some(caps)))
            .await
            .expect("initialize 超时")
            .expect("initialize 请求失败");
        let _ = cs.notify("initialized", None);
        // 关闭握手:shutdown + exit。
        tokio::time::timeout(t, cs.request("shutdown", None))
            .await
            .expect("shutdown 超时")
            .expect("shutdown 请求失败");
        let _ = cs.notify("exit", None);

        let finished = tokio::time::timeout(t, task).await;
        assert!(finished.is_ok(), "服务器未在 10s 内随 exit 终止");
        assert!(finished.unwrap().is_ok(), "run_on 返回了错误");
        pump.abort();
    }
}
