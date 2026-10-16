# Sprite2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Sprite2D**

> A node that displays a 2D texture. The texture displayed can be a region from a larger atlas texture, or a frame from a sprite sheet animation.

<details><summary>官方英文描述（中文精译进行中）</summary>

A node that displays a 2D texture. The texture displayed can be a region from a larger atlas texture, or a frame from a sprite sheet animation.

</details>

## 方法

### get_frame {#get_frame}

```gdscript
int get_frame() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frame_coords {#get_frame_coords}

```gdscript
Vector2i get_frame_coords() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hframes {#get_hframes}

```gdscript
int get_hframes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rect {#get_rect}

```gdscript
Rect2 get_rect() const
```

Returns a Rect2 representing the Sprite2D's boundary in local coordinates.

Example: Detect if the Sprite2D was clicked:

```
func _input(event):
    if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
        if get_rect().has_point(to_local(event.position)):
            print("A click!")
```

```
public override void _Input(InputEvent @event)
{
    if (@event is InputEventMouseButton inputEventMouse)
    {
        if (inputEventMouse.Pressed && inputEventMouse.ButtonIndex == MouseButton.Left)
        {
            if (GetRect().HasPoint(ToLocal(inputEventMouse.Position)))
            {
                GD.Print("A click!");
            }
        }
    }
}
```

### get_region_rect {#get_region_rect}

```gdscript
Rect2 get_region_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vframes {#get_vframes}

```gdscript
int get_vframes() const
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

### is_pixel_opaque {#is_pixel_opaque}

```gdscript
bool is_pixel_opaque(pos: Vector2) const
```

Returns true if the pixel at the given position is opaque, false otherwise. Also returns false if the given position is out of bounds or this sprite's texture is null. pos is in local coordinates.

### is_region_enabled {#is_region_enabled}

```gdscript
bool is_region_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_region_filter_clip_enabled {#is_region_filter_clip_enabled}

```gdscript
bool is_region_filter_clip_enabled() const
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

### set_frame_coords {#set_frame_coords}

```gdscript
void set_frame_coords(coords: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hframes {#set_hframes}

```gdscript
void set_hframes(hframes: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region_enabled {#set_region_enabled}

```gdscript
void set_region_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region_filter_clip_enabled {#set_region_filter_clip_enabled}

```gdscript
void set_region_filter_clip_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region_rect {#set_region_rect}

```gdscript
void set_region_rect(rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vframes {#set_vframes}

```gdscript
void set_vframes(vframes: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

Texture2D object to draw.

### centered {#centered}

```gdscript
centered: bool
```

- void set_centered(value: bool)
- bool is_centered()

If true, texture is centered.

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

### hframes {#hframes}

```gdscript
hframes: int
```

- void set_hframes(value: int)
- int get_hframes()

The number of columns in the sprite sheet. When this property is changed, frame is adjusted so that the same visual frame is maintained (same row and column). If that's impossible, frame is reset to 0.

### vframes {#vframes}

```gdscript
vframes: int
```

- void set_vframes(value: int)
- int get_vframes()

The number of rows in the sprite sheet. When this property is changed, frame is adjusted so that the same visual frame is maintained (same row and column). If that's impossible, frame is reset to 0.

### frame {#frame}

```gdscript
frame: int
```

- void set_frame(value: int)
- int get_frame()

Current frame to display from sprite sheet. hframes or vframes must be greater than 1. This property is automatically adjusted when hframes or vframes are changed to keep pointing to the same visual frame (same column and row). If that's impossible, this value is reset to 0.

### frame_coords {#frame_coords}

```gdscript
frame_coords: Vector2i
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### region_enabled {#region_enabled}

```gdscript
region_enabled: bool
```

- void set_region_enabled(value: bool)
- bool is_region_enabled()

If true, texture is cut from a larger atlas texture. See region_rect.

Note: When using a custom Shader on a Sprite2D, the UV shader built-in will refer to the entire texture space. Use the REGION_RECT built-in to get the currently visible region defined in region_rect instead. See CanvasItem shaders for details.

### region_rect {#region_rect}

```gdscript
region_rect: Rect2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### region_filter_clip_enabled {#region_filter_clip_enabled}

```gdscript
region_filter_clip_enabled: bool
```

- void set_region_filter_clip_enabled(value: bool)
- bool is_region_filter_clip_enabled()

If true, the area outside of the region_rect is clipped to avoid bleeding of the surrounding texture pixels. region_enabled must be true.

## 信号

### frame_changed {#frame_changed}

```gdscript
signal frame_changed()
```

Emitted when the frame changes.

### texture_changed {#texture_changed}

```gdscript
signal texture_changed()
```

Emitted when the texture changes.
