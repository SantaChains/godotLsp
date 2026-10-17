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
    // 根级:外部资源分组(A3)置顶,其后场景节点树。
    const resGroup = this.resGroup(uri);
    return resGroup ? [resGroup, ...this.cache.nodes] : this.cache.nodes;
  }

  /**
   * A3 外部资源分组:kind 分类(ext.type 优先,扩展名兜底)→ 同 path 去重
   * (一个资源被多个 ext id / 多节点引用只显示一次),条目带引用计数。
   * 点击由 goto 命令兜底:文本资源跳转,二进制 fallback vscode.open 原生打开。
   */
  private resGroup(docUri: string): PNode | undefined {
    const scene = this.cache!.scene;
    if (scene.ext.size === 0) {
      return undefined;
    }
    // kind → path → 归一条目(同 path 多 ext id 合并)。
    const byKind = new Map<ResKind, Map<string, { type: string; path: string }>>();
    for (const { type, path } of scene.ext.values()) {
      const kind = kindOf(type, path);
      let bucket = byKind.get(kind);
      if (!bucket) {
        bucket = new Map();
        byKind.set(kind, bucket);
      }
      if (!bucket.has(path)) {
        bucket.set(path, { type, path });
      }
    }
    // 引用计数挂归一后的 path 上(跨 ext id 累加)。
    const countByPath = new Map<string, number>();
    for (const [id, n] of scene.refCounts) {
      const path = scene.ext.get(id)?.path;
      if (path) {
        countByPath.set(path, (countByPath.get(path) ?? 0) + n);
      }
    }
    const kindNodes: PNode[] = [];
    for (const kind of KIND_ORDER) {
      const bucket = byKind.get(kind);
      if (!bucket) {
        continue;
      }
      const items = [...bucket.values()].map(({ type, path }) => {
        const refs = countByPath.get(path) ?? 0;
        const label = path.split("/").pop() ?? path;
        const desc = refs > 1 ? `${path} · ${refs} 处引用` : path;
        return new PNode(label, desc, KIND_ICON[kind], resJump(path, docUri), undefined, undefined, type || kind);
      });
      kindNodes.push(new PNode(KIND_LABEL[kind], String(items.length), KIND_ICON[kind], undefined, undefined, items));
    }
    if (kindNodes.length === 0) {
      return undefined;
    }
    return new PNode("外部资源", String(scene.ext.size), "file-media", undefined, undefined, kindNodes);
  }

  private toNode(n: SNode, uri: string): PNode {
    const scene = this.cache?.scene;
    const ext = n.instanceId ? scene?.ext.get(n.instanceId) : undefined;
    // A2:script 绑定最常关心(引擎 resource_format_text.cpp:1985 同键语义),
    // 优先进 description;其余绑定降为子资源条目。
    const scriptExt = n.bindings.has("script") ? scene?.ext.get(n.bindings.get("script")!) : undefined;
    const scriptName = scriptExt?.path.split("/").pop();
    const description = scriptName
      ? `${scriptName} (${n.type ?? ext?.type ?? "instance"})`
      : (ext?.path ?? n.type ?? (n.instanceId ? `instance ExtResource("${n.instanceId}")` : undefined));
    const icon = scriptName ? "symbol-file" : n.type ? iconForKind(5) : "symbol-reference";
    // 其余绑定(script 之外)作为资源子条目,置顶于场景子节点之前。
    const bindingNodes = [...n.bindings.entries()]
      .filter(([attr]) => attr !== "script")
      .map(([attr, id]) => {
        const e = scene?.ext.get(id);
        const kind = e ? kindOf(e.type, e.path) : "other";
        return new PNode(
          e?.path.split("/").pop() ?? attr,
          `${attr} = ExtResource("${id}")`,
          KIND_ICON[kind],
          e ? resJump(e.path, uri) : undefined,
          undefined,
          undefined,
          e?.type,
        );
      });
    const children = [...bindingNodes, ...n.children.map((c) => this.toNode(c, uri))];
    const node = new PNode(
      n.name,
      description,
      icon,
      { uri, line: n.line },
      undefined,
      children.length ? children : undefined,
    );
    this.index.set(n, node);
    return node;
  }

  /** 21c:光标跟随——返回行号落在其声明区间的最深节点;无命中 undefined。 */
  locate(line: number): PNode | undefined {
    if (!this.cache) {
      return undefined;
    }
    const s = deepestSNode(this.cache.scene.roots, line);
    return s ? this.index.get(s) : undefined;
  }
}

// ───────────────────────── 资源 kind 分类与跳转(A2/A3 共用) ─────────────────────────

type ResKind = "script" | "scene" | "texture" | "audio" | "font" | "other";

/** ext.type 优先(引擎资源类名即语义),path 扩展名兜底。 */
function kindOf(type: string, path: string): ResKind {
  const t = type.toLowerCase();
  if (t.includes("script")) {
    return "script";
  }
  if (t.includes("scene")) {
    return "scene";
  }
  if (t.includes("texture") || t.includes("image")) {
    return "texture";
  }
  if (t.includes("audio") || t.includes("stream")) {
    return "audio";
  }
  if (t.includes("font")) {
    return "font";
  }
  switch (path.split(".").pop()?.toLowerCase()) {
    case "gd":
      return "script";
    case "tscn":
    case "scn":
      return "scene";
    case "png":
    case "jpg":
    case "jpeg":
    case "svg":
    case "webp":
      return "texture";
    case "ogg":
    case "wav":
    case "mp3":
      return "audio";
    case "ttf":
    case "otf":
    case "fnt":
    case "woff":
    case "woff2":
      return "font";
    default:
      return "other";
  }
}

const KIND_ORDER: ResKind[] = ["script", "scene", "texture", "audio", "font", "other"];
const KIND_LABEL: Record<ResKind, string> = {
  script: "脚本",
  scene: "场景",
  texture: "纹理",
  audio: "音频",
  font: "字体",
  other: "其他",
};
const KIND_ICON: Record<ResKind, string> = {
  script: "symbol-file",
  scene: "symbol-reference",
  texture: "file-media",
  audio: "play",
  font: "symbol-string",
  other: "file",
};

/** res:// → workspace file uri;未打开工程时 null。 */
export function resToUri(res: string): vscode.Uri | null {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    return null;
  }
  const rel = res.replace(/^res:\/\//, "");
  return vscode.Uri.joinPath(folder.uri, ...rel.split("/").filter(Boolean));
}

/** 资源条目跳转:文本资源 goto(file uri);二进制也走 goto,由命令层兜底 vscode.open。 */
function resJump(res: string, docUri: string): { uri: string; line: number } | undefined {
  const target = resToUri(res);
  return target ? { uri: target.toString(), line: 0 } : { uri: docUri, line: 0 };
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
