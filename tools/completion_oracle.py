#!/usr/bin/env python3
"""补全 oracle harness —— 用引擎自带补全测试夹具量化 godotlsp 与引擎 LSP 的差距。

语料:`corpus/godot-engine-tests/completion/`(引擎 `modules/gdscript/tests/scripts/completion/`,
pinned 9a05d57,由 `tools/fetch_corpus.py` 拉取,不入库)。夹具语义以上游
`.refs/godot/modules/gdscript/tests/test_completion.h` 为权威:

  - `.gd` 内 `➡`(U+27A1)是光标 sentinel(等价于上游的 0xFFFF);
  - `.cfg` 是 Godot ConfigFile(INI):
      `[input]`  `scene=res://...`(场景 owner)、`node_path`、`use_single_quotes`、
                `add_node_path_literals`、`add_string_name_literals`、`cs`
      `[output]` `include=[{display,insert_text,location}...]`(必须出现,容许超集)、
                `exclude=[...]`(不得出现)、`call_hint`、`forced`
  - 判定镜像上游 `match_option` + CHECK 逻辑:expected 字典中出现的每个键都须相等;
    `display` ↔ LSP `label`,`insert_text` ↔ `insertText`(缺省回退 `label`,
    与 LSP「无 insertText 时插入 label」同构);`location` 是引擎内部偏移,LSP 侧
    不可验证,忽略(全语料 6 处,均在 builtin_enum);`kind` 全语料未用(实测键频:
    display 599 / insert_text 46 / location 6);`call_hint`/`forced` 语料未用,不验。

匹配严格对齐引擎(精确相等),不做模糊——差距正是要量化的东西。未命中的 include 会
按成因分类(`paren-display` = 引擎 `name(…)` 展示约定,即路线图 C6.4;
`string-literal` = 引号面,C8;`absent` = 真缺)。

用法:
    python tools/completion_oracle.py [--server PATH] [--corpus DIR] [--filter S]
        [--timeout SEC] [--update-baseline] [--list] [--verbose]

  默认 server = target/debug/godotlsp.exe(不存在回退 release);
  默认 corpus = corpus/godot-engine-tests/completion;
  默认 baseline = tools/completion_oracle_baseline.json。

退出码:0 = 无回归(或无基线);1 = 相对基线有回归(pass→fail);2 = 环境/语料错误。
工作流:首跑 `--update-baseline` 建立普查基线;之后每期重跑,对应桶 fail→pass 即验收。
"""

from __future__ import annotations

import argparse
import json
import queue
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_CORPUS = REPO / "corpus/godot-engine-tests/completion"
DEFAULT_BASELINE = REPO / "tools/completion_oracle_baseline.json"
ARROW = "\u27a1"  # ➡ 光标 sentinel

# 分桶说明:已知缺口/明确不做项的路由图索引(打不进也先知道为什么)。
CATEGORY_NOTES = {
    "index": "字典键补全 = 明确不做 #3(需字面常量折叠),仅观察",
    "enum_values_in_dictionary": "字典内枚举值(依赖 #3,观察)",
    "argument_options": "注解/调用参数面(C8)+ 场景信号(C6.2)+ 字符串字面量",
    "get_node": "节点路径类型推断(语义层)+ 场景树(C7.3+)",
    "no_parenthesis_when_callable_is_expected": "callable 上下文不出括号(C6.4 附带)",
    "types": "类型推断面(语义层)",
}


# ---------------------------------------------------------------- cfg 解析

def godot_unquote(s: str) -> str:
    """Godot ConfigFile 字符串值:去外层引号 + 反转义(\\\" \\\\ \\n \\t)。"""
    s = s.strip()
    if len(s) >= 2 and s[0] == '"' and s[-1] == '"':
        s = s[1:-1]
    return (
        s.replace('\\"', '"')
        .replace("\\\\", "\\")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
    )


def parse_cfg(text: str) -> dict:
    """解析 ConfigFile 子集:section + 多行数组值 + 标量。

    返回 {"input": {...}, "output": {"include": [dict], "exclude": [dict]}}。
    数组收集用括号配平(尊重引号);`;` 行注释在数组内常见,整行剔除。
    """
    sections: dict[str, list[str]] = {}
    current = None
    for raw in text.splitlines():
        line = raw.strip()
        if not line or line.startswith(";"):
            continue
        m = re.match(r"^\[(\w+)]$", line)
        if m:
            current = m.group(1)
            sections.setdefault(current, [])
            continue
        if current is not None:
            sections[current].append(raw)

    def collect_array(body: str) -> list[dict]:
        # 括号配平截取 [ ... ](尊重字符串引号)
        start = body.find("[")
        if start < 0:
            return []
        depth, i, in_str = 0, start, False
        while i < len(body):
            c = body[i]
            if in_str:
                if c == "\\":
                    i += 2
                    continue
                if c == '"':
                    in_str = False
            elif c == '"':
                in_str = True
            elif c == "[":
                depth += 1
            elif c == "]":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        inner = body[start + 1 : i]
        # 去行注释后抽 {..} 字典
        inner = re.sub(r"^\s*;.*$", "", inner, flags=re.M)
        out = []
        for m in re.finditer(r"\{[^{}]*\}", inner, flags=re.S):
            d = {}
            for kv in re.finditer(r'"(\w+)"\s*:\s*("(?:[^"\\]|\\.)*"|-?\d+|true|false)', m.group(0)):
                k, v = kv.group(1), kv.group(2)
                d[k] = godot_unquote(v) if v.startswith('"') else (
                    int(v) if re.fullmatch(r"-?\d+", v) else v == "true"
                )
            if d:
                out.append(d)
        return out

    result: dict = {"input": {}, "output": {"include": [], "exclude": []}}
    for sec, lines in sections.items():
        buf = "\n".join(lines)
        for m in re.finditer(r"^(\w+)\s*=\s*(.*)$", buf, flags=re.M | re.S):
            key, val = m.group(1), m.group(2).strip()
            if sec == "input":
                result["input"][key] = godot_unquote(val) if val.startswith('"') else val
            elif key == "include":
                result["output"]["include"] = collect_array(val)
            elif key == "exclude":
                result["output"]["exclude"] = collect_array(val)
    return result


# ---------------------------------------------------------------- 夹具发现

class Fixture:
    __slots__ = ("rel", "gd", "cfg", "category", "text", "line", "character", "expect")

    def __init__(self, rel: str, gd: Path, cfg: Path):
        self.rel = rel.replace("\\", "/")  # Windows 下统一为正斜杠(分桶/基线键)
        self.gd = gd
        self.cfg = cfg
        self.category = self.rel.split("/")[0]
        raw = gd.read_text(encoding="utf-8-sig")
        idx = raw.find(ARROW)
        if idx < 0:
            raise ValueError(f"{rel}: 缺少 ➡ 光标 sentinel")
        # LSP position 是 UTF-16 码元列
        line_start = raw.rfind("\n", 0, idx) + 1
        self.line = raw.count("\n", 0, idx)
        self.character = len(raw[line_start:idx].encode("utf-16-le")) // 2
        self.text = raw[:idx] + raw[idx + 1 :]  # 删 sentinel,行内其余内容保留
        self.expect = parse_cfg(cfg.read_text(encoding="utf-8-sig"))

    @property
    def prefs(self) -> dict:
        """[input] 段三键 → 编辑器补全设置(镜像上游 test_completion.h:132-134
        每夹具 EditorSettings::set_setting;缺省 false = 引擎默认)。
        parse_cfg 的标量保持字符串("true"/"false"),须显式转 bool——
        bool("false") 是 True,曾致 dont_add 夹具回归。"""
        def as_bool(v) -> bool:
            return v is True or (isinstance(v, str) and v.strip().lower() == "true")
        inp = self.expect.get("input", {})
        return {
            "use_single_quotes": as_bool(inp.get("use_single_quotes", False)),
            "add_node_path_literals": as_bool(inp.get("add_node_path_literals", False)),
            "add_string_name_literals": as_bool(inp.get("add_string_name_literals", False)),
        }


def discover(corpus: Path) -> tuple[list[Fixture], list[str]]:
    fixtures, skipped = [], []
    for gd in sorted(corpus.rglob("*.gd")):
        if gd.name.endswith(".notest.gd"):
            continue  # 上游同款:.notest.gd 是夹具辅助文件,不参与判定
        cfg = gd.with_suffix(".cfg")
        if not cfg.exists():
            skipped.append(gd.relative_to(corpus).as_posix())
            continue
        fixtures.append(Fixture(gd.relative_to(corpus).as_posix(), gd, cfg))
    return fixtures, skipped


# ---------------------------------------------------------------- LSP 客户端

class Client:
    """最小 stdio 客户端(仿 crates/godotlsp/tests/bench_load.py 的既有形态)。"""

    def __init__(self, bin_path: str):
        self.proc = subprocess.Popen(
            [bin_path], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL)
        self.q: queue.Queue = queue.Queue()
        self.next_id = 100
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
                first = out.readline()
                if first in (b"\r\n", b"\n", b""):  # EOF 或空行先行:流结束
                    break
                line = first
                while True:
                    if line in (b"\r\n", b"\n", b""):
                        break
                    k, v = line.decode().strip().split(": ", 1)
                    headers[k.lower()] = v
                    line = out.readline()
                n = int(headers["content-length"])
                self.q.put(json.loads(out.read(n)))
        except (OSError, ValueError):
            pass
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
            if "method" in msg and "id" in msg:  # 服务器→客户端请求:应答
                result = [{}] if msg["method"] == "workspace/configuration" else None
                self.send({"jsonrpc": "2.0", "id": msg["id"], "result": result})
                continue
            if "method" in msg:
                continue  # 通知(diagnostics/logMessage)直接丢弃
            if msg.get("id") == rid:
                return msg
        return None

    def drain(self, window: float = 0.05) -> None:
        deadline = time.monotonic() + window
        while time.monotonic() < deadline:
            try:
                msg = self.q.get(timeout=max(0.0, deadline - time.monotonic()))
            except queue.Empty:
                return
            if msg.get("__eof__"):
                return
            if "method" in msg and "id" in msg:
                result = [{}] if msg["method"] == "workspace/configuration" else None
                self.send({"jsonrpc": "2.0", "id": msg["id"], "result": result})

    def stop(self) -> None:
        try:
            self.request("shutdown", None, 10)
            self.send({"jsonrpc": "2.0", "method": "exit", "params": None})
            self.proc.wait(timeout=10)
        except Exception:  # noqa: BLE001
            self.proc.kill()


# ---------------------------------------------------------------- 判定

def option_matches(item: dict, expected: dict) -> bool:
    """镜像上游 match_option:expected 出现的每个键都须相等。

    location(引擎内部偏移)LSP 侧不可验证,跳过;kind 语料未用。
    """
    if "display" in expected and expected["display"] != item.get("label", ""):
        return False
    if "insert_text" in expected:
        # LSP 线上是 camelCase(insertText);snake 兜底防手写夹具变体。
        got = item.get("insertText") or item.get("insert_text") or item.get("label", "")
        # 引擎 LSP 适配器(gdscript_language_protocol.cpp:562-568)在客户端
        # snippetSupport 时把 option.insert_text "name(" 改写为 "name($1)";
        # oracle 期望值是适配前的 option 值,比对时容许剥掉 "$1)"。
        candidates = {got}
        if got.endswith("$1)"):
            candidates.add(got[:-3])
        if expected["insert_text"] not in candidates:
            return False
    return True


def miss_reason(expected: dict) -> str:
    """未命中 include 的成因分类(供报告聚合,不影响判定)。"""
    disp = expected.get("display", "")
    if disp.endswith("(…)"):
        return "paren-display(C6.4)"
    if disp.startswith('"'):
        return "string-literal(C8)"
    return "absent"


def run_fixture(client: Client, ws: Path, fx: Fixture, timeout: float) -> dict:
    # 每夹具前同步设置(镜像上游 set_setting;服务端经 didChangeConfiguration
    # 更新 State.prefs)。settings 形状与服务端约定:{"gdscript": {...}}。
    client.send({"jsonrpc": "2.0", "method": "workspace/didChangeConfiguration", "params": {
        "settings": {"gdscript": fx.prefs}}})
    # 语料镜像在 ws/completion/ 下(见 build_workspace),夹具 uri 须与工程索引路径一致。
    uri = (ws / "completion" / fx.rel).as_uri()
    client.send({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
        "textDocument": {"uri": uri, "version": 1, "languageId": "gdscript",
                         "text": fx.text}}})
    resp = client.request("textDocument/completion", {
        "textDocument": {"uri": uri},
        "position": {"line": fx.line, "character": fx.character},
    }, timeout)
    client.send({"jsonrpc": "2.0", "method": "textDocument/didClose", "params": {
        "textDocument": {"uri": uri}}})
    client.drain()
    if resp is None or "result" not in resp:
        return {"status": "error", "detail": "无响应/超时"}
    items = resp["result"] or []
    labels = [i.get("label", "") for i in items]

    missing, illegal = [], []
    for e in fx.expect["output"]["include"]:
        if not any(option_matches(i, e) for i in items):
            missing.append(e)
    for e in fx.expect["output"]["exclude"]:
        hit = next((i for i in items if option_matches(i, e)), None)
        if hit is not None:
            illegal.append(e)
    ok = not missing and not illegal
    return {
        "status": "pass" if ok else "fail",
        "n_items": len(items),
        "missing": missing,
        "illegal": illegal,
        "labels_head": labels[:12],
    }


# ---------------------------------------------------------------- 工作区

def build_workspace(corpus: Path) -> Path:
    """镜像语料到临时工作区(res:// 布局):.gd/.tscn 原样拷贝 + 最小 project.godot。

    上游在引擎仓库根跑测试,夹具源内的引用路径形如 `res://completion/class_a.notest.gd`
    (脚本 extends / scene= 都是 res://completion/ 前缀)——因此语料必须镜像到
    `ws/completion/` 子目录(而非平铺根),使 res://completion/… 真实可解析;
    此前平铺导致 P5 脚本链(`extends "res://…"`)与场景路径全部断链。
    夹具 uri 相应取 ws/completion/<rel>(didOpen 文本由客户端注入,不依赖盘上路径;
    但工程索引按盘上路径建立,uri 必须与索引路径一致才能解析脚本链)。
    """
    ws = Path(tempfile.mkdtemp(prefix="gd-oracle-"))
    for p in corpus.rglob("*"):
        if p.is_file() and p.suffix in (".gd", ".tscn"):
            dest = ws / "completion" / p.relative_to(corpus)
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(p, dest)
    # 引擎测试环境自带 project.godot([input] test_input_action 等);
    # 语料根在 completion/ 上一级(godot-engine-tests/),两级都探。
    # argument_options/string_literals 的输入动作面依赖此文件
    # (此前写最小配置导致该面永远缺数据)。
    corpus_pg = corpus / "project.godot"
    if not corpus_pg.is_file():
        corpus_pg = corpus.parent / "project.godot"
    if corpus_pg.is_file():
        shutil.copy2(corpus_pg, ws / "project.godot")
    else:
        (ws / "project.godot").write_text(
            'config_version=5\n\n[application]\nconfig/name="oracle"\n',
            encoding="utf-8")
    return ws


# ---------------------------------------------------------------- 主流程

def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--server", help="godotlsp.exe 路径(默认 debug,回退 release)")
    ap.add_argument("--corpus", default=str(DEFAULT_CORPUS))
    ap.add_argument("--filter", default="", help="只跑 rel 路径含该子串的夹具")
    ap.add_argument("--timeout", type=float, default=15.0, help="单夹具请求超时(秒)")
    ap.add_argument("--baseline", default=str(DEFAULT_BASELINE))
    ap.add_argument("--update-baseline", action="store_true", help="重写基线(首跑普查用)")
    ap.add_argument("--list", action="store_true", help="只列夹具不跑")
    ap.add_argument("--verbose", action="store_true", help="失败明细全量输出")
    args = ap.parse_args()

    corpus = Path(args.corpus)
    if not corpus.is_dir():
        print(f"[oracle] 语料缺失:{corpus}(先跑 tools/fetch_corpus.py)", file=sys.stderr)
        return 2
    fixtures, skipped = discover(corpus)
    if args.filter:
        fixtures = [f for f in fixtures if args.filter in f.rel]
    if not fixtures:
        print("[oracle] 没有匹配夹具", file=sys.stderr)
        return 2
    if args.list:
        for f in fixtures:
            inc = len(f.expect["output"]["include"])
            exc = len(f.expect["output"]["exclude"])
            print(f"{f.rel:<70} include={inc:<3} exclude={exc}")
        print(f"[oracle] 共 {len(fixtures)} 个夹具(跳过无 cfg 的 {len(skipped)} 个)")
        return 0

    server = args.server
    if not server:
        for cand in (REPO / "target/debug/godotlsp.exe", REPO / "target/release/godotlsp.exe"):
            if cand.exists():
                server = str(cand)
                break
    if not server or not Path(server).exists():
        print("[oracle] 找不到 godotlsp.exe(cargo build 先)", file=sys.stderr)
        return 2

    print(f"[oracle] server={server} (mtime {time.strftime('%m-%d %H:%M', time.localtime(Path(server).stat().st_mtime))})")
    print(f"[oracle] corpus={corpus}  夹具={len(fixtures)}(无 cfg 跳过 {len(skipped)})")

    ws = build_workspace(corpus)
    try:
        client = Client(server)
        r = client.request("initialize", {
            "capabilities": {"textDocument": {"completion": {"completionItem": {
                "snippetSupport": True}}}},
            "rootUri": ws.as_uri()}, 30)
        if not r or "result" not in r:
            print("[oracle] initialize 失败", file=sys.stderr)
            return 2
        client.send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
        client.drain(0.5)  # 项目扫描在 initialize  handler 内同步完成,此处仅等异步静默

        results: dict[str, dict] = {}
        t0 = time.monotonic()
        for i, fx in enumerate(fixtures):
            results[fx.rel] = run_fixture(client, ws, fx, args.timeout)
            if (i + 1) % 20 == 0:
                print(f"[oracle]   ... {i + 1}/{len(fixtures)}")
        elapsed = time.monotonic() - t0
        client.stop()
    finally:
        shutil.rmtree(ws, ignore_errors=True)

    # 汇总
    cats: dict[str, dict] = {}
    for rel, res in results.items():
        cat = rel.split("/")[0] if "/" in rel else "(root)"
        c = cats.setdefault(cat, {"pass": 0, "fail": 0, "error": 0})
        c[res["status"] if res["status"] in c else "fail"] += 1
    n_pass = sum(c["pass"] for c in cats.values())
    n_fail = sum(c["fail"] for c in cats.values())
    n_err = sum(c["error"] for c in cats.values())
    print(f"[oracle] ---- 分桶({elapsed:.1f}s,{len(fixtures)} 夹具)----")
    for cat in sorted(cats):
        c = cats[cat]
        note = CATEGORY_NOTES.get(cat, "")
        print(f"[oracle] {cat:<38} {c['pass']:>3}/{c['pass'] + c['fail'] + c['error']:<4}"
              f"{'  ' + note if note else ''}")

    fails = {r: v for r, v in results.items() if v["status"] != "pass"}
    reason_stat: dict[str, int] = {}
    for v in fails.values():
        for e in v.get("missing", []):
            k = miss_reason(e)
            reason_stat[k] = reason_stat.get(k, 0) + 1
    print(f"[oracle] ---- 失败 include 成因 ----")
    for k, n in sorted(reason_stat.items(), key=lambda kv: -kv[1]):
        print(f"[oracle] {k:<24} {n}")

    show = fails if args.verbose else dict(list(fails.items())[:25])
    print(f"[oracle] ---- 失败明细({len(show)}/{len(fails)})----")
    for rel, v in show.items():
        if v["status"] == "error":
            print(f"[oracle] {rel}: {v['detail']}")
            continue
        miss = ", ".join(
            e.get("display", e.get("insert_text", "?")) for e in v["missing"][:4])
        bad = ", ".join(
            e.get("display", e.get("insert_text", "?")) for e in v["illegal"][:4])
        print(f"[oracle] {rel}: 缺[{miss}]" + (f" 多[{bad}]" if bad else ""))

    total = len(fixtures)
    print(f"[oracle] 总计 {n_pass}/{total} 通过({n_pass / total:.1%})"
          + (f",error {n_err}" if n_err else ""))

    # 基线比对 / 落盘
    baseline_path = Path(args.baseline)
    payload = {
        "generated_at": time.strftime("%Y-%m-%d %H:%M:%S"),
        "server": f"{server} mtime={time.strftime('%m-%d %H:%M', time.localtime(Path(server).stat().st_mtime))}",
        "corpus": str(corpus.relative_to(REPO)),
        "summary": {"pass": n_pass, "fail": n_fail, "error": n_err, "total": total},
        "fixtures": results,
    }
    if args.update_baseline:
        baseline_path.write_text(json.dumps(payload, ensure_ascii=False, indent=1),
                                 encoding="utf-8")
        print(f"[oracle] 基线已写入 {baseline_path}")
        return 0
    if baseline_path.exists():
        old = json.loads(baseline_path.read_text(encoding="utf-8"))
        old_fx = old.get("fixtures", {})
        regressed = [r for r, v in results.items()
                     if old_fx.get(r, {}).get("status") == "pass" and v["status"] != "pass"]
        improved = [r for r, v in results.items()
                    if old_fx.get(r, {}).get("status") in ("fail", "error") and v["status"] == "pass"]
        op, ot = old["summary"]["pass"], old["summary"]["total"]
        print(f"[oracle] 基线({old['generated_at']}): {op}/{ot};"
              f" 本次改善 {len(improved)} 项,回归 {len(regressed)} 项")
        for r in regressed:
            print(f"[oracle]   回归: {r}")
        if regressed:
            print(f"[oracle] 有回归——修复或确认后 --update-baseline")
            return 1
    else:
        print(f"[oracle] 无基线文件;首跑建议 --update-baseline 建立普查基线")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
