#!/usr/bin/env python3
"""从 godot-docs 语料提取 GDScript 全局面清单与独立文档,生成 Rust 静态表。

产出 `crates/godotlsp/src/gdscript_docs.rs`:
- SINGLETON_NAMES / UTILITY_NAMES / GLOBAL_ENUM_NAMES:名字清单(运行时经 gdscript-api
  模型逐一验证后才进入补全/hover,准确性由 extension_api.json 模型保证);
- ANNOTATION_DOCS / BUILTIN_FN_DOCS / CONST_DOCS:@GDScript 独立文档(模型不含)。

用法:python tools/extract_docs.py godot-docs crates/godotlsp/src/gdscript_docs.rs
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

MAX_DOC_CHARS = 3000
SIG_RE = re.compile(r"^(?:\w[\w<>\[\], .]*\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*\(")


def raw(s: str) -> str:
    """转成 Rust raw string 字面量,自动加 # 规避冲突。"""
    s = s.strip()
    hashes = "#"
    while f'"{hashes}' in s or f"{hashes}\"" in s:
        hashes += "#"
    return f'r{hashes}"{s}"{hashes}'


def split_entries(section: str) -> list[str]:
    """按独立成行的 `---` 分条。"""
    return [e.strip() for e in re.split(r"^---\s*$", section, flags=re.M) if e.strip()]


def clamp(s: str) -> str:
    s = re.sub(r"\n{3,}", "\n\n", s).strip()
    if len(s) > MAX_DOC_CHARS:
        s = s[:MAX_DOC_CHARS].rsplit("\n", 1)[0] + "\n\n…(截断,完整内容见官方文档)"
    return s


def parse_gdscript_md(text: str) -> tuple[list[tuple[str, str]], list[tuple[str, str]], list[tuple[str, str]]]:
    """返回 (annotations, builtin_fns, constants)。"""
    annotations_sec = text.split("## Annotations", 1)[1].split("\n## ", 1)[0]
    methods_sec = text.split("## Method Descriptions", 1)[1].split("\n## ", 1)[0]
    consts_sec = text.split("## Constants", 1)[1].split("\n## ", 1)[0]

    annotations = []
    for entry in split_entries(annotations_sec):
        m = re.match(r"^@([a-z_][a-z0-9_]*)\s*(?:\(.*?\))?(?:\s+vararg)?\s*$", entry.splitlines()[0])
        if not m:
            continue
        body = "\n".join(entry.splitlines()[1:]).strip()
        annotations.append((m.group(1), clamp(body)))

    builtin_fns = []
    for entry in split_entries(methods_sec):
        lines = entry.splitlines()
        m = SIG_RE.match(lines[0])
        if not m:
            continue
        sig = lines[0].strip()
        sig = re.sub(r"\s+vararg$", "(…)", sig)  # `void print ( ... ) vararg` 已含括号体,仅去尾标
        body = "\n".join(lines[1:]).strip()
        builtin_fns.append((m.group(1), clamp(f"```gdscript\n{sig}\n```\n\n{body}")))

    constants = []
    # 常量节不按 `---` 分条:以 `NAME = value` 行为块起点,正文延续到下一块。
    blocks = re.split(r"^(?=[A-Z_]+\s*=)", consts_sec, flags=re.M)
    for entry in blocks:
        entry = entry.strip()
        if not entry:
            continue
        lines = entry.splitlines()
        m = re.match(r"^([A-Z_]+)\s*=\s*(.+)$", lines[0])
        if not m:
            continue
        body = "\n".join(lines[1:]).strip()
        constants.append((m.group(1), clamp(body)))
    return annotations, builtin_fns, constants


def parse_globalscope_md(text: str) -> tuple[list[str], list[str], list[str]]:
    """返回 (singletons, utilities, global_enums)。"""
    props_sec = text.split("## Properties", 1)[1].split("\n## ", 1)[0]
    methods_sec = text.split("## Methods", 1)[1].split("\n## ", 1)[0]
    enums_sec = text.split("## Enumerations", 1)[1].split("\n## ", 1)[0]

    singletons = []
    for line in props_sec.splitlines():
        m = re.match(r"^([A-Za-z_]\w*)\s*\|", line.strip())
        if m:
            singletons.append(m.group(1))

    utilities = []
    for line in methods_sec.splitlines():
        m = re.match(r"^[\w\[\]]+\s*\|\s*([a-z_][a-z0-9_]*)\s*\(", line.strip())
        if m:
            utilities.append(m.group(1))

    enums = []
    for line in enums_sec.splitlines():
        m = re.match(r"^enum\s+([A-Za-z_]\w*)", line.strip())
        if m:
            enums.append(m.group(1))
    return singletons, utilities, enums


def parse_keywords_md(text: str) -> list[tuple[str, str]]:
    """GDScript reference 的 Keywords 表(Keyword | Description)。"""
    kw_sec = text.split("## Keywords", 1)[1].split("\n## ", 1)[0]
    keywords = []
    for line in kw_sec.splitlines():
        m = re.match(r"^([A-Za-z_][A-Za-z0-9_]*)\s*\|\s*(.+)$", line.strip())
        if m and m.group(1) not in ("Keyword",):
            keywords.append((m.group(1), re.sub(r"\s+", " ", m.group(2)).strip()))
    return keywords


def main() -> None:
    corpus_dir, out_path = Path(sys.argv[1]), Path(sys.argv[2])
    gd = (corpus_dir / "gdd_1590_@GDScript.md").read_text(encoding="utf-8")
    gs = (corpus_dir / "gdd_1591_@GlobalScope.md").read_text(encoding="utf-8")
    ref = (corpus_dir / "gdd_0304_GDScript_reference.md").read_text(encoding="utf-8")

    annotations, builtin_fns, constants = parse_gdscript_md(gd)
    singletons, utilities, enums = parse_globalscope_md(gs)
    keywords = parse_keywords_md(ref)

    def table(entries: list[tuple[str, str]]) -> str:
        return ",\n".join(f'    ({raw(k)}, {raw(v)})' for k, v in entries)

    def names(values: list[str]) -> str:
        return ",\n".join(f'    {raw(v)}' for v in values)

    out = f'''//! 构建期从 Godot 官方文档语料提取的全局面清单与独立文档——**勿手改**,
//! 由 `tools/extract_docs.py` 生成(源:gdd_1590_@GDScript.md / gdd_1591_@GlobalScope.md)。
//!
//! 名单(`*_NAMES`)运行时经 `gdscript-api` 模型逐一验证后才生效,签名、枚举值与
//! 类/成员文档一律以模型(`extension_api.json`)为权威;本表只补模型不枚举的名字
//! 与模型不携带的独立文档。

/// `@GlobalScope` 单例名(官方文档 Properties 表)。
pub const SINGLETON_NAMES: &[&str] = &[
{names(singletons)}
];

/// `@GlobalScope` 工具函数名(官方文档 Methods 表)。
pub const UTILITY_NAMES: &[&str] = &[
{names(utilities)}
];

/// `@GlobalScope` 全局枚举名(官方文档 Enumerations 表)。
pub const GLOBAL_ENUM_NAMES: &[&str] = &[
{names(enums)}
];

/// GDScript 注解文档(`@` 后缀名 → Markdown,官方文档原文)。
pub const ANNOTATION_DOCS: &[(&str, &str)] = &[
{table(annotations)}
];

/// GDScript 内建函数文档(`preload`/`range`/… → Markdown)。
pub const BUILTIN_FN_DOCS: &[(&str, &str)] = &[
{table(builtin_fns)}
];

/// `@GDScript` 伪常量文档(PI/TAU/INF/NAN → Markdown)。
pub const CONST_DOCS: &[(&str, &str)] = &[
{table(constants)}
];

/// GDScript 关键字一行说明(GDScript reference Keywords 表,含 PI/TAU 等伪常量别名)。
pub const KEYWORD_DOCS: &[(&str, &str)] = &[
{table(keywords)}
];
'''
    out_path.write_text(out, encoding="utf-8")
    print(f"annotations={len(annotations)} builtin_fns={len(builtin_fns)} constants={len(constants)}")
    print(f"singletons={len(singletons)} utilities={len(utilities)} enums={len(enums)} keywords={len(keywords)}")


if __name__ == "__main__":
    main()
