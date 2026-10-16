# 空间数据结构

**O(n²) 碰撞对查询是万级实例的灾难。Hash grid 把邻域查询降到接近 O(1)。**

## 为什么需要

1000 个物体两两检测 = 50 万次比较。用 hash grid 只查邻接 9 格，接近 O(n)。

## 常见结构

| 结构 | 原理 | 适用 |
|---|---|---|
| Hash Grid | 按格子分桶，只查邻格 | 2D 均匀分布 |
| Quadtree | 递归四分空间 | 2D 不均匀 |
| BVH | 递归二叉树包围体 | 3D 渲染/碰撞 |
| Octree | 递归八分空间 | 3D 均匀 |

## Hash Grid 实现（2D）

```gdscript
const CELL_SIZE := 64.0  # 格子边长，略大于物体直径

var grid := {}  # key = "cx,cy"，value = 节点数组

func _physics_process(_delta):
    grid.clear()
    # 1. 所有物体入桶
    for body in all_bodies:
        var cell := Vector2i(
            floor(body.global_position.x / CELL_SIZE),
            floor(body.global_position.y / CELL_SIZE)
        )
        grid[cell] = grid.get(cell, []) + [body]

func query_neighbors(pos: Vector2) -> Array:
    # 2. 只查 3x3 邻格
    var center := Vector2i(
        floor(pos.x / CELL_SIZE),
        floor(pos.y / CELL_SIZE)
    )
    var result := []
    for dx in [-1, 0, 1]:
        for dy in [-1, 0, 1]:
            var cell := center + Vector2i(dx, dy)
            if grid.has(cell):
                result.append_array(grid[cell])
    return result
```

## Godot 内部

Godot 物理引擎（PhysicsServer2D/3D）内部就是这些结构。你写的 `intersect_shape()` 查询底层走加速结构，不用自己造。

## 什么时候自己造

> 值得注意的是：**boids 上百实例的邻域查询**是典型场景。如果每帧遍历所有物体算距离，O(n²) 卡顿。自己维护 hash grid，邻域查询 O(1)。游戏逻辑层面的群体模拟（非物理碰撞）才需要。

## 延伸阅读

- 本站：[几何检测](../math/04-collision.md)
- Red Blob Games：[Grids](https://www.redblobgames.com/x/1842/)
