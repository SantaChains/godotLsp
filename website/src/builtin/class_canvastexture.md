# CanvasTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CanvasTexture**

> CanvasTexture is an alternative to ImageTexture for 2D rendering. It allows using normal maps and specular maps in any node that inherits from CanvasItem. CanvasTexture also allows overriding the texture's filter and repeat mode independently of the node's properties (or the project settings).

<details><summary>官方英文描述（中文精译进行中）</summary>

CanvasTexture is an alternative to ImageTexture for 2D rendering. It allows using normal maps and specular maps in any node that inherits from CanvasItem. CanvasTexture also allows overriding the texture's filter and repeat mode independently of the node's properties (or the project settings).

Note: CanvasTexture cannot be used in 3D. It will not display correctly when applied to any VisualInstance3D, such as Sprite3D or Decal. For physically-based materials in 3D, use BaseMaterial3D instead.

</details>

## 方法

### get_diffuse_texture {#get_diffuse_texture}

```gdscript
Texture2D get_diffuse_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_normal_texture {#get_normal_texture}

```gdscript
Texture2D get_normal_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_specular_color {#get_specular_color}

```gdscript
Color get_specular_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_specular_shininess {#get_specular_shininess}

```gdscript
float get_specular_shininess() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_specular_texture {#get_specular_texture}

```gdscript
Texture2D get_specular_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_filter {#get_texture_filter}

```gdscript
CanvasItem.TextureFilter get_texture_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_repeat {#get_texture_repeat}

```gdscript
CanvasItem.TextureRepeat get_texture_repeat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_diffuse_texture {#set_diffuse_texture}

```gdscript
void set_diffuse_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_normal_texture {#set_normal_texture}

```gdscript
void set_normal_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_specular_color {#set_specular_color}

```gdscript
void set_specular_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_specular_shininess {#set_specular_shininess}

```gdscript
void set_specular_shininess(shininess: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_specular_texture {#set_specular_texture}

```gdscript
void set_specular_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_filter {#set_texture_filter}

```gdscript
void set_texture_filter(filter: CanvasItem.TextureFilter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_repeat {#set_texture_repeat}

```gdscript
void set_texture_repeat(repeat: CanvasItem.TextureRepeat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### diffuse_texture {#diffuse_texture}

```gdscript
diffuse_texture: Texture2D
```

- void set_diffuse_texture(value: Texture2D)
- Texture2D get_diffuse_texture()

The diffuse (color) texture to use. This is the main texture you want to set in most cases.

### normal_texture {#normal_texture}

```gdscript
normal_texture: Texture2D
```

- void set_normal_texture(value: Texture2D)
- Texture2D get_normal_texture()

The normal map texture to use. Only has a visible effect if Light2Ds are affecting this CanvasTexture.

Note: Godot expects the normal map to use X+, Y+, and Z+ coordinates. See this page [http://wiki.polycount.com/wiki/Normal_Map_Technical_Details#Common_Swizzle_Coordinates] for a comparison of normal map coordinates expected by popular engines.

### specular_texture {#specular_texture}

```gdscript
specular_texture: Texture2D
```

- void set_specular_texture(value: Texture2D)
- Texture2D get_specular_texture()

The specular map to use for Light2D specular reflections. This should be a grayscale or colored texture, with brighter areas resulting in a higher specular_shininess value. Using a colored specular_texture allows controlling specular shininess on a per-channel basis. Only has a visible effect if Light2Ds are affecting this CanvasTexture.

### specular_color {#specular_color}

```gdscript
specular_color: Color
```

- void set_specular_color(value: Color)
- Color get_specular_color()

The multiplier for specular reflection colors. The Light2D's color is also taken into account when determining the reflection color. Only has a visible effect if Light2Ds are affecting this CanvasTexture.

### specular_shininess {#specular_shininess}

```gdscript
specular_shininess: float
```

- void set_specular_shininess(value: float)
- float get_specular_shininess()

The specular exponent for Light2D specular reflections. Higher values result in a more glossy/"wet" look, with reflections becoming more localized and less visible overall. The default value of 1.0 disables specular reflections entirely. Only has a visible effect if Light2Ds are affecting this CanvasTexture.

### texture_filter {#texture_filter}

```gdscript
texture_filter: int
```

- void set_texture_filter(value: TextureFilter)
- TextureFilter get_texture_filter()

The texture filtering mode to use when drawing this CanvasTexture.

### texture_repeat {#texture_repeat}

```gdscript
texture_repeat: int
```

- void set_texture_repeat(value: TextureRepeat)
- TextureRepeat get_texture_repeat()

The texture repeat mode to use when drawing this CanvasTexture.
