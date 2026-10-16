# SkeletonIK3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SkeletonIK3D**

> SkeletonIK3D is used to rotate all bones of a Skeleton3D bone chain a way that places the end bone at a desired 3D position. A typical scenario for IK in games is to place a character's feet on the ground or a character's hands on a currently held object. SkeletonIK uses FabrikInverseKinematic internally to solve the bone chain and applies the results to the Skeleton3D bones_global_pose_override property for all affected bones in the chain. If fully applied, this overwrites any bone transform from Animations or bone custom poses set by users. The applied amount can be controlled with the SkeletonModifier3D.influence property.

<details><summary>官方英文描述</summary>

SkeletonIK3D is used to rotate all bones of a Skeleton3D bone chain a way that places the end bone at a desired 3D position. A typical scenario for IK in games is to place a character's feet on the ground or a character's hands on a currently held object. SkeletonIK uses FabrikInverseKinematic internally to solve the bone chain and applies the results to the Skeleton3D bones_global_pose_override property for all affected bones in the chain. If fully applied, this overwrites any bone transform from Animations or bone custom poses set by users. The applied amount can be controlled with the SkeletonModifier3D.influence property.

```gdscript
# Apply IK effect automatically on every new frame (not the current)
skeleton_ik_node.start()
# Apply IK effect only on the current frame
skeleton_ik_node.start(true)
# Stop IK effect and reset bones_global_pose_override on Skeleton
skeleton_ik_node.stop()
# Apply full IK effect
skeleton_ik_node.set_influence(1.0)
# Apply half IK effect
skeleton_ik_node.set_influence(0.5)
# Apply zero IK effect (a value at or below 0.01 also removes bones_global_pose_override on Skeleton)
skeleton_ik_node.set_influence(0.0)
```

</details>

## 方法

### get_interpolation {#get_interpolation}

```gdscript
float get_interpolation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_magnet_position {#get_magnet_position}

```gdscript
Vector3 get_magnet_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_iterations {#get_max_iterations}

```gdscript
int get_max_iterations() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_distance {#get_min_distance}

```gdscript
float get_min_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parent_skeleton {#get_parent_skeleton}

```gdscript
Skeleton3D get_parent_skeleton() const
```

Returns the parent Skeleton3D node that was present when SkeletonIK entered the scene tree. Returns null if the parent node was not a Skeleton3D node when SkeletonIK3D entered the scene tree.

### get_root_bone {#get_root_bone}

```gdscript
StringName get_root_bone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_transform {#get_target_transform}

```gdscript
Transform3D get_target_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tip_bone {#get_tip_bone}

```gdscript
StringName get_tip_bone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_override_tip_basis {#is_override_tip_basis}

```gdscript
bool is_override_tip_basis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_running {#is_running}

```gdscript
bool is_running()
```

Returns true if SkeletonIK is applying IK effects on continues frames to the Skeleton3D bones. Returns false if SkeletonIK is stopped or start() was used with the one_time parameter set to true.

### is_using_magnet {#is_using_magnet}

```gdscript
bool is_using_magnet() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interpolation {#set_interpolation}

```gdscript
void set_interpolation(interpolation: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_magnet_position {#set_magnet_position}

```gdscript
void set_magnet_position(local_position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_iterations {#set_max_iterations}

```gdscript
void set_max_iterations(iterations: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_distance {#set_min_distance}

```gdscript
void set_min_distance(min_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_override_tip_basis {#set_override_tip_basis}

```gdscript
void set_override_tip_basis(override: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_root_bone {#set_root_bone}

```gdscript
void set_root_bone(root_bone: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_node {#set_target_node}

```gdscript
void set_target_node(node: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_transform {#set_target_transform}

```gdscript
void set_target_transform(target: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tip_bone {#set_tip_bone}

```gdscript
void set_tip_bone(tip_bone: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_magnet {#set_use_magnet}

```gdscript
void set_use_magnet(use: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### start {#start}

```gdscript
void start(one_time: bool = false)
```

Starts applying IK effects on each frame to the Skeleton3D bones but will only take effect starting on the next frame. If one_time is true, this will take effect immediately but also reset on the next frame.

### stop {#stop}

```gdscript
void stop()
```

Stops applying IK effects on each frame to the Skeleton3D bones and also calls Skeleton3D.clear_bones_global_pose_override() to remove existing overrides on all bones.

## 属性

### root_bone {#root_bone}

```gdscript
root_bone: StringName
```

- void set_root_bone(value: StringName)
- StringName get_root_bone()

The name of the current root bone, the first bone in the IK chain.

### tip_bone {#tip_bone}

```gdscript
tip_bone: StringName
```

- void set_tip_bone(value: StringName)
- StringName get_tip_bone()

The name of the current tip bone, the last bone in the IK chain placed at the target transform (or target_node if defined).

### target {#target}

```gdscript
target: Transform3D
```

- void set_target_transform(value: Transform3D)
- Transform3D get_target_transform()

First target of the IK chain where the tip bone is placed and, if override_tip_basis is true, how the tip bone is rotated. If a target_node path is available the nodes transform is used instead and this property is ignored.

### override_tip_basis {#override_tip_basis}

```gdscript
override_tip_basis: bool
```

- void set_override_tip_basis(value: bool)
- bool is_override_tip_basis()

If true overwrites the rotation of the tip bone with the rotation of the target (or target_node if defined).

### use_magnet {#use_magnet}

```gdscript
use_magnet: bool
```

- void set_use_magnet(value: bool)
- bool is_using_magnet()

If true, instructs the IK solver to consider the secondary magnet target (pole target) when calculating the bone chain. Use the magnet position (pole target) to control the bending of the IK chain.

### magnet {#magnet}

```gdscript
magnet: Vector3
```

- void set_magnet_position(value: Vector3)
- Vector3 get_magnet_position()

Secondary target position (first is target property or target_node) for the IK chain. Use magnet position (pole target) to control the bending of the IK chain. Only works if the bone chain has more than 2 bones. The middle chain bone position will be linearly interpolated with the magnet position.

### target_node {#target_node}

```gdscript
target_node: NodePath
```

- void set_target_node(value: NodePath)
- NodePath get_target_node()

Target node NodePath for the IK chain. If available, the node's current Transform3D is used instead of the target property.

### min_distance {#min_distance}

```gdscript
min_distance: float
```

- void set_min_distance(value: float)
- float get_min_distance()

The minimum distance between bone and goal target. If the distance is below this value, the IK solver stops further iterations.

### max_iterations {#max_iterations}

```gdscript
max_iterations: int
```

- void set_max_iterations(value: int)
- int get_max_iterations()

Number of iteration loops used by the IK solver to produce more accurate (and elegant) bone chain results.

### interpolation {#interpolation}

```gdscript
interpolation: float
```

- void set_interpolation(value: float)
- float get_interpolation()

Deprecated: Use SkeletonModifier3D.influence instead.

Interpolation value for how much the IK results are applied to the current skeleton bone chain. A value of 1.0 will overwrite all skeleton bone transforms completely while a value of 0.0 will visually disable the SkeletonIK.
