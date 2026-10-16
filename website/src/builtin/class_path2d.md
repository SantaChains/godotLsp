# Path2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Path2D**

> Can have PathFollow2D child nodes moving along the Curve2D. See PathFollow2D for more information on usage.

<details><summary>官方英文描述</summary>

Can have PathFollow2D child nodes moving along the Curve2D. See PathFollow2D for more information on usage.

Note: The path is considered as relative to the moved nodes (children of PathFollow2D). As such, the curve should usually start with a zero vector ((0, 0)).

</details>

## 方法

### get_curve {#get_curve}

```gdscript
Curve2D get_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve {#set_curve}

```gdscript
void set_curve(curve: Curve2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### curve {#curve}

```gdscript
curve: Curve2D
```

- void set_curve(value: Curve2D)
- Curve2D get_curve()

A Curve2D describing the path.
