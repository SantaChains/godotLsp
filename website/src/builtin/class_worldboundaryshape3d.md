# WorldBoundaryShape3D

**继承**：[Shape3D](class_shape3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **WorldBoundaryShape3D**

> A 3D world boundary shape, intended for use in physics. WorldBoundaryShape3D works like an infinite plane that forces all physics bodies to stay above it. The plane's normal determines which direction is considered as "above" and in the editor, the line over the plane represents this direction. It can for example be used for endless flat floors.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 3D world boundary shape, intended for use in physics. WorldBoundaryShape3D works like an infinite plane that forces all physics bodies to stay above it. The plane's normal determines which direction is considered as "above" and in the editor, the line over the plane represents this direction. It can for example be used for endless flat floors.

Note: When the physics engine is set to Jolt Physics in the project settings (ProjectSettings.physics/3d/physics_engine), WorldBoundaryShape3D has a finite size (centered at the shape's origin). It can be adjusted by changing ProjectSettings.physics/jolt_physics_3d/limits/world_boundary_shape_size.

</details>

## 方法

### get_plane {#get_plane}

```gdscript
Plane get_plane() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_plane {#set_plane}

```gdscript
void set_plane(plane: Plane)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### plane {#plane}

```gdscript
plane: Plane
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
