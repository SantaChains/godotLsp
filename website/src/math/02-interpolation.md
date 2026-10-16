# 插值家族

**lerp 是最基础的插值：a + (b-a) * t。Godot 的 Tween 内部全靠它。**

## 基础函数

```gdscript
# lerp：线性插值
# t=0 → a，t=1 → b，t=0.5 → 中间
var x = lerp(100, 200, 0.5)  # 150

# inverse_lerp：反求 t
# 已知 v 在 a-b 之间，求 t
var t = inverse_lerp(100, 200, 150)  # 0.5

# remap：区间映射
# 把 [0,100] 的值映射到 [0,1]
var normalized = remap(50, 0, 100, 0, 1)  # 0.5

# clamp：截断
var v = clamp(150, 0, 100)  # 100
```

## Color 插值

```gdscript
# 从红渐变到绿
var c = Color.lerp(Color.RED, Color.GREEN, 0.5)  # 黄色
```

## Tween：自动插值

```gdscript
# 创建 Tween
var tween := create_tween()

# 1 秒内把 position.x 从当前值移到 500
tween.tween_property($Sprite2D, "position:x", 500, 1.0) \
    .set_trans(Tween.TRANS_SINE) \
    .set_ease(Tween.EASE_IN_OUT)

# 链式：先移到 (500, 0)，再旋转 360 度
tween.tween_property($Sprite2D, "position", Vector2(500, 0), 1.0)
tween.tween_property($Sprite2D, "rotation", TAU, 1.0)
```

## 缓动曲线

| TRANS 常量 | 效果 |
|---|---|
| TRANS_LINEAR | 匀速 |
| TRANS_SINE | 平滑正弦 |
| TRANS_QUAD | 二次方 |
| TRANS_CUBIC | 三次方 |
| TRANS_ELASTIC | 弹性回弹 |
| TRANS_BACK | 轻微过冲 |
| TRANS_BOUNCE | 弹跳 |

| EASE 常量 | 效果 |
|---|---|
| EASE_IN | 慢启动，加速 |
| EASE_OUT | 快启动，减速 |
| EASE_IN_OUT | 两端慢，中间快 |

## 贝塞尔曲线

```gdscript
# Curve2D：2D 路径
var curve := Curve2D.new()
curve.add_point(Vector2(0, 0))
curve.add_point(Vector2(100, 100))
curve.add_point(Vector2(200, 0))

# 取曲线上 t 位置的点
var pos := curve.sample_baked(0.5)  # 中间点
```

相机路径、技能轨迹用这个。

## 延伸阅读

- 本站：[变换与四元数](03-transform.md)
- 官方：[Tween](https://docs.godotengine.org/en/stable/classes/class_tween.html)
