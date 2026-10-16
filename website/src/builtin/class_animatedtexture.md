# AnimatedTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimatedTexture**

> AnimatedTexture is a resource format for frame-based animations, where multiple textures can be chained automatically with a predefined delay for each frame. Unlike AnimationPlayer or AnimatedSprite2D, it isn't a Node, but has the advantage of being usable anywhere a Texture2D resource can be used, e.g. in a TileSet.

<details><summary>官方英文描述（中文精译进行中）</summary>

AnimatedTexture is a resource format for frame-based animations, where multiple textures can be chained automatically with a predefined delay for each frame. Unlike AnimationPlayer or AnimatedSprite2D, it isn't a Node, but has the advantage of being usable anywhere a Texture2D resource can be used, e.g. in a TileSet.

The playback of the animation is controlled by the speed_scale property, as well as each frame's duration (see set_frame_duration()). The animation loops, i.e. it will restart at frame 0 automatically after playing the last frame.

AnimatedTexture currently requires all frame textures to have the same size, otherwise the bigger ones will be cropped to match the smallest one.

Note: AnimatedTexture doesn't support using AtlasTextures. Each frame needs to be a separate Texture2D.

Warning: The current implementation is not efficient for the modern renderers.

</details>

## 方法

### get_current_frame {#get_current_frame}

```gdscript
int get_current_frame() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frame_duration {#get_frame_duration}

```gdscript
float get_frame_duration(frame: int) const
```

Returns the given frame's duration, in seconds.

### get_frame_texture {#get_frame_texture}

```gdscript
Texture2D get_frame_texture(frame: int) const
```

Returns the given frame's Texture2D.

### get_frames {#get_frames}

```gdscript
int get_frames() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_one_shot {#get_one_shot}

```gdscript
bool get_one_shot() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pause {#get_pause}

```gdscript
bool get_pause() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_speed_scale {#get_speed_scale}

```gdscript
float get_speed_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current_frame {#set_current_frame}

```gdscript
void set_current_frame(frame: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_frame_duration {#set_frame_duration}

```gdscript
void set_frame_duration(frame: int, duration: float)
```

Sets the duration of any given frame. The final duration is affected by the speed_scale. If set to 0, the frame is skipped during playback.

### set_frame_texture {#set_frame_texture}

```gdscript
void set_frame_texture(frame: int, texture: Texture2D)
```

Assigns a Texture2D to the given frame. Frame IDs start at 0, so the first frame has ID 0, and the last frame of the animation has ID frames - 1.

You can define any number of textures up to MAX_FRAMES, but keep in mind that only frames from 0 to frames - 1 will be part of the animation.

### set_frames {#set_frames}

```gdscript
void set_frames(frames: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_shot {#set_one_shot}

```gdscript
void set_one_shot(one_shot: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pause {#set_pause}

```gdscript
void set_pause(pause: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_speed_scale {#set_speed_scale}

```gdscript
void set_speed_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### frames {#frames}

```gdscript
frames: int
```

- void set_frames(value: int)
- int get_frames()

Number of frames to use in the animation. While you can create the frames independently with set_frame_texture(), you need to set this value for the animation to take new frames into account. The maximum number of frames is MAX_FRAMES.

### current_frame {#current_frame}

```gdscript
current_frame: int
```

- void set_current_frame(value: int)
- int get_current_frame()

Sets the currently visible frame of the texture. Setting this frame while playing resets the current frame time, so the newly selected frame plays for its whole configured frame duration.

### pause {#pause}

```gdscript
pause: bool
```

- void set_pause(value: bool)
- bool get_pause()

If true, the animation will pause where it currently is (i.e. at current_frame). The animation will continue from where it was paused when changing this property to false.

### one_shot {#one_shot}

```gdscript
one_shot: bool
```

- void set_one_shot(value: bool)
- bool get_one_shot()

If true, the animation will only play once and will not loop back to the first frame after reaching the end. Note that reaching the end will not set pause to true.

### speed_scale {#speed_scale}

```gdscript
speed_scale: float
```

- void set_speed_scale(value: float)
- float get_speed_scale()

The animation speed is multiplied by this value. If set to a negative value, the animation is played in reverse.

## 常量

### MAX_FRAMES {#MAX_FRAMES}

```gdscript
const MAX_FRAMES = 256
```

The maximum number of frames supported by AnimatedTexture. If you need more frames in your animation, use AnimationPlayer or AnimatedSprite2D.
