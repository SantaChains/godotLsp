#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Godot 工程验收门:改完 project.godot / .tscn / .tres / .gd 后跑这一条,别靠"看起来对"。

    uv run tools/godot_verify.py check <工程目录> [--godot EXE] [--frames N]
    uv run tools/godot_verify.py lsp   <工作区根> [--project 工程目录] [--server EXE]

`check` —— 引擎侧加载门(全程无头,不弹窗):
    1) `--import`  先跑一次"工程生成"(生成 .godot/,含 global_script_class_cache.cfg)
    2) `--quit-after N` 实际运行 N 帧
    只回显 ERROR/WARNING/SCRIPT ERROR 行(含紧随的 `at:` 定位行),有 ERROR 即退出码 1。

    注意:Godot 遇到致命错误也可能 **exit code = 0**,所以本工具按**错误行**判账,不看退出码。

`lsp` —— 分析侧门:对工作区根起一个 stdio LSP 会话,回显每个文件的诊断数、
    documentSymbol 数、三类关键 hover(自成员 / 跨文件 class_name / autoload)与
    `autoload.` 补全项数;出现 error 级诊断即退出码 1。

    工作区根 = 语言服务器的 cwd(插件取 workspaceFolders[0])。工程位于子目录时,
    把 `--project` 指向工程目录即可复现"根错位"的后果(诊断爆炸、补全塌缩)。

退出码: 0 干净 / 1 有错 / 2 环境或用法问题。
"""

from __future__ import annotations

import argparse
import json
import os
import queue
import re
import shutil
import subprocess
import sys
import threading
import time
from pathlib import Path
from typing import Any, NoReturn

ERROR_LINE_RE = re.compile(r"^(?:ERROR|USER ERROR|SCRIPT ERROR|SHADER ERROR|WARNING|USER WARNING):")
AT_LINE_RE = re.compile(r"^\s+at: ")
SEV_ERROR = 1


# --------------------------------------------------------------------------- 公共

def fail(message: str) -> NoReturn:
    """环境/用法问题:错误走 stderr,退出码 2(与"有错=1"区分开)。"""
    print(f"[err] {message}", file=sys.stderr)
    sys.exit(2)


def find_exe(explicit: str | None, env: str, names: tuple[str, ...], what: str, flag: str) -> str:
    for cand in (explicit, os.environ.get(env)):
        if cand:
            path = Path(cand)
            if path.is_file():
                return str(path)
            fail(f"{what} 不存在: {cand}")
    for name in names:
        found = shutil.which(name)
        if found:
            return found
    fail(f"找不到{what}。用 {flag} 指定、或设 {env} 环境变量、或放进 PATH。")


def gd_files(root: Path, limit: int) -> tuple[list[Path], bool]:
    """工程内 .gd 文件;点目录(如 .godot/.refs)按引擎规则跳过。"""
    files = [
        p
        for p in sorted(root.rglob("*.gd"))
        if not any(seg.startswith(".") for seg in p.relative_to(root).parts)
    ]
    return files[:limit], len(files) > limit


def error_report(output: str) -> list[tuple[str, str]]:
    """抽出 ERROR/WARNING 行,并带上紧随其后的 `at:` 定位行。返回 (级别, 文本)。"""
    lines = output.splitlines()
    kept: list[tuple[str, str]] = []
    for i, line in enumerate(lines):
        if not ERROR_LINE_RE.match(line):
            continue
        kind = "warning" if line.startswith(("WARNING:", "USER WARNING:")) else "error"
        kept.append((kind, line.rstrip()))
        if i + 1 < len(lines) and AT_LINE_RE.match(lines[i + 1]):
            kept.append(("at", lines[i + 1].rstrip()))
            # 紧跟的 GDScript backtrace(带 [N] 帧)直接给出调用点,别丢。
            j = i + 2
            while j < len(lines) and (lines[j].strip().startswith("GDScript backtrace")
                                      or re.match(r"^\s+\[\d+\] ", lines[j])):
                kept.append(("at", lines[j].rstrip()))
                j += 1
    return kept


# --------------------------------------------------------------------------- check

def cmd_check(args: argparse.Namespace) -> int:
    project = Path(args.project).resolve()
    if not (project / "project.godot").is_file():
        fail(f"{project} 下没有 project.godot(工程根传错了?)")
    godot = find_exe(args.godot, "GODOT", ("godot", "godot4", "Godot"), "Godot 可执行文件", "--godot")

    phases = [("import(工程生成)", ["--import"], 600), (f"run {args.frames} 帧", ["--quit-after", str(args.frames)], 300)]
    total = 0
    total_warn = 0
    for label, extra, timeout in phases:
        cmd = [godot, "--headless", "--path", str(project), *extra]
        try:
            proc = subprocess.run(cmd, check=False, capture_output=True, text=True,
                                 timeout=timeout, errors="replace")
        except subprocess.TimeoutExpired:
            print(f"[check] {label}: 超时 {timeout}s")
            total += 1
            continue
        hits = error_report((proc.stdout or "") + (proc.stderr or ""))
        errs = sum(1 for kind, _ in hits if kind == "error")
        warns = sum(1 for kind, _ in hits if kind == "warning")
        print(f"[check] {label}: exit={proc.returncode} error={errs} warning={warns}")
        for _, line in hits:
            print("        " + line)
        total += errs
        total_warn += warns

    verdict = "干净" if total == 0 else f"有 {total} 条 ERROR"
    print(f"[check] 结论: {verdict}(error={total} warning={total_warn}; "
          f"无头只覆盖加载/解析/运行期报错,不覆盖渲染与手感)")
    return 0 if total == 0 else 1


# --------------------------------------------------------------------------- lsp

class LspClient:
    """最小 LSP stdio 客户端;带请求超时,避免服务器不响应时挂死。"""

    def __init__(self, cmd: list[str], cwd: Path, default_timeout: float) -> None:
        self.proc = subprocess.Popen(cmd, cwd=str(cwd), stdin=subprocess.PIPE,
                                     stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        self.default_timeout = default_timeout
        self.next_id = 1
        self.responses: queue.Queue[dict] = queue.Queue()
        self.diagnostics: dict[str, list[dict]] = {}
        self.stderr_chunks: list[str] = []
        threading.Thread(target=self._read_stdout, daemon=True).start()
        threading.Thread(target=self._read_stderr, daemon=True).start()

    def _read_stderr(self) -> None:
        assert self.proc.stderr
        for line in self.proc.stderr:
            self.stderr_chunks.append(line.decode("utf-8", "replace"))

    def _read_stdout(self) -> None:
        assert self.proc.stdout
        buf = b""
        while True:
            chunk = os.read(self.proc.stdout.fileno(), 65536)
            if not chunk:
                return
            buf += chunk
            while True:
                head_end = buf.find(b"\r\n\r\n")
                if head_end < 0:
                    break
                match = re.search(rb"Content-Length:\s*(\d+)", buf[:head_end], re.IGNORECASE)
                if not match:
                    buf = buf[head_end + 4:]
                    continue
                length = int(match.group(1))
                start = head_end + 4
                if len(buf) < start + length:
                    break
                body, buf = buf[start:start + length], buf[start + length:]
                try:
                    msg = json.loads(body.decode("utf-8", "replace"))
                except json.JSONDecodeError:
                    continue
                if msg.get("method") == "textDocument/publishDiagnostics":
                    params = msg.get("params", {})
                    self.diagnostics[params.get("uri", "")] = params.get("diagnostics", [])
                elif "id" in msg:
                    self.responses.put(msg)

    def send(self, msg: dict) -> None:
        payload = json.dumps({"jsonrpc": "2.0", **msg}).encode("utf-8")
        frame = b"Content-Length: %d\r\n\r\n" % len(payload) + payload
        assert self.proc.stdin
        try:
            self.proc.stdin.write(frame)
            self.proc.stdin.flush()
        except (BrokenPipeError, OSError, ValueError):
            # 服务器已退出:写入失败按"无响应"处理,交给 request 的超时兜底。
            # 注意 Windows 上向已关闭管道 flush 报的是 OSError(EINVAL),不是 BrokenPipeError。
            pass

    def request(self, method: str, params, timeout: float | None = None) -> dict[str, Any]:
        req_id = self.next_id
        self.next_id += 1
        self.send({"id": req_id, "method": method, "params": params})
        deadline = time.monotonic() + (timeout or self.default_timeout)
        while True:
            remain = deadline - time.monotonic()
            if remain <= 0:
                return {"error": "timeout"}
            try:
                msg = self.responses.get(timeout=remain)
            except queue.Empty:
                return {"error": "timeout"}
            if msg.get("id") == req_id:
                return msg

    def notify(self, method: str, params) -> None:
        self.send({"method": method, "params": params})

    def close(self) -> None:
        try:
            self.request("shutdown", None, timeout=2)
            self.notify("exit", {})
            time.sleep(0.3)
        finally:
            try:
                self.proc.kill()
            except OSError:
                pass
            # 显式关闭:否则解释器退出时 finalize 死管道会打“Exception ignored”回溯。
            for stream in (self.proc.stdin, self.proc.stdout, self.proc.stderr):
                if stream is not None:
                    try:
                        stream.close()
                    except OSError:
                        pass


def hover_targets(text: str, autoloads: list[str]) -> list[tuple[str, int, int]]:
    """三类关键 hover:自成员(func)、跨文件 class_name、autoload。"""
    lines = text.splitlines()
    wanted = [("self-member", re.compile(r"^\s*(?:static\s+)?func\s+([a-zA-Z_]\w*)\s*\(")),
              ("cross-file-class", re.compile(r"([A-Z]\w*)\.new\s*\("))]
    found: list[tuple[str, int, int]] = []
    for label, pat in wanted:
        for i, line in enumerate(lines):
            m = pat.search(line)
            if m:
                found.append((f"{label}:{m.group(1)}", i, m.start(1) + 1))
                break
    for name in autoloads[:1]:
        for i, line in enumerate(lines):
            col = line.find(name)
            if col >= 0:
                found.append((f"autoload:{name}", i, col + 1))
                break
    return found[:3]


def parse_autoloads(project: Path) -> list[str]:
    """从 project.godot 的 [autoload] 段取单例名(可被 `*` 前缀标记)。"""
    cfg = project / "project.godot"
    if not cfg.is_file():
        return []
    names, in_section = [], False
    for line in cfg.read_text(encoding="utf-8", errors="replace").splitlines():
        stripped = line.strip()
        if stripped.startswith("["):
            in_section = stripped == "[autoload]"
            continue
        if in_section and "=" in stripped and not stripped.startswith(";"):
            names.append(stripped.split("=", 1)[0].strip())
    return names


def cmd_lsp(args: argparse.Namespace) -> int:
    root = Path(args.workspace).resolve()
    project = Path(args.project).resolve() if args.project else root
    server = find_exe(args.server, "GODOTLSP", ("godotlsp", "godotlsp.exe"), "语言服务器", "--server")
    files, truncated = gd_files(project, args.max_files)
    if not files:
        fail(f"{project} 下没有 .gd")
    print(f"[lsp] 服务器={server}")
    print(f"[lsp] 工作区根(cwd)={root}")
    print(f"[lsp] 待查文件={len(files)} 个(来自 {project}{'; 已截断' if truncated else ''})")

    client = LspClient([server], root, args.timeout)
    init = client.request("initialize", {
        "processId": os.getpid(),
        "rootUri": root.as_uri(),
        "capabilities": {"textDocument": {"hover": {}, "documentSymbol": {}, "publishDiagnostics": {}}},
        "workspaceFolders": [{"uri": root.as_uri(), "name": root.name}],
    })
    caps = init.get("result")
    if not isinstance(caps, dict):
        # 服务器没起来/不响应时必须失败:否则收不到诊断会被当成"干净"(假通过)。
        print(f"[lsp] initialize 失败: {json.dumps(init, ensure_ascii=False)[:200]}")
        client.close()
        return 2
    client.notify("initialized", {})

    for path in files:
        client.notify("textDocument/didOpen", {"textDocument": {
            "uri": path.as_uri(), "languageId": "gdscript", "version": 1,
            "text": path.read_text(encoding="utf-8", errors="replace")}})
    time.sleep(args.settle)

    errors = 0
    for path in files:
        rel = path.relative_to(project).as_posix()
        diags = client.diagnostics.get(path.as_uri(), [])
        bad = [d for d in diags if d.get("severity") == SEV_ERROR]
        errors += len(bad)
        print(f"[lsp] DIAG {rel}: total={len(diags)} errors={len(bad)}")
        for d in diags[:6]:
            print(f"        - [sev{d.get('severity')}] {d.get('message', '').splitlines()[0][:120]}")

    for path in files:
        rel = path.relative_to(project).as_posix()
        syms = client.request("textDocument/documentSymbol", {"textDocument": {"uri": path.as_uri()}})
        result = syms.get("result") if isinstance(syms.get("result"), list) else None
        print(f"[lsp] SYMBOLS {rel}: {len(result) if result is not None else '无响应'} 项")

    autoloads = parse_autoloads(project)
    for path in files:
        rel = path.relative_to(project).as_posix()
        text = path.read_text(encoding="utf-8", errors="replace")
        for label, line, col in hover_targets(text, autoloads):
            resp = client.request("textDocument/hover", {
                "textDocument": {"uri": path.as_uri()}, "position": {"line": line, "character": col}})
            result = resp.get("result")
            if result is None:
                shown = resp.get("error", "null")
            else:
                value = result.get("contents", {})
                value = value.get("value", "") if isinstance(value, dict) else str(value)
                shown = " ".join(value.split())[:70]
            print(f"[lsp] HOVER {rel} {label}: {shown}")

    if autoloads:
        name = autoloads[0]
        # 落在真正引用了该单例的文件上:补全探针要有引用语境才有意义。
        pattern = re.compile(rf"\b{re.escape(name)}\b")
        target = next((p for p in files
                       if pattern.search(p.read_text(encoding="utf-8", errors="replace"))), files[0])
        text = target.read_text(encoding="utf-8", errors="replace")
        base = text if text.endswith("\n") else text + "\n"
        probe_line = f"\tvar _x = {name}."
        sent = base + "func _godot_verify_probe() -> void:\n" + probe_line + "\n"
        client.notify("textDocument/didChange", {"textDocument": {"uri": target.as_uri(), "version": 2},
                                                "contentChanges": [{"text": sent}]})
        time.sleep(args.settle)
        # 行号直接取自实际发送的文本,不用算术推——差一行就会拿到全局集而非成员集。
        line = sent.split("\n").index(probe_line)
        resp = client.request("textDocument/completion", {
            "textDocument": {"uri": target.as_uri()},
            "position": {"line": line, "character": len(probe_line)}})
        result = resp.get("result")
        result = result.get("items", []) if isinstance(result, dict) else (result or [])
        print(f"[lsp] COMPLETION {name}. -> {len(result)} 项")
        print(f"        {', '.join(i.get('label', '?') for i in result[:16])}")
    else:
        print("[lsp] COMPLETION 跳过(project.godot 无 [autoload])")

    stderr = "".join(client.stderr_chunks).strip().replace("\n", " | ")
    if stderr:
        print(f"[lsp] STDERR: {stderr[:300]}")
    client.close()
    print(f"[lsp] 结论: {'干净' if errors == 0 else f'有 {errors} 条 error 级诊断'}")
    return 0 if errors == 0 else 1


# --------------------------------------------------------------------------- main

def main() -> int:
    parser = argparse.ArgumentParser(description="Godot 工程验收门(引擎侧加载 + 分析侧 LSP)")
    sub = parser.add_subparsers(dest="mode", required=True)

    c = sub.add_parser("check", help="无头导入 + 运行 N 帧,只回显错误行")
    c.add_argument("project", help="工程目录(含 project.godot)")
    c.add_argument("--godot", default=None, help="Godot 可执行文件;默认取 GODOT 或 PATH")
    c.add_argument("--frames", type=int, default=120, help="运行帧数(默认 120)")
    c.set_defaults(func=cmd_check)

    l = sub.add_parser("lsp", help="stdio LSP 会话:诊断/符号/hover/补全")
    l.add_argument("workspace", help="工作区根 = 语言服务器 cwd")
    l.add_argument("--project", default=None, help="工程目录(默认同工作区根);用于复现根错位")
    l.add_argument("--server", default=None, help="godotlsp 可执行文件;默认取 GODOTLSP 或 PATH")
    l.add_argument("--max-files", type=int, default=10, help="最多打开的文件数(默认 10)")
    l.add_argument("--timeout", type=float, default=15.0, help="单请求超时秒(默认 15)")
    l.add_argument("--settle", type=float, default=2.5, help="等诊断推送的静默秒(默认 2.5)")
    l.set_defaults(func=cmd_lsp)

    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
