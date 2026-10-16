# VisualShaderNodeInput

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeInput**

> Gives access to input variables (built-ins) available for the shader. See the shading reference for the list of available built-ins for each shader type (check Tutorials section for link).

<details><summary>官方英文描述（中文精译进行中）</summary>

Gives access to input variables (built-ins) available for the shader. See the shading reference for the list of available built-ins for each shader type (check Tutorials section for link).

</details>

## 方法

### get_input_name {#get_input_name}

```gdscript
String get_input_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_input_real_name {#get_input_real_name}

```gdscript
String get_input_real_name() const
```

Returns a translated name of the current constant in the Godot Shader Language. E.g. "ALBEDO" if the input_name equal to "albedo".

### set_input_name {#set_input_name}

```gdscript
void set_input_name(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### input_name {#input_name}

```gdscript
input_name: StringName
```

- void set_input_name(value: String)
- String get_input_name()

One of the several input constants in lower-case style like: "vertex" (VERTEX) or "point_size" (POINT_SIZE).

## 信号

### input_type_changed {#input_type_changed}

```gdscript
signal input_type_changed()
```

Emitted when input is changed via input_name.
