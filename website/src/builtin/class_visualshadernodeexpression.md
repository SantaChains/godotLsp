# VisualShaderNodeExpression

**继承**：[VisualShaderNodeGroupBase](class_visualshadernodegroupbase.md) → [VisualShaderNodeResizableBase](class_visualshadernoderesizablebase.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeExpression**

> Custom Godot Shading Language expression, with a custom number of input and output ports.

<details><summary>官方英文描述</summary>

Custom Godot Shading Language expression, with a custom number of input and output ports.

The provided code is directly injected into the graph's matching shader function (vertex, fragment, or light), so it cannot be used to declare functions, varyings, uniforms, or global constants. See VisualShaderNodeGlobalExpression for such global definitions.

</details>

## 方法

### get_expression {#get_expression}

```gdscript
String get_expression() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_expression {#set_expression}

```gdscript
void set_expression(expression: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### expression {#expression}

```gdscript
expression: String
```

- void set_expression(value: String)
- String get_expression()

An expression in Godot Shading Language, which will be injected at the start of the graph's matching shader function (vertex, fragment, or light), and thus cannot be used to declare functions, varyings, uniforms, or global constants.
