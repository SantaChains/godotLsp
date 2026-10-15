import * as vscode from "vscode";
import type { LanguageClient } from "vscode-languageclient/node";
import { documentSymbols, iconForKind, LspSymbol } from "./lsp";

/** 符号树节点:根节点持有文档 URI,子节点只携带符号。 */
export class SymbolNode {
  constructor(
    readonly symbol: LspSymbol | undefined,
    readonly uri: string | undefined,
    readonly children: SymbolNode[],
  ) {}
}

/**
 * 「当前文件符号」树:数据源是服务器早已实现的 LSP `textDocument/documentSymbol`
 * (类/函数/枚举/信号/常量嵌套结构),点击即跳转到声明。请求与降级统一走
 * `lsp.ts`;服务器离线或文件无声明时视图留空,不报错刷屏。
 */
export class SymbolTreeProvider implements vscode.TreeDataProvider<SymbolNode> {
  private _onDidChange = new vscode.EventEmitter<SymbolNode | undefined | void>();
  readonly onDidChangeTreeData = this._onDidChange.event;

  constructor(
    private readonly client: () => LanguageClient | undefined,
    private readonly output: vscode.LogOutputChannel,
  ) {}

  refresh(): void {
    this._onDidChange.fire();
  }

  getTreeItem(node: SymbolNode): vscode.TreeItem {
    const s = node.symbol;
    if (!s) {
      return new vscode.TreeItem("", vscode.TreeItemCollapsibleState.None);
    }
    const item = new vscode.TreeItem(
      s.name,
      node.children.length > 0
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None,
    );
    item.description = s.detail;
    item.iconPath = new vscode.ThemeIcon(iconForKind(s.kind));
    item.command = {
      command: "godotlsp.goto",
      title: "跳转到声明",
      arguments: [node.uri, s.selectionRange.start.line, s.selectionRange.start.character, s.name],
    };
    return item;
  }

  async getChildren(node?: SymbolNode): Promise<SymbolNode[]> {
    if (node) {
      return node.children;
    }
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      return [];
    }
    const uri = editor.document.uri.toString();
    const syms = await documentSymbols(this.client(), uri, this.output);
    return (syms ?? []).map((s) => new SymbolNode(s, uri, toNodes(s.children ?? [], uri)));
  }
}

function toNodes(syms: LspSymbol[], uri: string): SymbolNode[] {
  return syms.map((s) => new SymbolNode(s, uri, toNodes(s.children ?? [], uri)));
}
