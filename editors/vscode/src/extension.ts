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
    setStatus("stopped", "godotlsp waits for this workspace to be trusted.");
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
    setStatus("error", "godotlsp was not found.");
    void vscode.window
      .showErrorMessage(
        "godotlsp: no server binary found. Bundle one with the extension (server/), point `godotlsp.path` at it, or put `godotlsp` on PATH.",
        "Set Path",
      )
      .then((pick) => {
        if (pick === "Set Path") {
          void vscode.commands.executeCommand("workbench.action.openSettings", "godotlsp.path");
        }
      });
    return;
  }

  output.appendLine(`Starting ${command}`);
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
      setStatus("starting", "godotlsp is starting.");
    } else {
      setStatus("stopped", "godotlsp is not running.");
    }
  });

  try {
    await client.start();
  } catch (err) {
    setStatus("error", `godotlsp failed to start: ${String(err)}`);
    output.appendLine(`Failed to start godotlsp: ${String(err)}`);
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
      output.appendLine(`Error while stopping godotlsp: ${String(err)}`);
    }
  }
}

async function restart(): Promise<void> {
  await stop();
  await start();
}

/** The `godotlsp.path` setting, else the bundled `server/godotlsp`, else `godotlsp` on PATH. */
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

/** Expands `${workspaceFolder}` and a leading `~` in a path setting. */
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
