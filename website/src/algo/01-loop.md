# 游戏循环与时间

**固定物理步长 + 可变渲染步长。这是 Gaffer "Fix Your Timestep" 的核心。**

## Godot 的映射

| 回调 | 步长 | 对应 Gaffer 概念 |
|---|---|---|
| `_physics_process(delta)` | 固定 1/60s | 物理步长 |
| `_process(delta)` | 可变 | 渲染步长 |
| `Engine.physics_ticks_per_second` | 可调 | 物理频率 |

## 为什么固定步长

可变步长在不同帧率下物理行为不一致。固定步长保证：60 FPS 和 144 FPS 下，物体一秒移动的距离相同。

## Gaffer 模式

```
accumulator = 0
while accumulator >= fixed_dt:
    physics_update(fixed_dt)   # 固定步长
    accumulator -= fixed_dt
render(accumulator / fixed_dt) # 插值渲染
```

Godot 内部就是这么做的。你只需要：移动放 `_physics_process`，视觉放 `_process`。

## 时间缩放

```gdscript
Engine.time_scale = 0.5  # 慢动作
Engine.time_scale = 0.0  # 暂停
```

> 值得注意的是：`time_scale` 同时影响 `_process` 和 `_physics_process` 的 delta。Tween 也受影响。

## 延伸阅读

- 本站：[delta 与时间](../paradigm/05-delta.md)
- 原文：[Gaffer on Games - Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/)
