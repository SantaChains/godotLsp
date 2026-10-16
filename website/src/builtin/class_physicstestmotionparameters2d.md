# PhysicsTestMotionParameters2D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **PhysicsTestMotionParameters2D**

> By changing various properties of this object, such as the motion, you can configure the parameters for PhysicsServer2D.body_test_motion().

<details><summary>官方英文描述（中文精译进行中）</summary>

By changing various properties of this object, such as the motion, you can configure the parameters for PhysicsServer2D.body_test_motion().

</details>

## 方法

### get_exclude_bodies {#get_exclude_bodies}

```gdscript
typedarray::RID get_exclude_bodies() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exclude_objects {#get_exclude_objects}

```gdscript
typedarray::int get_exclude_objects() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_from {#get_from}

```gdscript
Transform2D get_from() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_margin {#get_margin}

```gdscript
float get_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_motion {#get_motion}

```gdscript
Vector2 get_motion() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_separation_ray_enabled {#is_collide_separation_ray_enabled}

```gdscript
bool is_collide_separation_ray_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_recovery_as_collision_enabled {#is_recovery_as_collision_enabled}

```gdscript
bool is_recovery_as_collision_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collide_separation_ray_enabled {#set_collide_separation_ray_enabled}

```gdscript
void set_collide_separation_ray_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclude_bodies {#set_exclude_bodies}

```gdscript
void set_exclude_bodies(exclude_list: typedarray::RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclude_objects {#set_exclude_objects}

```gdscript
void set_exclude_objects(exclude_list: typedarray::int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_from {#set_from}

```gdscript
void set_from(from: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motion {#set_motion}

```gdscript
void set_motion(motion: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_recovery_as_collision_enabled {#set_recovery_as_collision_enabled}

```gdscript
void set_recovery_as_collision_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### from {#from}

```gdscript
from: Transform2D
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### motion {#motion}

```gdscript
motion: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### margin {#margin}

```gdscript
margin: float
```

- void set_margin(value: float)
- float get_margin()

Increases the size of the shapes involved in the collision detection.

### collide_separation_ray {#collide_separation_ray}

```gdscript
collide_separation_ray: bool
```

- void set_collide_separation_ray_enabled(value: bool)
- bool is_collide_separation_ray_enabled()

If set to true, shapes of type PhysicsServer2D.SHAPE_SEPARATION_RAY are used to detect collisions and can stop the motion. Can be useful when snapping to the ground.

If set to false, shapes of type PhysicsServer2D.SHAPE_SEPARATION_RAY are only used for separation when overlapping with other bodies. That's the main use for separation ray shapes.

### exclude_bodies {#exclude_bodies}

```gdscript
exclude_bodies: typedarray::RID
```

- void set_exclude_bodies(value: Array[RID])
- Array[RID] get_exclude_bodies()

Optional array of body RID to exclude from collision. Use CollisionObject2D.get_rid() to get the RID associated with a CollisionObject2D-derived node.

### exclude_objects {#exclude_objects}

```gdscript
exclude_objects: Array
```

- void set_exclude_objects(value: Array[int])
- Array[int] get_exclude_objects()

Optional array of object unique instance ID to exclude from collision. See Object.get_instance_id().

### recovery_as_collision {#recovery_as_collision}

```gdscript
recovery_as_collision: bool
```

- void set_recovery_as_collision_enabled(value: bool)
- bool is_recovery_as_collision_enabled()

If set to true, any depenetration from the recovery phase is reported as a collision; this is used e.g. by CharacterBody2D for improving floor detection during floor snapping.

If set to false, only collisions resulting from the motion are reported, which is generally the desired behavior.
