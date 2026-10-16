# StaticBody3D

**继承**：[PhysicsBody3D](class_physicsbody3d.md) → [CollisionObject3D](class_collisionobject3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **StaticBody3D**

> A static 3D physics body. It can't be moved by external forces or contacts, but can be moved manually by other means such as code, AnimationMixers (with AnimationMixer.callback_mode_process set to AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS), and RemoteTransform3D.

<details><summary>官方英文描述</summary>

A static 3D physics body. It can't be moved by external forces or contacts, but can be moved manually by other means such as code, AnimationMixers (with AnimationMixer.callback_mode_process set to AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS), and RemoteTransform3D.

When StaticBody3D is moved, it is teleported to its new position without affecting other physics bodies in its path. If this is not desired, use AnimatableBody3D instead.

StaticBody3D is useful for completely static objects like floors and walls, as well as moving surfaces like conveyor belts and circular revolving platforms (by using constant_linear_velocity and constant_angular_velocity).

</details>

## 方法

### get_constant_angular_velocity {#get_constant_angular_velocity}

```gdscript
Vector3 get_constant_angular_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_constant_linear_velocity {#get_constant_linear_velocity}

```gdscript
Vector3 get_constant_linear_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_material_override {#get_physics_material_override}

```gdscript
PhysicsMaterial get_physics_material_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constant_angular_velocity {#set_constant_angular_velocity}

```gdscript
void set_constant_angular_velocity(vel: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constant_linear_velocity {#set_constant_linear_velocity}

```gdscript
void set_constant_linear_velocity(vel: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_material_override {#set_physics_material_override}

```gdscript
void set_physics_material_override(physics_material_override: PhysicsMaterial)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### physics_material_override {#physics_material_override}

```gdscript
physics_material_override: PhysicsMaterial
```

- void set_physics_material_override(value: PhysicsMaterial)
- PhysicsMaterial get_physics_material_override()

The physics material override for the body.

If a material is assigned to this property, it will be used instead of any other physics material, such as an inherited one.

### constant_linear_velocity {#constant_linear_velocity}

```gdscript
constant_linear_velocity: Vector3
```

- void set_constant_linear_velocity(value: Vector3)
- Vector3 get_constant_linear_velocity()

The body's constant linear velocity. This does not move the body, but affects touching bodies, as if it were moving.

### constant_angular_velocity {#constant_angular_velocity}

```gdscript
constant_angular_velocity: Vector3
```

- void set_constant_angular_velocity(value: Vector3)
- Vector3 get_constant_angular_velocity()

The body's constant angular velocity. This does not rotate the body, but affects touching bodies, as if it were rotating.
