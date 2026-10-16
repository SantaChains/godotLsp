# SpringArm3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SpringArm3D**

> SpringArm3D casts a ray or a shape along its Z axis and moves all its direct children to the collision point, with an optional margin. This is useful for 3rd person cameras that move closer to the player when inside a tight space (you may need to exclude the player's collider from the SpringArm3D's collision check).

<details><summary>官方英文描述（中文精译进行中）</summary>

SpringArm3D casts a ray or a shape along its Z axis and moves all its direct children to the collision point, with an optional margin. This is useful for 3rd person cameras that move closer to the player when inside a tight space (you may need to exclude the player's collider from the SpringArm3D's collision check).

</details>

## 方法

### add_excluded_object {#add_excluded_object}

```gdscript
void add_excluded_object(RID: RID)
```

Adds the PhysicsBody3D object with the given RID to the list of PhysicsBody3D objects excluded from the collision check.

### clear_excluded_objects {#clear_excluded_objects}

```gdscript
void clear_excluded_objects()
```

Clears the list of PhysicsBody3D objects excluded from the collision check.

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hit_length {#get_hit_length}

```gdscript
float get_hit_length()
```

Returns the spring arm's current length.

### get_length {#get_length}

```gdscript
float get_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_margin {#get_margin}

```gdscript
float get_margin()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
Shape3D get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_excluded_object {#remove_excluded_object}

```gdscript
bool remove_excluded_object(RID: RID)
```

Removes the given RID from the list of PhysicsBody3D objects excluded from the collision check.

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_length {#set_length}

```gdscript
void set_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: Shape3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The layers against which the collision check will be done. See Collision layers and masks in the documentation for more information.

### shape {#shape}

```gdscript
shape: Shape3D
```

- void set_shape(value: Shape3D)
- Shape3D get_shape()

The Shape3D to use for the SpringArm3D.

When the shape is set, the SpringArm3D will cast the Shape3D on its z axis instead of performing a ray cast.

### spring_length {#spring_length}

```gdscript
spring_length: float
```

- void set_length(value: float)
- float get_length()

The maximum extent of the SpringArm3D. This is used as a length for both the ray and the shape cast used internally to calculate the desired position of the SpringArm3D's child nodes.

To know more about how to perform a shape cast or a ray cast, please consult the PhysicsDirectSpaceState3D documentation.

### margin {#margin}

```gdscript
margin: float
```

- void set_margin(value: float)
- float get_margin()

When the collision check is made, a candidate length for the SpringArm3D is given.

The margin is then subtracted to this length and the translation is applied to the child objects of the SpringArm3D.

This margin is useful for when the SpringArm3D has a Camera3D as a child node: without the margin, the Camera3D would be placed on the exact point of collision, while with the margin the Camera3D would be placed close to the point of collision.
