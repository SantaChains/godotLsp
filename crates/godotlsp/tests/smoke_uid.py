#!/usr/bin/env python3
"""P2.7 冒烟：tscn 自嵌 uid 索引 + documentLink（uid:// 与 tscn 内 res://）。"""
import json, os, pathlib, subprocess, sys, threading

BIN = sys.argv[1]
FIXTURE = pathlib.Path(__file__).parent / "fixtures" / "uidscene"
ROOT_URI = FIXTURE.as_uri()

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

def request(mid, method, params):
    send({"jsonrpc":"2.0","id":mid,"method":method,"params":params})
    for _ in range(20):
        msg = recv()
        if msg.get("id") == mid:
            return msg.get("result")
    raise AssertionError(f"未收到 {method} 响应")

# 1. initialize：rootUri 指向 uidscene 工程夹具
result = request(1, "initialize", {"capabilities":{}, "rootUri": ROOT_URI})
print("OK initialize, rootUri =", ROOT_URI)

send({"jsonrpc":"2.0","method":"initialized","params":{}})

gd_uri = (FIXTURE / "main.gd").as_uri()
tscn_uri = (FIXTURE / "main.tscn").as_uri()

# 2. didOpen main.gd（引用 uid://c9fixtmain 与 res://main.tscn）
send({"jsonrpc":"2.0","method":"textDocument/didOpen","params":{
    "textDocument":{"uri":gd_uri,"version":1,"languageId":"gdscript",
    "text":(FIXTURE / "main.gd").read_text(encoding="utf-8")}}})

# 3. documentLink main.gd：两条都应解析（uid → tscn；res:// → tscn）
links = request(3, "textDocument/documentLink", {"textDocument":{"uri":gd_uri}}) or []
targets = [l.get("target","") for l in links]
print("OK documentLink(main.gd):", len(links), "条")
for l in links:
    print("  -", l.get("tooltip","res://"), "->", l["target"])
assert any(l["target"].endswith("main.tscn") for l in links), targets
uid_links = [l for l in links if l.get("tooltip") == "uid://c9fixtmain"]
assert uid_links, "uid:// 链接缺失——tscn 自嵌 uid 未进索引"
assert uid_links[0]["target"].endswith("main.tscn"), uid_links[0]

# 4. didOpen main.tscn → documentLink：ext_resource 的 res://main.gd 可点
send({"jsonrpc":"2.0","method":"textDocument/didOpen","params":{
    "textDocument":{"uri":tscn_uri,"version":1,"languageId":"godot-resource",
    "text":(FIXTURE / "main.tscn").read_text(encoding="utf-8")}}})
tlinks = request(4, "textDocument/documentLink", {"textDocument":{"uri":tscn_uri}}) or []
print("OK documentLink(main.tscn):", len(tlinks), "条")
for l in tlinks:
    print("  - ->", l.get("target"))
assert any(l["target"].endswith("main.gd") for l in tlinks), tlinks

# 5. P2.9 documentColor：main.gd 的 Color(0.5,...) + Color.RED 应各出一条
colors = request(5, "textDocument/documentColor", {"textDocument":{"uri":gd_uri}}) or []
print("OK documentColor(main.gd):", len(colors), "条")
for c in colors:
    print("  -", c["color"])
assert len(colors) == 2, colors
lit = [c for c in colors if abs(c["color"]["red"] - 0.5) < 1e-6]
named = [c for c in colors if c["color"]["red"] > 0.8]
assert lit and named, colors

# 6. P2.10 codeLens：func _ready 上方应有场景连接 lens（main.tscn [connection]）
lenses = request(6, "textDocument/codeLens", {"textDocument":{"uri":gd_uri}}) or []
print("OK codeLens(main.gd):", len(lenses), "条")
for l in lenses:
    print("  -", l.get("command", {}).get("title"), "@line", l["range"]["start"]["line"])
assert len(lenses) == 1, lenses
cmd = lenses[0]["command"]
assert cmd["title"] == "1 个场景连接", cmd
assert cmd["command"] == "godotlsp.openConnection", cmd
assert cmd["arguments"][0].endswith("main.tscn"), cmd

# 7. shutdown + exit
send({"jsonrpc":"2.0","id":2,"method":"shutdown","params":None})
sh = recv()
assert "result" in sh, sh
send({"jsonrpc":"2.0","method":"exit","params":{}})
try:
    proc.wait(timeout=5)
except subprocess.TimeoutExpired:
    proc.kill()
print("ALL OK")
