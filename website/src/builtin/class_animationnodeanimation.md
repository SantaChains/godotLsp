# AnimationNodeAnimation

**继承**：[AnimationRootNode](class_animationrootnode.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeAnimation**

> A resource to add to an AnimationNodeBlendTree. Only has one output port using the animation property. Used as an input for AnimationNodes that blend animations together.

<details><summary>官方英文描述（中文精译进行中）</summary>

A resource to add to an AnimationNodeBlendTree. Only has one output port using the animation property. Used as an input for AnimationNodes that blend animations together.

</details>

## 方法

### get_animation {#get_animation}

```gdscript
StringName get_animation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_mode {#get_loop_mode}

```gdscript
Animation.LoopMode get_loop_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_play_mode {#get_play_mode}

```gdscript
AnimationNodeAnimation.PlayMode get_play_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_start_offset {#get_start_offset}

```gdscript
float get_start_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_timeline_length {#get_timeline_length}

```gdscript
float get_timeline_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_advance_on_start {#is_advance_on_start}

```gdscript
bool is_advance_on_start() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_stretching_time_scale {#is_stretching_time_scale}

```gdscript
bool is_stretching_time_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_custom_timeline {#is_using_custom_timeline}

```gdscript
bool is_using_custom_timeline() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_advance_on_start {#set_advance_on_start}

```gdscript
void set_advance_on_start(advance_on_start: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_animation {#set_animation}

```gdscript
void set_animation(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop_mode {#set_loop_mode}

```gdscript
void set_loop_mode(loop_mode: Animation.LoopMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_play_mode {#set_play_mode}

```gdscript
void set_play_mode(mode: AnimationNodeAnimation.PlayMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_start_offset {#set_start_offset}

```gdscript
void set_start_offset(start_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stretch_time_scale {#set_stretch_time_scale}

```gdscript
void set_stretch_time_scale(stretch_time_scale: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_timeline_length {#set_timeline_length}

```gdscript
void set_timeline_length(timeline_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_custom_timeline {#set_use_custom_timeline}

```gdscript
void set_use_custom_timeline(use_custom_timeline: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### animation {#animation}

```gdscript
animation: StringName
```

- void set_animation(value: StringName)
- StringName get_animation()

Animation to use as an output. It is one of the animations provided by AnimationTree.anim_player.

### play_mode {#play_mode}

```gdscript
play_mode: int
```

- void set_play_mode(value: PlayMode)
- PlayMode get_play_mode()

Determines the playback direction of the animation.

### advance_on_start {#advance_on_start}

```gdscript
advance_on_start: bool
```

- void set_advance_on_start(value: bool)
- bool is_advance_on_start()

If true, on receiving a request to play an animation from the start, the first frame is not drawn, but only processed, and playback starts from the next frame.

See also the notes of AnimationPlayer.play().

### use_custom_timeline {#use_custom_timeline}

```gdscript
use_custom_timeline: bool
```

- void set_use_custom_timeline(value: bool)
- bool is_using_custom_timeline()

If true, AnimationNode provides an animation based on the Animation resource with some parameters adjusted.

### timeline_length {#timeline_length}

```gdscript
timeline_length: float
```

- void set_timeline_length(value: float)
- float get_timeline_length()

The length of the custom timeline.

If stretch_time_scale is true, scales the animation to this length.

### stretch_time_scale {#stretch_time_scale}

```gdscript
stretch_time_scale: bool
```

- void set_stretch_time_scale(value: bool)
- bool is_stretching_time_scale()

If true, scales the time so that the length specified in timeline_length is one cycle.

This is useful for matching the periods of walking and running animations.

If false, the original animation length is respected. If you set the loop to loop_mode, the animation will loop in timeline_length.

### start_offset {#start_offset}

```gdscript
start_offset: float
```

- void set_start_offset(value: float)
- float get_start_offset()

If use_custom_timeline is true, offset the start position of the animation.

This is useful for adjusting which foot steps first in 3D walking animations.

### loop_mode {#loop_mode}

```gdscript
loop_mode: int
```

- void set_loop_mode(value: LoopMode)
- LoopMode get_loop_mode()

If use_custom_timeline is true, override the loop settings of the original Animation resource with the value.

Note: If the Animation.loop_mode isn't set to looping, the Animation.track_set_interpolation_loop_wrap() option will not be respected. If you cannot get the expected behavior, consider duplicating the Animation resource and changing the loop settings.

## 枚举

### PlayMode {#enum-PlayMode}

```gdscript
enum PlayMode { PLAY_MODE_FORWARD = 0, PLAY_MODE_BACKWARD = 1 }
```

PlayMode PLAY_MODE_FORWARD = 0

Plays animation in forward direction.

PlayMode PLAY_MODE_BACKWARD = 1

Plays animation in backward direction.

#### PLAY_MODE_FORWARD {#PLAY_MODE_FORWARD}

```gdscript
PLAY_MODE_FORWARD = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLAY_MODE_BACKWARD {#PLAY_MODE_BACKWARD}

```gdscript
PLAY_MODE_BACKWARD = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
