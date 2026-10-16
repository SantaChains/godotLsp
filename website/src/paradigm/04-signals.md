# 信号深入

**信号 = 节点声明"我发生了什么"，其他节点连接后响应。发送方不知道接收方是谁。**

## 解耦本质

```
Player.gd:
signal died(cause: String)  # 声明信号
func take_damage():
    died.emit("shot")       # 发射信号，不关心谁监听

HUD.gd:
func _ready():
    player.died.connect(_on_player_died)  # HUD 连接
func _on_player_died(cause: String):
    show_game_over(cause)
```

Player 不知道 HUD 的存在。HUD 自己去连接。这就是**事件驱动解耦**。

## 编辑器连接 vs 代码连接

| 方式 | 适用场景 |
|---|---|
| 编辑器连接（.tscn [connection] 段） | 固定连接，节点关系在编辑器中确定 |
| 代码 `connect()` | 动态连接，运行时决定连接谁 |

## 连接生命周期

> 需要注意的是：**同树节点销毁时自动断连**。如果 A 连接了 B 的信号，A 被 `queue_free()`，连接自动断开，不会出现"悬空回调"。但跨树引用（异步 lambda 捕获了已销毁节点）仍然会崩。

## 4.x 写法

```gdscript
# 4.x 推荐
my_signal.emit(arg1, arg2)
my_signal.connect(callable)

# 3.x 遗留（仍兼容但不推荐）
emit_signal("my_signal", arg1, arg2)
```

## 延伸阅读

- 本站：[语法篇 · 信号系统](../syntax/10-signals.md)
- 官方：[Signals](https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html)
