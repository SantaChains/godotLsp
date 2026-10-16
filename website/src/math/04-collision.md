# 几何检测

**Raycast 是最常用的几何检测。Godot 物理引擎底层用 SAT。**

## Raycast 原理

参数化射线：`P(t) = origin + direction * t`，t ∈ [0, max_distance]。遍历场景中物体，求 t 最小的交点。

## RayCast2D 节点

编辑器里拖一个 RayCast2D 节点，画一条射线。代码里：

```gdscript
# RayCast2D 节点自动检测
func _physics_process(_delta):
    if $RayCast2D.is_colliding():
        var collider = $RayCast2D.get_collider()
        print("撞到: ", collider.name)
```

| 属性 | 作用 |
|---|---|
| `target_position` | 射线终点（局部坐标），向量长度就是探测距离 |
| `collision_mask` | 检测哪些层 |
| `enabled` | 是否启用 |
| `exclude_parent` | 是否自动排除挂载它的父节点 |

## 代码查询（不挂节点）

```gdscript
# 直接查物理空间，不需要 RayCast2D 节点
var space := get_world_2d().direct_space_state
var query := PhysicsRayQueryParameters2D.new()
query.from = global_position
query.to = global_position + Vector2.RIGHT * 100
query.collision_mask = 1  # 只查 layer 1
query.exclude = [self]     # 排除自己

var hit := space.intersect_ray(query)
if hit:
    print("撞到: ", hit.collider.name, " 位置: ", hit.position)
```

## 碰撞层与掩码

```
collision_layer = 我是谁（哪些层）
collision_mask  = 我检测谁（查哪些层）
```

| 层 | 用途 |
|---|---|
| Layer 1 | 玩家 |
| Layer 2 | 地图/墙壁 |
| Layer 3 | 敌人 |
| Layer 4 | 触发器 |

玩家 `layer=1, mask=2`：玩家是 layer 1，检测 layer 2（墙壁）。

## SAT 分离轴定理

> 值得注意的是：Godot 2D 物理引擎用 SAT 判断凸多边形碰撞。核心思想：如果存在一条轴，两个物体在这条轴上的投影不重叠，则不碰撞。只需检查有限条轴（多边形边的法线）。

## Godot API 速查

| 检测 | API | 节点 |
|---|---|---|
| 射线 | `intersect_ray()` | RayCast2D |
| 形状 | `intersect_shape()` | ShapeCast2D |
| 点 | `intersect_point()` | 无 |
| 移动碰撞 | `move_and_collide()` / `move_and_slide()` | CharacterBody2D |

## 延伸阅读

- 本站：[物理与渲染预算](../perf/04-physics-render.md)
- 官方：[Physics2D](https://docs.godotengine.org/en/stable/tutorials/physics/index.html)
