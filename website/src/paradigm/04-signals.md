# 信号深入

**信号 = 节点声明"我发生了什么"，其他节点连接后响应。发送方不知道接收方是谁。**

## 声明与发射

```gdscript
# player.gd
extends CharacterBody2D

# 声明信号，参数类型可选
signal died(cause: String)
signal health_changed(new_hp: int, max_hp: int)

var hp := 100

func take_damage(amount: int):
    hp -= amount
    health_changed.emit(hp, 100)  # 发射，带参数
    if hp <= 0:
        died.emit("dead")          # 发射，不关心谁监听
```

## 连接方式

```gdscript
# HUD.gd
func _ready():
    # 方式1：代码连接
    player.died.connect(_on_player_died)
    # 方式2：绑定额外参数
    player.health_changed.connect(_on_health_changed.bind("HUD"))

func _on_player_died(cause: String):
    show_game_over()

func _on_health_changed(new_hp: int, max_hp: int, source: String):
    $HealthBar.value = float(new_hp) / max_hp
```

## 连接标志

```gdscript
# 只触发一次，自动断开
player.died.connect(_on_died, CONNECT_ONE_SHOT)

# 延迟到下一帧调用
player.died.connect(_on_died, CONNECT_DEFERRED)

# 用 Callable 精确引用方法
player.died.connect(Callable(self, "_on_died"))
```

| 标志 | 作用 |
|---|---|
| `CONNECT_ONE_SHOT` | 触发一次后自动断开 |
| `CONNECT_DEFERRED` | 延迟到帧末调用，避免迭代中修改 |
| `CONNECT_PERSIST` | 保存到 .tscn（编辑器连接默认） |

## 生命周期：什么时候断连

> 值得注意的是：**同树节点销毁时信号自动断连**。A 连接了 B，A 被 `queue_free()`，连接自动消失。不会悬空回调。

但以下情况会崩：

```gdscript
# 危险：lambda 捕获了已销毁节点
button.pressed.connect(func(): player.take_damage(10))
# player 被 free 后，lambda 还在，点按钮就崩
```

解法：用 Callable 而不是 lambda，或手动 `disconnect()`。

## 什么时候不用信号

```gdscript
# 紧耦合：直接调用
player.health.take_damage(10)

# 松耦合：信号广播
player.died.emit("shot")
```

| 场景 | 用什么 |
|---|---|
| 明确知道谁该响应 | 直接调用 |
| 多个系统都关心（HUD/音效/掉落） | 信号 |
| 发送方不知道接收方 | 信号 |

## 延伸阅读

- 本站：[语法篇 · 信号系统](../syntax/10-signals.md)、[信号连接的存储](../internals/04-connections.md)
- 官方：[Signals](https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html)
