# KinematicCollision3D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **KinematicCollision3D**

> Holds collision data from the movement of a PhysicsBody3D, usually from PhysicsBody3D.move_and_collide(). When a PhysicsBody3D is moved, it stops if it detects a collision with another body. If a collision is detected, a KinematicCollision3D object is returned.

<details><summary>官方英文描述（中文精译进行中）</summary>

Holds collision data from the movement of a PhysicsBody3D, usually from PhysicsBody3D.move_and_collide(). When a PhysicsBody3D is moved, it stops if it detects a collision with another body. If a collision is detected, a KinematicCollision3D object is returned.

The collision data includes the colliding object, the remaining motion, and the collision position. This data can be used to determine a custom response to the collision.

</details>

## 方法

### get_angle {#get_angle}

```gdscript
float get_angle(collision_index: int = 0, up_direction: Vector3 = Vector3(0, 1, 0)) const
```

Returns the collision angle according to up_direction, which is Vector3.UP by default. This value is always positive.

### get_collider {#get_collider}

```gdscript
Object get_collider(collision_index: int = 0) const
```

Returns the colliding body's attached Object given a collision index (the deepest collision by default).

### get_collider_id {#get_collider_id}

```gdscript
int get_collider_id(collision_index: int = 0) const
```

Returns the unique instance ID of the colliding body's attached Object given a collision index (the deepest collision by default). See Object.get_instance_id().

### get_collider_rid {#get_collider_rid}

```gdscript
RID get_collider_rid(collision_index: int = 0) const
```

Returns the colliding body's RID used by the PhysicsServer3D given a collision index (the deepest collision by default).

### get_collider_shape {#get_collider_shape}

```gdscript
Object get_collider_shape(collision_index: int = 0) const
```

Returns the colliding body's shape given a collision index (the deepest collision by default).

### get_collider_shape_index {#get_collider_shape_index}

```gdscript
int get_collider_shape_index(collision_index: int = 0) const
```

Returns the colliding body's shape index given a collision index (the deepest collision by default). See CollisionObject3D.

### get_collider_velocity {#get_collider_velocity}

```gdscript
Vector3 get_collider_velocity(collision_index: int = 0) const
```

Returns the colliding body's velocity given a collision index (the deepest collision by default).

### get_collision_count {#get_collision_count}

```gdscript
int get_collision_count() const
```

Returns the number of detected collisions.

### get_depth {#get_depth}

```gdscript
float get_depth() const
```

Returns the colliding body's length of overlap along the collision normal.

### get_local_shape {#get_local_shape}

```gdscript
Object get_local_shape(collision_index: int = 0) const
```

Returns the moving object's colliding shape given a collision index (the deepest collision by default).

### get_normal {#get_normal}

```gdscript
Vector3 get_normal(collision_index: int = 0) const
```

Returns the colliding body's shape's normal at the point of collision given a collision index (the deepest collision by default).

### get_position {#get_position}

```gdscript
Vector3 get_position(collision_index: int = 0) const
```

Returns the point of collision in global coordinates given a collision index (the deepest collision by default).

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
