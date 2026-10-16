# Godot 与 Blender 搭配管线

**首选 glTF 2.0：它是 Godot 唯一完整支持网格、骨骼、动画、PBR 材质的交换格式。** Blender 官方导出器即可，无需 FBX 转换工具。

## 格式怎么选

| 格式 | 结论 | 原因 |
|---|---|---|
| glTF 2.0 `.glb`（二进制） | 推荐默认 | 网格+贴图打包成单文件，体积小，材质自动生成 |
| glTF `.gltf` + `.bin` + 贴图 | 版本控制用 | 场景描述是文本，diff 可读；纹理单独管理 |
| `.blend` 直读 | 单人快速迭代 | 编辑器调 Blender 无头导出 glTF，见下 |
| FBX（ufbx） | 接收现成资源 | 4.3+ 默认 ufbx；旧 FBX2glTF 方案不再推荐 |
| OBJ | 仅静态无骨骼道具 | 无骨架、无动画、无 PBR |
| DAE/COLLADA | 不用 | Blender 内置导出对引擎支持差 |

### .blend 直读的前提与限制

- 本机装 Blender（3.5+，用 blender.org 官方版，避开发行版打包/Flatpak 的沙箱限制）；未自动识别时在 Editor Settings > Filesystem > Import > Blender > Blender Path 手填。
- 工作流：Blender 保存 → alt-tab 回 Godot → 编辑器聚焦即自动重导，所见即所得。
- 代价：**团队每个人都得装 Blender**；Android/Web 编辑器无法调用外部程序，此路不通；要分发或 CI 就提交导出好的 `.glb`。

## 坐标系约定（最容易出错的一步）

Godot 是右手系、Y 轴向上，相机朝 **-Z**（同 OpenGL）：

| 用途 | Godot 方向 | Blender 对应 |
|---|---|---|
| 有朝向资产（角色）正面 | +Z（正对相机） | -Y 为前，+Y 为后 |
| 地图方位 | +X 东、-X 西、+Z 南、-Z 北 | +Y 北、-Y 南 |

- 角色建模时让模型面朝 Blender 的 -Y，导入 Godot 后面朝 +Z，`look_at` 才正确。
- 代码里让带朝向的物体对准目标用 `look_at(target, use_model_front = true)`，局部空间计算用 `Vector3.MODEL_*` 常量。

## Blender 导出前检查清单

1. **Apply All Transforms**：选中物体 Ctrl + A > All Transforms，锁定位置/旋转/缩放，避免导入后缩放错乱、骨骼偏移。
2. **场景单位缩放 1.0**：Blender Scene Properties > Unit Scale = 1.0，否则模型在 Godot 里微米级或行星级。
3. **三角化**：加 Triangulate 修改器并在导出对话框勾 Apply Modifiers；N-gon 让引擎自己三角化结果不可预测。
4. **骨架归位 T-pose / Rest Pose**：导出瞬间骨骼不能处在变形姿态。
5. **材质开 Backface Culling**：Blender 默认关闭背面剔除，导出会让 Godot 材质 Cull Mode = Disabled，背面照画浪费性能。
6. Shape keys（形变目标）：导出 Data > Armature > Export Deformation Bones Only = Enabled，否则多余骨骼导致着色错误。
7. Blender < 3.2 不导出发光贴图，Emission 需单独带入。

导出对话框：Formats = glTF Binary(.glb)；Include > Selected Objects；Data > Mesh 勾 Apply Modifiers；Animation 勾需要的动作。

## 材质：Principled BSDF 直通

Blender 的 Principled BSDF 与 Godot PBR 几乎一对一映射，纹理随 `.glb` 打包，导入后生成 `StandardMaterial3D`：

| Blender 输入 | Godot |
|---|---|
| Base Color | Albedo |
| Metallic / Roughness | Metallic / Roughness |
| Normal | Normal Map |
| Emission | Emission |
| Occlusion | AO 通道 |

外部 PBR 绘制工具（Substance、开源 ArmorPaint、Material Maker）导出的金属-粗糙度贴图组同样可在 Godot 重新挂接。**灯光不要从 Blender 带过来**：各引擎光照算法不同，进 Godot 后在场景里重新打，效果才准。

## 动画

- 仅关键帧（位移/旋转/缩放）、Shape keys、Armature 骨骼蒙皮能导出；程序化约束需先烘焙。
- 要进 glTF 的动作须是该对象的 active Action，或压成单条 NLA strip；多个动作分别导出或在 NLA 切段。
- 循环、根骨骼缩放等在 Godot 的 Import dock（Animations 页签）设置，不要指望文件名后缀。

## Godot Import dock 关键项

| 设置 | 作用 |
|---|---|
| Import As = Scene | 作为可实例化场景，而非裸 Mesh |
| Apply Root Scale | 根缩放烘焙进网格/动画，根节点保持 (1,1,1)，后续加子节点不被二次缩放 |
| Animations > Loop | 勾选 idle/walk 等循环 clip |
| 重定向（Retarget） | 多角色共享动作时在此映射骨骼 |

导入设置存在 `.import` 旁的覆盖资源里，改完 Reimport；场景被多处实例化时，美术改 `.glb` 后全局生效。

## 日常迭代循环

单人：`.blend` 留在工程里，保存即重导，不提交中间文件。
团队/CI：Blender 导出 `.glb` 进版本库，`.blend` 作为源文件另存美术仓库，工程不强制装 Blender。

## 延伸阅读

- 官方：[Available 3D formats](https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_3d_scenes/available_formats.html)、[Model export considerations](https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_3d_scenes/model_export_considerations.html)
- 本站：[项目设置](01-project-settings.md)、[动画消费路径](../assets/01-animation-paths.md)
