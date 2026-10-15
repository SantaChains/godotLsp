/**
 * ConfigFile 家族(.godot / .cfg / .gdextension / .import)的纯解析与查询层。
 * 不依赖 vscode,可被 node:test 直接测试;Provider 胶水见 configAssist.ts。
 *
 * 语法全部核验自 .refs/godot(Godot 4.7.0):
 * - core/config/config_file.cpp:ConfigFile 读写全权委托
 *   VariantParser::parse_tag_assign_eof;
 * - core/io/variant_parser.cpp:节头是 simple tag(`[` 后到第一个未转义 `]`
 *   全为节名,支持 `\]` 转义);键 = 任意非空白字符序列(引号键 `"k"=v` 合法),
 *   止于第一个 `=`;`;` 为注释(行首与值后均认可);字符串与括号内的换行使值
 *   跨物理行(c_escape_multiline 只转义 `\` 与 `"`,换行原样保留)。
 *
 * 数据集(project-settings.json)由 tools/extract_project_settings.py 从 Godot
 * 源码提取并经 TextMate 令牌化交叉验证;clone 后无需 Godot 源码即可使用。
 */

import settingsData from "../data/project-settings.json";

// ---------------------------------------------------------------------------
// 数据类型(与 project-settings.json 的结构对应)
// ---------------------------------------------------------------------------

export interface SettingEntry {
  /** Variant 类型名(godot 侧小写形式,如 int/string/color);无法可靠推断时省略。 */
  type?: string;
  /** 属性提示(range/enum/file/dir/...)。 */
  hint?: string;
  /** 提示参数:range 为 "min,max,step[,or_greater]",enum 为逗号分隔候选值。 */
  hintString?: string;
  /** 注册的默认值(原始 C++ 表达式,压平空白)。 */
  default?: string;
  /** 新建工程默认值(EditorNode::get_initial_settings,仅部分键有)。 */
  initialDefault?: string;
  /** 注册点所在源码文件(相对 .refs/godot)。 */
  file?: string;
  /** 修改后需重启编辑器生效。 */
  restart?: boolean;
  /** 无注册默认值(GLOBAL_DEF_NOVAL)。 */
  noval?: boolean;
  /** 编辑器项目设置对话框的基础分组。 */
  basic?: boolean;
  /** 内部设置(不对用户展示)。 */
  internal?: boolean;
  /** 来自新建工程默认值(get_initial_settings)。 */
  initial?: boolean;
  /** 手工核验的说明(仅结构性键有)。 */
  doc?: string;
}

export interface SectionKeyDoc {
  required?: boolean;
  doc: string;
}

export interface SectionDoc {
  keys: Record<string, SectionKeyDoc>;
  doc: string;
}

export interface ProjectSettings {
  version: string;
  count: number;
  /** 全路径设置键(如 application/config/name)。 */
  keys: Record<string, SettingEntry>;
  /** 按 importer 聚合的 .import [params] 键。 */
  import_params: Record<string, Record<string, SettingEntry>>;
  /** importer → 导入产物资源类型(.import [remap] 的 type=)。 */
  import_types: Record<string, string>;
  gdextension: Record<string, SectionDoc>;
  import: Record<string, SectionDoc>;
}

export const settings: ProjectSettings = settingsData as ProjectSettings;

/**
 * 少量非 GLOBAL_DEF 注册的结构性键,手工核验自源码补充:
 * - config_version:project_settings.cpp:981 特殊分支(节为空时的 assign),
 *   CONFIG_VERSION = 5(core/config/project_settings.h:161)。
 */
const EXTRA_KEYS: Record<string, SettingEntry> = {
  config_version: {
    type: "int",
    default: "5",
    doc: "project.godot 格式版本。高于引擎 CONFIG_VERSION 的文件会被拒绝打开。",
  },
};

// ---------------------------------------------------------------------------
// ConfigFile 解析
// ---------------------------------------------------------------------------

export interface Position {
  line: number;
  character: number;
}

export interface ConfigSection {
  /** 节名(已反转义)。 */
  name: string;
  /** 节头所在行(0-based)。 */
  line: number;
  /** 节名文本在行内的起止字符(不含方括号)。 */
  nameStart: number;
  nameEnd: number;
  /** 是否已闭合(找到 ])。半输入节头不计入当前节。 */
  closed: boolean;
}

export interface ConfigEntry {
  /** 所属节名;首个节之前为 ""。 */
  section: string;
  /** 键名(已去引号、反转义)。 */
  key: string;
  /** 键所在行。 */
  line: number;
  /** 键文本在行内的起止字符(引号键含引号)。 */
  keyStart: number;
  keyEnd: number;
  /** 值文本在文档中的起止(可跨行)。 */
  valueStart: Position;
  valueEnd: Position;
  /** 值原始文本(未去引号;首尾空白已剥离,内部跨行换行原样保留)。 */
  value: string;
}

export interface ConfigFile {
  sections: ConfigSection[];
  entries: ConfigEntry[];
}

const WS = /\s/;

/** 按 ConfigFile 语义切行(Godot 认可 \n 与 \r\n)。 */
function splitLines(text: string): string[] {
  return text.split(/\r\n|\r|\n/);
}

/**
 * 解析 ConfigFile 文本。容错优先:无法识别的行跳过,不抛异常——
 * 编辑器里的文件随时处于半输入状态,解析器必须对残缺输入稳定。
 */
export function parseConfigFile(text: string): ConfigFile {
  const lines = splitLines(text);
  const sections: ConfigSection[] = [];
  const entries: ConfigEntry[] = [];
  let current = "";

  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const lead = line.length - line.trimStart().length;

    // 空行与整行注释(;)。
    if (lead >= line.length || line[lead] === ";") {
      i++;
      continue;
    }

    // 节头:[ 后到第一个未转义 ] 全为节名;\] 是转义。
    // 未闭合(半输入)时节名吞到行尾并标记 closed=false—— Godot 原版会吞到
    // EOF,这里刻意截断到行尾,保证后续行仍可解析(编辑器容错优先)。
    if (line[lead] === "[") {
      let j = lead + 1;
      let name = "";
      let nameEnd = line.length;
      let closed = false;
      while (j < line.length) {
        const c = line[j];
        if (c === "\\" && j + 1 < line.length) {
          name += line[j + 1];
          j += 2;
          continue;
        }
        if (c === "]") {
          nameEnd = j;
          closed = true;
          break;
        }
        name += c;
        j++;
      }
      sections.push({ name, line: i, nameStart: lead + 1, nameEnd, closed });
      current = name;
      i++;
      continue;
    }

    // 键:引号键整体一个 token,裸键为非空白序列(止于第一个 =)。
    let k = lead;
    let keyEnd: number;
    let key: string;
    if (line[k] === '"') {
      let j = k + 1;
      key = "";
      let closed = false;
      while (j < line.length) {
        const c = line[j];
        if (c === "\\" && j + 1 < line.length) {
          key += line[j + 1];
          j += 2;
          continue;
        }
        if (c === '"') {
          closed = true;
          j++;
          break;
        }
        key += c;
        j++;
      }
      keyEnd = closed ? j : line.length;
      if (!closed) {
        key = line.slice(k + 1);
      }
      k = j;
    } else {
      let j = k;
      while (j < line.length && !WS.test(line[j]) && line[j] !== "=") {
        j++;
      }
      key = line.slice(k, j);
      keyEnd = j;
      k = j;
    }

    // 键后必须是 =(容许空白);否则该行不是键值行,跳过。
    let m = k;
    while (m < line.length && WS.test(line[m])) {
      m++;
    }
    if (line[m] !== "=") {
      i++;
      continue;
    }

    // 值:'=' 之后开始,字符串/括号感知地跨行累积;
    // depth==0 且不在字符串内时,遇 ; (注释)或行尾结束。
    let vc = m + 1;
    while (vc < lines[i].length && WS.test(lines[i][vc])) {
      vc++;
    }
    const valueStart: Position = { line: i, character: vc };
    let value = "";
    let depth = 0;
    let inStr = false;
    let li = i;
    let ci = vc;
    let valueEnd: Position = { line: i, character: vc };
    for (;;) {
      const cur = lines[li];
      let stopped = false;
      while (ci < cur.length) {
        const c = cur[ci];
        if (inStr) {
          if (c === "\\" && ci + 1 < cur.length) {
            value += cur.slice(ci, ci + 2);
            ci += 2;
            continue;
          }
          if (c === '"') {
            inStr = false;
          }
          value += c;
          ci++;
          continue;
        }
        if (c === '"') {
          inStr = true;
          value += c;
          ci++;
          continue;
        }
        if (c === "(" || c === "[" || c === "{") {
          depth++;
          value += c;
          ci++;
          continue;
        }
        if (c === ")" || c === "]" || c === "}") {
          depth = Math.max(0, depth - 1);
          value += c;
          ci++;
          continue;
        }
        if (c === ";" && depth === 0) {
          stopped = true;
          break;
        }
        value += c;
        ci++;
      }
      if (stopped) {
        valueEnd = { line: li, character: ci };
        break;
      }
      valueEnd = { line: li, character: cur.length };
      // 括号未闭合或字符串未闭合:换行是值的一部分,继续吞下一行。
      if (depth > 0 || inStr) {
        if (li + 1 >= lines.length) {
          break;
        }
        value += "\n";
        li++;
        ci = 0;
        continue;
      }
      break;
    }

    // 剥离尾随空白(逐字符累积,尾随空白数 = 差值)。
    const trailing = value.length - value.replace(/\s+$/, "").length;
    value = value.slice(0, value.length - trailing);
    // 同行才需要钳位(防空值把终点钳到起点之前);跨行时 character 属于结束行。
    if (valueEnd.line === valueStart.line) {
      valueEnd = {
        line: valueEnd.line,
        character: Math.max(valueStart.character, valueEnd.character - trailing),
      };
    }

    entries.push({
      section: current,
      key,
      line: i,
      keyStart: lead,
      keyEnd,
      valueStart,
      valueEnd,
      value,
    });
    i = li + 1;
  }

  return { sections, entries };
}

// ---------------------------------------------------------------------------
// 位置 → 上下文
// ---------------------------------------------------------------------------

export type ConfigContext =
  | { kind: "section-name"; section: string; header?: ConfigSection }
  | { kind: "key"; section: string; entry?: ConfigEntry }
  | { kind: "value"; section: string; entry: ConfigEntry }
  | { kind: "other"; section: string };

/** pos 是否落在 entry 的值区间内(含边界,可跨行)。 */
function inValue(entry: ConfigEntry, pos: Position): boolean {
  const { valueStart: s, valueEnd: e } = entry;
  if (pos.line < s.line || pos.line > e.line) {
    return false;
  }
  if (pos.line === s.line && pos.character < s.character) {
    return false;
  }
  if (pos.line === e.line && pos.character > e.character) {
    return false;
  }
  return true;
}

/** pos 处的节名(已闭合节头中最近的一个;未闭合/无则为上一节)。 */
function sectionAt(cf: ConfigFile, pos: Position): string {
  let name = "";
  for (const s of cf.sections) {
    // 半输入节头不算数:用户还在敲名字,当前节应保持上一个。
    if (s.line <= pos.line && s.closed) {
      name = s.name;
    }
  }
  return name;
}

/**
 * 判断光标处的上下文。lineText 是光标所在行文本——半输入状态
 * (未闭合 `[`、还没有 `=` 的键)只有行文本能识别。
 */
export function contextAt(cf: ConfigFile, pos: Position, lineText: string): ConfigContext {
  // 1. 已闭合节头内部(含节名区间)。
  for (const s of cf.sections) {
    if (!s.closed || s.line !== pos.line) {
      continue;
    }
    if (pos.character >= s.nameStart && pos.character <= s.nameEnd) {
      return { kind: "section-name", section: s.name, header: s };
    }
  }
  // 2. 半输入节头:行首到光标只有 `[` 加若干非 `]` 字符。
  const before = lineText.slice(0, pos.character);
  if (/^\s*\[[^\]]*$/.test(before)) {
    return { kind: "section-name", section: sectionAt(cf, pos) };
  }
  // 3. 值区间优先于键行判断:多行值的中间行不属于任何键行,
  //    必须先按值区间命中(未闭合值吞掉后续行,与 Godot 语义一致)。
  const valueEntry = cf.entries.find((e) => inValue(e, pos));
  if (valueEntry) {
    return { kind: "value", section: valueEntry.section, entry: valueEntry };
  }
  // 4. 键 token(含还没有 `=` 的半输入键)。
  const entry = cf.entries.find((e) => e.line === pos.line);
  if (entry) {
    if (pos.character >= entry.keyStart && pos.character <= entry.keyEnd) {
      return { kind: "key", section: entry.section, entry };
    }
  } else {
    // 尚未成形的键行:行首到光标是非空白、不含 = 的 token。
    if (/^\s*[^\s=[\]]+$/.test(before)) {
      return { kind: "key", section: sectionAt(cf, pos) };
    }
  }
  return { kind: "other", section: sectionAt(cf, pos) };
}

// ---------------------------------------------------------------------------
// 数据集查询
// ---------------------------------------------------------------------------

/** project.godot / override.cfg 的节名 = 设置键的第一段(数据驱动,不硬编码)。 */
export function projectSections(): string[] {
  const set = new Set<string>();
  for (const key of Object.keys(settings.keys)) {
    const head = key.split("/")[0];
    if (head) {
      set.add(head);
    }
  }
  // [autoload] 不是 GLOBAL_DEF 注册的设置,而是 project_settings.cpp
  // begins_with("autoload/") 特殊分支(core/config/project_settings.cpp:301),
  // 手动补入。
  set.add("autoload");
  return [...set].sort();
}

/** 用 节名+文档内键 拼出全路径设置键并查数据集。 */
export function lookupSetting(section: string, key: string): SettingEntry | undefined {
  if (section && key.startsWith(`${section}/`)) {
    // 文档内已写全路径(非标准但容错)。
    return settings.keys[key];
  }
  const full = section ? `${section}/${key}` : key;
  return settings.keys[full] ?? EXTRA_KEYS[full];
}

/** 某节下可用的设置键(返回去掉节前缀的文档内键形式)。 */
export function settingKeysInSection(section: string): Array<{ key: string; entry: SettingEntry }> {
  const prefix = `${section}/`;
  const out: Array<{ key: string; entry: SettingEntry }> = [];
  for (const [full, entry] of Object.entries(settings.keys)) {
    if (full.startsWith(prefix)) {
      out.push({ key: full.slice(prefix.length), entry });
    }
  }
  return out;
}

/** enum/enum_suggestion 提示串 → 候选值列表。 */
export function enumValues(hintString: string | undefined): string[] {
  if (!hintString) {
    return [];
  }
  return hintString
    .split(",")
    .map((s) => s.trim())
    .filter((s) => s !== "");
}

/** range 提示串 → {min,max,step,orGreater}(解析失败返回 undefined)。 */
export function rangeParts(
  hintString: string | undefined,
): { min: string; max: string; step: string; orGreater: boolean } | undefined {
  if (!hintString) {
    return undefined;
  }
  const parts = hintString.split(",").map((s) => s.trim());
  if (parts.length < 3) {
    return undefined;
  }
  return { min: parts[0], max: parts[1], step: parts[2], orGreater: parts[3] === "or_greater" };
}

// ---------------------------------------------------------------------------
// Hover 文本(markdown 字符串,由 Provider 包成 vscode.MarkdownString)
// ---------------------------------------------------------------------------

const HINT_LABELS: Record<string, string> = {
  range: "数值范围",
  enum: "枚举",
  enum_suggestion: "枚举(建议)",
  expression: "表达式",
  file: "文件路径",
  dir: "目录路径",
  global_file: "全局文件路径",
  global_dir: "全局目录路径",
  resource_type: "资源类型",
  node_type: "节点类型",
  type_string: "类型字符串",
  locale_id: "区域 ID",
  multiline_text: "多行文本",
  localizable_string: "可本地化字符串",
  flags: "位标志",
  layers_2d_render: "2D 渲染层",
  layers_2d_physics: "2D 物理层",
  layers_2d_navigation: "2D 导航层",
  layers_3d_render: "3D 渲染层",
  layers_3d_physics: "3D 物理层",
  layers_3d_navigation: "3D 导航层",
};

/** 设置键的 hover markdown;fullKey 为含节前缀的全路径。 */
export function buildSettingHover(fullKey: string, entry: SettingEntry): string {
  const chips: string[] = [];
  if (entry.type) {
    chips.push(`\`${entry.type}\``);
  }
  if (entry.basic) {
    chips.push("基础");
  }
  if (entry.restart) {
    chips.push("修改后需重启编辑器");
  }
  if (entry.noval) {
    chips.push("无注册默认值");
  }
  if (entry.internal) {
    chips.push("内部");
  }
  if (entry.initial) {
    chips.push("新建工程默认");
  }
  const lines: string[] = [`**${fullKey}** ${chips.join(" · ")}`.trimEnd()];

  if (entry.doc) {
    lines.push(entry.doc);
  }

  if (entry.hint && entry.hint !== "none") {
    const label = HINT_LABELS[entry.hint] ?? entry.hint;
    const range = entry.hint === "range" ? rangeParts(entry.hintString) : undefined;
    if (range) {
      const max = range.orGreater ? `${range.max}(或更大)` : range.max;
      lines.push(`${label}:\`${range.min}\` – \`${max}\`,步进 \`${range.step}\``);
    } else {
      const values = enumValues(entry.hintString);
      if (values.length > 0) {
        lines.push(`${label}:${values.map((v) => `\`${v}\``).join(" / ")}`);
      } else if (entry.hintString) {
        lines.push(`${label}:\`${entry.hintString}\``);
      } else {
        lines.push(`${label}`);
      }
    }
  }

  const defaults: string[] = [];
  if (entry.default !== undefined) {
    defaults.push(`默认值 \`${entry.default}\``);
  }
  if (entry.initialDefault !== undefined && entry.initialDefault !== entry.default) {
    defaults.push(`新建工程 \`${entry.initialDefault}\``);
  }
  if (defaults.length > 0) {
    lines.push(defaults.join(" · "));
  }

  if (entry.file) {
    lines.push(`注册点:\`${entry.file}\`(Godot ${settings.version} 源码)`);
  }
  return lines.join("\n\n");
}

/** .gdextension / .import 节内键的 hover markdown。 */
export function buildSectionKeyHover(kind: "gdextension" | "import", section: string, key: string): string | undefined {
  const map = kind === "gdextension" ? settings.gdextension : settings.import;
  const doc = map[section]?.keys[key];
  if (!doc) {
    return undefined;
  }
  const head = `**${key}**${doc.required ? "(必需)" : "(可选)"} — ${section} 节`;
  return `${head}\n\n${doc.doc}\n\n来源:Godot ${settings.version} 源码核验`;
}
