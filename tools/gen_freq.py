#!/usr/bin/env python3
"""C4:补全频率表生成器。

双语料挖掘 → rank 序名单 → crates/godotlsp/src/completion_freq.json（产物入库，
include_str! 内嵌——用户 clone 后必须可直接编译，语料缺失不算错误，算 SKIP 降级）。

语料（权重）：
  - godot-docs/**/*.md 的 ```gdscript 代码块（×1）——官方示例，覆盖广但偏教学
  - corpus/**/*.gd（×2）——真实工程代码，更能代表使用频率；目录缺失时 SKIP

候选过滤：只保留「已知 API 名」——extension_api.json 的类名/内建类型/方法名/
属性名/常量名/单例名/工具函数名/全局枚举值 + langdocs 关键字排除。用户自定义
标识符由 C3 词条面处理，这里只给 API 名排序。

输出：JSON 字符串数组，频率降序（rank = 数组下标，rank 0 最热）。
用法：python tools/gen_freq.py [--check]
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "godot-docs"
CORPUS = ROOT / "corpus"
API_JSON = ROOT / "crates/gdscript-engine/testdata/extension_api.json"
LANGDOCS = ROOT / "crates/godotlsp/src/langdocs.rs"
OUT = ROOT / "crates/godotlsp/src/completion_freq.json"
TOP_N = 4000

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
GD_BLOCK = re.compile(r"```gdscript\n(.*?)```", re.DOTALL)


def api_names() -> set[str]:
    """从 extension_api.json 收集全部 API 标识符。"""
    d = json.loads(API_JSON.read_text(encoding="utf-8"))
    names: set[str] = set()
    for cls in d["classes"] + d["builtin_classes"]:
        names.add(cls["name"])
        for m in cls.get("methods", []):
            names.add(m["name"])
        for p in cls.get("properties", []):
            names.add(p["name"])
        for c in cls.get("constants", []):
            names.add(c["name"])
        for e in cls.get("enums", []):
            for v in e.get("values", []):
                names.add(v["name"])
    for f in d["utility_functions"]:
        names.add(f["name"])
    for c in d["global_constants"]:
        names.add(c["name"])
    for e in d["global_enums"]:
        for v in e.get("values", []):
            names.add(v["name"])
    for s in d["singletons"]:
        names.add(s["name"])
    # langdocs 关键字表：KEYWORD_DOCS: &[(&str, &str, &str)] 的首元。
    kw_src = LANGDOCS.read_text(encoding="utf-8")
    kw_block = kw_src.split("const KEYWORD_DOCS", 1)[1].split("];", 1)[0]
    kw = set(re.findall(r'^\s*\(\s*"([a-z_0-9@$]+)"\s*,', kw_block, re.M))
    return names - kw


def count_idents(text: str, weight: int, counts: dict[str, int]) -> None:
    for m in IDENT.finditer(text):
        counts[m.group(0)] = counts.get(m.group(0), 0) + weight


def gather_counts() -> tuple[dict[str, int], list[str]]:
    counts: dict[str, int] = {}
    skips: list[str] = []

    if DOCS.is_dir():
        for md in DOCS.rglob("*.md"):
            for block in GD_BLOCK.findall(md.read_text(encoding="utf-8", errors="replace")):
                count_idents(block, 1, counts)
    else:
        skips.append("godot-docs")

    if CORPUS.is_dir():
        n_gd = 0
        for gd in CORPUS.rglob("*.gd"):
            count_idents(gd.read_text(encoding="utf-8", errors="replace"), 2, counts)
            n_gd += 1
        if n_gd == 0:
            skips.append("corpus(空)")
    else:
        skips.append("corpus")

    return counts, skips


def build_rank() -> tuple[list[str], list[str]]:
    counts, skips = gather_counts()
    known = api_names()
    hits = [(n, c) for n, c in counts.items() if n in known and c >= 2]
    hits.sort(key=lambda t: (-t[1], t[0]))  # 频率降序，同频按名（确定性）
    return [n for n, _ in hits[:TOP_N]], skips


def main() -> int:
    check = "--check" in sys.argv
    ranked, skips = build_rank()
    body = json.dumps(ranked, ensure_ascii=False, indent=0) + "\n"
    if check:
        if not OUT.exists():
            print("DRIFT: completion_freq.json 缺失")
            return 1
        if OUT.read_text(encoding="utf-8") != body:
            print("DRIFT: completion_freq.json")
            return 1
        print(f"OK completion_freq.json（{len(ranked)} 名）")
    else:
        OUT.write_text(body, encoding="utf-8")
        print(f"写 {OUT.relative_to(ROOT)}（{len(ranked)} 名）")
    for s in skips:
        print(f"SKIP: {s}（降级为部分语料）")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
