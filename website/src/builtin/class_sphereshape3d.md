# SphereShape3D

**继承**：[Shape3D](class_shape3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SphereShape3D**

> A 3D sphere shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 3D sphere shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

Performance: SphereShape3D is fast to check collisions against. It is faster than BoxShape3D, CapsuleShape3D, and CylinderShape3D.

</details>

## 方法

### get_radius {#get_radius}

```gdscript
float get_radius() const
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

The sphere's radius. The shape's diameter is double the radius.
