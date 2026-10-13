# godotlsp

A standalone Godot LSP written in Rust, with a VSCode extension.

脱离 Godot 引擎运行的 GDScript 语言服务器:LSP over stdio,内嵌 Godot 4.x 引擎模型,不启动编辑器即可提供完整的语言智能。CI、无头环境、远程开发均可使用。

## 功能

- 悬停(类型 + 引擎文档 + `##` 注释)、代码补全、跳转定义、查找引用、重命名、`prepareRename`
- 引擎全局面补全:1036 引擎类、38 内建类型、单例、`@GlobalScope` 工具函数、GDScript 内建函数、全局枚举及值、`PI`/`TAU` 等常量(签名与文档以引擎模型为权威)
- 补全/签名文档回填:成员补全与 `recv.method(` 签名帮助按引擎模型补全签名与文档;自有成员补全带声明签名与 `##` 注释;注解补全带说明;语言关键字 hover
- 文档跳转:悬停附官方 zh-CN 文档链接;类名/单例/全局函数/注解/关键字 hover 全覆盖
- 诊断(语法 + 语义,支持严格模式与按码忽略)、语义高亮、inlay hint(可开关)、折叠范围、签名帮助
- 代码操作、整文档与选区格式化
- 符号高亮(`documentHighlight`)、`res://` 路径可点击跳转(`documentLink`)
- `Color()` 字面量取色器(`documentColor`)
- 增量文本同步、文件监视(无需打开即感知项目文件增删改)
- 配置双通道:`workspace/configuration` 拉取 + `didChangeConfiguration` 推送(纯 LSP,无编辑器特设)
- 诊断带 `source`/`code`/`version`;请求级 panic 隔离,单请求崩溃不拖垮服务器
- 跨文件解析:`class_name`、autoload、`preload`/`extends`
- 场景感知:`$NodePath`、`%unique`、`get_node` 到场景节点的类型推断
- 工作区符号、项目扫描(`project.godot` 热更新、`res://` 路径)

## 构建

```sh
cargo build --release
# 服务器产物:target/release/godotlsp
```

## 使用

任意 LSP 客户端:以 stdio 方式启动 `godotlsp`。

VSCode:安装 [editors/vscode](editors/vscode)。`godotlsp.path` 指向服务器二进制,或将二进制拷入 `editors/vscode/server/` 随扩展打包。

### 其他编辑器

服务器是纯 LSP 3.17 over stdio,任何客户端均可直接接入(示例以 `target/release/godotlsp` 为准):

**Neovim**(0.11+ 原生 LSP,`lsp/godotlsp.lua`):

```lua
vim.lsp.config("godotlsp", {
  cmd = { vim.fn.expand("~/path/to/godotlsp"), "--stdio" },
  filetypes = { "gdscript" },
  root_markers = { "project.godot", ".git" },
})
vim.lsp.enable("godotlsp")
```

**Zed**(`~/.config/zed/settings.json`,利用其自定义 LSP):

```json
{
  "lsp": {
    "godotlsp": {
      "binary": { "path": "/path/to/godotlsp", "arguments": ["--stdio"] }
    }
  },
  "languages": {
    "GDScript": { "language_servers": ["godotlsp", "!godot"] }
  }
}
```

**Helix**(`languages.toml`):

```toml
[[language]]
name = "gdscript"
language-servers = ["godotlsp"]

[language-server.godotlsp]
command = "godotlsp"
args = ["--stdio"]
```

配置经 `workspace/configuration` / `didChangeConfiguration` 下发,键名与 VSCode 扩展一致(`diagnostics.strict`、`diagnostics.ignore`、`inlayHints.typeHints`)。设计依据与路线图见 [docs/lsp-design.md](docs/lsp-design.md) 与 [docs/gdscript-features-roadmap.md](docs/gdscript-features-roadmap.md)。

## 测试

```sh
cargo test
```

单元测试覆盖引擎模型接入、跨文件推断与诊断;端到端冒烟测试覆盖完整 stdio 协议会话,可通过 `GODOTLSP_SMOKE_ROOT` 指定语料目录。

## 致谢

本项目站在这些项目的肩膀上:

- [gdscript-analyzer](https://github.com/yanivkalfa/gdscript-analyzer) — 核心分析引擎 `gdscript-ide`(salsa 增量、场景感知、内嵌引擎模型);`uri.rs` 的 URI 转换移植自其 `gdscript-lsp`。
- [gdls](https://github.com/kurushimee/gdls) — VSCode 扩展复用了其语言配置与 TextMate 语法文件,扩展结构亦受其启发。
- [gdck](https://github.com/eth0net/gdck) — 无损 GDScript 解析器,技术选型阶段的重要参考。
- [godot-vscode-plugin](https://github.com/godotengine/godot-vscode-plugin) — 官方插件,功能对照与行为参考。
- [godot-docs](https://docs.godotengine.org) — Godot 官方文档。本仓库本地 `godot-docs/` 语料源自《GodotEngine.epub》中文翻译(译者:孤辰辰),用于后续离线中文文档集成。

## License

MIT
