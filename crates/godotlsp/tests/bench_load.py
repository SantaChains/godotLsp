#!/usr/bin/env python3
"""真实工程负载基准:spawn godotlsp(release),对 corpus/bench/ 下的大型
工程做全量 didOpen + 请求压测,报告索引收敛时间、请求延迟分位数与峰值内存。

前置:tools/fetch_bench.py 已拉取语料;服务器用 release 构建。

用法:
    python -u crates/godotlsp/tests/bench_load.py <godotlsp.exe> <project-dir> \
        [--timeout 300] [--samples 40]

指标口径:
  - open:    全量 didOpen 的发送耗时(客户端侧,不含服务器消化)
  - settle:  首个 didOpen 起到最后一条 publishDiagnostics 的收敛时间
             (对未发布空诊断的文件以 10s 静默窗口兜底,报告覆盖率)
  - edit:    单文件 didChange 后该 uri 下一条诊断的增量延迟
  - symbol/  documentSymbol / hover / completion 往返延迟 p50/p95/max
  - rss:     服务器进程峰值工作集(Windows;其他平台 N/A)
"""

from __future__ import annotations

import argparse
import json
import queue
import subprocess
import sys
import threading
import time
from pathlib import Path

QUIET_WINDOW = 10.0  # settle 判定的静默窗口(秒):窗口内无新诊断则视为收敛


def fmt_ms(ms: float) -> str:
    return f"{ms:,.0f}ms" if ms >= 100 else f"{ms:.1f}ms"


class Client:
    def __init__(self, bin_path: str):
        self.proc = subprocess.Popen(
            [bin_path], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL)
        self.q: queue.Queue = queue.Queue()
        self.next_id = 100
        self.diag_seen: dict[str, int] = {}   # uri -> 最近一次诊断条数
        self.diag_last_ts = 0.0
        self.log_count = 0
        self._lock = threading.Lock()
        self._reader = threading.Thread(target=self._read_loop, daemon=True)
        self._reader.start()

    def send(self, msg: dict) -> None:
        body = json.dumps(msg).encode("utf-8")
        self.proc.stdin.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
        self.proc.stdin.flush()

    def _read_loop(self) -> None:
        out = self.proc.stdout
        try:
            while True:
                headers = {}
                while True:
                    line = out.readline()
                    if line in (b"\r\n", b"\n", b""):
                        break
                    k, v = line.decode().strip().split(": ", 1)
                    headers[k.lower()] = v
                n = int(headers["content-length"])
                self.q.put(json.loads(out.read(n)))
        except (OSError, ValueError):
            self.q.put({"__eof__": True})

    def request(self, method: str, params: dict, timeout: float) -> dict | None:
        with self._lock:
            rid = self.next_id
            self.next_id += 1
        self.send({"jsonrpc": "2.0", "id": rid, "method": method, "params": params})
        return self.wait_response(rid, timeout)

    def wait_response(self, rid: int, timeout: float) -> dict | None:
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            try:
                msg = self.q.get(timeout=max(0.0, deadline - time.monotonic()))
            except queue.Empty:
                break
            if msg.get("__eof__"):
                return None
            if "method" in msg and "id" in msg:  # 服务器→客户端请求:礼貌性应答
                result = [{}] if msg["method"] == "workspace/configuration" else None
                self.send({"jsonrpc": "2.0", "id": msg["id"], "result": result})
                continue
            if "method" in msg:
                if msg["method"] == "textDocument/publishDiagnostics":
                    uri = msg["params"]["uri"]
                    self.diag_seen[uri] = len(msg["params"].get("diagnostics", []))
                    self.diag_last_ts = time.monotonic()
                elif msg["method"].startswith("window/logMessage"):
                    self.log_count += 1
                continue
            if msg.get("id") == rid:
                return msg
        return None

    def wait_quiet(self, quiet: float, timeout: float) -> float:
        """等诊断流静默 quiet 秒,返回自计时起点起最后一条诊断的相对时刻。"""
        t0 = time.monotonic()
        last = self.diag_last_ts
        while time.monotonic() - t0 < timeout:
            try:
                msg = self.q.get(timeout=min(quiet, 1.0))
            except queue.Empty:
                if time.monotonic() - max(last, t0) >= quiet:
                    return last - t0
                continue
            if msg.get("__eof__"):
                return last - t0
            if "method" in msg and "id" in msg:
                result = [{}] if msg["method"] == "workspace/configuration" else None
                self.send({"jsonrpc": "2.0", "id": msg["id"], "result": result})
                continue
            if "method" in msg:
                if msg["method"] == "textDocument/publishDiagnostics":
                    uri = msg["params"]["uri"]
                    self.diag_seen[uri] = len(msg["params"].get("diagnostics", []))
                    last = self.diag_last_ts = time.monotonic()
                elif msg["method"].startswith("window/logMessage"):
                    self.log_count += 1

        return last - t0


def rss_sampler(pid: int, stop: threading.Event, out: list[float]) -> None:
    """Windows:OpenProcess + GetProcessMemoryInfo 采样工作集。"""
    try:
        import ctypes
        psapi = ctypes.WinDLL("psapi")
        kernel32 = ctypes.WinDLL("kernel32")

        class PMC(ctypes.Structure):
            _fields_ = [
                ("cb", ctypes.c_ulong), ("PageFaultCount", ctypes.c_ulong),
                ("PeakWorkingSetSize", ctypes.c_size_t),
                ("WorkingSetSize", ctypes.c_size_t),
                ("QuotaPeakPagedPoolUsage", ctypes.c_size_t),
                ("QuotaPagedPoolUsage", ctypes.c_size_t),
                ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
                ("QuotaNonPagedPoolUsage", ctypes.c_size_t),
                ("PagefileUsage", ctypes.c_size_t),
                ("PeakPagefileUsage", ctypes.c_size_t)]

        h = kernel32.OpenProcess(0x1000, False, pid)  # PROCESS_QUERY_LIMITED_INFORMATION
        while not stop.is_set() and h:
            pmc = PMC()
            pmc.cb = ctypes.sizeof(PMC)
            if psapi.GetProcessMemoryInfo(h, ctypes.byref(pmc), pmc.cb):
                out.append(pmc.WorkingSetSize / (1024 * 1024))
            stop.wait(0.25)
    except Exception:  # noqa: BLE001 — 非 Windows / 权限不足时内存指标降级为 N/A
        pass


def pct(xs: list[float], p: float) -> float:
    if not xs:
        return 0.0
    s = sorted(xs)
    return s[min(len(s) - 1, int(len(s) * p))]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("bin")
    ap.add_argument("project")
    ap.add_argument("--timeout", type=float, default=300.0)
    ap.add_argument("--samples", type=int, default=40)
    args = ap.parse_args()

    root = Path(args.project).resolve()
    files = sorted(
        p for p in root.rglob("*.gd")
        if ".git" not in p.parts and ".godot" not in p.parts)
    if not files:
        print(f"[bench] {root} 下没有 .gd 文件", file=sys.stderr)
        return 2
    total_lines = 0
    for p in files:
        try:
            total_lines += sum(1 for _ in p.open("rb"))
        except OSError:
            pass
    print(f"[bench] 工程={root.name}  .gd={len(files)}  总行数={total_lines:,}")

    c = Client(args.bin)
    rss: list[float] = []
    stop = threading.Event()
    threading.Thread(target=rss_sampler, args=(c.proc.pid, stop, rss), daemon=True).start()

    def uri_of(p: Path) -> str:
        return p.as_uri()

    # 1. initialize
    t0 = time.monotonic()
    r = c.request("initialize", {
        "capabilities": {"textDocument": {"completion": {
            "completionItem": {"snippetSupport": True}}}},
        "rootUri": root.as_uri()}, args.timeout)
    assert r and "result" in r, "initialize 失败"
    print(f"[bench] initialize: {fmt_ms((time.monotonic() - t0) * 1000)}")
    c.send({"jsonrpc": "2.0", "method": "initialized", "params": {}})

    # 2. 全量 didOpen + 诊断收敛
    t_open0 = time.monotonic()
    for p in files:
        try:
            text = p.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        c.send({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
            "textDocument": {"uri": uri_of(p), "version": 1,
                             "languageId": "gdscript", "text": text}}})
    open_ms = (time.monotonic() - t_open0) * 1000

    settle = c.wait_quiet(QUIET_WINDOW, args.timeout)
    covered = len([u for u in c.diag_seen if u.startswith(root.as_uri())])
    print(f"[bench] didOpen 全量发送: {fmt_ms(open_ms)}")
    print(f"[bench] 诊断收敛(settle): {fmt_ms(settle * 1000)}"
          f"  覆盖 {covered}/{len(files)} 文件  共 {sum(c.diag_seen.values()):,} 条诊断")

    # 3. 请求压测:均匀采样
    step = max(1, len(files) // args.samples)
    sample = files[::step][:args.samples]
    lat = {"symbol": [], "hover": [], "completion": []}
    for i, p in enumerate(sample):
        uri = uri_of(p)
        t = time.monotonic()
        r = c.request("textDocument/documentSymbol", {"textDocument": {"uri": uri}}, 30)
        if r:
            lat["symbol"].append((time.monotonic() - t) * 1000)
        t = time.monotonic()
        r = c.request("textDocument/hover", {
            "textDocument": {"uri": uri}, "position": {"line": 0, "character": 8}}, 30)
        if r:
            lat["hover"].append((time.monotonic() - t) * 1000)
        t = time.monotonic()
        r = c.request("textDocument/completion", {
            "textDocument": {"uri": uri}, "position": {"line": 1, "character": 0}}, 30)
        if r:
            lat["completion"].append((time.monotonic() - t) * 1000)
        if (i + 1) % 10 == 0:
            print(f"[bench]   ... {i + 1}/{len(sample)}")

    # 4. 增量编辑延迟:改最大文件,等该 uri 下一条诊断
    big = max(files, key=lambda p: p.stat().st_size)
    uri = uri_of(big)
    before = c.diag_seen.get(uri)
    c.diag_seen.pop(uri, None)
    t = time.monotonic()
    c.send({"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {
        "textDocument": {"uri": uri, "version": 2},
        "contentChanges": [{"text": big.read_text(encoding="utf-8", errors="replace")
                            + "\n# bench_edit\n"}]}})
    edit_deadline = time.monotonic() + args.timeout
    edit_ms = None
    while time.monotonic() < edit_deadline:
        try:
            msg = c.q.get(timeout=1.0)
        except queue.Empty:
            continue
        if msg.get("__eof__"):
            break
        if "method" in msg and "id" in msg:
            result = [{}] if msg["method"] == "workspace/configuration" else None
            c.send({"jsonrpc": "2.0", "id": msg["id"], "result": result})
            continue
        if msg.get("method") == "textDocument/publishDiagnostics" \
                and msg["params"]["uri"] == uri:
            edit_ms = (time.monotonic() - t) * 1000
            break
    c.diag_seen[uri] = c.diag_seen.get(uri, before or 0)

    # 5. 汇总
    stop.set()
    print("[bench] ---- 请求延迟(往返) ----")
    for k, xs in lat.items():
        if xs:
            print(f"[bench] {k:<10} n={len(xs):<4} p50={fmt_ms(pct(xs, 0.50))}"
                  f"  p95={fmt_ms(pct(xs, 0.95))}  max={fmt_ms(max(xs))}")
        else:
            print(f"[bench] {k:<10} 无响应样本")
    if edit_ms is not None:
        print(f"[bench] edit 增量诊断延迟: {fmt_ms(edit_ms)} ({big.name})")
    else:
        print(f"[bench] edit 增量诊断延迟: 未观测到({big.name})")
    if rss:
        print(f"[bench] 服务器内存: 峰值 {max(rss):.0f}MB  均值 {sum(rss) / len(rss):.0f}MB"
              f"  (n={len(rss)})")
    else:
        print("[bench] 服务器内存: N/A")
    print(f"[bench] logMessage 通知数: {c.log_count:,}")

    c.send({"jsonrpc": "2.0", "id": 999999, "method": "shutdown", "params": None})
    c.wait_response(999999, 15)
    c.send({"jsonrpc": "2.0", "method": "exit", "params": {}})
    try:
        c.proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        c.proc.kill()
    print("[bench] ALL OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
