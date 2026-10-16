# Godot 是什么

**Godot 是一套免费、开源（MIT 许可，无版税、无分成）的一体化 2D/3D 游戏引擎：编辑器、场景系统、物理、渲染、音频、脚本、导出打包全部内置，不需要拼装第三方中间件。** 2D 能力是其传统强项，3D 在 4.x 换代后达到可用水准。

> 版本快照以 2026 年 10 月为准，引擎持续发版，下载前以官网当前稳定版为准；本书行为基线为 Godot 4.x。

## 许可与成本

- **MIT 许可**：引擎源码、编辑器可自由使用、修改、商用，不抽成、不按销量收费。
- 无强制启动画面、无功能锁、无订阅档位；导出到任何平台不向引擎付费。
- 编辑器本体是单个可执行文件，无需安装器，解压即用；标准版与 .NET 版（带 C#）分开下载。

## 版本现状（2026-10）

| 线 | 状态 | 说明 |
|---|---|---|
| 4.7 | 当前稳定主线 | "Lights, Camera, Action!"；维护版已到 4.7.2 |
| 4.6 | 上一稳定线 | 选择与变换模式解耦（Select-only / Transform），体验打磨 |
| 4.5 | 旧维护线 | 仅接收关键修复，新项目不上 4.5 |
| 3.6 | 旧 LTS | 仅维护，新脚本语法（@export、await、类型化数组）与 4.x 不兼容 |
| Godot 5 | 开发中 | 截至本书快照未发布稳定版，不要按 5.x API 写新项目 |

4.7 值得知道的新增（详细以官方 release note 为准）：

- `AreaLight3D` 矩形面光源，软阴影与真实反射，不必再用自发光+GI 冒充面光。
- HDR 输出（Windows/macOS/iOS/visionOS/Linux Wayland，仅 Forward+/Mobile，Compatibility 不支持）。
- `Control` 的 `offset_transform_*`：不被容器重置的视觉位移/旋转/缩放，默认纯视觉不影响鼠标命中。
- 新版 Asset Store 取代旧 Asset Library，可按平台/架构单独下载导出模板。
- `DrawableTexture2D` 简化的运行时画纹理 API；Android 独立导出发布（GABE）；3D 最近邻过滤（复古低分辨率 3D）；2D Scene Paint Mode 散布放置。

## 三种渲染器

Project Settings → Rendering → Renderer 选择，决定画质上限与兼容面：

| 渲染器 | 目标 | 特点 |
|---|---|---|
| Forward+ | 桌面高性能 | 功能最全：SDFGI、大量灯光、HDR、后期效果 |
| Mobile | 手机/中端 | 裁剪灯光，功能子集，功耗友好；XR 新项目推荐 |
| Compatibility | 老旧设备/WebGL | 无 compute shader，最低画质，兼容面最广 |

渲染器能力差异（自动曝光、DOF、面光阴影等仅部分支持）见[物理与渲染预算](../perf/04-physics-render.md)与各类参考页注释。

## 三种脚本语言

| 语言 | 适用 |
|---|---|
| GDScript | 内置，Python 风格，与场景/信号深度咬合，默认首选（本书主体） |
| C# | .NET 版引擎，适合团队带 .NET 栈、复杂业务逻辑 |
| GDExtension（C++/Rust/Zig 等） | 性能热路径、复用原生库，编译为动态库接入，见[GDExtension 篇](../ext/01-what-is.md) |

## 导出平台

一次工程，一键导出：桌面 Windows/macOS/Linux，移动 Android/iOS，Web HTML5；另有无头 Server 模板用于服务端。iOS 导出仍需配合 Xcode 签名，Android 需 JDK/Gradle 环境（4.7 起支持设备端 GABE 独立导出）。导出前下载对应平台 Export Templates，命令行可无头打包进 CI。

## 生态与配套工具

Godot 自身不包揽美术生产，外围按管线分工：

| 环节 | 常用工具 | 与 Godot 的交接 |
|---|---|---|
| 像素美术/帧动画 | Aseprite（或开源替代 LibreSprite） | 导出均匀网格 PNG，或用 Aseprite Wizard 插件直读 `.aseprite` 生成 SpriteFrames |
| 2D 关卡瓦片 | Tiled（开源） | 导出 JSON/PNG，经插件或自定义解析进 TileSet/TileMapLayer |
| 概念图/纹理绘制 | Krita、GIMP；矢量 UI 用 Inkscape | 导出 PNG/SVG，走标准导入管线 |
| 3D 建模/骨骼动画 | Blender | glTF 2.0 `.glb` 或直读 `.blend`，见[Blender 搭配](03-blender-interop.md) |
| 音频 | Audacity 剪辑，OpenGameArt/Kenney 取免费素材 | WAV/OGG，音频节点选型见[节点选型目录](../paradigm/10-node-catalog.md) |
| 插件市场 | 内置 Asset Store | 导航寻路、对话、保存系统等现成插件，先搜再造 |

> 选型原则：引擎内解决不了的再上外部工具；能用标准格式（PNG/glTF/WAV/OGG）交接，避免绑定私有工程文件。

## 延伸阅读

- 官方：[Godot 官网](https://godotengine.org/)、[4.7 Release Notes](https://godotengine.org/releases/4.7/)、[系统需求](https://docs.godotengine.org/en/stable/about/system_requirements.html)
- 本站：[项目设置与像素清晰](01-project-settings.md)、[Blender 搭配](03-blender-interop.md)
