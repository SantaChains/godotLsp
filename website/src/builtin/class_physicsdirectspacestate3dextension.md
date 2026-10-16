# PhysicsDirectSpaceState3DExtension

**继承**：[PhysicsDirectSpaceState3D](class_physicsdirectspacestate3d.md) → [Object](class_object.md) → **PhysicsDirectSpaceState3DExtension**

> This class extends PhysicsDirectSpaceState3D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

<details><summary>官方英文描述</summary>

This class extends PhysicsDirectSpaceState3D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

Intended for use with GDExtension to create custom implementations of PhysicsDirectSpaceState3D.

</details>

## 方法

### _cast_motion {#_cast_motion}

```gdscript
bool _cast_motion(shape_rid: RID, transform: Transform3D, motion: Vector3, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_closest_safe: float*, r_closest_unsafe: float*, r_info: PhysicsServer3DExtensionShapeRestInfo*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _collide_shape {#_collide_shape}

```gdscript
bool _collide_shape(shape_rid: RID, transform: Transform3D, motion: Vector3, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_results: void*, max_results: int, r_result_count: int32_t*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_closest_point_to_object_volume {#_get_closest_point_to_object_volume}

```gdscript
Vector3 _get_closest_point_to_object_volume(object: RID, point: Vector3) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _intersect_point {#_intersect_point}

```gdscript
int _intersect_point(position: Vector3, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_results: PhysicsServer3DExtensionShapeResult*, max_results: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _intersect_ray {#_intersect_ray}

```gdscript
bool _intersect_ray(from: Vector3, to: Vector3, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, hit_from_inside: bool, hit_back_faces: bool, pick_ray: bool, r_result: PhysicsServer3DExtensionRayResult*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _intersect_shape {#_intersect_shape}

```gdscript
int _intersect_shape(shape_rid: RID, transform: Transform3D, motion: Vector3, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_result_count: PhysicsServer3DExtensionShapeResult*, max_results: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _rest_info {#_rest_info}

```gdscript
bool _rest_info(shape_rid: RID, transform: Transform3D, motion: Vector3, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_rest_info: PhysicsServer3DExtensionShapeRestInfo*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### is_body_excluded_from_query {#is_body_excluded_from_query}

```gdscript
bool is_body_excluded_from_query(body: RID) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!
