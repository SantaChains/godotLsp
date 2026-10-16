# 信号系统

**信号是 Godot 的观察者模式**：节点 A 声明「发生了什么」，任意多的订阅者收到通知——**A 不需要知道 B 存在**。这是解耦节点间通信的唯一正道，也是 [范式篇 · 三大支柱](../paradigm/01-pillars.md)之一。

## 声明与发射

```gdscript
signal health_changed              # 无参
signal died(cause: String)         # 带参（参数名进编辑器 Signals 面板）

func take_damage(amount: int) -> void:
    hp -= amount
    health_changed.emit(hp)        # 4.x 首选：直接在信号上 emit
    if hp <= 0:
        died.emit("combat")
```

`emit()` 的参数个数**不必与声明一致**（声明只服务编辑器生成回调），但请自觉对齐——错位的参数是隐晦 bug 的温床。`emit_signal("health_changed", hp)` 是 3.x 遗留写法，等价但不再推荐。

## 连接与断开

```gdscript
func _ready() -> void:
    health_changed.connect(_on_health_changed)   # 连接本类方法
    died.connect(get_node("/root/GameOver").show) # 连接其他节点的方法

func _on_health_changed(new_hp: int) -> void:
    bar.value = new_hp
```

- `connect(callable)` 返回 `Error`（重复连接同一对返回 `ERR_INVALID_PARAMETER`，**不会自动去重**）。
- 断开：`signal.disconnect(callable)`；查询：`signal.is_connected(callable)`。
- **连接生命周期**：连接双方都在场景树内时，任一方销毁自动断连——**同树场景不用手写清理**。野引用主要来自三类：跨树连接、`Callable.bind` 持有的外部引用、编辑器连接的 lost-connection 警告。深入语义见[范式篇 · 信号深入](../paradigm/04-signals.md)。

## bind 与参数预绑

回调需要的上下文超出信号给的参数时，用 `Callable.bind()` 预绑：

```gdscript
for i in button_count:
    var btn := buttons[i]
    btn.pressed.connect(_on_button.bind(i))   # 回调多收到一个 i

func _on_button(index: int) -> void:
    select(index)
```

绑定值追加在**信号参数之后**。lambda 捕获是另一种解法，但 lambda 按值捕获循环变量时只捕获**当时**的值——两者语义差异务必分辨。

## CONNECT_* 连接标志

```gdscript
timer.timeout.connect(spawn, CONNECT_ONE_SHOT)   # 触发一次自动断开
area.body_entered.connect(on_hit, CONNECT_DEFERRED)  # 推迟到帧末执行
```

| 标志 | 语义 |
|---|---|
| `CONNECT_DEFERRED` | 推迟到当前帧末（空闲时）调用；**物理回调里改场景树必加** |
| `CONNECT_PERSIST` | 连接随场景保存（编辑器连接的默认） |
| `CONNECT_ONE_SHOT` | 首次触发后自动断开 |
| `CONNECT_REFERENCE_COUNTED` | 重复连接按引用计数（一般不用） |

## 内置节点信号

所有节点自带的高频信号：`ready`（进入树且子树就绪）、`tree_entered` / `tree_exited`、`child_entered_tree` 等；控件类有 `pressed`、`text_changed`；物理体有 `body_entered`……全表查各节点类页的「信号」节，如 [Node · 信号](../builtin/class_node.md)、[BaseButton · pressed](../builtin/class_basebutton.md)。

> 值得注意的是：`_ready()` 回调与 `ready` 信号是同一件事的两个入口——**覆写 `_ready` 与 connect `ready` 不要同时做**，选一个。

## 编辑器连接 vs 代码连接

编辑器里从节点 Signals 面板拖线生成 `.tscn` 内的 `[connection]` 声明（引擎加载时自动 connect）；代码连接适合动态对象（子弹、临时 UI）。**同一对信号-方法别两处都连**，会触发两次回调。引擎如何存储与解析连接声明见[引擎内部篇 · 信号连接的存储](../internals/04-connections.md)。

## 选型：信号 vs 直接调用

| 场景 | 用什么 |
|---|---|
| 发射方不该知道接收方（成就系统、UI、音效） | 信号 |
| 一对一强耦合的父子（`Player` 调自己的 `AnimationPlayer.play()`） | 直接调用 |
| 每帧都要同步的连续数据 | 直接调用或共享状态，**信号每帧发射是反模式** |

## 延伸阅读

- 官方英文文档：[Using signals](https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html)
- 类参考锚点：[Signal](../builtin/class_signal.md)、[Callable](../builtin/class_callable.md)
- 本站：[await 与协程](12-await.md)（下一步——信号还能被 await）
