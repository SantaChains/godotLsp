# Shader

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Shader**

> A custom shader program implemented in the Godot shading language, saved with the .gdshader extension.

<details><summary>官方英文描述（中文精译进行中）</summary>

A custom shader program implemented in the Godot shading language, saved with the .gdshader extension.

This class is used by a ShaderMaterial and allows you to write your own custom behavior for rendering visual items or updating particle information. For a detailed explanation and usage, please see the tutorials linked below.

</details>

## 方法

### get_code {#get_code}

```gdscript
String get_code() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_texture_parameter {#get_default_texture_parameter}

```gdscript
Texture get_default_texture_parameter(name: StringName, index: int = 0) const
```

Returns the texture that is set as default for the specified parameter.

Note: name must match the name of the uniform in the code exactly.

Note: If the sampler array is used use index to access the specified texture.

### get_mode {#get_mode}

```gdscript
Shader.Mode get_mode() const
```

Returns the shader mode for the shader.

### get_shader_uniform_list {#get_shader_uniform_list}

```gdscript
Array get_shader_uniform_list(get_groups: bool = false)
```

Returns the list of shader uniforms that can be assigned to a ShaderMaterial, for use with ShaderMaterial.set_shader_parameter() and ShaderMaterial.get_shader_parameter(). The parameters returned are contained in dictionaries in a similar format to the ones returned by Object.get_property_list().

If argument get_groups is true, parameter grouping hints are also included in the list.

### inspect_native_shader_code {#inspect_native_shader_code}

```gdscript
void inspect_native_shader_code()
```

Only available when running in the editor. Opens a popup that visualizes the generated shader code, including all variants and internal shader code. See also Material.inspect_native_shader_code().

### set_code {#set_code}

```gdscript
void set_code(code: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_texture_parameter {#set_default_texture_parameter}

```gdscript
void set_default_texture_parameter(name: StringName, texture: Texture, index: int = 0)
```

Sets the default texture to be used with a texture uniform. The default is used if a texture is not set in the ShaderMaterial.

Note: name must match the name of the uniform in the code exactly.

Note: If the sampler array is used use index to access the specified texture.

## 属性

### code {#code}

```gdscript
code: String
```

- void set_code(value: String)
- String get_code()

Returns the shader's code as the user has written it, not the full generated code used internally.

## 枚举

### Mode {#enum-Mode}

```gdscript
enum Mode { MODE_SPATIAL = 0, MODE_CANVAS_ITEM = 1, MODE_PARTICLES = 2, MODE_SKY = 3, MODE_FOG = 4, MODE_TEXTURE_BLIT = 5 }
```

Mode MODE_SPATIAL = 0

Mode used to draw all 3D objects.

Mode MODE_CANVAS_ITEM = 1

Mode used to draw all 2D objects.

Mode MODE_PARTICLES = 2

Mode used to calculate particle information on a per-particle basis. Not used for drawing.

Mode MODE_SKY = 3

Mode used for drawing skies. Only works with shaders attached to Sky objects.

Mode MODE_FOG = 4

Mode used for setting the color and density of volumetric fog effect.

Mode MODE_TEXTURE_BLIT = 5

Mode used for drawing to DrawableTexture resources via blit calls.

#### MODE_SPATIAL {#MODE_SPATIAL}

```gdscript
MODE_SPATIAL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_CANVAS_ITEM {#MODE_CANVAS_ITEM}

```gdscript
MODE_CANVAS_ITEM = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_PARTICLES {#MODE_PARTICLES}

```gdscript
MODE_PARTICLES = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_SKY {#MODE_SKY}

```gdscript
MODE_SKY = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_FOG {#MODE_FOG}

```gdscript
MODE_FOG = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_TEXTURE_BLIT {#MODE_TEXTURE_BLIT}

```gdscript
MODE_TEXTURE_BLIT = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
