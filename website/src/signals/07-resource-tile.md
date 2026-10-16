<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 资源 / 材质 / 瓦片 / 贴图

> 曲线、路径、TileMap 系列变更、精灵与实例贴图替换、配置资源更新。

## Curve（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal domain_changed()` | 定义域 max/min_domain 改变时触发。 | 横轴范围刷新。 [类页锚点](../builtin/class_curve.md#domain_changed) |
| `signal range_changed()` | 值域 max/min_value 改变时触发。 | 纵轴范围刷新。 [类页锚点](../builtin/class_curve.md#range_changed) |

## FoldableGroup（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal expanded(container: FoldableContainer)` | 组内某个容器被展开时触发，携带容器。 | 组状态同步。 [类页锚点](../builtin/class_foldablegroup.md#expanded) |

## GridMap（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal cell_size_changed(cell_size: Vector3)` | 单元格尺寸设置改变时触发，携带新尺寸。 | 尺寸变更重建显示。 [类页锚点](../builtin/class_gridmap.md#cell_size_changed) |
| `signal changed()` | 网格单元被设置或移除、内容修改时触发。 | 网格变更刷新。 [类页锚点](../builtin/class_gridmap.md#changed) |

## MeshInstance2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal texture_changed()` | 2D 网格实例纹理改变时触发。 | 换图。 [类页锚点](../builtin/class_meshinstance2d.md#texture_changed) |

## MultiMeshInstance2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal texture_changed()` | 多网格实例纹理改变时触发。 | 换图。 [类页锚点](../builtin/class_multimeshinstance2d.md#texture_changed) |

## Path3D（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal curve_changed()` | 曲线改变时触发。 | 路径重建。 [类页锚点](../builtin/class_path3d.md#curve_changed) |
| `signal debug_color_changed()` | 自定义调试颜色改变时触发。 | 调试显示刷新。 [类页锚点](../builtin/class_path3d.md#debug_color_changed) |

## SkeletonProfile（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal profile_updated()` | 配置文件值改变时触发。 | 配置刷新。 [类页锚点](../builtin/class_skeletonprofile.md#profile_updated) |

## Sprite2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal frame_changed()` | 当前帧改变时触发。 | 逐帧事件。 [类页锚点](../builtin/class_sprite2d.md#frame_changed) |
| `signal texture_changed()` | 纹理替换时触发。 | 换图重置。 [类页锚点](../builtin/class_sprite2d.md#texture_changed) |

## Sprite3D（继承 SpriteBase3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal frame_changed()` | 3D 精灵当前帧改变时触发。 | 逐帧事件。 [类页锚点](../builtin/class_sprite3d.md#frame_changed) |
| `signal texture_changed()` | 3D 精灵纹理替换时触发。 | 换图重置。 [类页锚点](../builtin/class_sprite3d.md#texture_changed) |

## TileData（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal changed()` | 任意瓦片属性改变时触发。 | 瓦片数据同步。 [类页锚点](../builtin/class_tiledata.md#changed) |

## TileMap（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal changed()` | TileMap 的 TileSet 改变时触发。 | 换图集重建。 [类页锚点](../builtin/class_tilemap.md#changed) |

## TileMapLayer（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal changed()` | 图层属性改变时触发。 | 图层刷新。 [类页锚点](../builtin/class_tilemaplayer.md#changed) |

<!-- SIGNALS:END -->
