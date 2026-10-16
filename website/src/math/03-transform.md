# 变换与四元数

**欧拉角会万向节死锁，四元数不会。Godot 3D 内部用四元数。**

## 三种表示

| 表示 | 优点 | 缺点 |
|---|---|---|
| 矩阵 | 计算快，可拼接 | 4x4 冗余，插值难 |
| 欧拉角 | 人类可读（编辑器里看到的） | 万向节死锁 |
| 四元数 | 无死锁，插值顺滑 | 不可读（4 个数） |

## 万向节死锁

欧拉角三个轴独立旋转。当两个轴对齐时（比如俯仰 90 度），偏航和翻滚变成同一个轴，丢失一个自由度。

```
飞机俯仰 90 度 → 机头朝上
此时偏航 = 翻滚，无法独立控制
```

> 值得注意的是：Godot 编辑器里你看到的是欧拉角（`rotation_degrees`），但内部存储和插值用四元数。直接改 `rotation` 不会出问题，但自己写插值旋转必须用 `Quaternion.slerp()`。

## slerp 球面插值

```gdscript
# 从 A 的姿态平滑转到 B 的姿态
var from := Quaternion(node_a.transform.basis)
var to := Quaternion(node_b.transform.basis)

# t=0 → A，t=1 → B，t=0.5 → 中间姿态
var result := from.slerp(to, 0.5)
```

## Transform2D 实用 API

```gdscript
# Node2D 的 transform 是 Transform2D
# basis_x = transform.x（右方向）
# basis_y = transform.y（下方向）
# origin = transform.origin（位置）

# 让物体朝目标方向转
look_at(target_position)

# 局部坐标 → 世界坐标
var world_pos := to_global(local_pos)

# 世界坐标 → 局部坐标
var local_pos := to_local(world_pos)
```

## Transform3D

| 属性 | 含义 |
|---|---|
| `transform` | 局部变换（相对父节点） |
| `global_transform` | 全局变换（相对世界） |
| `transform.basis` | 旋转+缩放（3x3 矩阵） |
| `transform.origin` | 平移（位置） |
| `-transform.basis.z` | 前方向（Godot 朝 -Z） |

## 延伸阅读

- 本站：[向量实战](01-vectors.md)、[2D/3D 空间](../paradigm/06-space.md)
- 官方：[Transform3D](https://docs.godotengine.org/en/stable/classes/class_transform3d.html)、[Quaternion](https://docs.godotengine.org/en/stable/classes/class_quaternion.html)
