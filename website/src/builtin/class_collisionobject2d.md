# CollisionObject2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **CollisionObject2D**

> Abstract base class for 2D physics objects. CollisionObject2D can hold any number of Shape2Ds for collision. Each shape must be assigned to a shape owner. Shape owners are not nodes and do not appear in the editor, but are accessible through code using the shape_owner_* methods.

<details><summary>官方英文描述</summary>

Abstract base class for 2D physics objects. CollisionObject2D can hold any number of Shape2Ds for collision. Each shape must be assigned to a shape owner. Shape owners are not nodes and do not appear in the editor, but are accessible through code using the shape_owner_* methods.

Note: Only collisions between objects within the same canvas (Viewport canvas or CanvasLayer) are supported. The behavior of collisions between objects in different canvases is undefined.

</details>

## 方法

### _input_event {#_input_event}

```gdscript
void _input_event(viewport: Viewport, event: InputEvent, shape_idx: int) virtual
```

Accepts unhandled InputEvents. shape_idx is the child index of the clicked Shape2D. Connect to input_event to easily pick up these events.

Note: _input_event() requires input_pickable to be true and at least one collision_layer bit to be set.

### _mouse_enter {#_mouse_enter}

```gdscript
void _mouse_enter() virtual
```

Called when the mouse pointer enters any of this object's shapes. Requires input_pickable to be true and at least one collision_layer bit to be set. Note that moving between different shapes within a single CollisionObject2D won't cause this function to be called.

### _mouse_exit {#_mouse_exit}

```gdscript
void _mouse_exit() virtual
```

Called when the mouse pointer exits all this object's shapes. Requires input_pickable to be true and at least one collision_layer bit to be set. Note that moving between different shapes within a single CollisionObject2D won't cause this function to be called.

### _mouse_shape_enter {#_mouse_shape_enter}

```gdscript
void _mouse_shape_enter(shape_idx: int) virtual
```

Called when the mouse pointer enters any of this object's shapes or moves from one shape to another. shape_idx is the child index of the newly entered Shape2D. Requires input_pickable to be true and at least one collision_layer bit to be called.

### _mouse_shape_exit {#_mouse_shape_exit}

```gdscript
void _mouse_shape_exit(shape_idx: int) virtual
```

Called when the mouse pointer exits any of this object's shapes. shape_idx is the child index of the exited Shape2D. Requires input_pickable to be true and at least one collision_layer bit to be called.

### create_shape_owner {#create_shape_owner}

```gdscript
int create_shape_owner(owner: Object)
```

Creates a new shape owner for the given object. Returns owner_id of the new owner for future reference.

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

### get_collision_priority {#get_collision_priority}

```gdscript
float get_collision_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_disable_mode {#get_disable_mode}

```gdscript
CollisionObject2D.DisableMode get_disable_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the object's RID.

### get_shape_owner_one_way_collision_direction {#get_shape_owner_one_way_collision_direction}

```gdscript
Vector2 get_shape_owner_one_way_collision_direction(owner_id: int) const
```

Returns the one_way_collision_direction of the shape owner identified by the given owner_id.

### get_shape_owner_one_way_collision_margin {#get_shape_owner_one_way_collision_margin}

```gdscript
float get_shape_owner_one_way_collision_margin(owner_id: int) const
```

Returns the one_way_collision_margin of the shape owner identified by given owner_id.

### get_shape_owners {#get_shape_owners}

```gdscript
PackedInt32Array get_shape_owners()
```

Returns an Array of owner_id identifiers. You can use these ids in other methods that take owner_id as an argument.

### is_pickable {#is_pickable}

```gdscript
bool is_pickable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shape_owner_disabled {#is_shape_owner_disabled}

```gdscript
bool is_shape_owner_disabled(owner_id: int) const
```

If true, the shape owner and its shapes are disabled.

### is_shape_owner_one_way_collision_enabled {#is_shape_owner_one_way_collision_enabled}

```gdscript
bool is_shape_owner_one_way_collision_enabled(owner_id: int) const
```

Returns true if collisions for the shape owner originating from this CollisionObject2D will not be reported to collided with CollisionObject2Ds.

### remove_shape_owner {#remove_shape_owner}

```gdscript
void remove_shape_owner(owner_id: int)
```

Removes the given shape owner.

### set_collision_layer {#set_collision_layer}

```gdscript
void set_collision_layer(layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_layer_value {#set_collision_layer_value}

```gdscript
void set_collision_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_layer, given a layer_number between 1 and 32.

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask_value {#set_collision_mask_value}

```gdscript
void set_collision_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_mask, given a layer_number between 1 and 32.

### set_collision_priority {#set_collision_priority}

```gdscript
void set_collision_priority(priority: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_mode {#set_disable_mode}

```gdscript
void set_disable_mode(mode: CollisionObject2D.DisableMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pickable {#set_pickable}

```gdscript
void set_pickable(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### shape_find_owner {#shape_find_owner}

```gdscript
int shape_find_owner(shape_index: int) const
```

Returns the owner_id of the given shape.

### shape_owner_add_shape {#shape_owner_add_shape}

```gdscript
void shape_owner_add_shape(owner_id: int, shape: Shape2D)
```

Adds a Shape2D to the shape owner.

### shape_owner_clear_shapes {#shape_owner_clear_shapes}

```gdscript
void shape_owner_clear_shapes(owner_id: int)
```

Removes all shapes from the shape owner.

### shape_owner_get_owner {#shape_owner_get_owner}

```gdscript
Object shape_owner_get_owner(owner_id: int) const
```

Returns the parent object of the given shape owner.

### shape_owner_get_shape {#shape_owner_get_shape}

```gdscript
Shape2D shape_owner_get_shape(owner_id: int, shape_id: int) const
```

Returns the Shape2D with the given ID from the given shape owner.

### shape_owner_get_shape_count {#shape_owner_get_shape_count}

```gdscript
int shape_owner_get_shape_count(owner_id: int) const
```

Returns the number of shapes the given shape owner contains.

### shape_owner_get_shape_index {#shape_owner_get_shape_index}

```gdscript
int shape_owner_get_shape_index(owner_id: int, shape_id: int) const
```

Returns the child index of the Shape2D with the given ID from the given shape owner.

### shape_owner_get_transform {#shape_owner_get_transform}

```gdscript
Transform2D shape_owner_get_transform(owner_id: int) const
```

Returns the shape owner's Transform2D.

### shape_owner_remove_shape {#shape_owner_remove_shape}

```gdscript
void shape_owner_remove_shape(owner_id: int, shape_id: int)
```

Removes a shape from the given shape owner.

### shape_owner_set_disabled {#shape_owner_set_disabled}

```gdscript
void shape_owner_set_disabled(owner_id: int, disabled: bool)
```

If true, disables the given shape owner.

### shape_owner_set_one_way_collision {#shape_owner_set_one_way_collision}

```gdscript
void shape_owner_set_one_way_collision(owner_id: int, enable: bool)
```

If enable is true, collisions for the shape owner originating from this CollisionObject2D will not be reported to collided with CollisionObject2Ds.

### shape_owner_set_one_way_collision_direction {#shape_owner_set_one_way_collision_direction}

```gdscript
void shape_owner_set_one_way_collision_direction(owner_id: int, direction: Vector2)
```

Sets the one_way_collision_direction of the shape owner identified by the given owner_id to direction.

### shape_owner_set_one_way_collision_margin {#shape_owner_set_one_way_collision_margin}

```gdscript
void shape_owner_set_one_way_collision_margin(owner_id: int, margin: float)
```

Sets the one_way_collision_margin of the shape owner identified by given owner_id to margin pixels.

### shape_owner_set_transform {#shape_owner_set_transform}

```gdscript
void shape_owner_set_transform(owner_id: int, transform: Transform2D)
```

Sets the Transform2D of the given shape owner.

## 属性

### disable_mode {#disable_mode}

```gdscript
disable_mode: int
```

- void set_disable_mode(value: DisableMode)
- DisableMode get_disable_mode()

Defines the behavior in physics when Node.process_mode is set to Node.PROCESS_MODE_DISABLED.

### collision_layer {#collision_layer}

```gdscript
collision_layer: int
```

- void set_collision_layer(value: int)
- int get_collision_layer()

The physics layers this CollisionObject2D is in. Collision objects can exist in one or more of 32 different layers. See also collision_mask.

Note: Object A can detect a contact with object B only if object B is in any of the layers that object A scans. See Collision layers and masks in the documentation for more information.

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The physics layers this CollisionObject2D scans. Collision objects can scan one or more of 32 different layers. See also collision_layer.

Note: Object A can detect a contact with object B only if object B is in any of the layers that object A scans. See Collision layers and masks in the documentation for more information.

### collision_priority {#collision_priority}

```gdscript
collision_priority: float
```

- void set_collision_priority(value: float)
- float get_collision_priority()

The priority used to solve colliding when occurring penetration. The higher the priority is, the lower the penetration into the object will be. This can for example be used to prevent the player from breaking through the boundaries of a level.

### input_pickable {#input_pickable}

```gdscript
input_pickable: bool
```

- void set_pickable(value: bool)
- bool is_pickable()

If true, this object is pickable. A pickable object can detect the mouse pointer entering/leaving, and if the mouse is inside it, report input events. Requires at least one collision_layer bit to be set.

## 信号

### input_event {#input_event}

```gdscript
signal input_event(viewport: Node, event: InputEvent, shape_idx: int)
```

Emitted when an input event occurs. Requires input_pickable to be true and at least one collision_layer bit to be set. See _input_event() for details.

### mouse_entered {#mouse_entered}

```gdscript
signal mouse_entered()
```

Emitted when the mouse pointer enters any of this object's shapes. Requires input_pickable to be true and at least one collision_layer bit to be set. Note that moving between different shapes within a single CollisionObject2D won't cause this signal to be emitted.

Note: Due to the lack of continuous collision detection, this signal may not be emitted in the expected order if the mouse moves fast enough and the CollisionObject2D's area is small. This signal may also not be emitted if another CollisionObject2D is overlapping the CollisionObject2D in question.

### mouse_exited {#mouse_exited}

```gdscript
signal mouse_exited()
```

Emitted when the mouse pointer exits all this object's shapes. Requires input_pickable to be true and at least one collision_layer bit to be set. Note that moving between different shapes within a single CollisionObject2D won't cause this signal to be emitted.

Note: Due to the lack of continuous collision detection, this signal may not be emitted in the expected order if the mouse moves fast enough and the CollisionObject2D's area is small. This signal may also not be emitted if another CollisionObject2D is overlapping the CollisionObject2D in question.

### mouse_shape_entered {#mouse_shape_entered}

```gdscript
signal mouse_shape_entered(shape_idx: int)
```

Emitted when the mouse pointer enters any of this object's shapes or moves from one shape to another. shape_idx is the child index of the newly entered Shape2D. Requires input_pickable to be true and at least one collision_layer bit to be set.

### mouse_shape_exited {#mouse_shape_exited}

```gdscript
signal mouse_shape_exited(shape_idx: int)
```

Emitted when the mouse pointer exits any of this object's shapes. shape_idx is the child index of the exited Shape2D. Requires input_pickable to be true and at least one collision_layer bit to be set.

## 枚举

### DisableMode {#enum-DisableMode}

```gdscript
enum DisableMode { DISABLE_MODE_REMOVE = 0, DISABLE_MODE_MAKE_STATIC = 1, DISABLE_MODE_KEEP_ACTIVE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DISABLE_MODE_REMOVE {#DISABLE_MODE_REMOVE}

```gdscript
DISABLE_MODE_REMOVE = 0
```

When Node.process_mode is set to Node.PROCESS_MODE_DISABLED, remove from the physics simulation to stop all physics interactions with this CollisionObject2D.

Automatically re-added to the physics simulation when the Node is processed again.

#### DISABLE_MODE_MAKE_STATIC {#DISABLE_MODE_MAKE_STATIC}

```gdscript
DISABLE_MODE_MAKE_STATIC = 1
```

When Node.process_mode is set to Node.PROCESS_MODE_DISABLED, make the body static. Doesn't affect Area2D. PhysicsBody2D can't be affected by forces or other bodies while static.

Automatically set PhysicsBody2D back to its original mode when the Node is processed again.

#### DISABLE_MODE_KEEP_ACTIVE {#DISABLE_MODE_KEEP_ACTIVE}

```gdscript
DISABLE_MODE_KEEP_ACTIVE = 2
```

When Node.process_mode is set to Node.PROCESS_MODE_DISABLED, do not affect the physics simulation.

---
