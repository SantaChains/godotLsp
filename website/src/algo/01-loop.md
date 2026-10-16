# 游戏循环与时间

**物理用固定步长推进，渲染用可变步长呈现，两者解耦。这是 Glenn Fiedler "Fix Your Timestep" 的核心，Godot 用 `_physics_process` 与 `_process` 两条回调直接落地。**

## 两条回调，两种时钟

| 回调 | 步长 | 用途 |
|---|---|---|
| `_physics_process(delta)` | 固定（默认 1/60s，由 `Engine.physics_ticks_per_second` 决定） | 移动、碰撞、AI、状态推进等确定性逻辑 |
| `_process(delta)` | 可变（每渲染帧一次，随帧率波动） | 纯视觉：补间、镜头跟随、特效、UI 反馈 |

`delta` 在物理回调里是常量 1/60（缩放 time_scale 后变），在渲染回调里是真实帧间隔。

## 为什么物理必须固定步长

若直接用 `_process` 的可变 delta 积分位置：帧率高的机器一秒走的步多、帧率低的机器走得少，同一输入在 60 FPS 和 144 FPS 下位移不一致，物理结果不可复现，网络同步与回放更无从谈起。固定步长把"逻辑推进"和"屏幕刷新"分开：逻辑每秒精确跑 N 次，屏幕每秒刷新多少次都行。

## Fiedler 的累积器模型

经典手写循环（引擎内部思想，你不必自己实现）：

```text
accumulator += 经过的真实时间
while accumulator >= fixed_dt:
    physics_update(fixed_dt)      # 补够欠账的固定步
    accumulator -= fixed_dt
render(accumulator / fixed_dt)    # 用剩余比例做渲染插值
```

帧率高于物理频率时一帧补多次物理，低于时跳过，渲染再用累积比例平滑。

## Godot 的实现：物理插值

Godot 3.5+/4.x 内置物理插值，不必手写累积器。开启 Project Settings > Physics > Common > Physics Interpolation（`physics/common/physics_interpolation`）后：

- 引擎为每个可插值节点保存**上一物理步**与**当前物理步**的变换；
- 渲染帧落在两个物理步之间时，按比例自动插值显示；
- 逻辑仍只写在 `_physics_process`，高刷屏上运动不再一卡一卡。

三条使用约束：

1. 位移/速度逻辑只放 `_physics_process`，别在 `_process` 里改物理对象变换，否则插值基准错乱。
2. **传送/瞬间跳转**后调用 `reset_physics_interpolation()`，否则引擎会从旧位置插值滑过去。
3. 单个节点可用 `physics_interpolation_mode` 关闭插值（继承默认），整棵子树跟随父节点。

```gdscript
extends CharacterBody2D
const SPEED := 200.0

func _physics_process(delta: float) -> void:
    velocity = Input.get_vector("left", "right", "up", "down") * SPEED
    move_and_slide()

func teleport_to(p: Vector2) -> void:
    global_position = p
    reset_physics_interpolation()  # 关键：禁止从旧位置插值滑行
```

## 时间缩放

```gdscript
Engine.time_scale = 0.5   # 慢动作，物理与渲染 delta 同时减半
Engine.time_scale = 0.0   # 暂停：delta 归 0，_process 仍被调用但不推进
Engine.time_scale = 1.0   # 恢复
```

`time_scale` 同时作用于 `_process` 与 `_physics_process` 的 delta，Tween、物理一并减速；要独立的 UI 动画不受慢动作影响，用不受 time_scale 的计时或单独标记。

## 易错点

- 在 `_process` 里 `position += velocity * delta` 做移动：帧率不同结果不同，且与物理插值冲突。
- 改了 `physics_ticks_per_second` 却按旧 delta 心算速度：速度单位是"像素/秒"，用 `velocity * delta` 而不是写死每帧增量。
- 暂停只用 `Engine.time_scale = 0` 却忘了停止需要继续的 UI 计时器：区分世界时间与 UI 时间。

## 延伸阅读

- 本站：[delta 与时间](../paradigm/05-delta.md)
- 官方：[Using physics interpolation](https://docs.godotengine.org/en/stable/tutorials/physics/interpolation/using_physics_interpolation.html)
- 原文：[Gaffer on Games - Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/)
