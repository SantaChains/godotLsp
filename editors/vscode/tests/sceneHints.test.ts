/**
 * A5/A6 纯逻辑单测:资源引用定位、inlay 生成、res 路径换算、脚本↔场景互跳匹配。
 */

import { test } from "node:test";
import assert from "node:assert/strict";

import { resRefAt, sceneInlays } from "../src/sceneLogic";
import { toResPath, sceneBindsScript, sceneRootScript } from "../src/sceneLogic";

const SCENE = [
  "[gd_scene load_steps=3 format=3]",
  "",
  '[ext_resource type="Script" path="res://player.gd" id="1_s"]',
  '[ext_resource type="Texture2D" path="res://icon.png" id="2_t"]',
  '[ext_resource type="Texture2D" path="res://icon.png" id="3_t"]',
  "",
  '[sub_resource type="CircleShape2D" id="CircleShape2D_1"]',
  "",
  '[node name="Main" type="Node2D"]',
  'script = ExtResource("1_s")',
  "",
  '[node name="Sprite" type="Sprite2D" parent="." instance=ExtResource("2_t")]',
].join("\n");

test("resRefAt:光标命中 ExtResource/SubResource,区间外 undefined", () => {
  const line = 'script = ExtResource("1_s")';
  const start = line.indexOf("ExtResource");
  assert.deepEqual(resRefAt(line, start + 3), { kind: "ext", id: "1_s" });
  // 引用区间外(行首属性名上)不命中
  assert.equal(resRefAt(line, 0), undefined);
  const sub = 'shape = SubResource("CircleShape2D_1")';
  const s = sub.indexOf("SubResource");
  assert.deepEqual(resRefAt(sub, s + 2), { kind: "sub", id: "CircleShape2D_1" });
});

test("sceneInlays:属性行与 instance 行尾 hint,同 path 多 id 都给(使用侧视角)", () => {
  const inlays = sceneInlays(SCENE);
  assert.deepEqual(inlays, [
    { line: 9, label: "← res://player.gd" },
    { line: 11, label: "← res://icon.png" },
  ]);
});

test("toResPath:workspace 相对换算与越界 undefined", () => {
  assert.equal(toResPath("D:/proj", "D:/proj/src/player.gd"), "res://src/player.gd");
  // win 反斜杠
  assert.equal(toResPath("D:\\proj", "D:\\proj\\main.tscn"), "res://main.tscn");
  assert.equal(toResPath("D:/proj", "D:/other/x.gd"), undefined);
});

test("sceneBindsScript:script 绑定与实例化路径都算命中", () => {
  assert.equal(sceneBindsScript(SCENE, "res://player.gd"), true);
  assert.equal(sceneBindsScript(SCENE, "res://icon.png"), true); // instance 绑的是场景,但 path 匹配 ext 即命中——icon.png 是纹理,真实场景里不会 instance 纹理;此处验证 instance 分支
  assert.equal(sceneBindsScript(SCENE, "res://enemy.gd"), false);
});

test("sceneRootScript:根节点 script 绑定解析,无绑定 undefined", () => {
  assert.equal(sceneRootScript(SCENE), "res://player.gd");
  assert.equal(sceneRootScript('[node name="A" type="Node"]\n'), undefined);
});
