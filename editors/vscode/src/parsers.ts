/**
 * Godot 文本格式纯解析器:零 vscode/LSP 依赖,可独立单测(node:test)。
 * 覆盖 project.godot [autoload] 段与 .tscn 节点/资源结构。
 */

export interface AutoloadEntry {
  name: string;
  resPath: string;
}

/** 解析 project.godot 的 [autoload] 段:Name="*res://..."(前导 * 表示单例)。 */
export function parseAutoloads(text: string): AutoloadEntry[] {
  const out: AutoloadEntry[] = [];
  let inSection = false;
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (line.startsWith("[")) {
      inSection = line === "[autoload]";
      continue;
    }
    if (!inSection || line.startsWith(";") || !line.includes("=")) {
      continue;
    }
    const eq = line.indexOf("=");
    const name = line.slice(0, eq).trim();
    const value = line.slice(eq + 1).trim().replace(/^"|"$/g, "");
    const res = value.replace(/^\*/, "");
    if (name && res.startsWith("res://")) {
      out.push({ name, resPath: res });
    }
  }
  return out;
}

export interface SNode {
  name: string;
  type?: string;
  instanceId?: string;
  parent?: string;
  line: number;
  children: SNode[];
  /**
   * 节点属性行里的 ExtResource 绑定:属性名 → ext id。
   * 引擎锚点(.refs/godot/scene/resources/resource_format_text.cpp):
   * :1985 `is_script = name == CoreStringName(script)` —— `script` 键即脚本绑定;
   * :1634 `ExtResource("id")` 序列化形态。任意属性名不设白名单——
   * 任何 OBJECT 属性(script/texture/stream/theme_override_styles/…)
   * 都可能是外部资源引用。SubResource 引用不在本表(纯场景内)。
   */
  bindings: Map<string, string>;
}

export interface SceneFile {
  ext: Map<string, { type: string; path: string }>;
  roots: SNode[];
  /** NodePath(含根 ".")→ 节点;[connection] from/to 的跳转定位用。 */
  nodePaths: Map<string, SNode>;
  /** 全场景 ext id → 引用次数(instance + 任意属性行绑定),A3 资源分组用。 */
  refCounts: Map<string, number>;
  /** sub_resource id → 类型(A5 hover 展示;纯场景内资源无 path)。 */
  subs: Map<string, string>;
}

const RE_NODE = /^\s*\[node\s+([^\]]*)\]/;
const RE_EXT = /^\s*\[ext_resource\s+([^\]]*)\]/;
const RE_SUB = /^\s*\[sub_resource\s+([^\]]*)\]/;
// 值的三形态:引号串 / ExtResource("id") / 裸值(connection 的 binds=1 等数值字段无引号)。
const RE_ATTR = /(\w+)\s*=\s*(?:"((?:\\.|[^"\\])*)"|ExtResource\("([^"]+)"\)|([^\s\]]+))/g;

function parseAttrs(source: string): Map<string, string> {
  const attrs = new Map<string, string>();
  for (const a of source.matchAll(RE_ATTR)) {
    attrs.set(a[1], a[3] ?? a[4] ?? a[2] ?? "");
  }
  return attrs;
}

// 属性赋值行的 ExtResource 引用(属性名可含 /:theme_override_styles/panel 等)。
const RE_PROP = /^\s*([\w/]+)\s*=\s*ExtResource\("([^"]+)"\)/;

/** 解析 .tscn:ext_resource 注册表 + node 层级(parent 路径 "." 为场景根)。 */
export function parseScene(text: string): SceneFile {
  const lines = text.split(/\r?\n/);
  const ext = new Map<string, { type: string; path: string }>();
  const subs = new Map<string, string>();
  const nodes: SNode[] = [];
  // 当前归属节点:属性行归最近一个 [node];其他标签(sub_resource/resource/
  // connection/gd_scene…)切断归属——其下属性不属于任何场景节点。
  let current: SNode | undefined;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (/^\s*\[/.test(line)) {
      current = undefined;
      const e = RE_EXT.exec(line);
      if (e) {
        const attrs = parseAttrs(e[1]);
        const id = attrs.get("id") ?? "";
        if (id) {
          ext.set(id, { type: attrs.get("type") ?? "", path: attrs.get("path") ?? "" });
        }
        continue;
      }
      const s = RE_SUB.exec(line);
      if (s) {
        const attrs = parseAttrs(s[1]);
        const id = attrs.get("id") ?? "";
        if (id) {
          subs.set(id, attrs.get("type") ?? "");
        }
        continue;
      }
      const n = RE_NODE.exec(line);
      if (n) {
        const attrs = parseAttrs(n[1]);
        current = {
          name: attrs.get("name") ?? "(匿名)",
          type: attrs.get("type"),
          instanceId: attrs.get("instance"),
          parent: attrs.get("parent"),
          line: i,
          children: [],
          bindings: new Map(),
        };
        nodes.push(current);
      }
      continue;
    }
    if (current) {
      const m = RE_PROP.exec(line);
      if (m) {
        current.bindings.set(m[1], m[2]);
      }
    }
  }
  // 组装层级:parent="." 挂场景根;路径 "A/B" 逐级查找;找不到父亲的挂根级(容错)。
  const byPath = new Map<string, SNode>();
  const roots: SNode[] = [];
  for (const node of nodes) {
    if (node.parent === undefined) {
      byPath.set(".", node);
      roots.push(node);
    } else if (node.parent === "." && byPath.has(".")) {
      byPath.set(node.name, node);
      byPath.get(".")?.children.push(node);
    } else {
      // parent 指向不存在的节点(含 parent="." 但场景无根的损坏文件)——容错挂根级。
      const p = byPath.get(node.parent);
      if (p) {
        byPath.set(`${node.parent}/${node.name}`, node);
        p.children.push(node);
      } else {
        roots.push(node);
      }
    }
  }
  // 引用计数:instance(instanceId)与属性行绑定(bindings)都是 ext id 的引用。
  const refCounts = new Map<string, number>();
  for (const n of nodes) {
    if (n.instanceId) {
      refCounts.set(n.instanceId, (refCounts.get(n.instanceId) ?? 0) + 1);
    }
    for (const id of n.bindings.values()) {
      refCounts.set(id, (refCounts.get(id) ?? 0) + 1);
    }
  }
  return { ext, roots, nodePaths: byPath, refCounts, subs };
}

export interface SceneConnection {
  /** 发射者 NodePath(引擎存时 simplified(),"." 为场景根)。 */
  from: string;
  /** 接收者 NodePath。 */
  to: string;
  signal: string;
  method: string;
  /** [connection] 标签行号(0 基)。 */
  line: number;
  /**
   * 缺失的必填字段。规格(engine-analyzer.md S1):四字段缺一引擎即
   * ERR_FILE_CORRUPT 拒载——解析层不丢弃,由视图以警告呈现,
   * 恰好暴露 AI 生成/手改工程的损坏连接。
   */
  missing: string[];
  /** 可选字段原样保留(flags=3 即默认 CONNECT_PERSIST;binds/unbinds 为参数个数)。 */
  flags?: string;
  binds?: string;
  unbinds?: string;
}

// [connection] 可完全无属性(损坏 tag)——四字段全 missing,引擎拒载。
const RE_CONN = /^\s*\[connection(?:\s+([^\]]*))?\]/;

/** 解析 .tscn 的 [connection] 段:信号连接的声明式数据(非代码)。 */
export function parseConnections(text: string): SceneConnection[] {
  const out: SceneConnection[] = [];
  const lines = text.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const c = RE_CONN.exec(lines[i]);
    if (!c) {
      continue;
    }
    const attrs = parseAttrs(c[1] ?? "");
    const conn: SceneConnection = {
      from: attrs.get("from") ?? "",
      to: attrs.get("to") ?? "",
      signal: attrs.get("signal") ?? "",
      method: attrs.get("method") ?? "",
      line: i,
      missing: [],
    };
    for (const key of ["from", "to", "signal", "method"] as const) {
      if (!conn[key]) {
        conn.missing.push(key);
      }
    }
    if (attrs.has("flags")) {
      conn.flags = attrs.get("flags");
    }
    if (attrs.has("binds")) {
      conn.binds = attrs.get("binds");
    }
    if (attrs.has("unbinds")) {
      conn.unbinds = attrs.get("unbinds");
    }
    out.push(conn);
  }
  return out;
}
