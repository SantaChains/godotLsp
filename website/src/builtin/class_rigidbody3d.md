# RigidBody3D

**继承**：[PhysicsBody3D](class_physicsbody3d.md) → [CollisionObject3D](class_collisionobject3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **RigidBody3D**

> RigidBody3D implements full 3D physics. It cannot be controlled directly, instead, you must apply forces to it (gravity, impulses, etc.), and the physics simulation will calculate the resulting movement, rotation, react to collisions, and affect other physics bodies in its path.

<details><summary>官方英文描述（中文精译进行中）</summary>

RigidBody3D implements full 3D physics. It cannot be controlled directly, instead, you must apply forces to it (gravity, impulses, etc.), and the physics simulation will calculate the resulting movement, rotation, react to collisions, and affect other physics bodies in its path.

The body's behavior can be adjusted via lock_rotation, freeze, and freeze_mode. By changing various properties of the object, such as mass, you can control how the physics simulation acts on it.

A rigid body will always maintain its shape and size, even when forces are applied to it. It is useful for objects that can be interacted with in an environment, such as a tree that can be knocked over or a stack of crates that can be pushed around.

If you need to directly affect the body, prefer _integrate_forces() as it allows you to directly access the physics state.

If you need to override the default physics behavior, you can write a custom force integration function. See custom_integrator.

Note: Changing the 3D transform or linear_velocity of a RigidBody3D very often may lead to some unpredictable behaviors. This also happens when a RigidBody3D is the descendant of a constantly moving node, like another RigidBody3D, as that will cause its global transform to be set whenever its ancestor moves.

</details>

## 方法

### _integrate_forces {#_integrate_forces}

```gdscript
void _integrate_forces(state: PhysicsDirectBodyState3D) virtual
```

Called during physics processing, allowing you to read and safely modify the simulation state for the object. By default, it is called before the standard force integration, but the custom_integrator property allows you to disable the standard force integration and do fully custom force integration for a body.

### add_constant_central_force {#add_constant_central_force}

```gdscript
void add_constant_central_force(force: Vector3)
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
void apply_central_force(force: Vector3)
```

Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.

This is equivalent to using apply_force() at the body's center of mass.

### apply_central_impulse {#apply_central_impulse}

```gdscript
void apply_central_impulse(impulse: Vector3)
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

Note: inertia is required for this to work. To have inertia, an active CollisionShape3D must be a child of the node, or you can manually set inertia.

### apply_torque_impulse {#apply_torque_impulse}

```gdscript
void apply_torque_impulse(impulse: Vector3)
```

Applies a rotational impulse to the body without affecting the position.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_force" functions otherwise).

Note: inertia is required for this to work. To have inertia, an active CollisionShape3D must be a child of the node, or you can manually set inertia.

### get_angular_damp {#get_angular_damp}

```gdscript
float get_angular_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_angular_damp_mode {#get_angular_damp_mode}

```gdscript
RigidBody3D.DampMode get_angular_damp_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### get_center_of_mass_mode {#get_center_of_mass_mode}

```gdscript
RigidBody3D.CenterOfMassMode get_center_of_mass_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_colliding_bodies {#get_colliding_bodies}

```gdscript
typedarray::Node3D get_colliding_bodies() const
```

Returns a list of the bodies colliding with this one. Requires contact_monitor to be set to true and max_contacts_reported to be set high enough to detect all the collisions.

Note: The result of this test is not immediate after moving objects. For performance, list of collisions is updated once per frame and before the physics step. Consider using signals instead.

### get_constant_force {#get_constant_force}

```gdscript
Vector3 get_constant_force() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_constant_torque {#get_constant_torque}

```gdscript
Vector3 get_constant_torque() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_contact_count {#get_contact_count}

```gdscript
int get_contact_count() const
```

Returns the number of contacts this body has with other bodies. By default, this returns 0 unless bodies are configured to monitor contacts (see contact_monitor).

Note: To retrieve the colliding bodies, use get_colliding_bodies().

### get_freeze_mode {#get_freeze_mode}

```gdscript
RigidBody3D.FreezeMode get_freeze_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity_scale {#get_gravity_scale}

```gdscript
float get_gravity_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inertia {#get_inertia}

```gdscript
Vector3 get_inertia() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inverse_inertia_tensor {#get_inverse_inertia_tensor}

```gdscript
Basis get_inverse_inertia_tensor() const
```

Returns the inverse inertia tensor basis. This is used to calculate the angular acceleration resulting from a torque applied to the RigidBody3D.

### get_linear_damp {#get_linear_damp}

```gdscript
float get_linear_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_damp_mode {#get_linear_damp_mode}

```gdscript
RigidBody3D.DampMode get_linear_damp_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_velocity {#get_linear_velocity}

```gdscript
Vector3 get_linear_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mass {#get_mass}

```gdscript
float get_mass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_contacts_reported {#get_max_contacts_reported}

```gdscript
int get_max_contacts_reported() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_material_override {#get_physics_material_override}

```gdscript
PhysicsMaterial get_physics_material_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_able_to_sleep {#is_able_to_sleep}

```gdscript
bool is_able_to_sleep() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_contact_monitor_enabled {#is_contact_monitor_enabled}

```gdscript
bool is_contact_monitor_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_freeze_enabled {#is_freeze_enabled}

```gdscript
bool is_freeze_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_lock_rotation_enabled {#is_lock_rotation_enabled}

```gdscript
bool is_lock_rotation_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_sleeping {#is_sleeping}

```gdscript
bool is_sleeping() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_continuous_collision_detection {#is_using_continuous_collision_detection}

```gdscript
bool is_using_continuous_collision_detection() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_custom_integrator {#is_using_custom_integrator}

```gdscript
bool is_using_custom_integrator()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_damp {#set_angular_damp}

```gdscript
void set_angular_damp(angular_damp: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_damp_mode {#set_angular_damp_mode}

```gdscript
void set_angular_damp_mode(angular_damp_mode: RigidBody3D.DampMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_velocity {#set_angular_velocity}

```gdscript
void set_angular_velocity(angular_velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_axis_velocity {#set_axis_velocity}

```gdscript
void set_axis_velocity(axis_velocity: Vector3)
```

Sets an axis velocity. The velocity in the given vector axis will be set as the given vector length. This is useful for jumping behavior.

### set_can_sleep {#set_can_sleep}

```gdscript
void set_can_sleep(able_to_sleep: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_of_mass {#set_center_of_mass}

```gdscript
void set_center_of_mass(center_of_mass: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_of_mass_mode {#set_center_of_mass_mode}

```gdscript
void set_center_of_mass_mode(mode: RigidBody3D.CenterOfMassMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constant_force {#set_constant_force}

```gdscript
void set_constant_force(force: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constant_torque {#set_constant_torque}

```gdscript
void set_constant_torque(torque: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_contact_monitor {#set_contact_monitor}

```gdscript
void set_contact_monitor(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_freeze_enabled {#set_freeze_enabled}

```gdscript
void set_freeze_enabled(freeze_mode: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_freeze_mode {#set_freeze_mode}

```gdscript
void set_freeze_mode(freeze_mode: RigidBody3D.FreezeMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_scale {#set_gravity_scale}

```gdscript
void set_gravity_scale(gravity_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inertia {#set_inertia}

```gdscript
void set_inertia(inertia: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_damp {#set_linear_damp}

```gdscript
void set_linear_damp(linear_damp: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_damp_mode {#set_linear_damp_mode}

```gdscript
void set_linear_damp_mode(linear_damp_mode: RigidBody3D.DampMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_velocity {#set_linear_velocity}

```gdscript
void set_linear_velocity(linear_velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lock_rotation_enabled {#set_lock_rotation_enabled}

```gdscript
void set_lock_rotation_enabled(lock_rotation: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mass {#set_mass}

```gdscript
void set_mass(mass: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_contacts_reported {#set_max_contacts_reported}

```gdscript
void set_max_contacts_reported(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_material_override {#set_physics_material_override}

```gdscript
void set_physics_material_override(physics_material_override: PhysicsMaterial)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sleeping {#set_sleeping}

```gdscript
void set_sleeping(sleeping: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_continuous_collision_detection {#set_use_continuous_collision_detection}

```gdscript
void set_use_continuous_collision_detection(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_custom_integrator {#set_use_custom_integrator}

```gdscript
void set_use_custom_integrator(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mass {#mass}

```gdscript
mass: float
```

- void set_mass(value: float)
- float get_mass()

The body's mass.

### physics_material_override {#physics_material_override}

```gdscript
physics_material_override: PhysicsMaterial
```

- void set_physics_material_override(value: PhysicsMaterial)
- PhysicsMaterial get_physics_material_override()

The physics material override for the body.

If a material is assigned to this property, it will be used instead of any other physics material, such as an inherited one.

### gravity_scale {#gravity_scale}

```gdscript
gravity_scale: float
```

- void set_gravity_scale(value: float)
- float get_gravity_scale()

This is multiplied by ProjectSettings.physics/3d/default_gravity to produce this body's gravity. For example, a value of 1.0 will apply normal gravity, 2.0 will apply double the gravity, and 0.5 will apply half the gravity to this body.

### center_of_mass_mode {#center_of_mass_mode}

```gdscript
center_of_mass_mode: int
```

- void set_center_of_mass_mode(value: CenterOfMassMode)
- CenterOfMassMode get_center_of_mass_mode()

Defines the way the body's center of mass is set.

### center_of_mass {#center_of_mass}

```gdscript
center_of_mass: Vector3
```

- void set_center_of_mass(value: Vector3)
- Vector3 get_center_of_mass()

The body's custom center of mass, relative to the body's origin position, when center_of_mass_mode is set to CENTER_OF_MASS_MODE_CUSTOM. This is the balanced point of the body, where applied forces only cause linear acceleration. Applying forces outside of the center of mass causes angular acceleration.

When center_of_mass_mode is set to CENTER_OF_MASS_MODE_AUTO (default value), the center of mass is automatically determined, but this does not update the value of center_of_mass.

### inertia {#inertia}

```gdscript
inertia: Vector3
```

- void set_inertia(value: Vector3)
- Vector3 get_inertia()

The body's moment of inertia. This is like mass, but for rotation: it determines how much torque it takes to rotate the body on each axis. The moment of inertia is usually computed automatically from the mass and the shapes, but this property allows you to set a custom value.

If set to Vector3.ZERO, inertia is automatically computed (default value).

Note: This value does not change when inertia is automatically computed. Use PhysicsServer3D to get the computed inertia.

```
@onready var ball = $Ball
func get_ball_inertia():
    return PhysicsServer3D.body_get_direct_state(ball.get_rid()).inverse_inertia.inverse()
```

```
private RigidBody3D _ball;
public override void _Ready()
{
    _ball = GetNode("Ball");
}
private Vector3 GetBallInertia()
{
    return PhysicsServer3D.BodyGetDirectState(_ball.GetRid()).InverseInertia.Inverse();
}
```

### sleeping {#sleeping}

```gdscript
sleeping: bool
```

- void set_sleeping(value: bool)
- bool is_sleeping()

If true, the body will not move and will not calculate forces until woken up by another body through, for example, a collision, or by using the apply_impulse() or apply_force() methods.

### can_sleep {#can_sleep}

```gdscript
can_sleep: bool
```

- void set_can_sleep(value: bool)
- bool is_able_to_sleep()

If true, the body can enter sleep mode when there is no movement. See sleeping.

### lock_rotation {#lock_rotation}

```gdscript
lock_rotation: bool
```

- void set_lock_rotation_enabled(value: bool)
- bool is_lock_rotation_enabled()

If true, the body cannot rotate. Gravity and forces only apply linear movement.

### freeze {#freeze}

```gdscript
freeze: bool
```

- void set_freeze_enabled(value: bool)
- bool is_freeze_enabled()

If true, the body is frozen. Gravity and forces are not applied anymore.

See freeze_mode to set the body's behavior when frozen.

Note: For a body that is always frozen, use StaticBody3D or AnimatableBody3D instead.

### freeze_mode {#freeze_mode}

```gdscript
freeze_mode: int
```

- void set_freeze_mode(value: FreezeMode)
- FreezeMode get_freeze_mode()

The body's freeze mode. Determines the body's behavior when freeze is true.

Note: For a body that is always frozen, use StaticBody3D or AnimatableBody3D instead.

### custom_integrator {#custom_integrator}

```gdscript
custom_integrator: bool
```

- void set_use_custom_integrator(value: bool)
- bool is_using_custom_integrator()

If true, the standard force integration (like gravity or damping) will be disabled for this body. Other than collision response, the body will only move as determined by the _integrate_forces() method, if that virtual method is overridden.

Setting this property will call the method PhysicsServer3D.body_set_omit_force_integration() internally.

### continuous_cd {#continuous_cd}

```gdscript
continuous_cd: bool
```

- void set_use_continuous_collision_detection(value: bool)
- bool is_using_continuous_collision_detection()

If true, continuous collision detection is used.

Continuous collision detection tries to predict where a moving body will collide, instead of moving it and correcting its movement if it collided. Continuous collision detection is more precise, and misses fewer impacts by small, fast-moving objects. Not using continuous collision detection is faster to compute, but can miss small, fast-moving objects.

### contact_monitor {#contact_monitor}

```gdscript
contact_monitor: bool
```

- void set_contact_monitor(value: bool)
- bool is_contact_monitor_enabled()

If true, the RigidBody3D will emit signals when it collides with another body.

Note: By default the maximum contacts reported is set to 0, meaning nothing will be recorded, see max_contacts_reported.

### max_contacts_reported {#max_contacts_reported}

```gdscript
max_contacts_reported: int
```

- void set_max_contacts_reported(value: int)
- int get_max_contacts_reported()

The maximum number of contacts that will be recorded. Requires a value greater than 0 and contact_monitor to be set to true to start to register contacts. Use get_contact_count() to retrieve the count or get_colliding_bodies() to retrieve bodies that have been collided with.

Note: The number of contacts is different from the number of collisions. Collisions between parallel edges will result in two contacts (one at each end), and collisions between parallel faces will result in four contacts (one at each corner).

### linear_velocity {#linear_velocity}

```gdscript
linear_velocity: Vector3
```

- void set_linear_velocity(value: Vector3)
- Vector3 get_linear_velocity()

The body's linear velocity in units per second. Can be used sporadically, but don't set this every frame, because physics may run in another thread and runs at a different granularity. Use _integrate_forces() as your process loop for precise control of the body state.

### linear_damp_mode {#linear_damp_mode}

```gdscript
linear_damp_mode: int
```

- void set_linear_damp_mode(value: DampMode)
- DampMode get_linear_damp_mode()

Defines how linear_damp is applied.

### linear_damp {#linear_damp}

```gdscript
linear_damp: float
```

- void set_linear_damp(value: float)
- float get_linear_damp()

Damps the body's movement. By default, the body will use the ProjectSettings.physics/3d/default_linear_damp project setting or any value override set by an Area3D the body is in. Depending on linear_damp_mode, you can set linear_damp to be added to or to replace the body's damping value.

See ProjectSettings.physics/3d/default_linear_damp for more details about damping.

### angular_velocity {#angular_velocity}

```gdscript
angular_velocity: Vector3
```

- void set_angular_velocity(value: Vector3)
- Vector3 get_angular_velocity()

The RigidBody3D's rotational velocity in radians per second.

### angular_damp_mode {#angular_damp_mode}

```gdscript
angular_damp_mode: int
```

- void set_angular_damp_mode(value: DampMode)
- DampMode get_angular_damp_mode()

Defines how angular_damp is applied.

### angular_damp {#angular_damp}

```gdscript
angular_damp: float
```

- void set_angular_damp(value: float)
- float get_angular_damp()

Damps the body's rotation. By default, the body will use the ProjectSettings.physics/3d/default_angular_damp project setting or any value override set by an Area3D the body is in. Depending on angular_damp_mode, you can set angular_damp to be added to or to replace the body's damping value.

See ProjectSettings.physics/3d/default_angular_damp for more details about damping.

### constant_force {#constant_force}

```gdscript
constant_force: Vector3
```

- void set_constant_force(value: Vector3)
- Vector3 get_constant_force()

The body's total constant positional forces applied during each physics update.

See add_constant_force() and add_constant_central_force().

### constant_torque {#constant_torque}

```gdscript
constant_torque: Vector3
```

- void set_constant_torque(value: Vector3)
- Vector3 get_constant_torque()

The body's total constant rotational forces applied during each physics update.

See add_constant_torque().

## 信号

### body_shape_entered {#body_shape_entered}

```gdscript
signal body_shape_entered(body_rid: RID, body: Node, body_shape_index: int, local_shape_index: int)
```

Emitted when one of this RigidBody3D's Shape3Ds collides with another PhysicsBody3D or GridMap's Shape3Ds. Requires contact_monitor to be set to true and max_contacts_reported to be set high enough to detect all the collisions. GridMaps are detected if the MeshLibrary has Collision Shape3Ds.

body_rid the RID of the other PhysicsBody3D or MeshLibrary's CollisionObject3D used by the PhysicsServer3D.

body the Node, if it exists in the tree, of the other PhysicsBody3D or GridMap.

body_shape_index the index of the Shape3D of the other PhysicsBody3D or GridMap used by the PhysicsServer3D. Get the CollisionShape3D node with body.shape_owner_get_owner(body.shape_find_owner(body_shape_index)).

local_shape_index the index of the Shape3D of this RigidBody3D used by the PhysicsServer3D. Get the CollisionShape3D node with self.shape_owner_get_owner(self.shape_find_owner(local_shape_index)).

### body_shape_exited {#body_shape_exited}

```gdscript
signal body_shape_exited(body_rid: RID, body: Node, body_shape_index: int, local_shape_index: int)
```

Emitted when the collision between one of this RigidBody3D's Shape3Ds and another PhysicsBody3D or GridMap's Shape3Ds ends. Requires contact_monitor to be set to true and max_contacts_reported to be set high enough to detect all the collisions. GridMaps are detected if the MeshLibrary has Collision Shape3Ds.

body_rid the RID of the other PhysicsBody3D or MeshLibrary's CollisionObject3D used by the PhysicsServer3D. GridMaps are detected if the Meshes have Shape3Ds.

body the Node, if it exists in the tree, of the other PhysicsBody3D or GridMap.

body_shape_index the index of the Shape3D of the other PhysicsBody3D or GridMap used by the PhysicsServer3D. Get the CollisionShape3D node with body.shape_owner_get_owner(body.shape_find_owner(body_shape_index)).

local_shape_index the index of the Shape3D of this RigidBody3D used by the PhysicsServer3D. Get the CollisionShape3D node with self.shape_owner_get_owner(self.shape_find_owner(local_shape_index)).

### body_entered {#body_entered}

```gdscript
signal body_entered(body: Node)
```

Emitted when a collision with another PhysicsBody3D or GridMap occurs. Requires contact_monitor to be set to true and max_contacts_reported to be set high enough to detect all the collisions. GridMaps are detected if the MeshLibrary has Collision Shape3Ds.

body the Node, if it exists in the tree, of the other PhysicsBody3D or GridMap.

### body_exited {#body_exited}

```gdscript
signal body_exited(body: Node)
```

Emitted when the collision with another PhysicsBody3D or GridMap ends. Requires contact_monitor to be set to true and max_contacts_reported to be set high enough to detect all the collisions. GridMaps are detected if the MeshLibrary has Collision Shape3Ds.

body the Node, if it exists in the tree, of the other PhysicsBody3D or GridMap.

### sleeping_state_changed {#sleeping_state_changed}

```gdscript
signal sleeping_state_changed()
```

Emitted when the physics engine changes the body's sleeping state.

Note: Changing the value sleeping will not trigger this signal. It is only emitted if the sleeping state is changed by the physics engine or emit_signal("sleeping_state_changed") is used.

## 枚举

### FreezeMode {#enum-FreezeMode}

```gdscript
enum FreezeMode { FREEZE_MODE_STATIC = 0, FREEZE_MODE_KINEMATIC = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FREEZE_MODE_STATIC {#FREEZE_MODE_STATIC}

```gdscript
FREEZE_MODE_STATIC = 0
```

Static body freeze mode (default). The body is not affected by gravity and forces. It can be only moved by user code and doesn't collide with other bodies along its path.

#### FREEZE_MODE_KINEMATIC {#FREEZE_MODE_KINEMATIC}

```gdscript
FREEZE_MODE_KINEMATIC = 1
```

Kinematic body freeze mode. Similar to FREEZE_MODE_STATIC, but collides with other bodies along its path when moved. Useful for a frozen body that needs to be animated.

---

### CenterOfMassMode {#enum-CenterOfMassMode}

```gdscript
enum CenterOfMassMode { CENTER_OF_MASS_MODE_AUTO = 0, CENTER_OF_MASS_MODE_CUSTOM = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CENTER_OF_MASS_MODE_AUTO {#CENTER_OF_MASS_MODE_AUTO}

```gdscript
CENTER_OF_MASS_MODE_AUTO = 0
```

In this mode, the body's center of mass is calculated automatically based on its shapes. This assumes that the shapes' origins are also their center of mass.

#### CENTER_OF_MASS_MODE_CUSTOM {#CENTER_OF_MASS_MODE_CUSTOM}

```gdscript
CENTER_OF_MASS_MODE_CUSTOM = 1
```

In this mode, the body's center of mass is set through center_of_mass. Defaults to the body's origin position.

---

### DampMode {#enum-DampMode}

```gdscript
enum DampMode { DAMP_MODE_COMBINE = 0, DAMP_MODE_REPLACE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DAMP_MODE_COMBINE {#DAMP_MODE_COMBINE}

```gdscript
DAMP_MODE_COMBINE = 0
```

In this mode, the body's damping value is added to any value set in areas or the default value.

#### DAMP_MODE_REPLACE {#DAMP_MODE_REPLACE}

```gdscript
DAMP_MODE_REPLACE = 1
```

In this mode, the body's damping value replaces any value set in areas or the default value.

---
