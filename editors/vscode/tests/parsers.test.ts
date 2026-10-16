import assert from "node:assert/strict";
import { test } from "node:test";
import { parseAutoloads, parseConnections, parseScene } from "../src/parsers";

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
