# RDShaderSource

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDShaderSource**

> Shader source code in text form.

<details><summary>官方英文描述（中文精译进行中）</summary>

Shader source code in text form.

See also RDShaderFile. RDShaderSource is only meant to be used with the RenderingDevice API. It should not be confused with Godot's own Shader resource, which is what Godot's various nodes use for high-level shader programming.

</details>

## 方法

### get_language {#get_language}

```gdscript
RenderingDevice.ShaderLanguage get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stage_source {#get_stage_source}

```gdscript
String get_stage_source(stage: RenderingDevice.ShaderStage) const
```

Returns source code for the specified shader stage. Equivalent to getting one of source_compute, source_fragment, source_tesselation_control, source_tesselation_evaluation or source_vertex.

### set_language {#set_language}

```gdscript
void set_language(language: RenderingDevice.ShaderLanguage)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stage_source {#set_stage_source}

```gdscript
void set_stage_source(stage: RenderingDevice.ShaderStage, source: String)
```

Sets source code for the specified shader stage. Equivalent to setting one of source_compute, source_fragment, source_tesselation_control, source_tesselation_evaluation or source_vertex.

Note: If you set the compute shader source code using this method directly, remember to remove the Godot-specific hint #[compute].

## 属性

### source_vertex {#source_vertex}

```gdscript
source_vertex: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's vertex stage.

### source_fragment {#source_fragment}

```gdscript
source_fragment: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's fragment stage.

### source_tesselation_control {#source_tesselation_control}

```gdscript
source_tesselation_control: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's tessellation control stage.

### source_tesselation_evaluation {#source_tesselation_evaluation}

```gdscript
source_tesselation_evaluation: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's tessellation evaluation stage.

### source_compute {#source_compute}

```gdscript
source_compute: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's compute stage.

### source_raygen {#source_raygen}

```gdscript
source_raygen: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's ray generation stage.

### source_any_hit {#source_any_hit}

```gdscript
source_any_hit: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's any hit stage.

### source_closest_hit {#source_closest_hit}

```gdscript
source_closest_hit: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's closest hit stage.

### source_miss {#source_miss}

```gdscript
source_miss: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's miss stage.

### source_intersection {#source_intersection}

```gdscript
source_intersection: String
```

- void set_stage_source(stage: ShaderStage, source: String)
- String get_stage_source(stage: ShaderStage) const

Source code for the shader's intersection stage.

### language {#language}

```gdscript
language: int
```

- void set_language(value: ShaderLanguage)
- ShaderLanguage get_language()

The language the shader is written in.
