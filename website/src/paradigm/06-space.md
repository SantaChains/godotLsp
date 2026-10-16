# 2D/3D 空间

**Node2D/Node3D 的 position、rotation、scale 是局部变换。理解坐标系就理解了一切。**

## 三种坐标

| 坐标 | 含义 | 获取方式 |
|---|---|---|
| 局部坐标 | 相对于父节点 | `position` |
| 全局坐标 | 相对于世界原点 | `global_position` |
| 屏幕坐标 | 相对于屏幕左上角 | 需通过 Camera 转换 |

```gdscript
# 局部 → 全局
var world_pos = global_position

# 全局 → 局部
var local_pos = to_local(world_pos)

# 屏幕 → 世界（用相机）
var world_mouse = get_global_mouse_position()
```

## 变换层级

```
World
└── Node2D (position: 100, 0, rotation: 0)
    └── Child (position: 50, 0)
        # Child 的 global_position = (150, 0)
```

移动父节点，子节点跟着移动。旋转父节点，子节点绕父节点转。缩放父节点，子节点被拉伸。这就是坐标继承。

## 方向向量

Node2D 的 `transform` 有两个基向量：

```gdscript
# Node2D
var right := transform.x  # 右方向（单位向量）
var down := transform.y   # 下方向（单位向量）

# 让角色朝右前方移动
var move_dir := (transform.x + Vector2(0, 1)).normalized()
```

```gdscript
# Node3D
var forward := -transform.basis.z  # 前方向（Godot 朝 -Z）
var right := transform.basis.x     # 右方向
```

## rotation vs rotation_degrees

```gdscript
# 弧度
rotation = PI  # 180 度

# 角度（更直观）
rotation_degrees = 180.0
```

编辑器里显示的是角度，代码里用弧度。`rotation_degrees` 是便捷属性。

## z_index（2D 特有）

```gdscript
# z_index 决定绘制顺序
# 值越大越靠前
sprite.z_index = 100  # 画在所有 z_index < 100 的上面
```

> 值得注意的是：节点在场景树中的顺序也影响绘制顺序。同级节点，下面的先画（被上面的盖住）。z_index 是全局覆盖，不受场景树顺序限制。

## look_at

```gdscript
# Node2D：让本节点的 -Y 方向指向目标
look_at(target_position)

# Node3D：让本节点的 -Z 方向指向目标
look_at(target_position)
```

## 延伸阅读

- 本站：[向量实战](../math/01-vectors.md)、[变换与四元数](../math/03-transform.md)
- 官方：[2D transforms](https://docs.godotengine.org/en/stable/tutorials/2d/index.html)
