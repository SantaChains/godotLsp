# Path3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Path3D**

> Can have PathFollow3D child nodes moving along the Curve3D. See PathFollow3D for more information on the usage.

<details><summary>官方英文描述（中文精译进行中）</summary>

Can have PathFollow3D child nodes moving along the Curve3D. See PathFollow3D for more information on the usage.

Note that the path is considered as relative to the moved nodes (children of PathFollow3D). As such, the curve should usually start with a zero vector (0, 0, 0).

</details>

## 方法

### get_curve {#get_curve}

```gdscript
Curve3D get_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_custom_color {#get_debug_custom_color}

```gdscript
Color get_debug_custom_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve {#set_curve}

```gdscript
void set_curve(curve: Curve3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_custom_color {#set_debug_custom_color}

```gdscript
void set_debug_custom_color(debug_custom_color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### curve {#curve}

```gdscript
curve: Curve3D
```

- void set_curve(value: Curve3D)
- Curve3D get_curve()

A Curve3D describing the path.

### debug_custom_color {#debug_custom_color}

```gdscript
debug_custom_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 信号

### curve_changed {#curve_changed}

```gdscript
signal curve_changed()
```

Emitted when the curve changes.

### debug_color_changed {#debug_color_changed}

```gdscript
signal debug_color_changed()
```

Emitted when the debug_custom_color changes.
