# PhysicsServer2DExtension

**继承**：[PhysicsServer2D](class_physicsserver2d.md) → [Object](class_object.md) → **PhysicsServer2DExtension**

> This class extends PhysicsServer2D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class extends PhysicsServer2D by providing additional virtual methods that can be overridden. When these methods are overridden, they will be called instead of the internal methods of the physics server.

Intended for use with GDExtension to create custom implementations of PhysicsServer2D.

</details>

## 方法

### _area_add_shape {#_area_add_shape}

```gdscript
void _area_add_shape(area: RID, shape: RID, transform: Transform2D, disabled: bool) virtual
```

Overridable version of PhysicsServer2D.area_add_shape().

### _area_attach_canvas_instance_id {#_area_attach_canvas_instance_id}

```gdscript
void _area_attach_canvas_instance_id(area: RID, id: int) virtual
```

Overridable version of PhysicsServer2D.area_attach_canvas_instance_id().

### _area_attach_object_instance_id {#_area_attach_object_instance_id}

```gdscript
void _area_attach_object_instance_id(area: RID, id: int) virtual
```

Overridable version of PhysicsServer2D.area_attach_object_instance_id().

### _area_clear_shapes {#_area_clear_shapes}

```gdscript
void _area_clear_shapes(area: RID) virtual
```

Overridable version of PhysicsServer2D.area_clear_shapes().

### _area_create {#_area_create}

```gdscript
RID _area_create() virtual
```

Overridable version of PhysicsServer2D.area_create().

### _area_get_canvas_instance_id {#_area_get_canvas_instance_id}

```gdscript
int _area_get_canvas_instance_id(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_canvas_instance_id().

### _area_get_collision_layer {#_area_get_collision_layer}

```gdscript
int _area_get_collision_layer(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_collision_layer().

### _area_get_collision_mask {#_area_get_collision_mask}

```gdscript
int _area_get_collision_mask(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_collision_mask().

### _area_get_object_instance_id {#_area_get_object_instance_id}

```gdscript
int _area_get_object_instance_id(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_object_instance_id().

### _area_get_param {#_area_get_param}

```gdscript
Variant _area_get_param(area: RID, param: PhysicsServer2D.AreaParameter) virtual const
```

Overridable version of PhysicsServer2D.area_get_param().

### _area_get_shape {#_area_get_shape}

```gdscript
RID _area_get_shape(area: RID, shape_idx: int) virtual const
```

Overridable version of PhysicsServer2D.area_get_shape().

### _area_get_shape_count {#_area_get_shape_count}

```gdscript
int _area_get_shape_count(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_shape_count().

### _area_get_shape_transform {#_area_get_shape_transform}

```gdscript
Transform2D _area_get_shape_transform(area: RID, shape_idx: int) virtual const
```

Overridable version of PhysicsServer2D.area_get_shape_transform().

### _area_get_space {#_area_get_space}

```gdscript
RID _area_get_space(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_space().

### _area_get_transform {#_area_get_transform}

```gdscript
Transform2D _area_get_transform(area: RID) virtual const
```

Overridable version of PhysicsServer2D.area_get_transform().

### _area_remove_shape {#_area_remove_shape}

```gdscript
void _area_remove_shape(area: RID, shape_idx: int) virtual
```

Overridable version of PhysicsServer2D.area_remove_shape().

### _area_set_area_monitor_callback {#_area_set_area_monitor_callback}

```gdscript
void _area_set_area_monitor_callback(area: RID, callback: Callable) virtual
```

Overridable version of PhysicsServer2D.area_set_area_monitor_callback().

### _area_set_collision_layer {#_area_set_collision_layer}

```gdscript
void _area_set_collision_layer(area: RID, layer: int) virtual
```

Overridable version of PhysicsServer2D.area_set_collision_layer().

### _area_set_collision_mask {#_area_set_collision_mask}

```gdscript
void _area_set_collision_mask(area: RID, mask: int) virtual
```

Overridable version of PhysicsServer2D.area_set_collision_mask().

### _area_set_monitor_callback {#_area_set_monitor_callback}

```gdscript
void _area_set_monitor_callback(area: RID, callback: Callable) virtual
```

Overridable version of PhysicsServer2D.area_set_monitor_callback().

### _area_set_monitorable {#_area_set_monitorable}

```gdscript
void _area_set_monitorable(area: RID, monitorable: bool) virtual
```

Overridable version of PhysicsServer2D.area_set_monitorable().

### _area_set_param {#_area_set_param}

```gdscript
void _area_set_param(area: RID, param: PhysicsServer2D.AreaParameter, value: Variant) virtual
```

Overridable version of PhysicsServer2D.area_set_param().

### _area_set_pickable {#_area_set_pickable}

```gdscript
void _area_set_pickable(area: RID, pickable: bool) virtual
```

If set to true, allows the area with the given RID to detect mouse inputs when the mouse cursor is hovering on it.

Overridable version of PhysicsServer2D's internal area_set_pickable method. Corresponds to CollisionObject2D.input_pickable.

### _area_set_shape {#_area_set_shape}

```gdscript
void _area_set_shape(area: RID, shape_idx: int, shape: RID) virtual
```

Overridable version of PhysicsServer2D.area_set_shape().

### _area_set_shape_disabled {#_area_set_shape_disabled}

```gdscript
void _area_set_shape_disabled(area: RID, shape_idx: int, disabled: bool) virtual
```

Overridable version of PhysicsServer2D.area_set_shape_disabled().

### _area_set_shape_transform {#_area_set_shape_transform}

```gdscript
void _area_set_shape_transform(area: RID, shape_idx: int, transform: Transform2D) virtual
```

Overridable version of PhysicsServer2D.area_set_shape_transform().

### _area_set_space {#_area_set_space}

```gdscript
void _area_set_space(area: RID, space: RID) virtual
```

Overridable version of PhysicsServer2D.area_set_space().

### _area_set_transform {#_area_set_transform}

```gdscript
void _area_set_transform(area: RID, transform: Transform2D) virtual
```

Overridable version of PhysicsServer2D.area_set_transform().

### _body_add_collision_exception {#_body_add_collision_exception}

```gdscript
void _body_add_collision_exception(body: RID, excepted_body: RID) virtual
```

Overridable version of PhysicsServer2D.body_add_collision_exception().

### _body_add_constant_central_force {#_body_add_constant_central_force}

```gdscript
void _body_add_constant_central_force(body: RID, force: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_add_constant_central_force().

### _body_add_constant_force {#_body_add_constant_force}

```gdscript
void _body_add_constant_force(body: RID, force: Vector2, position: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_add_constant_force().

### _body_add_constant_torque {#_body_add_constant_torque}

```gdscript
void _body_add_constant_torque(body: RID, torque: float) virtual
```

Overridable version of PhysicsServer2D.body_add_constant_torque().

### _body_add_shape {#_body_add_shape}

```gdscript
void _body_add_shape(body: RID, shape: RID, transform: Transform2D, disabled: bool) virtual
```

Overridable version of PhysicsServer2D.body_add_shape().

### _body_apply_central_force {#_body_apply_central_force}

```gdscript
void _body_apply_central_force(body: RID, force: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_apply_central_force().

### _body_apply_central_impulse {#_body_apply_central_impulse}

```gdscript
void _body_apply_central_impulse(body: RID, impulse: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_apply_central_impulse().

### _body_apply_force {#_body_apply_force}

```gdscript
void _body_apply_force(body: RID, force: Vector2, position: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_apply_force().

### _body_apply_impulse {#_body_apply_impulse}

```gdscript
void _body_apply_impulse(body: RID, impulse: Vector2, position: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_apply_impulse().

### _body_apply_torque {#_body_apply_torque}

```gdscript
void _body_apply_torque(body: RID, torque: float) virtual
```

Overridable version of PhysicsServer2D.body_apply_torque().

### _body_apply_torque_impulse {#_body_apply_torque_impulse}

```gdscript
void _body_apply_torque_impulse(body: RID, impulse: float) virtual
```

Overridable version of PhysicsServer2D.body_apply_torque_impulse().

### _body_attach_canvas_instance_id {#_body_attach_canvas_instance_id}

```gdscript
void _body_attach_canvas_instance_id(body: RID, id: int) virtual
```

Overridable version of PhysicsServer2D.body_attach_canvas_instance_id().

### _body_attach_object_instance_id {#_body_attach_object_instance_id}

```gdscript
void _body_attach_object_instance_id(body: RID, id: int) virtual
```

Overridable version of PhysicsServer2D.body_attach_object_instance_id().

### _body_clear_shapes {#_body_clear_shapes}

```gdscript
void _body_clear_shapes(body: RID) virtual
```

Overridable version of PhysicsServer2D.body_clear_shapes().

### _body_collide_shape {#_body_collide_shape}

```gdscript
bool _body_collide_shape(body: RID, body_shape: int, shape: RID, shape_xform: Transform2D, motion: Vector2, r_results: void*, result_max: int, r_result_count: int32_t*) virtual
```

Given a body, a shape, and their respective parameters, this method should return true if a collision between the two would occur, with additional details passed in r_results.

Overridable version of PhysicsServer2D's internal shape_collide method. Corresponds to PhysicsDirectSpaceState2D.collide_shape().

### _body_create {#_body_create}

```gdscript
RID _body_create() virtual
```

Overridable version of PhysicsServer2D.body_create().

### _body_get_canvas_instance_id {#_body_get_canvas_instance_id}

```gdscript
int _body_get_canvas_instance_id(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_canvas_instance_id().

### _body_get_collision_exceptions {#_body_get_collision_exceptions}

```gdscript
typedarray::RID _body_get_collision_exceptions(body: RID) virtual const
```

Returns the RIDs of all bodies added as collision exceptions for the given body. See also _body_add_collision_exception() and _body_remove_collision_exception().

Overridable version of PhysicsServer2D's internal body_get_collision_exceptions method. Corresponds to PhysicsBody2D.get_collision_exceptions().

### _body_get_collision_layer {#_body_get_collision_layer}

```gdscript
int _body_get_collision_layer(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_collision_layer().

### _body_get_collision_mask {#_body_get_collision_mask}

```gdscript
int _body_get_collision_mask(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_collision_mask().

### _body_get_collision_priority {#_body_get_collision_priority}

```gdscript
float _body_get_collision_priority(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_collision_priority().

### _body_get_constant_force {#_body_get_constant_force}

```gdscript
Vector2 _body_get_constant_force(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_constant_force().

### _body_get_constant_torque {#_body_get_constant_torque}

```gdscript
float _body_get_constant_torque(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_constant_torque().

### _body_get_contacts_reported_depth_threshold {#_body_get_contacts_reported_depth_threshold}

```gdscript
float _body_get_contacts_reported_depth_threshold(body: RID) virtual const
```

Overridable version of PhysicsServer2D's internal body_get_contacts_reported_depth_threshold method.

Note: This method is currently unused by Godot's default physics implementation.

### _body_get_continuous_collision_detection_mode {#_body_get_continuous_collision_detection_mode}

```gdscript
PhysicsServer2D.CCDMode _body_get_continuous_collision_detection_mode(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_continuous_collision_detection_mode().

### _body_get_direct_state {#_body_get_direct_state}

```gdscript
PhysicsDirectBodyState2D _body_get_direct_state(body: RID) virtual
```

Overridable version of PhysicsServer2D.body_get_direct_state().

### _body_get_max_contacts_reported {#_body_get_max_contacts_reported}

```gdscript
int _body_get_max_contacts_reported(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_max_contacts_reported().

### _body_get_mode {#_body_get_mode}

```gdscript
PhysicsServer2D.BodyMode _body_get_mode(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_mode().

### _body_get_object_instance_id {#_body_get_object_instance_id}

```gdscript
int _body_get_object_instance_id(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_object_instance_id().

### _body_get_param {#_body_get_param}

```gdscript
Variant _body_get_param(body: RID, param: PhysicsServer2D.BodyParameter) virtual const
```

Overridable version of PhysicsServer2D.body_get_param().

### _body_get_shape {#_body_get_shape}

```gdscript
RID _body_get_shape(body: RID, shape_idx: int) virtual const
```

Overridable version of PhysicsServer2D.body_get_shape().

### _body_get_shape_count {#_body_get_shape_count}

```gdscript
int _body_get_shape_count(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_shape_count().

### _body_get_shape_transform {#_body_get_shape_transform}

```gdscript
Transform2D _body_get_shape_transform(body: RID, shape_idx: int) virtual const
```

Overridable version of PhysicsServer2D.body_get_shape_transform().

### _body_get_space {#_body_get_space}

```gdscript
RID _body_get_space(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_get_space().

### _body_get_state {#_body_get_state}

```gdscript
Variant _body_get_state(body: RID, state: PhysicsServer2D.BodyState) virtual const
```

Overridable version of PhysicsServer2D.body_get_state().

### _body_is_omitting_force_integration {#_body_is_omitting_force_integration}

```gdscript
bool _body_is_omitting_force_integration(body: RID) virtual const
```

Overridable version of PhysicsServer2D.body_is_omitting_force_integration().

### _body_remove_collision_exception {#_body_remove_collision_exception}

```gdscript
void _body_remove_collision_exception(body: RID, excepted_body: RID) virtual
```

Overridable version of PhysicsServer2D.body_remove_collision_exception().

### _body_remove_shape {#_body_remove_shape}

```gdscript
void _body_remove_shape(body: RID, shape_idx: int) virtual
```

Overridable version of PhysicsServer2D.body_remove_shape().

### _body_reset_mass_properties {#_body_reset_mass_properties}

```gdscript
void _body_reset_mass_properties(body: RID) virtual
```

Overridable version of PhysicsServer2D.body_reset_mass_properties().

### _body_set_axis_velocity {#_body_set_axis_velocity}

```gdscript
void _body_set_axis_velocity(body: RID, axis_velocity: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_set_axis_velocity().

### _body_set_collision_layer {#_body_set_collision_layer}

```gdscript
void _body_set_collision_layer(body: RID, layer: int) virtual
```

Overridable version of PhysicsServer2D.body_set_collision_layer().

### _body_set_collision_mask {#_body_set_collision_mask}

```gdscript
void _body_set_collision_mask(body: RID, mask: int) virtual
```

Overridable version of PhysicsServer2D.body_set_collision_mask().

### _body_set_collision_priority {#_body_set_collision_priority}

```gdscript
void _body_set_collision_priority(body: RID, priority: float) virtual
```

Overridable version of PhysicsServer2D.body_set_collision_priority().

### _body_set_constant_force {#_body_set_constant_force}

```gdscript
void _body_set_constant_force(body: RID, force: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_set_constant_force().

### _body_set_constant_torque {#_body_set_constant_torque}

```gdscript
void _body_set_constant_torque(body: RID, torque: float) virtual
```

Overridable version of PhysicsServer2D.body_set_constant_torque().

### _body_set_contacts_reported_depth_threshold {#_body_set_contacts_reported_depth_threshold}

```gdscript
void _body_set_contacts_reported_depth_threshold(body: RID, threshold: float) virtual
```

Overridable version of PhysicsServer2D's internal body_set_contacts_reported_depth_threshold method.

Note: This method is currently unused by Godot's default physics implementation.

### _body_set_continuous_collision_detection_mode {#_body_set_continuous_collision_detection_mode}

```gdscript
void _body_set_continuous_collision_detection_mode(body: RID, mode: PhysicsServer2D.CCDMode) virtual
```

Overridable version of PhysicsServer2D.body_set_continuous_collision_detection_mode().

### _body_set_force_integration_callback {#_body_set_force_integration_callback}

```gdscript
void _body_set_force_integration_callback(body: RID, callable: Callable, userdata: Variant) virtual
```

Overridable version of PhysicsServer2D.body_set_force_integration_callback().

### _body_set_max_contacts_reported {#_body_set_max_contacts_reported}

```gdscript
void _body_set_max_contacts_reported(body: RID, amount: int) virtual
```

Overridable version of PhysicsServer2D.body_set_max_contacts_reported().

### _body_set_mode {#_body_set_mode}

```gdscript
void _body_set_mode(body: RID, mode: PhysicsServer2D.BodyMode) virtual
```

Overridable version of PhysicsServer2D.body_set_mode().

### _body_set_omit_force_integration {#_body_set_omit_force_integration}

```gdscript
void _body_set_omit_force_integration(body: RID, enable: bool) virtual
```

Overridable version of PhysicsServer2D.body_set_omit_force_integration().

### _body_set_param {#_body_set_param}

```gdscript
void _body_set_param(body: RID, param: PhysicsServer2D.BodyParameter, value: Variant) virtual
```

Overridable version of PhysicsServer2D.body_set_param().

### _body_set_pickable {#_body_set_pickable}

```gdscript
void _body_set_pickable(body: RID, pickable: bool) virtual
```

If set to true, allows the body with the given RID to detect mouse inputs when the mouse cursor is hovering on it.

Overridable version of PhysicsServer2D's internal body_set_pickable method. Corresponds to CollisionObject2D.input_pickable.

### _body_set_shape {#_body_set_shape}

```gdscript
void _body_set_shape(body: RID, shape_idx: int, shape: RID) virtual
```

Overridable version of PhysicsServer2D.body_set_shape().

### _body_set_shape_as_one_way_collision {#_body_set_shape_as_one_way_collision}

```gdscript
void _body_set_shape_as_one_way_collision(body: RID, shape_idx: int, enable: bool, margin: float, direction: Vector2) virtual
```

Overridable version of PhysicsServer2D.body_set_shape_as_one_way_collision().

### _body_set_shape_disabled {#_body_set_shape_disabled}

```gdscript
void _body_set_shape_disabled(body: RID, shape_idx: int, disabled: bool) virtual
```

Overridable version of PhysicsServer2D.body_set_shape_disabled().

### _body_set_shape_transform {#_body_set_shape_transform}

```gdscript
void _body_set_shape_transform(body: RID, shape_idx: int, transform: Transform2D) virtual
```

Overridable version of PhysicsServer2D.body_set_shape_transform().

### _body_set_space {#_body_set_space}

```gdscript
void _body_set_space(body: RID, space: RID) virtual
```

Overridable version of PhysicsServer2D.body_set_space().

### _body_set_state {#_body_set_state}

```gdscript
void _body_set_state(body: RID, state: PhysicsServer2D.BodyState, value: Variant) virtual
```

Overridable version of PhysicsServer2D.body_set_state().

### _body_set_state_sync_callback {#_body_set_state_sync_callback}

```gdscript
void _body_set_state_sync_callback(body: RID, callable: Callable) virtual
```

Assigns the body to call the given callable during the synchronization phase of the loop, before _step() is called. See also _sync().

Overridable version of PhysicsServer2D.body_set_state_sync_callback().

### _body_test_motion {#_body_test_motion}

```gdscript
bool _body_test_motion(body: RID, from: Transform2D, motion: Vector2, margin: float, collide_separation_ray: bool, recovery_as_collision: bool, r_result: PhysicsServer2DExtensionMotionResult*) virtual const
```

Overridable version of PhysicsServer2D.body_test_motion(). Unlike the exposed implementation, this method does not receive all of the arguments inside a PhysicsTestMotionParameters2D.

### _capsule_shape_create {#_capsule_shape_create}

```gdscript
RID _capsule_shape_create() virtual
```

Overridable version of PhysicsServer2D.capsule_shape_create().

### _circle_shape_create {#_circle_shape_create}

```gdscript
RID _circle_shape_create() virtual
```

Overridable version of PhysicsServer2D.circle_shape_create().

### _concave_polygon_shape_create {#_concave_polygon_shape_create}

```gdscript
RID _concave_polygon_shape_create() virtual
```

Overridable version of PhysicsServer2D.concave_polygon_shape_create().

### _convex_polygon_shape_create {#_convex_polygon_shape_create}

```gdscript
RID _convex_polygon_shape_create() virtual
```

Overridable version of PhysicsServer2D.convex_polygon_shape_create().

### _damped_spring_joint_get_param {#_damped_spring_joint_get_param}

```gdscript
float _damped_spring_joint_get_param(joint: RID, param: PhysicsServer2D.DampedSpringParam) virtual const
```

Overridable version of PhysicsServer2D.damped_spring_joint_get_param().

### _damped_spring_joint_set_param {#_damped_spring_joint_set_param}

```gdscript
void _damped_spring_joint_set_param(joint: RID, param: PhysicsServer2D.DampedSpringParam, value: float) virtual
```

Overridable version of PhysicsServer2D.damped_spring_joint_set_param().

### _end_sync {#_end_sync}

```gdscript
void _end_sync() virtual
```

Called to indicate that the physics server has stopped synchronizing. It is in the loop's iteration/physics phase, and can access physics objects even if running on a separate thread. See also _sync().

Overridable version of PhysicsServer2D's internal end_sync method.

### _finish {#_finish}

```gdscript
void _finish() virtual
```

Called when the main loop finalizes to shut down the physics server. See also MainLoop._finalize() and _init().

Overridable version of PhysicsServer2D's internal finish method.

### _flush_queries {#_flush_queries}

```gdscript
void _flush_queries() virtual
```

Called every physics step before _step() to process all remaining queries.

Overridable version of PhysicsServer2D's internal flush_queries method.

### _free_rid {#_free_rid}

```gdscript
void _free_rid(rid: RID) virtual
```

Overridable version of PhysicsServer2D.free_rid().

### _get_process_info {#_get_process_info}

```gdscript
int _get_process_info(process_info: PhysicsServer2D.ProcessInfo) virtual
```

Overridable version of PhysicsServer2D.get_process_info().

### _init {#_init}

```gdscript
void _init() virtual
```

Called when the main loop is initialized and creates a new instance of this physics server. See also MainLoop._initialize() and _finish().

Overridable version of PhysicsServer2D's internal init method.

### _is_flushing_queries {#_is_flushing_queries}

```gdscript
bool _is_flushing_queries() virtual const
```

Overridable method that should return true when the physics server is processing queries. See also _flush_queries().

Overridable version of PhysicsServer2D's internal is_flushing_queries method.

### _joint_clear {#_joint_clear}

```gdscript
void _joint_clear(joint: RID) virtual
```

Overridable version of PhysicsServer2D.joint_clear().

### _joint_create {#_joint_create}

```gdscript
RID _joint_create() virtual
```

Overridable version of PhysicsServer2D.joint_create().

### _joint_disable_collisions_between_bodies {#_joint_disable_collisions_between_bodies}

```gdscript
void _joint_disable_collisions_between_bodies(joint: RID, disable: bool) virtual
```

Overridable version of PhysicsServer2D.joint_disable_collisions_between_bodies().

### _joint_get_param {#_joint_get_param}

```gdscript
float _joint_get_param(joint: RID, param: PhysicsServer2D.JointParam) virtual const
```

Overridable version of PhysicsServer2D.joint_get_param().

### _joint_get_type {#_joint_get_type}

```gdscript
PhysicsServer2D.JointType _joint_get_type(joint: RID) virtual const
```

Overridable version of PhysicsServer2D.joint_get_type().

### _joint_is_disabled_collisions_between_bodies {#_joint_is_disabled_collisions_between_bodies}

```gdscript
bool _joint_is_disabled_collisions_between_bodies(joint: RID) virtual const
```

Overridable version of PhysicsServer2D.joint_is_disabled_collisions_between_bodies().

### _joint_make_damped_spring {#_joint_make_damped_spring}

```gdscript
void _joint_make_damped_spring(joint: RID, anchor_a: Vector2, anchor_b: Vector2, body_a: RID, body_b: RID) virtual
```

Overridable version of PhysicsServer2D.joint_make_damped_spring().

### _joint_make_groove {#_joint_make_groove}

```gdscript
void _joint_make_groove(joint: RID, a_groove1: Vector2, a_groove2: Vector2, b_anchor: Vector2, body_a: RID, body_b: RID) virtual
```

Overridable version of PhysicsServer2D.joint_make_groove().

### _joint_make_pin {#_joint_make_pin}

```gdscript
void _joint_make_pin(joint: RID, anchor: Vector2, body_a: RID, body_b: RID) virtual
```

Overridable version of PhysicsServer2D.joint_make_pin().

### _joint_set_param {#_joint_set_param}

```gdscript
void _joint_set_param(joint: RID, param: PhysicsServer2D.JointParam, value: float) virtual
```

Overridable version of PhysicsServer2D.joint_set_param().

### _pin_joint_get_flag {#_pin_joint_get_flag}

```gdscript
bool _pin_joint_get_flag(joint: RID, flag: PhysicsServer2D.PinJointFlag) virtual const
```

Overridable version of PhysicsServer2D.pin_joint_get_flag().

### _pin_joint_get_param {#_pin_joint_get_param}

```gdscript
float _pin_joint_get_param(joint: RID, param: PhysicsServer2D.PinJointParam) virtual const
```

Overridable version of PhysicsServer2D.pin_joint_get_param().

### _pin_joint_set_flag {#_pin_joint_set_flag}

```gdscript
void _pin_joint_set_flag(joint: RID, flag: PhysicsServer2D.PinJointFlag, enabled: bool) virtual
```

Overridable version of PhysicsServer2D.pin_joint_set_flag().

### _pin_joint_set_param {#_pin_joint_set_param}

```gdscript
void _pin_joint_set_param(joint: RID, param: PhysicsServer2D.PinJointParam, value: float) virtual
```

Overridable version of PhysicsServer2D.pin_joint_set_param().

### _rectangle_shape_create {#_rectangle_shape_create}

```gdscript
RID _rectangle_shape_create() virtual
```

Overridable version of PhysicsServer2D.rectangle_shape_create().

### _segment_shape_create {#_segment_shape_create}

```gdscript
RID _segment_shape_create() virtual
```

Overridable version of PhysicsServer2D.segment_shape_create().

### _separation_ray_shape_create {#_separation_ray_shape_create}

```gdscript
RID _separation_ray_shape_create() virtual
```

Overridable version of PhysicsServer2D.separation_ray_shape_create().

### _set_active {#_set_active}

```gdscript
void _set_active(active: bool) virtual
```

Overridable version of PhysicsServer2D.set_active().

### _shape_collide {#_shape_collide}

```gdscript
bool _shape_collide(shape_A: RID, xform_A: Transform2D, motion_A: Vector2, shape_B: RID, xform_B: Transform2D, motion_B: Vector2, r_results: void*, result_max: int, r_result_count: int32_t*) virtual
```

Given two shapes and their parameters, should return true if a collision between the two would occur, with additional details passed in r_results.

Overridable version of PhysicsServer2D's internal shape_collide method. Corresponds to PhysicsDirectSpaceState2D.collide_shape().

### _shape_get_custom_solver_bias {#_shape_get_custom_solver_bias}

```gdscript
float _shape_get_custom_solver_bias(shape: RID) virtual const
```

Should return the custom solver bias of the given shape, which defines how much bodies are forced to separate on contact when this shape is involved.

Overridable version of PhysicsServer2D's internal shape_get_custom_solver_bias method. Corresponds to Shape2D.custom_solver_bias.

### _shape_get_data {#_shape_get_data}

```gdscript
Variant _shape_get_data(shape: RID) virtual const
```

Overridable version of PhysicsServer2D.shape_get_data().

### _shape_get_type {#_shape_get_type}

```gdscript
PhysicsServer2D.ShapeType _shape_get_type(shape: RID) virtual const
```

Overridable version of PhysicsServer2D.shape_get_type().

### _shape_set_custom_solver_bias {#_shape_set_custom_solver_bias}

```gdscript
void _shape_set_custom_solver_bias(shape: RID, bias: float) virtual
```

Should set the custom solver bias for the given shape. It defines how much bodies are forced to separate on contact.

Overridable version of PhysicsServer2D's internal shape_get_custom_solver_bias method. Corresponds to Shape2D.custom_solver_bias.

### _shape_set_data {#_shape_set_data}

```gdscript
void _shape_set_data(shape: RID, data: Variant) virtual
```

Overridable version of PhysicsServer2D.shape_set_data().

### _space_create {#_space_create}

```gdscript
RID _space_create() virtual
```

Overridable version of PhysicsServer2D.space_create().

### _space_get_contact_count {#_space_get_contact_count}

```gdscript
int _space_get_contact_count(space: RID) virtual const
```

Should return how many contacts have occurred during the last physics step in the given space. See also _space_get_contacts() and _space_set_debug_contacts().

Overridable version of PhysicsServer2D's internal space_get_contact_count method.

### _space_get_contacts {#_space_get_contacts}

```gdscript
PackedVector2Array _space_get_contacts(space: RID) virtual const
```

Should return the positions of all contacts that have occurred during the last physics step in the given space. See also _space_get_contact_count() and _space_set_debug_contacts().

Overridable version of PhysicsServer2D's internal space_get_contacts method.

### _space_get_direct_state {#_space_get_direct_state}

```gdscript
PhysicsDirectSpaceState2D _space_get_direct_state(space: RID) virtual
```

Overridable version of PhysicsServer2D.space_get_direct_state().

### _space_get_param {#_space_get_param}

```gdscript
float _space_get_param(space: RID, param: PhysicsServer2D.SpaceParameter) virtual const
```

Overridable version of PhysicsServer2D.space_get_param().

### _space_is_active {#_space_is_active}

```gdscript
bool _space_is_active(space: RID) virtual const
```

Overridable version of PhysicsServer2D.space_is_active().

### _space_set_active {#_space_set_active}

```gdscript
void _space_set_active(space: RID, active: bool) virtual
```

Overridable version of PhysicsServer2D.space_set_active().

### _space_set_debug_contacts {#_space_set_debug_contacts}

```gdscript
void _space_set_debug_contacts(space: RID, max_contacts: int) virtual
```

Used internally to allow the given space to store contact points, up to max_contacts. This is automatically set for the main World2D's space when SceneTree.debug_collisions_hint is true, or by checking "Visible Collision Shapes" in the editor. Only works in debug builds.

Overridable version of PhysicsServer2D's internal space_set_debug_contacts method.

### _space_set_param {#_space_set_param}

```gdscript
void _space_set_param(space: RID, param: PhysicsServer2D.SpaceParameter, value: float) virtual
```

Overridable version of PhysicsServer2D.space_set_param().

### _step {#_step}

```gdscript
void _step(step: float) virtual
```

Called every physics step to process the physics simulation. step is the time elapsed since the last physics step, in seconds. It is usually the same as the value returned by Node.get_physics_process_delta_time().

Overridable version of PhysicsServer2D's internal step method.

### _sync {#_sync}

```gdscript
void _sync() virtual
```

Called to indicate that the physics server is synchronizing and cannot access physics states if running on a separate thread. See also _end_sync().

Overridable version of PhysicsServer2D's internal sync method.

### _world_boundary_shape_create {#_world_boundary_shape_create}

```gdscript
RID _world_boundary_shape_create() virtual
```

Overridable version of PhysicsServer2D.world_boundary_shape_create().

### body_test_motion_is_excluding_body {#body_test_motion_is_excluding_body}

```gdscript
bool body_test_motion_is_excluding_body(body: RID) const
```

Returns true if the body with the given RID is being excluded from _body_test_motion(). See also Object.get_instance_id().

### body_test_motion_is_excluding_object {#body_test_motion_is_excluding_object}

```gdscript
bool body_test_motion_is_excluding_object(object: int) const
```

Returns true if the object with the given instance ID is being excluded from _body_test_motion(). See also Object.get_instance_id().
