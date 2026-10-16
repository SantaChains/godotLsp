import * as vscode from "vscode";
import { PNode } from "./projectTree";
import { iconForKind } from "./lsp";
import { parseScene, SNode } from "./parsers";

/**
 * 「场景结构」树(路线图 P2-17 插件侧):解析当前打开的 .tscn,展示节点层级
 * (instance 节点显示被实例化资源路径),点击跳转 tscn 声明行。
 * 解析器是纯函数(parsers.ts),本文件只做 vscode 树适配。
 *
 * 大纲增强(21c):子树急切物化(PNode.children 直填)——跟随定位需要
 * 元素同一性 + 全树可遍历;场景节点是轻量对象,急切构建开销可忽略。
 * 过滤(21b)由 PTreeDataProvider 统一承担。
 */
export class SceneTreeProvider {
  private cache: { uri: string; scene: ReturnType<typeof parseScene>; nodes: PNode[] } | undefined;
  /** SNode → PNode 映射(跟随定位:命中 SNode 后取 reveal 用的 PNode 同一引用)。 */
  private index = new Map<SNode, PNode>();

  invalidate(): void {
    this.cache = undefined;
    this.index.clear();
  }

  async roots(): Promise<PNode[]> {
    const editor = vscode.window.activeTextEditor;
    const uri = editor?.document.uri.toString();
    if (!editor || !uri || !/\.(tscn|scn)$/i.test(editor.document.fileName)) {
      return [new PNode("打开一个 .tscn 场景文件以查看结构", undefined, "info", undefined, undefined)];
    }
    if (!this.cache || this.cache.uri !== uri) {
      this.index.clear();
      const scene = parseScene(editor.document.getText());
      this.cache = { uri, scene, nodes: scene.roots.map((n) => this.toNode(n, uri)) };
    }
    return this.cache.nodes;
  }

  /** 21c:光标跟随——返回行号落在其声明区间的最深节点;无命中 undefined。 */
  locate(line: number): PNode | undefined {
    if (!this.cache) {
      return undefined;
    }
    const s = deepestSNode(this.cache.scene.roots, line);
    return s ? this.index.get(s) : undefined;
  }

  private toNode(n: SNode, uri: string): PNode {
    const ext = n.instanceId ? this.cache?.scene.ext.get(n.instanceId) : undefined;
    const description =
      ext?.path ?? n.type ?? (n.instanceId ? `instance ExtResource("${n.instanceId}")` : undefined);
    const icon = n.type ? iconForKind(5) : "symbol-reference";
    const node = new PNode(
      n.name,
      description,
      icon,
      { uri, line: n.line },
      undefined,
      n.children.length ? n.children.map((c) => this.toNode(c, uri)) : undefined,
    );
    this.index.set(n, node);
    return node;
  }
}

/** 子树最大声明行:节点区间判定的闭区间右端。 */
function subtreeMaxLine(n: SNode): number {
  return n.children.reduce((m, c) => Math.max(m, subtreeMaxLine(c)), n.line);
}

/** 包含 line 的最深 SNode(子命中优先于父;场景声明行单调,区间互不覆盖)。 */
function deepestSNode(nodes: SNode[], line: number): SNode | undefined {
  for (const n of nodes) {
    if (line >= n.line && line <= subtreeMaxLine(n)) {
      return deepestSNode(n.children, line) ?? n;
    }
  }
  return undefined;
}
