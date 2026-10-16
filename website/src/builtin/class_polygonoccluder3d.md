# PolygonOccluder3D

**继承**：[Occluder3D](class_occluder3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PolygonOccluder3D**

> PolygonOccluder3D stores a polygon shape that can be used by the engine's occlusion culling system. When an OccluderInstance3D with a PolygonOccluder3D is selected in the editor, an editor will appear at the top of the 3D viewport so you can add/remove points. All points must be placed on the same 2D plane, which means it is not possible to create arbitrary 3D shapes with a single PolygonOccluder3D. To use arbitrary 3D shapes as occluders, use ArrayOccluder3D or OccluderInstance3D's baking feature instead.

<details><summary>官方英文描述（中文精译进行中）</summary>

PolygonOccluder3D stores a polygon shape that can be used by the engine's occlusion culling system. When an OccluderInstance3D with a PolygonOccluder3D is selected in the editor, an editor will appear at the top of the 3D viewport so you can add/remove points. All points must be placed on the same 2D plane, which means it is not possible to create arbitrary 3D shapes with a single PolygonOccluder3D. To use arbitrary 3D shapes as occluders, use ArrayOccluder3D or OccluderInstance3D's baking feature instead.

See OccluderInstance3D's documentation for instructions on setting up occlusion culling.

</details>

## 方法

### get_polygon {#get_polygon}

```gdscript
PackedVector2Array get_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polygon {#set_polygon}

```gdscript
void set_polygon(polygon: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### polygon {#polygon}

```gdscript
polygon: PackedVector2Array
```

- void set_polygon(value: PackedVector2Array)
- PackedVector2Array get_polygon()

The polygon to use for occlusion culling. The polygon can be convex or concave, but it should have as few points as possible to maximize performance.

The polygon must not have intersecting lines. Otherwise, triangulation will fail (with an error message printed).

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.
