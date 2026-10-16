import assert from "node:assert/strict";
import { test } from "node:test";
import { diagSuffix, filterTree, matchFilter } from "../src/treefilter";

// ── matchFilter ──

test("matchFilter:大小写不敏感子串,任一字段命中", () => {
  assert.equal(matchFilter("HP", "hp_bar"), true);
  assert.equal(matchFilter("bar", undefined, "HP Bar"), true);
  assert.equal(matchFilter("zzz", "hp", "bar"), false);
});

test("matchFilter:空过滤词恒真(未过滤状态)", () => {
  assert.equal(matchFilter("", "anything"), true);
  assert.equal(matchFilter("   ", undefined), true);
});

// ── filterTree ──

interface N {
  name: string;
  desc?: string;
  kids: N[];
}

const SPEC = {
  labelOf: (n: N) => n.name,
  descOf: (n: N) => n.desc,
  kidsOf: (n: N) => n.kids,
};

const TREE: N[] = [
  {
    name: "Player",
    desc: "CharacterBody2D",
    kids: [
      { name: "hp_bar", desc: "ProgressBar", kids: [] },
      { name: "sprite", desc: "Sprite2D", kids: [] },
    ],
  },
  { name: "Enemy", desc: undefined, kids: [{ name: "hp", desc: "int", kids: [] }] },
];

test("filterTree:命中后代的祖先链完整保留", async () => {
  const out = await filterTree(TREE, "hp_bar", SPEC);
  assert.equal(out.length, 1);
  assert.equal(out[0].name, "Player");
  // 显示时 getChildren 每层再过滤:子层只留命中子集(父层节点保留全子集引用)。
  const kids = await filterTree(out[0].kids, "hp_bar", SPEC);
  assert.deepEqual(
    kids.map((k) => k.name),
    ["hp_bar"],
  );
});

test("filterTree:自身命中也保留(叶子)", async () => {
  const out = await filterTree(TREE, "enemy", SPEC);
  assert.equal(out.length, 1);
  assert.equal(out[0].name, "Enemy");
  // 子节点不含过滤词但被拖进来?不——Enemy 自身命中,子层由调用方再过滤。
});

test("filterTree:描述字段参与匹配", async () => {
  const out = await filterTree(TREE, "sprite2d", SPEC);
  assert.equal(out.length, 1);
  assert.equal(out[0].name, "Player");
  const kids = await filterTree(out[0].kids, "sprite2d", SPEC);
  assert.deepEqual(
    kids.map((k) => k.name),
    ["sprite"],
  );
});

test("filterTree:空过滤词恒等返回", async () => {
  const out = await filterTree(TREE, "  ", SPEC);
  assert.equal(out.length, 2);
  assert.equal(out[0], TREE[0]); // 同一引用(不过滤时零成本)
});

test("filterTree:全不命中返回空", async () => {
  assert.equal((await filterTree(TREE, "不存在", SPEC)).length, 0);
});

// ── diagSuffix ──

test("diagSuffix:按区间求交并计数 severity", () => {
  const diags = [
    { range: { start: { line: 2 } }, severity: 1, message: "e1" },
    { range: { start: { line: 3 } }, severity: 2, message: "w1" },
    { range: { start: { line: 9 } }, severity: 1, message: "out" },
  ];
  const { suffix, tooltip } = diagSuffix(diags, 0, 4);
  assert.equal(suffix, " ✗1 ⚠1");
  assert.match(tooltip, /e1/);
  assert.match(tooltip, /w1/);
  assert.doesNotMatch(tooltip, /out/);
});

test("diagSuffix:无命中时空后缀空 tooltip", () => {
  const { suffix, tooltip } = diagSuffix([{ range: { start: { line: 100 } }, severity: 1 }], 0, 4);
  assert.equal(suffix, "");
  assert.equal(tooltip, "");
});

test("diagSuffix:畸形诊断(无 range)被忽略", () => {
  const { suffix } = diagSuffix([{} as never], 0, 10);
  assert.equal(suffix, "");
});
