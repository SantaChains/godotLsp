# VisualShaderNodeTextureParameter

**继承**：[VisualShaderNodeParameter](class_visualshadernodeparameter.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeTextureParameter**

> Performs a lookup operation on the texture provided as a uniform for the shader.

<details><summary>官方英文描述</summary>

Performs a lookup operation on the texture provided as a uniform for the shader.

</details>

## 方法

### get_color_default {#get_color_default}

```gdscript
VisualShaderNodeTextureParameter.ColorDefault get_color_default() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_filter {#get_texture_filter}

```gdscript
VisualShaderNodeTextureParameter.TextureFilter get_texture_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_repeat {#get_texture_repeat}

```gdscript
VisualShaderNodeTextureParameter.TextureRepeat get_texture_repeat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_source {#get_texture_source}

```gdscript
VisualShaderNodeTextureParameter.TextureSource get_texture_source() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_type {#get_texture_type}

```gdscript
VisualShaderNodeTextureParameter.TextureType get_texture_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_default {#set_color_default}

```gdscript
void set_color_default(color: VisualShaderNodeTextureParameter.ColorDefault)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_filter {#set_texture_filter}

```gdscript
void set_texture_filter(filter: VisualShaderNodeTextureParameter.TextureFilter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_repeat {#set_texture_repeat}

```gdscript
void set_texture_repeat(repeat: VisualShaderNodeTextureParameter.TextureRepeat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_source {#set_texture_source}

```gdscript
void set_texture_source(source: VisualShaderNodeTextureParameter.TextureSource)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_type {#set_texture_type}

```gdscript
void set_texture_type(type: VisualShaderNodeTextureParameter.TextureType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture_type {#texture_type}

```gdscript
texture_type: int
```

- void set_texture_type(value: TextureType)
- TextureType get_texture_type()

Defines the type of data provided by the source texture.

### color_default {#color_default}

```gdscript
color_default: int
```

- void set_color_default(value: ColorDefault)
- ColorDefault get_color_default()

Sets the default color if no texture is assigned to the uniform.

### texture_filter {#texture_filter}

```gdscript
texture_filter: int
```

- void set_texture_filter(value: TextureFilter)
- TextureFilter get_texture_filter()

Sets the texture filtering mode.

### texture_repeat {#texture_repeat}

```gdscript
texture_repeat: int
```

- void set_texture_repeat(value: TextureRepeat)
- TextureRepeat get_texture_repeat()

Sets the texture repeating mode.

### texture_source {#texture_source}

```gdscript
texture_source: int
```

- void set_texture_source(value: TextureSource)
- TextureSource get_texture_source()

Sets the texture source mode. Used for reading from the screen, depth, or normal_roughness texture.

## 枚举

### TextureType {#enum-TextureType}

```gdscript
enum TextureType { TYPE_DATA = 0, TYPE_COLOR = 1, TYPE_NORMAL_MAP = 2, TYPE_ANISOTROPY = 3, TYPE_MAX = 4 }
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

#### TYPE_ANISOTROPY {#TYPE_ANISOTROPY}

```gdscript
TYPE_ANISOTROPY = 3
```

Adds hint_anisotropy as hint to the uniform declaration to use for a flowmap.

#### TYPE_MAX {#TYPE_MAX}

```gdscript
TYPE_MAX = 4
```

Represents the size of the TextureType enum.

---

### ColorDefault {#enum-ColorDefault}

```gdscript
enum ColorDefault { COLOR_DEFAULT_WHITE = 0, COLOR_DEFAULT_BLACK = 1, COLOR_DEFAULT_TRANSPARENT = 2, COLOR_DEFAULT_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COLOR_DEFAULT_WHITE {#COLOR_DEFAULT_WHITE}

```gdscript
COLOR_DEFAULT_WHITE = 0
```

Defaults to fully opaque white color.

#### COLOR_DEFAULT_BLACK {#COLOR_DEFAULT_BLACK}

```gdscript
COLOR_DEFAULT_BLACK = 1
```

Defaults to fully opaque black color.

#### COLOR_DEFAULT_TRANSPARENT {#COLOR_DEFAULT_TRANSPARENT}

```gdscript
COLOR_DEFAULT_TRANSPARENT = 2
```

Defaults to fully transparent black color.

#### COLOR_DEFAULT_MAX {#COLOR_DEFAULT_MAX}

```gdscript
COLOR_DEFAULT_MAX = 3
```

Represents the size of the ColorDefault enum.

---

### TextureFilter {#enum-TextureFilter}

```gdscript
enum TextureFilter { FILTER_DEFAULT = 0, FILTER_NEAREST = 1, FILTER_LINEAR = 2, FILTER_NEAREST_MIPMAP = 3, FILTER_LINEAR_MIPMAP = 4, FILTER_NEAREST_MIPMAP_ANISOTROPIC = 5, FILTER_LINEAR_MIPMAP_ANISOTROPIC = 6, FILTER_MAX = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILTER_DEFAULT {#FILTER_DEFAULT}

```gdscript
FILTER_DEFAULT = 0
```

Sample the texture using the filter determined by the node this shader is attached to.

#### FILTER_NEAREST {#FILTER_NEAREST}

```gdscript
FILTER_NEAREST = 1
```

The texture filter reads from the nearest pixel only. This makes the texture look pixelated from up close, and grainy from a distance (due to mipmaps not being sampled).

#### FILTER_LINEAR {#FILTER_LINEAR}

```gdscript
FILTER_LINEAR = 2
```

The texture filter blends between the nearest 4 pixels. This makes the texture look smooth from up close, and grainy from a distance (due to mipmaps not being sampled).

#### FILTER_NEAREST_MIPMAP {#FILTER_NEAREST_MIPMAP}

```gdscript
FILTER_NEAREST_MIPMAP = 3
```

The texture filter reads from the nearest pixel and blends between the nearest 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true). This makes the texture look pixelated from up close, and smooth from a distance.

Use this for non-pixel art textures that may be viewed at a low scale (e.g. due to Camera2D zoom or sprite scaling), as mipmaps are important to smooth out pixels that are smaller than on-screen pixels.

#### FILTER_LINEAR_MIPMAP {#FILTER_LINEAR_MIPMAP}

```gdscript
FILTER_LINEAR_MIPMAP = 4
```

The texture filter blends between the nearest 4 pixels and between the nearest 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true). This makes the texture look smooth from up close, and smooth from a distance.

Use this for non-pixel art textures that may be viewed at a low scale (e.g. due to Camera2D zoom or sprite scaling), as mipmaps are important to smooth out pixels that are smaller than on-screen pixels.

#### FILTER_NEAREST_MIPMAP_ANISOTROPIC {#FILTER_NEAREST_MIPMAP_ANISOTROPIC}

```gdscript
FILTER_NEAREST_MIPMAP_ANISOTROPIC = 5
```

The texture filter reads from the nearest pixel and blends between 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true) based on the angle between the surface and the camera view. This makes the texture look pixelated from up close, and smooth from a distance. Anisotropic filtering improves texture quality on surfaces that are almost in line with the camera, but is slightly slower. The anisotropic filtering level can be changed by adjusting ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

Note: This texture filter is rarely useful in 2D projects. FILTER_NEAREST_MIPMAP is usually more appropriate in this case.

#### FILTER_LINEAR_MIPMAP_ANISOTROPIC {#FILTER_LINEAR_MIPMAP_ANISOTROPIC}

```gdscript
FILTER_LINEAR_MIPMAP_ANISOTROPIC = 6
```

The texture filter blends between the nearest 4 pixels and blends between 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true) based on the angle between the surface and the camera view. This makes the texture look smooth from up close, and smooth from a distance. Anisotropic filtering improves texture quality on surfaces that are almost in line with the camera, but is slightly slower. The anisotropic filtering level can be changed by adjusting ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

Note: This texture filter is rarely useful in 2D projects. FILTER_LINEAR_MIPMAP is usually more appropriate in this case.

#### FILTER_MAX {#FILTER_MAX}

```gdscript
FILTER_MAX = 7
```

Represents the size of the TextureFilter enum.

---

### TextureRepeat {#enum-TextureRepeat}

```gdscript
enum TextureRepeat { REPEAT_DEFAULT = 0, REPEAT_ENABLED = 1, REPEAT_DISABLED = 2, REPEAT_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REPEAT_DEFAULT {#REPEAT_DEFAULT}

```gdscript
REPEAT_DEFAULT = 0
```

Sample the texture using the repeat mode determined by the node this shader is attached to.

#### REPEAT_ENABLED {#REPEAT_ENABLED}

```gdscript
REPEAT_ENABLED = 1
```

Texture will repeat normally.

#### REPEAT_DISABLED {#REPEAT_DISABLED}

```gdscript
REPEAT_DISABLED = 2
```

Texture will not repeat.

#### REPEAT_MAX {#REPEAT_MAX}

```gdscript
REPEAT_MAX = 3
```

Represents the size of the TextureRepeat enum.

---

### TextureSource {#enum-TextureSource}

```gdscript
enum TextureSource { SOURCE_NONE = 0, SOURCE_SCREEN = 1, SOURCE_DEPTH = 2, SOURCE_NORMAL_ROUGHNESS = 3, SOURCE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_NONE {#SOURCE_NONE}

```gdscript
SOURCE_NONE = 0
```

The texture source is not specified in the shader.

#### SOURCE_SCREEN {#SOURCE_SCREEN}

```gdscript
SOURCE_SCREEN = 1
```

The texture source is the screen texture which captures all opaque objects drawn this frame.

#### SOURCE_DEPTH {#SOURCE_DEPTH}

```gdscript
SOURCE_DEPTH = 2
```

The texture source is the depth texture from the depth prepass.

#### SOURCE_NORMAL_ROUGHNESS {#SOURCE_NORMAL_ROUGHNESS}

```gdscript
SOURCE_NORMAL_ROUGHNESS = 3
```

The texture source is the normal-roughness buffer from the depth prepass.

#### SOURCE_MAX {#SOURCE_MAX}

```gdscript
SOURCE_MAX = 4
```

Represents the size of the TextureSource enum.

---
