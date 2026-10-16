#!/usr/bin/env python3
"""godotlsp 文档站类参考生成器（W2 核心闭环）。

数据源（与 LSP 内核同源）：
  - crates/gdscript-engine/testdata/extension_api.json  签名/结构（唯一事实源）
  - godot-docs/gdd_*.md                                 英文描述语料（CC BY 3.0）

产出：
  - website/src/builtin/class_<slug>.md   每类一页，成员锚点显式 {#id} 锁定
  - website/src/SUMMARY.md                仅重写 BUILTIN 标记区间（按首字母分组）
  - website/src/builtin/index.md          仅重写 BUILTIN 标记区间（类目录）

锚点契约（与内核 doclinks.rs 一致，cargo test 防漂移）：
  类页   /builtin/class_<小写去@>.html
  成员   同页 #<原名>          枚举 #enum-<Enum名>

用法：python tools/gen_docs.py [--check]（--check 校验产物无漂移，CI 用）
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
API_JSON = ROOT / "crates/gdscript-engine/testdata/extension_api.json"
DOCS_DIR = ROOT / "godot-docs"
SITE_SRC = ROOT / "website/src"
BUILTIN_DIR = SITE_SRC / "builtin"
SUMMARY = SITE_SRC / "SUMMARY.md"

BEGIN = "<!-- BUILTIN:BEGIN（生成器区间，勿手改） -->"
END = "<!-- BUILTIN:END -->"


PSEUDO_CLASSES = {"@GlobalScope", "@GDScript"}

# 真实类名全集（collect() 填充），供伪类 slug 消歧。
REAL_NAMES: set[str] = set()
REAL_NAMES_LOWER: set[str] = set()


def slug_of(class_name: str) -> str:
    """类页 slug：class_ + 小写去 @（对齐 Sphinx 命名，规避 URL 特殊字符）。

    伪类（@ 前缀）去掉 @ 后若与真实类名冲突（如 @GDScript vs 真类 GDScript），
    加 at_ 前缀消解；无冲突则保持官方同名形态（@GlobalScope → class_globalscope）。
    """
    base = class_name.lower().replace("@", "")
    if class_name in PSEUDO_CLASSES and base in REAL_NAMES_LOWER:
        return "class_at_" + base
    return "class_" + base


def page_file(class_name: str) -> str:
    return f"{slug_of(class_name)}.md"


# ───────────────────────── 描述语料解析 ─────────────────────────

def load_docs_index() -> dict[str, Path]:
    """ClassName -> gdd md 文件（按文件名尾部类名匹配）。"""
    idx: dict[str, Path] = {}
    pat = re.compile(r"^gdd_\d+_(.+)\.md$")
    for f in sorted(DOCS_DIR.glob("gdd_*.md")):
        m = pat.match(f.name)
        if m:
            idx.setdefault(m.group(1), f)
    return idx


def split_blocks(text: str) -> dict[str, str]:
    """把 md 按 '## <Header>' 切成 {header: body}。"""
    out: dict[str, str] = {}
    cur, buf = "PREAMBLE", []
    for line in text.splitlines():
        m = re.match(r"^## (.+?)\s*$", line)
        if m:
            out[cur] = "\n".join(buf).strip()
            cur, buf = m.group(1), []
        else:
            buf.append(line)
    out[cur] = "\n".join(buf).strip()
    return out


def parse_entries(body: str) -> dict[str, str]:
    """'## X Descriptions'/'## Signals' 区块 → {成员名: 描述正文}。

    格式（godot-docs 提取语料，全站同构）：成员以签名行开头，成员之间 '---' 分隔。
    """
    entries: dict[str, str] = {}
    for chunk in re.split(r"^---\s*$", body, flags=re.M):
        chunk = chunk.strip()
        if not chunk:
            continue
        lines = chunk.split("\n")
        first = lines[0].strip()
        m = re.search(r"(\w+)\s*\(", first)
        name = m.group(1) if m else None
        if not name:
            m2 = re.match(r"^(?:[\w:\[\]\.]+\s+)?(\w+)\s*(?:=|$)", first)
            name = m2.group(1) if m2 else None
        if not name:
            continue
        desc = "\n".join(lines[1:]).strip()
        entries.setdefault(name, desc)
    return entries


def class_docs(file: Path) -> dict:
    """提取类级描述与成员描述。"""
    blocks = split_blocks(file.read_text(encoding="utf-8"))
    desc = blocks.get("Description", "")
    # Description 取首段为 brief
    brief = desc.split("\n\n")[0].replace("\n", " ").strip() if desc else ""
    return {
        "brief": brief,
        "description": desc,
        "methods": parse_entries(blocks.get("Method Descriptions", "")),
        "properties": parse_entries(blocks.get("Property Descriptions", "")),
        "signals": parse_entries(blocks.get("Signals", "")),
        "enums": parse_entries(blocks.get("Enumerations", "")),
        "constants": parse_entries(blocks.get("Constants", "")),
    }


# ───────────────────────── 签名渲染 ─────────────────────────

def default_str(v: str) -> str:
    """api json 的 default_value 是 GDScript 字面量串，直接用。"""
    return v if v not in ("",) else ""


def render_method(f: dict) -> str:
    args = []
    for a in f.get("arguments", []):
        s = f"{a['name']}: {a['type'].replace('enum::', '')}"
        if "default_value" in a:
            s += f" = {default_str(a['default_value'])}"
        args.append(s)
    if f.get("is_vararg"):
        args.append("...")
    # 类方法用 return_value.type，工具函数用 return_type（api json 双形态）。
    ret = f.get("return_value", {}).get("type") or f.get("return_type", "void")
    ret = ret.replace("enum::", "")
    prefix = ""
    if f.get("is_static"):
        prefix = "static "
    mods = []
    if f.get("is_virtual"):
        mods.append("virtual")
    if f.get("is_const") and not f.get("is_static"):
        mods.append("const")
    mod_sfx = (" " + " ".join(mods)) if mods else ""
    return f"{prefix}{ret} {f['name']}({', '.join(args)}){mod_sfx}"


def render_signal(sig: dict) -> str:
    args = ", ".join(f"{a['name']}: {a['type'].replace('enum::', '')}" for a in sig.get("arguments", []))
    return f"signal {sig['name']}({args})"


def render_property(p: dict) -> str:
    return f"{p['name']}: {p['type'].replace('enum::', '')}"


def render_enum(e: dict) -> str:
    vals = ", ".join(f"{v['name']} = {v['value']}" for v in e["values"])
    bit = "（位标志）" if e.get("is_bitfield") else ""
    return f"enum {e['name']}{bit} {{ {vals} }}"


# ───────────────────────── 页面生成 ─────────────────────────

def md_desc(desc: str) -> str:
    """gdd 描述 → 页面正文。轻处理：保留段落，空描述给占位。"""
    if not desc:
        return "> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。"
    return desc


def member_section(title: str, anchor_prefix: str | None, rows: list[str]) -> str:
    out = [f"## {title}", ""]
    out.extend(rows)
    return "\n".join(out)


def gen_class_page(name: str, cls: dict, kind: str, docs: dict, api: dict, inh_links: str) -> str:
    L: list[str] = []
    L.append(f"# {name}")
    L.append("")
    if cls.get("inherits"):
        L.append(f"**继承**：{inh_links}")
        L.append("")
    if docs["brief"]:
        L.append(f"> {docs['brief']}")
        L.append("")
        L.append(f"<details><summary>官方英文描述（中文精译进行中）</summary>")
        L.append("")
        L.append(docs["description"])
        L.append("")
        L.append("</details>")
        L.append("")

    # 方法
    methods = cls.get("methods", [])
    if methods:
        L.append("## 方法")
        L.append("")
        for f in sorted(methods, key=lambda x: x["name"]):
            L.append(f"### {f['name']} {{#{f['name']}}}")
            L.append("")
            L.append("```gdscript")
            L.append(render_method(f))
            L.append("```")
            L.append("")
            L.append(md_desc(docs["methods"].get(f["name"], "")))
            L.append("")

    # 属性（builtin 用 members）
    props = cls.get("properties") or cls.get("members") or []
    if props:
        L.append("## 属性")
        L.append("")
        for p in props:
            L.append(f"### {p['name']} {{#{p['name']}}}")
            L.append("")
            L.append("```gdscript")
            L.append(render_property(p))
            L.append("```")
            L.append("")
            L.append(md_desc(docs["properties"].get(p["name"], "")))
            L.append("")

    # 信号
    sigs = cls.get("signals", [])
    if sigs:
        L.append("## 信号")
        L.append("")
        for s in sigs:
            L.append(f"### {s['name']} {{#{s['name']}}}")
            L.append("")
            L.append("```gdscript")
            L.append(render_signal(s))
            L.append("```")
            L.append("")
            L.append(md_desc(docs["signals"].get(s["name"], "")))
            L.append("")

    # 枚举（容器锚 enum-<名>；每个值独立锚 <值名>，供 Class.VALUE 直达）
    enums = cls.get("enums", [])
    if enums:
        L.append("## 枚举")
        L.append("")
        for e in enums:
            L.append(f"### {e['name']} {{#enum-{e['name']}}}")
            L.append("")
            L.append("```gdscript")
            L.append(render_enum(e))
            L.append("```")
            L.append("")
            L.append(md_desc(docs["enums"].get(e["name"], "")))
            L.append("")
            for v in e["values"]:
                L.append(f"#### {v['name']} {{#{v['name']}}}")
                L.append("")
                L.append(f"```gdscript\n{v['name']} = {v['value']}\n```")
                L.append("")
                L.append(md_desc(docs["enums"].get(v["name"], "")))
                L.append("")

    # 常量（跳过已作为枚举值渲染的名字，避免重复锚点）
    enum_value_names = {v["name"] for e in cls.get("enums", []) for v in e["values"]}
    consts = [c for c in cls.get("constants", []) if c["name"] not in enum_value_names]
    if consts:
        L.append("## 常量")
        L.append("")
        for c in consts:
            cn, cv = c["name"], c["value"]
            L.append(f"### {cn} {{#{cn}}}")
            L.append("")
            L.append(f"```gdscript\nconst {cn} = {cv}\n```")
            L.append("")
            L.append(md_desc(docs["constants"].get(cn, "")))
            L.append("")

    return "\n".join(L).rstrip() + "\n"


def gen_global_scope_page(api: dict, docs: dict) -> str:
    """@GlobalScope 特殊页：utility functions + global enums + global constants + singletons。"""
    L: list[str] = []
    L.append("# @GlobalScope")
    L.append("")
    L.append("> " + docs["brief"] if docs["brief"] else "")
    L.append("")

    L.append("## 工具函数")
    L.append("")
    for f in api["utility_functions"]:
        nm = f["name"]
        L.append(f"### {nm} {{#{nm}}}")
        L.append("")
        L.append("```gdscript")
        L.append(render_method(f))
        L.append("```")
        L.append("")
        L.append(md_desc(docs["methods"].get(nm, "")))
        L.append("")

    L.append("## 全局枚举")
    L.append("")
    for e in api["global_enums"]:
        L.append(f"### {e['name']} {{#enum-{e['name']}}}")
        L.append("")
        L.append("```gdscript")
        L.append(render_enum(e))
        L.append("```")
        L.append("")
        L.append(md_desc(docs["enums"].get(e["name"], "")))
        L.append("")
        for v in e["values"]:
            L.append(f"#### {v['name']} {{#{v['name']}}}")
            L.append("")
            L.append(f"```gdscript\n{v['name']} = {v['value']}\n```")
            L.append("")
            L.append(md_desc(docs["enums"].get(v["name"], "")))
            L.append("")

    L.append("## 全局常量")
    L.append("")
    for c in api["global_constants"]:
        cn, cv = c["name"], c["value"]
        L.append(f"### {cn} {{#{cn}}}")
        L.append("")
        L.append(f"```gdscript\nconst {cn} = {cv}\n```")
        L.append("")
        L.append(md_desc(docs["constants"].get(cn, "")))
        L.append("")

    L.append("## 单例")
    L.append("")
    for s in api["singletons"]:
        L.append(f"- `{s['name']}` → [{s['type']}]({page_file(s['type'])})")
    L.append("")
    return "\n".join(L).rstrip() + "\n"


# ───────────────────────── SUMMARY / 索引 ─────────────────────────

def gen_summary_builtin(all_names: list[str]) -> str:
    groups: dict[str, list[str]] = {}
    for n in sorted(all_names, key=str.lower):
        key = "特殊" if n.startswith("@") else n[0].upper()
        groups.setdefault(key, []).append(n)
    parts = [BEGIN, "", "# 类参考", "", "- [类参考说明](builtin/index.md)", ""]
    for key in ["特殊"] + sorted(k for k in groups if k != "特殊"):
        parts.append(f"# 类参考 · {key}")
        parts.append("")
        for n in groups[key]:
            title = n
            parts.append(f"- [{title}](builtin/{page_file(n)})")
        parts.append("")
    parts.append(END)
    return "\n".join(parts)


def gen_builtin_index(all_names: list[str]) -> str:
    L = [BEGIN]
    L.append("# 类参考说明")
    L.append("")
    L.append("> 🚧 **生成页说明**：本目录所有 `class_*.md` 由 `tools/gen_docs.py` 从")
    L.append("> `crates/gdscript-engine/testdata/extension_api.json`（与 LSP 内核同源）+")
    L.append("> `godot-docs` 英文语料（CC BY 3.0）生成，**勿手改**，改动会被下次生成覆盖。")
    L.append("> 中文精译按 P0 高频 → P2 长尾推进，当前成员描述为英文原文。")
    L.append("")
    L.append(f"共 {len(all_names)} 类。签名与成员锚点为唯一事实源，与 LSP hover/补全一致；")
    L.append("锚点契约见[版本与适配说明](../intro/versions.md)。")
    L.append("")
    for n in sorted(all_names, key=str.lower):
        L.append(f"- [`{n}`]({page_file(n)})")
    L.append(END)
    return "\n".join(L) + "\n"


def rewrite_between(path: Path, new_block: str) -> bool:
    """重写 BEGIN/END 标记区间；文件无区间时整文件替换（仅限生成文件）。"""
    text = path.read_text(encoding="utf-8") if path.exists() else ""
    if BEGIN in text and END in text:
        pre, rest = text.split(BEGIN, 1)
        _, post = rest.split(END, 1)
        path.write_text(pre + new_block + post, encoding="utf-8", newline="\n")
        return True
    path.write_text(new_block + "\n", encoding="utf-8", newline="\n")
    return False


# ───────────────────────── 主流程 ─────────────────────────

def collect() -> tuple[dict[str, dict], dict[str, dict]]:
    global REAL_NAMES, REAL_NAMES_LOWER
    api = json.loads(API_JSON.read_text(encoding="utf-8"))
    docs_idx = load_docs_index()
    REAL_NAMES = {c["name"] for c in api["classes"]} | {b["name"] for b in api["builtin_classes"]}
    REAL_NAMES_LOWER = {n.lower() for n in REAL_NAMES}
    entries: dict[str, dict] = {}
    for c in api["classes"]:
        entries[c["name"]] = {"kind": "class", "cls": c}
    for b in api["builtin_classes"]:
        entries[b["name"]] = {"kind": "builtin", "cls": b}
    entries["@GlobalScope"] = {"kind": "global_scope", "cls": None}
    # @GDScript 语法级全局（api json 无此伪类）：从语料生成骨架
    entries["@GDScript"] = {"kind": "gdscript", "cls": None}
    return entries, docs_idx


def inherit_links(name: str, entries: dict) -> str:
    chain = []
    cur = entries.get(name, {}).get("cls") or {}
    while cur and cur.get("inherits"):
        parent = cur["inherits"]
        if parent in entries:
            chain.append(f"[{parent}]({page_file(parent)})")
        else:
            chain.append(parent)
        cur = entries.get(parent, {}).get("cls") or {}
    chain.append(f"**{name}**")
    return " → ".join(chain)


def main() -> int:
    check = "--check" in sys.argv
    entries, docs_idx = collect()
    BUILTIN_DIR.mkdir(parents=True, exist_ok=True)

    api = json.loads(API_JSON.read_text(encoding="utf-8"))
    gs_docs_file = docs_idx.get("@GlobalScope")
    gs_docs = class_docs(gs_docs_file) if gs_docs_file else {"brief": "", "description": "", "methods": {}, "properties": {}, "signals": {}, "enums": {}, "constants": {}}

    all_names = sorted(entries, key=str.lower)
    written = 0
    for name in all_names:
        e = entries[name]
        docs_file = docs_idx.get(name)
        docs = class_docs(docs_file) if docs_file else {"brief": "", "description": "", "methods": {}, "properties": {}, "signals": {}, "enums": {}, "constants": {}}
        if e["kind"] == "global_scope":
            content = gen_global_scope_page(api, gs_docs)
        elif e["kind"] == "gdscript":
            brief = docs["brief"]
            content = (
                f"# @GDScript\n\n"
                + (f"> {brief}\n\n" if brief else "")
                + "GDScript 语法级全局：内建函数（`print`/`range`/`preload`/…）与常量（`PI`/`TAU`/…）。\n"
                "完整成员清单见语法篇；此处骨架保证链接契约完备。\n\n"
                + (f"<details><summary>官方英文描述</summary>\n\n{docs['description']}\n\n</details>\n" if docs["description"] else "")
            )
        else:
            cls = e["cls"]
            if not cls:
                continue
            content = gen_class_page(name, cls, e["kind"], docs, api, inherit_links(name, entries))
        out = BUILTIN_DIR / page_file(name)
        if check:
            if not out.exists() or out.read_text(encoding="utf-8") != content:
                print(f"DRIFT: {out}")
                return 1
        else:
            out.write_text(content, encoding="utf-8", newline="\n")
        written += 1

    # SUMMARY / index
    if not check:
        rewrite_between(SUMMARY, gen_summary_builtin(all_names))
        rewrite_between(BUILTIN_DIR / "index.md", gen_builtin_index(all_names))

    mode = "check" if check else "generate"
    print(f"{mode}: {written} class pages, {len(all_names)} entries")
    return 0


if __name__ == "__main__":
    sys.exit(main())
