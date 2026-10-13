# godotlsp

A standalone Godot LSP written in Rust, with a VSCode extension.

脱离 Godot 引擎运行的 GDScript 语言服务器:LSP over stdio,内嵌 Godot 4.x 引擎模型,不启动编辑器即可提供完整的语言智能。CI、无头环境、远程开发均可使用。

## 功能

- 悬停、代码补全、跳转定义、查找引用、重命名、`prepareRename`
- 诊断(语法 + 语义)、语义高亮、inlay hint、折叠范围、签名帮助
- 代码操作、整文档与选区格式化
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

## License

MIT
