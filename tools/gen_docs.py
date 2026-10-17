#!/usr/bin/env python3
"""godotlsp 文档站类参考生成器（W2 核心闭环）。

数据源（与 LSP 内核同源）：
  - crates/gdscript-engine/testdata/extension_api.json  签名/结构（唯一事实源）
  - godot-docs/gdd_*.md                                 英文描述语料（CC BY 3.0）

产出：
  - website/src/builtin/class_<slug>.md   每类一页，成员锚点显式 {#id} 锁定
  - website/src/SUMMARY.md                仅重写 BUILTIN 标记区间（按首字母分组）
  - website/src/builtin/index.md          仅重写 BUILTIN 标记区间（类目录）
  - crates/godotlsp/src/hoverdata.json    LSP hover 描述层（同一解析器派生，紧凑摘要）

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

sys.path.insert(0, str(Path(__file__).resolve().parent))
from translations import TRANSLATIONS


def tr(key: str) -> str | None:
    return TRANSLATIONS.get(key)


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


def parse_entries_full(body: str) -> dict[str, tuple[str, str]]:
    """'## X Descriptions' 区块 → {成员名: (签名行, 描述正文)}。

    与 parse_entries 同一解析（防漂移：hover 数据不得另写解析器），
    额外保留签名行供 @GDScript 内建函数渲染（api json 无该伪类）。
    """
    entries: dict[str, tuple[str, str]] = {}
    for chunk in re.split(r"^---\s*$", body, flags=re.M):
        chunk = chunk.strip()
        if not chunk:
            continue
        lines = chunk.split("\n")
        first = lines[0].strip()
        # 判定优先级：带 = 的「类型 名字 = 值」（属性/常量）> 函数签名（名字后跟括号）>
        # 裸「类型 名字」（无默认值属性）。函数形态不能先判——
        # `Vector2 position = Vector2(0, 0)` 的默认值括号会把名字误判成类型（W3 遗留 bug）。
        m = re.match(r"^(?:[\w:\[\]\.]+\s+)?([A-Za-z_]\w*)\s*=", first)
        if not m:
            m = re.search(r"(\w+)\s*\(", first)
        if not m:
            m = re.match(r"^(?:[\w:\[\]\.]+\s+)?([A-Za-z_]\w*)\s*$", first)
        name = m.group(1) if m else None
        if not name:
            continue
        desc = "\n".join(lines[1:]).strip()
        entries.setdefault(name, (first, desc))
    return entries


def parse_entries(body: str) -> dict[str, str]:
    """parse_entries_full 的描述投影（网站页渲染用，签名来自 api json）。"""
    return {name: desc for name, (_sig, desc) in parse_entries_full(body).items()}


def parse_overload_entries(body: str) -> list[tuple[str, str]]:
    """'## X Descriptions' 区块 → [(签名行, 描述)]，保序不去重。

    构造/运算符按重载并列（`int int()` / `int int(from: int)` …），
    parse_entries_full 的名字键会全部撞成 "int"——专用变体，
    与 parse_entries 同一 `---` 切分纪律（首行签名、其余描述）。
    """
    out: list[tuple[str, str]] = []
    for chunk in re.split(r"^---\s*$", body, flags=re.M):
        chunk = chunk.strip()
        if not chunk:
            continue
        lines = chunk.split("\n")
        out.append((lines[0].strip(), "\n".join(lines[1:]).strip()))
    return out


_SIG_DESC_RE = re.compile(r"^[\w\[\]]+\s+(?=operator\b|[\w]+\s*\()")


def _norm_sig(line: str) -> str:
    """签名归一化：表格 `type | sig` 与描述 `type sig` 两种形态统一为 `sig`。"""
    s = line.strip()
    s = re.sub(r"^[\w\[\]]+\s*\|\s*", "", s)
    s = _SIG_DESC_RE.sub("", s)
    return s.strip()


def builtin_ctor_op_data(blocks: dict[str, str]) -> tuple[list[dict], list[dict]]:
    """gdd 内建页构造/运算符数据：表格（Constructors/Operators）为权威条目集，
    Descriptions 按归一化签名匹配补描述。

    语料注记：表格尾部 `---` 水平分隔线需跳过；剔除后 int 表 39/描述 39、
    Vector2 表 18/描述 18，与 doc XML 计数完全一致（零漂移）。
    """
    def merge(table_block: str, desc_block: str) -> list[dict]:
        desc_by_norm = {_norm_sig(sig): d for sig, d in parse_overload_entries(desc_block)}
        out: list[dict] = []
        seen: set[str] = set()
        for line in table_block.splitlines():
            line = line.strip()
            if not line or line.startswith("---"):
                continue
            sig = _norm_sig(line)
            if not sig or sig in seen:
                continue
            seen.add(sig)
            out.append({"s": sig, "d": desc_by_norm.get(sig, "")})
        return out

    ctors = merge(blocks.get("Constructors", ""), blocks.get("Constructor Descriptions", ""))
    ops = merge(blocks.get("Operators", ""), blocks.get("Operator Descriptions", ""))
    return ctors, ops


def empty_docs() -> dict:
    return {"brief": "", "description": "", "methods": {}, "methods_full": {},
            "properties": {}, "signals": {}, "enums": {}, "enums_tree": {},
            "constants": {}, "constants_full": {}, "annotations": {}, "annotations_full": {},
            "ctors": [], "ops": []}


def parse_enum_entries(body: str) -> dict[str, dict[str, str]]:
    """Enumerations 区块 → {枚举名: {枚举值名: 描述}}。

    该区块与 Descriptions 不同构：**空行分隔、无 ---**（`enum Key` 头 + `Key KEY_LEFT = 0`
    值行 + 描述段落），parse_entries 对其失效（W3 遗留缺口：全站枚举值描述从未被提取）。
    """
    out: dict[str, dict[str, str]] = {}
    header_re = re.compile(r"^enum\s+(\w+)\s*$")
    value_re = re.compile(r"^(?:[\w:\[\]\.]+\s+)?([A-Z]\w*)\s*=\s*\S")
    cur_enum: str | None = None
    last_key: str | None = None
    buf: list[str] = []

    def emit() -> None:
        nonlocal last_key, buf
        if cur_enum is not None and last_key is not None:
            text = "\n".join(buf).strip()
            if text and last_key not in out[cur_enum]:
                out[cur_enum][last_key] = text
        buf = []

    for line in body.splitlines():
        mh = header_re.match(line)
        mv = value_re.match(line)
        if mh:
            emit()
            cur_enum = mh.group(1)
            out.setdefault(cur_enum, {})
            last_key = cur_enum  # 枚举级描述（若有）挂在枚举名上
        elif mv:
            emit()
            last_key = mv.group(1)
        else:
            buf.append(line)
    emit()
    return out


def class_docs(file: Path) -> dict:
    """提取类级描述与成员描述。"""
    blocks = split_blocks(file.read_text(encoding="utf-8"))
    desc = blocks.get("Description", "")
    # Description 取首段为 brief
    brief = desc.split("\n\n")[0].replace("\n", " ").strip() if desc else ""
    full_methods = parse_entries_full(blocks.get("Method Descriptions", ""))
    full_consts = parse_entries_full(blocks.get("Constants", ""))
    full_annos = parse_entries_full(blocks.get("Annotations", ""))
    enum_tree = parse_enum_entries(blocks.get("Enumerations", ""))
    # 扁平投影（页面/hover 渲染按名查）：枚举名 → 枚举级描述；枚举值名 → 值描述
    enums_flat: dict[str, str] = {}
    for ename, vals in enum_tree.items():
        enums_flat[ename] = vals.get(ename, "")
        for vname, vdesc in vals.items():
            if vname != ename:
                enums_flat[vname] = vdesc
    ret = {
        "brief": brief,
        "description": desc,
        "methods_full": full_methods,
        "methods": {k: v[1] for k, v in full_methods.items()},
        "properties": parse_entries(blocks.get("Property Descriptions", "")),
        "signals": parse_entries(blocks.get("Signals", "")),
        "enums_tree": enum_tree,
        "enums": enums_flat,
        "constants_full": full_consts,
        "constants": {k: v[1] for k, v in full_consts.items()},
        "annotations_full": full_annos,
        "annotations": {k: v[1] for k, v in full_annos.items()},
    }
    # C6.5 内建构造/运算符（仅内建页语料有该区块；类页 blocks 无 → 空列表）
    ret["ctors"], ret["ops"] = builtin_ctor_op_data(blocks)
    return ret


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


HOVER_DESC_CAP = 400


def hover_para(desc: str) -> str:
    """gdd 描述 → hover 摘要：首个实质段落（跳过 setter/getter 弹点与代码块），限长截断。"""
    if not desc:
        return ""
    for para in desc.split("\n\n"):
        lines = [ln for ln in para.splitlines() if ln.strip()]
        if not lines:
            continue
        # 跳过纯 bullet 段（属性 setter/getter 列表）与代码块
        if all(ln.lstrip().startswith(("- ", "```")) for ln in lines):
            continue
        text = " ".join(ln.strip() for ln in lines if not ln.lstrip().startswith("```"))
        text = text.strip()
        if len(text) > HOVER_DESC_CAP:
            text = text[: HOVER_DESC_CAP - 1].rstrip() + "…"
        return text
    return ""


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
    cn_brief = tr(f"class:{name}")
    if cn_brief or docs["brief"]:
        main_brief = cn_brief or docs["brief"]
        L.append(f"> {main_brief}")
        L.append("")
        if docs["description"]:
            L.append(f"<details><summary>官方英文描述</summary>")
            L.append("")
            L.append(docs["description"])
            L.append("")
            L.append("</details>")
            L.append("")

    # 构造函数（C6.5b，builtin 专属；重载并列无 ### 锚——同名会撞锚）
    if docs["ctors"]:
        L.append("## 构造函数")
        L.append("")
        for e in docs["ctors"]:
            L.append("```gdscript")
            L.append(e["s"])
            L.append("```")
            L.append("")
            if e["d"]:
                L.append(md_desc(e["d"]))
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
            cn = tr(f"member:{name}.{f['name']}")
            L.append(md_desc(cn or docs["methods"].get(f["name"], "")))
            L.append("")

    # 运算符（C6.5b，builtin 专属；同构造函数无 ### 锚）
    if docs["ops"]:
        L.append("## 运算符")
        L.append("")
        for e in docs["ops"]:
            L.append("```gdscript")
            L.append(e["s"])
            L.append("```")
            L.append("")
            if e["d"]:
                L.append(md_desc(e["d"]))
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
            pcn = tr(f"member:{name}.{p['name']}")
            L.append(md_desc(pcn or docs["properties"].get(p["name"], "")))
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
            scn = tr(f"member:{name}.{s['name']}")
            L.append(md_desc(scn or docs["signals"].get(s["name"], "")))
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
                ven = tr(f"member:{name}.{v['name']}")
                L.append(md_desc(ven or docs["enums"].get(v["name"], "")))
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
            ccn = tr(f"member:{name}.{cn}")
            L.append(md_desc(ccn or docs["constants"].get(cn, "")))
            L.append("")

    return "\n".join(L).rstrip() + "\n"


def strip_call(sig: str) -> str:
    """`@export()` → `@export`；`@export_category(name: String)` 原样。"""
    return sig[:-2] if sig.endswith("()") else sig


def gen_gdscript_page(docs: dict) -> str:
    """@GDScript 全量页（api json 无此伪类，签名/描述全部来自语料）：
    内建函数（len/preload/range/…）+ 常量（PI/TAU/…）+ 注解（@export/…）。
    锚点契约：函数/常量 #<原名>，注解 #annotation-<名>（hover 链接同源）。
    """
    L: list[str] = []
    L.append("# @GDScript")
    L.append("")
    if docs["brief"]:
        L.append(f"> {docs['brief']}")
        L.append("")
        L.append("<details><summary>官方英文描述（中文精译进行中）</summary>")
        L.append("")
        L.append(docs["description"])
        L.append("")
        L.append("</details>")
        L.append("")

    if docs["methods_full"]:
        L.append("## 内建函数")
        L.append("")
        for name in sorted(docs["methods_full"], key=str.lower):
            sig, desc = docs["methods_full"][name]
            L.append(f"### {name} {{#{name}}}")
            L.append("")
            L.append("```gdscript")
            L.append(sig)
            L.append("```")
            L.append("")
            L.append(md_desc(desc))
            L.append("")

    if docs["constants_full"]:
        L.append("## 常量")
        L.append("")
        for name in sorted(docs["constants_full"], key=str.lower):
            sig, desc = docs["constants_full"][name]
            L.append(f"### {name} {{#{name}}}")
            L.append("")
            L.append("```gdscript")
            L.append(f"const {sig}")
            L.append("```")
            L.append("")
            L.append(md_desc(desc))
            L.append("")

    if docs["annotations_full"]:
        L.append("## 注解")
        L.append("")
        for name in sorted(docs["annotations_full"], key=str.lower):
            sig, desc = docs["annotations_full"][name]
            disp = strip_call(sig)
            L.append(f"### {disp} {{#annotation-{name}}}")
            L.append("")
            L.append("```gdscript")
            L.append(disp)
            L.append("```")
            L.append("")
            L.append(md_desc(desc))
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
        cn = tr(f"member:@GlobalScope.{nm}")
        L.append(md_desc(cn or docs["methods"].get(nm, "")))
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
    """重写 BEGIN/END 标记区间。

    安全策略：SUMMARY 不含标记时**报错退出**而非整文件替换——
    曾经的整文件替换兜底把手写的语法/范式/内部篇条目全部吃掉（W2 事故）。
    """
    text = path.read_text(encoding="utf-8") if path.exists() else ""
    if BEGIN in text and END in text:
        pre, rest = text.split(BEGIN, 1)
        _, post = rest.split(END, 1)
        path.write_text(pre + new_block + post, encoding="utf-8", newline="\n")
        return True
    if path == SUMMARY:
        raise SystemExit(
            f"{path}: 缺少 BUILTIN 标记区间，拒绝整文件替换。"
            "请在手写 SUMMARY 中手动加入标记对：\n"
            f"{BEGIN}\n\n{END}"
        )
    path.write_text(new_block + "\n", encoding="utf-8", newline="\n")
    return False


# ───────────────────────── hover 描述层（LSP 消费端） ─────────────────────────

HOVERDATA = ROOT / "crates/godotlsp/src/hoverdata.json"


def build_hoverdata(entries: dict, docs_idx: dict) -> dict:
    """派生 LSP hover 描述层（与网站页同源同解析器——约束：LSP 渲染层不得自带描述文本）。

    结构：
      c  类 brief：{类名: 首段}
      m  成员描述：{"<类名>.<成员>": 首段}（方法/属性/信号/枚举/枚举值/常量；@GlobalScope 用类名键）
      f  @GDScript 内建函数：{名: {"s": 语料签名行, "d": 摘要}}
      a  注解：{名(去@): 摘要}
      k  内建构造/运算符（C6.5）：{类型: {"c"/"o": [{"s": 签名, "d": 摘要}]}}
    """
    c: dict[str, str] = {}
    m: dict[str, str] = {}
    f: dict[str, dict[str, str]] = {}
    a: dict[str, str] = {}
    k: dict[str, dict] = {}
    for name in entries:
        docs_file = docs_idx.get(name)
        docs = class_docs(docs_file) if docs_file else empty_docs()
        c[name] = docs["brief"]
        if name == "@GDScript":
            for n, (sig, desc) in docs["methods_full"].items():
                f[n] = {"s": sig, "d": hover_para(desc)}
            for n, (sig, desc) in docs["constants_full"].items():
                m[f"@GDScript.{n}"] = hover_para(desc)
            for n, (sig, desc) in docs["annotations_full"].items():
                a[n] = hover_para(desc)
            continue
        for member, desc in docs["methods"].items():
            m[f"{name}.{member}"] = hover_para(desc)
        for member, desc in docs["properties"].items():
            m[f"{name}.{member}"] = hover_para(desc)
        for member, desc in docs["signals"].items():
            m[f"{name}.{member}"] = hover_para(desc)
        for member, desc in docs["enums"].items():  # 枚举名与枚举值都在 Enumerations 区块
            m[f"{name}.{member}"] = hover_para(desc)
        for member, desc in docs["constants"].items():
            m[f"{name}.{member}"] = hover_para(desc)
        # C6.5：内建构造/运算符（重载并列，sig 键）
        if docs["ctors"] or docs["ops"]:
            k[name] = {}
            if docs["ctors"]:
                k[name]["c"] = [{"s": e["s"], "d": hover_para(e["d"])} for e in docs["ctors"]]
            if docs["ops"]:
                k[name]["o"] = [{"s": e["s"], "d": hover_para(e["d"])} for e in docs["ops"]]
    # P0 中文翻译覆盖
    for name in c:
        if t := tr(f"class:{name}"):
            c[name] = t
    for key in list(m):
        if t := tr(f"member:{key}"):
            m[key] = t
    for name in f:
        if t := tr(f"fn:{name}"):
            f[name]["d"] = t
    for name in a:
        if t := tr(f"anno:{name}"):
            a[name] = t
    # C6.5：构造/运算符中文覆盖（翻译键 `ctor:<类型>:<签名>` / `op:<类型>:<签名>`）
    for name, kinds in k.items():
        for kind, prefix in (("c", "ctor"), ("o", "op")):
            for e in kinds.get(kind, []):
                if t := tr(f"{prefix}:{name}:{e['s']}"):
                    e["d"] = t
    return {"c": c, "m": m, "f": f, "a": a, "k": k}


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
    gs_docs = class_docs(gs_docs_file) if gs_docs_file else empty_docs()

    all_names = sorted(entries, key=str.lower)
    written = 0
    for name in all_names:
        e = entries[name]
        docs_file = docs_idx.get(name)
        docs = class_docs(docs_file) if docs_file else empty_docs()
        if e["kind"] == "global_scope":
            content = gen_global_scope_page(api, gs_docs)
        elif e["kind"] == "gdscript":
            content = gen_gdscript_page(docs)
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

    # LSP hover 描述层（与网站同源同解析器）
    hover = build_hoverdata(entries, docs_idx)
    hover_text = json.dumps(hover, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    if check:
        if not HOVERDATA.exists() or HOVERDATA.read_text(encoding="utf-8") != hover_text:
            print("DRIFT: hoverdata.json")
            return 1
    else:
        HOVERDATA.write_text(hover_text, encoding="utf-8", newline="\n")

    mode = "check" if check else "generate"
    k_ct = sum(len(v.get("c", [])) for v in hover.get("k", {}).values())
    k_op = sum(len(v.get("o", [])) for v in hover.get("k", {}).values())
    print(f"{mode}: {written} class pages, {len(all_names)} entries, hover members {len(hover['m'])} + fns {len(hover['f'])} + annos {len(hover['a'])} + ctors {k_ct} + ops {k_op}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
