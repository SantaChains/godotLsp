# BoxShape3D

**继承**：[Shape3D](class_shape3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **BoxShape3D**

> A 3D box shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 3D box shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

Performance: BoxShape3D is fast to check collisions against. It is faster than CapsuleShape3D and CylinderShape3D, but slower than SphereShape3D.

</details>

## 方法

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector3
```

- void set_size(value: Vector3)
- Vector3 get_size()

The box's width, height and depth.
