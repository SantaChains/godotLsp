import assert from "node:assert/strict";
import { test } from "node:test";
import {
  buildSettingHover,
  contextAt,
  enumValues,
  lookupSetting,
  parseConfigFile,
  projectSections,
  rangeParts,
  settingKeysInSection,
  settings,
} from "../src/configFile";

// ---------------------------------------------------------------------------
// 解析器(语义核验自 core/io/variant_parser.cpp 的 parse_tag_assign_eof)
// ---------------------------------------------------------------------------

test("parseConfigFile:节/键/值三要素与区间", () => {
  const text = [
    "; 头部注释",
    "config_version=5",
    "",
    "[application]",
    "",
    "config/name=\"My Game\"",
    "run/max_fps=0",
  ].join("\n");
  const cf = parseConfigFile(text);
  assert.deepEqual(
    cf.sections.map((s) => s.name),
    ["application"],
  );
  assert.equal(cf.sections[0].line, 3);
  assert.equal(cf.sections[0].nameStart, 1);
  assert.equal(cf.sections[0].nameEnd, 12);
  assert.equal(cf.entries.length, 3);
  // 首个节之前的键 section 为 ""。
  assert.equal(cf.entries[0].section, "");
  assert.equal(cf.entries[0].key, "config_version");
  assert.equal(cf.entries[0].value, "5");
  const name = cf.entries[1];
  assert.equal(name.section, "application");
  assert.equal(name.key, "config/name");
  assert.equal(name.value, "\"My Game\"");
  // 值区间从引号开始到行尾。
  assert.deepEqual(name.valueStart, { line: 5, character: 12 });
  assert.deepEqual(name.valueEnd, { line: 5, character: 21 });
});

test("parseConfigFile:节名支持 \\] 转义,节头到第一个未转义 ] 为止", () => {
  const cf = parseConfigFile("[we\\]ird]\nk=1\n");
  assert.equal(cf.sections[0].name, "we]ird");
  assert.equal(cf.entries[0].section, "we]ird");
});

test("parseConfigFile:引号键合法,键后容许空白再接 =", () => {
  const cf = parseConfigFile("[s]\n\"quoted key\" = 1\nbare=2\n");
  assert.deepEqual(
    cf.entries.map((e) => e.key),
    ["quoted key", "bare"],
  );
  assert.equal(cf.entries[0].value, "1");
});

test("parseConfigFile:; 注释(行首与值后),字符串内的 ; 不是注释", () => {
  const cf = parseConfigFile([
    "[s]",
    "; 整行注释",
    "a=1 ; 行尾注释",
    "b=\"含有;分号\"",
    "c=2;无空格注释",
  ].join("\n"));
  assert.equal(cf.entries.length, 3);
  assert.equal(cf.entries[0].value, "1");
  assert.equal(cf.entries[1].value, "\"含有;分号\"");
  assert.equal(cf.entries[2].value, "2");
});

test("parseConfigFile:括号与字符串未闭合时值跨物理行(换行原样保留)", () => {
  const cf = parseConfigFile([
    "[s]",
    "arr=PackedStringArray(\"a\",",
    "  \"b\")",
    "dict={",
    "  \"k\": 1,",
    "}",
    "text=\"第一行",
    "第二行\"",
    "next=1",
  ].join("\n"));
  assert.equal(cf.entries.length, 4);
  assert.equal(cf.entries[0].key, "arr");
  assert.equal(cf.entries[0].value, "PackedStringArray(\"a\",\n  \"b\")");
  assert.equal(cf.entries[1].key, "dict");
  assert.equal(cf.entries[1].value, "{\n  \"k\": 1,\n}");
  // 多行字符串:引号内的换行是值的一部分。
  assert.equal(cf.entries[2].key, "text");
  assert.equal(cf.entries[2].value, "\"第一行\n第二行\"");
  assert.equal(cf.entries[2].valueEnd.line, 7);
  // 跨行值之后,新键仍能正确切分。
  assert.equal(cf.entries[3].key, "next");
  assert.equal(cf.entries[3].value, "1");
});

test("parseConfigFile:半输入状态稳定(未闭合节头/无 = 的行/EOF 截断)", () => {
  const cf = parseConfigFile("[appl\nconfig/na\nconfig/name=\"De");
  // 未闭合节头吞到行尾;无 = 的行跳过;未闭合字符串吞到 EOF。
  assert.deepEqual(
    cf.sections.map((s) => s.name),
    ["appl"],
  );
  assert.equal(cf.sections[0].closed, false);
  assert.equal(cf.entries.length, 1);
  assert.equal(cf.entries[0].key, "config/name");
  assert.equal(cf.entries[0].value, "\"De");
});

test("parseConfigFile:闭合节头 closed=true,转义与边界", () => {
  const cf = parseConfigFile("[a]b\n[s]\nk=1\n");
  assert.equal(cf.sections[0].closed, true);
  assert.equal(cf.sections[0].nameEnd, 2);
  assert.equal(cf.sections[1].closed, true);
  assert.equal(cf.entries.length, 1);
});

test("parseConfigFile:真实 project.godot 样本", () => {
  const text = [
    "; Engine configuration file.",
    "config_version=5",
    "",
    "[application]",
    "",
    "config/name=\"My Game\"",
    "run/main_scene=\"res://main.tscn\"",
    "config/features=PackedStringArray(\"4.7\", \"Forward Plus\")",
    "",
    "[autoload]",
    "",
    "Global=\"*res://scripts/global.gd\"",
    "",
    "[display]",
    "",
    "window/size/viewport_width=1152",
    "window/size/viewport_height=648",
  ].join("\n");
  const cf = parseConfigFile(text);
  assert.deepEqual(
    cf.sections.map((s) => s.name),
    ["application", "autoload", "display"],
  );
  const byKey = new Map(cf.entries.map((e) => [`${e.section}/${e.key}`, e]));
  assert.equal(byKey.get("application/config/features")?.value, "PackedStringArray(\"4.7\", \"Forward Plus\")");
  assert.equal(byKey.get("autoload/Global")?.value, "\"*res://scripts/global.gd\"");
  assert.equal(byKey.get("display/window/size/viewport_width")?.value, "1152");
});

// ---------------------------------------------------------------------------
// 位置 → 上下文
// ---------------------------------------------------------------------------

const CTX_TEXT = [
  "config_version=5",
  "",
  "[application]",
  "",
  "config/name=\"My Game\"",
  "run/max_fps=0",
  "",
  "[display]",
  "window/size/viewport_width=1152",
].join("\n");

function ctx(line: number, character: number) {
  const cf = parseConfigFile(CTX_TEXT);
  const lineText = CTX_TEXT.split("\n")[line];
  return contextAt(cf, { line, character }, lineText);
}

test("contextAt:已闭合节头内 → section-name", () => {
  const c = ctx(2, 5);
  assert.equal(c.kind, "section-name");
  if (c.kind === "section-name") {
    assert.equal(c.section, "application");
  }
});

test("contextAt:半输入节头 → section-name", () => {
  // 行内是未闭合的 "[dis"(CTX_TEXT 第 7 行是闭合的 [display],这里单独构造)。
  const half = "[application]\nconfig/name=\"X\"\n[dis";
  const cf = parseConfigFile(half);
  const c = contextAt(cf, { line: 2, character: 3 }, "[dis");
  assert.equal(c.kind, "section-name");
  if (c.kind === "section-name") {
    // 尚未闭合,沿用上一节。
    assert.equal(c.section, "application");
  }
});

test("contextAt:键 token(含无 = 的半输入键)→ key", () => {
  assert.equal(ctx(4, 3).kind, "key"); // config/name 行首
  assert.equal(ctx(4, 10).kind, "key"); // 键内
  const c = ctx(4, 3);
  if (c.kind === "key") {
    assert.equal(c.section, "application");
    assert.equal(c.entry?.key, "config/name");
  }
  // 引号内是值上下文(键与值的边界在 = 之后)。
  assert.equal(ctx(4, 12).kind, "value");
});

test("contextAt:值区间(含空值)→ value", () => {
  assert.equal(ctx(4, 14).kind, "value"); // 引号内
  const c = ctx(5, 12); // run/max_fps=0 末尾
  assert.equal(c.kind, "value");
  if (c.kind === "value") {
    assert.equal(c.entry.key, "run/max_fps");
    assert.equal(c.entry.value, "0");
  }
});

test("contextAt:多行值中间行按值区间命中(回归:键行查找漏中间行)", () => {
  const text = [
    "[input]",
    "move_left={",
    "\"deadzone\": 0.2,",
    "\"events\": [Object(InputEventKey,\"physical_keycode\":65)]",
    "}",
    "next=1",
  ].join("\n");
  const cf = parseConfigFile(text);
  const ml = cf.entries.find((e) => e.key === "move_left");
  assert.ok(ml);
  // 值区间跨 4 行;结束行的 character 属于结束行(不被起点钳位)。
  assert.deepEqual(ml.valueStart, { line: 1, character: 10 });
  assert.deepEqual(ml.valueEnd, { line: 4, character: 1 });
  // 中间任意行都是 value 上下文(键行本身要取 = 之后的位置)。
  const probe: Array<[number, number]> = [
    [1, 11],
    [2, 2],
    [3, 2],
    [4, 0],
  ];
  for (const [line, ch] of probe) {
    const c = contextAt(cf, { line, character: ch }, text.split("\n")[line]);
    assert.equal(c.kind, "value", `line ${line} 应为 value`);
    if (c.kind === "value") {
      assert.equal(c.entry.key, "move_left");
    }
  }
  // 多行值之后的新键不受影响。
  const next = cf.entries.find((e) => e.key === "next");
  assert.ok(next);
  assert.equal(
    contextAt(cf, { line: next.line, character: 0 }, text.split("\n")[next.line]).kind,
    "key",
  );
});

test("contextAt:空行/注释 → other 且带上当前节", () => {
  const c = ctx(6, 0);
  assert.equal(c.kind, "other");
  if (c.kind === "other") {
    assert.equal(c.section, "application");
  }
});

// ---------------------------------------------------------------------------
// 数据集查询
// ---------------------------------------------------------------------------

test("lookupSetting:已知键带 type/default/hint,未知键返回 undefined", () => {
  const name = lookupSetting("application", "config/name");
  assert.equal(name?.type, "string");
  assert.equal(name?.default, "\"\"");
  assert.equal(name?.basic, true);
  const fps = lookupSetting("application", "run/max_fps");
  assert.equal(fps?.type, "int");
  assert.equal(fps?.hint, "range");
  assert.equal(fps?.hintString, "0,1000,1");
  assert.equal(lookupSetting("application", "不存在的键"), undefined);
  // 特性标签覆写键(如 .mobile)是真实注册;默认值为 C++ 变量,类型不可推断。
  const mobile = lookupSetting("rendering", "renderer/rendering_method.mobile");
  assert.equal(mobile?.default, "default_renderer_mobile");
  assert.equal(mobile?.restart, true);
  // 结构性键(非 GLOBAL_DEF)。
  assert.equal(lookupSetting("", "config_version")?.type, "int");
});

test("lookupSetting:physics/3d/physics_engine 来自 get_initial_settings 兜底", () => {
  const e = lookupSetting("physics", "3d/physics_engine");
  assert.equal(e?.type, "string");
  assert.equal(e?.default, "\"Jolt Physics\"");
  assert.equal(e?.initial, true);
});

test("projectSections:数据驱动 + autoload 特殊节", () => {
  const sections = projectSections();
  assert.ok(sections.includes("application"));
  assert.ok(sections.includes("display"));
  assert.ok(sections.includes("autoload"));
  // 排序稳定。
  assert.deepEqual(sections, [...sections].sort());
});

test("settingKeysInSection:节内键去掉节前缀", () => {
  const keys = settingKeysInSection("application").map((k) => k.key);
  assert.ok(keys.includes("config/name"));
  assert.ok(keys.includes("run/max_fps"));
  assert.ok(!keys.some((k) => k.startsWith("application/")));
  assert.ok(keys.includes("run/max_fps"));
});

test("enumValues/rangeParts:提示串解析", () => {
  assert.deepEqual(enumValues("forward_plus,mobile,gl_compatibility"), [
    "forward_plus",
    "mobile",
    "gl_compatibility",
  ]);
  assert.deepEqual(enumValues(undefined), []);
  assert.deepEqual(rangeParts("1,7680,1,or_greater"), {
    min: "1",
    max: "7680",
    step: "1",
    orGreater: true,
  });
  assert.equal(rangeParts("0,1000,1")?.orGreater, false);
});

test("buildSettingHover:键/类型/范围/默认值/来源齐备", () => {
  const md = buildSettingHover("display/window/size/viewport_width", {
    type: "int",
    hint: "range",
    hintString: "1,7680,1,or_greater",
    default: "1152",
    file: "core/config/project_settings.cpp",
  });
  assert.match(md, /display\/window\/size\/viewport_width/);
  assert.match(md, /`int`/);
  assert.match(md, /数值范围/);
  assert.match(md, /7680\(或更大\)/);
  assert.match(md, /默认值 `1152`/);
  assert.match(md, /core\/config\/project_settings\.cpp/);
  // 新建工程默认值与注册默认值不同时并列展示。
  const md2 = buildSettingHover("display/window/stretch/aspect", {
    type: "string",
    hint: "enum",
    hintString: "ignore,keep,keep_width,keep_height,expand",
    default: "\"keep\"",
    initialDefault: "\"expand\"",
    initial: true,
  });
  assert.match(md2, /默认值 `"keep"`/);
  assert.match(md2, /新建工程 `"expand"`/);
  assert.match(md2, /`ignore` \/ `keep`/);
});

// ---------------------------------------------------------------------------
// .import 数据集
// ---------------------------------------------------------------------------

test("import_params:importer → params/type 映射(含单类多 mode 按位配对)", () => {
  const texture = settings.import_params.texture;
  assert.equal(texture["compress/mode"].type, "int");
  assert.equal(texture["compress/mode"].hint, "enum");
  assert.deepEqual(enumValues(texture["compress/mode"].hintString), [
    "Lossless",
    "Lossy",
    "VRAM Compressed",
    "VRAM Uncompressed",
    "Basis Universal",
  ]);
  assert.equal(texture["mipmaps/generate"].type, "bool");
  assert.equal(texture["roughness/src_normal"].hint, "file");
  // 单类多 mode(ResourceImporterLayeredTexture)按 switch 序配对。
  assert.equal(settings.import_types["2d_array_texture"], "CompressedTexture2DArray");
  assert.equal(settings.import_types["3d_texture"], "CompressedTexture3D");
  assert.equal(settings.import_types["cubemap_array_texture"], "CompressedCubemapArray");
  assert.equal(settings.import_types["cubemap_texture"], "CompressedCubemap");
  assert.equal(settings.import_types.texture, "CompressedTexture2D");
  // gltf/fbx 后端选项归入 scene importer。
  assert.ok("gltf/naming_version" in settings.import_params.scene);
  assert.ok("nodes/import_as_skeleton_bones" in settings.import_params.scene);
  // importer 清单规模(防数据回潮)。
  assert.ok(Object.keys(settings.import_params).length >= 15);
});

test("import/gdextension 节键文档齐全", () => {
  assert.equal(settings.gdextension.configuration.keys.entry_symbol.required, true);
  assert.equal(settings.gdextension.configuration.keys.compatibility_minimum.required, true);
  assert.equal(settings.import.remap.keys.importer.required, true);
  assert.equal(settings.import.remap.keys.uid.required, false);
});
