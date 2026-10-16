# KinematicCollision2D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **KinematicCollision2D**

> Holds collision data from the movement of a PhysicsBody2D, usually from PhysicsBody2D.move_and_collide(). When a PhysicsBody2D is moved, it stops if it detects a collision with another body. If a collision is detected, a KinematicCollision2D object is returned.

<details><summary>官方英文描述（中文精译进行中）</summary>

Holds collision data from the movement of a PhysicsBody2D, usually from PhysicsBody2D.move_and_collide(). When a PhysicsBody2D is moved, it stops if it detects a collision with another body. If a collision is detected, a KinematicCollision2D object is returned.

The collision data includes the colliding object, the remaining motion, and the collision position. This data can be used to determine a custom response to the collision.

</details>

## 方法

### get_angle {#get_angle}

```gdscript
float get_angle(up_direction: Vector2 = Vector2(0, -1)) const
```

Returns the collision angle according to up_direction, which is Vector2.UP by default. This value is always positive.

### get_collider {#get_collider}

```gdscript
Object get_collider() const
```

Returns the colliding body's attached Object.

### get_collider_id {#get_collider_id}

```gdscript
int get_collider_id() const
```

Returns the unique instance ID of the colliding body's attached Object. See Object.get_instance_id().

### get_collider_rid {#get_collider_rid}

```gdscript
RID get_collider_rid() const
```

Returns the colliding body's RID used by the PhysicsServer2D.

### get_collider_shape {#get_collider_shape}

```gdscript
Object get_collider_shape() const
```

Returns the colliding body's shape.

### get_collider_shape_index {#get_collider_shape_index}

```gdscript
int get_collider_shape_index() const
```

Returns the colliding body's shape index. See CollisionObject2D.

### get_collider_velocity {#get_collider_velocity}

```gdscript
Vector2 get_collider_velocity() const
```

Returns the colliding body's velocity.

### get_depth {#get_depth}

```gdscript
float get_depth() const
```

Returns the colliding body's length of overlap along the collision normal.

### get_local_shape {#get_local_shape}

```gdscript
Object get_local_shape() const
```

Returns the moving object's colliding shape.

### get_normal {#get_normal}

```gdscript
Vector2 get_normal() const
```

Returns the colliding body's shape's normal at the point of collision.

### get_position {#get_position}

```gdscript
Vector2 get_position() const
```

Returns the point of collision in global coordinates.

### get_remainder {#get_remainder}

```gdscript
Vector2 get_remainder() const
```

Returns the moving object's remaining movement vector.

### get_travel {#get_travel}

```gdscript
Vector2 get_travel() const
```

Returns the moving object's travel before collision.
