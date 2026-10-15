import * as vscode from "vscode";
import type { Middleware } from "vscode-languageclient/node";
import { clampTail } from "./clampTail";

/**
 * LSP 客户端中间件:documentHighlight 响应的词级矫正。
 *
 * 服务器内核(R0 冻结的旧 gdscript-ide)返回的高亮区间起点锚在行首,
 * 吞掉行首缩进(制表符/空格)——见截图复现。内核修复属 R2 路线图
 * (P1.5 条目 15 的验收探针),插件侧在此做防御性收缩:对每个单行
 * 高亮区间按「尾锚收缩」回归真实词段;多行/异常区间原样保留。
 */
export const highlightClampMiddleware: Middleware = {
  async provideDocumentHighlights(document, position, token, next) {
    const highlights = await next(document, position, token);
    if (!highlights || highlights.length === 0) {
      return highlights;
    }
    const out: vscode.DocumentHighlight[] = [];
    for (const h of highlights) {
      const { range, kind } = h;
      if (range.start.line !== range.end.line || range.start.line < 0) {
        out.push(h);
        continue;
      }
      const line = range.start.line;
      const lineText = document.lineAt(line).text;
      const seg = clampTail(lineText, range.end.character);
      if (!seg) {
        out.push(h);
        continue;
      }
      out.push(
        new vscode.DocumentHighlight(
          new vscode.Range(line, seg.start, line, seg.end),
          kind,
        ),
      );
    }
    return out;
  },
};
