import * as fs from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";
import type { LanguageClient } from "vscode-languageclient/node";
import { documentSymbols, iconForKind, LspSymbol } from "./lsp";
import { parseAutoloads } from "./parsers";
import { withCopy } from "./copyable";

/** 项目结构树节点:jump = 点击跳转目标;expand = 惰性子节点加载器。 */
export class PNode {
  children: PNode[] | undefined;
  constructor(
    readonly label: string,
    readonly description: string | undefined,
    readonly icon: string,
    readonly jump: { uri: string; line: number } | undefined,
    readonly expand: (() => Promise<PNode[] | undefined>) | undefined,
  ) {}
}

/** PNode → TreeItem(项目结构/场景结构共用)。 */
export function pTreeItem(node: PNode): vscode.TreeItem {
  const hasChildren = node.expand !== undefined || (node.children?.length ?? 0) > 0;
  const item = new vscode.TreeItem(
    node.label,
    hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None,
  );
  item.description = node.description;
  item.iconPath = new vscode.ThemeIcon(node.icon);
  if (node.jump) {
    item.command = {
      command: "godotlsp.goto",
      title: "跳转",
      arguments: [node.jump.uri, node.jump.line, 0, node.label],
    };
  }
  return withCopy(item, {
    name: node.label,
    ref: node.jump ? `${relDisplay(node.jump.uri)}:${node.jump.line + 1}` : node.description,
  });
}

/** 通用适配器:把「roots() + PNode 缓存子节点」包装成 TreeDataProvider。 */
export class PTreeDataProvider implements vscode.TreeDataProvider<PNode> {
  private _onDidChange = new vscode.EventEmitter<PNode | undefined | void>();
  readonly onDidChangeTreeData = this._onDidChange.event;

  constructor(private readonly roots: () => Promise<PNode[]>) {}

  refresh(): void {
    this._onDidChange.fire();
  }

  async getChildren(node?: PNode): Promise<PNode[]> {
    if (!node) {
      return this.roots();
    }
    if (node.children) {
      return node.children;
    }
    if (node.expand) {
      node.children = (await node.expand()) ?? [];
      return node.children;
    }
    return [];
  }

  getTreeItem(node: PNode): vscode.TreeItem {
    return pTreeItem(node);
  }
}

/** 延迟膨胀的分区节点:children 在首次展开时才计算。 */
export function section(label: string, icon: string, load: () => Promise<PNode[]>): PNode {
  return new PNode(label, undefined, icon, undefined, async () => await load());
}

/** 文件节点:惰性展开为 LSP documentSymbol 子树(通信统一走 lsp.ts)。 */
export async function fileSymbolChildren(
  client: () => LanguageClient | undefined,
  uri: string,
  log: vscode.LogOutputChannel,
): Promise<PNode[] | undefined> {
  const syms = await documentSymbols(client(), uri, log);
  if (syms === null) {
    return undefined;
  }
  return syms.map((s) => symbolToNode(s, uri));
}

function symbolToNode(s: LspSymbol, uri: string): PNode {
  const node = new PNode(
    s.name,
    s.detail,
    iconForKind(s.kind),
    { uri, line: s.selectionRange.start.line },
    s.children?.length
      ? async () => (s.children ?? []).map((c) => symbolToNode(c, uri))
      : undefined,
  );
  return node;
}

/** project.godot 的 res:// 路径 → 工作区内绝对路径(找不到返回 undefined)。 */
export async function resToUri(resPath: string): Promise<string | undefined> {
  const rel = resPath.replace(/^res:\/\//, "");
  const files = await vscode.workspace.findFiles(`**/${rel.split("/").map(escapeSeg).join("/")}`, undefined, 1);
  return files[0]?.toString();
}

function escapeSeg(seg: string): string {
  return seg.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** 扫描工作区 .gd 文件的 class_name 注册表(跳过 .godot/.git/.gdignore 目录)。 */
export async function scanClassNames(
  onProgress?: (msg: string) => void,
): Promise<{ name: string; uri: string; line: number }[]> {
  const out: { name: string; uri: string; line: number }[] = [];
  const files = await vscode.workspace.findFiles("**/*.gd", "**/.godot/**,**/.git/**", 5000);
  for (const file of files) {
    let text: string;
    try {
      text = fs.readFileSync(file.fsPath, "utf8");
    } catch {
      continue;
    }
    // 只看前 64 行:class_name 按规范在文件头;避免为巨型文件做全文正则。
    const head = text.split(/\r?\n/, 64);
    for (let i = 0; i < head.length; i++) {
      const m = /^\s*class_name\s+([A-Za-z_]\w*)/.exec(head[i]);
      if (m) {
        out.push({ name: m[1], uri: file.toString(), line: i });
        break;
      }
    }
  }
  onProgress?.(`class_name 扫描完成:${out.length} 个`);
  return out.sort((a, b) => a.name.localeCompare(b.name));
}

/** 相对工作区根的展示路径。 */
export function relDisplay(uri: string): string {
  const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const fsPath = uri.startsWith("file:") ? vscode.Uri.parse(uri).fsPath : uri;
  return root && fsPath.startsWith(root) ? path.relative(root, fsPath) : fsPath;
}

/**
 * 「项目结构」树(路线图 P2-18):autoload 单例 → class_name 注册表 →
 * 文件级大纲(复用 LSP documentSymbol,与「当前文件符号」同源)。点击跳转。
 */
export class ProjectTreeProvider {
  private classesCache: { name: string; uri: string; line: number }[] | undefined;

  constructor(
    private readonly client: () => LanguageClient | undefined,
    private readonly output: vscode.LogOutputChannel,
  ) {}

  invalidateClasses(): void {
    this.classesCache = undefined;
  }

  async roots(): Promise<PNode[]> {
    return [
      section("Autoload 单例", "gear", () => this.autoloadChildren()),
      section("全局类 (class_name)", "symbol-class", () => this.classChildren()),
    ];
  }

  private async autoloadChildren(): Promise<PNode[]> {
    const files = await vscode.workspace.findFiles("project.godot", undefined, 1);
    if (files.length === 0) {
      return [new PNode("未找到 project.godot(当前工作区不是 Godot 工程)", undefined, "info", undefined, undefined)];
    }
    let text: string;
    try {
      text = fs.readFileSync(files[0].fsPath, "utf8");
    } catch (err) {
      this.output.debug(`读取 project.godot 失败:${String(err)}`);
      return [];
    }
    const entries = parseAutoloads(text);
    const nodes: PNode[] = [];
    for (const e of entries) {
      const uri = await resToUri(e.resPath);
      nodes.push(
        new PNode(
          e.name,
          e.resPath,
          "symbol-namespace",
          uri ? { uri, line: 0 } : undefined,
          uri ? () => fileSymbolChildren(this.client, uri, this.output) : undefined,
        ),
      );
    }
    return nodes.length > 0 ? nodes : [new PNode("没有 autoload 单例", undefined, "info", undefined, undefined)];
  }

  private async classChildren(): Promise<PNode[]> {
    if (!this.classesCache) {
      this.classesCache = await scanClassNames();
    }
    const nodes = this.classesCache.map(
      (c) =>
        new PNode(
          c.name,
          relDisplay(c.uri),
          "symbol-class",
          { uri: c.uri, line: c.line },
          () => fileSymbolChildren(this.client, c.uri, this.output),
        ),
    );
    return nodes.length > 0 ? nodes : [new PNode("没有 class_name 全局类", undefined, "info", undefined, undefined)];
  }
}
