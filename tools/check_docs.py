#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Markdown 结构校验门：给"用脚本对 .md 做过逐行 splice/insert 手术"之后跑。

检查项（每项只在文档**确实用了该约定**时才启用，避免对别的文档误报）：

1. 代码围栏成对（``` 计数为偶数）
2. 没有空标题
3. 章节号（`## N.`）唯一、递增、从 0 连续
4. 表格：同一表格块内单元格数一致，且第 2 行是分隔行
5. 规则编号：凡被引用的 `Rn`（含 `R6-1` 这类子编号）都必须有定义
6. `§N` 交叉引用必须指向存在的章节
7. 引擎源码引用：围栏外（正文里写的引用）必须文件存在、行号在范围内、该行非空；
   围栏内视为"引用引擎原始输出"，只统计不判失败

用法：
    uv run tools/check_docs.py docs/godot-editing-rules.md [更多 .md ...] [--refs-root .refs/godot]

退出码：0 全过 / 1 有失败 / 2 用法问题
"""

from __future__ import annotations

import argparse
import re
import sys
from itertools import pairwise
from pathlib import Path
from typing import NoReturn

HEADING_RE = re.compile(r"^## (\d+)\.\s*(.+)$")
RULE_DEF_RE = re.compile(r"^\*\*R(\d+)(?:-\d+)?[\s*]")
RULE_REF_RE = re.compile(r"\bR(\d+)(?:[-–]\d+)?\b")
SEC_REF_RE = re.compile(r"§(\d+)")
TABLE_ROW_RE = re.compile(r"^\s*\|.*\|\s*$")
TABLE_SEP_RE = re.compile(r"^\s*\|[\s:|-]+\|\s*$")
CITE_RE = re.compile(r"((?:[\w-]+/)*[\w.-]+\.(?:cpp|h|xml)):(\d+)")


class Fail(Exception):
    pass


def fail(message: str) -> NoReturn:
    print(f"[err] {message}", file=sys.stderr)
    sys.exit(2)


def check(label: str, ok: bool, detail: str = "") -> int:
    # detail 只在失败时打印,避免 PASS 行携带空/噪声详情。
    print(("PASS | " if ok else "FAIL | ") + label + (f" → {detail}" if detail and not ok else ""))
    return 0 if ok else 1


def fence_mask(lines: list[str]) -> list[bool]:
    """标记每一行是否位于代码围栏内（围栏行本身也算在内）。"""
    mask, open_fence = [], False
    for line in lines:
        if line.lstrip().startswith("```"):
            open_fence = not open_fence
            mask.append(True)
            continue
        mask.append(open_fence)
    return mask


def check_tables(lines: list[str]) -> tuple[int, int]:
    fails = tables = 0
    i = 0
    while i < len(lines):
        if not TABLE_ROW_RE.match(lines[i]):
            i += 1
            continue
        j = i
        while j < len(lines) and TABLE_ROW_RE.match(lines[j]):
            j += 1
        block = lines[i:j]
        cells = [len(row.strip().strip("|").split("|")) for row in block]
        tables += 1
        fails += check(
            f"表格@L{i + 1}: {len(block)} 行 / {cells[0]} 列，列数一致",
            len(set(cells)) == 1,
            "/".join(map(str, cells)),
        )
        fails += check(
            f"表格@L{i + 1}: 第 2 行为分隔行",
            len(block) > 1 and bool(TABLE_SEP_RE.match(block[1])),
            repr(block[1] if len(block) > 1 else None),
        )
        i = j
    return fails, tables


def check_doc(path: Path, refs_root: Path) -> int:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    masked = fence_mask(lines)
    fails = 0

    fences = sum(1 for line in lines if line.lstrip().startswith("```"))
    fails += check(f"代码围栏成对（围栏行 {fences}，需为偶数）", fences % 2 == 0)
    fails += check("没有空标题", not any(re.fullmatch(r"#{1,6}\s*", line) for line in lines))

    heads = [(i + 1, int(m.group(1)), m.group(2)) for i, line in enumerate(lines) if (m := HEADING_RE.match(line))]
    nums = [n for _, n, _ in heads]
    if len(nums) >= 2:
        fails += check(f"章节号唯一（{nums}）", len(set(nums)) == len(nums))
        fails += check(f"章节号递增（{nums}）", all(b > a for a, b in pairwise(nums)))
        fails += check(f"章节号从 0 连续（{nums}）", nums == list(range(len(nums))))

    header_fails, tables = check_tables(lines)
    fails += header_fails
    print(f"PASS | 表格检查完成（{tables} 个表格）" if tables else "PASS | 无表格，跳过表格检查")

    defined = {m.group(1) for line in lines if (m := RULE_DEF_RE.match(line))}
    if len(defined) >= 3:
        referenced = {m.group(1) for m in RULE_REF_RE.finditer(text)}
        missing = sorted(referenced - defined, key=int)
        fails += check(
            f"规则编号引用均可解析（定义 {','.join(sorted(defined, key=int))}）",
            not missing,
            f"未定义: {','.join(missing)}",
        )

    if len(nums) >= 2:
        sec_refs = {int(m.group(1)) for m in SEC_REF_RE.finditer(text)}
        bad = sorted(sec_refs - set(nums))
        fails += check(
            f"§N 交叉引用均存在（引用 {'、'.join(f'§{n}' for n in sorted(sec_refs))}）",
            not bad,
            "不存在: " + "、".join(f"§{n}" for n in bad),
        )

    strict = quoted = cite_fails = 0
    declares_refs = str(refs_root).replace("\\", "/") in text
    if not declares_refs:
        # 文档没声明这个基线,里面的 foo.cpp:line 可能是 clangd/上游库等别的源码
        # (实测 docs/lsp-design.md 引的是 clangd 的 TUScheduler.cpp),不能当失效引用。
        print(f"PASS | 文档未声明 {refs_root} 基线，跳过源码引用检查")
    else:
        for i, line in enumerate(lines):
            for m in CITE_RE.finditer(line):
                rel = m.group(1)
                if "refs/godot/" in rel:
                    rel = rel.split("refs/godot/", 1)[1]
                lineno = int(m.group(2))
                if masked[i]:
                    quoted += 1
                    continue
                strict += 1
                target = refs_root / rel
                if not target.is_file():
                    cite_fails += 1
                    print(f"FAIL | 引用 {rel}:{lineno} 文件不存在")
                    continue
                body = target.read_text(encoding="utf-8", errors="replace").splitlines()
                if lineno > len(body):
                    cite_fails += 1
                    print(f"FAIL | 引用 {rel}:{lineno} 超出文件行数（{len(body)}）")
                elif not (body[lineno - 1] or "").strip():
                    cite_fails += 1
                    print(f"FAIL | 引用 {rel}:{lineno} 该行为空（引用可能失效）")
    if declares_refs and (strict or quoted):
        fails += check(f"围栏外源码引用全部有效（{strict} 处）", cite_fails == 0, f"{cite_fails} 处失效")
        print(f"PASS | 围栏内为引用引擎原始输出（{quoted} 处，不参与严格校验）")
    return fails


def main() -> int:
    parser = argparse.ArgumentParser(description="Markdown 结构校验门")
    parser.add_argument("docs", nargs="+", help="要校验的 .md")
    parser.add_argument("--refs-root", default=".refs/godot", help="引擎源码根（默认 .refs/godot）")
    args = parser.parse_args()

    refs_root = Path(args.refs_root)
    total = 0
    for name in args.docs:
        path = Path(name)
        if not path.is_file():
            fail(f"{path} 不存在")
        print(f"===== {path} =====")
        total += check_doc(path, refs_root)
    print(f"\nRESULT failures={total}")
    return 0 if total == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
