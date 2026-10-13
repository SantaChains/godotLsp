//! 端到端冒烟测试:以子进程方式启动服务器二进制,跑一遍完整协议会话。

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

macro_rules! step {
    ($($t:tt)*) => { eprintln!("[step] {}", format!($($t)*)) };
}

struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Server {
    fn spawn() -> Self {
        // 带 `--stdio` 启动,与 vscode-languageclient 的真实调用方式一致。
        let mut child = Command::new(env!("CARGO_BIN_EXE_godotlsp"))
            .arg("--stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("启动服务器失败");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Self { child, stdin, stdout }
    }

    fn send(&mut self, msg: &Value) {
        let body = msg.to_string();
        write!(self.stdin, "Content-Length: {}\r\n\r\n{body}", body.len()).unwrap();
        self.stdin.flush().unwrap();
    }

    /// 读取下一条消息(请求/响应/通知均可)。
    fn read(&mut self) -> Value {
        let mut len = None;
        loop {
            let mut line = String::new();
            self.stdout.read_line(&mut line).unwrap();
            let line = line.trim_end();
            if line.is_empty() {
                break;
            }
            if let Some(v) = line.strip_prefix("Content-Length:") {
                len = Some(v.trim().parse::<usize>().unwrap());
            }
        }
        let len = len.expect("缺少 Content-Length 头");
        let mut body = vec![0u8; len];
        std::io::Read::read_exact(&mut self.stdout, &mut body).unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    /// 读取 id 为 `id` 的下一条响应,跳过通知。
    fn response(&mut self, id: u64) -> Value {
        loop {
            let msg = self.read();
            if msg.get("id").and_then(Value::as_u64) == Some(id) {
                return msg;
            }
        }
    }

    /// 读取下一条携带至少一条诊断的 publishDiagnostics 通知。
    fn first_diagnostics(&mut self) -> Vec<Value> {
        loop {
            let msg = self.read();
            if msg["method"] == "textDocument/publishDiagnostics"
                && msg["params"]["diagnostics"].as_array().is_some_and(|d| !d.is_empty())
            {
                return msg["params"]["diagnostics"].as_array().unwrap().clone();
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

const URI: &str = "file:///smoke/main.gd";
const GOOD: &str = "extends Node\n\nvar count := 5\n\nfunc tick(delta: float) -> int:\n\tvar total := count + 1\n\treturn total\n";
const STRINGIFIED: &str = "extends Node\n\nvar count := \"hello\"\n\nfunc tick() -> String:\n\treturn count.\n";
const BROKEN: &str = "func f(:\n\tpass\n";

#[test]
fn full_stdio_session() {
    step!("spawn");
    let mut s = Server::spawn();
    let root = std::env::var("GODOTLSP_SMOKE_ROOT")
        .map(|r| format!("file:///{}/", r.replace('\\', "/")))
        .unwrap_or_else(|_| format!("file:///{}/", env!("CARGO_MANIFEST_DIR").replace('\\', "/")));

    // initialize → 能力集
    step!("initialize");
    s.send(&json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "processId": null, "rootUri": root,
            "capabilities": {},
        },
    }));
    let caps = s.response(1)["result"]["capabilities"].clone();
    assert_eq!(caps["positionEncoding"], "utf-16");
    assert_eq!(caps["textDocumentSync"], 2, "应为增量同步");
    assert_eq!(caps["hoverProvider"], Value::Bool(true));
    assert!(caps["completionProvider"]["triggerCharacters"].as_array().is_some_and(|a| !a.is_empty()));
    assert!(caps.get("semanticTokensProvider").is_some());
    assert!(caps.get("documentSymbolProvider").is_some());
    assert!(caps.get("renameProvider").is_some());
    assert!(caps.get("inlayHintProvider").is_some());
    assert!(caps.get("foldingRangeProvider").is_some());
    assert!(caps.get("documentFormattingProvider").is_some());
    assert!(caps.get("documentHighlightProvider").is_some());
    assert!(caps.get("documentLinkProvider").is_some());
    assert!(caps.get("colorProvider").is_some());
    s.send(&json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}));

    // hover 到 `count`(第 3 行第 5 列)
    step!("didOpen + hover");
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
        "textDocument": {"uri": URI, "languageId": "gdscript", "version": 1, "text": GOOD}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 2, "method": "textDocument/hover", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 2, "character": 5}}}));
    let hover = s.response(2)["result"].clone();
    let value = hover["contents"]["value"].as_str().unwrap_or_default();
    assert!(value.contains("```gdscript") && value.contains("int"), "hover:{hover}");

    // `count.` 之后补全 — 期望 String 成员
    step!("completion");
    change(&mut s, 2, STRINGIFIED);
    s.send(&json!({"jsonrpc": "2.0", "id": 3, "method": "textDocument/completion", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 4, "character": 16}}}));
    let items = s.response(3)["result"].clone();
    let items = if items.is_array() { items } else { items["items"].clone() };
    assert!(items.as_array().is_some_and(|a| !a.is_empty()), "补全为空:{items}");

    // 语法错误 → publishDiagnostics
    step!("diagnostics");
    change(&mut s, 3, BROKEN);
    let diags = s.first_diagnostics();
    let d = &diags[0];
    assert_eq!(d["source"], "godotlsp");
    assert!(d["code"].is_string(), "诊断 code:{d}");
    assert!(d["severity"].as_i64().is_some_and(|sev| (1..=4).contains(&sev)));

    // 从 `return total`(第 7 行第 9 列)跳转定义 → 声明处(第 6 行)
    step!("definition");
    change(&mut s, 4, GOOD);
    s.send(&json!({"jsonrpc": "2.0", "id": 5, "method": "textDocument/definition", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 6, "character": 9}}}));
    let result = s.response(5)["result"].clone();
    let loc = if result.is_array() { result[0].clone() } else { result["location"].clone() };
    assert_eq!(loc["range"]["start"]["line"], 5, "definition:{result}");
    assert_eq!(loc["range"]["start"]["character"], 5, "definition:{result}");

    // 语义 token + 文档符号
    step!("semanticTokens + symbols");
    s.send(&json!({"jsonrpc": "2.0", "id": 6, "method": "textDocument/semanticTokens/full", "params": {
        "textDocument": {"uri": URI}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 7, "method": "textDocument/documentSymbol", "params": {
        "textDocument": {"uri": URI}}}));
    let toks = s.response(6)["result"]["data"].clone();
    let syms = s.response(7)["result"].clone();
    assert!(toks.as_array().is_some_and(|a| !a.is_empty() && a.len() % 5 == 0), "token");
    let names: Vec<String> = syms.as_array().unwrap().iter()
        .filter_map(|sym| sym["name"].as_str().map(Into::into)).collect();
    assert!(names.iter().any(|n| n == "tick"), "符号:{names:?}");

    // documentHighlight:`count`(第 5 行)应有声明(第 2 行)+ 引用(第 5 行)
    step!("documentHighlight");
    s.send(&json!({"jsonrpc": "2.0", "id": 8, "method": "textDocument/documentHighlight", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 5, "character": 14}}}));
    let highlights = s.response(8)["result"].as_array().cloned().unwrap_or_default();
    assert!(highlights.len() >= 2, "highlight:{highlights:?}");

    // documentLink:res:// 路径 → 项目内文件 URI(根为 crate 目录,Cargo.toml 必然存在)
    step!("documentLink");
    change(&mut s, 5, &format!("{GOOD}const P = preload(\"res://Cargo.toml\")\n"));
    s.send(&json!({"jsonrpc": "2.0", "id": 9, "method": "textDocument/documentLink", "params": {
        "textDocument": {"uri": URI}}}));
    let links = s.response(9)["result"].as_array().cloned().unwrap_or_default();
    let target = links[0]["target"].as_str().unwrap_or_default();
    assert!(target.ends_with("Cargo.toml"), "links:{links:?}");

    // documentColor:Color( 字面量应被识别为颜色
    step!("documentColor");
    change(&mut s, 6, &format!("{GOOD}var c = Color(1, 0.5, 0, 0.8)\n"));
    s.send(&json!({"jsonrpc": "2.0", "id": 10, "method": "textDocument/documentColor", "params": {
        "textDocument": {"uri": URI}}}));
    let colors = s.response(10)["result"].as_array().cloned().unwrap_or_default();
    assert!(
        colors.iter().any(|c| (c["color"]["green"].as_f64().unwrap_or(0.0) - 0.5).abs() < 1e-6),
        "colors:{colors:?}"
    );

    // 增量同步:在 `var count := 5` 行尾(range 编辑)插入一行,documentSymbol 应看到新符号
    step!("incremental didChange");
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {
        "textDocument": {"uri": URI, "version": 7},
        "contentChanges": [{"range": {
            "start": {"line": 2, "character": 14}, "end": {"line": 2, "character": 14}},
            "text": "\nvar extra := 1"}]}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 11, "method": "textDocument/documentSymbol", "params": {
        "textDocument": {"uri": URI}}}));
    let names = s.response(11)["result"].clone();
    let names: Vec<String> = names.as_array().unwrap().iter()
        .filter_map(|sym| sym["name"].as_str().map(Into::into)).collect();
    assert!(names.iter().any(|n| n == "extra"), "增量编辑未生效:{names:?}");

    // 配置同步:关闭 inlay 提示 → inlayHint 返回 null;恢复后非空
    step!("configuration");
    s.send(&json!({"jsonrpc": "2.0", "method": "workspace/didChangeConfiguration", "params": {
        "settings": {"godotlsp": {"inlayHints": {"typeHints": false}}}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 12, "method": "textDocument/inlayHint", "params": {
        "textDocument": {"uri": URI}, "range": {"start": {"line": 0, "character": 0},
        "end": {"line": 20, "character": 0}}}}));
    assert!(s.response(12)["result"].is_null(), "关闭后 inlayHint 应为 null");
    s.send(&json!({"jsonrpc": "2.0", "method": "workspace/didChangeConfiguration", "params": {
        "settings": {"godotlsp": {"inlayHints": {"typeHints": true}}}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 13, "method": "textDocument/inlayHint", "params": {
        "textDocument": {"uri": URI}, "range": {"start": {"line": 0, "character": 0},
        "end": {"line": 20, "character": 0}}}}));
    let hints = s.response(13)["result"].as_array().cloned().unwrap_or_default();
    assert!(!hints.is_empty(), "恢复后应有 inlay 提示:{hints:?}");

    // shutdown + exit
    step!("shutdown + exit");
    s.send(&json!({"jsonrpc": "2.0", "id": 14, "method": "shutdown", "params": null}));
    s.response(14);
    s.send(&json!({"jsonrpc": "2.0", "method": "exit", "params": null}));
    let status = s.child.wait().expect("exit 后等待退出");
    assert!(status.success(), "exit 后退出码非零:{status}");
}

fn change(s: &mut Server, version: i64, text: &str) {
    eprintln!("[step] didChange v{version}");
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {
        "textDocument": {"uri": URI, "version": version},
        "contentChanges": [{"text": text}]}}));
}

/// 拉取式配置 + 诊断版本:zed/helix 类客户端只回 `workspace/configuration`,
/// 服务器必须主动拉取;诊断通知需携带文档版本供客户端丢弃迟到结果。
#[test]
fn configuration_pull_and_diagnostics_version() {
    step!("spawn(声明 configuration 能力)");
    let mut s = Server::spawn();
    s.send(&json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "processId": null, "rootUri": null,
            "capabilities": {"workspace": {"configuration": true}},
        },
    }));
    s.response(1);
    s.send(&json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}));

    // 服务器应主动发起 workspace/configuration 请求
    step!("workspace/configuration 拉取");
    let req = loop {
        let msg = s.read();
        if msg["method"] == "workspace/configuration" {
            break msg;
        }
    };
    let req_id = req["id"].as_i64().expect("服务器请求应为数字 id");
    s.send(&json!({
        "jsonrpc": "2.0", "id": req_id,
        "result": [{"inlayHints": {"typeHints": false}}],
    }));

    // 拉取到的配置立即生效:inlayHint 返回 null
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
        "textDocument": {"uri": URI, "languageId": "gdscript", "version": 1, "text": GOOD}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 2, "method": "textDocument/inlayHint", "params": {
        "textDocument": {"uri": URI}, "range": {"start": {"line": 0, "character": 0},
        "end": {"line": 20, "character": 0}}}}));
    assert!(s.response(2)["result"].is_null(), "拉取配置后 inlayHint 应为 null");

    // publishDiagnostics 携带 didChange 的文档版本
    step!("诊断版本");
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {
        "textDocument": {"uri": URI, "version": 2},
        "contentChanges": [{"text": BROKEN}]}}));
    loop {
        let msg = s.read();
        if msg["method"] == "textDocument/publishDiagnostics"
            && msg["params"]["diagnostics"].as_array().is_some_and(|d| !d.is_empty())
        {
            assert_eq!(msg["params"]["version"], 2, "诊断应携带文档版本:{msg}");
            break;
        }
    }

    s.send(&json!({"jsonrpc": "2.0", "id": 3, "method": "shutdown", "params": null}));
    s.response(3);
    s.send(&json!({"jsonrpc": "2.0", "method": "exit", "params": null}));
    let status = s.child.wait().expect("exit 后等待退出");
    assert!(status.success(), "exit 后退出码非零:{status}");
}
