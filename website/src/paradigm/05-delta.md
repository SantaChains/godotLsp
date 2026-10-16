# delta 与时间

**每帧的时间间隔叫 delta。用 delta 做帧率无关移动。**

## delta 是什么

```gdscript
func _process(delta: float):
    # delta = 上一帧到这一帧经过的秒数
    # 60 FPS 时 delta ≈ 0.0167
    # 30 FPS 时 delta ≈ 0.0333
    position.x += speed * delta
```

不用 delta 的代码在 144Hz 屏幕上跑得比 60Hz 快一倍。

## _process vs _physics_process

| 回调 | 频率 | 用途 |
|---|---|---|
| `_process(delta)` | 每渲染帧（可变） | 动画、视觉效果、输入检测 |
| `_physics_process(delta)` | 每物理帧（固定 60Hz） | 移动、物理计算、碰撞 |

> 值得注意的是：**移动放 `_physics_process`，视觉放 `_process`**。物理帧固定频率，保证不同设备上物理行为一致。

## 时间缩放

```gdscript
Engine.time_scale = 0.5  # 慢动作
Engine.time_scale = 0.0  # 暂停
```

## 延伸阅读

- 本站：[节点生命周期](02-node.md)
- 官方：[delta](https://docs.godotengine.org/en/stable/tutorials/best_practices/gdscript_best_practices.html)
