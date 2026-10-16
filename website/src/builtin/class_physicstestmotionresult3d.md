# PhysicsTestMotionResult3D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **PhysicsTestMotionResult3D**

> Describes the motion and collision result from PhysicsServer3D.body_test_motion().

<details><summary>官方英文描述</summary>

Describes the motion and collision result from PhysicsServer3D.body_test_motion().

</details>

## 方法

### get_collider {#get_collider}

```gdscript
Object get_collider(collision_index: int = 0) const
```

Returns the colliding body's attached Object given a collision index (the deepest collision by default), if a collision occurred.

### get_collider_id {#get_collider_id}

```gdscript
int get_collider_id(collision_index: int = 0) const
```

Returns the unique instance ID of the colliding body's attached Object given a collision index (the deepest collision by default), if a collision occurred. See Object.get_instance_id().

### get_collider_rid {#get_collider_rid}

```gdscript
RID get_collider_rid(collision_index: int = 0) const
```

Returns the colliding body's RID used by the PhysicsServer3D given a collision index (the deepest collision by default), if a collision occurred.

### get_collider_shape {#get_collider_shape}

```gdscript
int get_collider_shape(collision_index: int = 0) const
```

Returns the colliding body's shape index given a collision index (the deepest collision by default), if a collision occurred. See CollisionObject3D.

### get_collider_velocity {#get_collider_velocity}

```gdscript
Vector3 get_collider_velocity(collision_index: int = 0) const
```

Returns the colliding body's velocity given a collision index (the deepest collision by default), if a collision occurred.

### get_collision_count {#get_collision_count}

```gdscript
int get_collision_count() const
```

Returns the number of detected collisions.

### get_collision_depth {#get_collision_depth}

```gdscript
float get_collision_depth(collision_index: int = 0) const
```

Returns the length of overlap along the collision normal given a collision index (the deepest collision by default), if a collision occurred.

### get_collision_local_shape {#get_collision_local_shape}

```gdscript
int get_collision_local_shape(collision_index: int = 0) const
```

Returns the moving object's colliding shape given a collision index (the deepest collision by default), if a collision occurred.

### get_collision_normal {#get_collision_normal}

```gdscript
Vector3 get_collision_normal(collision_index: int = 0) const
```

Returns the colliding body's shape's normal at the point of collision given a collision index (the deepest collision by default), if a collision occurred.

### get_collision_point {#get_collision_point}

```gdscript
Vector3 get_collision_point(collision_index: int = 0) const
```

Returns the point of collision in global coordinates given a collision index (the deepest collision by default), if a collision occurred.

### get_collision_safe_fraction {#get_collision_safe_fraction}

```gdscript
float get_collision_safe_fraction() const
```

Returns the maximum fraction of the motion that can occur without a collision, between 0 and 1.

### get_collision_unsafe_fraction {#get_collision_unsafe_fraction}

```gdscript
float get_collision_unsafe_fraction() const
```

Returns the minimum fraction of the motion needed to collide, if a collision occurred, between 0 and 1.

### get_remainder {#get_remainder}

```gdscript
Vector3 get_remainder() const
```

Returns the moving object's remaining movement vector.

### get_travel {#get_travel}

```gdscript
Vector3 get_travel() const
```

Returns the moving object's travel before collision.
