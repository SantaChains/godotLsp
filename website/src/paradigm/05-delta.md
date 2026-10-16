# delta 与时间

**每帧的时间间隔叫 delta。用 delta 做帧率无关移动。**

## delta 是什么

```gdscript
func _process(delta: float):
    # delta = 上一帧到这一帧经过的秒数
    # 60 FPS 时 delta ≈ 0.0167
    # 30 FPS 时 delta ≈ 0.0333
    # 144 FPS 时 delta ≈ 0.0069
    position.x += speed * delta
```

## 不用 delta 会怎样

```gdscript
# 错：每帧加固定值
func _process(delta):
    position.x += speed  # speed = 100

# 60 FPS：每秒移动 6000 像素
# 144 FPS：每秒移动 14400 像素（快了一倍多）
```

帧率越高，同样代码跑得越快。这就是为什么必须乘 delta。

## _process vs _physics_process

| 回调 | 频率 | delta 值 | 用途 |
|---|---|---|---|
| `_process(delta)` | 每渲染帧（可变） | 可变 | 动画、视觉、输入 |
| `_physics_process(delta)` | 固定 60Hz | 固定 1/60 | 移动、碰撞、物理 |

> 值得注意的是：**移动放 `_physics_process`，视觉放 `_process`**。物理帧固定频率，保证不同设备物理行为一致。渲染帧可变，适合做插值和动画。

## Time API

```gdscript
# 获取系统时间
var now := Time.get_ticks_msec()      # 毫秒
var now_usec := Time.get_ticks_usec() # 微秒

# 倒计时
var timer := 3.0
func _process(delta):
    timer -= delta
    if timer <= 0:
        print("时间到")
```

## SceneTreeTimer

```gdscript
# 等 2 秒后执行
await get_tree().create_timer(2.0).timeout
print("2 秒过去了")

# 不用手动记时间，一行搞定
```

## 时间缩放

```gdscript
Engine.time_scale = 0.5  # 慢动作，所有 delta 减半
Engine.time_scale = 0.0  # 暂停，delta = 0
Engine.time_scale = 2.0  # 快进
```

`time_scale` 同时影响 `_process` 和 `_physics_process` 的 delta，Tween 也受影响。

## 延伸阅读

- 本站：[节点生命周期](02-node.md)、[游戏循环与时间](../algo/01-loop.md)
- 官方：[Time](https://docs.godotengine.org/en/stable/classes/class_time.html)
