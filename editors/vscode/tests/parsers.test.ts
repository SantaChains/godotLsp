import assert from "node:assert/strict";
import { test } from "node:test";
import { parseAutoloads, parseScene } from "../src/parsers";

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
