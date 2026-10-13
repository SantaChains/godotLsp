import * as fs from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  State,
  TransportKind,
} from "vscode-languageclient/node";

const SECTION = "godotlsp";

let client: LanguageClient | undefined;
let output: vscode.LogOutputChannel;
let status: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  output = vscode.window.createOutputChannel("godotlsp", { log: true });
  status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
  status.command = "godotlsp.showOutput";
  context.subscriptions.push(output, status);

  context.subscriptions.push(
    vscode.commands.registerCommand("godotlsp.restart", () => restart()),
    vscode.commands.registerCommand("godotlsp.showOutput", () => output.show(true)),
    vscode.workspace.onDidChangeConfiguration(
      (e) => e.affectsConfiguration(SECTION) && vscode.workspace.isTrusted && void restart(),
    ),
  );

  if (vscode.workspace.isTrusted) {
    await start();
  } else {
    setStatus("stopped", "godotlsp 等待此工作区被信任。");
    context.subscriptions.push(vscode.workspace.onDidGrantWorkspaceTrust(() => start()));
  }
}

export async function deactivate(): Promise<void> {
  await stop();
}

async function start(): Promise<void> {
  if (client) {
    return;
  }
  const command = resolveServer();
  if (!command) {
    setStatus("error", "未找到 godotlsp。");
    void vscode.window
      .showErrorMessage(
        "godotlsp:未找到服务器二进制。随扩展打包(server/)、将 `godotlsp.path` 指向它,或把 `godotlsp` 放入 PATH。",
        "设置路径",
      )
      .then((pick) => {
        if (pick === "设置路径") {
          void vscode.commands.executeCommand("workbench.action.openSettings", "godotlsp.path");
        }
      });
    return;
  }

  output.appendLine(`正在启动 ${command}`);
  const serverOptions: ServerOptions = {
    command,
    transport: TransportKind.stdio,
    options: { cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath },
  };
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "gdscript" }],
    outputChannel: output,
  };

  client = new LanguageClient(SECTION, "godotlsp", serverOptions, clientOptions);
  client.onDidChangeState((e) => {
    if (e.newState === State.Running) {
      const info = client?.initializeResult?.serverInfo;
      setStatus("running", `${info?.name ?? "godotlsp"} ${info?.version ?? ""}`.trim());
    } else if (e.newState === State.Starting) {
      setStatus("starting", "godotlsp 正在启动。");
    } else {
      setStatus("stopped", "godotlsp 未在运行。");
    }
  });

  try {
    await client.start();
  } catch (err) {
    setStatus("error", `godotlsp 启动失败:${String(err)}`);
    output.appendLine(`启动 godotlsp 失败:${String(err)}`);
    client = undefined;
  }
}

async function stop(): Promise<void> {
  const c = client;
  client = undefined;
  if (c) {
    try {
      await c.stop();
    } catch (err) {
      output.appendLine(`停止 godotlsp 时出错:${String(err)}`);
    }
  }
}

async function restart(): Promise<void> {
  await stop();
  await start();
}

/** `godotlsp.path` 配置,否则扩展自带的 `server/godotlsp`,否则 PATH 上的 `godotlsp`。 */
function resolveServer(): string | undefined {
  const configured = vscode.workspace.getConfiguration(SECTION).get<string | null>("path");
  if (typeof configured === "string" && configured.trim() !== "") {
    return expand(configured.trim());
  }
  const exe = process.platform === "win32" ? "godotlsp.exe" : "godotlsp";
  const bundled = path.join(__dirname, "..", "server", exe);
  if (fs.existsSync(bundled)) {
    return bundled;
  }
  return onPath(exe);
}

/** 展开路径配置中的 `${workspaceFolder}` 与前导 `~`。 */
function expand(value: string): string {
  const folder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? "";
  let v = value.replace(/\$\{workspaceFolder\}/g, folder);
  if (v === "~" || v.startsWith("~/") || v.startsWith("~\\")) {
    v = path.join(process.env.HOME ?? process.env.USERPROFILE ?? "", v.slice(1));
  }
  return v;
}

function onPath(exe: string): string | undefined {
  for (const dir of (process.env.PATH ?? "").split(path.delimiter)) {
    if (dir === "") {
      continue;
    }
    const candidate = path.join(dir, exe);
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }
  return undefined;
}

function setStatus(state: "starting" | "running" | "stopped" | "error", tooltip: string): void {
  const icon = { starting: "$(sync~spin)", running: "$(check)", stopped: "$(circle-slash)", error: "$(error)" }[state];
  status.text = `${icon} godotlsp`;
  status.tooltip = `${tooltip}\nClick to show the godotlsp output.`;
  status.backgroundColor =
    state === "error" ? new vscode.ThemeColor("statusBarItem.errorBackground") : undefined;
  status.show();
}
