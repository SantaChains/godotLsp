/**
 * A6(路线图 §3.5.1):脚本↔场景互跳(官方 godot-tools Alt+O 同款)。
 * .gd → 扫 workspace *.tscn 找 script 绑定反向索引(多命中 quick pick);
 * .tscn → 打开根节点绑定的脚本。
 * 纯逻辑(toResPath/sceneBindsScript/sceneRootScript)在 sceneLogic.ts。
 */

import * as vscode from "vscode";
import { sceneBindsScript, sceneRootScript, toResPath } from "./sceneLogic";
import { resToUri } from "./sceneTree";

const TSCN_GLOB = "{**/*.tscn,**/*.scn}";
const EXCLUDE = "**/{.godot,.git}/**";

export function registerSceneSwitch(): vscode.Disposable[] {
  const cmd = vscode.commands.registerCommand("godotlsp.switchSource", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      return;
    }
    const { document } = editor;
    const folder = vscode.workspace.getWorkspaceFolder(document.uri);
    if (!folder) {
      return;
    }
    if (/\.tscn$/i.test(document.fileName)) {
      // .tscn → 根脚本
      const script = sceneRootScript(document.getText());
      if (!script) {
        void vscode.window.showInformationMessage("场景根节点未绑定脚本。");
        return;
      }
      await vscode.commands.executeCommand("godotlsp.goto", (await resToUri(script))?.toString(), 0);
      return;
    }
    // .gd → 反向找引用它的场景
    const scriptRes = toResPath(folder.uri.fsPath, document.uri.fsPath);
    if (!scriptRes) {
      return;
    }
    const files = await vscode.workspace.findFiles(TSCN_GLOB, EXCLUDE);
    const matches: { label: string; uri: vscode.Uri }[] = [];
    for (const f of files) {
      try {
        const text = new TextDecoder().decode(await vscode.workspace.fs.readFile(f));
        if (sceneBindsScript(text, scriptRes)) {
          matches.push({ label: vscode.workspace.asRelativePath(f), uri: f });
        }
      } catch {
        // 不可读文件跳过
      }
    }
    if (!matches.length) {
      void vscode.window.showInformationMessage(`没有场景绑定 ${scriptRes}`);
      return;
    }
    const pick = matches.length === 1 ? matches[0] : await vscode.window.showQuickPick(matches, { placeHolder: "跳转到绑定此脚本的场景" });
    if (pick) {
      await vscode.commands.executeCommand("godotlsp.goto", pick.uri.toString(), 0);
    }
  });
  return [cmd];
}
