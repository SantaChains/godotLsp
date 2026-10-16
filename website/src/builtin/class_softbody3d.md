# SoftBody3D

**继承**：[MeshInstance3D](class_meshinstance3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SoftBody3D**

> A deformable 3D physics mesh. Used to create elastic or deformable objects such as cloth, rubber, or other flexible materials.

<details><summary>官方英文描述（中文精译进行中）</summary>

A deformable 3D physics mesh. Used to create elastic or deformable objects such as cloth, rubber, or other flexible materials.

Additionally, SoftBody3D is subject to wind forces defined in Area3D (see Area3D.wind_source_path, Area3D.wind_force_magnitude, and Area3D.wind_attenuation_factor).

Note: It's recommended to use Jolt Physics when using SoftBody3D instead of the default GodotPhysics3D, as Jolt Physics' soft body implementation is faster and more reliable. You can switch the physics engine using the ProjectSettings.physics/3d/physics_engine project setting.

</details>

## 方法

### add_collision_exception_with {#add_collision_exception_with}

```gdscript
void add_collision_exception_with(body: Node)
```

Adds a body to the list of bodies that this body can't collide with.

### apply_central_force {#apply_central_force}

```gdscript
void apply_central_force(force: Vector3)
```

Distributes and applies a force to all points. A force is time dependent and meant to be applied every physics update.

### apply_central_impulse {#apply_central_impulse}

```gdscript
void apply_central_impulse(impulse: Vector3)
```

Distributes and applies an impulse to all points.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_force" functions otherwise).

### apply_force {#apply_force}

```gdscript
void apply_force(point_index: int, force: Vector3)
```

Applies a force to a point. A force is time dependent and meant to be applied every physics update.

### apply_impulse {#apply_impulse}

```gdscript
void apply_impulse(point_index: int, impulse: Vector3)
```

Applies an impulse to a point.

An impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the "_force" functions otherwise).

### get_collision_exceptions {#get_collision_exceptions}

```gdscript
typedarray::PhysicsBody3D get_collision_exceptions()
```

Returns an array of nodes that were added as collision exceptions for this body.

### get_collision_layer {#get_collision_layer}

```gdscript
int get_collision_layer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_layer_value {#get_collision_layer_value}

```gdscript
bool get_collision_layer_value(layer_number: int) const
```

Returns whether or not the specified layer of the collision_layer is enabled, given a layer_number between 1 and 32.

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_mask_value {#get_collision_mask_value}

```gdscript
bool get_collision_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the collision_mask is enabled, given a layer_number between 1 and 32.

### get_damping_coefficient {#get_damping_coefficient}

```gdscript
float get_damping_coefficient()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_disable_mode {#get_disable_mode}

```gdscript
SoftBody3D.DisableMode get_disable_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_coefficient {#get_drag_coefficient}

```gdscript
float get_drag_coefficient()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_stiffness {#get_linear_stiffness}

```gdscript
float get_linear_stiffness()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parent_collision_ignore {#get_parent_collision_ignore}

```gdscript
NodePath get_parent_collision_ignore() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_rid {#get_physics_rid}

```gdscript
RID get_physics_rid() const
```

Returns the internal RID used by the PhysicsServer3D for this body.

### get_point_transform {#get_point_transform}

```gdscript
Vector3 get_point_transform(point_index: int)
```

Returns local translation of a vertex in the surface array.

### get_pressure_coefficient {#get_pressure_coefficient}

```gdscript
float get_pressure_coefficient()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shrinking_factor {#get_shrinking_factor}

```gdscript
float get_shrinking_factor()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_simulation_precision {#get_simulation_precision}

```gdscript
int get_simulation_precision()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_total_mass {#get_total_mass}

```gdscript
float get_total_mass()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_point_pinned {#is_point_pinned}

```gdscript
bool is_point_pinned(point_index: int) const
```

Returns true if vertex is set to pinned.

### is_ray_pickable {#is_ray_pickable}

```gdscript
bool is_ray_pickable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_collision_exception_with {#remove_collision_exception_with}

```gdscript
void remove_collision_exception_with(body: Node)
```

Removes a body from the list of bodies that this body can't collide with.

### set_collision_layer {#set_collision_layer}

```gdscript
void set_collision_layer(collision_layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_layer_value {#set_collision_layer_value}

```gdscript
void set_collision_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_layer, given a layer_number between 1 and 32.

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(collision_mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask_value {#set_collision_mask_value}

```gdscript
void set_collision_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_mask, given a layer_number between 1 and 32.

### set_damping_coefficient {#set_damping_coefficient}

```gdscript
void set_damping_coefficient(damping_coefficient: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_mode {#set_disable_mode}

```gdscript
void set_disable_mode(mode: SoftBody3D.DisableMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_coefficient {#set_drag_coefficient}

```gdscript
void set_drag_coefficient(drag_coefficient: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_stiffness {#set_linear_stiffness}

```gdscript
void set_linear_stiffness(linear_stiffness: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_parent_collision_ignore {#set_parent_collision_ignore}

```gdscript
void set_parent_collision_ignore(parent_collision_ignore: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_pinned {#set_point_pinned}

```gdscript
void set_point_pinned(point_index: int, pinned: bool, attachment_path: NodePath = NodePath(""), insert_at: int = -1)
```

Sets the pinned state of a surface vertex. When set to true, the optional attachment_path can define a Node3D the pinned vertex will be attached to.

### set_pressure_coefficient {#set_pressure_coefficient}

```gdscript
void set_pressure_coefficient(pressure_coefficient: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ray_pickable {#set_ray_pickable}

```gdscript
void set_ray_pickable(ray_pickable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shrinking_factor {#set_shrinking_factor}

```gdscript
void set_shrinking_factor(shrinking_factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_simulation_precision {#set_simulation_precision}

```gdscript
void set_simulation_precision(simulation_precision: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_total_mass {#set_total_mass}

```gdscript
void set_total_mass(mass: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### collision_layer {#collision_layer}

```gdscript
collision_layer: int
```

- void set_collision_layer(value: int)
- int get_collision_layer()

The physics layers this SoftBody3D is in. Collision objects can exist in one or more of 32 different layers. See also collision_mask.

Note: Object A can detect a contact with object B only if object B is in any of the layers that object A scans. See Collision layers and masks in the documentation for more information.

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The physics layers this SoftBody3D scans. Collision objects can scan one or more of 32 different layers. See also collision_layer.

Note: Object A can detect a contact with object B only if object B is in any of the layers that object A scans. See Collision layers and masks in the documentation for more information.

### parent_collision_ignore {#parent_collision_ignore}

```gdscript
parent_collision_ignore: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### simulation_precision {#simulation_precision}

```gdscript
simulation_precision: int
```

- void set_simulation_precision(value: int)
- int get_simulation_precision()

Increasing this value will improve the resulting simulation, but can affect performance. Use with care.

### total_mass {#total_mass}

```gdscript
total_mass: float
```

- void set_total_mass(value: float)
- float get_total_mass()

The SoftBody3D's mass.

### linear_stiffness {#linear_stiffness}

```gdscript
linear_stiffness: float
```

- void set_linear_stiffness(value: float)
- float get_linear_stiffness()

Higher values will result in a stiffer body, while lower values will increase the body's ability to bend. The value can be between 0.0 and 1.0 (inclusive).

### shrinking_factor {#shrinking_factor}

```gdscript
shrinking_factor: float
```

- void set_shrinking_factor(value: float)
- float get_shrinking_factor()

Scales the rest lengths of SoftBody3D's edge constraints. Positive values shrink the mesh, while negative values expand it. For example, a value of 0.1 shortens the edges of the mesh by 10%, while -0.1 expands the edges by 10%.

Note: shrinking_factor is best used on surface meshes with pinned points.

### pressure_coefficient {#pressure_coefficient}

```gdscript
pressure_coefficient: float
```

- void set_pressure_coefficient(value: float)
- float get_pressure_coefficient()

The pressure coefficient of this soft body. Simulate pressure build-up from inside this body. Higher values increase the strength of this effect.

### damping_coefficient {#damping_coefficient}

```gdscript
damping_coefficient: float
```

- void set_damping_coefficient(value: float)
- float get_damping_coefficient()

The body's damping coefficient. Higher values will slow down the body more noticeably when forces are applied.

### drag_coefficient {#drag_coefficient}

```gdscript
drag_coefficient: float
```

- void set_drag_coefficient(value: float)
- float get_drag_coefficient()

The body's drag coefficient. Higher values increase this body's air resistance.

Note: This value is currently unused by Godot's default physics implementation.

### ray_pickable {#ray_pickable}

```gdscript
ray_pickable: bool
```

- void set_ray_pickable(value: bool)
- bool is_ray_pickable()

If true, the SoftBody3D will respond to RayCast3Ds.

### disable_mode {#disable_mode}

```gdscript
disable_mode: int
```

- void set_disable_mode(value: DisableMode)
- DisableMode get_disable_mode()

Defines the behavior in physics when Node.process_mode is set to Node.PROCESS_MODE_DISABLED.

## 枚举

### DisableMode {#enum-DisableMode}

```gdscript
enum DisableMode { DISABLE_MODE_REMOVE = 0, DISABLE_MODE_KEEP_ACTIVE = 1 }
```

DisableMode DISABLE_MODE_REMOVE = 0

When Node.process_mode is set to Node.PROCESS_MODE_DISABLED, remove from the physics simulation to stop all physics interactions with this SoftBody3D.

Automatically re-added to the physics simulation when the Node is processed again.

DisableMode DISABLE_MODE_KEEP_ACTIVE = 1

When Node.process_mode is set to Node.PROCESS_MODE_DISABLED, do not affect the physics simulation.

#### DISABLE_MODE_REMOVE {#DISABLE_MODE_REMOVE}

```gdscript
DISABLE_MODE_REMOVE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DISABLE_MODE_KEEP_ACTIVE {#DISABLE_MODE_KEEP_ACTIVE}

```gdscript
DISABLE_MODE_KEEP_ACTIVE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
