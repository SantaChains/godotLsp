# Node3D

**继承**：[Node](class_node.md) → [Object](class_object.md) → **Node3D**

> The Node3D node is the base representation of a node in 3D space. All other 3D nodes inherit from this class.

<details><summary>官方英文描述（中文精译进行中）</summary>

The Node3D node is the base representation of a node in 3D space. All other 3D nodes inherit from this class.

Affine operations (translation, rotation, scale) are calculated in the coordinate system relative to the parent, unless the Node3D's top_level is true. In this coordinate system, affine operations correspond to direct affine operations on the Node3D's transform. The term parent space refers to this coordinate system. The coordinate system that is attached to the Node3D itself is referred to as object-local coordinate system, or local space.

Note: Unless otherwise specified, all methods that need angle parameters must receive angles in radians. To convert degrees to radians, use @GlobalScope.deg_to_rad().

Note: In Godot 3 and older, Node3D was named Spatial.

</details>

## 方法

### add_gizmo {#add_gizmo}

```gdscript
void add_gizmo(gizmo: Node3DGizmo)
```

Attaches the given gizmo to this node. Only works in the editor.

Note: gizmo should be an EditorNode3DGizmo. The argument type is Node3DGizmo to avoid depending on editor classes in Node3D.

### clear_gizmos {#clear_gizmos}

```gdscript
void clear_gizmos()
```

Clears all EditorNode3DGizmo objects attached to this node. Only works in the editor.

### clear_subgizmo_selection {#clear_subgizmo_selection}

```gdscript
void clear_subgizmo_selection()
```

Deselects all subgizmos for this node. Useful to call when the selected subgizmo may no longer exist after a property change. Only works in the editor.

### force_update_transform {#force_update_transform}

```gdscript
void force_update_transform()
```

Forces the node's global_transform to update, by sending NOTIFICATION_TRANSFORM_CHANGED. Fails if the node is not inside the tree.

Note: For performance reasons, transform changes are usually accumulated and applied once at the end of the frame. The update propagates through Node3D children, as well. Therefore, use this method only when you need an up-to-date transform (such as during physics operations).

### get_basis {#get_basis}

```gdscript
Basis get_basis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gizmos {#get_gizmos}

```gdscript
typedarray::Node3DGizmo get_gizmos() const
```

Returns all the EditorNode3DGizmo objects attached to this node. Only works in the editor.

### get_global_basis {#get_global_basis}

```gdscript
Basis get_global_basis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_position {#get_global_position}

```gdscript
Vector3 get_global_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_rotation {#get_global_rotation}

```gdscript
Vector3 get_global_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_rotation_degrees {#get_global_rotation_degrees}

```gdscript
Vector3 get_global_rotation_degrees() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_transform {#get_global_transform}

```gdscript
Transform3D get_global_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_transform_interpolated {#get_global_transform_interpolated}

```gdscript
Transform3D get_global_transform_interpolated()
```

When using physics interpolation, there will be circumstances in which you want to know the interpolated (displayed) transform of a node rather than the standard transform (which may only be accurate to the most recent physics tick).

This is particularly important for frame-based operations that take place in Node._process(), rather than Node._physics_process(). Examples include Camera3Ds focusing on a node, or finding where to fire lasers from on a frame rather than physics tick.

Note: This function creates an interpolation pump on the Node3D the first time it is called, which can respond to physics interpolation resets. If you get problems with "streaking" when initially following a Node3D, be sure to call get_global_transform_interpolated() at least once before resetting the Node3D physics interpolation.

### get_parent_node_3d {#get_parent_node_3d}

```gdscript
Node3D get_parent_node_3d() const
```

Returns the parent Node3D that directly affects this node's global_transform. Returns null if no parent exists, the parent is not a Node3D, or top_level is true.

Note: This method is not always equivalent to Node.get_parent(), which does not take top_level into account.

### get_position {#get_position}

```gdscript
Vector3 get_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_quaternion {#get_quaternion}

```gdscript
Quaternion get_quaternion() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation {#get_rotation}

```gdscript
Vector3 get_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_degrees {#get_rotation_degrees}

```gdscript
Vector3 get_rotation_degrees() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_edit_mode {#get_rotation_edit_mode}

```gdscript
Node3D.RotationEditMode get_rotation_edit_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_order {#get_rotation_order}

```gdscript
EulerOrder get_rotation_order() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale {#get_scale}

```gdscript
Vector3 get_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform {#get_transform}

```gdscript
Transform3D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visibility_parent {#get_visibility_parent}

```gdscript
NodePath get_visibility_parent() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_world_3d {#get_world_3d}

```gdscript
World3D get_world_3d() const
```

Returns the World3D this node is registered to.

Usually, this is the same as the world used by this node's viewport (see Node.get_viewport() and Viewport.find_world_3d()).

### global_rotate {#global_rotate}

```gdscript
void global_rotate(axis: Vector3, angle: float)
```

Rotates this node's global_basis around the global axis by the given angle, in radians. This operation is calculated in global space (relative to the world) and preserves the global_position.

### global_scale {#global_scale}

```gdscript
void global_scale(scale: Vector3)
```

Scales this node's global_basis by the given scale factor. This operation is calculated in global space (relative to the world) and preserves the global_position.

Note: This method is not to be confused with the scale property.

### global_translate {#global_translate}

```gdscript
void global_translate(offset: Vector3)
```

Adds the given translation offset to the node's global_position in global space (relative to the world).

### hide {#hide}

```gdscript
void hide()
```

Prevents this node from being rendered. Equivalent to setting visible to false. This is the opposite of show().

### is_local_transform_notification_enabled {#is_local_transform_notification_enabled}

```gdscript
bool is_local_transform_notification_enabled() const
```

Returns true if the node receives NOTIFICATION_LOCAL_TRANSFORM_CHANGED whenever transform changes. This is enabled with set_notify_local_transform().

### is_scale_disabled {#is_scale_disabled}

```gdscript
bool is_scale_disabled() const
```

Returns true if this node's global_transform is automatically orthonormalized. This results in this node not appearing distorted, as if its global scale were set to Vector3.ONE (or its negative counterpart). See also set_disable_scale() and orthonormalize().

Note: transform is not affected by this setting.

### is_set_as_top_level {#is_set_as_top_level}

```gdscript
bool is_set_as_top_level() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_transform_notification_enabled {#is_transform_notification_enabled}

```gdscript
bool is_transform_notification_enabled() const
```

Returns true if the node receives NOTIFICATION_TRANSFORM_CHANGED whenever global_transform changes. This is enabled with set_notify_transform().

### is_visible {#is_visible}

```gdscript
bool is_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_visible_in_tree {#is_visible_in_tree}

```gdscript
bool is_visible_in_tree() const
```

Returns true if this node is inside the scene tree and the visible property is true for this node and all of its Node3D ancestors in sequence. An ancestor of any other type (such as Node or Node2D) breaks the sequence. See also Node.get_parent().

Note: This method cannot take VisualInstance3D.layers into account, so even if this method returns true, the node may not be rendered.

### look_at {#look_at}

```gdscript
void look_at(target: Vector3, up: Vector3 = Vector3(0, 1, 0), use_model_front: bool = false)
```

Rotates the node so that the local forward axis (-Z, Vector3.FORWARD) points toward the target position. This operation is calculated in global space (relative to the world).

The local up axis (+Y) points as close to the up vector as possible while staying perpendicular to the local forward axis. The resulting transform is orthogonal, and the scale is preserved. Non-uniform scaling may not work correctly.

The target position cannot be the same as the node's position, the up vector cannot be Vector3.ZERO. Furthermore, the direction from the node's position to the target position cannot be parallel to the up vector, to avoid an unintended rotation around the local Z axis.

If use_model_front is true, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the target position. By default, the -Z axis (camera forward) is treated as forward (implies +X is right).

Note: This method fails if the node is not in the scene tree. If necessary, use look_at_from_position() instead.

### look_at_from_position {#look_at_from_position}

```gdscript
void look_at_from_position(position: Vector3, target: Vector3, up: Vector3 = Vector3(0, 1, 0), use_model_front: bool = false)
```

Moves the node to the specified position, then rotates the node to point toward the target position, similar to look_at(). This operation is calculated in global space (relative to the world).

### orthonormalize {#orthonormalize}

```gdscript
void orthonormalize()
```

Orthonormalizes this node's basis. This method sets this node's scale to Vector3.ONE (or its negative counterpart), but preserves the position and rotation. See also Transform3D.orthonormalized().

### rotate {#rotate}

```gdscript
void rotate(axis: Vector3, angle: float)
```

Rotates this node's basis around the axis by the given angle, in radians. This operation is calculated in parent space (relative to the parent) and preserves the position.

### rotate_object_local {#rotate_object_local}

```gdscript
void rotate_object_local(axis: Vector3, angle: float)
```

Rotates this node's basis around the axis by the given angle, in radians. This operation is calculated in local space (relative to this node) and preserves the position.

### rotate_x {#rotate_x}

```gdscript
void rotate_x(angle: float)
```

Rotates this node's basis around the X axis by the given angle, in radians. This operation is calculated in parent space (relative to the parent) and preserves the position.

### rotate_y {#rotate_y}

```gdscript
void rotate_y(angle: float)
```

Rotates this node's basis around the Y axis by the given angle, in radians. This operation is calculated in parent space (relative to the parent) and preserves the position.

### rotate_z {#rotate_z}

```gdscript
void rotate_z(angle: float)
```

Rotates this node's basis around the Z axis by the given angle, in radians. This operation is calculated in parent space (relative to the parent) and preserves the position.

### scale_object_local {#scale_object_local}

```gdscript
void scale_object_local(scale: Vector3)
```

Scales this node's basis by the given scale factor. This operation is calculated in local space (relative to this node) and preserves the position.

### set_as_top_level {#set_as_top_level}

```gdscript
void set_as_top_level(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_basis {#set_basis}

```gdscript
void set_basis(basis: Basis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_scale {#set_disable_scale}

```gdscript
void set_disable_scale(disable: bool)
```

If true, this node's global_transform is automatically orthonormalized. This results in this node not appearing distorted, as if its global scale were set to Vector3.ONE (or its negative counterpart). See also is_scale_disabled() and orthonormalize().

Note: transform is not affected by this setting.

### set_global_basis {#set_global_basis}

```gdscript
void set_global_basis(basis: Basis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_position {#set_global_position}

```gdscript
void set_global_position(position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_rotation {#set_global_rotation}

```gdscript
void set_global_rotation(euler_radians: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_rotation_degrees {#set_global_rotation_degrees}

```gdscript
void set_global_rotation_degrees(euler_degrees: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_transform {#set_global_transform}

```gdscript
void set_global_transform(global: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_identity {#set_identity}

```gdscript
void set_identity()
```

Sets this node's transform to Transform3D.IDENTITY, which resets all transformations in parent space (position, rotation, and scale).

### set_ignore_transform_notification {#set_ignore_transform_notification}

```gdscript
void set_ignore_transform_notification(enabled: bool)
```

If true, the node will not receive NOTIFICATION_TRANSFORM_CHANGED or NOTIFICATION_LOCAL_TRANSFORM_CHANGED.

It may useful to call this method when handling these notifications to prevent infinite recursion.

### set_notify_local_transform {#set_notify_local_transform}

```gdscript
void set_notify_local_transform(enable: bool)
```

If true, the node will receive NOTIFICATION_LOCAL_TRANSFORM_CHANGED whenever transform changes.

Note: Some 3D nodes such as CSGShape3D or CollisionShape3D automatically enable this to function correctly.

### set_notify_transform {#set_notify_transform}

```gdscript
void set_notify_transform(enable: bool)
```

If true, the node will receive NOTIFICATION_TRANSFORM_CHANGED whenever global_transform changes.

Note: Most 3D nodes such as VisualInstance3D or CollisionObject3D automatically enable this to function correctly.

Note: In the editor, nodes will propagate this notification to their children if a gizmo is attached (see add_gizmo()).

### set_position {#set_position}

```gdscript
void set_position(position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_quaternion {#set_quaternion}

```gdscript
void set_quaternion(quaternion: Quaternion)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation {#set_rotation}

```gdscript
void set_rotation(euler_radians: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_degrees {#set_rotation_degrees}

```gdscript
void set_rotation_degrees(euler_degrees: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_edit_mode {#set_rotation_edit_mode}

```gdscript
void set_rotation_edit_mode(edit_mode: Node3D.RotationEditMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_order {#set_rotation_order}

```gdscript
void set_rotation_order(order: EulerOrder)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale {#set_scale}

```gdscript
void set_scale(scale: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subgizmo_selection {#set_subgizmo_selection}

```gdscript
void set_subgizmo_selection(gizmo: Node3DGizmo, id: int, transform: Transform3D)
```

Selects the gizmo's subgizmo with the given id and sets its transform. Only works in the editor.

Note: The gizmo object would typically be an instance of EditorNode3DGizmo, but the argument type is kept generic to avoid creating a dependency on editor classes in Node3D.

### set_transform {#set_transform}

```gdscript
void set_transform(local: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visibility_parent {#set_visibility_parent}

```gdscript
void set_visibility_parent(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible {#set_visible}

```gdscript
void set_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### show {#show}

```gdscript
void show()
```

Allows this node to be rendered. Equivalent to setting visible to true. This is the opposite of hide().

### to_global {#to_global}

```gdscript
Vector3 to_global(local_point: Vector3) const
```

Returns the local_point converted from this node's local space to global space. This is the opposite of to_local().

### to_local {#to_local}

```gdscript
Vector3 to_local(global_point: Vector3) const
```

Returns the global_point converted from global space to this node's local space. This is the opposite of to_global().

### translate {#translate}

```gdscript
void translate(offset: Vector3)
```

Adds the given translation offset to the node's position, in local space (relative to this node).

Note: Prefer using translate_object_local(), instead, as this method may be changed in a future release.

Note: Despite the naming convention, this operation is not calculated in parent space for compatibility reasons. To translate in parent space, add offset to the position (node_3d.position += offset).

### translate_object_local {#translate_object_local}

```gdscript
void translate_object_local(offset: Vector3)
```

Adds the given translation offset to the node's position, in local space (relative to this node).

### update_gizmos {#update_gizmos}

```gdscript
void update_gizmos()
```

Updates all the EditorNode3DGizmo objects attached to this node. Only works in the editor.

## 属性

### transform {#transform}

```gdscript
transform: Transform3D
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### global_transform {#global_transform}

```gdscript
global_transform: Transform3D
```

- void set_global_transform(value: Transform3D)
- Transform3D get_global_transform()

The transformation of this node, in global space (relative to the world). Contains and represents this node's global_position, global_rotation, and global scale.

Note: If the node is not inside the tree, getting this property fails and returns Transform3D.IDENTITY.

### position {#position}

```gdscript
position: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### rotation {#rotation}

```gdscript
rotation: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### rotation_degrees {#rotation_degrees}

```gdscript
rotation_degrees: Vector3
```

- void set_rotation_degrees(value: Vector3)
- Vector3 get_rotation_degrees()

The rotation of this node, in degrees instead of radians.

Note: This is not the property available in the Inspector dock.

### quaternion {#quaternion}

```gdscript
quaternion: Quaternion
```

- void set_quaternion(value: Quaternion)
- Quaternion get_quaternion()

Rotation of this node represented as a Quaternion in parent space (relative to the parent node). This value is obtained from basis's rotation.

Note: Quaternions are much more suitable for 3D math but are less intuitive. Setting this property can be useful for interpolation (see Quaternion.slerp()).

### basis {#basis}

```gdscript
basis: Basis
```

- void set_basis(value: Basis)
- Basis get_basis()

Basis of the transform property. Represents the rotation, scale, and shear of this node in parent space (relative to the parent node).

### scale {#scale}

```gdscript
scale: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### rotation_edit_mode {#rotation_edit_mode}

```gdscript
rotation_edit_mode: int
```

- void set_rotation_edit_mode(value: RotationEditMode)
- RotationEditMode get_rotation_edit_mode()

How this node's rotation and scale are displayed in the Inspector dock.

### rotation_order {#rotation_order}

```gdscript
rotation_order: int
```

- void set_rotation_order(value: EulerOrder)
- EulerOrder get_rotation_order()

The axis rotation order of the rotation property. In Godot, Euler angles always use intrinsic order, meaning that the final orientation is calculated by rotating around the local axes in this order.

### top_level {#top_level}

```gdscript
top_level: bool
```

- void set_as_top_level(value: bool)
- bool is_set_as_top_level()

If true, the node does not inherit its transformations from its parent. As such, node transformations will only be in global space, which also means that global_transform and transform will be identical.

### global_position {#global_position}

```gdscript
global_position: Vector3
```

- void set_global_position(value: Vector3)
- Vector3 get_global_position()

Global position (translation) of this node in global space (relative to the world). This is equivalent to the global_transform's Transform3D.origin.

Note: If the node is not inside the tree, getting this property fails and returns Vector3.ZERO.

### global_basis {#global_basis}

```gdscript
global_basis: Basis
```

- void set_global_basis(value: Basis)
- Basis get_global_basis()

Basis of the global_transform property. Represents the rotation, scale, and shear of this node in global space (relative to the world).

Note: If the node is not inside the tree, getting this property fails and returns Basis.IDENTITY.

### global_rotation {#global_rotation}

```gdscript
global_rotation: Vector3
```

- void set_global_rotation(value: Vector3)
- Vector3 get_global_rotation()

Global rotation of this node as Euler angles [https://en.wikipedia.org/wiki/Euler_angles], in radians and in global space (relative to the world). This value is obtained from global_basis's rotation.

- The Vector3.x is the angle around the global X axis (pitch);
- The Vector3.y is the angle around the global Y axis (yaw);
- The Vector3.z is the angle around the global Z axis (roll).

Note: Unlike rotation, this property always follows the YXZ convention (@GlobalScope.EULER_ORDER_YXZ).

Note: If the node is not inside the tree, getting this property fails and returns Vector3.ZERO.

### global_rotation_degrees {#global_rotation_degrees}

```gdscript
global_rotation_degrees: Vector3
```

- void set_global_rotation_degrees(value: Vector3)
- Vector3 get_global_rotation_degrees()

The global_rotation of this node, in degrees instead of radians.

Note: If the node is not inside the tree, getting this property fails and returns Vector3.ZERO.

### visible {#visible}

```gdscript
visible: bool
```

- void set_visible(value: bool)
- bool is_visible()

If true, this node can be visible. The node is only rendered when all of its ancestors are visible, as well. That means is_visible_in_tree() must return true.

### visibility_parent {#visibility_parent}

```gdscript
visibility_parent: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 信号

### visibility_changed {#visibility_changed}

```gdscript
signal visibility_changed()
```

Emitted when this node's visibility changes (see visible and is_visible_in_tree()).

This signal is emitted after the related NOTIFICATION_VISIBILITY_CHANGED notification.

## 枚举

### RotationEditMode {#enum-RotationEditMode}

```gdscript
enum RotationEditMode { ROTATION_EDIT_MODE_EULER = 0, ROTATION_EDIT_MODE_QUATERNION = 1, ROTATION_EDIT_MODE_BASIS = 2 }
```

RotationEditMode ROTATION_EDIT_MODE_EULER = 0

The rotation is edited using a Vector3 in Euler angles [https://en.wikipedia.org/wiki/Euler_angles]. In Godot, Euler angles always use intrinsic order, meaning that rotation happens around the local axes of the object.

RotationEditMode ROTATION_EDIT_MODE_QUATERNION = 1

The rotation is edited using a Quaternion. Quaternions avoid gimbal lock and having to choose an order of rotation, but are less intuitive. Quaternion rotation is mostly the same as rotors in 3D geometric algebra, except that the numbers are labeled differently.

RotationEditMode ROTATION_EDIT_MODE_BASIS = 2

The rotation is edited using a Basis. In this mode, the raw basis's axes can be freely modified, but the scale property is not available.

#### ROTATION_EDIT_MODE_EULER {#ROTATION_EDIT_MODE_EULER}

```gdscript
ROTATION_EDIT_MODE_EULER = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_EDIT_MODE_QUATERNION {#ROTATION_EDIT_MODE_QUATERNION}

```gdscript
ROTATION_EDIT_MODE_QUATERNION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_EDIT_MODE_BASIS {#ROTATION_EDIT_MODE_BASIS}

```gdscript
ROTATION_EDIT_MODE_BASIS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 常量

### NOTIFICATION_TRANSFORM_CHANGED {#NOTIFICATION_TRANSFORM_CHANGED}

```gdscript
const NOTIFICATION_TRANSFORM_CHANGED = 2000
```

Notification received when this node's global_transform changes, if is_transform_notification_enabled() is true. See also set_notify_transform().

Note: Most 3D nodes such as VisualInstance3D or CollisionObject3D automatically enable this to function correctly.

Note: In the editor, nodes will propagate this notification to their children if a gizmo is attached (see add_gizmo()).

NOTIFICATION_ENTER_WORLD = 41 

Notification received when this node is registered to a new World3D (see get_world_3d()).

NOTIFICATION_EXIT_WORLD = 42 

Notification received when this node is unregistered from the current World3D (see get_world_3d()).

This notification is sent in reversed order.

NOTIFICATION_VISIBILITY_CHANGED = 43 

Notification received when this node's visibility changes (see visible and is_visible_in_tree()).

This notification is received before the related visibility_changed signal.

NOTIFICATION_LOCAL_TRANSFORM_CHANGED = 44 

Notification received when this node's transform changes, if is_local_transform_notification_enabled() is true. This is not received when a parent Node3D's transform changes. See also set_notify_local_transform().

Note: Some 3D nodes such as CSGShape3D or CollisionShape3D automatically enable this to function correctly.

### NOTIFICATION_ENTER_WORLD {#NOTIFICATION_ENTER_WORLD}

```gdscript
const NOTIFICATION_ENTER_WORLD = 41
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_EXIT_WORLD {#NOTIFICATION_EXIT_WORLD}

```gdscript
const NOTIFICATION_EXIT_WORLD = 42
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_VISIBILITY_CHANGED {#NOTIFICATION_VISIBILITY_CHANGED}

```gdscript
const NOTIFICATION_VISIBILITY_CHANGED = 43
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_LOCAL_TRANSFORM_CHANGED {#NOTIFICATION_LOCAL_TRANSFORM_CHANGED}

```gdscript
const NOTIFICATION_LOCAL_TRANSFORM_CHANGED = 44
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
