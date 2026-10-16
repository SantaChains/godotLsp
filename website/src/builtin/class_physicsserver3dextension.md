# PhysicsServer3DExtension

**继承**：[PhysicsServer3D](class_physicsserver3d.md) → [Object](class_object.md) → **PhysicsServer3DExtension**

> This class extends PhysicsServer3D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

<details><summary>官方英文描述</summary>

This class extends PhysicsServer3D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

Intended for use with GDExtension to create custom implementations of PhysicsServer3D.

</details>

## 方法

### _area_add_shape {#_area_add_shape}

```gdscript
void _area_add_shape(area: RID, shape: RID, transform: Transform3D, disabled: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_attach_object_instance_id {#_area_attach_object_instance_id}

```gdscript
void _area_attach_object_instance_id(area: RID, id: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_clear_shapes {#_area_clear_shapes}

```gdscript
void _area_clear_shapes(area: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_create {#_area_create}

```gdscript
RID _area_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_collision_layer {#_area_get_collision_layer}

```gdscript
int _area_get_collision_layer(area: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_collision_mask {#_area_get_collision_mask}

```gdscript
int _area_get_collision_mask(area: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_object_instance_id {#_area_get_object_instance_id}

```gdscript
int _area_get_object_instance_id(area: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_param {#_area_get_param}

```gdscript
Variant _area_get_param(area: RID, param: PhysicsServer3D.AreaParameter) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_shape {#_area_get_shape}

```gdscript
RID _area_get_shape(area: RID, shape_idx: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_shape_count {#_area_get_shape_count}

```gdscript
int _area_get_shape_count(area: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_shape_transform {#_area_get_shape_transform}

```gdscript
Transform3D _area_get_shape_transform(area: RID, shape_idx: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_space {#_area_get_space}

```gdscript
RID _area_get_space(area: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_get_transform {#_area_get_transform}

```gdscript
Transform3D _area_get_transform(area: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_remove_shape {#_area_remove_shape}

```gdscript
void _area_remove_shape(area: RID, shape_idx: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_area_monitor_callback {#_area_set_area_monitor_callback}

```gdscript
void _area_set_area_monitor_callback(area: RID, callback: Callable) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_collision_layer {#_area_set_collision_layer}

```gdscript
void _area_set_collision_layer(area: RID, layer: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_collision_mask {#_area_set_collision_mask}

```gdscript
void _area_set_collision_mask(area: RID, mask: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_monitor_callback {#_area_set_monitor_callback}

```gdscript
void _area_set_monitor_callback(area: RID, callback: Callable) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_monitorable {#_area_set_monitorable}

```gdscript
void _area_set_monitorable(area: RID, monitorable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_param {#_area_set_param}

```gdscript
void _area_set_param(area: RID, param: PhysicsServer3D.AreaParameter, value: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_ray_pickable {#_area_set_ray_pickable}

```gdscript
void _area_set_ray_pickable(area: RID, enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_shape {#_area_set_shape}

```gdscript
void _area_set_shape(area: RID, shape_idx: int, shape: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_shape_disabled {#_area_set_shape_disabled}

```gdscript
void _area_set_shape_disabled(area: RID, shape_idx: int, disabled: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_shape_transform {#_area_set_shape_transform}

```gdscript
void _area_set_shape_transform(area: RID, shape_idx: int, transform: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_space {#_area_set_space}

```gdscript
void _area_set_space(area: RID, space: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _area_set_transform {#_area_set_transform}

```gdscript
void _area_set_transform(area: RID, transform: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_add_collision_exception {#_body_add_collision_exception}

```gdscript
void _body_add_collision_exception(body: RID, excepted_body: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_add_constant_central_force {#_body_add_constant_central_force}

```gdscript
void _body_add_constant_central_force(body: RID, force: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_add_constant_force {#_body_add_constant_force}

```gdscript
void _body_add_constant_force(body: RID, force: Vector3, position: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_add_constant_torque {#_body_add_constant_torque}

```gdscript
void _body_add_constant_torque(body: RID, torque: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_add_shape {#_body_add_shape}

```gdscript
void _body_add_shape(body: RID, shape: RID, transform: Transform3D, disabled: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_apply_central_force {#_body_apply_central_force}

```gdscript
void _body_apply_central_force(body: RID, force: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_apply_central_impulse {#_body_apply_central_impulse}

```gdscript
void _body_apply_central_impulse(body: RID, impulse: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_apply_force {#_body_apply_force}

```gdscript
void _body_apply_force(body: RID, force: Vector3, position: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_apply_impulse {#_body_apply_impulse}

```gdscript
void _body_apply_impulse(body: RID, impulse: Vector3, position: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_apply_torque {#_body_apply_torque}

```gdscript
void _body_apply_torque(body: RID, torque: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_apply_torque_impulse {#_body_apply_torque_impulse}

```gdscript
void _body_apply_torque_impulse(body: RID, impulse: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_attach_object_instance_id {#_body_attach_object_instance_id}

```gdscript
void _body_attach_object_instance_id(body: RID, id: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_clear_shapes {#_body_clear_shapes}

```gdscript
void _body_clear_shapes(body: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_create {#_body_create}

```gdscript
RID _body_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_collision_exceptions {#_body_get_collision_exceptions}

```gdscript
typedarray::RID _body_get_collision_exceptions(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_collision_layer {#_body_get_collision_layer}

```gdscript
int _body_get_collision_layer(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_collision_mask {#_body_get_collision_mask}

```gdscript
int _body_get_collision_mask(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_collision_priority {#_body_get_collision_priority}

```gdscript
float _body_get_collision_priority(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_constant_force {#_body_get_constant_force}

```gdscript
Vector3 _body_get_constant_force(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_constant_torque {#_body_get_constant_torque}

```gdscript
Vector3 _body_get_constant_torque(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_contacts_reported_depth_threshold {#_body_get_contacts_reported_depth_threshold}

```gdscript
float _body_get_contacts_reported_depth_threshold(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_direct_state {#_body_get_direct_state}

```gdscript
PhysicsDirectBodyState3D _body_get_direct_state(body: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_max_contacts_reported {#_body_get_max_contacts_reported}

```gdscript
int _body_get_max_contacts_reported(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_mode {#_body_get_mode}

```gdscript
PhysicsServer3D.BodyMode _body_get_mode(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_object_instance_id {#_body_get_object_instance_id}

```gdscript
int _body_get_object_instance_id(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_param {#_body_get_param}

```gdscript
Variant _body_get_param(body: RID, param: PhysicsServer3D.BodyParameter) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_shape {#_body_get_shape}

```gdscript
RID _body_get_shape(body: RID, shape_idx: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_shape_count {#_body_get_shape_count}

```gdscript
int _body_get_shape_count(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_shape_transform {#_body_get_shape_transform}

```gdscript
Transform3D _body_get_shape_transform(body: RID, shape_idx: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_space {#_body_get_space}

```gdscript
RID _body_get_space(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_state {#_body_get_state}

```gdscript
Variant _body_get_state(body: RID, state: PhysicsServer3D.BodyState) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_get_user_flags {#_body_get_user_flags}

```gdscript
int _body_get_user_flags(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_is_axis_locked {#_body_is_axis_locked}

```gdscript
bool _body_is_axis_locked(body: RID, axis: PhysicsServer3D.BodyAxis) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_is_continuous_collision_detection_enabled {#_body_is_continuous_collision_detection_enabled}

```gdscript
bool _body_is_continuous_collision_detection_enabled(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_is_omitting_force_integration {#_body_is_omitting_force_integration}

```gdscript
bool _body_is_omitting_force_integration(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_remove_collision_exception {#_body_remove_collision_exception}

```gdscript
void _body_remove_collision_exception(body: RID, excepted_body: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_remove_shape {#_body_remove_shape}

```gdscript
void _body_remove_shape(body: RID, shape_idx: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_reset_mass_properties {#_body_reset_mass_properties}

```gdscript
void _body_reset_mass_properties(body: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_axis_lock {#_body_set_axis_lock}

```gdscript
void _body_set_axis_lock(body: RID, axis: PhysicsServer3D.BodyAxis, lock: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_axis_velocity {#_body_set_axis_velocity}

```gdscript
void _body_set_axis_velocity(body: RID, axis_velocity: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_collision_layer {#_body_set_collision_layer}

```gdscript
void _body_set_collision_layer(body: RID, layer: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_collision_mask {#_body_set_collision_mask}

```gdscript
void _body_set_collision_mask(body: RID, mask: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_collision_priority {#_body_set_collision_priority}

```gdscript
void _body_set_collision_priority(body: RID, priority: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_constant_force {#_body_set_constant_force}

```gdscript
void _body_set_constant_force(body: RID, force: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_constant_torque {#_body_set_constant_torque}

```gdscript
void _body_set_constant_torque(body: RID, torque: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_contacts_reported_depth_threshold {#_body_set_contacts_reported_depth_threshold}

```gdscript
void _body_set_contacts_reported_depth_threshold(body: RID, threshold: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_enable_continuous_collision_detection {#_body_set_enable_continuous_collision_detection}

```gdscript
void _body_set_enable_continuous_collision_detection(body: RID, enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_force_integration_callback {#_body_set_force_integration_callback}

```gdscript
void _body_set_force_integration_callback(body: RID, callable: Callable, userdata: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_max_contacts_reported {#_body_set_max_contacts_reported}

```gdscript
void _body_set_max_contacts_reported(body: RID, amount: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_mode {#_body_set_mode}

```gdscript
void _body_set_mode(body: RID, mode: PhysicsServer3D.BodyMode) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_omit_force_integration {#_body_set_omit_force_integration}

```gdscript
void _body_set_omit_force_integration(body: RID, enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_param {#_body_set_param}

```gdscript
void _body_set_param(body: RID, param: PhysicsServer3D.BodyParameter, value: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_ray_pickable {#_body_set_ray_pickable}

```gdscript
void _body_set_ray_pickable(body: RID, enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_shape {#_body_set_shape}

```gdscript
void _body_set_shape(body: RID, shape_idx: int, shape: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_shape_disabled {#_body_set_shape_disabled}

```gdscript
void _body_set_shape_disabled(body: RID, shape_idx: int, disabled: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_shape_transform {#_body_set_shape_transform}

```gdscript
void _body_set_shape_transform(body: RID, shape_idx: int, transform: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_space {#_body_set_space}

```gdscript
void _body_set_space(body: RID, space: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_state {#_body_set_state}

```gdscript
void _body_set_state(body: RID, state: PhysicsServer3D.BodyState, value: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_state_sync_callback {#_body_set_state_sync_callback}

```gdscript
void _body_set_state_sync_callback(body: RID, callable: Callable) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_set_user_flags {#_body_set_user_flags}

```gdscript
void _body_set_user_flags(body: RID, flags: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _body_test_motion {#_body_test_motion}

```gdscript
bool _body_test_motion(body: RID, from: Transform3D, motion: Vector3, margin: float, max_collisions: int, collide_separation_ray: bool, recovery_as_collision: bool, r_result: PhysicsServer3DExtensionMotionResult*) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _box_shape_create {#_box_shape_create}

```gdscript
RID _box_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _capsule_shape_create {#_capsule_shape_create}

```gdscript
RID _capsule_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _concave_polygon_shape_create {#_concave_polygon_shape_create}

```gdscript
RID _concave_polygon_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _cone_twist_joint_get_param {#_cone_twist_joint_get_param}

```gdscript
float _cone_twist_joint_get_param(joint: RID, param: PhysicsServer3D.ConeTwistJointParam) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _cone_twist_joint_set_param {#_cone_twist_joint_set_param}

```gdscript
void _cone_twist_joint_set_param(joint: RID, param: PhysicsServer3D.ConeTwistJointParam, value: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _convex_polygon_shape_create {#_convex_polygon_shape_create}

```gdscript
RID _convex_polygon_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _custom_shape_create {#_custom_shape_create}

```gdscript
RID _custom_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _cylinder_shape_create {#_cylinder_shape_create}

```gdscript
RID _cylinder_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _end_sync {#_end_sync}

```gdscript
void _end_sync() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _finish {#_finish}

```gdscript
void _finish() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _flush_queries {#_flush_queries}

```gdscript
void _flush_queries() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _free_rid {#_free_rid}

```gdscript
void _free_rid(rid: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _generic_6dof_joint_get_flag {#_generic_6dof_joint_get_flag}

```gdscript
bool _generic_6dof_joint_get_flag(joint: RID, axis: Vector3.Axis, flag: PhysicsServer3D.G6DOFJointAxisFlag) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _generic_6dof_joint_get_param {#_generic_6dof_joint_get_param}

```gdscript
float _generic_6dof_joint_get_param(joint: RID, axis: Vector3.Axis, param: PhysicsServer3D.G6DOFJointAxisParam) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _generic_6dof_joint_set_flag {#_generic_6dof_joint_set_flag}

```gdscript
void _generic_6dof_joint_set_flag(joint: RID, axis: Vector3.Axis, flag: PhysicsServer3D.G6DOFJointAxisFlag, enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _generic_6dof_joint_set_param {#_generic_6dof_joint_set_param}

```gdscript
void _generic_6dof_joint_set_param(joint: RID, axis: Vector3.Axis, param: PhysicsServer3D.G6DOFJointAxisParam, value: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_process_info {#_get_process_info}

```gdscript
int _get_process_info(process_info: PhysicsServer3D.ProcessInfo) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _heightmap_shape_create {#_heightmap_shape_create}

```gdscript
RID _heightmap_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _hinge_joint_get_flag {#_hinge_joint_get_flag}

```gdscript
bool _hinge_joint_get_flag(joint: RID, flag: PhysicsServer3D.HingeJointFlag) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _hinge_joint_get_param {#_hinge_joint_get_param}

```gdscript
float _hinge_joint_get_param(joint: RID, param: PhysicsServer3D.HingeJointParam) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _hinge_joint_set_flag {#_hinge_joint_set_flag}

```gdscript
void _hinge_joint_set_flag(joint: RID, flag: PhysicsServer3D.HingeJointFlag, enabled: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _hinge_joint_set_param {#_hinge_joint_set_param}

```gdscript
void _hinge_joint_set_param(joint: RID, param: PhysicsServer3D.HingeJointParam, value: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _init {#_init}

```gdscript
void _init() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _is_flushing_queries {#_is_flushing_queries}

```gdscript
bool _is_flushing_queries() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_clear {#_joint_clear}

```gdscript
void _joint_clear(joint: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_create {#_joint_create}

```gdscript
RID _joint_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_disable_collisions_between_bodies {#_joint_disable_collisions_between_bodies}

```gdscript
void _joint_disable_collisions_between_bodies(joint: RID, disable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_get_solver_priority {#_joint_get_solver_priority}

```gdscript
int _joint_get_solver_priority(joint: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_get_type {#_joint_get_type}

```gdscript
PhysicsServer3D.JointType _joint_get_type(joint: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_is_disabled_collisions_between_bodies {#_joint_is_disabled_collisions_between_bodies}

```gdscript
bool _joint_is_disabled_collisions_between_bodies(joint: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_make_cone_twist {#_joint_make_cone_twist}

```gdscript
void _joint_make_cone_twist(joint: RID, body_A: RID, local_ref_A: Transform3D, body_B: RID, local_ref_B: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_make_generic_6dof {#_joint_make_generic_6dof}

```gdscript
void _joint_make_generic_6dof(joint: RID, body_A: RID, local_ref_A: Transform3D, body_B: RID, local_ref_B: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_make_hinge {#_joint_make_hinge}

```gdscript
void _joint_make_hinge(joint: RID, body_A: RID, hinge_A: Transform3D, body_B: RID, hinge_B: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_make_hinge_simple {#_joint_make_hinge_simple}

```gdscript
void _joint_make_hinge_simple(joint: RID, body_A: RID, pivot_A: Vector3, axis_A: Vector3, body_B: RID, pivot_B: Vector3, axis_B: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_make_pin {#_joint_make_pin}

```gdscript
void _joint_make_pin(joint: RID, body_A: RID, local_A: Vector3, body_B: RID, local_B: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_make_slider {#_joint_make_slider}

```gdscript
void _joint_make_slider(joint: RID, body_A: RID, local_ref_A: Transform3D, body_B: RID, local_ref_B: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _joint_set_solver_priority {#_joint_set_solver_priority}

```gdscript
void _joint_set_solver_priority(joint: RID, priority: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _pin_joint_get_local_a {#_pin_joint_get_local_a}

```gdscript
Vector3 _pin_joint_get_local_a(joint: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _pin_joint_get_local_b {#_pin_joint_get_local_b}

```gdscript
Vector3 _pin_joint_get_local_b(joint: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _pin_joint_get_param {#_pin_joint_get_param}

```gdscript
float _pin_joint_get_param(joint: RID, param: PhysicsServer3D.PinJointParam) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _pin_joint_set_local_a {#_pin_joint_set_local_a}

```gdscript
void _pin_joint_set_local_a(joint: RID, local_A: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _pin_joint_set_local_b {#_pin_joint_set_local_b}

```gdscript
void _pin_joint_set_local_b(joint: RID, local_B: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _pin_joint_set_param {#_pin_joint_set_param}

```gdscript
void _pin_joint_set_param(joint: RID, param: PhysicsServer3D.PinJointParam, value: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _separation_ray_shape_create {#_separation_ray_shape_create}

```gdscript
RID _separation_ray_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _set_active {#_set_active}

```gdscript
void _set_active(active: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_get_custom_solver_bias {#_shape_get_custom_solver_bias}

```gdscript
float _shape_get_custom_solver_bias(shape: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_get_data {#_shape_get_data}

```gdscript
Variant _shape_get_data(shape: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_get_margin {#_shape_get_margin}

```gdscript
float _shape_get_margin(shape: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_get_type {#_shape_get_type}

```gdscript
PhysicsServer3D.ShapeType _shape_get_type(shape: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_set_custom_solver_bias {#_shape_set_custom_solver_bias}

```gdscript
void _shape_set_custom_solver_bias(shape: RID, bias: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_set_data {#_shape_set_data}

```gdscript
void _shape_set_data(shape: RID, data: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _shape_set_margin {#_shape_set_margin}

```gdscript
void _shape_set_margin(shape: RID, margin: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _slider_joint_get_param {#_slider_joint_get_param}

```gdscript
float _slider_joint_get_param(joint: RID, param: PhysicsServer3D.SliderJointParam) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _slider_joint_set_param {#_slider_joint_set_param}

```gdscript
void _slider_joint_set_param(joint: RID, param: PhysicsServer3D.SliderJointParam, value: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_add_collision_exception {#_soft_body_add_collision_exception}

```gdscript
void _soft_body_add_collision_exception(body: RID, body_b: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_apply_central_force {#_soft_body_apply_central_force}

```gdscript
void _soft_body_apply_central_force(body: RID, force: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_apply_central_impulse {#_soft_body_apply_central_impulse}

```gdscript
void _soft_body_apply_central_impulse(body: RID, impulse: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_apply_point_force {#_soft_body_apply_point_force}

```gdscript
void _soft_body_apply_point_force(body: RID, point_index: int, force: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_apply_point_impulse {#_soft_body_apply_point_impulse}

```gdscript
void _soft_body_apply_point_impulse(body: RID, point_index: int, impulse: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_create {#_soft_body_create}

```gdscript
RID _soft_body_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_bounds {#_soft_body_get_bounds}

```gdscript
AABB _soft_body_get_bounds(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_collision_exceptions {#_soft_body_get_collision_exceptions}

```gdscript
typedarray::RID _soft_body_get_collision_exceptions(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_collision_layer {#_soft_body_get_collision_layer}

```gdscript
int _soft_body_get_collision_layer(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_collision_mask {#_soft_body_get_collision_mask}

```gdscript
int _soft_body_get_collision_mask(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_damping_coefficient {#_soft_body_get_damping_coefficient}

```gdscript
float _soft_body_get_damping_coefficient(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_drag_coefficient {#_soft_body_get_drag_coefficient}

```gdscript
float _soft_body_get_drag_coefficient(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_linear_stiffness {#_soft_body_get_linear_stiffness}

```gdscript
float _soft_body_get_linear_stiffness(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_point_global_position {#_soft_body_get_point_global_position}

```gdscript
Vector3 _soft_body_get_point_global_position(body: RID, point_index: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_pressure_coefficient {#_soft_body_get_pressure_coefficient}

```gdscript
float _soft_body_get_pressure_coefficient(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_shrinking_factor {#_soft_body_get_shrinking_factor}

```gdscript
float _soft_body_get_shrinking_factor(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_simulation_precision {#_soft_body_get_simulation_precision}

```gdscript
int _soft_body_get_simulation_precision(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_space {#_soft_body_get_space}

```gdscript
RID _soft_body_get_space(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_state {#_soft_body_get_state}

```gdscript
Variant _soft_body_get_state(body: RID, state: PhysicsServer3D.BodyState) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_get_total_mass {#_soft_body_get_total_mass}

```gdscript
float _soft_body_get_total_mass(body: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_is_point_pinned {#_soft_body_is_point_pinned}

```gdscript
bool _soft_body_is_point_pinned(body: RID, point_index: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_move_point {#_soft_body_move_point}

```gdscript
void _soft_body_move_point(body: RID, point_index: int, global_position: Vector3) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_pin_point {#_soft_body_pin_point}

```gdscript
void _soft_body_pin_point(body: RID, point_index: int, pin: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_remove_all_pinned_points {#_soft_body_remove_all_pinned_points}

```gdscript
void _soft_body_remove_all_pinned_points(body: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_remove_collision_exception {#_soft_body_remove_collision_exception}

```gdscript
void _soft_body_remove_collision_exception(body: RID, body_b: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_collision_layer {#_soft_body_set_collision_layer}

```gdscript
void _soft_body_set_collision_layer(body: RID, layer: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_collision_mask {#_soft_body_set_collision_mask}

```gdscript
void _soft_body_set_collision_mask(body: RID, mask: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_damping_coefficient {#_soft_body_set_damping_coefficient}

```gdscript
void _soft_body_set_damping_coefficient(body: RID, damping_coefficient: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_drag_coefficient {#_soft_body_set_drag_coefficient}

```gdscript
void _soft_body_set_drag_coefficient(body: RID, drag_coefficient: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_linear_stiffness {#_soft_body_set_linear_stiffness}

```gdscript
void _soft_body_set_linear_stiffness(body: RID, linear_stiffness: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_mesh {#_soft_body_set_mesh}

```gdscript
void _soft_body_set_mesh(body: RID, mesh: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_pressure_coefficient {#_soft_body_set_pressure_coefficient}

```gdscript
void _soft_body_set_pressure_coefficient(body: RID, pressure_coefficient: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_ray_pickable {#_soft_body_set_ray_pickable}

```gdscript
void _soft_body_set_ray_pickable(body: RID, enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_shrinking_factor {#_soft_body_set_shrinking_factor}

```gdscript
void _soft_body_set_shrinking_factor(body: RID, shrinking_factor: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_simulation_precision {#_soft_body_set_simulation_precision}

```gdscript
void _soft_body_set_simulation_precision(body: RID, simulation_precision: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_space {#_soft_body_set_space}

```gdscript
void _soft_body_set_space(body: RID, space: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_state {#_soft_body_set_state}

```gdscript
void _soft_body_set_state(body: RID, state: PhysicsServer3D.BodyState, variant: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_total_mass {#_soft_body_set_total_mass}

```gdscript
void _soft_body_set_total_mass(body: RID, total_mass: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_set_transform {#_soft_body_set_transform}

```gdscript
void _soft_body_set_transform(body: RID, transform: Transform3D) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _soft_body_update_rendering_server {#_soft_body_update_rendering_server}

```gdscript
void _soft_body_update_rendering_server(body: RID, rendering_server_handler: PhysicsServer3DRenderingServerHandler) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_create {#_space_create}

```gdscript
RID _space_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_get_contact_count {#_space_get_contact_count}

```gdscript
int _space_get_contact_count(space: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_get_contacts {#_space_get_contacts}

```gdscript
PackedVector3Array _space_get_contacts(space: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_get_direct_state {#_space_get_direct_state}

```gdscript
PhysicsDirectSpaceState3D _space_get_direct_state(space: RID) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_get_param {#_space_get_param}

```gdscript
float _space_get_param(space: RID, param: PhysicsServer3D.SpaceParameter) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_is_active {#_space_is_active}

```gdscript
bool _space_is_active(space: RID) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_set_active {#_space_set_active}

```gdscript
void _space_set_active(space: RID, active: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_set_debug_contacts {#_space_set_debug_contacts}

```gdscript
void _space_set_debug_contacts(space: RID, max_contacts: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _space_set_param {#_space_set_param}

```gdscript
void _space_set_param(space: RID, param: PhysicsServer3D.SpaceParameter, value: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _sphere_shape_create {#_sphere_shape_create}

```gdscript
RID _sphere_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _step {#_step}

```gdscript
void _step(step: float) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _sync {#_sync}

```gdscript
void _sync() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _world_boundary_shape_create {#_world_boundary_shape_create}

```gdscript
RID _world_boundary_shape_create() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### body_test_motion_is_excluding_body {#body_test_motion_is_excluding_body}

```gdscript
bool body_test_motion_is_excluding_body(body: RID) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### body_test_motion_is_excluding_object {#body_test_motion_is_excluding_object}

```gdscript
bool body_test_motion_is_excluding_object(object: int) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!
