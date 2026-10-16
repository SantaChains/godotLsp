# SeparationRayShape2D

**继承**：[Shape2D](class_shape2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SeparationRayShape2D**

> A 2D ray shape, intended for use in physics. Usually used to provide a shape for a CollisionShape2D. When a SeparationRayShape2D collides with an object, it tries to separate itself from it by moving its endpoint to the collision point. For example, a SeparationRayShape2D next to a character can allow it to instantly move up when touching stairs.

<details><summary>官方英文描述</summary>

A 2D ray shape, intended for use in physics. Usually used to provide a shape for a CollisionShape2D. When a SeparationRayShape2D collides with an object, it tries to separate itself from it by moving its endpoint to the collision point. For example, a SeparationRayShape2D next to a character can allow it to instantly move up when touching stairs.

</details>

## 方法

### get_length {#get_length}

```gdscript
float get_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_slide_on_slope {#get_slide_on_slope}

```gdscript
bool get_slide_on_slope() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_length {#set_length}

```gdscript
void set_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_slide_on_slope {#set_slide_on_slope}

```gdscript
void set_slide_on_slope(active: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### length {#length}

```gdscript
length: float
```

- void set_length(value: float)
- float get_length()

The ray's length.

### slide_on_slope {#slide_on_slope}

```gdscript
slide_on_slope: bool
```

- void set_slide_on_slope(value: bool)
- bool get_slide_on_slope()

If false (default), the shape always separates and returns a normal along its own direction.

If true, the shape can return the correct normal and separate in any direction, allowing sliding motion on slopes.
