# PhysicsDirectBodyState2DExtension

**继承**：[PhysicsDirectBodyState2D](class_physicsdirectbodystate2d.md) → [Object](class_object.md) → **PhysicsDirectBodyState2DExtension**

> This class extends PhysicsDirectBodyState2D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

<details><summary>官方英文描述</summary>

This class extends PhysicsDirectBodyState2D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

Intended for use with GDExtension to create custom implementations of PhysicsDirectBodyState2D.

</details>

## 方法

### _add_constant_central_force {#_add_constant_central_force}

```gdscript
void _add_constant_central_force(force: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.add_constant_central_force().

### _add_constant_force {#_add_constant_force}

```gdscript
void _add_constant_force(force: Vector2, position: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.add_constant_force().

### _add_constant_torque {#_add_constant_torque}

```gdscript
void _add_constant_torque(torque: float) virtual
```

Overridable version of PhysicsDirectBodyState2D.add_constant_torque().

### _apply_central_force {#_apply_central_force}

```gdscript
void _apply_central_force(force: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.apply_central_force().

### _apply_central_impulse {#_apply_central_impulse}

```gdscript
void _apply_central_impulse(impulse: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.apply_central_impulse().

### _apply_force {#_apply_force}

```gdscript
void _apply_force(force: Vector2, position: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.apply_force().

### _apply_impulse {#_apply_impulse}

```gdscript
void _apply_impulse(impulse: Vector2, position: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.apply_impulse().

### _apply_torque {#_apply_torque}

```gdscript
void _apply_torque(torque: float) virtual
```

Overridable version of PhysicsDirectBodyState2D.apply_torque().

### _apply_torque_impulse {#_apply_torque_impulse}

```gdscript
void _apply_torque_impulse(impulse: float) virtual
```

Overridable version of PhysicsDirectBodyState2D.apply_torque_impulse().

### _get_angular_velocity {#_get_angular_velocity}

```gdscript
float _get_angular_velocity() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.angular_velocity and its respective getter.

### _get_center_of_mass {#_get_center_of_mass}

```gdscript
Vector2 _get_center_of_mass() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.center_of_mass and its respective getter.

### _get_center_of_mass_local {#_get_center_of_mass_local}

```gdscript
Vector2 _get_center_of_mass_local() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.center_of_mass_local and its respective getter.

### _get_collision_layer {#_get_collision_layer}

```gdscript
int _get_collision_layer() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_collision_mask {#_get_collision_mask}

```gdscript
int _get_collision_mask() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_constant_force {#_get_constant_force}

```gdscript
Vector2 _get_constant_force() virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_constant_force().

### _get_constant_torque {#_get_constant_torque}

```gdscript
float _get_constant_torque() virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_constant_torque().

### _get_contact_collider {#_get_contact_collider}

```gdscript
RID _get_contact_collider(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_collider().

### _get_contact_collider_id {#_get_contact_collider_id}

```gdscript
int _get_contact_collider_id(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_collider_id().

### _get_contact_collider_object {#_get_contact_collider_object}

```gdscript
Object _get_contact_collider_object(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_collider_object().

### _get_contact_collider_position {#_get_contact_collider_position}

```gdscript
Vector2 _get_contact_collider_position(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_collider_position().

### _get_contact_collider_shape {#_get_contact_collider_shape}

```gdscript
int _get_contact_collider_shape(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_collider_shape().

### _get_contact_collider_velocity_at_position {#_get_contact_collider_velocity_at_position}

```gdscript
Vector2 _get_contact_collider_velocity_at_position(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_collider_velocity_at_position().

### _get_contact_count {#_get_contact_count}

```gdscript
int _get_contact_count() virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_count().

### _get_contact_impulse {#_get_contact_impulse}

```gdscript
Vector2 _get_contact_impulse(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_impulse().

### _get_contact_local_normal {#_get_contact_local_normal}

```gdscript
Vector2 _get_contact_local_normal(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_local_normal().

### _get_contact_local_position {#_get_contact_local_position}

```gdscript
Vector2 _get_contact_local_position(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_local_position().

### _get_contact_local_shape {#_get_contact_local_shape}

```gdscript
int _get_contact_local_shape(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_local_shape().

### _get_contact_local_velocity_at_position {#_get_contact_local_velocity_at_position}

```gdscript
Vector2 _get_contact_local_velocity_at_position(contact_idx: int) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_contact_local_velocity_at_position().

### _get_inverse_inertia {#_get_inverse_inertia}

```gdscript
float _get_inverse_inertia() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.inverse_inertia and its respective getter.

### _get_inverse_mass {#_get_inverse_mass}

```gdscript
float _get_inverse_mass() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.inverse_mass and its respective getter.

### _get_linear_velocity {#_get_linear_velocity}

```gdscript
Vector2 _get_linear_velocity() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.linear_velocity and its respective getter.

### _get_space_state {#_get_space_state}

```gdscript
PhysicsDirectSpaceState2D _get_space_state() virtual
```

Overridable version of PhysicsDirectBodyState2D.get_space_state().

### _get_step {#_get_step}

```gdscript
float _get_step() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.step and its respective getter.

### _get_total_angular_damp {#_get_total_angular_damp}

```gdscript
float _get_total_angular_damp() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.total_angular_damp and its respective getter.

### _get_total_gravity {#_get_total_gravity}

```gdscript
Vector2 _get_total_gravity() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.total_gravity and its respective getter.

### _get_total_linear_damp {#_get_total_linear_damp}

```gdscript
float _get_total_linear_damp() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.total_linear_damp and its respective getter.

### _get_transform {#_get_transform}

```gdscript
Transform2D _get_transform() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.transform and its respective getter.

### _get_velocity_at_local_position {#_get_velocity_at_local_position}

```gdscript
Vector2 _get_velocity_at_local_position(local_position: Vector2) virtual const
```

Overridable version of PhysicsDirectBodyState2D.get_velocity_at_local_position().

### _integrate_forces {#_integrate_forces}

```gdscript
void _integrate_forces() virtual
```

Overridable version of PhysicsDirectBodyState2D.integrate_forces().

### _is_sleeping {#_is_sleeping}

```gdscript
bool _is_sleeping() virtual const
```

Implement to override the behavior of PhysicsDirectBodyState2D.sleeping and its respective getter.

### _set_angular_velocity {#_set_angular_velocity}

```gdscript
void _set_angular_velocity(velocity: float) virtual
```

Implement to override the behavior of PhysicsDirectBodyState2D.angular_velocity and its respective setter.

### _set_collision_layer {#_set_collision_layer}

```gdscript
void _set_collision_layer(layer: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _set_collision_mask {#_set_collision_mask}

```gdscript
void _set_collision_mask(mask: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _set_constant_force {#_set_constant_force}

```gdscript
void _set_constant_force(force: Vector2) virtual
```

Overridable version of PhysicsDirectBodyState2D.set_constant_force().

### _set_constant_torque {#_set_constant_torque}

```gdscript
void _set_constant_torque(torque: float) virtual
```

Overridable version of PhysicsDirectBodyState2D.set_constant_torque().

### _set_linear_velocity {#_set_linear_velocity}

```gdscript
void _set_linear_velocity(velocity: Vector2) virtual
```

Implement to override the behavior of PhysicsDirectBodyState2D.linear_velocity and its respective setter.

### _set_sleep_state {#_set_sleep_state}

```gdscript
void _set_sleep_state(enabled: bool) virtual
```

Implement to override the behavior of PhysicsDirectBodyState2D.sleeping and its respective setter.

### _set_transform {#_set_transform}

```gdscript
void _set_transform(transform: Transform2D) virtual
```

Implement to override the behavior of PhysicsDirectBodyState2D.transform and its respective setter.
