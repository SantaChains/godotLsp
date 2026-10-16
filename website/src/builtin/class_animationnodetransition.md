# AnimationNodeTransition

**继承**：[AnimationNodeSync](class_animationnodesync.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeTransition**

> Simple state machine for cases which don't require a more advanced AnimationNodeStateMachine. Animations can be connected to the inputs and transition times can be specified.

<details><summary>官方英文描述</summary>

Simple state machine for cases which don't require a more advanced AnimationNodeStateMachine. Animations can be connected to the inputs and transition times can be specified.

After setting the request and changing the animation playback, the transition node automatically clears the request on the next process frame by setting its transition_request value to empty.

Note: When using a cross-fade, current_state and current_index change to the next state immediately after the cross-fade begins.

```
# Play child animation connected to "state_2" port.
animation_tree.set("parameters/Transition/transition_request", "state_2")
# Alternative syntax (same result as above).
animation_tree["parameters/Transition/transition_request"] = "state_2"
# Get current state name (read-only).
animation_tree.get("parameters/Transition/current_state")
# Alternative syntax (same result as above).
animation_tree["parameters/Transition/current_state"]
# Get current state index (read-only).
animation_tree.get("parameters/Transition/current_index")
# Alternative syntax (same result as above).
animation_tree["parameters/Transition/current_index"]
```

```
// Play child animation connected to "state_2" port.
animationTree.Set("parameters/Transition/transition_request", "state_2");
// Get current state name (read-only).
animationTree.Get("parameters/Transition/current_state");
// Get current state index (read-only).
animationTree.Get("parameters/Transition/current_index");
```

</details>

## 方法

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

### is_allow_transition_to_self {#is_allow_transition_to_self}

```gdscript
bool is_allow_transition_to_self() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_input_loop_broken_at_end {#is_input_loop_broken_at_end}

```gdscript
bool is_input_loop_broken_at_end(input: int) const
```

Returns whether the animation breaks the loop at the end of the loop cycle for transition.

### is_input_reset {#is_input_reset}

```gdscript
bool is_input_reset(input: int) const
```

Returns whether the animation restarts when the animation transitions from the other animation.

### is_input_set_as_auto_advance {#is_input_set_as_auto_advance}

```gdscript
bool is_input_set_as_auto_advance(input: int) const
```

Returns true if auto-advance is enabled for the given input index.

### set_allow_transition_to_self {#set_allow_transition_to_self}

```gdscript
void set_allow_transition_to_self(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_as_auto_advance {#set_input_as_auto_advance}

```gdscript
void set_input_as_auto_advance(input: int, enable: bool)
```

Enables or disables auto-advance for the given input index. If enabled, state changes to the next input after playing the animation once. If enabled for the last input state, it loops to the first.

### set_input_break_loop_at_end {#set_input_break_loop_at_end}

```gdscript
void set_input_break_loop_at_end(input: int, enable: bool)
```

If true, breaks the loop at the end of the loop cycle for transition, even if the animation is looping.

### set_input_count {#set_input_count}

```gdscript
void set_input_count(input_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_reset {#set_input_reset}

```gdscript
void set_input_reset(input: int, enable: bool)
```

If true, the destination animation is restarted when the animation transitions.

### set_xfade_curve {#set_xfade_curve}

```gdscript
void set_xfade_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_xfade_time {#set_xfade_time}

```gdscript
void set_xfade_time(time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### xfade_time {#xfade_time}

```gdscript
xfade_time: float
```

- void set_xfade_time(value: float)
- float get_xfade_time()

Cross-fading time (in seconds) between each animation connected to the inputs.

Note: AnimationNodeTransition transitions the current state immediately after the start of the fading. The precise remaining time can only be inferred from the main animation. When AnimationNodeOutput is considered as the most upstream, so the xfade_time is not scaled depending on the downstream delta. See also AnimationNodeOneShot.fadeout_time.

### xfade_curve {#xfade_curve}

```gdscript
xfade_curve: Curve
```

- void set_xfade_curve(value: Curve)
- Curve get_xfade_curve()

Determines how cross-fading between animations is eased. If empty, the transition will be linear. Should be a unit Curve.

### allow_transition_to_self {#allow_transition_to_self}

```gdscript
allow_transition_to_self: bool
```

- void set_allow_transition_to_self(value: bool)
- bool is_allow_transition_to_self()

If true, allows transition to the self state. When the reset option is enabled in input, the animation is restarted. If false, nothing happens on the transition to the self state.

### input_count {#input_count}

```gdscript
input_count: int
```

- void set_input_count(value: int)
- int get_input_count()

The number of enabled input ports for this animation node.
