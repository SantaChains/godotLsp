import * as vscode from "vscode";
import { PNode } from "./projectTree";
import { parseConnections, parseGdConnects, parseScene, type SNode } from "./parsers";

/**
 * 「信号连接」树(路线图 P1-10,编辑器侧):双源呈现。
 * - .tscn:解析 [connection] 段——连接是声明式数据而非代码(engine-analyzer.md S 节),
 *   插件侧纯文本解析即可完整呈现,零服务器依赖。
 * - .gd:静态扫描 connect() 调用(parseGdConnects)——代码级运行期连接,
 *   tscn 里没有对应标签,此前只认 tscn 导致「gd 有信号链接却看不见」。
 *
 * 点击连接跳声明行;展开可见发射者/接收者节点(跳 [node] 声明行)。
 * 缺必填字段的连接以警告呈现(引擎对缺字段硬拒载,S1 规格)——
 * 这里是 AI 生成/手改工程损坏连接的最早暴露点。
 *
 * 大纲增强(21c):端点子节点急切物化(轻量),`locate` 供光标跟随;
 * 过滤(21b)由 PTreeDataProvider 统一承担。
 */
export class ConnectionTreeProvider {
  private cache:
    | { uri: string; scene: ReturnType<typeof parseScene>; conns: Connection[]; nodes: PNode[] }
    | { uri: string; gd: true; nodes: PNode[] }
    | undefined;

  invalidate(): void {
    this.cache = undefined;
  }

  async roots(): Promise<PNode[]> {
    const editor = vscode.window.activeTextEditor;
    const uri = editor?.document.uri.toString();
    if (!editor || !uri) {
      return [placeholder()];
    }
    if (editor.document.languageId === "gdscript") {
      // gd 侧:代码级 connect() 调用,点击跳调用行。
      const nodes = parseGdConnects(editor.document.getText()).map(
        (c) =>
          new PNode(
            c.signal || "(匿名 callable)",
            c.source,
            "zap",
            { uri, line: c.line },
            undefined,
            undefined,
            c.text,
          ),
      );
      this.cache = { uri, gd: true, nodes };
      return nodes.length
        ? nodes
        : [
            new PNode(
              "本脚本没有 connect() 调用;声明式连接在对应 .tscn 的 [connection] 段",
              undefined,
              "info",
              undefined,
              undefined,
              undefined,
              undefined,
              { command: "godotlsp.pickScene", title: "选择场景" },
            ),
          ];
    }
    if (!/\.(tscn|scn)$/i.test(editor.document.fileName)) {
      return [placeholder()];
    }
    if (!this.cache || this.cache.uri !== uri || "gd" in this.cache) {
      const text = editor.document.getText();
      const scene = parseScene(text);
      const conns = parseConnections(text);
      // toNode 以参数收 scene——对象字面量构造期 this.cache 尚未更新,
      // 读 this.cache!.scene 是 undefined(修复「跳转资源时 reading 'scene' 崩溃」)。
      this.cache = { uri, scene, conns, nodes: conns.map((c) => this.toNode(c, scene, uri)) };
    }
    return this.cache.nodes.length
      ? this.cache.nodes
      : [
          new PNode(
            "本场景没有 [connection];代码级连接请切到对应 .gd 查看",
            undefined,
            "info",
            undefined,
            undefined,
            undefined,
            undefined,
            { command: "godotlsp.pickScene", title: "选择场景" },
          ),
        ];
  }

  /** 21c:光标跟随——光标行所在或其上方最近的连接(conns 与 nodes 同序)。 */
  locate(line: number): PNode | undefined {
    const c = this.cache;
    if (!c || "gd" in c) {
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

  private toNode(c: Connection, scene: ReturnType<typeof parseScene>, uri: string): PNode {
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

/** 非 tscn/gd 编辑器时的占位引导:点击弹 quick pick 选场景。 */
function placeholder(): PNode {
  return new PNode(
    "点击选择要查看的 .tscn 场景…",
    undefined,
    "list-unordered",
    undefined,
    undefined,
    undefined,
    undefined,
    { command: "godotlsp.pickScene", title: "选择场景" },
  );
}

/** parseConnections 的返回元素(类型别名,避免函数类型索引噪音)。 */
type Connection = ReturnType<typeof parseConnections>[number];
