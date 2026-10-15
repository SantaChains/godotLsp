import assert from "node:assert/strict";
import { test } from "node:test";
import { clampTail } from "../src/clampTail";

test("clampTail:起点锚到行首(吞制表符+空格)时收缩回词段", () => {
  // 截图复现形态:行首 \t + 空格 + type_hint_prefixes,服务器起点=0,终点=词尾。
  const line = "\t\ttype_hint_prefixes += \"x\"";
  const end = line.indexOf(" +=");
  assert.deepEqual(clampTail(line, end), { start: 2, end });
});

test("clampTail:区间本身已是纯标识符时原样返回", () => {
  assert.deepEqual(clampTail("  foobar  ", 9), { start: 2, end: 8 });
});

test("clampTail:终点吞了尾随空白/错位时先跳非标识符再收缩", () => {
  const line = "var value = 1";
  // 终点错位到词尾 +2 的空白处。
  assert.deepEqual(clampTail(line, 10), { start: 4, end: 9 });
});

test("clampTail:窗口内无标识符字符时返回 undefined", () => {
  assert.equal(clampTail("   \t\t ", 5), undefined);
  assert.equal(clampTail("", 0), undefined);
});

test("clampTail:endChar 超出行长时安全夹紧", () => {
  assert.deepEqual(clampTail("abc", 999), { start: 0, end: 3 });
});

test("clampTail:词首不受人为窗口下界影响(向前扫到非标识符为止)", () => {
  const line = "if elem_type_hint.is_valid_int():";
  // 光标词 elem_type_hint,服务器起点锚行首 0,终点在词尾。
  const end = line.indexOf(".");
  assert.deepEqual(clampTail(line, end), { start: 3, end });
});
