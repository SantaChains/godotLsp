import * as vscode from "vscode";
import { PNode } from "./projectTree";
import { iconForKind } from "./lsp";
import { parseScene, SNode } from "./parsers";

/**
 * 「场景结构」树(路线图 P2-17 插件侧):解析当前打开的 .tscn,展示节点层级
 * (instance 节点显示被实例化资源路径),点击跳转 tscn 声明行。
 * 解析器是纯函数(parsers.ts),本文件只做 vscode 树适配。
 */
export class SceneTreeProvider {
  private cache: { uri: string; scene: ReturnType<typeof parseScene> } | undefined;

  invalidate(): void {
    this.cache = undefined;
  }

  async roots(): Promise<PNode[]> {
    const editor = vscode.window.activeTextEditor;
    const uri = editor?.document.uri.toString();
    if (!editor || !uri || !/\.(tscn|scn)$/i.test(editor.document.fileName)) {
      return [new PNode("打开一个 .tscn 场景文件以查看结构", undefined, "info", undefined, undefined)];
    }
    if (!this.cache || this.cache.uri !== uri) {
      this.cache = { uri, scene: parseScene(editor.document.getText()) };
    }
    const { scene, uri: cachedUri } = this.cache;
    return scene.roots.map((n) => this.toNode(n, cachedUri));
  }

  private toNode(n: SNode, uri: string): PNode {
    const ext = n.instanceId ? this.cache?.scene.ext.get(n.instanceId) : undefined;
    const description =
      ext?.path ?? n.type ?? (n.instanceId ? `instance ExtResource("${n.instanceId}")` : undefined);
    const icon = n.type ? iconForKind(5) : "symbol-reference";
    return new PNode(
      n.name,
      description,
      icon,
      { uri, line: n.line },
      n.children.length ? async () => n.children.map((c) => this.toNode(c, uri)) : undefined,
    );
  }
}
