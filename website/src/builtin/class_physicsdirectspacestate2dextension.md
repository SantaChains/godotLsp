# PhysicsDirectSpaceState2DExtension

**继承**：[PhysicsDirectSpaceState2D](class_physicsdirectspacestate2d.md) → [Object](class_object.md) → **PhysicsDirectSpaceState2DExtension**

> This class extends PhysicsDirectSpaceState2D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

<details><summary>官方英文描述</summary>

This class extends PhysicsDirectSpaceState2D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

Intended for use with GDExtension to create custom implementations of PhysicsDirectSpaceState2D.

</details>

## 方法

### _cast_motion {#_cast_motion}

```gdscript
bool _cast_motion(shape_rid: RID, transform: Transform2D, motion: Vector2, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_closest_safe: float*, r_closest_unsafe: float*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _collide_shape {#_collide_shape}

```gdscript
bool _collide_shape(shape_rid: RID, transform: Transform2D, motion: Vector2, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_results: void*, max_results: int, r_result_count: int32_t*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _intersect_point {#_intersect_point}

```gdscript
int _intersect_point(position: Vector2, canvas_instance_id: int, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_results: PhysicsServer2DExtensionShapeResult*, max_results: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _intersect_ray {#_intersect_ray}

```gdscript
bool _intersect_ray(from: Vector2, to: Vector2, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, hit_from_inside: bool, r_result: PhysicsServer2DExtensionRayResult*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _intersect_shape {#_intersect_shape}

```gdscript
int _intersect_shape(shape_rid: RID, transform: Transform2D, motion: Vector2, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_result: PhysicsServer2DExtensionShapeResult*, max_results: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _rest_info {#_rest_info}

```gdscript
bool _rest_info(shape_rid: RID, transform: Transform2D, motion: Vector2, margin: float, collision_mask: int, collide_with_bodies: bool, collide_with_areas: bool, r_rest_info: PhysicsServer2DExtensionShapeRestInfo*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### is_body_excluded_from_query {#is_body_excluded_from_query}

```gdscript
bool is_body_excluded_from_query(body: RID) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!
