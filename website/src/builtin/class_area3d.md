# Area3D

**继承**：[CollisionObject3D](class_collisionobject3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Area3D**

> Area3D is a region of 3D space defined by one or multiple CollisionShape3D or CollisionPolygon3D child nodes. It detects when other CollisionObject3Ds enter or exit it, and it also keeps track of which collision objects haven't exited it yet (i.e. which one are overlapping it).

<details><summary>官方英文描述（中文精译进行中）</summary>

Area3D is a region of 3D space defined by one or multiple CollisionShape3D or CollisionPolygon3D child nodes. It detects when other CollisionObject3Ds enter or exit it, and it also keeps track of which collision objects haven't exited it yet (i.e. which one are overlapping it).

This node can also locally alter or override physics parameters (gravity, damping) and route audio to custom audio buses.

Note: Areas and bodies created with PhysicsServer3D might not interact as expected with Area3Ds, and might not emit signals or track objects correctly.

Warning: Using a ConcavePolygonShape3D inside a CollisionShape3D child of this node (created e.g. by using the Create Trimesh Collision Sibling option in the Mesh menu that appears when selecting a MeshInstance3D node) may give unexpected results, since this collision shape is hollow. If this is not desired, it has to be split into multiple ConvexPolygonShape3Ds or primitive shapes like BoxShape3D, or in some cases it may be replaceable by a CollisionPolygon3D.

</details>

## 方法

### get_angular_damp {#get_angular_damp}

```gdscript
float get_angular_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_angular_damp_space_override_mode {#get_angular_damp_space_override_mode}

```gdscript
Area3D.SpaceOverride get_angular_damp_space_override_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_audio_bus_name {#get_audio_bus_name}

```gdscript
StringName get_audio_bus_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity {#get_gravity}

```gdscript
float get_gravity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity_direction {#get_gravity_direction}

```gdscript
Vector3 get_gravity_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity_point_center {#get_gravity_point_center}

```gdscript
Vector3 get_gravity_point_center() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity_point_unit_distance {#get_gravity_point_unit_distance}

```gdscript
float get_gravity_point_unit_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity_space_override_mode {#get_gravity_space_override_mode}

```gdscript
Area3D.SpaceOverride get_gravity_space_override_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_damp {#get_linear_damp}

```gdscript
float get_linear_damp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_damp_space_override_mode {#get_linear_damp_space_override_mode}

```gdscript
Area3D.SpaceOverride get_linear_damp_space_override_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_overlapping_areas {#get_overlapping_areas}

```gdscript
typedarray::Area3D get_overlapping_areas() const
```

Returns a list of intersecting Area3Ds. The overlapping area's CollisionObject3D.collision_layer must be part of this area's CollisionObject3D.collision_mask in order to be detected.

For performance reasons (collisions are all processed at the same time) this list is modified once during the physics step, not immediately after objects are moved. Consider using signals instead.

### get_overlapping_bodies {#get_overlapping_bodies}

```gdscript
typedarray::Node3D get_overlapping_bodies() const
```

Returns a list of intersecting PhysicsBody3Ds, SoftBody3Ds, and GridMaps. The overlapping body's CollisionObject3D.collision_layer must be part of this area's CollisionObject3D.collision_mask in order to be detected.

For performance reasons (collisions are all processed at the same time) this list is modified once during the physics step, not immediately after objects are moved. Consider using signals instead.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will not return any such bodies.

### get_priority {#get_priority}

```gdscript
int get_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_reverb_amount {#get_reverb_amount}

```gdscript
float get_reverb_amount() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_reverb_bus_name {#get_reverb_bus_name}

```gdscript
StringName get_reverb_bus_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_reverb_uniformity {#get_reverb_uniformity}

```gdscript
float get_reverb_uniformity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wind_attenuation_factor {#get_wind_attenuation_factor}

```gdscript
float get_wind_attenuation_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wind_force_magnitude {#get_wind_force_magnitude}

```gdscript
float get_wind_force_magnitude() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wind_source_path {#get_wind_source_path}

```gdscript
NodePath get_wind_source_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_overlapping_areas {#has_overlapping_areas}

```gdscript
bool has_overlapping_areas() const
```

Returns true if intersecting any Area3Ds, otherwise returns false. The overlapping area's CollisionObject3D.collision_layer must be part of this area's CollisionObject3D.collision_mask in order to be detected.

For performance reasons (collisions are all processed at the same time) the list of overlapping areas is modified once during the physics step, not immediately after objects are moved. Consider using signals instead.

### has_overlapping_bodies {#has_overlapping_bodies}

```gdscript
bool has_overlapping_bodies() const
```

Returns true if intersecting any PhysicsBody3Ds, SoftBody3Ds, or GridMaps, otherwise returns false. The overlapping body's CollisionObject3D.collision_layer must be part of this area's CollisionObject3D.collision_mask in order to be detected.

For performance reasons (collisions are all processed at the same time) the list of overlapping bodies is modified once during the physics step, not immediately after objects are moved. Consider using signals instead.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will not consider such bodies.

### is_gravity_a_point {#is_gravity_a_point}

```gdscript
bool is_gravity_a_point() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_monitorable {#is_monitorable}

```gdscript
bool is_monitorable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_monitoring {#is_monitoring}

```gdscript
bool is_monitoring() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_overriding_audio_bus {#is_overriding_audio_bus}

```gdscript
bool is_overriding_audio_bus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_reverb_bus {#is_using_reverb_bus}

```gdscript
bool is_using_reverb_bus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### overlaps_area {#overlaps_area}

```gdscript
bool overlaps_area(area: Node) const
```

Returns true if the given Area3D intersects or overlaps this Area3D, false otherwise.

Note: The result of this test is not immediate after moving objects. For performance, list of overlaps is updated once per frame and before the physics step. Consider using signals instead.

### overlaps_body {#overlaps_body}

```gdscript
bool overlaps_body(body: Node) const
```

Returns true if the given physics body intersects or overlaps this Area3D, false otherwise.

body argument can either be a PhysicsBody3D, SoftBody3D, or a GridMap instance. While GridMaps are not physics body themselves, they register their tiles with collision shapes as a virtual physics body.

Note: The result of this test is not immediate after moving objects. For performance, list of overlaps is updated once per frame and before the physics step. Consider using signals instead.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will return false in such cases.

### set_angular_damp {#set_angular_damp}

```gdscript
void set_angular_damp(angular_damp: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_damp_space_override_mode {#set_angular_damp_space_override_mode}

```gdscript
void set_angular_damp_space_override_mode(space_override_mode: Area3D.SpaceOverride)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_audio_bus_name {#set_audio_bus_name}

```gdscript
void set_audio_bus_name(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_audio_bus_override {#set_audio_bus_override}

```gdscript
void set_audio_bus_override(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity {#set_gravity}

```gdscript
void set_gravity(gravity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_direction {#set_gravity_direction}

```gdscript
void set_gravity_direction(direction: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_is_point {#set_gravity_is_point}

```gdscript
void set_gravity_is_point(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_point_center {#set_gravity_point_center}

```gdscript
void set_gravity_point_center(center: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_point_unit_distance {#set_gravity_point_unit_distance}

```gdscript
void set_gravity_point_unit_distance(distance_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity_space_override_mode {#set_gravity_space_override_mode}

```gdscript
void set_gravity_space_override_mode(space_override_mode: Area3D.SpaceOverride)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_damp {#set_linear_damp}

```gdscript
void set_linear_damp(linear_damp: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_damp_space_override_mode {#set_linear_damp_space_override_mode}

```gdscript
void set_linear_damp_space_override_mode(space_override_mode: Area3D.SpaceOverride)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_monitorable {#set_monitorable}

```gdscript
void set_monitorable(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_monitoring {#set_monitoring}

```gdscript
void set_monitoring(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_priority {#set_priority}

```gdscript
void set_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_reverb_amount {#set_reverb_amount}

```gdscript
void set_reverb_amount(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_reverb_bus_name {#set_reverb_bus_name}

```gdscript
void set_reverb_bus_name(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_reverb_uniformity {#set_reverb_uniformity}

```gdscript
void set_reverb_uniformity(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_reverb_bus {#set_use_reverb_bus}

```gdscript
void set_use_reverb_bus(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wind_attenuation_factor {#set_wind_attenuation_factor}

```gdscript
void set_wind_attenuation_factor(wind_attenuation_factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wind_force_magnitude {#set_wind_force_magnitude}

```gdscript
void set_wind_force_magnitude(wind_force_magnitude: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wind_source_path {#set_wind_source_path}

```gdscript
void set_wind_source_path(wind_source_path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### monitoring {#monitoring}

```gdscript
monitoring: bool
```

- void set_monitoring(value: bool)
- bool is_monitoring()

If true, the area detects bodies or areas entering and exiting it.

### monitorable {#monitorable}

```gdscript
monitorable: bool
```

- void set_monitorable(value: bool)
- bool is_monitorable()

If true, other monitoring areas can detect this area.

### priority {#priority}

```gdscript
priority: int
```

- void set_priority(value: int)
- int get_priority()

The area's priority. Higher priority areas are processed first. The World3D's physics is always processed last, after all areas.

### gravity_space_override {#gravity_space_override}

```gdscript
gravity_space_override: int
```

- void set_gravity_space_override_mode(value: SpaceOverride)
- SpaceOverride get_gravity_space_override_mode()

Override mode for gravity calculations within this area.

### gravity_point {#gravity_point}

```gdscript
gravity_point: bool
```

- void set_gravity_is_point(value: bool)
- bool is_gravity_a_point()

If true, gravity is calculated from a point (set via gravity_point_center). See also gravity_space_override.

### gravity_point_unit_distance {#gravity_point_unit_distance}

```gdscript
gravity_point_unit_distance: float
```

- void set_gravity_point_unit_distance(value: float)
- float get_gravity_point_unit_distance()

The distance at which the gravity strength is equal to gravity. For example, on a planet 100 meters in radius with a surface gravity of 4.0 m/s², set the gravity to 4.0 and the unit distance to 100.0. The gravity will have falloff according to the inverse square law, so in the example, at 200 meters from the center the gravity will be 1.0 m/s² (twice the distance, 1/4th the gravity), at 50 meters it will be 16.0 m/s² (half the distance, 4x the gravity), and so on.

The above is true only when the unit distance is a positive number. When this is set to 0.0, the gravity will be constant regardless of distance.

### gravity_point_center {#gravity_point_center}

```gdscript
gravity_point_center: Vector3
```

- void set_gravity_point_center(value: Vector3)
- Vector3 get_gravity_point_center()

If gravity is a point (see gravity_point), this will be the point of attraction.

### gravity_direction {#gravity_direction}

```gdscript
gravity_direction: Vector3
```

- void set_gravity_direction(value: Vector3)
- Vector3 get_gravity_direction()

The area's gravity vector (not normalized).

### gravity {#gravity}

```gdscript
gravity: float
```

- void set_gravity(value: float)
- float get_gravity()

The area's gravity intensity (in meters per second squared). This value multiplies the gravity direction. This is useful to alter the force of gravity without altering its direction.

### linear_damp_space_override {#linear_damp_space_override}

```gdscript
linear_damp_space_override: int
```

- void set_linear_damp_space_override_mode(value: SpaceOverride)
- SpaceOverride get_linear_damp_space_override_mode()

Override mode for linear damping calculations within this area.

### linear_damp {#linear_damp}

```gdscript
linear_damp: float
```

- void set_linear_damp(value: float)
- float get_linear_damp()

The rate at which objects stop moving in this area. Represents the linear velocity lost per second.

See ProjectSettings.physics/3d/default_linear_damp for more details about damping.

### angular_damp_space_override {#angular_damp_space_override}

```gdscript
angular_damp_space_override: int
```

- void set_angular_damp_space_override_mode(value: SpaceOverride)
- SpaceOverride get_angular_damp_space_override_mode()

Override mode for angular damping calculations within this area.

### angular_damp {#angular_damp}

```gdscript
angular_damp: float
```

- void set_angular_damp(value: float)
- float get_angular_damp()

The rate at which objects stop spinning in this area. Represents the angular velocity lost per second.

See ProjectSettings.physics/3d/default_angular_damp for more details about damping.

### wind_force_magnitude {#wind_force_magnitude}

```gdscript
wind_force_magnitude: float
```

- void set_wind_force_magnitude(value: float)
- float get_wind_force_magnitude()

The magnitude of area-specific wind force.

Note: This wind force only applies to SoftBody3D nodes. Other physics bodies are currently not affected by wind.

### wind_attenuation_factor {#wind_attenuation_factor}

```gdscript
wind_attenuation_factor: float
```

- void set_wind_attenuation_factor(value: float)
- float get_wind_attenuation_factor()

The exponential rate at which wind force decreases with distance from its origin.

Note: This wind force only applies to SoftBody3D nodes. Other physics bodies are currently not affected by wind.

### wind_source_path {#wind_source_path}

```gdscript
wind_source_path: NodePath
```

- void set_wind_source_path(value: NodePath)
- NodePath get_wind_source_path()

The Node3D which is used to specify the direction and origin of an area-specific wind force. The direction is opposite to the z-axis of the Node3D's local transform, and its origin is the origin of the Node3D's local transform.

Note: This wind force only applies to SoftBody3D nodes. Other physics bodies are currently not affected by wind.

### audio_bus_override {#audio_bus_override}

```gdscript
audio_bus_override: bool
```

- void set_audio_bus_override(value: bool)
- bool is_overriding_audio_bus()

If true, the area's audio bus overrides the default audio bus.

### audio_bus_name {#audio_bus_name}

```gdscript
audio_bus_name: StringName
```

- void set_audio_bus_name(value: StringName)
- StringName get_audio_bus_name()

The name of the area's audio bus.

### reverb_bus_enabled {#reverb_bus_enabled}

```gdscript
reverb_bus_enabled: bool
```

- void set_use_reverb_bus(value: bool)
- bool is_using_reverb_bus()

If true, the area applies reverb to its associated audio.

### reverb_bus_name {#reverb_bus_name}

```gdscript
reverb_bus_name: StringName
```

- void set_reverb_bus_name(value: StringName)
- StringName get_reverb_bus_name()

The name of the reverb bus to use for this area's associated audio.

### reverb_bus_amount {#reverb_bus_amount}

```gdscript
reverb_bus_amount: float
```

- void set_reverb_amount(value: float)
- float get_reverb_amount()

The degree to which this area applies reverb to its associated audio. Ranges from 0 to 1 with 0.1 precision.

### reverb_bus_uniformity {#reverb_bus_uniformity}

```gdscript
reverb_bus_uniformity: float
```

- void set_reverb_uniformity(value: float)
- float get_reverb_uniformity()

The degree to which this area's reverb is a uniform effect. Ranges from 0 to 1 with 0.1 precision.

## 信号

### body_shape_entered {#body_shape_entered}

```gdscript
signal body_shape_entered(body_rid: RID, body: Node3D, body_shape_index: int, local_shape_index: int)
```

Emitted when a Shape3D of the received body enters a shape of this area. body can be a PhysicsBody3D, SoftBody3D or GridMap. GridMaps are detected if their MeshLibrary has collision shapes configured. Requires monitoring to be set to true.

local_shape_index and body_shape_index contain indices of the interacting shapes from this area and the interacting body, respectively. body_rid contains the RID of the body. These values can be used with the PhysicsServer3D.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will not emit this signal in such cases.

Example: Get the CollisionShape3D node from the shape index:

```
var body_shape_owner = body.shape_find_owner(body_shape_index)
var body_shape_node = body.shape_owner_get_owner(body_shape_owner)
var local_shape_owner = shape_find_owner(local_shape_index)
var local_shape_node = shape_owner_get_owner(local_shape_owner)
```

### body_shape_exited {#body_shape_exited}

```gdscript
signal body_shape_exited(body_rid: RID, body: Node3D, body_shape_index: int, local_shape_index: int)
```

Emitted when a Shape3D of the received body exits a shape of this area. body can be a PhysicsBody3D, SoftBody3D or GridMap. GridMaps are detected if their MeshLibrary has collision shapes configured. Requires monitoring to be set to true.

See also body_shape_entered.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will not emit this signal in such cases.

### body_entered {#body_entered}

```gdscript
signal body_entered(body: Node3D)
```

Emitted when the received body enters this area. body can be a PhysicsBody3D, SoftBody3D or GridMap. GridMaps are detected if their MeshLibrary has collision shapes configured. Requires monitoring to be set to true.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will not emit this signal in such cases.

### body_exited {#body_exited}

```gdscript
signal body_exited(body: Node3D)
```

Emitted when the received body exits this area. body can be a PhysicsBody3D, SoftBody3D or GridMap. GridMaps are detected if their MeshLibrary has collision shapes configured. Requires monitoring to be set to true.

Note: Godot Physics does not support reporting overlaps with SoftBody3D, so will not emit this signal in such cases.

### area_shape_entered {#area_shape_entered}

```gdscript
signal area_shape_entered(area_rid: RID, area: Area3D, area_shape_index: int, local_shape_index: int)
```

Emitted when a Shape3D of the received area enters a shape of this area. Requires monitoring to be set to true.

local_shape_index and area_shape_index contain indices of the interacting shapes from this area and the other area, respectively. area_rid contains the RID of the other area. These values can be used with the PhysicsServer3D.

Example: Get the CollisionShape3D node from the shape index:

```
var other_shape_owner = area.shape_find_owner(area_shape_index)
var other_shape_node = area.shape_owner_get_owner(other_shape_owner)
var local_shape_owner = shape_find_owner(local_shape_index)
var local_shape_node = shape_owner_get_owner(local_shape_owner)
```

### area_shape_exited {#area_shape_exited}

```gdscript
signal area_shape_exited(area_rid: RID, area: Area3D, area_shape_index: int, local_shape_index: int)
```

Emitted when a Shape3D of the received area exits a shape of this area. Requires monitoring to be set to true.

See also area_shape_entered.

### area_entered {#area_entered}

```gdscript
signal area_entered(area: Area3D)
```

Emitted when the received area enters this area. Requires monitoring to be set to true.

### area_exited {#area_exited}

```gdscript
signal area_exited(area: Area3D)
```

Emitted when the received area exits this area. Requires monitoring to be set to true.

## 枚举

### SpaceOverride {#enum-SpaceOverride}

```gdscript
enum SpaceOverride { SPACE_OVERRIDE_DISABLED = 0, SPACE_OVERRIDE_COMBINE = 1, SPACE_OVERRIDE_COMBINE_REPLACE = 2, SPACE_OVERRIDE_REPLACE = 3, SPACE_OVERRIDE_REPLACE_COMBINE = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SPACE_OVERRIDE_DISABLED {#SPACE_OVERRIDE_DISABLED}

```gdscript
SPACE_OVERRIDE_DISABLED = 0
```

This area does not affect gravity/damping.

#### SPACE_OVERRIDE_COMBINE {#SPACE_OVERRIDE_COMBINE}

```gdscript
SPACE_OVERRIDE_COMBINE = 1
```

This area adds its gravity/damping values to whatever has been calculated so far (in priority order).

#### SPACE_OVERRIDE_COMBINE_REPLACE {#SPACE_OVERRIDE_COMBINE_REPLACE}

```gdscript
SPACE_OVERRIDE_COMBINE_REPLACE = 2
```

This area adds its gravity/damping values to whatever has been calculated so far (in priority order), ignoring any lower priority areas.

#### SPACE_OVERRIDE_REPLACE {#SPACE_OVERRIDE_REPLACE}

```gdscript
SPACE_OVERRIDE_REPLACE = 3
```

This area replaces any gravity/damping, even the defaults, ignoring any lower priority areas.

#### SPACE_OVERRIDE_REPLACE_COMBINE {#SPACE_OVERRIDE_REPLACE_COMBINE}

```gdscript
SPACE_OVERRIDE_REPLACE_COMBINE = 4
```

This area replaces any gravity/damping calculated so far (in priority order), but keeps calculating the rest of the areas.

---
