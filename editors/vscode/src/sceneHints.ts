/**
 * A5(路线图 §3.5.1):tscn 资源引用的 hover 预览 + 行尾 inlay 可视化。
 * 对标官方 godot-tools「inlay hints visualize External/Sub Resources」;
 * inlay 落在**使用侧**(属性/instance 行尾)——声明行本身已含 type/path,声明侧冗余。
 * 纯逻辑在 sceneLogic.ts(零 vscode 依赖)。
 */

import * as vscode from "vscode";
import { parseScene } from "./parsers";
import { resRefAt, sceneInlays } from "./sceneLogic";

/** ext id → 引用它的节点名列表(A1 bindings + instance 反查,hover 展示)。 */
function referrers(scene: ReturnType<typeof parseScene>, id: string): string[] {
  const acc: string[] = [];
  const visit = (n: { name: string; instanceId?: string; bindings: Map<string, string>; children: unknown[] }): void => {
    if (n.instanceId === id || [...n.bindings.values()].includes(id)) {
      acc.push(n.name);
    }
    for (const c of n.children as Parameters<typeof visit>[0][]) {
      visit(c);
    }
  };
  for (const r of scene.roots) {
    visit(r as never);
  }
  return acc;
}

export class SceneHintsProvider implements vscode.HoverProvider, vscode.InlayHintsProvider {
  provideHover(doc: vscode.TextDocument, pos: vscode.Position): vscode.Hover | undefined {
    const line = doc.lineAt(pos.line).text;
    const ref = resRefAt(line, pos.character);
    if (!ref) {
      return undefined;
    }
    const scene = parseScene(doc.getText());
    const md = new vscode.MarkdownString();
    if (ref.kind === "ext") {
      const e = scene.ext.get(ref.id);
      if (!e) {
        return undefined;
      }
      md.appendMarkdown(`**${e.type || "Resource"}** \`${e.path}\`\n\n`);
      const refs = referrers(scene, ref.id);
      if (refs.length) {
        const shown = refs.slice(0, 5).map((n) => `\`${n}\``).join(" ");
        md.appendMarkdown(`被 ${refs.length} 个节点引用:${shown}${refs.length > 5 ? " …" : ""}`);
      }
    } else {
      const sub = scene.subs.get(ref.id);
      if (!sub) {
        return undefined;
      }
      md.appendMarkdown(`**SubResource** \`${sub}\``);
    }
    return new vscode.Hover(md);
  }

  provideInlayHints(doc: vscode.TextDocument, range: vscode.Range): vscode.InlayHint[] {
    return sceneInlays(doc.getText())
      .filter((h) => h.line >= range.start.line && h.line <= range.end.line)
      .map((h) => {
        const hint = new vscode.InlayHint(doc.lineAt(h.line).range.end, h.label);
        hint.paddingLeft = true;
        return hint;
      });
  }
}
