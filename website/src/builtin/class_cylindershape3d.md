# CylinderShape3D

**继承**：[Shape3D](class_shape3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CylinderShape3D**

> A 3D cylinder shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 3D cylinder shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

Note: There are several known bugs with cylinder collision shapes. Using CapsuleShape3D or BoxShape3D instead is recommended.

Performance: CylinderShape3D is fast to check collisions against, but it is slower than CapsuleShape3D, BoxShape3D, and SphereShape3D.

</details>

## 方法

### get_height {#get_height}

```gdscript
float get_height() const
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

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

The cylinder's height.

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The cylinder's radius.
