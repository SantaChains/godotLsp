/**
 * ConfigFile 家族的补全 / hover Provider(godot-project 语言:
 * project.godot、override.cfg、.godot/*.cfg、.gdextension、.import)。
 *
 * 纯解析逻辑在 configFile.ts(node:test 可测);本文件只做 vscode 适配:
 * 文档类型分派、CompletionItem/Hover 组装、插入文本的引号处理。
 *
 * 范围说明:.tscn/.tres(godot-resource)的节名与节点属性键不在本 Provider——
 * 属性键全集依赖引擎类数据库(如 script = ExtResource("1_p") 的 script 键
 * 来自节点类成员),属于另一条数据链路,此处不做半成品。
 */

import * as vscode from "vscode";
import {
  buildSectionKeyHover,
  buildSettingHover,
  contextAt,
  enumValues,
  lookupSetting,
  parseConfigFile,
  projectSections,
  settingKeysInSection,
  settings,
  type ConfigContext,
  type ConfigFile,
  type SettingEntry,
} from "./configFile";

type DocKind = "project" | "gdextension" | "import";

function docKind(uri: vscode.Uri): DocKind {
  const p = uri.fsPath.toLowerCase();
  if (p.endsWith(".gdextension")) {
    return "gdextension";
  }
  if (p.endsWith(".import")) {
    return "import";
  }
  // project.godot / override.cfg / .godot/*.cfg 等 ConfigFile 均按设置键处理。
  return "project";
}

/** [libraries]/[dependencies] 的常用特性标签(点分,经 OS::has_feature 开放匹配;
 *  词表为社区约定而非引擎枚举——find_extension_library 对任意 tag 都接受)。 */
const FEATURE_TAGS = [
  "windows.x86_64",
  "windows.arm64",
  "linux.x86_64",
  "linux.arm64",
  "linux.rv64",
  "macos",
  "android.arm64",
  "android.arm32",
  "ios",
  "web.wasm32",
  "debug",
  "release",
];

/** 读 .import 文档 [remap] 节声明的 importer 名(决定 [params] 可用键)。 */
function declaredImporter(cf: ConfigFile): string | undefined {
  const e = cf.entries.find((x) => x.section === "remap" && x.key === "importer");
  if (!e) {
    return undefined;
  }
  const v = e.value.trim();
  if (v.length >= 2 && v.startsWith('"') && v.endsWith('"')) {
    return v.slice(1, -1);
  }
  return v;
}

/** 字符串值的插入策略:已在引号内插裸值,空值/裸值插带引号值。 */
function quoteInsert(raw: string, value: string): string {
  return raw.trimStart().startsWith('"') ? value : `"${value}"`;
}

function keyItem(
  label: string,
  detail: string,
  doc: string | undefined,
  sortPrefix = "1",
): vscode.CompletionItem {
  const item = new vscode.CompletionItem(label, vscode.CompletionItemKind.Field);
  item.detail = detail;
  item.sortText = sortPrefix + label;
  if (doc) {
    item.documentation = new vscode.MarkdownString(doc);
  }
  return item;
}

function entryFlags(entry: SettingEntry): string {
  const flags: string[] = [];
  if (entry.basic) {
    flags.push("基础");
  }
  if (entry.restart) {
    flags.push("需重启");
  }
  if (entry.initial) {
    flags.push("新建默认");
  }
  return flags.join("/");
}

function fullSettingKey(section: string, key: string): string {
  if (section && !key.startsWith(`${section}/`)) {
    return `${section}/${key}`;
  }
  return section ? `${section}/${key}` : key;
}

// ---------------------------------------------------------------------------
// 补全
// ---------------------------------------------------------------------------

export class ConfigCompletionProvider implements vscode.CompletionItemProvider {
  provideCompletionItems(
    document: vscode.TextDocument,
    position: vscode.Position,
  ): vscode.CompletionItem[] | undefined {
    const kind = docKind(document.uri);
    const cf = parseConfigFile(document.getText());
    const ctx = contextAt(cf, position, document.lineAt(position.line).text);
    switch (ctx.kind) {
      case "section-name":
        return this.sections(kind, ctx.section);
      case "key":
        return this.keys(kind, cf, ctx);
      case "value":
        return this.values(kind, cf, ctx);
      default:
        return undefined;
    }
  }

  private sections(kind: DocKind, current: string): vscode.CompletionItem[] | undefined {
    if (kind === "gdextension") {
      return Object.entries(settings.gdextension).map(([name, doc]) => {
        const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Module);
        item.detail = doc.doc;
        item.sortText = name === current ? `0${name}` : `1${name}`;
        return item;
      });
    }
    if (kind === "import") {
      return Object.entries(settings.import).map(([name, doc]) => {
        const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Module);
        item.detail = doc.doc;
        item.sortText = name === current ? `0${name}` : `1${name}`;
        return item;
      });
    }
    const count = new Map<string, number>();
    for (const key of Object.keys(settings.keys)) {
      const head = key.split("/")[0];
      count.set(head, (count.get(head) ?? 0) + 1);
    }
    return projectSections().map((name) => {
      const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Module);
      const n = count.get(name) ?? 0;
      item.detail = n > 0 ? `${n} 项设置(Godot ${settings.version})` : "特殊节(非 GLOBAL_DEF 注册)";
      item.sortText = name === current ? `0${name}` : `1${name}`;
      return item;
    });
  }

  private keys(
    kind: DocKind,
    cf: ConfigFile,
    ctx: Extract<ConfigContext, { kind: "key" }>,
  ): vscode.CompletionItem[] | undefined {
    if (kind === "gdextension") {
      const doc = settings.gdextension[ctx.section];
      if (!doc) {
        return undefined;
      }
      const items = Object.entries(doc.keys).map(([key, kd]) => {
        const item = new vscode.CompletionItem(key, vscode.CompletionItemKind.Field);
        item.detail = kd.required ? "必需" : "可选";
        item.documentation = new vscode.MarkdownString(kd.doc);
        item.sortText = kd.required ? `0${key}` : `1${key}`;
        return item;
      });
      // [libraries]/[dependencies] 的键是特性标签,补常用约定词表。
      if (ctx.section === "libraries" || ctx.section === "dependencies") {
        for (const tag of FEATURE_TAGS) {
          const item = new vscode.CompletionItem(tag, vscode.CompletionItemKind.Constant);
          item.detail = "特性标签(常用约定)";
          item.documentation = new vscode.MarkdownString(
            "点分特性标签,按 `OS::has_feature` 匹配;全部命中且标签最多者胜出。",
          );
          item.sortText = `2${tag}`;
          items.push(item);
        }
      }
      return items;
    }
    if (kind === "import") {
      const doc = settings.import[ctx.section];
      if (!doc) {
        return undefined;
      }
      const items = Object.entries(doc.keys).map(([key, kd]) =>
        keyItem(key, kd.required ? "必需" : "可选", kd.doc, kd.required ? "0" : "1"),
      );
      // [params] 的键由本文件 [remap] 的 importer= 决定。
      if (ctx.section === "params") {
        const importer = declaredImporter(cf);
        const params = importer ? settings.import_params[importer] : undefined;
        if (params) {
          for (const [key, entry] of Object.entries(params)) {
            const detail = [entry.type, entry.hint !== "none" ? entry.hint : undefined]
              .filter(Boolean)
              .join(" · ");
            items.push(keyItem(key, detail || "参数", buildSettingHover(`${importer}:${key}`, entry)));
          }
        }
      }
      return items;
    }
    // project.godot:节内设置键(去掉节前缀的文档内形式)。
    return settingKeysInSection(ctx.section).map(({ key, entry }) => {
      const detail = [entry.type, entryFlags(entry)].filter(Boolean).join(" · ");
      return keyItem(
        key,
        detail || "设置",
        buildSettingHover(fullSettingKey(ctx.section, key), entry),
        entry.basic ? "0" : "1",
      );
    });
  }

  private values(
    kind: DocKind,
    cf: ConfigFile,
    ctx: Extract<ConfigContext, { kind: "value" }>,
  ): vscode.CompletionItem[] | undefined {
    const entry = ctx.entry;
    const raw = entry.value;
    if (kind === "import") {
      if (ctx.section === "remap" && entry.key === "importer") {
        return Object.keys(settings.import_params).map((name) => {
          const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Constant);
          item.detail = `导入器 → ${settings.import_types[name] ?? "?"}`;
          item.sortText = `0${name}`;
          return item;
        });
      }
      if (ctx.section === "remap" && entry.key === "type") {
        const importer = declaredImporter(cf);
        const type = importer ? settings.import_types[importer] : undefined;
        if (!type) {
          return undefined;
        }
        const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.Constant);
        item.detail = `${importer} 的导入产物类型`;
        return [item];
      }
      return undefined;
    }
    if (kind === "gdextension") {
      if (ctx.section === "configuration" && entry.key === "reloadable") {
        return ["true", "false"].map((v) => boolItem(v));
      }
      return undefined;
    }
    // project.godot:按数据集的 type/hint 提供值补全。
    const info = lookupSetting(ctx.section, entry.key);
    if (!info) {
      return undefined;
    }
    if (info.type === "bool") {
      return ["true", "false"].map((v) => boolItem(v));
    }
    if (info.hint === "enum" || info.hint === "enum_suggestion") {
      const values = enumValues(info.hintString);
      if (values.length === 0) {
        return undefined;
      }
      return values.map((v) => {
        const item = new vscode.CompletionItem(v, vscode.CompletionItemKind.EnumMember);
        item.insertText = quoteInsert(raw, v);
        item.sortText = `0${v}`;
        return item;
      });
    }
    return undefined;
  }
}

function boolItem(v: string): vscode.CompletionItem {
  const item = new vscode.CompletionItem(v, vscode.CompletionItemKind.Constant);
  item.sortText = `0${v}`;
  return item;
}

// ---------------------------------------------------------------------------
// Hover
// ---------------------------------------------------------------------------

export class ConfigHoverProvider implements vscode.HoverProvider {
  provideHover(
    document: vscode.TextDocument,
    position: vscode.Position,
  ): vscode.Hover | undefined {
    const kind = docKind(document.uri);
    const cf = parseConfigFile(document.getText());
    const ctx = contextAt(cf, position, document.lineAt(position.line).text);

    if (ctx.kind === "section-name") {
      return this.sectionHover(kind, ctx.section);
    }
    if (ctx.kind === "key" || ctx.kind === "value") {
      const entry = ctx.entry;
      if (!entry) {
        return undefined;
      }
      if (kind === "project") {
        const full = fullSettingKey(ctx.section, entry.key);
        const info = lookupSetting(ctx.section, entry.key);
        if (!info) {
          return undefined;
        }
        return new vscode.Hover(new vscode.MarkdownString(buildSettingHover(full, info)));
      }
      const md = buildSectionKeyHover(kind, ctx.section, entry.key);
      return md ? new vscode.Hover(new vscode.MarkdownString(md)) : undefined;
    }
    return undefined;
  }

  private sectionHover(kind: DocKind, section: string): vscode.Hover | undefined {
    if (kind !== "project") {
      const doc = (kind === "gdextension" ? settings.gdextension : settings.import)[section];
      if (!doc) {
        return undefined;
      }
      const keys = Object.keys(doc.keys);
      const list = keys.map((k) => `- \`${k}\`${doc.keys[k].required ? "(必需)" : ""}`).join("\n");
      return new vscode.Hover(
        new vscode.MarkdownString(`**[${section}]**\n\n${doc.doc}\n\n${list}`),
      );
    }
    const items = settingKeysInSection(section);
    if (items.length === 0) {
      return undefined;
    }
    return new vscode.Hover(
      new vscode.MarkdownString(
        `**[${section}]** — ${items.length} 项设置(Godot ${settings.version} 源码提取)`,
      ),
    );
  }
}
