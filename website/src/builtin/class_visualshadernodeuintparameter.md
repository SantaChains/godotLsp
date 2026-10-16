# VisualShaderNodeUIntParameter

**继承**：[VisualShaderNodeParameter](class_visualshadernodeparameter.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeUIntParameter**

> A VisualShaderNodeParameter of type unsigned int. Offers additional customization for range of accepted values.

<details><summary>官方英文描述</summary>

A VisualShaderNodeParameter of type unsigned int. Offers additional customization for range of accepted values.

</details>

## 方法

### get_default_value {#get_default_value}

```gdscript
int get_default_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_default_value_enabled {#is_default_value_enabled}

```gdscript
bool is_default_value_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_value {#set_default_value}

```gdscript
void set_default_value(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_value_enabled {#set_default_value_enabled}

```gdscript
void set_default_value_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### default_value_enabled {#default_value_enabled}

```gdscript
default_value_enabled: bool
```

- void set_default_value_enabled(value: bool)
- bool is_default_value_enabled()

If true, the node will have a custom default value.

### default_value {#default_value}

```gdscript
default_value: int
```

- void set_default_value(value: int)
- int get_default_value()

Default value of this parameter, which will be used if not set externally. default_value_enabled must be enabled; defaults to 0 otherwise.
