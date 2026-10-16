#!/usr/bin/env python3
"""LSP 链接契约校验（规划文档 §五之三 §E）。

双向校验：
  正向（出口→锚点）：langdocs.rs KEYWORD_DOCS/OPERATOR_DOCS 的每个 `slug#anchor`
    目标，必须在 mdbook 产物 website/book/syntax/<slug>.html 中存在 id="anchor"。
  反向（锚点→出口）：语法篇源码中定义的每个 kw-*/op-* 锚，必须被 langdocs
    两表消费（防悬空锚；op-index 索引节锚与 c-* 概念锚不在约束范围）。

用法：python tools/check_lsp_contract.py
退出码：0 全过；1 有违约（逐条打印）。
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LANGDOCS = ROOT / "crates/godotlsp/src/langdocs.rs"
BOOK = ROOT / "website/book/syntax"
SRC = ROOT / "website/src/syntax"

KW_RE = re.compile(r'\("(\w+)",\s*r#"(.*?)"#,\s*"([\w-]+)#([\w-]+)"\)')
OP_RE = re.compile(r'\("([^"]+)",\s*r#"(.*?)"#,\s*"([\w-]+)"\)')
BLOCK_RE = re.compile(r"const (\w+): &\[.*?\] = &\[(.*?)\];", re.S)


def parse_tables() -> tuple[list[tuple[str, str]], list[tuple[str, str]]]:
    """→ (keyword targets [(slug, anchor)], operator anchors [(sym, anchor)])。"""
    text = LANGDOCS.read_text(encoding="utf-8")
    tables = dict(BLOCK_RE.findall(text))
    if "KEYWORD_DOCS" not in tables or "OPERATOR_DOCS" not in tables:
        sys.exit("langdocs.rs 解析失败：找不到 KEYWORD_DOCS/OPERATOR_DOCS 表")
    kw = [(m[2], m[3]) for m in KW_RE.findall(tables["KEYWORD_DOCS"])]
    op = [(m[0], m[2]) for m in OP_RE.findall(tables["OPERATOR_DOCS"])]
    return kw, op


def main() -> int:
    kw, op = parse_tables()
    errors: list[str] = []

    # ── 正向：出口 → 锚点 ──
    html_cache: dict[str, str] = {}

    def book_html(slug: str) -> str:
        if slug not in html_cache:
            p = BOOK / f"{slug}.html"
            if not p.exists():
                sys.exit(f"产物缺失: website/book/syntax/{slug}.html（先 mdbook build website）")
            html_cache[slug] = p.read_text(encoding="utf-8")
        return html_cache[slug]

    for slug, anchor in kw:
        if f'id="{anchor}"' not in book_html(slug):
            errors.append(f"[出口→锚点] {slug}.html 缺 id=\"{anchor}\"（langdocs KEYWORD_DOCS 引用了它）")
    for sym, anchor in op:
        if f'id="{anchor}"' not in book_html("06-operators"):
            errors.append(f"[出口→锚点] 06-operators.html 缺 id=\"{anchor}\"（操作符 {sym}）")

    # ── 反向：锚点 → 出口（防悬空锚）──
    kw_anchors = {a for _, a in kw}
    op_anchors = {a for _, a in op}
    defined: dict[str, list[str]] = {}
    for md in sorted(SRC.glob("*.md")):
        text = md.read_text(encoding="utf-8")
        ids = re.findall(r'\{#([\w-]+)\}', text) + re.findall(r'<a id="([\w-]+)"></a>', text)
        for i in ids:
            if i.startswith(("kw-", "op-")) and i != "op-index":
                defined.setdefault(i, []).append(md.name)

    for anchor, pages in defined.items():
        pool = kw_anchors if anchor.startswith("kw-") else op_anchors
        if anchor not in pool:
            errors.append(f"[锚点→出口] {pages[0]} 定义了 {anchor} 但 langdocs 表未消费（悬空锚）")
    for anchor in kw_anchors:
        if anchor not in defined:
            errors.append(f"[锚点→出口] KEYWORD_DOCS 目标 {anchor} 在语法篇源码中无定义")
    for anchor in op_anchors:
        if anchor not in defined:
            errors.append(f"[锚点→出口] OPERATOR_DOCS 目标 {anchor} 在语法篇源码中无定义")

    # ── 报告 ──
    print(f"关键字目标: {len(kw)} 条 | 操作符锚点: {len(op)} 条 | 页面锚点: {len(defined)} 个")
    if errors:
        print(f"\n契约违约 {len(errors)} 条:")
        for e in errors:
            print("  ✗", e)
        return 1
    print("契约校验全过：出口↔锚点双向闭环。")
    return 0


if __name__ == "__main__":
    sys.exit(main())
