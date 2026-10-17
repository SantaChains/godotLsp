import assert from "node:assert/strict";
import { test } from "node:test";
import { parseAutoloads, parseConnections, parseGdConnects, parseScene } from "../src/parsers";

test("parseAutoloads:提取 [autoload] 段并剥离单例前导 *", () => {
  const text = [
    "; Engine configuration file.",
    "[application]",
    "config/name=\"Demo\"",
    "",
    "[autoload]",
    "Global=\"*res://scripts/global.gd\"",
    "Utils=\"res://lib/utils.gd\"",
    "",
    "[display]",
    "window/size/viewport_width=1152",
  ].join("\n");
  assert.deepEqual(parseAutoloads(text), [
    { name: "Global", resPath: "res://scripts/global.gd" },
    { name: "Utils", resPath: "res://lib/utils.gd" },
  ]);
});

test("parseAutoloads:无 autoload 段/注释行/非 res:// 值全部跳过", () => {
  assert.deepEqual(parseAutoloads("[application]\nconfig/name=\"X\"\n"), []);
  assert.deepEqual(parseAutoloads("[autoload]\n; 注释行\nBad=\"not_res\"\n"), []);
});

test("parseScene:ext_resource 注册表与节点层级", () => {
  const text = [
    "[gd_scene load_steps=3 format=3 uid=\"uid://abc\"]",
    "",
    "[ext_resource type=\"Script\" path=\"res://player.gd\" id=\"1_p\"]",
    "[ext_resource type=\"PackedScene\" path=\"res://hud.tscn\" id=\"2_h\"]",
    "",
    "[node name=\"Main\" type=\"Node\"]",
    "",
    "[node name=\"Player\" type=\"CharacterBody2D\" parent=\".\"]",
    "script = ExtResource(\"1_p\")",
    "",
    "[node name=\"Sprite\" type=\"Sprite2D\" parent=\"Player\"]",
    "",
    "[node name=\"HUD\" parent=\".\" instance=ExtResource(\"2_h\")]",
  ].join("\n");
  const scene = parseScene(text);
  assert.equal(scene.ext.get("1_p")?.path, "res://player.gd");
  assert.equal(scene.ext.get("2_h")?.type, "PackedScene");
  assert.equal(scene.roots.length, 1);
  const root = scene.roots[0];
  assert.equal(root.name, "Main");
  assert.equal(root.type, "Node");
  // 直接子节点(parent=".")与孙节点(parent="子名")逐级挂载
  assert.deepEqual(root.children.map((c) => c.name), ["Player", "HUD"]);
  const player = root.children[0];
  assert.equal(player.type, "CharacterBody2D");
  assert.deepEqual(player.children.map((c) => c.name), ["Sprite"]);
  assert.equal(root.children[1].instanceId, "2_h");
});

test("parseScene:孤儿节点(parent 路径缺失)容错挂根级", () => {
  const text = [
    "[node name=\"Root\" type=\"Node\"]",
    "",
    "[node name=\"Ghost\" parent=\"Missing/Path\"]",
  ].join("\n");
  const scene = parseScene(text);
  assert.equal(scene.roots.length, 2);
  assert.equal(scene.roots[1].name, "Ghost");
  assert.equal(scene.roots[1].children.length, 0);
});

test("parseScene:行号精确定位(用于树跳转)", () => {
  const text = "[node name=\"A\"]\n\n[node name=\"B\" parent=\".\"]";
  const scene = parseScene(text);
  assert.equal(scene.roots[0].line, 0);
  assert.equal(scene.roots[0].children[0].line, 2);
});

// ---------------------------------------------------------------------------
// parseConnections + nodePaths(信号连接视图,P1-10)
// ---------------------------------------------------------------------------

const CONN_SCENE = [
  "[gd_scene load_steps=2 format=3]",
  "",
  "[ext_resource type=\"Script\" path=\"res://enemy.gd\" id=\"1_e\"]",
  "",
  "[node name=\"Main\" type=\"Node\"]",
  "",
  "[node name=\"Player\" type=\"CharacterBody2D\" parent=\".\"]",
  "",
  "[node name=\"Enemy\" type=\"Node2D\" parent=\".\"]",
  "script = ExtResource(\"1_e\")",
  "",
  "[connection signal=\"pressed\" from=\"Player\" to=\"Enemy\" method=\"_on_player_pressed\"]",
  "[connection signal=\"died\" from=\"Enemy\" to=\".\" method=\"_on_enemy_died\" binds=1]",
].join("\n");

test("parseConnections:四字段提取、行号定位、可选字段保留", () => {
  const conns = parseConnections(CONN_SCENE);
  assert.equal(conns.length, 2);
  assert.deepEqual(conns[0], {
    from: "Player",
    to: "Enemy",
    signal: "pressed",
    method: "_on_player_pressed",
    line: 11,
    missing: [],
  });
  assert.equal(conns[1].signal, "died");
  assert.equal(conns[1].to, ".");
  assert.equal(conns[1].binds, "1");
  assert.deepEqual(conns[1].missing, []);
});

test("parseConnections:缺必填字段打 missing 标记(引擎将拒载)", () => {
  const conns = parseConnections('[connection signal="died" from="Enemy"]');
  assert.equal(conns.length, 1);
  assert.equal(conns[0].line, 0);
  assert.deepEqual(conns[0].missing, ["to", "method"]);
  // 完全空 tag 同样容错
  const empty = parseConnections("[connection]");
  assert.deepEqual(empty[0].missing, ["from", "to", "signal", "method"]);
});

test("parseConnections:无连接返回空;非 connection 标签不误匹配", () => {
  assert.deepEqual(parseConnections("[node name=\"A\"]\n[ext_resource id=\"1\"]"), []);
});

test("nodePaths:connection from/to 可定位节点声明行", () => {
  const scene = parseScene(CONN_SCENE);
  assert.equal(scene.nodePaths.get(".")?.name, "Main");
  assert.equal(scene.nodePaths.get("Player")?.line, 6);
  assert.equal(scene.nodePaths.get("Enemy")?.line, 8);
  assert.equal(scene.nodePaths.get("Nowhere"), undefined);
});

test("parseScene:属性行 ExtResource 绑定与归属切断", () => {
  const text = [
    "[gd_scene load_steps=5 format=3]",
    "",
    '[ext_resource type="Script" path="res://player.gd" id="1_s"]',
    '[ext_resource type="Texture2D" path="res://icon.png" id="2_t"]',
    '[ext_resource type="AudioStream" path="res://hit.ogg" id="4_a"]',
    "",
    '[sub_resource type="CircleShape2D" id="CircleShape2D_1"]',
    "radius = 12.0",
    "",
    '[node name="Main" type="Node2D"]',
    'script = ExtResource("1_s")',
    "",
    '[node name="Sprite" type="Sprite2D" parent="."]',
    'texture = ExtResource("2_t")',
    "texture_filter = 1",
    "",
    '[node name="Snd" type="AudioStreamPlayer2D" parent="."]',
    'stream = ExtResource("4_a")',
    "",
    '[connection signal="ready" from="." to="." method="_on_ready"]',
    "flags = 3",
  ].join("\n");
  const scene = parseScene(text);
  const main = scene.roots[0];
  // 脚本绑定(resource_format_text.cpp:1985 is_script=name==script 同源)
  assert.equal(main.bindings.get("script"), "1_s");
  const sprite = main.children[0];
  assert.equal(sprite.bindings.get("texture"), "2_t");
  // 非资源属性行(数值 texture_filter)不入表
  assert.equal(sprite.bindings.size, 1);
  // sub_resource 段的 radius 在任何 node 之前,不归属;connection 段 flags 不误吞
  const snd = main.children[1];
  assert.equal(snd.bindings.get("stream"), "4_a");
  assert.equal(snd.bindings.size, 1);
  // 带斜杠属性名(theme_override_styles/panel 形态)
  const text2 = [
    '[node name="B" type="Button" parent="."]',
    'theme_override_styles/normal = ExtResource("9_st")',
  ].join("\n");
  assert.equal(parseScene(text2).roots[0].bindings.get("theme_override_styles/normal"), "9_st");
});

test("parseGdConnects:点链 connect 提取与注释行跳过", () => {
  const text = [
    "func _ready():",
    "\t$Button.pressed.connect(_on_pressed)",
    "\t# self.deleted.connect(_x) 注释不报",
    "\tself.health_changed.connect(_on_hp)",
    "\tsignal_bus.game_over.connect(func(): pass)",
    "\titems[0].ready.connect(_on_ready)",
  ].join("\n");
  const out = parseGdConnects(text);
  assert.deepEqual(
    out.map((c) => [c.line, c.signal, c.source]),
    [
      [1, "pressed", "$Button.pressed"],
      [3, "health_changed", "self.health_changed"],
      [4, "game_over", "signal_bus.game_over"],
      // items[0] 索引链:点链段无法跨越 `]`,退化只报信号段(静态可见化降级)。
      [5, "ready", "ready"],
    ],
  );
  assert.equal(parseGdConnects("").length, 0);
});
