# RDShaderFile

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDShaderFile**

> Compiled shader file in SPIR-V form.

<details><summary>官方英文描述（中文精译进行中）</summary>

Compiled shader file in SPIR-V form.

See also RDShaderSource. RDShaderFile is only meant to be used with the RenderingDevice API. It should not be confused with Godot's own Shader resource, which is what Godot's various nodes use for high-level shader programming.

</details>

## 方法

### get_base_error {#get_base_error}

```gdscript
String get_base_error() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spirv {#get_spirv}

```gdscript
RDShaderSPIRV get_spirv(version: StringName = &"") const
```

Returns the SPIR-V intermediate representation for the specified shader version.

### get_version_list {#get_version_list}

```gdscript
typedarray::StringName get_version_list() const
```

Returns the list of compiled versions for this shader.

### set_base_error {#set_base_error}

```gdscript
void set_base_error(error: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bytecode {#set_bytecode}

```gdscript
void set_bytecode(bytecode: RDShaderSPIRV, version: StringName = &"")
```

Sets the SPIR-V bytecode that will be compiled for the specified version.

## 属性

### base_error {#base_error}

```gdscript
base_error: String
```

- void set_base_error(value: String)
- String get_base_error()

The base compilation error message, which indicates errors not related to a specific shader stage if non-empty. If empty, shader compilation is not necessarily successful (check RDShaderSPIRV's error message members).
