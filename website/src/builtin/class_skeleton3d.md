# Skeleton3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Skeleton3D**

> Skeleton3D provides an interface for managing a hierarchy of bones, including pose, rest and animation (see Animation). It can also use ragdoll physics.

<details><summary>官方英文描述（中文精译进行中）</summary>

Skeleton3D provides an interface for managing a hierarchy of bones, including pose, rest and animation (see Animation). It can also use ragdoll physics.

The overall transform of a bone with respect to the skeleton is determined by bone pose. Bone rest defines the initial transform of the bone pose.

Note that "global pose" below refers to the overall transform of the bone with respect to skeleton, so it is not the actual global/world transform of the bone.

</details>

## 方法

### add_bone {#add_bone}

```gdscript
int add_bone(name: String)
```

Adds a new bone with the given name. Returns the new bone's index, or -1 if this method fails.

Note: Bone names should be unique, non empty, and cannot include the : and / characters.

### advance {#advance}

```gdscript
void advance(delta: float)
```

Manually advance the child SkeletonModifier3Ds by the specified time (in seconds).

Note: The delta is temporarily accumulated in the Skeleton3D, and the deferred process uses the accumulated value to process the modification.

### clear_bones {#clear_bones}

```gdscript
void clear_bones()
```

Clear all the bones in this skeleton.

### clear_bones_global_pose_override {#clear_bones_global_pose_override}

```gdscript
void clear_bones_global_pose_override()
```

Deprecated: This method may be changed or removed in future versions.

Removes the global pose override on all bones in the skeleton.

### create_skin_from_rest_transforms {#create_skin_from_rest_transforms}

```gdscript
Skin create_skin_from_rest_transforms()
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### find_bone {#find_bone}

```gdscript
int find_bone(name: String) const
```

Returns the bone index that matches name as its name. Returns -1 if no bone with this name exists.

### force_update_all_bone_transforms {#force_update_all_bone_transforms}

```gdscript
void force_update_all_bone_transforms()
```

Deprecated: This method should only be called internally.

Force updates the bone transforms/poses for all bones in the skeleton.

### force_update_bone_child_transform {#force_update_bone_child_transform}

```gdscript
void force_update_bone_child_transform(bone_idx: int)
```

Force updates the bone transform for the bone at bone_idx and all of its children.

### get_animate_physical_bones {#get_animate_physical_bones}

```gdscript
bool get_animate_physical_bones() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_children {#get_bone_children}

```gdscript
PackedInt32Array get_bone_children(bone_idx: int) const
```

Returns an array containing the bone indexes of all the child node of the passed in bone, bone_idx.

### get_bone_count {#get_bone_count}

```gdscript
int get_bone_count() const
```

Returns the number of bones in the skeleton.

### get_bone_global_pose {#get_bone_global_pose}

```gdscript
Transform3D get_bone_global_pose(bone_idx: int) const
```

Returns the overall transform of the specified bone, with respect to the skeleton. Being relative to the skeleton frame, this is not the actual "global" transform of the bone.

Note: This is the global pose you set to the skeleton in the process, the final global pose can get overridden by modifiers in the deferred process, if you want to access the final global pose, use SkeletonModifier3D.modification_processed.

### get_bone_global_pose_no_override {#get_bone_global_pose_no_override}

```gdscript
Transform3D get_bone_global_pose_no_override(bone_idx: int) const
```

Deprecated: This method may be changed or removed in future versions.

Returns the overall transform of the specified bone, with respect to the skeleton, but without any global pose overrides. Being relative to the skeleton frame, this is not the actual "global" transform of the bone.

### get_bone_global_pose_override {#get_bone_global_pose_override}

```gdscript
Transform3D get_bone_global_pose_override(bone_idx: int) const
```

Deprecated: This method may be changed or removed in future versions.

Returns the global pose override transform for bone_idx.

### get_bone_global_rest {#get_bone_global_rest}

```gdscript
Transform3D get_bone_global_rest(bone_idx: int) const
```

Returns the global rest transform for bone_idx.

### get_bone_meta {#get_bone_meta}

```gdscript
Variant get_bone_meta(bone_idx: int, key: StringName) const
```

Returns the metadata with the given key for the bone at index bone_idx.

### get_bone_meta_list {#get_bone_meta_list}

```gdscript
typedarray::StringName get_bone_meta_list(bone_idx: int) const
```

Returns the list of all metadata keys for the bone at index bone_idx.

### get_bone_name {#get_bone_name}

```gdscript
String get_bone_name(bone_idx: int) const
```

Returns the name of the bone at index bone_idx.

### get_bone_parent {#get_bone_parent}

```gdscript
int get_bone_parent(bone_idx: int) const
```

Returns the bone index which is the parent of the bone at bone_idx. If -1, then bone has no parent.

Note: The parent bone returned will always be less than bone_idx.

### get_bone_pose {#get_bone_pose}

```gdscript
Transform3D get_bone_pose(bone_idx: int) const
```

Returns the pose transform of the specified bone.

Note: This is the pose you set to the skeleton in the process, the final pose can get overridden by modifiers in the deferred process, if you want to access the final pose, use SkeletonModifier3D.modification_processed.

### get_bone_pose_position {#get_bone_pose_position}

```gdscript
Vector3 get_bone_pose_position(bone_idx: int) const
```

Returns the pose position of the bone at bone_idx. The returned Vector3 is in the local coordinate space of the Skeleton3D node.

### get_bone_pose_rotation {#get_bone_pose_rotation}

```gdscript
Quaternion get_bone_pose_rotation(bone_idx: int) const
```

Returns the pose rotation of the bone at bone_idx. The returned Quaternion is local to the bone with respect to the rotation of any parent bones.

### get_bone_pose_scale {#get_bone_pose_scale}

```gdscript
Vector3 get_bone_pose_scale(bone_idx: int) const
```

Returns the pose scale of the bone at bone_idx.

### get_bone_rest {#get_bone_rest}

```gdscript
Transform3D get_bone_rest(bone_idx: int) const
```

Returns the rest transform for a bone bone_idx.

### get_concatenated_bone_names {#get_concatenated_bone_names}

```gdscript
StringName get_concatenated_bone_names() const
```

Returns all bone names concatenated with commas (,) as a single StringName.

It is useful to set it as a hint for the enum property.

### get_modifier_callback_mode_process {#get_modifier_callback_mode_process}

```gdscript
Skeleton3D.ModifierCallbackModeProcess get_modifier_callback_mode_process() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_motion_scale {#get_motion_scale}

```gdscript
float get_motion_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parentless_bones {#get_parentless_bones}

```gdscript
PackedInt32Array get_parentless_bones() const
```

Returns an array with all of the bones that are parentless. Another way to look at this is that it returns the indexes of all the bones that are not dependent or modified by other bones in the Skeleton.

### get_version {#get_version}

```gdscript
int get_version() const
```

Returns the number of times the bone hierarchy has changed within this skeleton, including renames.

The Skeleton version is not serialized: only use within a single instance of Skeleton3D.

Use for invalidating caches in IK solvers and other nodes which process bones.

### has_bone_meta {#has_bone_meta}

```gdscript
bool has_bone_meta(bone_idx: int, key: StringName) const
```

Returns true if the bone at index bone_idx has metadata with the given key.

### is_bone_enabled {#is_bone_enabled}

```gdscript
bool is_bone_enabled(bone_idx: int) const
```

Returns whether the bone pose for the bone at bone_idx is enabled.

### is_show_rest_only {#is_show_rest_only}

```gdscript
bool is_show_rest_only() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### localize_rests {#localize_rests}

```gdscript
void localize_rests()
```

Returns all bones in the skeleton to their rest poses.

### physical_bones_add_collision_exception {#physical_bones_add_collision_exception}

```gdscript
void physical_bones_add_collision_exception(exception: RID)
```

Deprecated: This method may be changed or removed in future versions.

Adds a collision exception to the physical bone.

Works just like the RigidBody3D node.

### physical_bones_remove_collision_exception {#physical_bones_remove_collision_exception}

```gdscript
void physical_bones_remove_collision_exception(exception: RID)
```

Deprecated: This method may be changed or removed in future versions.

Removes a collision exception to the physical bone.

Works just like the RigidBody3D node.

### physical_bones_start_simulation {#physical_bones_start_simulation}

```gdscript
void physical_bones_start_simulation(bones: typedarray::StringName = [])
```

Deprecated: This method may be changed or removed in future versions.

Tells the PhysicalBone3D nodes in the Skeleton to start simulating and reacting to the physics world.

Optionally, a list of bone names can be passed-in, allowing only the passed-in bones to be simulated.

### physical_bones_stop_simulation {#physical_bones_stop_simulation}

```gdscript
void physical_bones_stop_simulation()
```

Deprecated: This method may be changed or removed in future versions.

Tells the PhysicalBone3D nodes in the Skeleton to stop simulating.

### register_skin {#register_skin}

```gdscript
SkinReference register_skin(skin: Skin)
```

Binds the given Skin to the Skeleton.

### reset_bone_pose {#reset_bone_pose}

```gdscript
void reset_bone_pose(bone_idx: int)
```

Sets the bone pose to rest for bone_idx.

### reset_bone_poses {#reset_bone_poses}

```gdscript
void reset_bone_poses()
```

Sets all bone poses to rests.

### set_animate_physical_bones {#set_animate_physical_bones}

```gdscript
void set_animate_physical_bones(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_enabled {#set_bone_enabled}

```gdscript
void set_bone_enabled(bone_idx: int, enabled: bool = true)
```

Disables the pose for the bone at bone_idx if false, enables the bone pose if true.

### set_bone_global_pose {#set_bone_global_pose}

```gdscript
void set_bone_global_pose(bone_idx: int, pose: Transform3D)
```

Sets the global pose transform, pose, for the bone at bone_idx.

Note: If other bone poses have been changed, this method executes a dirty poses recalculation and will cause performance to deteriorate. If you know that multiple global poses will be applied, consider using set_bone_pose() with precalculation.

### set_bone_global_pose_override {#set_bone_global_pose_override}

```gdscript
void set_bone_global_pose_override(bone_idx: int, pose: Transform3D, amount: float, persistent: bool = false)
```

Deprecated: This method may be changed or removed in future versions.

Sets the global pose transform, pose, for the bone at bone_idx.

amount is the interpolation strength that will be used when applying the pose, and persistent determines if the applied pose will remain.

Note: The pose transform needs to be a global pose! To convert a world transform from a Node3D to a global bone pose, multiply the Transform3D.affine_inverse() of the node's Node3D.global_transform by the desired world transform.

### set_bone_meta {#set_bone_meta}

```gdscript
void set_bone_meta(bone_idx: int, key: StringName, value: Variant)
```

Sets the metadata with the given key to value for the bone at index bone_idx.

### set_bone_name {#set_bone_name}

```gdscript
void set_bone_name(bone_idx: int, name: String)
```

Sets the bone name, name, for the bone at bone_idx.

### set_bone_parent {#set_bone_parent}

```gdscript
void set_bone_parent(bone_idx: int, parent_idx: int)
```

Sets the bone index parent_idx as the parent of the bone at bone_idx. If -1, then bone has no parent.

Note: parent_idx must be less than bone_idx.

### set_bone_pose {#set_bone_pose}

```gdscript
void set_bone_pose(bone_idx: int, pose: Transform3D)
```

Sets the pose transform, pose, for the bone at bone_idx.

### set_bone_pose_position {#set_bone_pose_position}

```gdscript
void set_bone_pose_position(bone_idx: int, position: Vector3)
```

Sets the pose position of the bone at bone_idx to position. position is a Vector3 describing a position local to the Skeleton3D node.

### set_bone_pose_rotation {#set_bone_pose_rotation}

```gdscript
void set_bone_pose_rotation(bone_idx: int, rotation: Quaternion)
```

Sets the pose rotation of the bone at bone_idx to rotation. rotation is a Quaternion describing a rotation in the bone's local coordinate space with respect to the rotation of any parent bones.

### set_bone_pose_scale {#set_bone_pose_scale}

```gdscript
void set_bone_pose_scale(bone_idx: int, scale: Vector3)
```

Sets the pose scale of the bone at bone_idx to scale.

### set_bone_rest {#set_bone_rest}

```gdscript
void set_bone_rest(bone_idx: int, rest: Transform3D)
```

Sets the rest transform for bone bone_idx.

### set_modifier_callback_mode_process {#set_modifier_callback_mode_process}

```gdscript
void set_modifier_callback_mode_process(mode: Skeleton3D.ModifierCallbackModeProcess)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motion_scale {#set_motion_scale}

```gdscript
void set_motion_scale(motion_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_rest_only {#set_show_rest_only}

```gdscript
void set_show_rest_only(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### unparent_bone_and_rest {#unparent_bone_and_rest}

```gdscript
void unparent_bone_and_rest(bone_idx: int)
```

Unparents the bone at bone_idx and sets its rest position to that of its parent prior to being reset.

## 属性

### motion_scale {#motion_scale}

```gdscript
motion_scale: float
```

- void set_motion_scale(value: float)
- float get_motion_scale()

Multiplies the 3D position track animation.

Note: Unless this value is 1.0, the key value in animation will not match the actual position value.

### show_rest_only {#show_rest_only}

```gdscript
show_rest_only: bool
```

- void set_show_rest_only(value: bool)
- bool is_show_rest_only()

If true, forces the bones in their default rest pose, regardless of their values. In the editor, this also prevents the bones from being edited.

### modifier_callback_mode_process {#modifier_callback_mode_process}

```gdscript
modifier_callback_mode_process: int
```

- void set_modifier_callback_mode_process(value: ModifierCallbackModeProcess)
- ModifierCallbackModeProcess get_modifier_callback_mode_process()

Sets the processing timing for the Modifier.

### animate_physical_bones {#animate_physical_bones}

```gdscript
animate_physical_bones: bool
```

- void set_animate_physical_bones(value: bool)
- bool get_animate_physical_bones()

Deprecated: This property may be changed or removed in future versions.

If you follow the recommended workflow and explicitly have PhysicalBoneSimulator3D as a child of Skeleton3D, you can control whether it is affected by raycasting without running physical_bones_start_simulation(), by its SkeletonModifier3D.active.

However, for old (deprecated) configurations, Skeleton3D has an internal virtual PhysicalBoneSimulator3D for compatibility. This property controls the internal virtual PhysicalBoneSimulator3D's SkeletonModifier3D.active.

## 信号

### rest_updated {#rest_updated}

```gdscript
signal rest_updated()
```

Emitted when the rest is updated.

### pose_updated {#pose_updated}

```gdscript
signal pose_updated()
```

Emitted when the pose is updated.

Note: During the update process, this signal is not fired, so modification by SkeletonModifier3D is not detected.

### skeleton_updated {#skeleton_updated}

```gdscript
signal skeleton_updated()
```

Emitted when the final pose has been calculated will be applied to the skin in the update process.

This means that all SkeletonModifier3D processing is complete. In order to detect the completion of the processing of each SkeletonModifier3D, use SkeletonModifier3D.modification_processed.

### bone_enabled_changed {#bone_enabled_changed}

```gdscript
signal bone_enabled_changed(bone_idx: int)
```

Emitted when the bone at bone_idx is toggled with set_bone_enabled(). Use is_bone_enabled() to check the new value.

### bone_list_changed {#bone_list_changed}

```gdscript
signal bone_list_changed()
```

Emitted when the list of bones changes, such as when calling add_bone(), set_bone_parent(), unparent_bone_and_rest(), or clear_bones().

### show_rest_only_changed {#show_rest_only_changed}

```gdscript
signal show_rest_only_changed()
```

Emitted when the value of show_rest_only changes.

## 枚举

### ModifierCallbackModeProcess {#enum-ModifierCallbackModeProcess}

```gdscript
enum ModifierCallbackModeProcess { MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS = 0, MODIFIER_CALLBACK_MODE_PROCESS_IDLE = 1, MODIFIER_CALLBACK_MODE_PROCESS_MANUAL = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS {#MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS}

```gdscript
MODIFIER_CALLBACK_MODE_PROCESS_PHYSICS = 0
```

Set a flag to process modification during physics frames (see Node.NOTIFICATION_INTERNAL_PHYSICS_PROCESS).

#### MODIFIER_CALLBACK_MODE_PROCESS_IDLE {#MODIFIER_CALLBACK_MODE_PROCESS_IDLE}

```gdscript
MODIFIER_CALLBACK_MODE_PROCESS_IDLE = 1
```

Set a flag to process modification during process frames (see Node.NOTIFICATION_INTERNAL_PROCESS).

#### MODIFIER_CALLBACK_MODE_PROCESS_MANUAL {#MODIFIER_CALLBACK_MODE_PROCESS_MANUAL}

```gdscript
MODIFIER_CALLBACK_MODE_PROCESS_MANUAL = 2
```

Do not process modification. Use advance() to process the modification manually.

---

## 常量

### NOTIFICATION_UPDATE_SKELETON {#NOTIFICATION_UPDATE_SKELETON}

```gdscript
const NOTIFICATION_UPDATE_SKELETON = 50
```

Notification received when this skeleton's pose needs to be updated. In that case, this is called only once per frame in a deferred process.
