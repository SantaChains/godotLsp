# 输入系统

**按「动作（action）」处理输入，永远不要硬编码按键**。键盘、手柄、触屏在动作层归一，改键位不动代码。

## 动作映射

Project Settings → Input Map 里定义动作（`move_left`、`jump`……），每个动作绑任意多个设备事件。查询全部走 `Input` 单例：

```gdscript
func _physics_process(delta: float) -> void:
    var axis := Input.get_axis("move_left", "move_right")   # -1.0 ~ 1.0
    var dir := Input.get_vector("move_left", "move_right", "move_up", "move_down")
    velocity = dir * speed

    if Input.is_action_just_pressed("jump"):    # 本物理帧刚按下
        jump()
    if Input.is_action_pressed("run"):          # 持续按住
        speed *= 2.0
```

| 查询 | 语义 |
|---|---|
| `is_action_pressed(a)` | 按住中（每帧真） |
| `is_action_just_pressed(a)` | 刚按下的那一帧 |
| `is_action_just_released(a)` | 刚松开的那一帧 |
| `get_axis(neg, pos)` | 双动作合成 -1~1 |
| `get_vector(neg_x, pos_x, neg_y, pos_y)` | 四动作合成归一化向量（斜向不超速） |

`just_pressed` 有 `_physics` 与 `_process` 两个时间基的版本（`is_action_just_pressed` 自动跟随调用处），**放错回调会丢帧**——轮询放 `_physics_process`。

## 事件回调：_input vs _unhandled_input

```gdscript
func _unhandled_input(event: InputEvent) -> void:
    if event.is_action_pressed("pause"):
        toggle_pause()

func _unhandled_key_input(event: InputEventKey) -> void:  # 仅键盘的窄化版
    pass
```

| 回调 | 收到什么 | 用途 |
|---|---|---|
| `_input(event)` | **一切输入**，最先 | 截获/吞掉输入（输入记录器） |
| `_gui_input(event)` | 仅本 Control 上 | UI 控件自己处理点击 |
| `_unhandled_input(event)` | UI 消费后剩下的 | **gameplay 输入首选**——不与按钮抢事件 |
| `_shortcut_input(event)` | 快捷键类 | 全局快捷键 |

**事件沿 `_input → GUI → _unhandled_input` 流动**，`get_viewport().set_input_as_handled()` 可在中途终止传播。

## 事件对象

```gdscript
func _unhandled_input(event: InputEvent) -> void:
    if event is InputEventMouseButton and event.pressed:
        if event.button_index == MOUSE_BUTTON_LEFT:
            shoot(event.position)     # 鼠标：按键 + 位置
    elif event is InputEventKey and event.pressed and not event.echo:
        print(event.keycode)          # 键盘：键码；echo=长按重复
```

`event.is_action_pressed("jump")` 对事件对象同样可用——**轮询（Input 单例）查状态，事件（InputEvent）拿瞬时**，混用前想清楚你要的是哪一个。

## 使用准则

- gameplay 一律 `_unhandled_input`；只有 UI 拦截类需求才用 `_input`。
- 触屏：`InputEventScreenTouch/Drag`；模拟多点触控开 `ProjectSettings emulate_mouse_from_touch`。
- 手柄死区在动作映射里配置，代码层无需处理。

## 延伸阅读

- 官方英文文档：[Inputs](https://docs.godotengine.org/en/stable/tutorials/inputs/index.html)、[InputEvent](https://docs.godotengine.org/en/stable/tutorials/inputs/inputevent.html)
- 类参考锚点：[Input](../builtin/class_input.md)、[InputEvent](../builtin/class_inputevent.md)
- 本站：[字符串格式化与惯用法](16-strings.md)（下一篇）、[场景树章 · 生命周期](11-scenetree.md#生命周期回调全表)
