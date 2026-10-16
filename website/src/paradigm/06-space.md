# 2D/3D 空间

**Node2D/Node3D 有 position、rotation、scale。这些是局部变换。**

## 坐标系

| 坐标 | 含义 |
|---|---|
| 局部坐标 | 相对于父节点 |
| 全局坐标 | 相对于世界原点 |
| 屏幕坐标 | 相对于屏幕左上角 |

```gdscript
# 局部 → 全局
var global_pos = global_position  # 世界坐标

# 全局 → 局部
var local_pos = to_local(global_pos)
```

## 变换层级

```
World
└── Node2D (position: 100, 0)
    └── Child (position: 50, 0)
        # Child 的全局位置是 (150, 0)
```

移动父节点，子节点跟着移动。这就是**坐标继承**。

## 2D 特有

- `z_index`：绘制层级，越大越在前。
- `rotation`：弧度，绕 Z 轴。
- `scale`：(1,1) 原始大小。

## 延伸阅读

- 本站：[数学篇 · 向量](../math/01-vectors.md)
- 官方：[2D transforms](https://docs.godotengine.org/en/stable/tutorials/2d/index.html)
