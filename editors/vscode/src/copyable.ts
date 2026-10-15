import * as vscode from "vscode";
import { relDisplay } from "./projectTree";

/**
 * 侧边栏三视图条目的右键快速复制。载荷挂在 WeakMap(不污染 TreeItem
 * 字段面);contextValue 统一为 godotlsp.copyable,三个视图共用同一组
 * 右键菜单命令。
 */
export interface CopyInfo {
  /** 条目名称(符号名/节点名/类名)。 */
  name: string;
  /** 位置引用「相对路径:行号」,行号从 1 计;无跳转目标时省略。 */
  ref?: string;
}

const payloads = new WeakMap<vscode.TreeItem, CopyInfo>();

/** 给 TreeItem 打上可复制标记并登记载荷。 */
export function withCopy(item: vscode.TreeItem, info: CopyInfo): vscode.TreeItem {
  item.contextValue = "godotlsp.copyable";
  payloads.set(item, info);
  return item;
}

/**
 * 解析右键菜单传入的节点:可能是 getTreeItem 产出的 TreeItem(载荷在
 * WeakMap),也可能是 provider 的原始节点(SymbolNode/PNode,鸭子类型
 * 读取字段)。两种形状统一收敛为 CopyInfo。
 */
export function copyInfoFrom(node: unknown): CopyInfo | undefined {
  if (node instanceof vscode.TreeItem) {
    return payloads.get(node);
  }
  if (typeof node !== "object" || node === null) {
    return undefined;
  }
  const n = node as {
    symbol?: { name: string; selectionRange?: { start?: { line?: number } } };
    uri?: string;
    label?: string;
    jump?: { uri: string; line: number };
    description?: string;
  };
  // SymbolNode 形状:symbol + uri。
  if (n.symbol && typeof n.symbol.name === "string") {
    const line = n.symbol.selectionRange?.start?.line;
    return {
      name: n.symbol.name,
      ref: n.uri ? toRef(n.uri, line) : undefined,
    };
  }
  // PNode 形状:label + jump/description。
  if (typeof n.label === "string") {
    return {
      name: n.label,
      ref: n.jump ? toRef(n.jump.uri, n.jump.line) : n.description,
    };
  }
  return undefined;
}

function toRef(uri: string, line: number | undefined): string {
  const base = relDisplay(uri);
  return line === undefined ? base : `${base}:${line + 1}`;
}
