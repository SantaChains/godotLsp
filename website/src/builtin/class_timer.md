# Timer

**继承**：[Node](class_node.md) → [Object](class_object.md) → **Timer**

> 计时器节点。等待指定时间后发射 timeout 信号。

<details><summary>官方英文描述</summary>

The Timer node is a countdown timer and is the simplest way to handle time-based logic in the engine. When a timer reaches the end of its wait_time, it will emit the timeout signal.

After a timer enters the scene tree, it can be manually started with start(). A timer node is also started automatically if autostart is true.

Without requiring much code, a timer node can be added and configured in the editor. The timeout signal it emits can also be connected through the Signals dock in the editor:

```gdscript
func _on_timer_timeout():
    print("Time to attack!")
```

Note: To create a one-shot timer without instantiating a node, use SceneTree.create_timer().

Note: Timers are affected by Engine.time_scale unless ignore_time_scale is true. The higher the time scale, the sooner timers will end. How often a timer processes may depend on the framerate or Engine.physics_ticks_per_second.

</details>

## 方法

### get_time_left {#get_time_left}

```gdscript
float get_time_left() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_timer_process_callback {#get_timer_process_callback}

```gdscript
Timer.TimerProcessCallback get_timer_process_callback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wait_time {#get_wait_time}

```gdscript
float get_wait_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_autostart {#has_autostart}

```gdscript
bool has_autostart() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_ignoring_time_scale {#is_ignoring_time_scale}

```gdscript
bool is_ignoring_time_scale()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_one_shot {#is_one_shot}

```gdscript
bool is_one_shot() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_paused {#is_paused}

```gdscript
bool is_paused() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_stopped {#is_stopped}

```gdscript
bool is_stopped() const
```

Returns true if the timer is stopped or has not started.

### set_autostart {#set_autostart}

```gdscript
void set_autostart(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ignore_time_scale {#set_ignore_time_scale}

```gdscript
void set_ignore_time_scale(ignore: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_shot {#set_one_shot}

```gdscript
void set_one_shot(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_paused {#set_paused}

```gdscript
void set_paused(paused: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_timer_process_callback {#set_timer_process_callback}

```gdscript
void set_timer_process_callback(callback: Timer.TimerProcessCallback)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wait_time {#set_wait_time}

```gdscript
void set_wait_time(time_sec: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### start {#start}

```gdscript
void start(time_sec: float = -1)
```

启动计时器。start(time) 或 start()。

### stop {#stop}

```gdscript
void stop()
```

停止计时器。

## 属性

### process_callback {#process_callback}

```gdscript
process_callback: int
```

- void set_timer_process_callback(value: TimerProcessCallback)
- TimerProcessCallback get_timer_process_callback()

Specifies when the timer is updated during the main loop.

### wait_time {#wait_time}

```gdscript
wait_time: float
```

等待时间（秒）。

### one_shot {#one_shot}

```gdscript
one_shot: bool
```

- void set_one_shot(value: bool)
- bool is_one_shot()

If true, the timer will stop after reaching the end. Otherwise, as by default, the timer will automatically restart.

### autostart {#autostart}

```gdscript
autostart: bool
```

- void set_autostart(value: bool)
- bool has_autostart()

If true, the timer will start immediately when it enters the scene tree.

Note: After the timer enters the tree, this property is automatically set to false.

Note: This property does nothing when the timer is running in the editor.

### paused {#paused}

```gdscript
paused: bool
```

- void set_paused(value: bool)
- bool is_paused()

If true, the timer is paused. A paused timer does not process until this property is set back to false, even when start() is called. See also stop().

### ignore_time_scale {#ignore_time_scale}

```gdscript
ignore_time_scale: bool
```

- void set_ignore_time_scale(value: bool)
- bool is_ignoring_time_scale()

If true, the timer will ignore Engine.time_scale and update with the real, elapsed time.

### time_left {#time_left}

```gdscript
time_left: float
```

剩余时间。

## 信号

### timeout {#timeout}

```gdscript
signal timeout()
```

Emitted when the timer reaches the end.

## 枚举

### TimerProcessCallback {#enum-TimerProcessCallback}

```gdscript
enum TimerProcessCallback { TIMER_PROCESS_PHYSICS = 0, TIMER_PROCESS_IDLE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TIMER_PROCESS_PHYSICS {#TIMER_PROCESS_PHYSICS}

```gdscript
TIMER_PROCESS_PHYSICS = 0
```

Update the timer every physics process frame (see Node.NOTIFICATION_INTERNAL_PHYSICS_PROCESS).

#### TIMER_PROCESS_IDLE {#TIMER_PROCESS_IDLE}

```gdscript
TIMER_PROCESS_IDLE = 1
```

Update the timer every process (rendered) frame (see Node.NOTIFICATION_INTERNAL_PROCESS).

---
