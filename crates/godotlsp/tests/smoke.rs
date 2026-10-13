//! End-to-end smoke test: spawn the server binary over stdio and run a protocol session.

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
        let mut child = Command::new(env!("CARGO_BIN_EXE_godotlsp"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn server");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Self { child, stdin, stdout }
    }

    fn send(&mut self, msg: &Value) {
        let body = msg.to_string();
        write!(self.stdin, "Content-Length: {}\r\n\r\n{body}", body.len()).unwrap();
        self.stdin.flush().unwrap();
    }

    /// The next message as JSON, whatever its kind (request/response/notification).
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
        let len = len.expect("Content-Length header");
        let mut body = vec![0u8; len];
        std::io::Read::read_exact(&mut self.stdout, &mut body).unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    /// The next response with `id`, skipping notifications.
    fn response(&mut self, id: u64) -> Value {
        loop {
            let msg = self.read();
            if msg.get("id").and_then(Value::as_u64) == Some(id) {
                return msg;
            }
        }
    }

    /// The next publishDiagnostics notification carrying at least one diagnostic.
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

    // initialize → capabilities
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
    assert_eq!(caps["hoverProvider"], Value::Bool(true));
    assert!(caps["completionProvider"]["triggerCharacters"].as_array().is_some_and(|a| !a.is_empty()));
    assert!(caps.get("semanticTokensProvider").is_some());
    assert!(caps.get("documentSymbolProvider").is_some());
    assert!(caps.get("renameProvider").is_some());
    assert!(caps.get("inlayHintProvider").is_some());
    assert!(caps.get("foldingRangeProvider").is_some());
    assert!(caps.get("documentFormattingProvider").is_some());
    s.send(&json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}));

    // hover on `count` (line 2, char 5)
    step!("didOpen + hover");
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
        "textDocument": {"uri": URI, "languageId": "gdscript", "version": 1, "text": GOOD}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 2, "method": "textDocument/hover", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 2, "character": 5}}}));
    let hover = s.response(2)["result"].clone();
    let value = hover["contents"]["value"].as_str().unwrap_or_default();
    assert!(value.contains("```gdscript") && value.contains("int"), "hover: {hover}");

    // completion after `count.` — String members expected
    step!("completion");
    change(&mut s, 2, STRINGIFIED);
    s.send(&json!({"jsonrpc": "2.0", "id": 3, "method": "textDocument/completion", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 4, "character": 16}}}));
    let items = s.response(3)["result"].clone();
    let items = if items.is_array() { items } else { items["items"].clone() };
    assert!(items.as_array().is_some_and(|a| !a.is_empty()), "empty completion: {items}");

    // syntax error → publishDiagnostics
    step!("diagnostics");
    change(&mut s, 3, BROKEN);
    let diags = s.first_diagnostics();
    let d = &diags[0];
    assert_eq!(d["source"], "godotlsp");
    assert!(d["code"].is_string(), "diagnostic code: {d}");
    assert!(d["severity"].as_i64().is_some_and(|sev| (1..=4).contains(&sev)));

    // goto definition of `total` from `return total` (line 6, char 9) → its declaration (line 5)
    step!("definition");
    change(&mut s, 4, GOOD);
    s.send(&json!({"jsonrpc": "2.0", "id": 5, "method": "textDocument/definition", "params": {
        "textDocument": {"uri": URI}, "position": {"line": 6, "character": 9}}}));
    let result = s.response(5)["result"].clone();
    let loc = if result.is_array() { result[0].clone() } else { result["location"].clone() };
    assert_eq!(loc["range"]["start"]["line"], 5, "definition: {result}");
    assert_eq!(loc["range"]["start"]["character"], 5, "definition: {result}");

    // semantic tokens + document symbols
    step!("semanticTokens + symbols");
    s.send(&json!({"jsonrpc": "2.0", "id": 6, "method": "textDocument/semanticTokens/full", "params": {
        "textDocument": {"uri": URI}}}));
    s.send(&json!({"jsonrpc": "2.0", "id": 7, "method": "textDocument/documentSymbol", "params": {
        "textDocument": {"uri": URI}}}));
    let toks = s.response(6)["result"]["data"].clone();
    let syms = s.response(7)["result"].clone();
    assert!(toks.as_array().is_some_and(|a| !a.is_empty() && a.len() % 5 == 0), "tokens");
    let names: Vec<String> = syms.as_array().unwrap().iter()
        .filter_map(|sym| sym["name"].as_str().map(Into::into)).collect();
    assert!(names.iter().any(|n| n == "tick"), "symbols: {names:?}");

    // shutdown + exit
    step!("shutdown + exit");
    s.send(&json!({"jsonrpc": "2.0", "id": 8, "method": "shutdown", "params": null}));
    s.response(8);
    s.send(&json!({"jsonrpc": "2.0", "method": "exit", "params": null}));
    let status = s.child.wait().expect("wait after exit");
    assert!(status.success(), "nonzero exit after `exit`: {status}");
}

fn change(s: &mut Server, version: i64, text: &str) {
    eprintln!("[step] didChange v{version}");
    s.send(&json!({"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {
        "textDocument": {"uri": URI, "version": version},
        "contentChanges": [{"text": text}]}}));
}
