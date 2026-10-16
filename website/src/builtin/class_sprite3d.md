# Sprite3D

**继承**：[SpriteBase3D](class_spritebase3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Sprite3D**

> A node that displays a 2D texture in a 3D environment. The texture displayed can be a region from a larger atlas texture, or a frame from a sprite sheet animation. See also SpriteBase3D where properties such as the billboard mode are defined.

<details><summary>官方英文描述</summary>

A node that displays a 2D texture in a 3D environment. The texture displayed can be a region from a larger atlas texture, or a frame from a sprite sheet animation. See also SpriteBase3D where properties such as the billboard mode are defined.

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

### is_region_enabled {#is_region_enabled}

```gdscript
bool is_region_enabled() const
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

### set_region_enabled {#set_region_enabled}

```gdscript
void set_region_enabled(enabled: bool)
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

Texture2D object to draw. If GeometryInstance3D.material_override is used, this will be overridden. The size information is still used.

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

- void set_frame_coords(value: Vector2i)
- Vector2i get_frame_coords()

Coordinates of the frame to display from sprite sheet. This is as an alias for the frame property. hframes or vframes must be greater than 1.

### region_enabled {#region_enabled}

```gdscript
region_enabled: bool
```

- void set_region_enabled(value: bool)
- bool is_region_enabled()

If true, the sprite will use region_rect and display only the specified part of its texture.

### region_rect {#region_rect}

```gdscript
region_rect: Rect2
```

- void set_region_rect(value: Rect2)
- Rect2 get_region_rect()

The region of the atlas texture to display. region_enabled must be true.

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
