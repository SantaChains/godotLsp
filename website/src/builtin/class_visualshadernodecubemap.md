# VisualShaderNodeCubemap

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeCubemap**

> Translated to texture(cubemap, vec3) in the shader language. Returns a color vector and alpha channel as scalar.

<details><summary>官方英文描述</summary>

Translated to texture(cubemap, vec3) in the shader language. Returns a color vector and alpha channel as scalar.

</details>

## 方法

### get_cube_map {#get_cube_map}

```gdscript
TextureLayered get_cube_map() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_source {#get_source}

```gdscript
VisualShaderNodeCubemap.Source get_source() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_type {#get_texture_type}

```gdscript
VisualShaderNodeCubemap.TextureType get_texture_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cube_map {#set_cube_map}

```gdscript
void set_cube_map(value: TextureLayered)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source {#set_source}

```gdscript
void set_source(value: VisualShaderNodeCubemap.Source)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_type {#set_texture_type}

```gdscript
void set_texture_type(value: VisualShaderNodeCubemap.TextureType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### source {#source}

```gdscript
source: int
```

- void set_source(value: Source)
- Source get_source()

Defines which source should be used for the sampling.

### cube_map {#cube_map}

```gdscript
cube_map: Cubemap,CompressedCubemap,PlaceholderCubemap,TextureCubemapRD
```

- void set_cube_map(value: TextureLayered)
- TextureLayered get_cube_map()

The Cubemap texture to sample when using SOURCE_TEXTURE as source.

### texture_type {#texture_type}

```gdscript
texture_type: int
```

- void set_texture_type(value: TextureType)
- TextureType get_texture_type()

Defines the type of data provided by the source texture.

## 枚举

### Source {#enum-Source}

```gdscript
enum Source { SOURCE_TEXTURE = 0, SOURCE_PORT = 1, SOURCE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_TEXTURE {#SOURCE_TEXTURE}

```gdscript
SOURCE_TEXTURE = 0
```

Use the Cubemap set via cube_map. If this is set to source, the samplerCube port is ignored.

#### SOURCE_PORT {#SOURCE_PORT}

```gdscript
SOURCE_PORT = 1
```

Use the Cubemap sampler reference passed via the samplerCube port. If this is set to source, the cube_map texture is ignored.

#### SOURCE_MAX {#SOURCE_MAX}

```gdscript
SOURCE_MAX = 2
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
