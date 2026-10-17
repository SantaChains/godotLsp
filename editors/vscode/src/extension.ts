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
import { SymbolTreeProvider } from "./symbols";
import { ProjectTreeProvider, PTreeDataProvider } from "./projectTree";
import { SceneTreeProvider } from "./sceneTree";
import { ConnectionTreeProvider } from "./connections";
import { ConfigCompletionProvider, ConfigHoverProvider } from "./configAssist";
import { highlightClampMiddleware } from "./highlight";
import { isRunning } from "./lsp";
import { copyInfoFrom } from "./copyable";

const SECTION = "godotlsp";

let client: LanguageClient | undefined;
let output: vscode.LogOutputChannel;
let status: vscode.StatusBarItem;
let symbols: SymbolTreeProvider | undefined;
let projectTree: ProjectTreeProvider | undefined;
let sceneTree: SceneTreeProvider | undefined;
let connTree: ConnectionTreeProvider | undefined;

/**
 * 注册侧边栏视图。
 *
 * `createTreeView` 抛 `No view is registered with id` 时,view id 与 manifest
 * 是一致的(仓库内三方核对),唯一成因是**运行窗口的视图注册表来自旧版
 * manifest 缓存**——典型场景:安装/升级 vsix 后没有重载窗口,或同版本号
 * 重打包覆盖安装(VS Code 按版本号缓存 contributions)。捕获后给出可操作
 * 指引,再原样上抛保留错误面。
 */
function createView<T>(
  id: string,
  provider: vscode.TreeDataProvider<T>,
): vscode.TreeView<T> {
  try {
    return vscode.window.createTreeView(id, {
      treeDataProvider: provider,
      showCollapseAll: true,
    });
  } catch (err) {
    output.error(
      `侧边栏视图 ${id} 注册失败:${err instanceof Error ? err.message : String(err)}。` +
        `若信息为 "No view is registered with id",请执行 "Developer: Reload Window" 重载窗口` +
        `(旧版视图注册表缓存未刷新;若仍失败,先卸载旧版再安装后重载)。`,
    );
    throw err;
  }
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  output = vscode.window.createOutputChannel("godotlsp", { log: true });
  status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
  status.command = "godotlsp.showOutput";
  context.subscriptions.push(output, status);

  // 侧边栏四视图:当前文件符号 / 项目结构(autoload + class_name)/ 场景结构(tscn 节点树)/ 信号连接(tscn [connection])。
  symbols = new SymbolTreeProvider(() => client, output);
  projectTree = new ProjectTreeProvider(() => client, output);
  sceneTree = new SceneTreeProvider();
  connTree = new ConnectionTreeProvider();
  const tree = createView("godotlsp.symbols", symbols);
  const projectProvider = new PTreeDataProvider(() => projectTree!.roots());
  const sceneProvider = new PTreeDataProvider(() => sceneTree!.roots());
  const connProvider = new PTreeDataProvider(() => connTree!.roots());
  const projectView = createView("godotlsp.project", projectProvider);
  const sceneView = createView("godotlsp.scene", sceneProvider);
  const connView = createView("godotlsp.connections", connProvider);
  const isGd = (d: vscode.TextDocument) => d.languageId === "gdscript";
  const isRes = (d: vscode.TextDocument) => d.languageId === "godot-resource";
  // 场景结构与信号连接同为 tscn 派生视图,失效与刷新条件完全一致。
  const refreshSceneDerived = () => {
    sceneTree?.invalidate();
    sceneProvider.refresh();
    connTree?.invalidate();
    connProvider.refresh();
  };

  // ── 21b 视图过滤:四视图共用一个输入框流程,过滤词显示在视图描述行。 ──
  const describeFilter = (f: string) => (f ? `筛选: ${f}` : undefined);
  const filterTargets: Record<
    string,
    { apply: (f: string) => void; current: () => string; view: vscode.TreeView<unknown> }
  > = {
    "godotlsp.symbols": {
      apply: (f) => {
        symbols?.setFilter(f);
        tree.description = describeFilter(f);
      },
      current: () => symbols?.getFilter() ?? "",
      view: tree,
    },
    "godotlsp.project": {
      apply: (f) => {
        projectProvider.setFilter(f);
        projectView.description = describeFilter(f);
      },
      current: () => projectProvider.getFilter(),
      view: projectView,
    },
    "godotlsp.scene": {
      apply: (f) => {
        sceneProvider.setFilter(f);
        sceneView.description = describeFilter(f);
      },
      current: () => sceneProvider.getFilter(),
      view: sceneView,
    },
    "godotlsp.connections": {
      apply: (f) => {
        connProvider.setFilter(f);
        connView.description = describeFilter(f);
      },
      current: () => connProvider.getFilter(),
      view: connView,
    },
  };
  const registerFilter = (viewId: string) =>
    vscode.commands.registerCommand(`${viewId}.filter`, async () => {
      const target = filterTargets[viewId];
      const input = await vscode.window.showInputBox({
        value: target.current(),
        prompt: `${target.current() ? "修改" : "输入"}过滤词(名称子串匹配,保留命中后代;留空清除)`,
        placeHolder: "如:hp、Player、Button…",
      });
      if (input !== undefined) {
        target.apply(input.trim());
      }
    });
  context.subscriptions.push(
    registerFilter("godotlsp.symbols"),
    registerFilter("godotlsp.project"),
    registerFilter("godotlsp.scene"),
    registerFilter("godotlsp.connections"),
  );

  // ── 21c 光标跟随:选择变化防抖后 reveal 对应树节点(仅视图可见时)。 ──
  let followTimer: ReturnType<typeof setTimeout> | undefined;
  const followCursor = (editor: vscode.TextEditor) => {
    const line = editor.selection.active.line;
    if (isGd(editor.document)) {
      void followIn(tree, symbols?.locate(line));
    } else if (/\.(tscn|scn)$/i.test(editor.document.fileName)) {
      void followIn(sceneView, sceneTree?.locate(line));
      void followIn(connView, connTree?.locate(line));
    }
  };
  context.subscriptions.push(
    vscode.window.onDidChangeTextEditorSelection((e) => {
      if (e.textEditor.document.uri.scheme !== "file") {
        return;
      }
      if (followTimer) {
        clearTimeout(followTimer);
      }
      followTimer = setTimeout(() => {
        followTimer = undefined;
        followCursor(e.textEditor);
      }, 250);
    }),
  );
  // 21d:诊断到达/更新时刷新带旗标的视图(仅改 description,成本低)。
  context.subscriptions.push(
    vscode.languages.onDidChangeDiagnostics(() => {
      symbols?.refresh();
      sceneProvider.refresh();
      connProvider.refresh();
    }),
  );

  context.subscriptions.push(
    tree,
    projectView,
    sceneView,
    connView,
    // ConfigFile 家族(project.godot/.gdextension/.import/.cfg)的键/值补全与
    // hover:数据集来自 Godot 源码提取,与语言服务器相互独立。
    vscode.languages.registerCompletionItemProvider(
      "godot-project",
      new ConfigCompletionProvider(),
      "[",
      "/",
      ".",
      "=",
      '"',
      ",",
    ),
    vscode.languages.registerHoverProvider("godot-project", new ConfigHoverProvider()),
    vscode.commands.registerCommand("godotlsp.symbols.refresh", () => symbols?.refresh()),
    vscode.commands.registerCommand("godotlsp.project.refresh", () => {
      projectTree?.invalidateClasses();
      projectProvider.refresh();
    }),
    vscode.commands.registerCommand("godotlsp.scene.refresh", () => {
      sceneTree?.invalidate();
      sceneProvider.refresh();
    }),
    vscode.commands.registerCommand("godotlsp.connections.refresh", () => {
      connTree?.invalidate();
      connProvider.refresh();
    }),
    vscode.commands.registerCommand("godotlsp.goto", gotoSymbol),
    // P2.10 codeLens:跳到 tscn [connection] 行（服务器 lens 命令载荷 [uri, line]）。
    vscode.commands.registerCommand(
      "godotlsp.openConnection",
      async (uri?: string, line?: number) => {
        if (typeof uri !== "string" || typeof line !== "number") {
          return;
        }
        const doc = await vscode.workspace.openTextDocument(vscode.Uri.parse(uri));
        const editor = await vscode.window.showTextDocument(doc, { preview: true });
        const pos = new vscode.Position(line, 0);
        editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
        editor.selection = new vscode.Selection(pos, pos);
      },
    ),
    // 侧边栏条目右键快速复制(三视图共用,载荷解析见 copyable.ts)。
    vscode.commands.registerCommand("godotlsp.tree.copyName", async (node?: unknown) => {
      const info = copyInfoFrom(node);
      const label = (node as vscode.TreeItem | undefined)?.label;
      const text = info?.name ?? (typeof label === "string" ? label : undefined);
      if (text) {
        await vscode.env.clipboard.writeText(text);
      }
    }),
    vscode.commands.registerCommand("godotlsp.tree.copyRef", async (node?: unknown) => {
      const info = copyInfoFrom(node);
      if (info?.ref) {
        await vscode.env.clipboard.writeText(info.ref);
      }
    }),
    // 刷新时机:切换编辑器 / 打开与编辑 GDScript / 客户端状态变化。
    vscode.window.onDidChangeActiveTextEditor((e) => {
      symbols?.refresh();
      if (!e || isRes(e.document)) {
        refreshSceneDerived();
      }
    }),
    vscode.workspace.onDidOpenTextDocument((d) => {
      if (isGd(d)) {
        symbols?.refresh();
      }
      if (isRes(d)) {
        refreshSceneDerived();
      }
    }),
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (isGd(e.document)) {
        symbols?.refresh();
      }
      if (isRes(e.document)) {
        refreshSceneDerived();
      }
    }),
    // class_name 注册表只在保存时重建,避免每次击键全工程扫描。
    vscode.workspace.onDidSaveTextDocument((d) => {
      if (isGd(d)) {
        projectTree?.invalidateClasses();
      }
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("godotlsp.restart", () => restart()),
    vscode.commands.registerCommand("godotlsp.showOutput", () => output.show(true)),
    vscode.commands.registerCommand("godotlsp.runProject", () => runGodot([])),
    vscode.commands.registerCommand("godotlsp.runScene", () => runCurrentScene()),
    // 轻量配置(严格诊断/内联提示)由语言客户端经 didChangeConfiguration 推送,
    // 只有服务器二进制路径变化才值得重启进程。
    vscode.workspace.onDidChangeConfiguration(
      (e) =>
        e.affectsConfiguration(`${SECTION}.path`) &&
        vscode.workspace.isTrusted &&
        void restart(),
    ),
    // quote 三键即时推送,无需重启进程(服务器逐键解析,缺键兜底默认值)。
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (affectsQuotePrefs(e) && isRunning(client)) {
        pushQuotePrefs(client);
      }
    }),
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
  const resolved = resolveServer();
  if (!resolved) {
    setStatus("error", "未找到 godotlsp。");
    output.error("未找到服务器二进制:配置(godotlsp.path)/打包(server/)/PATH 均未命中。");
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
  // 结构化启动日志(pyrefly 模式):一行说明「选了哪个二进制、为什么」,
  // 启动异常时这是最快的定位入口。
  output.info(`启动服务器 [mode=${resolved.source}] ${resolved.command}`);

  const serverOptions: ServerOptions = {
    command: resolved.command,
    transport: TransportKind.stdio,
    options: { cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath },
  };
  const clientOptions: LanguageClientOptions = {
    // 支持 file 与 untitled（未保存的 GDScript 草稿也能获得补全/悬停）。
    documentSelector: [
      { scheme: "file", language: "gdscript" },
      { scheme: "untitled", language: "gdscript" },
    ],
    outputChannel: output,
    // documentHighlight 区间词级矫正(旧内核起点锚行首,详见 highlight.ts)。
    middleware: highlightClampMiddleware,
    // 服务器进程意外退出时自动重启（有限次，防崩溃循环）。
    connectionOptions: { maxRestartCount: 4 },
    synchronize: {
      // 服务器无需逐个打开即可感知项目文件的增删改,并在配置变化时收到推送。
      fileEvents: vscode.workspace.createFileSystemWatcher(
        "**/{*.gd,*.tscn,*.tres,project.godot}",
      ),
      configurationSection: SECTION,
    },
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
    // 客户端状态变化影响侧边栏数据可用性,同步刷新。
    symbols?.refresh();
  });

  try {
    await client.start();
    // initialize 握手完成后补发 quote 三键(languageclient 自动推送只含 godotlsp 段)。
    pushQuotePrefs(client);
  } catch (err) {
    setStatus("error", `godotlsp 启动失败:${String(err)}`);
    output.error(`启动 godotlsp 失败:${String(err)}`);
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
      output.error(`停止 godotlsp 时出错:${String(err)}`);
    }
  }
}

async function restart(): Promise<void> {
  await stop();
  await start();
}

/**
 * quote 三键 → `settings.gdscript` 段推送(服务器侧引擎 `text_editor/completion`
 * 同名语义,控制补全插入文本的引号与前缀行为)。
 * vscode-languageclient 的 synchronize 只推 `godotlsp` 段;此段手动合成——
 * 刻意不贡献全局 `gdscript.*` 配置命名空间,避免与官方 godot-tools 等插件冲突。
 */
function pushQuotePrefs(c: LanguageClient): void {
  const cfg = vscode.workspace.getConfiguration(`${SECTION}.completion`);
  c.sendNotification("workspace/didChangeConfiguration", {
    settings: {
      gdscript: {
        use_single_quotes: cfg.get<boolean>("useSingleQuotes", false),
        add_string_name_literals: cfg.get<boolean>("addStringNameLiterals", false),
        add_node_path_literals: cfg.get<boolean>("addNodePathLiterals", false),
      },
    },
  });
}

/** quote 三键是否有变动(配置变化监听用)。 */
function affectsQuotePrefs(e: vscode.ConfigurationChangeEvent): boolean {
  return (
    e.affectsConfiguration(`${SECTION}.completion.useSingleQuotes`) ||
    e.affectsConfiguration(`${SECTION}.completion.addStringNameLiterals`) ||
    e.affectsConfiguration(`${SECTION}.completion.addNodePathLiterals`)
  );
}

/** 服务器二进制解析:配置 → 扩展自带 → PATH;source 说明命中来源(结构化日志用)。 */
function resolveServer(): { command: string; source: "configured" | "bundled" | "path" } | undefined {
  const configured = vscode.workspace.getConfiguration(SECTION).get<string | null>("path");
  if (typeof configured === "string" && configured.trim() !== "") {
    return { command: expand(configured.trim()), source: "configured" };
  }
  const exe = process.platform === "win32" ? "godotlsp.exe" : "godotlsp";
  const bundled = path.join(__dirname, "..", "server", exe);
  if (fs.existsSync(bundled)) {
    return { command: bundled, source: "bundled" };
  }
  const found = onPath(exe);
  return found ? { command: found, source: "path" } : undefined;
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
  status.tooltip = `${tooltip}\n点击查看 godotlsp 输出。`;
  status.backgroundColor =
    state === "error" ? new vscode.ThemeColor("statusBarItem.errorBackground") : undefined;
  status.show();
}

/** Godot 可执行文件解析:`godotlsp.godotExecutable` 配置,否则 PATH 上的 `godot`。 */
function resolveGodot(): string | undefined {
  const configured = vscode.workspace.getConfiguration(SECTION).get<string | null>("godotExecutable");
  if (typeof configured === "string" && configured.trim() !== "") {
    return expand(configured.trim());
  }
  const exe = process.platform === "win32" ? "godot.exe" : "godot";
  return onPath(exe);
}

/**
 * 终端命令行引述:POSIX 单引号(最稳);Windows 校验拒绝 shell 元字符——
 * cmd/PowerShell 转义规则分歧大,与其造不完美的转义轮子,不如对含元字符的
 * 配置路径明确报错(CWE-77:sendText 拼接即注入面,宁紧勿松)。
 */
function quoteWin(value: string): string {
  if (/["`$|&;()<>!\n]/.test(value)) {
    throw new Error(`路径含 shell 元字符,拒绝执行:${value}`);
  }
  return /\s/.test(value) ? `"${value}"` : value;
}

function quotePosix(value: string): string {
  return `'${value.replaceAll("'", "'\\''")}'`;
}

/** 在集成终端里以项目根为 cwd 运行 Godot(调试/运行属 DAP 域之外的最简入口)。 */
function runGodot(args: string[]): void {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    void vscode.window.showErrorMessage("godotlsp:没有打开的工作区,无法运行 Godot 项目。");
    return;
  }
  const godot = resolveGodot();
  if (!godot) {
    void vscode.window
      .showErrorMessage("godotlsp:未找到 Godot 可执行文件。请设置 `godotlsp.godotExecutable` 或把 `godot` 放入 PATH。", "设置路径")
      .then((pick) => {
        if (pick === "设置路径") {
          void vscode.commands.executeCommand("workbench.action.openSettings", "godotlsp.godotExecutable");
        }
      });
    return;
  }
  const win = process.platform === "win32";
  const quote = win ? quoteWin : quotePosix;
  let command: string;
  try {
    // 场景路径派生自工作区文件名,同样过元字符校验(非受信输入原则)。
    command = [quote(godot), "--path", quote(folder.uri.fsPath), ...args.map(quote)].join(" ");
  } catch (err) {
    output.error(String(err));
    void vscode.window.showErrorMessage(`godotlsp:${String(err)}`);
    return;
  }
  const terminal = vscode.window.createTerminal({ name: "Godot", cwd: folder.uri.fsPath });
  terminal.sendText(command);
  terminal.show(true);
}

/** 运行当前编辑器打开的场景:`godot --path . <场景文件>`。 */
async function runCurrentScene(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  const file = editor?.document.uri.fsPath;
  if (!file || !/\.(tscn|scn)$/i.test(file)) {
    void vscode.window.showErrorMessage("godotlsp:请先打开一个场景文件(.tscn)再运行场景。");
    return;
  }
  runGodot([file]);
}

/**
 * 21c 光标跟随的 reveal:仅视图可见时尝试;过滤中被隐藏的节点 reveal
 * 会拒绝——跟随是尽力而为,静默吞掉(不打断输入流,不刷错误)。
 */
async function followIn(view: vscode.TreeView<unknown>, node: unknown | Promise<unknown>): Promise<void> {
  if (!view.visible || node === undefined || node === null) {
    return;
  }
  try {
    const target = await node;
    if (target) {
      await view.reveal(target, { select: true, focus: false, expand: true });
    }
  } catch {
    // 尽力而为:元素未渲染(过滤/折叠竞态)时放弃本次跟随。
  }
}

/**
 * 树节点点击 → 打开文档并定位。selectionRange 来自服务器内核,列可能偏移
 * (锚到行首缩进/UTF-8 字节列),因此选区做词级吸附:优先在目标行内找
 * 词条文本最近的出现处,退化为光标处的词,最后退化为零宽光标。
 */
async function gotoSymbol(
  uri: string | undefined,
  line: number | undefined,
  character = 0,
  word?: string,
): Promise<void> {
  if (!uri || line === undefined || line < 0) {
    return;
  }
  const doc = await vscode.workspace.openTextDocument(vscode.Uri.parse(uri));
  const editor = await vscode.window.showTextDocument(doc, { preserveFocus: false, preview: false });
  const clampedLine = Math.min(line, doc.lineCount - 1);
  const lineText = doc.lineAt(clampedLine).text;
  let chars = word ? nearestWordRange(lineText, word, character) : undefined;
  if (!chars) {
    const pos = new vscode.Position(clampedLine, Math.min(character, lineText.length));
    const wr = doc.getWordRangeAtPosition(pos);
    chars = wr
      ? { start: wr.start.character, end: wr.end.character }
      : { start: pos.character, end: pos.character };
  }
  const shifted = new vscode.Range(
    new vscode.Position(clampedLine, chars.start),
    new vscode.Position(clampedLine, chars.end),
  );
  editor.selection = new vscode.Selection(shifted.start, shifted.end);
  editor.revealRange(
    shifted,
    vscode.TextEditorRevealType.InCenterIfOutsideViewport,
  );
}

/** 在行文本中找 word 的出现位置,取离 expectCol 最近的;找不到返回 undefined。 */
function nearestWordRange(
  lineText: string,
  word: string,
  expectCol: number,
): { start: number; end: number } | undefined {
  if (!word) {
    return undefined;
  }
  let best: { start: number; end: number } | undefined;
  let bestDist = Number.MAX_SAFE_INTEGER;
  let from = 0;
  for (;;) {
    const idx = lineText.indexOf(word, from);
    if (idx === -1) {
      break;
    }
    // 词边界校验:两侧不得是标识符字符,避免选中子串(如 goto 命中 godotlsp_goto)。
    const before = idx > 0 ? lineText[idx - 1] : "";
    const after = idx + word.length < lineText.length ? lineText[idx + word.length] : "";
    const ident = /[A-Za-z0-9_]/;
    if (!ident.test(before) && !ident.test(after)) {
      const dist = Math.abs(idx - expectCol);
      if (dist < bestDist) {
        bestDist = dist;
        best = { start: idx, end: idx + word.length };
      }
    }
    from = idx + 1;
  }
  return best;
}
