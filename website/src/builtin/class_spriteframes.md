# SpriteFrames

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SpriteFrames**

> Sprite frame library for an AnimatedSprite2D or AnimatedSprite3D node. Contains frames and animation data for playback.

<details><summary>官方英文描述（中文精译进行中）</summary>

Sprite frame library for an AnimatedSprite2D or AnimatedSprite3D node. Contains frames and animation data for playback.

</details>

## 方法

### add_animation {#add_animation}

```gdscript
void add_animation(anim: StringName)
```

Adds a new anim animation to the library.

### add_frame {#add_frame}

```gdscript
void add_frame(anim: StringName, texture: Texture2D, duration: float = 1.0, at_position: int = -1)
```

Adds a frame to the anim animation. If at_position is -1, the frame will be added to the end of the animation. duration specifies the relative duration, see get_frame_duration() for details.

### clear {#clear}

```gdscript
void clear(anim: StringName)
```

Removes all frames from the anim animation.

### clear_all {#clear_all}

```gdscript
void clear_all()
```

Removes all animations. An empty default animation will be created.

### duplicate_animation {#duplicate_animation}

```gdscript
void duplicate_animation(anim_from: StringName, anim_to: StringName)
```

Duplicates the animation anim_from to a new animation named anim_to. Fails if anim_to already exists, or if anim_from does not exist.

### get_animation_loop {#get_animation_loop}

```gdscript
bool get_animation_loop(anim: StringName) const
```

Deprecated: Use get_animation_loop_mode() instead.

Returns true if get_animation_loop_mode(anim) == LOOP_LINEAR. Otherwise, returns false.

### get_animation_loop_mode {#get_animation_loop_mode}

```gdscript
SpriteFrames.LoopMode get_animation_loop_mode(anim: StringName) const
```

Returns the loop mode for the anim animation.

### get_animation_names {#get_animation_names}

```gdscript
PackedStringArray get_animation_names() const
```

Returns an array containing the names associated to each animation. Values are placed in alphabetical order.

### get_animation_speed {#get_animation_speed}

```gdscript
float get_animation_speed(anim: StringName) const
```

Returns the speed in frames per second for the anim animation.

### get_frame_count {#get_frame_count}

```gdscript
int get_frame_count(anim: StringName) const
```

Returns the number of frames for the anim animation.

### get_frame_duration {#get_frame_duration}

```gdscript
float get_frame_duration(anim: StringName, idx: int) const
```

Returns a relative duration of the frame idx in the anim animation (defaults to 1.0). For example, a frame with a duration of 2.0 is displayed twice as long as a frame with a duration of 1.0. You can calculate the absolute duration (in seconds) of a frame using the following formula:

```gdscript
absolute_duration = relative_duration / (animation_fps * abs(playing_speed))
```

In this example, playing_speed refers to either AnimatedSprite2D.get_playing_speed() or AnimatedSprite3D.get_playing_speed().

### get_frame_texture {#get_frame_texture}

```gdscript
Texture2D get_frame_texture(anim: StringName, idx: int) const
```

Returns the texture of the frame idx in the anim animation.

### has_animation {#has_animation}

```gdscript
bool has_animation(anim: StringName) const
```

Returns true if the anim animation exists.

### remove_animation {#remove_animation}

```gdscript
void remove_animation(anim: StringName)
```

Removes the anim animation.

### remove_frame {#remove_frame}

```gdscript
void remove_frame(anim: StringName, idx: int)
```

Removes the anim animation's frame idx.

### rename_animation {#rename_animation}

```gdscript
void rename_animation(anim: StringName, newname: StringName)
```

Changes the anim animation's name to newname.

### set_animation_loop {#set_animation_loop}

```gdscript
void set_animation_loop(anim: StringName, loop: bool)
```

Deprecated: Use set_animation_loop_mode() instead.

If loop is false equivalent to set_animation_loop_mode(LOOP_NONE).

If loop is true equivalent to set_animation_loop_mode(LOOP_LINEAR).

### set_animation_loop_mode {#set_animation_loop_mode}

```gdscript
void set_animation_loop_mode(anim: StringName, loop_mode: SpriteFrames.LoopMode)
```

Sets the loop_mode for the anim animation.

### set_animation_speed {#set_animation_speed}

```gdscript
void set_animation_speed(anim: StringName, fps: float)
```

Sets the speed for the anim animation in frames per second.

### set_frame {#set_frame}

```gdscript
void set_frame(anim: StringName, idx: int, texture: Texture2D, duration: float = 1.0)
```

Sets the texture and the duration of the frame idx in the anim animation. duration specifies the relative duration, see get_frame_duration() for details.

## 属性

### animations {#animations}

```gdscript
animations: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### LoopMode {#enum-LoopMode}

```gdscript
enum LoopMode { LOOP_NONE = 0, LOOP_LINEAR = 1, LOOP_PINGPONG = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOP_NONE {#LOOP_NONE}

```gdscript
LOOP_NONE = 0
```

The animation plays once and stops when it reaches the end, or the start if played in reverse.

#### LOOP_LINEAR {#LOOP_LINEAR}

```gdscript
LOOP_LINEAR = 1
```

The animation restarts from the beginning when it reaches the end, or from the end if played in reverse, repeating continuously.

#### LOOP_PINGPONG {#LOOP_PINGPONG}

```gdscript
LOOP_PINGPONG = 2
```

The animation alternates direction each time it reaches the end or start, playing forward and then in reverse repeatedly.

Note: Both AnimatedSprite2D and AnimatedSprite3D play the first/last frame for its duration only once at each end of the animation loop (instead of twice, once per forward/backward animation direction).

---
