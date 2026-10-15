# godotlsp — 独立 GDScript 语言服务器

<img src="godot_logo.png" alt="godotlsp" width="480">

**godotlsp** 是一个完全脱离 Godot 引擎运行的 GDScript 语言服务器:引擎 API 模型以静态数据内嵌在服务器二进制里,不连接 Godot 进程、无需打开 Godot 编辑器,即可获得完整的补全、悬停文档、导航与诊断体验。

> 仓库:<https://github.com/SantaChains/godotlsp>(尚未上架 VS Code Marketplace,当前以 VSIX 分发)

## 功能

语言智能全部由服务器离线提供,**运行期零 Godot 依赖**——`godot` 可执行文件只是「运行」类命令的可选入口,缺失不影响以下任何特性:

- **悬停文档**:引擎类/方法/属性/信号/常量完整签名与文档(附 zh-CN 官方文档链接);全局函数、单例、注解、关键字全覆盖
- **补全**:成员补全带类型与文档;全局类、内建类型、单例、工具函数、全局枚举;`$NodePath` / `%Unique` 场景节点路径感知
- **导航**:跳转定义、查找引用、文档符号(侧边栏符号树)、工作区符号、引用高亮
- **诊断**:解析错误与类型告警实时推送,支持按码忽略与严格模式
- **其他**:内联类型提示、文档颜色、折叠范围、格式化、语义高亮、`res://` 路径点击跳转

服务器能力全清单与架构说明见[仓库根 README](https://github.com/SantaChains/godotlsp#readme)。

## 侧边栏

活动栏的 **godotlsp** 视图提供三个树视图,点击即跳转:

- **当前文件符号**:类、函数、枚举、信号、常量(LSP `documentSymbol`,VS Code 内建「大纲」同样可用)
- **项目结构**:`[autoload]` 单例 → `class_name` 全局类注册表 → 展开任意脚本即见其函数/类/枚举大纲
- **场景结构**:当前 `.tscn` 的节点层级(`[node name/type/parent/instance]`),instance 节点直接显示被实例化资源路径

## 支持的文件类型

| 语言 | 扩展名 | 说明 |
|---|---|---|
| GDScript | `.gd` | 全 LSP 特性 |
| Godot Resource | `.tscn` `.tres` | 场景/资源文本格式高亮 |
| Godot Project | `project.godot` `.godot` `.import` `.gdextension` | 工程配置高亮 |
| Godot UID | `.uid` | 资源 UID 高亮 |

## 安装(VSIX)

在 VS Code 中:`扩展` 面板右上角 `···` → **从 VSIX 安装…**,选择 `godotlsp-0.1.0.vsix`。

或命令行:

```powershell
code --install-extension godotlsp-0.1.0.vsix
```

服务器二进制随扩展打包(`server/`),Windows 开箱即用;其他平台请自行构建后用 `godotlsp.path` 指向,或放入 `PATH`。

## 配置

| 配置项 | 默认 | 说明 |
|---|---|---|
| `godotlsp.path` | 空 | 服务器可执行文件路径;留空用扩展自带二进制,再回退 `PATH` |
| `godotlsp.diagnostics.strict` | `false` | 严格模式:开启默认关闭的可选警告 |
| `godotlsp.diagnostics.ignore` | `[]` | 按码忽略的诊断(如 `UNUSED_VARIABLE`) |
| `godotlsp.inlayHints.typeHints` | `true` | 内联 `: 类型` 提示 |
| `godotlsp.godotExecutable` | 空 | 「运行项目/场景」所用 Godot 路径;留空回退 `PATH` 中的 `godot` |
| `godotlsp.trace.server` | `off` | LSP 通信日志 |

## 命令

| 命令 | 说明 |
|---|---|
| `godotlsp.restart` | 重启服务器 |
| `godotlsp.showOutput` | 打开输出通道 |
| `godotlsp.runProject` | 运行项目(需 Godot) |
| `godotlsp.runScene` | 运行当前场景(需 Godot) |
| `godotlsp.symbols.refresh` | 刷新侧边栏符号树 |
| `godotlsp.project.refresh` | 刷新项目结构(重建 class_name 注册表) |
| `godotlsp.scene.refresh` | 刷新场景结构 |

## 从源码构建

```bash
# 服务器(Rust,workspace 根)
cargo build --release

# 插件
cd editors/vscode
npm install
npm run build          # esbuild 打包 dist/extension.js
npx @vscode/vsce package --no-dependencies
```

## 多 IDE 兼容

服务器是纯 LSP 3.17 实现,能力全协商、配置拉/推双通道,可接入 Neovim / Zed / Helix 等任意 LSP 客户端(接入示例见仓库根 `README.md`)。

## 许可

MIT。Godot 图标与 Logo 来自 Godot Engine([CC BY 4.0](https://creativecommons.org/licenses/by/4.0/))。
