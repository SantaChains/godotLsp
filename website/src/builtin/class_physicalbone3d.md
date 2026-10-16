# PhysicalBone3D

**继承**：[PhysicsBody3D](class_physicsbody3d.md) → [CollisionObject3D](class_collisionobject3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **PhysicalBone3D**

> The PhysicalBone3D node is a physics body that can be used to make bones in a Skeleton3D react to physics.

<details><summary>官方英文描述（中文精译进行中）</summary>

The PhysicalBone3D node is a physics body that can be used to make bones in a Skeleton3D react to physics.

Note: In order to detect physical bones with raycasts, the SkeletonModifier3D.active property of the parent PhysicalBoneSimulator3D must be true and the Skeleton3D's bone must be assigned to PhysicalBone3D correctly; it means that get_bone_id() should return a valid id (>= 0).

</details>

## 方法

### _integrate_forces {#_integrate_forces}

```gdscript
void _integrate_forces(state: PhysicsDirectBodyState3D) virtual
```

Called during physics processing, allowing you to read and safely modify the simulation state for the object. By default, it is called before the standard force integration, but the custom_integrator property allows you to disable the standard force integration and do fully custom force integration for a body.

### apply_central_impulse {#apply_central_impulse}

```gdscript
void apply_central_impulse(impulse: Vector3)
```

Applies a directional impulse without affecting rotation.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_integrate_forces" functions otherwise).

This is equivalent to using apply_impulse() at the body's center of mass.

### apply_impulse {#apply_impulse}

```gdscript
void apply_impulse(impulse: Vector3, position: Vector3 = Vector3(0, 0, 0))
```

Applies a positioned impulse to the PhysicsBone3D.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_integrate_forces" functions otherwise).

position is the offset from the PhysicsBone3D origin in global coordinates.

### get_angular_damp {#get_angular_damp}

```gdscript
float get_angular_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_angular_damp_mode {#get_angular_damp_mode}

```gdscript
PhysicalBone3D.DampMode get_angular_damp_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_angular_velocity {#get_angular_velocity}

```gdscript
Vector3 get_angular_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_body_offset {#get_body_offset}

```gdscript
Transform3D get_body_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_id {#get_bone_id}

```gdscript
int get_bone_id() const
```

Returns the unique identifier of the PhysicsBone3D.

### get_bounce {#get_bounce}

```gdscript
float get_bounce() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_friction {#get_friction}

```gdscript
float get_friction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity_scale {#get_gravity_scale}

```gdscript
float get_gravity_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_offset {#get_joint_offset}

```gdscript
Transform3D get_joint_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_rotation {#get_joint_rotation}

```gdscript
Vector3 get_joint_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_type {#get_joint_type}

```gdscript
PhysicalBone3D.JointType get_joint_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_damp {#get_linear_damp}

```gdscript
float get_linear_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_damp_mode {#get_linear_damp_mode}

```gdscript
PhysicalBone3D.DampMode get_linear_damp_mode() const
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

### get_simulate_physics {#get_simulate_physics}

```gdscript
bool get_simulate_physics()
```

Returns true if the PhysicsBone3D is allowed to simulate physics.

### is_able_to_sleep {#is_able_to_sleep}

```gdscript
bool is_able_to_sleep() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_simulating_physics {#is_simulating_physics}

```gdscript
bool is_simulating_physics()
```

Returns true if the PhysicsBone3D is currently simulating physics.

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
void set_angular_damp_mode(angular_damp_mode: PhysicalBone3D.DampMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_velocity {#set_angular_velocity}

```gdscript
void set_angular_velocity(angular_velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_body_offset {#set_body_offset}

```gdscript
void set_body_offset(offset: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bounce {#set_bounce}

```gdscript
void set_bounce(bounce: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_can_sleep {#set_can_sleep}

```gdscript
void set_can_sleep(able_to_sleep: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_friction {#set_friction}

```gdscript
void set_friction(friction: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_scale {#set_gravity_scale}

```gdscript
void set_gravity_scale(gravity_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_offset {#set_joint_offset}

```gdscript
void set_joint_offset(offset: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_rotation {#set_joint_rotation}

```gdscript
void set_joint_rotation(euler: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_type {#set_joint_type}

```gdscript
void set_joint_type(joint_type: PhysicalBone3D.JointType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_damp {#set_linear_damp}

```gdscript
void set_linear_damp(linear_damp: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_damp_mode {#set_linear_damp_mode}

```gdscript
void set_linear_damp_mode(linear_damp_mode: PhysicalBone3D.DampMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_velocity {#set_linear_velocity}

```gdscript
void set_linear_velocity(linear_velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mass {#set_mass}

```gdscript
void set_mass(mass: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_custom_integrator {#set_use_custom_integrator}

```gdscript
void set_use_custom_integrator(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### joint_type {#joint_type}

```gdscript
joint_type: int
```

- void set_joint_type(value: JointType)
- JointType get_joint_type()

Sets the joint type.

### joint_offset {#joint_offset}

```gdscript
joint_offset: Transform3D
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### joint_rotation {#joint_rotation}

```gdscript
joint_rotation: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### body_offset {#body_offset}

```gdscript
body_offset: Transform3D
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### mass {#mass}

```gdscript
mass: float
```

- void set_mass(value: float)
- float get_mass()

The body's mass.

### friction {#friction}

```gdscript
friction: float
```

- void set_friction(value: float)
- float get_friction()

The body's friction, from 0 (frictionless) to 1 (max friction).

### bounce {#bounce}

```gdscript
bounce: float
```

- void set_bounce(value: float)
- float get_bounce()

The body's bounciness. Values range from 0 (no bounce) to 1 (full bounciness).

Note: Even with bounce set to 1.0, some energy will be lost over time due to linear and angular damping. To have a PhysicalBone3D that preserves all its energy over time, set bounce to 1.0, linear_damp_mode to DAMP_MODE_REPLACE, linear_damp to 0.0, angular_damp_mode to DAMP_MODE_REPLACE, and angular_damp to 0.0.

### gravity_scale {#gravity_scale}

```gdscript
gravity_scale: float
```

- void set_gravity_scale(value: float)
- float get_gravity_scale()

This is multiplied by ProjectSettings.physics/3d/default_gravity to produce this body's gravity. For example, a value of 1.0 will apply normal gravity, 2.0 will apply double the gravity, and 0.5 will apply half the gravity to this body.

### custom_integrator {#custom_integrator}

```gdscript
custom_integrator: bool
```

- void set_use_custom_integrator(value: bool)
- bool is_using_custom_integrator()

If true, the standard force integration (like gravity or damping) will be disabled for this body. Other than collision response, the body will only move as determined by the _integrate_forces() method, if that virtual method is overridden.

Setting this property will call the method PhysicsServer3D.body_set_omit_force_integration() internally.

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

Damps the body's movement. By default, the body will use ProjectSettings.physics/3d/default_linear_damp or any value override set by an Area3D the body is in. Depending on linear_damp_mode, linear_damp may be added to or replace the body's damping value.

See ProjectSettings.physics/3d/default_linear_damp for more details about damping.

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

### linear_velocity {#linear_velocity}

```gdscript
linear_velocity: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### angular_velocity {#angular_velocity}

```gdscript
angular_velocity: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### can_sleep {#can_sleep}

```gdscript
can_sleep: bool
```

- void set_can_sleep(value: bool)
- bool is_able_to_sleep()

If true, the body is deactivated when there is no movement, so it will not take part in the simulation until it is awakened by an external force.

## 枚举

### DampMode {#enum-DampMode}

```gdscript
enum DampMode { DAMP_MODE_COMBINE = 0, DAMP_MODE_REPLACE = 1 }
```

DampMode DAMP_MODE_COMBINE = 0

In this mode, the body's damping value is added to any value set in areas or the default value.

DampMode DAMP_MODE_REPLACE = 1

In this mode, the body's damping value replaces any value set in areas or the default value.

#### DAMP_MODE_COMBINE {#DAMP_MODE_COMBINE}

```gdscript
DAMP_MODE_COMBINE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DAMP_MODE_REPLACE {#DAMP_MODE_REPLACE}

```gdscript
DAMP_MODE_REPLACE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### JointType {#enum-JointType}

```gdscript
enum JointType { JOINT_TYPE_NONE = 0, JOINT_TYPE_PIN = 1, JOINT_TYPE_CONE = 2, JOINT_TYPE_HINGE = 3, JOINT_TYPE_SLIDER = 4, JOINT_TYPE_6DOF = 5 }
```

JointType JOINT_TYPE_NONE = 0

No joint is applied to the PhysicsBone3D.

JointType JOINT_TYPE_PIN = 1

A pin joint is applied to the PhysicsBone3D.

JointType JOINT_TYPE_CONE = 2

A cone joint is applied to the PhysicsBone3D.

JointType JOINT_TYPE_HINGE = 3

A hinge joint is applied to the PhysicsBone3D.

JointType JOINT_TYPE_SLIDER = 4

A slider joint is applied to the PhysicsBone3D.

JointType JOINT_TYPE_6DOF = 5

A 6 degrees of freedom joint is applied to the PhysicsBone3D.

#### JOINT_TYPE_NONE {#JOINT_TYPE_NONE}

```gdscript
JOINT_TYPE_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_TYPE_PIN {#JOINT_TYPE_PIN}

```gdscript
JOINT_TYPE_PIN = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_TYPE_CONE {#JOINT_TYPE_CONE}

```gdscript
JOINT_TYPE_CONE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_TYPE_HINGE {#JOINT_TYPE_HINGE}

```gdscript
JOINT_TYPE_HINGE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_TYPE_SLIDER {#JOINT_TYPE_SLIDER}

```gdscript
JOINT_TYPE_SLIDER = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_TYPE_6DOF {#JOINT_TYPE_6DOF}

```gdscript
JOINT_TYPE_6DOF = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
