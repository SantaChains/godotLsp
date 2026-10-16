# PointLight2D

**继承**：[Light2D](class_light2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **PointLight2D**

> Casts light in a 2D environment. This light's shape is defined by a (usually grayscale) texture.

<details><summary>官方英文描述（中文精译进行中）</summary>

Casts light in a 2D environment. This light's shape is defined by a (usually grayscale) texture.

</details>

## 方法

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_offset {#get_texture_offset}

```gdscript
Vector2 get_texture_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_scale {#get_texture_scale}

```gdscript
float get_texture_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_offset {#set_texture_offset}

```gdscript
void set_texture_offset(texture_offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_scale {#set_texture_scale}

```gdscript
void set_texture_scale(texture_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture {#texture}

```gdscript
texture: Texture2D,-AnimatedTexture,-AtlasTexture,-CameraTexture,-CanvasTexture,-MeshTexture,-Texture2DRD,-ViewportTexture
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

Texture2D used for the light's appearance.

### offset {#offset}

```gdscript
offset: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### texture_scale {#texture_scale}

```gdscript
texture_scale: float
```

- void set_texture_scale(value: float)
- float get_texture_scale()

The texture's scale factor.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

The height of the light. Used with 2D normal mapping. The units are in pixels, e.g. if the height is 100, then it will illuminate an object 100 pixels away at a 45° angle to the plane.
