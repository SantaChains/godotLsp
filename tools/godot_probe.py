#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""把 GDScript 探针跑进 Godot 工程(临时副本)并汇总断言结果。

    uv run tools/godot_probe.py <工程目录> <探针.gd> [更多探针.gd ...] [--godot EXE] [--keep]

探针约定(见 tools/probes/*.gd):
  - `extends SceneTree`,用 `print("PASS | 说明")` / `print("FAIL | 说明")` 报每条断言;
  - 结尾必须 `print("RESULT failures=N")`——没有这一行就视为**未跑完**,按失败处理;
  - 需要 autoload 时用 `root.get_node("/root/X")` 运行时取(探针在 autoload 常量注册前被加载,
    解析期直接写 `RuleManager` 会 Compile Error)。

行为:
  - **永远在临时副本里跑**:探针会写进副本的 res:// 根,你的工程一个字节都不会被改;
  - 副本缺 `global_script_class_cache.cfg` 时先跑一次 `--import`(class_name 解析依赖"工程生成"产物);
  - 同时回显引擎的 ERROR/WARNING 行(带 `at:` 与 GDScript 回溯),它们同样会让本次失败。

退出码: 0 全过 / 1 有断言 FAIL、探针未跑完或引擎报 ERROR / 2 环境或用法问题。
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import NoReturn

ERROR_LINE_RE = re.compile(r"^(?:ERROR|USER ERROR|SCRIPT ERROR|SHADER ERROR|WARNING|USER WARNING):")
AT_LINE_RE = re.compile(r"^\s+at: ")
PASS_RE = re.compile(r"^PASS \|")
FAIL_RE = re.compile(r"^FAIL \|")
RESULT_RE = re.compile(r"^RESULT failures=(\d+)", re.MULTILINE)


def fail(message: str) -> NoReturn:
    print(f"[err] {message}", file=sys.stderr)
    sys.exit(2)


def find_godot(explicit: str | None) -> str:
    for cand in (explicit, os.environ.get("GODOT")):
        if cand:
            path = Path(cand)
            if path.is_file():
                return str(path)
            fail(f"Godot 可执行文件不存在: {cand}")
    for name in ("godot", "godot4", "Godot"):
        found = shutil.which(name)
        if found:
            return found
    fail("找不到 Godot 可执行文件。用 --godot 指定、或设 GODOT 环境变量、或放进 PATH。")


def noisy_lines(output: str) -> list[tuple[str, str]]:
    """ERROR/WARNING 行(带级别)与其紧随的 `at:` / GDScript 回溯。

    级别必须按**块**判定:回溯行紧跟警告时,不能算成独立错误。
    """
    lines = output.splitlines()
    kept: list[tuple[str, str]] = []
    for i, line in enumerate(lines):
        if not ERROR_LINE_RE.match(line):
            continue
        kind = "warning" if line.startswith(("WARNING:", "USER WARNING:")) else "error"
        kept.append((kind, line.rstrip()))
        j = i + 1
        while j < len(lines) and (AT_LINE_RE.match(lines[j])
                                  or lines[j].strip().startswith("GDScript backtrace")
                                  or re.match(r"^\s+\[\d+\] ", lines[j])):
            kept.append(("at", lines[j].rstrip()))
            j += 1
    return kept


def run(cmd: list[str], timeout: float) -> tuple[int, str]:
    try:
        proc = subprocess.run(cmd, check=False, capture_output=True, text=True,
                              timeout=timeout, errors="replace")
    except subprocess.TimeoutExpired:
        return 124, f"ERROR: 超时 {timeout:g}s\n"
    return proc.returncode, (proc.stdout or "") + (proc.stderr or "")


def main() -> int:
    parser = argparse.ArgumentParser(description="把 GDScript 探针跑进 Godot 工程(临时副本)")
    parser.add_argument("project", help="工程目录(含 project.godot)")
    parser.add_argument("probes", nargs="+", help="探针 .gd 文件")
    parser.add_argument("--godot", default=None, help="Godot 可执行文件;默认取 GODOT 或 PATH")
    parser.add_argument("--timeout", type=float, default=300.0, help="单个探针超时秒(默认 300)")
    parser.add_argument("--keep", action="store_true", help="保留临时副本并打印路径(调试用)")
    args = parser.parse_args()

    project = Path(args.project).resolve()
    if not (project / "project.godot").is_file():
        fail(f"{project} 下没有 project.godot(工程根传错了?)")
    probes = []
    for name in args.probes:
        path = Path(name)
        if not path.is_file():
            fail(f"探针不存在: {path}")
        probes.append(path)
    godot = find_godot(args.godot)

    workdir = Path(tempfile.mkdtemp(prefix="godot-probe-"))
    copy = workdir / project.name
    shutil.copytree(project, copy)
    print(f"[probe] 工程副本: {copy}")

    if not (copy / ".godot" / "global_script_class_cache.cfg").is_file():
        print("[probe] 副本缺工程生成产物 → 先跑一次 --import")
        code, output = run([godot, "--headless", "--path", str(copy), "--import"], args.timeout)
        noise = noisy_lines(output)
        for _, line in noise:
            print("        " + line)
        if code != 0 or any(kind == "error" for kind, _ in noise):
            print("[probe] import 阶段不干净,后续结果不可信")
            if not args.keep:
                shutil.rmtree(workdir, ignore_errors=True)
            return 1

    total_fail = 0
    for probe in probes:
        target = copy / probe.name
        shutil.copyfile(probe, target)
        code, output = run([godot, "--headless", "--path", str(copy),
                            "--script", f"res://{probe.name}"], args.timeout)
        passes = sum(1 for line in output.splitlines() if PASS_RE.match(line))
        fails = sum(1 for line in output.splitlines() if FAIL_RE.match(line))
        result = RESULT_RE.search(output)
        noise = noisy_lines(output)
        errors = [line for kind, line in noise if kind == "error"]

        print(f"===== {probe.name} =====")
        for line in output.splitlines():
            if FAIL_RE.match(line):
                print("  " + line)
        for _, line in noise:
            print("        " + line)
        verdict = "PASS" if (result and fails == 0 and not errors) else "FAIL"
        print(f"  断言 {passes} 通过 / {fails} 失败 · "
              f"{('RESULT failures=' + result.group(1)) if result else 'RESULT 缺失(探针未跑完)'}"
              f" · 引擎 ERROR={len(errors)} · exit={code} → {verdict}")
        if verdict == "FAIL":
            total_fail += 1
        target.unlink()  # 探针只留在副本里,顺手清掉

    if args.keep:
        print(f"[probe] 保留副本: {copy}")
    else:
        shutil.rmtree(workdir, ignore_errors=True)
    print(f"\nRESULT probes={len(probes)} failed={total_fail}")
    return 0 if total_fail == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
