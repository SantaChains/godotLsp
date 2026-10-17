/**
 * res:// 资源路径链接(A4,对标官方 godot-tools「ctrl+click on res:// links」):
 * gd 与 tscn/tres 里的 res:// 字符串变可点击链接,Ctrl+点击打开目标文件。
 * 纯客户端 documentLink,零协议成本;目标解析复用 sceneTree.resToUri
 * (res:// → workspace folder),解析不了(未打开工程)时不产链接。
 */

import * as vscode from "vscode";
import { resToUri } from "./sceneTree";

// 终止字符:引号/括号/空白/逗号(tscn 属性值与 gd 字符串两种宿主通吃)。
const RE_RES = /res:\/\/[^\s"'`)\]}>,]+/g;

export class ResLinkProvider implements vscode.DocumentLinkProvider {
  provideDocumentLinks(document: vscode.TextDocument): vscode.DocumentLink[] {
    const links: vscode.DocumentLink[] = [];
    const text = document.getText();
    for (const m of text.matchAll(RE_RES)) {
      const target = resToUri(m[0]);
      if (!target) {
        continue;
      }
      const start = document.positionAt(m.index ?? 0);
      const end = document.positionAt((m.index ?? 0) + m[0].length);
      links.push(new vscode.DocumentLink(new vscode.Range(start, end), target));
    }
    return links;
  }
}
