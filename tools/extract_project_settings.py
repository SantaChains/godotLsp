#!/usr/bin/env python3
"""从 Godot 源码提取 project.godot 设置键数据集(供 VS Code 插件补全/hover 使用)。

数据源:.refs/godot(Godot 引擎源码)中的 GLOBAL_DEF 宏家族注册点——
    GLOBAL_DEF / GLOBAL_DEF_RST / GLOBAL_DEF_NOVAL / GLOBAL_DEF_BASIC /
    GLOBAL_DEF_RST_BASIC / GLOBAL_DEF_NOVAL_BASIC / GLOBAL_DEF_RST_NOVAL_BASIC /
    GLOBAL_DEF_INTERNAL
两种形态:
    GLOBAL_DEF("application/run/max_fps", 0)
    GLOBAL_DEF(PropertyInfo(Variant::INT, "physics/common/physics_ticks_per_second",
                            PROPERTY_HINT_RANGE, "1,1000,1"), 60)
宏定义见 core/config/project_settings.h(_GLOBAL_DEF 签名含 restart/noval/basic/internal 标志)。

.gdextension 与 .import 的节/键位图不来自宏,直接核验自源码:
    core/extension/gdextension_library_loader.cpp(parse_gdextension_file)
    editor/file_system/editor_file_system.cpp(_save_import_file 写 [remap]/[deps]/[params])

用法:
    python tools/extract_project_settings.py [godot_src_dir] [-o 输出.json]

godot_src_dir 默认取仓库根的 .refs/godot(本地参考源码,不入库);
输出 JSON 入库——clone 后无需 Godot 源码即可用。
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

MACRO_RE = re.compile(
    r"\b(GLOBAL_DEF(?:_RST_NOVAL_BASIC|_RST_NOVAL|_NOVAL_BASIC|_RST_BASIC|_NOVAL|_BASIC|_RST|_INTERNAL)?)\s*\("
)

VARIANT_TYPE_MAP = {
    "NIL": "nil",
    "BOOL": "bool",
    "INT": "int",
    "FLOAT": "float",
    "STRING": "string",
    "STRING_NAME": "string_name",
    "NODE_PATH": "node_path",
    "VECTOR2": "vector2",
    "VECTOR2I": "vector2i",
    "VECTOR3": "vector3",
    "VECTOR3I": "vector3i",
    "VECTOR4": "vector4",
    "VECTOR4I": "vector4i",
    "RECT2": "rect2",
    "RECT2I": "rect2i",
    "TRANSFORM2D": "transform2d",
    "TRANSFORM3D": "transform3d",
    "PLANE": "plane",
    "QUATERNION": "quaternion",
    "AABB": "aabb",
    "BASIS": "basis",
    "PROJECTION": "projection",
    "COLOR": "color",
    "RID": "rid",
    "OBJECT": "object",
    "CALLABLE": "callable",
    "SIGNAL": "signal",
    "DICTIONARY": "dictionary",
    "ARRAY": "array",
    "PACKED_BYTE_ARRAY": "packed_byte_array",
    "PACKED_INT32_ARRAY": "packed_int32_array",
    "PACKED_INT64_ARRAY": "packed_int64_array",
    "PACKED_FLOAT32_ARRAY": "packed_float32_array",
    "PACKED_FLOAT64_ARRAY": "packed_float64_array",
    "PACKED_STRING_ARRAY": "packed_string_array",
    "PACKED_VECTOR2_ARRAY": "packed_vector2_array",
    "PACKED_VECTOR3_ARRAY": "packed_vector3_array",
    "PACKED_VECTOR4_ARRAY": "packed_vector4_array",
    "PACKED_COLOR_ARRAY": "packed_color_array",
}

HINT_MAP = {
    "NONE": "none",
    "RANGE": "range",
    "ENUM": "enum",
    "ENUM_SUGGESTION": "enum_suggestion",
    "EXPRESSION": "expression",
    "LAYERS_2D_RENDER": "layers_2d_render",
    "LAYERS_2D_PHYSICS": "layers_2d_physics",
    "LAYERS_2D_NAVIGATION": "layers_2d_navigation",
    "LAYERS_3D_RENDER": "layers_3d_render",
    "LAYERS_3D_PHYSICS": "layers_3d_physics",
    "LAYERS_3D_NAVIGATION": "layers_3d_navigation",
    "FILE": "file",
    "DIR": "dir",
    "GLOBAL_FILE": "global_file",
    "GLOBAL_DIR": "global_dir",
    "RESOURCE_TYPE": "resource_type",
    "NODE_TYPE": "node_type",
    "PLACEHOLDER_TEXT": "placeholder_text",
    "PROPERTY_OF_VARIANT_TYPE": "property_of_variant_type",
    "OBJECT_ID": "object_id",
    "TYPE_STRING": "type_string",
    "LOCALE_ID": "locale_id",
    "MAX": "max",
}

# 宏名后缀 → 标志位(核验自 project_settings.h 的 _GLOBAL_DEF 形参表)。
MACRO_FLAGS = {
    "GLOBAL_DEF": {},
    "GLOBAL_DEF_RST": {"restart": True},
    "GLOBAL_DEF_NOVAL": {"noval": True},
    "GLOBAL_DEF_RST_NOVAL": {"restart": True, "noval": True},
    "GLOBAL_DEF_BASIC": {"basic": True},
    "GLOBAL_DEF_RST_BASIC": {"restart": True, "basic": True},
    "GLOBAL_DEF_NOVAL_BASIC": {"noval": True, "basic": True},
    "GLOBAL_DEF_RST_NOVAL_BASIC": {"restart": True, "noval": True, "basic": True},
    "GLOBAL_DEF_INTERNAL": {"internal": True},
}


def split_top_level(text: str) -> list[str]:
    """按顶层逗号切分(跳过字符串字面量与嵌套括号)。"""
    args: list[str] = []
    depth = 0
    in_str = False
    cur: list[str] = []
    i = 0
    while i < len(text):
        c = text[i]
        if in_str:
            cur.append(c)
            if c == "\\" and i + 1 < len(text):
                cur.append(text[i + 1])
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if c == '"':
            in_str = True
            cur.append(c)
        elif c in "([{":
            depth += 1
            cur.append(c)
        elif c in ")]}":
            depth -= 1
            cur.append(c)
        elif c == "," and depth == 0:
            args.append("".join(cur))
            cur = []
        else:
            cur.append(c)
        i += 1
    if cur:
        args.append("".join(cur))
    return args


def read_call_args(src: str, open_paren: int) -> tuple[str, int]:
    """从 open_paren(指向 '(')读取平衡括号,返回(实参文本, 闭合括号后位置)。"""
    depth = 0
    in_str = False
    i = open_paren
    while i < len(src):
        c = src[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if c == '"':
            in_str = True
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return src[open_paren + 1 : i], i + 1
        i += 1
    return src[open_paren + 1 :], len(src)


def read_braces(src: str, open_brace: int) -> tuple[str, int]:
    """从 open_brace(指向 '{')读取平衡花括号,返回(函数体文本, 闭合括号后位置)。"""
    depth = 0
    in_str = False
    i = open_brace
    while i < len(src):
        c = src[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if c == '"':
            in_str = True
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return src[open_brace + 1 : i], i + 1
        i += 1
    return src[open_brace + 1 :], len(src)


def in_line_comment(src: str, pos: int) -> bool:
    """pos 是否处于某行的 // 注释中(粗略:同向前找最近的 //)。"""
    line_start = src.rfind("\n", 0, pos) + 1
    prefix = src[line_start:pos]
    idx = prefix.find("//")
    return idx != -1


# 单个字符串字面量:容许 C++ 前缀(u8/U/L/u),内容仅含非引号非反斜杠字符或转义对。
# 严格 fullmatch 拒绝拼接表达式(如 "512," + itos(N) + ",1")。
STRING_LITERAL_RE = re.compile(r'^(?:u8|U|L|u)?"((?:[^"\\]|\\.)*)"$')
INT_LITERAL_RE = re.compile(r"^[-+]?[0-9]+$")
FLOAT_LITERAL_RE = re.compile(r"^[-+]?(?:[0-9]+\.[0-9]*|\.[0-9]+|[0-9]+)(?:[eE][-+]?[0-9]+)?$")
CTOR_CALL_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)\((.*)\)$", re.S)

# 构造器名 → Variant 类型。仅当实参为空或全为严格字面量时才据此推断,
# 避免把含表达式的构造器调用误判类型(如 Color(1,1,1) * 2 不会匹配本正则)。
CONSTRUCTOR_TYPE_MAP = {
    "Color": "color",
    "Point2": "vector2",  # Godot 4.4+ Point2 为 Vector2 别名
    "Vector2": "vector2",
    "Vector2i": "vector2i",
    "Vector3": "vector3",
    "Vector3i": "vector3i",
    "Vector4": "vector4",
    "Vector4i": "vector4i",
    "Rect2": "rect2",
    "Rect2i": "rect2i",
    "Transform2D": "transform2d",
    "Transform3D": "transform3d",
    "Plane": "plane",
    "Quaternion": "quaternion",
    "AABB": "aabb",
    "Basis": "basis",
    "Projection": "projection",
    "String": "string",
    "StringName": "string_name",
    "NodePath": "node_path",
    "Array": "array",
    "Dictionary": "dictionary",
    "PackedByteArray": "packed_byte_array",
    "PackedInt32Array": "packed_int32_array",
    "PackedInt64Array": "packed_int64_array",
    "PackedFloat32Array": "packed_float32_array",
    "PackedFloat64Array": "packed_float64_array",
    "PackedStringArray": "packed_string_array",
    "PackedVector2Array": "packed_vector2_array",
    "PackedVector3Array": "packed_vector3_array",
    "PackedVector4Array": "packed_vector4_array",
    "PackedColorArray": "packed_color_array",
}


def unescape_c_string(s: str) -> str:
    """反转常见 C 字符串转义(仅影响展示,hintString/键不会含冷门转义)。"""
    out = s.replace('\\"', '"').replace("\\n", "\n").replace("\\t", "\t")
    return out.replace("\\\\", "\\")


def parse_string_literal(text: str) -> str | None:
    """严格匹配单个字符串字面量;非纯字面量(拼接/字符字面量等)返回 None。"""
    text = text.strip()
    m = STRING_LITERAL_RE.match(text)
    if not m:
        return None
    return unescape_c_string(m.group(1))


def infer_type_from_default(text: str) -> str | None:
    """plain 形态(无 PropertyInfo)按默认值字面量推断 Variant 类型。

    仅识别无歧义的字面量/纯字面量实参构造器;枚举常量、函数调用、
    局部变量等表达式不推断(宁缺毋滥)。
    """
    text = text.strip()
    if STRING_LITERAL_RE.match(text):
        return "string"
    if text in ("true", "false"):
        return "bool"
    if INT_LITERAL_RE.match(text):
        return "int"
    if FLOAT_LITERAL_RE.match(text):
        return "float"
    m = CTOR_CALL_RE.match(text)
    if m:
        ctor = m.group(1)
        if ctor not in CONSTRUCTOR_TYPE_MAP:
            return None
        inner = m.group(2).strip()
        if not inner:
            return CONSTRUCTOR_TYPE_MAP[ctor]
        parts = [p.strip() for p in split_top_level(inner)]
        if all(
            INT_LITERAL_RE.match(p) or FLOAT_LITERAL_RE.match(p) or STRING_LITERAL_RE.match(p)
            for p in parts
        ):
            return CONSTRUCTOR_TYPE_MAP[ctor]
    return None


def normalize_value(text: str) -> str:
    """默认值表达式:压平空白。"""
    return re.sub(r"\s+", " ", text.strip())


def parse_property_info(text: str) -> dict | None:
    """解析 PropertyInfo(Variant::T, "key"[, PROPERTY_HINT_X[, "hint"]]) 实参文本。

    返回含 key/type/hint/hintString 的 dict(缺省字段省略);键非字面量返回 None。
    GLOBAL_DEF 与 ImportOption 两种形态共用(后者 PropertyInfo 可带第 5 参
    PROPERTY_USAGE 标志,天然忽略)。
    """
    inner, _ = read_call_args(text, text.index("("))
    parts = [p.strip() for p in split_top_level(inner)]
    if len(parts) < 2:
        return None
    entry: dict = {}
    m = re.fullmatch(r"Variant::([A-Z0-9_]+)", parts[0])
    if m:
        entry["type"] = VARIANT_TYPE_MAP.get(m.group(1), m.group(1).lower())
    key = parse_string_literal(parts[1])
    if key is None:
        return None
    entry["key"] = key
    if len(parts) >= 3:
        hm = re.fullmatch(r"PROPERTY_HINT_([A-Z0-9_]+)", parts[2])
        if hm:
            entry["hint"] = HINT_MAP.get(hm.group(1), hm.group(1).lower())
    if len(parts) >= 4:
        hs = parse_string_literal(parts[3])
        if hs is not None:
            entry["hintString"] = hs
    return entry


def extract_setting(macro: str, args_text: str, rel_file: str) -> dict | None:
    args = split_top_level(args_text)
    if len(args) < 2:
        return None
    first = args[0].strip()
    entry: dict = {"file": rel_file}
    entry.update(MACRO_FLAGS.get(macro, {}))

    if first.startswith("PropertyInfo("):
        info = parse_property_info(first)
        if info is None:
            return None
        entry.update(info)
    else:
        key = parse_string_literal(first)
        if key is None:
            return None
        entry["key"] = key
        inferred = infer_type_from_default(args[1])
        if inferred is not None:
            entry["type"] = inferred

    entry["default"] = normalize_value(args[1])
    return entry


def walk_sources(root: Path):
    for path in sorted(root.rglob("*.cpp")):
        yield path
    for path in sorted(root.rglob("*.h")):
        yield path


def extract_project_settings(godot_src: Path) -> dict[str, dict]:
    keys: dict[str, dict] = {}
    for path in walk_sources(godot_src):
        try:
            src = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        rel = path.relative_to(godot_src).as_posix()
        for m in MACRO_RE.finditer(src):
            macro = m.group(1)
            open_paren = m.end() - 1
            # 跳过 #define 行与注释中的提及。
            line_start = src.rfind("\n", 0, m.start()) + 1
            if src[line_start : m.end()].lstrip().startswith("#"):
                continue
            if in_line_comment(src, m.start()):
                continue
            args_text, _ = read_call_args(src, open_paren)
            entry = extract_setting(macro, args_text, rel)
            if entry is None or "key" not in entry:
                continue
            key = entry.pop("key")
            if not key:
                continue
            # 特性标签覆写键(如 application/run/max_fps.debug)保留,标记 tag 段。
            if key in keys:
                # 重复注册:保留信息更全的一条(带 type/hint 优先)。
                old = keys[key]
                if len(entry) > len(old):
                    keys[key] = entry
                continue
            keys[key] = entry
    return keys


# .import [params] 键来源:各 importer 的 get_import_options() 中
# ImportOption(PropertyInfo(...), default) 声明(核验自
# core/io/resource_importer.h 的 ImportOption 结构)。
IMPORT_OPTION_RE = re.compile(r"\bImportOption\s*\(")
IMPORTER_NAME_RE = re.compile(r"::get_importer_name\(\)\s*(?:const\s*)?\{")
RESOURCE_TYPE_RE = re.compile(r"::get_resource_type\(\)\s*(?:const\s*)?\{")
RETURN_STRING_RE = re.compile(r'\breturn\s+("(?:[^"\\]|\\.)*")\s*;')

# EditorSceneFormatImporter 后端(gltf/fbx/blend)无 get_importer_name,
# 其选项经 ResourceImporterScene 落盘到 importer="scene" 的 .import 文件。
SCENE_IMPORTER_FILES = {
    "editor_scene_importer_gltf.cpp": "scene",
    "editor_scene_importer_blend.cpp": "scene",
    "editor_scene_importer_ufbx.cpp": "scene",
    "editor_scene_importer_fbx2gltf.cpp": "scene",
}


def _returns_in_function(src: str, sig_re: re.Pattern) -> list[str]:
    """取 sig_re 匹配函数体内全部 return 的字符串字面量(unescape 后)。"""
    m = sig_re.search(src)
    if not m:
        return []
    body, _ = read_braces(src, m.end() - 1)
    out = []
    for rm in RETURN_STRING_RE.finditer(body):
        lit = parse_string_literal(rm.group(1))
        if lit:
            out.append(lit)
    return out


def extract_import_options(src: str, rel_file: str) -> list[tuple[str, dict]]:
    """文件内全部 ImportOption(PropertyInfo(...), default) 声明,返回 (键, 条目) 对。"""
    out: list[dict] = []
    for m in IMPORT_OPTION_RE.finditer(src):
        args_text, _ = read_call_args(src, m.end() - 1)
        parts = [p.strip() for p in split_top_level(args_text)]
        if len(parts) < 2 or not parts[0].startswith("PropertyInfo("):
            continue
        info = parse_property_info(parts[0])
        if info is None or "key" not in info:
            continue
        key = info.pop("key")
        if not key:
            continue
        info["default"] = normalize_value(parts[1])
        info["file"] = rel_file
        out.append((key, info))
    return out


def extract_import_params(godot_src: Path) -> tuple[dict[str, dict], dict[str, str]]:
    """按 importer 名聚合 [params] 键,并收集 importer → 资源类型映射。"""
    params: dict[str, dict] = {}
    types: dict[str, str] = {}
    for path in walk_sources(godot_src):
        try:
            src = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        if "ImportOption(" not in src:
            continue
        rel = path.relative_to(godot_src).as_posix()
        options = extract_import_options(src, rel)
        if not options:
            continue
        names = _returns_in_function(src, IMPORTER_NAME_RE)
        if not names:
            fallback = SCENE_IMPORTER_FILES.get(path.name)
            names = [fallback] if fallback else []
        if not names:
            continue
        # 单类多 mode(如 ResourceImporterLayeredTexture 的 cubemap/2d_array/
        # cubemap_array/3d)时,name 与 resource_type 按 switch 分支序一一对应,
        # 只有数量一致才按位配对,否则宁缺毋滥。
        resource_types = _returns_in_function(src, RESOURCE_TYPE_RE)
        for i, name in enumerate(names):
            bucket = params.setdefault(name, {})
            for key, info in options:
                if key not in bucket:
                    bucket[key] = info
            if len(resource_types) == len(names):
                types[name] = resource_types[i]
    return params, types


# 新建工程默认值:EditorNode::get_initial_settings() 的 settings["k"] = 字面量 赋值
# (源码注释:"better defaults for new projects without affecting existing projects")。
INITIAL_SETTINGS_RE = re.compile(r"::get_initial_settings\(\)\s*(?:const\s*)?\{")
SETTINGS_ASSIGN_RE = re.compile(r'settings\[\s*("(?:[^"\\]|\\.)*")\s*\]\s*=\s*([^;]+);')


def extract_initial_settings(godot_src: Path) -> dict[str, str]:
    """返回 {键: 字面量默认值}(仅 get_initial_settings 内的字面量赋值)。"""
    out: dict[str, str] = {}
    for path in walk_sources(godot_src):
        try:
            src = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in INITIAL_SETTINGS_RE.finditer(src):
            body, _ = read_braces(src, m.end() - 1)
            for am in SETTINGS_ASSIGN_RE.finditer(body):
                key = parse_string_literal(am.group(1))
                if not key:
                    continue
                value = normalize_value(am.group(2))
                if key not in out:
                    out[key] = value
    return out


def merge_initial_settings(keys: dict[str, dict], initial: dict[str, str]) -> None:
    """把新建工程默认值并入设置键:已有条目补 initial 标记,新键直接入库。

    get_initial_settings 全仓仅定义于 editor/editor_node.cpp(源码注释明言
    "better defaults for new projects"),file 字段按此固定标注。
    """
    rel = "editor/editor_node.cpp"
    for key, value in initial.items():
        inferred = infer_type_from_default(value)
        if key in keys:
            entry = keys[key]
            entry["initial"] = True
            if entry.get("default") != value:
                entry["initialDefault"] = value
        else:
            entry = {"initial": True, "default": value, "file": rel}
            if inferred is not None:
                entry["type"] = inferred
            keys[key] = entry


# .gdextension 键位图(核验自 core/extension/gdextension_library_loader.cpp:
# parse_gdextension_file / find_extension_library / find_extension_dependencies)。
GDEXTENSION_MAP = {
    "configuration": {
        "keys": {
            "entry_symbol": {"required": True, "doc": "库入口函数名(GDExtensionBool GDE_EXPORT ...)"},
            "compatibility_minimum": {"required": True, "doc": "最低兼容 Godot 版本(须 ≥ 4.1.0)"},
            "compatibility_maximum": {"required": False, "doc": "最高兼容 Godot 版本(可选)"},
            "reloadable": {"required": False, "doc": "是否支持热重载(编辑器)"},
            "autodetect_library_prefix": {"required": False, "doc": "库文件自动探测前缀"},
        },
        "doc": "必需节:入口符号与兼容版本",
    },
    "libraries": {
        "keys": {},
        "doc": "按特性标签选库:键为特性标签列表(windows.x86_64 / linux.x86_64 / macos / android.arm64 / web.wasm32),值为库路径",
    },
    "dependencies": {
        "keys": {},
        "doc": "共享库依赖:键为特性标签,值为 {源路径: 目标路径} 字典",
    },
    "icons": {
        "keys": {},
        "doc": "类图标:键为类名,值为图标路径",
    },
}

# .import 键位图(核验自 editor/file_system/editor_file_system.cpp:
# _reimport_file 的手工写入——"Write manually, as order matters ([remap] has to
# go first for performance)",以及 _get_path_and_type / import_dock.cpp 的读路径)。
# 写入顺序即下方 remap 键的排列顺序(path 可为 path.<feature> 特性变体)。
IMPORT_MAP = {
    "remap": {
        "keys": {
            "importer": {"required": True, "doc": "导入器名(如 texture);keep/skip 表示不导入"},
            "importer_version": {"required": False, "doc": "导入器格式版本(变更时触发重新导入)"},
            "type": {"required": False, "doc": "导入产物资源类型(如 CompressedTexture2D)"},
            "uid": {"required": False, "doc": "资源 UID(uid://...)"},
            "group_file": {"required": False, "doc": "图集分组文件(Texture2DArray 合并导入用)"},
            "path": {"required": False, "doc": "导入产物路径(res://.godot/imported/...);变体导入写作 path.<feature>"},
            "valid": {"required": False, "doc": "上次导入是否成功(失败时为 false,需手动重新导入)"},
            "metadata": {"required": False, "doc": "导入元数据字典(如 imported_by 标记来源)"},
            "generator_parameters": {"required": False, "doc": "生成器参数字典(编辑器插件自定义导入)"},
        },
        "doc": "重映射信息(必须为首节)",
    },
    "deps": {
        "keys": {
            "files": {"required": False, "doc": "导入器生成的附带文件数组"},
            "source_file": {"required": True, "doc": "源文件路径"},
            "dest_files": {"required": False, "doc": "导入产物路径数组"},
        },
        "doc": "依赖关系",
    },
    "params": {
        "keys": {},
        "doc": "导入器参数(各 importer 的 get_import_options 声明,见 import_params)",
    },
}


def detect_version(godot_src: Path) -> str:
    version_py = godot_src / "version.py"
    if version_py.exists():
        text = version_py.read_text(encoding="utf-8", errors="replace")
        major = re.search(r'^\s*major\s*=\s*"?(\d+)"?', text, re.M)
        minor = re.search(r'^\s*minor\s*=\s*"?(\d+)"?', text, re.M)
        patch = re.search(r'^\s*patch\s*=\s*"?(\d+)"?', text, re.M)
        if major and minor and patch:
            return f"{major.group(1)}.{minor.group(1)}.{patch.group(1)}"
    return "unknown"


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "godot_src",
        nargs="?",
        default=str(repo_root / ".refs" / "godot"),
        help="Godot 源码目录(默认 .refs/godot)",
    )
    parser.add_argument(
        "-o",
        "--output",
        default=str(repo_root / "editors" / "vscode" / "data" / "project-settings.json"),
        help="输出 JSON 路径",
    )
    args = parser.parse_args()

    godot_src = Path(args.godot_src)
    if not godot_src.is_dir():
        print(f"error: Godot 源码目录不存在: {godot_src}", file=sys.stderr)
        return 1

    keys = extract_project_settings(godot_src)
    version = detect_version(godot_src)
    merge_initial_settings(keys, extract_initial_settings(godot_src))
    import_params, import_types = extract_import_params(godot_src)
    out = {
        "version": version,
        "count": len(keys),
        "keys": dict(sorted(keys.items())),
        "import_params": dict(sorted(import_params.items())),
        "import_types": dict(sorted(import_types.items())),
        "gdextension": GDEXTENSION_MAP,
        "import": IMPORT_MAP,
    }

    out_path = Path(args.output)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(
        json.dumps(out, ensure_ascii=False, indent=1) + "\n", encoding="utf-8"
    )
    print(f"extracted {len(keys)} settings from Godot {version} -> {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
