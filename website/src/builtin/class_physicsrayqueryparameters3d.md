# PhysicsRayQueryParameters3D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **PhysicsRayQueryParameters3D**

> By changing various properties of this object, such as the ray position, you can configure the parameters for PhysicsDirectSpaceState3D.intersect_ray().

<details><summary>官方英文描述（中文精译进行中）</summary>

By changing various properties of this object, such as the ray position, you can configure the parameters for PhysicsDirectSpaceState3D.intersect_ray().

</details>

## 方法

### create {#create}

```gdscript
static PhysicsRayQueryParameters3D create(from: Vector3, to: Vector3, collision_mask: int = 4294967295, exclude: typedarray::RID = Array[RID]([]))
```

Returns a new, pre-configured PhysicsRayQueryParameters3D object. Use it to quickly create query parameters using the most common options.

```gdscript
var query = PhysicsRayQueryParameters3D.create(position, position + Vector3(0, -10, 0))
var collision = get_world_3d().direct_space_state.intersect_ray(query)
```

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exclude {#get_exclude}

```gdscript
typedarray::RID get_exclude() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_from {#get_from}

```gdscript
Vector3 get_from() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_to {#get_to}

```gdscript
Vector3 get_to() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_with_areas_enabled {#is_collide_with_areas_enabled}

```gdscript
bool is_collide_with_areas_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_with_bodies_enabled {#is_collide_with_bodies_enabled}

```gdscript
bool is_collide_with_bodies_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hit_back_faces_enabled {#is_hit_back_faces_enabled}

```gdscript
bool is_hit_back_faces_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hit_from_inside_enabled {#is_hit_from_inside_enabled}

```gdscript
bool is_hit_from_inside_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collide_with_areas {#set_collide_with_areas}

```gdscript
void set_collide_with_areas(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collide_with_bodies {#set_collide_with_bodies}

```gdscript
void set_collide_with_bodies(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(collision_mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclude {#set_exclude}

```gdscript
void set_exclude(exclude: typedarray::RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_from {#set_from}

```gdscript
void set_from(from: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hit_back_faces {#set_hit_back_faces}

```gdscript
void set_hit_back_faces(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hit_from_inside {#set_hit_from_inside}

```gdscript
void set_hit_from_inside(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_to {#set_to}

```gdscript
void set_to(to: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### from {#from}

```gdscript
from: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to {#to}

```gdscript
to: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The physics layers the query will detect (as a bitmask). By default, all collision layers are detected. See Collision layers and masks in the documentation for more information.

### exclude {#exclude}

```gdscript
exclude: typedarray::RID
```

- void set_exclude(value: Array[RID])
- Array[RID] get_exclude()

The list of object RIDs that will be excluded from collisions. Use CollisionObject3D.get_rid() to get the RID associated with a CollisionObject3D-derived node.

Note: The returned array is copied and any changes to it will not update the original property value. To update the value you need to modify the returned array, and then assign it to the property again.

### collide_with_bodies {#collide_with_bodies}

```gdscript
collide_with_bodies: bool
```

- void set_collide_with_bodies(value: bool)
- bool is_collide_with_bodies_enabled()

If true, the query will take PhysicsBody3Ds into account.

### collide_with_areas {#collide_with_areas}

```gdscript
collide_with_areas: bool
```

- void set_collide_with_areas(value: bool)
- bool is_collide_with_areas_enabled()

If true, the query will take Area3Ds into account.

### hit_from_inside {#hit_from_inside}

```gdscript
hit_from_inside: bool
```

- void set_hit_from_inside(value: bool)
- bool is_hit_from_inside_enabled()

If true, the query will detect a hit when starting inside shapes. In this case the collision normal will be Vector3(0, 0, 0). Does not affect concave polygon shapes or heightmap shapes.

### hit_back_faces {#hit_back_faces}

```gdscript
hit_back_faces: bool
```

- void set_hit_back_faces(value: bool)
- bool is_hit_back_faces_enabled()

If true, the query will hit back faces with concave polygon shapes with back face enabled or heightmap shapes.
