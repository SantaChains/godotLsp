/**
 * 高亮区间收缩(纯函数,零 vscode/LSP 依赖,可单测)。
 *
 * 旧服务器内核(R0 冻结)的 documentHighlight/selectionRange 起点会把
 * 行首缩进(制表符/空格)一并计入 TextRange,终点正确锚在词尾——
 * 表现为高亮框吞掉前导空白。内核修复属 R2;插件侧按「尾锚收缩」
 * 防御性矫正:从区间终点向前跳过非标识符字符,再连续吞并标识符
 * 字符得到真实词段。对起点/终点多吞空白两种病态均有效。
 */

const IDENT = /[A-Za-z0-9_]/;

/**
 * 在行文本内,以 endChar 为终点向后收缩出一个标识符词段。
 * 返回 undefined 表示窗口内没有标识符字符(不该收缩,调用方保留原区间)。
 */
export function clampTail(lineText: string, endChar: number): { start: number; end: number } | undefined {
  let i = Math.min(endChar, lineText.length) - 1;
  // 先跳过区间尾部的非标识符字符(吞了尾随空白/错位的情形)。
  while (i >= 0 && !IDENT.test(lineText[i])) {
    i--;
  }
  if (i < 0) {
    return undefined;
  }
  const end = i + 1;
  // 再连续吞并标识符字符回到词首(吞了行首缩进的情形)。
  while (i >= 0 && IDENT.test(lineText[i])) {
    i--;
  }
  return { start: i + 1, end };
}
