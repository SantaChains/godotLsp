#!/usr/bin/env python3
"""从引擎源码提取 internal 属性清单（P6）。

引擎补全排除 PROPERTY_USAGE_INTERNAL 成员（oracle filter/usage_internal：
Translation.messages——extension_api.json 的 properties 无 usage 标记，
只能从源码 ADD_PROPERTY 提取）。

扫描 `.refs/godot/**/*.cpp`：
- 每个文件记录 `void ClassName::_bind_methods()` 的当前类名；
- `ADD_PROPERTY(PropertyInfo(..., "name", ... PROPERTY_USAGE_INTERNAL ...), ...)`
  归属当前类，取 PropertyInfo 第二个字符串参数为属性名。

输出 `crates/gdscript-engine/testdata/internal_props.json`：
  {"ClassName": ["prop", ...], ...}

自建派生数据 → 入库（与 hoverdata.json 同律）。重跑：`python tools/gen_internal_props.py`。
"""

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
ENGINE_SRC = REPO / ".refs" / "godot"
OUT = REPO / "crates" / "gdscript-engine" / "testdata" / "internal_props.json"

BIND_RE = re.compile(r"void\s+(\w+)::_bind_methods\(\)")
ADD_PROP_RE = re.compile(r'ADD_PROPERTY\s*\(\s*PropertyInfo\s*\(')
# PropertyInfo(Variant::X, "name", ...) —— 第二参数字符串（整行 search）。
PROP_NAME_RE = re.compile(r'Variant::\w+\s*,\s*"([^"]+)"')


def extract() -> dict[str, list[str]]:
    out: dict[str, set[str]] = {}
    files = sorted(ENGINE_SRC.rglob("*.cpp")) if ENGINE_SRC.exists() else []
    if not files:
        print(f"[gen] 错误：引擎源码不存在 {ENGINE_SRC}（先 fetch 引擎仓）", file=sys.stderr)
        sys.exit(2)
    for f in files:
        text = f.read_text(encoding="utf-8", errors="replace")
        current_class: str | None = None
        in_addprop = False
        for raw in text.splitlines():
            line = raw.strip()
            if m := BIND_RE.search(line):
                current_class = m.group(1)
                continue
            if ADD_PROP_RE.search(line):
                # ADD_PROPERTY 开始：整行或后续行（多行宏）含 INTERNAL 都收。
                in_addprop = not line.rstrip().endswith("),")
                if "PROPERTY_USAGE_INTERNAL" in line and current_class is not None:
                    if m := PROP_NAME_RE.search(line):
                        out.setdefault(current_class, set()).add(m.group(1))
                continue
            if in_addprop:
                if "PROPERTY_USAGE_INTERNAL" in line and current_class is not None:
                    if m := PROP_NAME_RE.search(line):
                        out.setdefault(current_class, set()).add(m.group(1))
                if line.rstrip().endswith("),"):
                    in_addprop = False
    return {k: sorted(v) for k, v in sorted(out.items())}


def main() -> int:
    data = extract()
    n_cls = len(data)
    n_props = sum(len(v) for v in data.values())
    OUT.write_text(json.dumps(data, indent=1, sort_keys=True) + "\n", encoding="utf-8")
    print(f"[gen] {OUT.relative_to(REPO)}  类={n_cls}  internal 属性={n_props}")
    # 抽检锚点（引擎 translation.cpp:277）。
    assert "messages" in data.get("Translation", []), "Translation.messages 缺失——提取逻辑失效"
    print("[gen] 抽检 Translation.messages ✓")
    return 0


if __name__ == "__main__":
    sys.exit(main())
