#!/usr/bin/env python3
"""Minimal LSP stdio client: spawn godotlsp.exe, open a project, dump diagnostics.

Usage: python lsp_probe.py <server_exe> <project_dir> <entry.gd> [wait_seconds]
"""
import json
import subprocess
import sys
import threading
import time
from pathlib import Path


def make_message(obj):
    body = json.dumps(obj).encode("utf-8")
    return f"Content-Length: {len(body)}\r\n\r\n".encode("ascii") + body


def read_messages(stream, out):
    while True:
        headers = {}
        while True:
            line = stream.readline()
            if not line:
                return
            line = line.decode("ascii", "replace").strip()
            if line == "":
                break
            if ":" in line:
                k, v = line.split(":", 1)
                headers[k.strip().lower()] = v.strip()
        n = int(headers.get("content-length", "0"))
        body = stream.read(n) if n else b""
        try:
            out.append(json.loads(body.decode("utf-8")))
        except Exception:
            pass


def main():
    exe, proj, entry = sys.argv[1], Path(sys.argv[2]), sys.argv[3]
    wait = float(sys.argv[4]) if len(sys.argv) > 4 else 4.0
    proc = subprocess.Popen(
        [exe], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL
    )
    msgs = []
    t = threading.Thread(target=read_messages, args=(proc.stdout, msgs), daemon=True)
    t.start()

    def send(obj):
        proc.stdin.write(make_message(obj))
        proc.stdin.flush()

    send({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "processId": None,
            "rootUri": proj.resolve().as_uri(),
            "capabilities": {},
        },
    })
    time.sleep(0.5)
    send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
    time.sleep(1.5)  # load_project + engine model
    text = (proj / entry).read_text(encoding="utf-8")
    send({
        "jsonrpc": "2.0", "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": (proj / entry).resolve().as_uri(),
                "languageId": "gdscript",
                "version": 1,
                "text": text,
            }
        },
    })
    deadline = time.time() + wait
    while time.time() < deadline:
        time.sleep(0.2)

    diags = [m for m in msgs if m.get("method") == "textDocument/publishDiagnostics"]
    logs = [m for m in msgs if m.get("method") == "window/logMessage"]
    print(f"== all methods: {[m.get('method') for m in msgs]} ==")
    print(f"== logMessage ({len(logs)}) ==")
    for m in logs:
        print("  ", m.get("params", {}).get("message"))
    print(f"== publishDiagnostics ({len(diags)}) ==")
    for m in diags:
        p = m.get("params", {})
        print(f"  uri={p.get('uri')}")
        for d in p.get("diagnostics", []):
            rng = d.get("range", {}).get("start", {})
            print(f"    L{rng.get('line', 0) + 1}: [{d.get('severity')}] {d.get('message')}")
    if not diags:
        print("   (none)")
    proc.kill()


if __name__ == "__main__":
    main()
