# WorldBoundaryShape2D

**继承**：[Shape2D](class_shape2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **WorldBoundaryShape2D**

> A 2D world boundary shape, intended for use in physics. WorldBoundaryShape2D works like an infinite straight line that forces all physics bodies to stay above it. The line's normal determines which direction is considered as "above" and in the editor, the smaller line over it represents this direction. It can for example be used for endless flat floors.

<details><summary>官方英文描述</summary>

A 2D world boundary shape, intended for use in physics. WorldBoundaryShape2D works like an infinite straight line that forces all physics bodies to stay above it. The line's normal determines which direction is considered as "above" and in the editor, the smaller line over it represents this direction. It can for example be used for endless flat floors.

</details>

## 方法

### get_distance {#get_distance}

```gdscript
float get_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_normal {#get_normal}

```gdscript
Vector2 get_normal() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_distance {#set_distance}

```gdscript
void set_distance(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_normal {#set_normal}

```gdscript
void set_normal(normal: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### normal {#normal}

```gdscript
normal: Vector2
```

- void set_normal(value: Vector2)
- Vector2 get_normal()

The line's normal, typically a unit vector. Its direction indicates the non-colliding half-plane. Can be of any length but zero. Defaults to Vector2.UP.

### distance {#distance}

```gdscript
distance: float
```

- void set_distance(value: float)
- float get_distance()

The distance from the origin to the line, expressed in terms of normal (according to its direction and magnitude). Actual absolute distance from the origin to the line can be calculated as abs(distance) / normal.length().

In the scalar equation of the line ax + by = d, this is d, while the (a, b) coordinates are represented by the normal property.
