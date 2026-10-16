# JointLimitationCone3D

**继承**：[JointLimitation3D](class_jointlimitation3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **JointLimitationCone3D**

> A cone shape limitation that interacts with ChainIK3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

A cone shape limitation that interacts with ChainIK3D.

</details>

## 方法

### get_angle {#get_angle}

```gdscript
float get_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angle {#set_angle}

```gdscript
void set_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### angle {#angle}

```gdscript
angle: float
```

- void set_angle(value: float)
- float get_angle()

The radius range of the hole made by the cone.

0 degrees makes a sphere without hole, 180 degrees makes a hemisphere, and 360 degrees become empty (no limitation).
