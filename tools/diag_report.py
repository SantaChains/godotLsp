#!/usr/bin/env python3
"""诊断误报分桶报告:全量 didOpen corpus/bench 工程,按消息模式聚合诊断。

模式归一:引号内的具体名字替换为 X,数字归一;输出 top 桶 + 每桶示例。
用法:
    python -u tools/diag_report.py <godotlsp.exe> <project-dir> [--top 25]
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import time
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "crates" / "godotlsp" / "tests"))
from bench_load import Client  # noqa: E402


def norm(msg: str) -> str:
    s = re.sub(r"'[^']*'", "'X'", msg)
    s = re.sub(r"\d+", "N", s)
    return s


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("bin")
    ap.add_argument("project")
    ap.add_argument("--top", type=int, default=25)
    args = ap.parse_args()

    root = Path(args.project).resolve()
    files = sorted(p for p in root.rglob("*.gd") if ".git" not in p.parts)
    c = Client(args.bin)
    r = c.request("initialize", {
        "capabilities": {}, "rootUri": root.as_uri()}, 120)
    assert r and "result" in r, "initialize 失败"
    c.send({"jsonrpc": "2.0", "method": "initialized", "params": {}})

    t0 = time.monotonic()
    for p in files:
        try:
            text = p.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        c.send({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
            "textDocument": {"uri": p.as_uri(), "version": 1,
                             "languageId": "gdscript", "text": text}}})

    buckets: Counter[str] = Counter()
    examples: dict[str, list[tuple[str, str]]] = {}
    covered = 0
    deadline = time.monotonic() + 300
    while time.monotonic() < deadline:
        try:
            msg = c.q.get(timeout=2.0)
        except Exception:
            if time.monotonic() - c.diag_last_ts > 8:
                break
            continue
        if msg.get("__eof__"):
            break
        if "method" in msg and "id" in msg:
            result = [{}] if msg["method"] == "workspace/configuration" else None
            c.send({"jsonrpc": "2.0", "id": msg["id"], "result": result})
            continue
        if msg.get("method") == "textDocument/publishDiagnostics":
            uri = msg["params"]["uri"]
            diags = msg["params"].get("diagnostics", [])
            if uri not in c.diag_seen:
                covered += 1
            c.diag_seen[uri] = len(diags)
            c.diag_last_ts = time.monotonic()
            rel = uri.replace(root.as_uri() + "/", "")
            for d in diags:
                key = norm(d["message"])
                buckets[key] += 1
                examples.setdefault(key, []).append((rel, d["message"]))
    elapsed = time.monotonic() - t0

    total = sum(buckets.values())
    print(f"[diag] {root.name}: {len(files)} 文件,收敛 {elapsed:.1f}s,"
          f"覆盖 {covered}/{len(files)},诊断 {total:,} 条")
    print(f"[diag] ---- top {args.top} 桶 ----")
    for key, n in buckets.most_common(args.top):
        print(f"{n:>6}  {key}")
        for rel, msg in examples[key][:3]:
            print(f"        例:{rel}: {msg}")
    c.send({"jsonrpc": "2.0", "id": 999999, "method": "shutdown", "params": None})
    c.wait_response(999999, 15)
    c.send({"jsonrpc": "2.0", "method": "exit", "params": {}})
    try:
        c.proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        c.proc.kill()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
