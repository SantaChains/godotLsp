# RDShaderSPIRV

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDShaderSPIRV**

> RDShaderSPIRV represents an RDShaderFile's SPIR-V [https://www.khronos.org/spir/] code for various shader stages, as well as possible compilation error messages. SPIR-V is a low-level intermediate shader representation. This intermediate representation is not used directly by GPUs for rendering, but it can be compiled into binary shaders that GPUs can understand. Unlike compiled shaders, SPIR-V is portable across GPU models and driver versions.

<details><summary>官方英文描述（中文精译进行中）</summary>

RDShaderSPIRV represents an RDShaderFile's SPIR-V [https://www.khronos.org/spir/] code for various shader stages, as well as possible compilation error messages. SPIR-V is a low-level intermediate shader representation. This intermediate representation is not used directly by GPUs for rendering, but it can be compiled into binary shaders that GPUs can understand. Unlike compiled shaders, SPIR-V is portable across GPU models and driver versions.

This object is used by RenderingDevice.

</details>

## 方法

### get_stage_bytecode {#get_stage_bytecode}

```gdscript
PackedByteArray get_stage_bytecode(stage: RenderingDevice.ShaderStage) const
```

Equivalent to getting one of bytecode_compute, bytecode_fragment, bytecode_tesselation_control, bytecode_tesselation_evaluation, bytecode_vertex.

### get_stage_compile_error {#get_stage_compile_error}

```gdscript
String get_stage_compile_error(stage: RenderingDevice.ShaderStage) const
```

Returns the compilation error message for the given shader stage. Equivalent to getting one of compile_error_compute, compile_error_fragment, compile_error_tesselation_control, compile_error_tesselation_evaluation, compile_error_vertex.

### set_stage_bytecode {#set_stage_bytecode}

```gdscript
void set_stage_bytecode(stage: RenderingDevice.ShaderStage, bytecode: PackedByteArray)
```

Sets the SPIR-V bytecode for the given shader stage. Equivalent to setting one of bytecode_compute, bytecode_fragment, bytecode_tesselation_control, bytecode_tesselation_evaluation, bytecode_vertex.

### set_stage_compile_error {#set_stage_compile_error}

```gdscript
void set_stage_compile_error(stage: RenderingDevice.ShaderStage, compile_error: String)
```

Sets the compilation error message for the given shader stage to compile_error. Equivalent to setting one of compile_error_compute, compile_error_fragment, compile_error_tesselation_control, compile_error_tesselation_evaluation, compile_error_vertex.

## 属性

### bytecode_vertex {#bytecode_vertex}

```gdscript
bytecode_vertex: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the vertex shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_fragment {#bytecode_fragment}

```gdscript
bytecode_fragment: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the fragment shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_tesselation_control {#bytecode_tesselation_control}

```gdscript
bytecode_tesselation_control: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the tessellation control shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_tesselation_evaluation {#bytecode_tesselation_evaluation}

```gdscript
bytecode_tesselation_evaluation: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the tessellation evaluation shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_compute {#bytecode_compute}

```gdscript
bytecode_compute: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the compute shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_raygen {#bytecode_raygen}

```gdscript
bytecode_raygen: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the ray generation shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_any_hit {#bytecode_any_hit}

```gdscript
bytecode_any_hit: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the any hit shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_closest_hit {#bytecode_closest_hit}

```gdscript
bytecode_closest_hit: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the closest hit shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_miss {#bytecode_miss}

```gdscript
bytecode_miss: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the miss shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### bytecode_intersection {#bytecode_intersection}

```gdscript
bytecode_intersection: PackedByteArray
```

- void set_stage_bytecode(stage: ShaderStage, bytecode: PackedByteArray)
- PackedByteArray get_stage_bytecode(stage: ShaderStage) const

The SPIR-V bytecode for the intersection shader stage.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### compile_error_vertex {#compile_error_vertex}

```gdscript
compile_error_vertex: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the vertex shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_fragment {#compile_error_fragment}

```gdscript
compile_error_fragment: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the fragment shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_tesselation_control {#compile_error_tesselation_control}

```gdscript
compile_error_tesselation_control: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the tessellation control shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_tesselation_evaluation {#compile_error_tesselation_evaluation}

```gdscript
compile_error_tesselation_evaluation: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the tessellation evaluation shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_compute {#compile_error_compute}

```gdscript
compile_error_compute: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the compute shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_raygen {#compile_error_raygen}

```gdscript
compile_error_raygen: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the ray generation shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_any_hit {#compile_error_any_hit}

```gdscript
compile_error_any_hit: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the any hit shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_closest_hit {#compile_error_closest_hit}

```gdscript
compile_error_closest_hit: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the closest hit shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_miss {#compile_error_miss}

```gdscript
compile_error_miss: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the miss shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.

### compile_error_intersection {#compile_error_intersection}

```gdscript
compile_error_intersection: String
```

- void set_stage_compile_error(stage: ShaderStage, compile_error: String)
- String get_stage_compile_error(stage: ShaderStage) const

The compilation error message for the intersection shader stage (set by the SPIR-V compiler and Godot). If empty, shader compilation was successful.
