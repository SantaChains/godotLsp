/**
 * A5/A6 纯逻辑(零 vscode 依赖,node:test 可测):
 * 资源引用定位、使用侧 inlay、res 路径换算、脚本↔场景互跳匹配。
 */

import { parseScene } from "./parsers";

/** 行内光标处的 ExtResource/SubResource 引用(col 为字符下标,闭区间命中)。 */
export function resRefAt(line: string, col: number): { kind: "ext" | "sub"; id: string } | undefined {
  for (const m of line.matchAll(/(Ext|Sub)Resource\("([^"]+)"\)/g)) {
    const start = m.index ?? 0;
    const end = start + m[0].length;
    if (col >= start && col <= end) {
      return { kind: m[1] === "Ext" ? "ext" : "sub", id: m[2] };
    }
  }
  return undefined;
}

export interface SceneInlay {
  /** 0-based 行号。 */
  line: number;
  /** 行尾 hint 文本,如 `← res://player.gd`。 */
  label: string;
}

/**
 * 使用侧行尾 inlay:`attr = ExtResource("id")` 与 `[node … instance=ExtResource("id")]`
 * → `← 解析后的 res:// 路径`。ext 表来自 parseScene(与树/A3 同源)。
 * 声明行([ext_resource])本身已含 type/path,不做声明侧 hint(信息冗余)。
 */
export function sceneInlays(text: string): SceneInlay[] {
  const scene = parseScene(text);
  const lines = text.split(/\r?\n/);
  const out: SceneInlay[] = [];
  lines.forEach((line, i) => {
    let id: string | undefined;
    const prop = /^\s*[\w/]+\s*=\s*ExtResource\("([^"]+)"\)/.exec(line);
    if (prop) {
      id = prop[1];
    } else {
      const inst = /\[node[^\]]*instance\s*=\s*ExtResource\("([^"]+)"\)/.exec(line);
      if (inst) {
        id = inst[1];
      }
    }
    if (!id) {
      return;
    }
    const e = scene.ext.get(id);
    if (e?.path) {
      out.push({ line: i, label: `← ${e.path}` });
    }
  });
  return out;
}

/** 文件系统路径 → res:// 路径(相对 workspace folder;win 反斜杠归一)。 */
export function toResPath(folderFsPath: string, fileFsPath: string): string | undefined {
  const norm = (p: string) => p.replace(/\\/g, "/").replace(/\/+$/, "");
  const f = norm(folderFsPath);
  const p = norm(fileFsPath);
  if (!p.startsWith(`${f}/`)) {
    return undefined;
  }
  return `res://${p.slice(f.length + 1)}`;
}

/** 场景内是否有节点(script 绑定或 instance)的 ext 引用指向 scriptRes。 */
export function sceneBindsScript(sceneText: string, scriptRes: string): boolean {
  const scene = parseScene(sceneText);
  const visit = (n: { instanceId?: string; bindings: Map<string, string>; children: unknown[] }): boolean => {
    if (n.instanceId && scene.ext.get(n.instanceId)?.path === scriptRes) {
      return true;
    }
    const sid = n.bindings.get("script");
    if (sid && scene.ext.get(sid)?.path === scriptRes) {
      return true;
    }
    return (n.children as Parameters<typeof visit>[0][]).some(visit);
  };
  return scene.roots.some(visit);
}

/** 场景根节点(首个无 parent 的 node)绑定的脚本 res:// 路径;无则 undefined。 */
export function sceneRootScript(sceneText: string): string | undefined {
  const scene = parseScene(sceneText);
  const sid = scene.roots[0]?.bindings.get("script");
  return sid ? scene.ext.get(sid)?.path : undefined;
}
