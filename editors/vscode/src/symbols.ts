import * as vscode from "vscode";
import type { LanguageClient } from "vscode-languageclient/node";
import { documentSymbols, iconForKind, LspSymbol } from "./lsp";
import { withCopy } from "./copyable";
import { relDisplay } from "./projectTree";
import { diagSuffix, filterTree } from "./treefilter";

/** 符号树节点:根节点持有文档 URI,子节点只携带符号。 */
export class SymbolNode {
  constructor(
    readonly symbol: LspSymbol | undefined,
    readonly uri: string | undefined,
    readonly children: SymbolNode[],
  ) {}
}

const SYM_SPEC = {
  labelOf: (n: SymbolNode) => n.symbol?.name ?? "",
  descOf: (n: SymbolNode) => n.symbol?.detail,
  kidsOf: (n: SymbolNode) => n.children,
} as const;

/**
 * 「当前文件符号」树:数据源是服务器早已实现的 LSP `textDocument/documentSymbol`
 * (类/函数/枚举/信号/常量嵌套结构),点击即跳转到声明。请求与降级统一走
 * `lsp.ts`;服务器离线或文件无声明时视图留空,不报错刷屏。
 *
 * 大纲增强(21b/21c/21d):过滤词按层生效(保留命中后代祖先链);
 * `locate` 供光标跟随 reveal;条目 description 附诊断计数旗标。
 */
export class SymbolTreeProvider implements vscode.TreeDataProvider<SymbolNode> {
  private _onDidChange = new vscode.EventEmitter<SymbolNode | undefined | void>();
  readonly onDidChangeTreeData = this._onDidChange.event;
  private filter = "";
  /** 最近一次构建的树(reveal 需要元素同一性,缓存引用)。 */
  private current: SymbolNode[] = [];

  constructor(
    private readonly client: () => LanguageClient | undefined,
    private readonly output: vscode.LogOutputChannel,
  ) {}

  refresh(): void {
    this._onDidChange.fire();
  }

  /** 设置过滤词(空串清除)并刷新。 */
  setFilter(filter: string): void {
    this.filter = filter;
    this.refresh();
  }

  getFilter(): string {
    return this.filter;
  }

  /**
   * 光标跟随(21c):返回包含该行的最深符号节点。
   * 树可能尚未构建(视图不可见时不渲染)——先确保构建一次。
   */
  async locate(line: number): Promise<SymbolNode | undefined> {
    await this.ensureTree();
    return findDeepest(this.current, line);
  }

  private async ensureTree(): Promise<SymbolNode[]> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      this.current = [];
      return this.current;
    }
    const uri = editor.document.uri.toString();
    const syms = await documentSymbols(this.client(), uri, this.output);
    this.current = (syms ?? []).map((s) => new SymbolNode(s, uri, toNodes(s.children ?? [], uri)));
    return this.current;
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
    // 21d:诊断旗标——类别图标保留(类型信息优先),severity 计数并入 description,
    // 具体消息进 tooltip(不遮挡,悬停可读)。
    const diags = node.uri ? vscode.languages.getDiagnostics(vscode.Uri.parse(node.uri)) : [];
    const { suffix, tooltip } = diagSuffix(diags, s.range.start.line, s.range.end.line);
    item.description = s.detail ? s.detail + suffix : suffix || undefined;
    if (tooltip) {
      item.tooltip = tooltip;
    }
    item.iconPath = new vscode.ThemeIcon(iconForKind(s.kind));
    item.command = {
      command: "godotlsp.goto",
      title: "跳转到声明",
      arguments: [node.uri, s.selectionRange.start.line, s.selectionRange.start.character, s.name],
    };
    return withCopy(item, {
      name: s.name,
      ref: node.uri ? `${relDisplay(node.uri)}:${s.selectionRange.start.line + 1}` : undefined,
    });
  }

  async getChildren(node?: SymbolNode): Promise<SymbolNode[]> {
    if (node) {
      return this.filter ? filterTree(node.children, this.filter, SYM_SPEC) : node.children;
    }
    const roots = await this.ensureTree();
    return this.filter ? filterTree(roots, this.filter, SYM_SPEC) : roots;
  }
}

/** 包含 line 的最深节点(区间闭区间;子命中优先于父)。 */
function findDeepest(nodes: SymbolNode[], line: number): SymbolNode | undefined {
  for (const n of nodes) {
    const s = n.symbol;
    if (!s) {
      continue;
    }
    if (line >= s.range.start.line && line <= s.range.end.line) {
      return findDeepest(n.children, line) ?? n;
    }
  }
  return undefined;
}

function toNodes(syms: LspSymbol[], uri: string): SymbolNode[] {
  return syms.map((s) => new SymbolNode(s, uri, toNodes(s.children ?? [], uri)));
}
