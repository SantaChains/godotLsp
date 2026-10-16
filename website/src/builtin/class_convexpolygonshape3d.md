# ConvexPolygonShape3D

**继承**：[Shape3D](class_shape3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ConvexPolygonShape3D**

> A 3D convex polyhedron shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

<details><summary>官方英文描述</summary>

A 3D convex polyhedron shape, intended for use in physics. Usually used to provide a shape for a CollisionShape3D.

ConvexPolygonShape3D is solid, which means it detects collisions from objects that are fully inside it, unlike ConcavePolygonShape3D which is hollow. This makes it more suitable for both detection and physics.

Convex decomposition: A concave polyhedron can be split up into several convex polyhedra. This allows dynamic physics bodies to have complex concave collisions (at a performance cost) and can be achieved by using several ConvexPolygonShape3D nodes. To generate a convex decomposition from a mesh, select the MeshInstance3D node, go to the Mesh menu that appears above the viewport, and choose Create Multiple Convex Collision Siblings. Alternatively, MeshInstance3D.create_multiple_convex_collisions() can be called in a script to perform this decomposition at run-time.

Performance: ConvexPolygonShape3D is faster to check collisions against compared to ConcavePolygonShape3D, but it is slower than primitive collision shapes such as SphereShape3D and BoxShape3D. Its use should generally be limited to medium-sized objects that cannot have their collision accurately represented by primitive shapes.

</details>

## 方法

### get_points {#get_points}

```gdscript
PackedVector3Array get_points() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_points {#set_points}

```gdscript
void set_points(points: PackedVector3Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### points {#points}

```gdscript
points: Array
```

- void set_points(value: PackedVector3Array)
- PackedVector3Array get_points()

The list of 3D points forming the convex polygon shape.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.
