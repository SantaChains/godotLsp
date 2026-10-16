# 精灵表规范

**精灵表（Sprite Sheet）又称纹理图集（Texture Atlas）：把多个精灵帧或一组静态资产打包进一张均匀网格的 PNG。引擎只上传一次纹理，按行列索引取帧，draw call 与纹理切换次数显著下降。** 动画序列（walk/attack/idle）与静态资产集合（道具、UI 图标）都可以打包。

> Godot **不会自动解析**第三方工具导出的 XML/JSON 图集元数据（如 TexturePacker 的 `.json`/`.plist`）。要使用它们必须安装对应导入插件或自写解析器；**默认可靠路径是均匀网格 PNG，引擎内直接切片**。

## 四条引擎消费路径速判

| 需求 | 用什么 | 见 |
|---|---|---|
| 播放角色/特效帧序列动画 | `AnimatedSprite2D` + `SpriteFrames` | [动画路径](01-animation-paths.md#animatedsprite2d) |
| 单帧静态精灵，或按索引读网格 | `Sprite2D` 的 `hframes`/`vframes`/`frame` | [动画路径](01-animation-paths.md#hframes) |
| 变时序、演出类关键帧动画 | `Sprite2D` + `AnimationPlayer` | [动画路径](01-animation-paths.md#animationplayer) |
| 跨场景复用同一帧，改布局一次生效 | `AtlasTexture` 独立 `.tres` | [动画路径](01-animation-paths.md#atlastexture) |
| 瓦片地图 | `TileSet` 编辑器 atlas 页签 | 类参考 [TileSet](../builtin/class_tileset.md) |
| 上千个相同实例（落叶、人群） | `MultiMeshInstance2D` + 共享纹理 | [批量与结构优化](../perf/03-bulk.md) |

## 制表规范

### 网格与尺寸

- **均匀网格，所有帧尺寸严格一致**。要在 Godot 内切片的表一律不做 trim（裁边）、不旋转打包，否则网格对不齐。
- 帧尺寸取 2 的幂：16×16 复古俯视角色、32×32 街机动作、48×48 稍大人形、64×64 动作 RPG 主角、128×128 Boss 级。
- **帧间 padding 1–2 px，整表外沿留 margin**。像素艺术缩放播放时相邻帧互相渗透（bleeding），2 px padding 或像素外扩（extrusion）可消除。
- **同一动画连续排一行**，便于 SpriteFrames 按行区间添加，也便于脚本按 index 计算定位。
- 表总尺寸建议同为 2 的幂（512×512、1024×1024），利于 GPU 纹理处理与移动端压缩格式。

### 项目设置（像素艺术必做）

| 设置项 | 路径 | 值 |
|---|---|---|
| 默认纹理过滤 | Rendering > Textures > Canvas Textures > Default Texture Filter | Nearest |
| 拉伸模式 | Display > Window > Stretch > Mode | viewport |
| 整数缩放 | Display > Window > Stretch > Scale Mode | integer |
| 2D 像素对齐 | Rendering > 2D > Snap 2D Transforms to Pixel | On |
| 基础分辨率 | Display > Window > Size | 配合瓦片尺寸，如 320×180 / 384×216 |

> 纹理过滤在两处生效：项目默认值，与每个 `CanvasItem` 节点的 `texture_filter` 覆盖（后者优先）。像素艺术需**两处均为 Nearest**，只改项目设置而节点残留 Linear 仍会模糊。整数缩放保证任何窗口尺寸下像素大小均匀（代价是特定尺寸出现黑边）；Snap 2D Transforms to Pixel 消除相机移动时的亚像素抖动。

### 导入设置

PNG 带透明通道是唯一推荐格式。选中文件在 Import 面板：Compress Mode 设 **Lossless**（像素艺术禁用 Lossy）；Filter 保持 Project Default（前提是项目默认已设 Nearest）；Repeat 设 Disabled（平铺纹理除外）。改完必须点 **Reimport** 才生效。

## 动画规范

- 动画命名与 GDScript 调用严格一致：`play("run")` 只匹配名为 `run` 的动画，**区分大小写**。
- FPS 与 `speed_scale` 必须大于 0；循环类动画（idle、walk）勾选 Loop。
- 帧时长参考：走跑循环每帧 60–120 ms，idle 呼吸类 100–200 ms。
- 切帧后逐一核对每个矩形，网格数填错是裁切、空白、重复、偏移帧的首要来源。
- 状态机复杂时用 `AnimationTree`；简单切换直接代码调 `play()`。

## 性能与分层

- **合批条件**：共享同一纹理与同一材质的 CanvasItem 才会被合批。因此尽量把相关精灵打进同一张图集，散落多张纹理会让 draw call 暴涨。
- 分层绘制：`CanvasLayer` 管最外层（世界 / HUD / UI 各一层）；同层内 `y_sort_enabled` 处理俯视/等距遮挡（低 y 后画，压在高 y 之上）；`z_index` 手动强制层级，少用；树序兜底。
- 移动端纹理压缩走标准导入管线：ASTC、ETC1/ETC2、DXT、Basis Universal（KTX2），按平台选择。

## AI 生成精灵表（精简）

扩散模型的默认目标（平滑渐变、抗锯齿、高细节）与像素艺术（硬边、限色调板、零抗锯齿、刻意低分辨率）相反，必须显式施加硬约束，且人工清洗与引擎内测试不可省。

**跨帧一致性四锚点**（每帧都保持恒定）：

1. 面部：骨相、瞳色、发型轮廓、识别标记。
2. 身体剪影：身高、体量、缩放下可读的轮廓。
3. 调色板：固定 3–5 种主色，明文写色值或色名，禁止近似色漂移。
4. 服装语法：披风长度、护甲块分布、武器形制。

提示词结构：风格参照 + 主体 + 技术规格 + 动画细节。

```text
16-bit pixel art sprite of a space marine, full body, idle standing pose,
side-scroller perspective, for a video game. Limited 5-color palette
(dark grey, cyan, silver, black, red visor). Crisp clean pixel art,
sharp black outlines, no dithering, no anti-aliasing, 32x32 grid,
plain white background.
```

负向词：blurry, fuzzy, soft, gradient, anti-aliasing, dithering, interpolation, photorealistic, 3D, jpeg artifacts, noise, watermark。指定世代锚点（NES 4 色 32×32、SNES 16 色 64×64、现代 128×128）比笼统 retro 精确；在工具原生尺寸生成，不要事后放大。

**一致性手段（按可靠性叠加）**：先生成一张定稿 idle 帧作为参考图锁定身份；同 seed 生成全部帧；IP-Adapter 以图像锚定身份、文本只控姿态；ControlNet OpenPose 约束骨架；自有素材训练 LoRA 固化 house style；把定稿提示词、调色板、负面词固化成风格卡。

**工作流**：定风格锁世代调色板分辨率 → 单张定稿 idle → 以定稿为参考图枚举姿态逐帧生成挑优 → 去背景、量化到固定调色板（索引 PNG）、手修边缘消除帧间漂移 → 统一帧尺寸 + 2 px padding 按动画分行排布导出 PNG → 入 Godot 切片试播 → 至少 5 个精灵放真实场景 QA（浏览器预览不等于场景内效果，核对帧对齐、缩放、时序、碰撞盒比例）。记录每批资产的提示词、参考图来源、工具与许可条款，商用前核对平台服务条款。

## 陷阱清单

- **模糊**：过滤不是 Nearest（两处都要改）或开了 mipmap。
- **渗色**：帧间无 padding，缩放后相邻帧像素泄漏。
- **帧错位**：横竖网格数与实际不符。
- **抖动**：未开 Snap 2D Transforms to Pixel 或未用整数缩放。
- **play 无反应**：动画名不匹配、FPS 为 0、未勾 Loop。
- **只加纹理不动**：帧必须加进 SpriteFrames 资源，光给节点贴纹理不会动。
- **XML/JSON 不生效**：无对应 importer/解析器，Godot 不自动转换。
- **AI 素材直接上线**：未清洗、未量化调色板、未引擎内测试。
- **合批失效**：精灵散落多张纹理导致 draw call 暴涨。

## 延伸阅读

- 官方：[2D sprite animation](https://docs.godotengine.org/en/stable/tutorials/2d/2d_sprite_animation.html)、[Sprite2D](../builtin/class_sprite2d.md)、[SpriteFrames](../builtin/class_spriteframes.md)
- 本站：[动画消费路径](01-animation-paths.md)、[物理与渲染预算](../perf/04-physics-render.md)
