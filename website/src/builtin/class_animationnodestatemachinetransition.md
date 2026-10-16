# AnimationNodeStateMachineTransition

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeStateMachineTransition**

> The path generated when using AnimationNodeStateMachinePlayback.travel() is limited to the nodes connected by AnimationNodeStateMachineTransition.

<details><summary>官方英文描述（中文精译进行中）</summary>

The path generated when using AnimationNodeStateMachinePlayback.travel() is limited to the nodes connected by AnimationNodeStateMachineTransition.

You can set the timing and conditions of the transition in detail.

</details>

## 方法

### get_advance_condition {#get_advance_condition}

```gdscript
StringName get_advance_condition() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_advance_expression {#get_advance_expression}

```gdscript
String get_advance_expression() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_advance_mode {#get_advance_mode}

```gdscript
AnimationNodeStateMachineTransition.AdvanceMode get_advance_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_priority {#get_priority}

```gdscript
int get_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_switch_mode {#get_switch_mode}

```gdscript
AnimationNodeStateMachineTransition.SwitchMode get_switch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_xfade_curve {#get_xfade_curve}

```gdscript
Curve get_xfade_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_xfade_time {#get_xfade_time}

```gdscript
float get_xfade_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_loop_broken_at_end {#is_loop_broken_at_end}

```gdscript
bool is_loop_broken_at_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_reset {#is_reset}

```gdscript
bool is_reset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_advance_condition {#set_advance_condition}

```gdscript
void set_advance_condition(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_advance_expression {#set_advance_expression}

```gdscript
void set_advance_expression(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_advance_mode {#set_advance_mode}

```gdscript
void set_advance_mode(mode: AnimationNodeStateMachineTransition.AdvanceMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_break_loop_at_end {#set_break_loop_at_end}

```gdscript
void set_break_loop_at_end(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_priority {#set_priority}

```gdscript
void set_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_reset {#set_reset}

```gdscript
void set_reset(reset: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_switch_mode {#set_switch_mode}

```gdscript
void set_switch_mode(mode: AnimationNodeStateMachineTransition.SwitchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_xfade_curve {#set_xfade_curve}

```gdscript
void set_xfade_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_xfade_time {#set_xfade_time}

```gdscript
void set_xfade_time(secs: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### xfade_time {#xfade_time}

```gdscript
xfade_time: float
```

- void set_xfade_time(value: float)
- float get_xfade_time()

The time to cross-fade between this state and the next.

Note: AnimationNodeStateMachine transitions the current state immediately after the start of the fading. The precise remaining time can only be inferred from the main animation. When AnimationNodeOutput is considered as the most upstream, so the xfade_time is not scaled depending on the downstream delta. See also AnimationNodeOneShot.fadeout_time.

### xfade_curve {#xfade_curve}

```gdscript
xfade_curve: Curve
```

- void set_xfade_curve(value: Curve)
- Curve get_xfade_curve()

Ease curve for better control over cross-fade between this state and the next. Should be a unit Curve.

### break_loop_at_end {#break_loop_at_end}

```gdscript
break_loop_at_end: bool
```

- void set_break_loop_at_end(value: bool)
- bool is_loop_broken_at_end()

If true, breaks the loop at the end of the loop cycle for transition, even if the animation is looping.

### reset {#reset}

```gdscript
reset: bool
```

- void set_reset(value: bool)
- bool is_reset()

If true, the destination animation is played back from the beginning when switched.

### priority {#priority}

```gdscript
priority: int
```

- void set_priority(value: int)
- int get_priority()

Lower priority transitions are preferred when travelling through the tree via AnimationNodeStateMachinePlayback.travel() or advance_mode is set to ADVANCE_MODE_AUTO.

### switch_mode {#switch_mode}

```gdscript
switch_mode: int
```

- void set_switch_mode(value: SwitchMode)
- SwitchMode get_switch_mode()

The transition type.

### advance_mode {#advance_mode}

```gdscript
advance_mode: int
```

- void set_advance_mode(value: AdvanceMode)
- AdvanceMode get_advance_mode()

Determines whether the transition should be disabled, enabled when using AnimationNodeStateMachinePlayback.travel(), or traversed automatically if the advance_condition and advance_expression checks are true (if assigned).

### advance_condition {#advance_condition}

```gdscript
advance_condition: StringName
```

- void set_advance_condition(value: StringName)
- StringName get_advance_condition()

Turn on auto advance when this condition is set. The provided name will become a boolean parameter on the AnimationTree that can be controlled from code (see Using AnimationTree). For example, if AnimationTree.tree_root is an AnimationNodeStateMachine and advance_condition is set to "idle":

```
$animation_tree.set("parameters/conditions/idle", is_on_floor and (linear_velocity.x == 0))
```

```
GetNode("animation_tree").Set("parameters/conditions/idle", IsOnFloor && (LinearVelocity.X == 0));
```

### advance_expression {#advance_expression}

```gdscript
advance_expression: String
```

- void set_advance_expression(value: String)
- String get_advance_expression()

Use an expression as a condition for state machine transitions. It is possible to create complex animation advance conditions for switching between states and gives much greater flexibility for creating complex state machines by directly interfacing with the script code.

## 信号

### advance_condition_changed {#advance_condition_changed}

```gdscript
signal advance_condition_changed()
```

Emitted when advance_condition is changed.

## 枚举

### SwitchMode {#enum-SwitchMode}

```gdscript
enum SwitchMode { SWITCH_MODE_IMMEDIATE = 0, SWITCH_MODE_SYNC = 1, SWITCH_MODE_AT_END = 2 }
```

SwitchMode SWITCH_MODE_IMMEDIATE = 0

Switch to the next state immediately. The current state will end and blend into the beginning of the new one.

SwitchMode SWITCH_MODE_SYNC = 1

Switch to the next state immediately, but will seek the new state to the playback position of the old state.

SwitchMode SWITCH_MODE_AT_END = 2

Wait for the current state playback to end, then switch to the beginning of the next state animation.

#### SWITCH_MODE_IMMEDIATE {#SWITCH_MODE_IMMEDIATE}

```gdscript
SWITCH_MODE_IMMEDIATE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SWITCH_MODE_SYNC {#SWITCH_MODE_SYNC}

```gdscript
SWITCH_MODE_SYNC = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SWITCH_MODE_AT_END {#SWITCH_MODE_AT_END}

```gdscript
SWITCH_MODE_AT_END = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### AdvanceMode {#enum-AdvanceMode}

```gdscript
enum AdvanceMode { ADVANCE_MODE_DISABLED = 0, ADVANCE_MODE_ENABLED = 1, ADVANCE_MODE_AUTO = 2 }
```

AdvanceMode ADVANCE_MODE_DISABLED = 0

Don't use this transition.

AdvanceMode ADVANCE_MODE_ENABLED = 1

Only use this transition during AnimationNodeStateMachinePlayback.travel().

AdvanceMode ADVANCE_MODE_AUTO = 2

Automatically use this transition if the advance_condition and advance_expression checks are true (if assigned).

#### ADVANCE_MODE_DISABLED {#ADVANCE_MODE_DISABLED}

```gdscript
ADVANCE_MODE_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ADVANCE_MODE_ENABLED {#ADVANCE_MODE_ENABLED}

```gdscript
ADVANCE_MODE_ENABLED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ADVANCE_MODE_AUTO {#ADVANCE_MODE_AUTO}

```gdscript
ADVANCE_MODE_AUTO = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
