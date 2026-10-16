# Shape3D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Shape3D**

> Abstract base class for all 3D shapes, intended for use in physics.

<details><summary>官方英文描述（中文精译进行中）</summary>

Abstract base class for all 3D shapes, intended for use in physics.

Performance: Primitive shapes, especially SphereShape3D, are fast to check collisions against. ConvexPolygonShape3D and HeightMapShape3D are slower, and ConcavePolygonShape3D is the slowest.

</details>

## 方法

### get_custom_solver_bias {#get_custom_solver_bias}

```gdscript
float get_custom_solver_bias() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_mesh {#get_debug_mesh}

```gdscript
ArrayMesh get_debug_mesh()
```

Returns the ArrayMesh used to draw the debug collision for this Shape3D.

### get_margin {#get_margin}

```gdscript
float get_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_solver_bias {#set_custom_solver_bias}

```gdscript
void set_custom_solver_bias(bias: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### custom_solver_bias {#custom_solver_bias}

```gdscript
custom_solver_bias: float
```

- void set_custom_solver_bias(value: float)
- float get_custom_solver_bias()

The shape's custom solver bias. Defines how much bodies react to enforce contact separation when this shape is involved.

When set to 0, the default value from ProjectSettings.physics/3d/solver/default_contact_bias is used.

Note: custom_solver_bias is only effective when using GodotPhysics3D. It has no effect when using Jolt Physics.

### margin {#margin}

```gdscript
margin: float
```

- void set_margin(value: float)
- float get_margin()

The collision margin for the shape. This is not used in Godot Physics.

Collision margins allow collision detection to be more efficient by adding an extra shell around shapes. Collision algorithms are more expensive when objects overlap by more than their margin, so a higher value for margins is better for performance, at the cost of accuracy around edges as it makes them less sharp.
