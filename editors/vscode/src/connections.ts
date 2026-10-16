import * as vscode from "vscode";
import { PNode } from "./projectTree";
import { parseConnections, parseScene, type SNode } from "./parsers";

/**
 * 「信号连接」树(路线图 P1-10,编辑器侧):解析当前打开的 .tscn 的
 * [connection] 段——连接是声明式数据而非代码(engine-analyzer.md S 节),
 * 插件侧纯文本解析即可完整呈现,零服务器依赖。
 *
 * 点击连接跳 tscn 声明行;展开可见发射者/接收者节点(跳 [node] 声明行)。
 * 缺必填字段的连接以警告呈现(引擎对缺字段硬拒载,S1 规格)——
 * 这里是 AI 生成/手改工程损坏连接的最早暴露点。
 *
 * 大纲增强(21c):端点子节点急切物化(轻量),`locate` 供光标跟随;
 * 过滤(21b)由 PTreeDataProvider 统一承担。
 */
export class ConnectionTreeProvider {
  private cache: { uri: string; scene: ReturnType<typeof parseScene>; conns: Connection[]; nodes: PNode[] } | undefined;

  invalidate(): void {
    this.cache = undefined;
  }

  async roots(): Promise<PNode[]> {
    const editor = vscode.window.activeTextEditor;
    const uri = editor?.document.uri.toString();
    if (!editor || !uri || !/\.(tscn|scn)$/i.test(editor.document.fileName)) {
      return [new PNode("打开一个 .tscn 场景文件以查看信号连接", undefined, "info", undefined, undefined)];
    }
    if (!this.cache || this.cache.uri !== uri) {
      const text = editor.document.getText();
      const scene = parseScene(text);
      const conns = parseConnections(text);
      this.cache = { uri, scene, conns, nodes: conns.map((c) => this.toNode(c, uri)) };
    }
    return this.cache.nodes;
  }

  /** 21c:光标跟随——光标行所在或其上方最近的连接(conns 与 nodes 同序)。 */
  locate(line: number): PNode | undefined {
    const c = this.cache;
    if (!c) {
      return undefined;
    }
    let hit = -1;
    for (let i = 0; i < c.conns.length; i++) {
      if (c.conns[i].line <= line) {
        hit = i;
      } else {
        break;
      }
    }
    return hit >= 0 ? c.nodes[hit] : undefined;
  }

  private toNode(c: Connection, uri: string): PNode {
    const scene = this.cache!.scene;
    const broken = c.missing.length > 0;
    const label = c.signal || "(缺 signal)";
    const description = broken
      ? `⚠ 缺 ${c.missing.join("/")}——引擎将拒载此场景`
      : `${c.from} → ${c.to}.${c.method}()`;
    return new PNode(
      label,
      description,
      broken ? "warning" : "zap",
      { uri, line: c.line },
      undefined,
      this.endpointChildren(c, scene.nodePaths, uri),
    );
  }

  /** 发射者/接收者端点子节点:跳 [node] 声明行;端点缺失时以警告呈现(S2 ⚪ 静默丢弃的静态可见化)。 */
  private endpointChildren(
    c: Connection,
    nodePaths: Map<string, SNode>,
    uri: string,
  ): PNode[] {
    return ([
      ["发射者", c.from],
      ["接收者", c.to],
    ] as const).map(([role, path]) => {
      const node = nodePaths.get(path);
      return new PNode(
        `${role}:${node?.name ?? path}`,
        node ? node.type ?? "instance" : `⚠ 节点 "${path}" 不在本场景中(实例化后运行期才存在或已损坏)`,
        node ? "symbol-object" : "warning",
        node ? { uri, line: node.line } : undefined,
        undefined,
      );
    });
  }
}

/** parseConnections 的返回元素(类型别名,避免函数类型索引噪音)。 */
type Connection = ReturnType<typeof parseConnections>[number];
