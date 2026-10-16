/**
 * 树视图增强的纯逻辑层(21b 过滤 / 21d 诊断旗标)。
 * 不 import vscode——node:test 直接单测;vscode 交互(读诊断/设置过滤词)
 * 在 provider/extension 层,把数据传进来。
 */

/** 过滤词匹配:大小写不敏感子串,空过滤词恒匹配。任一字段命中即可。 */
export function matchFilter(filter: string, ...fields: (string | undefined)[]): boolean {
  const f = filter.trim().toLowerCase();
  if (!f) {
    return true;
  }
  return fields.some((x) => x !== undefined && x.toLowerCase().includes(f));
}

/** 树节点的过滤规格:标签/描述取值 + 子节点解析器(同步或异步)。 */
export interface FilterSpec<T> {
  labelOf: (n: T) => string;
  descOf: (n: T) => string | undefined;
  kidsOf: (n: T) => readonly T[] | Promise<readonly T[]>;
}

/**
 * 一层树过滤(21b):命中自身或「含命中后代」的节点保留(祖先链完整)。
 * provider 在 getChildren 每层调用——子层会再次过滤,显示集与过滤词一致。
 * 空过滤词恒等返回(浅拷贝,零遍历)。
 */
export async function filterTree<T>(
  nodes: readonly T[],
  filter: string,
  spec: FilterSpec<T>,
): Promise<T[]> {
  if (!filter.trim()) {
    return [...nodes];
  }
  const out: T[] = [];
  for (const n of nodes) {
    const kids = await spec.kidsOf(n);
    const kept = await filterTree(kids, filter, spec);
    if (kept.length > 0 || matchFilter(filter, spec.labelOf(n), spec.descOf(n))) {
      out.push(n);
    }
  }
  return out;
}

/** 诊断的最小形状(0 基行号;severity 遵 LSP:1=Error 2=Warning)。 */
export interface DiagLike {
  range: { start: { line: number } };
  severity?: number;
  message?: string;
}

/**
 * 行区间诊断旗标(21d):统计与 [startLine, endLine] 相交的诊断,
 * 产出 description 后缀(` ✗1 ⚠2`)与 tooltip 附加文本。
 * 用区间相交而非仅起点,使多行符号(函数体)的内部错误也计入。
 */
export function diagSuffix(
  diags: readonly DiagLike[],
  startLine: number,
  endLine: number,
): { suffix: string; tooltip: string } {
  let errors = 0;
  let warnings = 0;
  const messages: string[] = [];
  for (const d of diags) {
    const line = d.range?.start?.line;
    if (line === undefined || line < startLine || line > endLine) {
      continue;
    }
    if (d.severity === 1) {
      errors++;
      messages.push(`✗ ${d.message ?? ""}`);
    } else if (d.severity === 2) {
      warnings++;
      messages.push(`⚠ ${d.message ?? ""}`);
    }
  }
  const suffix = (errors > 0 ? ` ✗${errors}` : "") + (warnings > 0 ? ` ⚠${warnings}` : "");
  return { suffix, tooltip: messages.join("\n") };
}
