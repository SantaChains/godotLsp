//! 回归测试:会话中途增删 autoload(`project.godot` 变更)后,已打开文档的
//! 陈旧诊断必须被重算重发。
//!
//! 背景(真实误报):用户在 Godot 编辑器配好 autoload 单例后,VS Code 里已
//! 打开的文档仍持续报 `Identifier "GameConfig" not declared in the current
//! scope` —— 服务器重载了项目配置,却没有重发诊断,错误一直挂到下次编辑。
//! 本测试锁定两条自愈路径:
//! - Phase A:`didChangeWatchedFiles` 事件 → 全局面重算(主路径);
//! - Phase B:无 watcher 事件,仅靠 `didOpen` 时的 mtime 自愈(兜底路径,
//!   覆盖不推送文件事件的 LSP 客户端)。

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

const TIMEOUT: Duration = Duration::from_secs(20);

struct Server {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<Value>,
    /// 全部收到的消息摘要(超时诊断用)。
    log: Vec<String>,
}

impl Server {
    fn spawn(root: &Path) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_godotlsp"))
            .arg("--stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("启动服务器失败");
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut stdout = stdout;
            loop {
                let mut len = None;
                let mut line = String::new();
                loop {
                    line.clear();
                    if stdout.read_line(&mut line).unwrap() == 0 {
                        return;
                    }
                    let line = line.trim_end();
                    if line.is_empty() {
                        break;
                    }
                    if let Some(v) = line.strip_prefix("Content-Length:") {
                        len = Some(v.trim().parse::<usize>().unwrap());
                    }
                }
                let Some(len) = len else { continue };
                let mut body = vec![0u8; len];
                std::io::Read::read_exact(&mut stdout, &mut body).unwrap();
                if let Ok(msg) = serde_json::from_slice::<Value>(&body)
                    && tx.send(msg).is_err()
                {
                    return;
                }
            }
        });
        let mut s = Self { child, stdin, rx, log: Vec::new() };
        s.send(&json!({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": file_uri(root),
                "capabilities": {},
            }
        }));
        let resp = s.recv_timeout().expect("等待 initialize 响应超时");
        assert_eq!(resp.get("id").and_then(Value::as_u64), Some(1), "响应 id 不匹配: {resp}");
        s.send(&json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }));
        // 等 load_project + 引擎模型就绪。
        thread::sleep(Duration::from_millis(1500));
        s
    }

    fn send(&mut self, msg: &Value) {
        let body = msg.to_string();
        write!(self.stdin, "Content-Length: {}\r\n\r\n{body}", body.len()).unwrap();
        self.stdin.flush().unwrap();
    }

    fn recv_timeout(&mut self) -> Option<Value> {
        let msg = self.rx.recv_timeout(TIMEOUT).ok();
        if let Some(m) = &msg {
            let method = m.get("method").and_then(Value::as_str).unwrap_or("<response>");
            let uri = m["params"]["uri"].as_str().unwrap_or("-");
            let diags = m["params"]["diagnostics"].as_array().map(|a| a.len()).unwrap_or(0);
            self.log.push(format!("{method} {uri} ndiags={diags}"));
        }
        msg
    }

    fn did_open(&mut self, path: &Path) {
        let uri = file_uri(path);
        let text = std::fs::read_to_string(path).unwrap();
        self.send(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": { "textDocument": {
                "uri": uri, "languageId": "gdscript", "version": 1, "text": text,
            } }
        }));
    }

    /// 等待一组 `(uri, declared)` 条件全部被**本次调用之后到达**的发布满足。
    /// 同批次的多文档重发顺序不定(`take_due` 走 `HashMap` 迭代序),必须在
    /// 同一个读取循环里等齐——分多次等待会先到者的消息被后一个等待漏消费。
    fn wait_all(&mut self, phase: &str, conds: &[(&str, bool)]) {
        let mut done = vec![false; conds.len()];
        loop {
            if done.iter().all(|d| *d) {
                return;
            }
            let msg = self
                .recv_timeout()
                .unwrap_or_else(|| panic!("[{phase}] 等待 {conds:?} 超时;全部消息: {:?}", self.log));
            if msg.get("method").and_then(Value::as_str)
                == Some("textDocument/publishDiagnostics")
            {
                let got = msg["params"]["uri"].as_str().unwrap_or("");
                let has = msg["params"]["diagnostics"]
                    .as_array()
                    .is_some_and(|a| {
                        a.iter().any(|d| {
                            d["message"]
                                .as_str()
                                .is_some_and(|m| m.contains("not declared in the current scope"))
                        })
                    });
                for (i, (uri, declared)) in conds.iter().enumerate() {
                    if !done[i] && got == *uri && has == *declared {
                        done[i] = true;
                    }
                }
            }
        }
    }
}

/// panic 兜底:测试断言失败(unwrap/expect)同样必须回收子进程,
/// 否则残留的 godotlsp.exe 会占着后续测试的管道导致整轮 cargo test 挂起。
impl Drop for Server {
    fn drop(&mut self) {
        self.child.kill().ok();
        let _ = self.child.wait();
    }
}

/// `file:///D:/...` 形式 URI(三斜杠 + 最小百分号编码,与 `uri::path_to_uri` 兼容)。
fn file_uri(path: &Path) -> String {
    let p = path.to_string_lossy().replace('\\', "/").replace(' ', "%20");
    format!("file:///{p}")
}

fn copy_dir(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).unwrap();
    for e in std::fs::read_dir(src).unwrap().flatten() {
        let to = dst.join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            copy_dir(&e.path(), &to);
        } else {
            std::fs::copy(e.path(), to).unwrap();
        }
    }
}

/// 把 mtime 推到 10 秒后,击败文件系统粗糙的 mtime 粒度。
fn bump_mtime(path: &Path) {
    let f = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    f.set_modified(std::time::SystemTime::now() + Duration::from_secs(10))
        .unwrap();
}

const AUTOLOAD_LINE: &str = "GameConfig=\"*res://scripts/game_config.gd\"";

#[test]
fn autoload_added_mid_session_clears_stale_diagnostics() {
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/autoload");
    let dir = std::env::temp_dir().join(format!("godotlsp-autoload-e2e-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    copy_dir(&fixture, &dir);

    let main_uri = file_uri(&dir.join("main.gd"));
    let second_uri = file_uri(&dir.join("second.gd"));
    let mut server = Server::spawn(&dir);

    // Phase 0:初始无 autoload —— main.gd 必须报未声明。
    server.did_open(&dir.join("main.gd"));
    server.wait_all("phase0", &[(&main_uri, true)]);

    // Phase A:外部在 project.godot 追加 autoload + watcher 事件 →
    // main.gd 必须被重发为无该错误(不依赖任何文档编辑)。
    let cfg = dir.join("project.godot");
    let text = std::fs::read_to_string(&cfg).unwrap();
    std::fs::write(&cfg, format!("{text}\n[autoload]\n{AUTOLOAD_LINE}\n")).unwrap();
    bump_mtime(&cfg);
    server.send(&json!({
        "jsonrpc": "2.0", "method": "workspace/didChangeWatchedFiles",
        "params": { "changes": [{ "uri": file_uri(&cfg), "type": 2 }] }
    }));
    server.wait_all("phaseA", &[(&main_uri, false)]);

    // Phase B:无 watcher 事件,仅靠 didOpen 的 mtime 自愈 ——
    // 撤掉 autoload 后打开 second.gd,必须立即看到未声明;且 main.gd 的
    // 全画面陈旧(已清除)也被重发为报错。
    let text = std::fs::read_to_string(&cfg).unwrap();
    std::fs::write(
        &cfg,
        text.replace(&format!("\n[autoload]\n{AUTOLOAD_LINE}\n"), ""),
    )
    .unwrap();
    bump_mtime(&cfg);
    server.did_open(&dir.join("second.gd"));
    // second.gd 与 main.gd 在同批次被重发(顺序不定),必须同一循环等齐。
    server.wait_all("phaseB", &[(&second_uri, true), (&main_uri, true)]);

    server.child.kill().ok();
    let _ = std::fs::remove_dir_all(&dir);
}
