# 向量实战

**Vector2/Vector3 是 Godot 一切位置、方向、速度的基础。点积判朝向，叉积判左右。**

## 核心 API

| 操作 | Godot 写法 | 数学含义 |
|---|---|---|
| 长度 | `v.length()` | 欧氏距离 |
| 长度平方 | `v.length_squared()` | 免开方，比较大小用 |
| 归一化 | `v.normalized()` | 单位向量（长度 1） |
| 点积 | `a.dot(b)` | cos(夹角) × |a| × |b| |
| 叉积 | `a.cross(b)` | 2D 返回标量，判左右 |
| 距离 | `a.distance_to(b)` | |a-b| |
| 角度 | `a.angle_to(b)` | 有符号夹角（弧度） |
| 旋转 | `v.rotated(angle)` | 返回旋转后的新向量 |
| 朝目标移动 | `a.move_toward(b, delta)` | 朝 b 走 delta 距离，不超过 |
| 方向 | `a.direction_to(b)` | 归一化的 a→b 向量 |
| 投影 | `a.project(b)` | a 在 b 上的投影 |

## 点积判朝向

```gdscript
# 敌人面向玩家吗？
var to_player := (player.global_position - global_position).normalized()
var facing := transform.x  # Node2D 的右方向
var dot := facing.dot(to_player)
if dot > 0.5:
    # dot > 0.5 ≈ 玩家在前方 60 度内
    alert()
```

dot 值的几何意义：

| dot 值 | 含义 |
|---|---|
| = 1 | 完全同向 |
| > 0 | 夹角小于 90 度（前方） |
| = 0 | 垂直 |
| < 0 | 夹角大于 90 度（后方） |
| = -1 | 完全反向 |

## 叉积判左右（2D）

```gdscript
# 目标在我的朝向哪一侧？
var to_player := player.global_position - global_position
var cross := transform.x.cross(to_player)
if cross > 0:
    turn_right()
elif cross < 0:
    turn_left()
```

2D 叉积返回标量，正负区分目标在朝向的哪一侧。注意 Godot 2D 的 Y 轴向下，左右符号与直觉相反：`cross > 0` 对应屏幕下方（朝 +X 时即角色右侧），`cross < 0` 对应屏幕上方（左侧）；自己项目里先打印一次确认，不要背符号。

## 距离族

| 距离 | 公式 | Godot |
|---|---|---|
| 欧氏 | sqrt(dx² + dy²) | `distance_to()` |
| 平方 | dx² + dy² | `distance_squared_to()`（免开方） |
| 曼哈顿 | |dx| + |dy| | `abs(a.x-b.x) + abs(a.y-b.y)` |

> 值得注意的是：**比较距离远近用 `distance_squared_to()`**，避免 sqrt 开销。只有需要实际距离数值时才用 `distance_to()`。

## 常用移动模式

```gdscript
# 朝玩家匀速移动
var dir := global_position.direction_to(player.global_position)
velocity = dir * speed

# 平滑转向目标角度
var target_angle := global_position.angle_to_point(player.global_position)
rotation = lerp_angle(rotation, target_angle, 10.0 * delta)

# 朝目标走但不超过（追击不越过）
position = position.move_toward(player.position, speed * delta)
```

## 延伸阅读

- 本站：[变换与四元数](03-transform.md)、[2D/3D 空间](../paradigm/06-space.md)
- 官方：[Vector2](https://docs.godotengine.org/en/stable/classes/class_vector2.html)
