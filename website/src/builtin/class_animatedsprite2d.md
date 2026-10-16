# AnimatedSprite2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **AnimatedSprite2D**

> AnimatedSprite2D is similar to the Sprite2D node, except it carries multiple textures as animation frames. Animations are created using a SpriteFrames resource, which allows you to import image files (or a folder containing said files) to provide the animation frames for the sprite. The SpriteFrames resource can be configured in the editor via the SpriteFrames bottom panel.

<details><summary>官方英文描述（中文精译进行中）</summary>

AnimatedSprite2D is similar to the Sprite2D node, except it carries multiple textures as animation frames. Animations are created using a SpriteFrames resource, which allows you to import image files (or a folder containing said files) to provide the animation frames for the sprite. The SpriteFrames resource can be configured in the editor via the SpriteFrames bottom panel.

</details>

## 方法

### get_animation {#get_animation}

```gdscript
StringName get_animation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_autoplay {#get_autoplay}

```gdscript
String get_autoplay() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frame {#get_frame}

```gdscript
int get_frame() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frame_progress {#get_frame_progress}

```gdscript
float get_frame_progress() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_playing_speed {#get_playing_speed}

```gdscript
float get_playing_speed() const
```

Returns the actual playing speed of current animation or 0 if not playing. This speed is the speed_scale property multiplied by custom_speed argument specified when calling the play() method.

Returns a negative value if the current animation is playing backwards.

### get_speed_scale {#get_speed_scale}

```gdscript
float get_speed_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sprite_frames {#get_sprite_frames}

```gdscript
SpriteFrames get_sprite_frames() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_centered {#is_centered}

```gdscript
bool is_centered() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flipped_h {#is_flipped_h}

```gdscript
bool is_flipped_h() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flipped_v {#is_flipped_v}

```gdscript
bool is_flipped_v() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_playing {#is_playing}

```gdscript
bool is_playing() const
```

Returns true if an animation is currently playing (even if speed_scale and/or custom_speed are 0).

### pause {#pause}

```gdscript
void pause()
```

Pauses the currently playing animation. The frame and frame_progress will be kept and calling play() or play_backwards() without arguments will resume the animation from the current playback position.

See also stop().

### play {#play}

```gdscript
void play(name: StringName = &"", custom_speed: float = 1.0, from_end: bool = false)
```

Plays the animation with key name. If custom_speed is negative and from_end is true, the animation will play backwards (which is equivalent to calling play_backwards()).

If this method is called with that same animation name, or with no name parameter, the assigned animation will resume playing if it was paused.

### play_backwards {#play_backwards}

```gdscript
void play_backwards(name: StringName = &"")
```

Plays the animation with key name in reverse.

This method is a shorthand for play() with custom_speed = -1.0 and from_end = true, so see its description for more information.

### set_animation {#set_animation}

```gdscript
void set_animation(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autoplay {#set_autoplay}

```gdscript
void set_autoplay(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_centered {#set_centered}

```gdscript
void set_centered(centered: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_h {#set_flip_h}

```gdscript
void set_flip_h(flip_h: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_v {#set_flip_v}

```gdscript
void set_flip_v(flip_v: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_frame {#set_frame}

```gdscript
void set_frame(frame: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_frame_and_progress {#set_frame_and_progress}

```gdscript
void set_frame_and_progress(frame: int, progress: float)
```

Sets frame and frame_progress to the given values. Unlike setting frame, this method does not reset the frame_progress to 0.0 implicitly.

Example: Change the animation while keeping the same frame and frame_progress:

```
var current_frame = animated_sprite.get_frame()
var current_progress = animated_sprite.get_frame_progress()
animated_sprite.play("walk_another_skin")
animated_sprite.set_frame_and_progress(current_frame, current_progress)
```

### set_frame_progress {#set_frame_progress}

```gdscript
void set_frame_progress(progress: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_speed_scale {#set_speed_scale}

```gdscript
void set_speed_scale(speed_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sprite_frames {#set_sprite_frames}

```gdscript
void set_sprite_frames(sprite_frames: SpriteFrames)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stop {#stop}

```gdscript
void stop()
```

Stops the currently playing animation. The animation position is reset to 0 and the custom_speed is reset to 1.0. See also pause().

## 属性

### sprite_frames {#sprite_frames}

```gdscript
sprite_frames: SpriteFrames
```

- void set_sprite_frames(value: SpriteFrames)
- SpriteFrames get_sprite_frames()

The SpriteFrames resource containing the animation(s). Allows you the option to load, edit, clear, make unique and save the states of the SpriteFrames resource.

### animation {#animation}

```gdscript
animation: StringName
```

- void set_animation(value: StringName)
- StringName get_animation()

The current animation from the sprite_frames resource. If this value is changed, the frame counter and the frame_progress are reset.

### autoplay {#autoplay}

```gdscript
autoplay: StringName
```

- void set_autoplay(value: String)
- String get_autoplay()

The key of the animation to play when the scene loads.

### frame {#frame}

```gdscript
frame: int
```

- void set_frame(value: int)
- int get_frame()

The displayed animation frame's index. Setting this property also resets frame_progress. If this is not desired, use set_frame_and_progress().

### frame_progress {#frame_progress}

```gdscript
frame_progress: float
```

- void set_frame_progress(value: float)
- float get_frame_progress()

The progress value between 0.0 and 1.0 until the current frame transitions to the next frame. If the animation is playing backwards, the value transitions from 1.0 to 0.0.

### speed_scale {#speed_scale}

```gdscript
speed_scale: float
```

- void set_speed_scale(value: float)
- float get_speed_scale()

The speed scaling ratio. For example, if this value is 1, then the animation plays at normal speed. If it's 0.5, then it plays at half speed. If it's 2, then it plays at double speed.

If set to a negative value, the animation is played in reverse. If set to 0, the animation will not advance.

### centered {#centered}

```gdscript
centered: bool
```

- void set_centered(value: bool)
- bool is_centered()

If true, texture will be centered.

Note: For games with a pixel art aesthetic, textures may appear deformed when centered. This is caused by their position being between pixels. To prevent this, set this property to false, or consider enabling ProjectSettings.rendering/2d/snap/snap_2d_vertices_to_pixel and ProjectSettings.rendering/2d/snap/snap_2d_transforms_to_pixel.

### offset {#offset}

```gdscript
offset: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### flip_h {#flip_h}

```gdscript
flip_h: bool
```

- void set_flip_h(value: bool)
- bool is_flipped_h()

If true, texture is flipped horizontally.

### flip_v {#flip_v}

```gdscript
flip_v: bool
```

- void set_flip_v(value: bool)
- bool is_flipped_v()

If true, texture is flipped vertically.

## 信号

### sprite_frames_changed {#sprite_frames_changed}

```gdscript
signal sprite_frames_changed()
```

Emitted when sprite_frames changes.

### animation_changed {#animation_changed}

```gdscript
signal animation_changed()
```

Emitted when animation changes.

### frame_changed {#frame_changed}

```gdscript
signal frame_changed()
```

Emitted when frame changes.

### animation_looped {#animation_looped}

```gdscript
signal animation_looped()
```

Emitted when the animation loops.

### animation_finished {#animation_finished}

```gdscript
signal animation_finished()
```

Emitted when the animation reaches the end, or the start if it is played in reverse. When the animation finishes, it pauses the playback.

Note: This signal is not emitted if an animation is looping.
