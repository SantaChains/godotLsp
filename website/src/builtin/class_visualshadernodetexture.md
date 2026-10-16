# VisualShaderNodeTexture

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeTexture**

> Performs a lookup operation on the provided texture, with support for multiple texture sources to choose from.

<details><summary>官方英文描述</summary>

Performs a lookup operation on the provided texture, with support for multiple texture sources to choose from.

</details>

## 方法

### get_source {#get_source}

```gdscript
VisualShaderNodeTexture.Source get_source() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_type {#get_texture_type}

```gdscript
VisualShaderNodeTexture.TextureType get_texture_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source {#set_source}

```gdscript
void set_source(value: VisualShaderNodeTexture.Source)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(value: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_type {#set_texture_type}

```gdscript
void set_texture_type(value: VisualShaderNodeTexture.TextureType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### source {#source}

```gdscript
source: int
```

- void set_source(value: Source)
- Source get_source()

Determines the source for the lookup.

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The source texture, if needed for the selected source.

### texture_type {#texture_type}

```gdscript
texture_type: int
```

- void set_texture_type(value: TextureType)
- TextureType get_texture_type()

Specifies the type of the texture if source is set to SOURCE_TEXTURE.

## 枚举

### Source {#enum-Source}

```gdscript
enum Source { SOURCE_TEXTURE = 0, SOURCE_SCREEN = 1, SOURCE_2D_TEXTURE = 2, SOURCE_2D_NORMAL = 3, SOURCE_DEPTH = 4, SOURCE_PORT = 5, SOURCE_3D_NORMAL = 6, SOURCE_ROUGHNESS = 7, SOURCE_MAX = 8 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_TEXTURE {#SOURCE_TEXTURE}

```gdscript
SOURCE_TEXTURE = 0
```

Use the texture given as an argument for this function.

#### SOURCE_SCREEN {#SOURCE_SCREEN}

```gdscript
SOURCE_SCREEN = 1
```

Use the current viewport's texture as the source.

#### SOURCE_2D_TEXTURE {#SOURCE_2D_TEXTURE}

```gdscript
SOURCE_2D_TEXTURE = 2
```

Use the texture from this shader's texture built-in (e.g. a texture of a Sprite2D).

#### SOURCE_2D_NORMAL {#SOURCE_2D_NORMAL}

```gdscript
SOURCE_2D_NORMAL = 3
```

Use the texture from this shader's normal map built-in.

#### SOURCE_DEPTH {#SOURCE_DEPTH}

```gdscript
SOURCE_DEPTH = 4
```

Use the depth texture captured during the depth prepass. Only available when the depth prepass is used (i.e. in spatial shaders and in the forward_plus or gl_compatibility renderers).

#### SOURCE_PORT {#SOURCE_PORT}

```gdscript
SOURCE_PORT = 5
```

Use the texture provided in the input port for this function.

#### SOURCE_3D_NORMAL {#SOURCE_3D_NORMAL}

```gdscript
SOURCE_3D_NORMAL = 6
```

Use the normal buffer captured during the depth prepass. Only available when the normal-roughness buffer is available (i.e. in spatial shaders and in the forward_plus renderer).

#### SOURCE_ROUGHNESS {#SOURCE_ROUGHNESS}

```gdscript
SOURCE_ROUGHNESS = 7
```

Use the roughness buffer captured during the depth prepass. Only available when the normal-roughness buffer is available (i.e. in spatial shaders and in the forward_plus renderer).

#### SOURCE_MAX {#SOURCE_MAX}

```gdscript
SOURCE_MAX = 8
```

Represents the size of the Source enum.

---

### TextureType {#enum-TextureType}

```gdscript
enum TextureType { TYPE_DATA = 0, TYPE_COLOR = 1, TYPE_NORMAL_MAP = 2, TYPE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_DATA {#TYPE_DATA}

```gdscript
TYPE_DATA = 0
```

No hints are added to the uniform declaration.

#### TYPE_COLOR {#TYPE_COLOR}

```gdscript
TYPE_COLOR = 1
```

Adds source_color as hint to the uniform declaration for proper conversion from nonlinear sRGB encoding to linear encoding.

#### TYPE_NORMAL_MAP {#TYPE_NORMAL_MAP}

```gdscript
TYPE_NORMAL_MAP = 2
```

Adds hint_normal as hint to the uniform declaration, which internally converts the texture for proper usage as normal map.

#### TYPE_MAX {#TYPE_MAX}

```gdscript
TYPE_MAX = 3
```

Represents the size of the TextureType enum.

---
