# PhysicsDirectBodyState3D

**继承**：[Object](class_object.md) → **PhysicsDirectBodyState3D**

> Provides direct access to a physics body in the PhysicsServer3D, allowing safe changes to physics properties. This object is passed via the direct state callback of RigidBody3D, and is intended for changing the direct state of that body. See RigidBody3D._integrate_forces().

<details><summary>官方英文描述</summary>

Provides direct access to a physics body in the PhysicsServer3D, allowing safe changes to physics properties. This object is passed via the direct state callback of RigidBody3D, and is intended for changing the direct state of that body. See RigidBody3D._integrate_forces().

</details>

## 方法

### add_constant_central_force {#add_constant_central_force}

```gdscript
void add_constant_central_force(force: Vector3 = Vector3(0, 0, 0))
```

Adds a constant directional force without affecting rotation that keeps being applied over time until cleared with constant_force = Vector3(0, 0, 0).

This is equivalent to using add_constant_force() at the body's center of mass.

### add_constant_force {#add_constant_force}

```gdscript
void add_constant_force(force: Vector3, position: Vector3 = Vector3(0, 0, 0))
```

Adds a constant positioned force to the body that keeps being applied over time until cleared with constant_force = Vector3(0, 0, 0).

position is the offset from the body origin in global coordinates.

### add_constant_torque {#add_constant_torque}

```gdscript
void add_constant_torque(torque: Vector3)
```

Adds a constant rotational force without affecting position that keeps being applied over time until cleared with constant_torque = Vector3(0, 0, 0).

### apply_central_force {#apply_central_force}

```gdscript
void apply_central_force(force: Vector3 = Vector3(0, 0, 0))
```

Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.

This is equivalent to using apply_force() at the body's center of mass.

### apply_central_impulse {#apply_central_impulse}

```gdscript
void apply_central_impulse(impulse: Vector3 = Vector3(0, 0, 0))
```

Applies a directional impulse without affecting rotation.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_force" functions otherwise).

This is equivalent to using apply_impulse() at the body's center of mass.

### apply_force {#apply_force}

```gdscript
void apply_force(force: Vector3, position: Vector3 = Vector3(0, 0, 0))
```

Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.

position is the offset from the body origin in global coordinates.

### apply_impulse {#apply_impulse}

```gdscript
void apply_impulse(impulse: Vector3, position: Vector3 = Vector3(0, 0, 0))
```

Applies a positioned impulse to the body.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_force" functions otherwise).

position is the offset from the body origin in global coordinates.

### apply_torque {#apply_torque}

```gdscript
void apply_torque(torque: Vector3)
```

Applies a rotational force without affecting position. A force is time dependent and meant to be applied every physics update.

Note: inverse_inertia is required for this to work. To have inverse_inertia, an active CollisionShape3D must be a child of the node, or you can manually set inverse_inertia.

### apply_torque_impulse {#apply_torque_impulse}

```gdscript
void apply_torque_impulse(impulse: Vector3)
```

Applies a rotational impulse to the body without affecting the position.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_force" functions otherwise).

Note: inverse_inertia is required for this to work. To have inverse_inertia, an active CollisionShape3D must be a child of the node, or you can manually set inverse_inertia.

### get_angular_velocity {#get_angular_velocity}

```gdscript
Vector3 get_angular_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_of_mass {#get_center_of_mass}

```gdscript
Vector3 get_center_of_mass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_of_mass_local {#get_center_of_mass_local}

```gdscript
Vector3 get_center_of_mass_local() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_layer {#get_collision_layer}

```gdscript
int get_collision_layer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_constant_force {#get_constant_force}

```gdscript
Vector3 get_constant_force() const
```

Returns the body's total constant positional forces applied during each physics update.

See add_constant_force() and add_constant_central_force().

### get_constant_torque {#get_constant_torque}

```gdscript
Vector3 get_constant_torque() const
```

Returns the body's total constant rotational forces applied during each physics update.

See add_constant_torque().

### get_contact_collider {#get_contact_collider}

```gdscript
RID get_contact_collider(contact_idx: int) const
```

Returns the collider's RID.

### get_contact_collider_id {#get_contact_collider_id}

```gdscript
int get_contact_collider_id(contact_idx: int) const
```

Returns the collider's object id.

### get_contact_collider_object {#get_contact_collider_object}

```gdscript
Object get_contact_collider_object(contact_idx: int) const
```

Returns the collider object.

### get_contact_collider_position {#get_contact_collider_position}

```gdscript
Vector3 get_contact_collider_position(contact_idx: int) const
```

Returns the position of the contact point on the collider in the global coordinate system.

### get_contact_collider_shape {#get_contact_collider_shape}

```gdscript
int get_contact_collider_shape(contact_idx: int) const
```

Returns the collider's shape index.

### get_contact_collider_velocity_at_position {#get_contact_collider_velocity_at_position}

```gdscript
Vector3 get_contact_collider_velocity_at_position(contact_idx: int) const
```

Returns the linear velocity vector at the collider's contact point.

### get_contact_count {#get_contact_count}

```gdscript
int get_contact_count() const
```

Returns the number of contacts this body has with other bodies.

Note: By default, this returns 0 unless bodies are configured to monitor contacts. See RigidBody3D.contact_monitor.

### get_contact_impulse {#get_contact_impulse}

```gdscript
Vector3 get_contact_impulse(contact_idx: int) const
```

Impulse created by the contact.

### get_contact_local_normal {#get_contact_local_normal}

```gdscript
Vector3 get_contact_local_normal(contact_idx: int) const
```

Returns the local normal at the contact point.

### get_contact_local_position {#get_contact_local_position}

```gdscript
Vector3 get_contact_local_position(contact_idx: int) const
```

Returns the position of the contact point on the body in the global coordinate system.

### get_contact_local_shape {#get_contact_local_shape}

```gdscript
int get_contact_local_shape(contact_idx: int) const
```

Returns the local shape index of the collision.

### get_contact_local_velocity_at_position {#get_contact_local_velocity_at_position}

```gdscript
Vector3 get_contact_local_velocity_at_position(contact_idx: int) const
```

Returns the linear velocity vector at the body's contact point.

### get_inverse_inertia {#get_inverse_inertia}

```gdscript
Vector3 get_inverse_inertia() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inverse_inertia_tensor {#get_inverse_inertia_tensor}

```gdscript
Basis get_inverse_inertia_tensor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inverse_mass {#get_inverse_mass}

```gdscript
float get_inverse_mass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_velocity {#get_linear_velocity}

```gdscript
Vector3 get_linear_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_principal_inertia_axes {#get_principal_inertia_axes}

```gdscript
Basis get_principal_inertia_axes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_space_state {#get_space_state}

```gdscript
PhysicsDirectSpaceState3D get_space_state()
```

Returns the current state of the space, useful for queries.

### get_step {#get_step}

```gdscript
float get_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_total_angular_damp {#get_total_angular_damp}

```gdscript
float get_total_angular_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_total_gravity {#get_total_gravity}

```gdscript
Vector3 get_total_gravity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_total_linear_damp {#get_total_linear_damp}

```gdscript
float get_total_linear_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform {#get_transform}

```gdscript
Transform3D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_velocity_at_local_position {#get_velocity_at_local_position}

```gdscript
Vector3 get_velocity_at_local_position(local_position: Vector3) const
```

Returns the body's velocity at the given relative position.

local_position is the offset from the body origin in global coordinates.

### integrate_forces {#integrate_forces}

```gdscript
void integrate_forces()
```

Updates the body's linear and angular velocity by applying gravity and damping for the equivalent of one physics tick.

### is_sleeping {#is_sleeping}

```gdscript
bool is_sleeping() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_velocity {#set_angular_velocity}

```gdscript
void set_angular_velocity(velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_layer {#set_collision_layer}

```gdscript
void set_collision_layer(layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constant_force {#set_constant_force}

```gdscript
void set_constant_force(force: Vector3)
```

Sets the body's total constant positional forces applied during each physics update.

See add_constant_force() and add_constant_central_force().

### set_constant_torque {#set_constant_torque}

```gdscript
void set_constant_torque(torque: Vector3)
```

Sets the body's total constant rotational forces applied during each physics update.

See add_constant_torque().

### set_linear_velocity {#set_linear_velocity}

```gdscript
void set_linear_velocity(velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sleep_state {#set_sleep_state}

```gdscript
void set_sleep_state(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform {#set_transform}

```gdscript
void set_transform(transform: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### step {#step}

```gdscript
step: float
```

- float get_step()

The timestep (delta) used for the simulation.

### inverse_mass {#inverse_mass}

```gdscript
inverse_mass: float
```

- float get_inverse_mass()

The inverse of the mass of the body.

### total_angular_damp {#total_angular_damp}

```gdscript
total_angular_damp: float
```

- float get_total_angular_damp()

The rate at which the body stops rotating, if there are not any other forces moving it.

### total_linear_damp {#total_linear_damp}

```gdscript
total_linear_damp: float
```

- float get_total_linear_damp()

The rate at which the body stops moving, if there are not any other forces moving it.

### inverse_inertia {#inverse_inertia}

```gdscript
inverse_inertia: Vector3
```

- Vector3 get_inverse_inertia()

The inverse of the inertia of the body.

### inverse_inertia_tensor {#inverse_inertia_tensor}

```gdscript
inverse_inertia_tensor: Basis
```

- Basis get_inverse_inertia_tensor()

The inverse of the inertia tensor of the body.

### total_gravity {#total_gravity}

```gdscript
total_gravity: Vector3
```

- Vector3 get_total_gravity()

The total gravity vector being currently applied to this body.

### center_of_mass {#center_of_mass}

```gdscript
center_of_mass: Vector3
```

- Vector3 get_center_of_mass()

The body's center of mass position relative to the body's center in the global coordinate system.

### center_of_mass_local {#center_of_mass_local}

```gdscript
center_of_mass_local: Vector3
```

- Vector3 get_center_of_mass_local()

The body's center of mass position in the body's local coordinate system.

### principal_inertia_axes {#principal_inertia_axes}

```gdscript
principal_inertia_axes: Basis
```

- Basis get_principal_inertia_axes()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### angular_velocity {#angular_velocity}

```gdscript
angular_velocity: Vector3
```

- void set_angular_velocity(value: Vector3)
- Vector3 get_angular_velocity()

The body's rotational velocity in radians per second.

### linear_velocity {#linear_velocity}

```gdscript
linear_velocity: Vector3
```

- void set_linear_velocity(value: Vector3)
- Vector3 get_linear_velocity()

The body's linear velocity in units per second.

### sleeping {#sleeping}

```gdscript
sleeping: bool
```

- void set_sleep_state(value: bool)
- bool is_sleeping()

If true, this body is currently sleeping (not active).

### collision_layer {#collision_layer}

```gdscript
collision_layer: int
```

- void set_collision_layer(value: int)
- int get_collision_layer()

The body's collision layer.

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The body's collision mask.

### transform {#transform}

```gdscript
transform: Transform3D
```

- void set_transform(value: Transform3D)
- Transform3D get_transform()

The body's transformation matrix.
