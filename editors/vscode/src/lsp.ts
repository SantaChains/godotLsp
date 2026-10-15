import * as vscode from "vscode";
import { LanguageClient, State } from "vscode-languageclient/node";

/**
 * LSP 通信统一入口:全部插件→服务器请求走这里,
 * 保证「运行态检查 + 错误降级 + 日志」一处实现,树视图同构复用。
 * 协议纪律:只用标准 LSP 方法,不自造私有 method(见路线图取舍原则)。
 */

/** LSP DocumentSymbol(嵌套)的最小形状——只取树渲染所需字段。 */
export interface LspSymbol {
  name: string;
  detail?: string;
  kind: number;
  range: { start: { line: number; character: number }; end: { line: number; character: number } };
  selectionRange: { start: { line: number; character: number }; end: { line: number; character: number } };
  children?: LspSymbol[];
}

/** 客户端就绪才能发请求;重启窗口期一律走降级路径。 */
export function isRunning(client: LanguageClient | undefined): client is LanguageClient {
  return client !== undefined && client.state === State.Running;
}

/**
 * `textDocument/documentSymbol`:服务器离线/文档关闭/请求竞态一律返回 null,
 * 调用方展示空态,不抛错刷屏;细节进 LogOutputChannel.debug。
 */
export async function documentSymbols(
  client: LanguageClient | undefined,
  uri: string,
  log: vscode.LogOutputChannel,
): Promise<LspSymbol[] | null> {
  if (!isRunning(client)) {
    return null;
  }
  try {
    return await client.sendRequest<LspSymbol[] | null>("textDocument/documentSymbol", {
      textDocument: { uri },
    });
  } catch (err) {
    log.debug(`documentSymbol ${uri} 请求失败:${String(err)}`);
    return null;
  }
}

/** LSP SymbolKind → VS Code 符号主题图标 id(所有树视图共用一份映射)。 */
export function iconForKind(kind: number): string {
  return (
    {
      1: "symbol-file",
      2: "symbol-module",
      3: "symbol-namespace",
      5: "symbol-class",
      6: "symbol-method",
      7: "symbol-property",
      8: "symbol-field",
      9: "symbol-constructor",
      10: "symbol-enum",
      11: "symbol-interface",
      12: "symbol-function",
      13: "symbol-variable",
      14: "symbol-constant",
      22: "symbol-enum-member",
      23: "symbol-class",
      24: "symbol-event",
      26: "symbol-type-parameter",
    } as Record<number, string>
  )[kind] ?? "symbol-variable";
}
