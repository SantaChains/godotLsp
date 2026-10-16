# CapsuleShape2D

**继承**：[Shape2D](class_shape2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CapsuleShape2D**

> A 2D capsule shape, intended for use in physics. Usually used to provide a shape for a CollisionShape2D.

<details><summary>官方英文描述</summary>

A 2D capsule shape, intended for use in physics. Usually used to provide a shape for a CollisionShape2D.

Performance: CapsuleShape2D is fast to check collisions against, but it is slower than RectangleShape2D and CircleShape2D.

</details>

## 方法

### get_height {#get_height}

```gdscript
float get_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mid_height {#get_mid_height}

```gdscript
float get_mid_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mid_height {#set_mid_height}

```gdscript
void set_mid_height(mid_height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The capsule's radius.

Note: The radius of a capsule cannot be greater than half of its height. Otherwise, the capsule becomes a circle. If the radius is greater than half of the height, the properties adjust to a valid value.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

The capsule's full height, including the semicircles.

Note: The height of a capsule must be at least twice its radius. Otherwise, the capsule becomes a circle. If the height is less than twice the radius, the properties adjust to a valid value.

### mid_height {#mid_height}

```gdscript
mid_height: float
```

- void set_mid_height(value: float)
- float get_mid_height()

The capsule's height, excluding the semicircles. This is the height of the central rectangular part in the middle of the capsule, and is the distance between the centers of the two semicircles. This is a wrapper for height.
