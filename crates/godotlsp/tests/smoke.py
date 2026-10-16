#!/usr/bin/env python3
"""LSP 闭环冒烟测试：spawn godotlsp.exe，走完整 JSON-RPC 握手。"""
import json, subprocess, sys, threading

BIN = sys.argv[1]
proc = subprocess.Popen([BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

def send(msg):
    body = json.dumps(msg).encode("utf-8")
    proc.stdin.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
    proc.stdin.flush()

def recv():
    headers = {}
    while True:
        line = proc.stdout.readline()
        if line in (b"\r\n", b"\n", b""):
            break
        k, v = line.decode().strip().split(": ", 1)
        headers[k.lower()] = v
    n = int(headers["content-length"])
    return json.loads(proc.stdout.read(n))

# 1. initialize
send({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{},"rootUri":None}})
init_resp = recv()
assert "result" in init_resp, init_resp
caps = init_resp["result"]["capabilities"]
assert caps.get("hoverProvider"), caps
print("OK initialize, hover =", caps.get("hoverProvider"))

# 2. initialized
send({"jsonrpc":"2.0","method":"initialized","params":{}})

# 3. didOpen 一个含未声明标识符的脚本
send({"jsonrpc":"2.0","method":"textDocument/didOpen","params":{
    "textDocument":{"uri":"file:///d:/test/main.gd","version":1,"languageId":"gdscript",
    "text":"extends Node2D\nfunc _ready():\n    print(nope_thing)\n"}}})

# 4. 等待 publishDiagnostics（可能先收到其他通知）
diag = None
for _ in range(5):
    msg = recv()
    if msg.get("method") == "textDocument/publishDiagnostics":
        diag = msg["params"]["diagnostics"]
        break
assert diag is not None, "未收到 publishDiagnostics"
print("OK publishDiagnostics, 条数 =", len(diag))
for d in diag:
    print("  -", d["message"])
assert any("nope_thing" in d["message"] for d in diag), diag

# 4b. documentSymbol：返回函数/变量等符号
send({"jsonrpc":"2.0","id":3,"method":"textDocument/documentSymbol","params":{
    "textDocument":{"uri":"file:///d:/test/main.gd"}}})
# 先把可能残留的通知消费掉，直到拿到 id=3 的响应
syms = None
for _ in range(10):
    msg = recv()
    if msg.get("id") == 3:
        syms = msg["result"]
        break
assert syms is not None, "未收到 documentSymbol 响应"
print("OK documentSymbol, 符号数 =", len(syms))
for s in syms:
    print("  -", s["kind"], s["name"])
assert any(s["name"] == "_ready" for s in syms), syms

# 4c. hover 类名 Node2D：应返回 class Node2D extends CanvasItem
send({"jsonrpc":"2.0","id":4,"method":"textDocument/hover","params":{
    "textDocument":{"uri":"file:///d:/test/main.gd"},
    "position":{"line":0,"character":8}}})  # "extends Node2D" 的 Node2D
hov = None
for _ in range(10):
    msg = recv()
    if msg.get("id") == 4:
        hov = msg["result"]
        break
assert hov is not None, "未收到 hover 响应"
value = hov["contents"]["value"] if hov else ""
print("OK hover Node2D:", value.replace("\n", " | "))
assert "Node2D" in value and "CanvasItem" in value, value

# 4d. 接收者补全：在 doc 里 var node: Node2D，然后 node.< 应补 get_position 等
send({"jsonrpc":"2.0","method":"textDocument/didChange","params":{
    "textDocument":{"uri":"file:///d:/test/main.gd","version":2},
    "contentChanges":[{"text":"extends Node2D\nvar node: Node2D\nfunc _ready():\n    node.\n"}]}})
send({"jsonrpc":"2.0","id":5,"method":"textDocument/completion","params":{
    "textDocument":{"uri":"file:///d:/test/main.gd"},
    "position":{"line":3,"character":9}}})  # node.<|
comp = None
for _ in range(10):
    msg = recv()
    if msg.get("id") == 5:
        comp = msg["result"]
        break
assert comp is not None, "未收到 completion"
labels = [i["label"] for i in comp]
print("OK completion node.:", len(labels), "项; 含 get_position:", "get_position" in labels)
assert "get_position" in labels, labels[:20]

# 5. shutdown + exit
send({"jsonrpc":"2.0","id":2,"method":"shutdown","params":None})
sh = recv()
assert "result" in sh, sh
send({"jsonrpc":"2.0","method":"exit","params":{}})
try:
    proc.wait(timeout=5)
except subprocess.TimeoutExpired:
    proc.kill()
print("ALL OK")
