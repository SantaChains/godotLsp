# VisualShaderNodeVarying

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeVarying**

> Varying values are shader variables that can be passed between shader functions, e.g. from Vertex shader to Fragment shader.

<details><summary>官方英文描述</summary>

Varying values are shader variables that can be passed between shader functions, e.g. from Vertex shader to Fragment shader.

</details>

## 方法

### get_varying_name {#get_varying_name}

```gdscript
String get_varying_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_varying_type {#get_varying_type}

```gdscript
VisualShader.VaryingType get_varying_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_varying_name {#set_varying_name}

```gdscript
void set_varying_name(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_varying_type {#set_varying_type}

```gdscript
void set_varying_type(type: VisualShader.VaryingType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### varying_name {#varying_name}

```gdscript
varying_name: StringName
```

- void set_varying_name(value: String)
- String get_varying_name()

Name of the variable. Must be unique.

### varying_type {#varying_type}

```gdscript
varying_type: int
```

- void set_varying_type(value: VaryingType)
- VaryingType get_varying_type()

Type of the variable. Determines where the variable can be accessed.
