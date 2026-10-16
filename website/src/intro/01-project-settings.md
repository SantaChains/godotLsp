# 项目设置与显示配置

**新建工程先定三件事：基础分辨率与拉伸策略、纹理重采样（过滤）方式、目标平台导出参数。** 这些在 Project Settings 与 Import 面板里一次性配好，后期返工成本最低。

## 窗口与拉伸（Display > Window）

| 设置 | 路径 | 作用 |
|---|---|---|
| Viewport Width/Height | Display > Window > Size | 逻辑分辨率，与屏幕实际像素解耦 |
| Stretch Mode | Display > Window > Stretch > Mode | 逻辑分辨率如何映射到屏幕 |
| Stretch Aspect | Display > Window > Stretch > Aspect | 宽高比不一致时的留白策略 |
| Scale Mode | Display > Window > Stretch > Scale Mode | 缩放取整方式 |

Stretch Mode 四种：

| Mode | 行为 | 适用 |
|---|---|---|
| disabled | 1:1，逻辑分辨率=物理像素 | 固定窗口、桌面工具 |
| canvas | 整体缩放画布，UI 随比例放大 | 2D 像素游戏 |
| viewport | 把逻辑画面当一张纹理再缩放，视野随窗口变化 | 2D 世界（相机移动） |
| items | 只缩放控件布局，世界不缩放 | UI 密集的应用 |

Stretch Aspect：`keep`（等比，两侧黑边，最稳）、`expand`（填满并扩展可见范围，需给 Control 配 anchor）、`ignore`（拉伸变形，禁用）。

## 像素不清析：重采样与过滤

**模糊的根因是纹理被放大时用了线性插值（Linear），相邻像素被平均成过渡色；像素艺术必须用最近邻（Nearest），即每个目标像素直接取最近源像素，不做混合。** 这就是常说的"关闭过滤/开最近邻重采样"。

过滤在两处生效，优先级后者覆盖前者：

1. 项目默认：Rendering > Textures > Canvas Textures > Default Texture Filter = Nearest。
2. 节点覆盖：每个 `CanvasItem` 的 Inspector > Texture > Filter，默认 Inherit（继承父级）。可在父节点统一设 Nearest，子节点留 Inherit 即全部生效。

导入面板兜底：选中 PNG → Import 面板取消 Filter，或右上角预设选 **2D pixel**，改完点 Reimport。

配套三项（像素游戏）：

| 设置 | 值 | 解决什么 |
|---|---|---|
| Stretch > Scale Mode | integer | 非整数倍缩放时像素大小忽大忽小、画面抖动 |
| Rendering > 2D > Snap 2D Transforms to Pixel | On | 相机/节点落在亚像素位置导致的边缘抖动 |
| 基础分辨率 | 与瓦片尺寸整除（如 320×180） | 整数缩放时不留半像素缝 |

> 完整像素项目设置表见[精灵表规范](../assets/00-sprite-sheet.md)；4.7 还提供 3D 最近邻过滤，用于复古低分辨率 3D 渲染。

### mipmap 的取舍

高清素材向下采样（屏幕分辨率低于纹理）时，不做 mipmap 会出现摩尔纹/闪烁；此时开启 mipmaps 做逐级预滤波。代价是内存约增加 33%，低端移动设备慎开。**纯像素艺术禁用 mipmap**，它会在缩小帧时混入相邻像素。

## 导入预设（Import 面板）

选中资源后在 Import 面板配好，点 Preset > Save As 存为预设，同类资源一键套用，避免逐张重配：

| 资源 | 关键项 |
|---|---|
| 像素 PNG | Compress Mode = Lossless；Filter = Project Default；Repeat = Disabled |
| 高清 UI/纹理 | Lossy 可接受时用 VRAM Compressed 省内存；按需开 mipmaps |
| glTF/.blend | Import As = Scene；Animation 页签配循环与根缩放（见[Blender 搭配](03-blender-interop.md)） |
| 音频 | WAV 短音效（即时触发）；OGG Vorbis 长音乐（流式） |

> 改任何导入项后必须点 Reimport；文件复制进项目但没重配，项目默认值不一定符合预期。

## 渲染默认（Rendering）

- Renderer（Forward+/Mobile/Compatibility）在 Project Settings > Rendering > Renderer Method，切换后需重启；选错渲染器后部分光照/后期效果静默不可用，见[物理与渲染预算](../perf/04-physics-render.md)。
- 窗口、光标、默认相机清除色、阴影上限等在此统一；不要在代码里临时补本该走项目默认的常量。

## 平台导出要点（Export 对话框）

| 平台 | 必备前置 |
|---|---|
| Windows | Export Template 已下载；图标 .ico；可选导出为 zip/exe |
| macOS | 签名/公证（分发必需）；Bundle Identifier |
| Linux | 单文件或目录包；图标 |
| Android | JDK、Gradle、keystore；Manifest 权限（INTERNET、震动等按需）；4.7 起可用 GABE 在设备端独立导出 |
| iOS | Xcode 工程签名；Bundle ID；Provisioning Profile |
| Web | HTML5 模板；CORS、线程模型（多线程需 SharedArrayBuffer 头） |

- Export Presets 每平台一份；权限、图标、启动屏按平台单独填，不要用一份配置套所有端。
- 命令行 `godot --headless --export-release <平台> <输出路径>` 可进 CI，见导出官方文档。

## 常见错配清单

1. 像素图模糊：只改了项目默认，节点 Filter 仍 Inherit 到了 Linear 父级。
2. 整数缩放后画面偏移黑边：基础分辨率与瓦片尺寸不整除。
3. 移动端闪退：忘了下载对应平台 Export Templates，或选了 Forward+ 而目标设备只支持 Compatibility。
4. 改完导入不生效：没点 Reimport。

## 延伸阅读

- 官方：[Multiple resolutions](https://docs.godotengine.org/en/stable/tutorials/rendering/multiple_resolutions.html)、[Import pipeline](https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/import_process.html)
- 本站：[精灵表规范](../assets/00-sprite-sheet.md)、[Blender 搭配](03-blender-interop.md)
