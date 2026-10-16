# SkeletonModification2DJiggle

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DJiggle**

> This modification moves a series of bones, typically called a bone chain, towards a target. What makes this modification special is that it calculates the velocity and acceleration for each bone in the bone chain, and runs a very light physics-like calculation using the inputted values. This allows the bones to overshoot the target and "jiggle" around. It can be configured to act more like a spring, or sway around like cloth might.

<details><summary>官方英文描述</summary>

This modification moves a series of bones, typically called a bone chain, towards a target. What makes this modification special is that it calculates the velocity and acceleration for each bone in the bone chain, and runs a very light physics-like calculation using the inputted values. This allows the bones to overshoot the target and "jiggle" around. It can be configured to act more like a spring, or sway around like cloth might.

This modification is useful for adding additional motion to things like hair, the edges of clothing, and more. It has several settings to that allow control over how the joint moves when the target moves.

Note: The Jiggle modifier has jiggle_joints, which are the data objects that hold the data for each joint in the Jiggle chain. This is different from than Bone2D nodes! Jiggle joints hold the data needed for each Bone2D in the bone chain used by the Jiggle modification.

</details>

## 方法

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

Returns the collision mask used by the Jiggle modifier when collisions are enabled.

### get_damping {#get_damping}

```gdscript
float get_damping() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity {#get_gravity}

```gdscript
Vector2 get_gravity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_jiggle_data_chain_length {#get_jiggle_data_chain_length}

```gdscript
int get_jiggle_data_chain_length()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_jiggle_joint_bone2d_node {#get_jiggle_joint_bone2d_node}

```gdscript
NodePath get_jiggle_joint_bone2d_node(joint_idx: int) const
```

Returns the Bone2D node assigned to the Jiggle joint at joint_idx.

### get_jiggle_joint_bone_index {#get_jiggle_joint_bone_index}

```gdscript
int get_jiggle_joint_bone_index(joint_idx: int) const
```

Returns the index of the Bone2D node assigned to the Jiggle joint at joint_idx.

### get_jiggle_joint_damping {#get_jiggle_joint_damping}

```gdscript
float get_jiggle_joint_damping(joint_idx: int) const
```

Returns the amount of damping of the Jiggle joint at joint_idx.

### get_jiggle_joint_gravity {#get_jiggle_joint_gravity}

```gdscript
Vector2 get_jiggle_joint_gravity(joint_idx: int) const
```

Returns a Vector2 representing the amount of gravity the Jiggle joint at joint_idx is influenced by.

### get_jiggle_joint_mass {#get_jiggle_joint_mass}

```gdscript
float get_jiggle_joint_mass(joint_idx: int) const
```

Returns the amount of mass of the jiggle joint at joint_idx.

### get_jiggle_joint_override {#get_jiggle_joint_override}

```gdscript
bool get_jiggle_joint_override(joint_idx: int) const
```

Returns a boolean that indicates whether the joint at joint_idx is overriding the default Jiggle joint data defined in the modification.

### get_jiggle_joint_stiffness {#get_jiggle_joint_stiffness}

```gdscript
float get_jiggle_joint_stiffness(joint_idx: int) const
```

Returns the stiffness of the Jiggle joint at joint_idx.

### get_jiggle_joint_use_gravity {#get_jiggle_joint_use_gravity}

```gdscript
bool get_jiggle_joint_use_gravity(joint_idx: int) const
```

Returns a boolean that indicates whether the joint at joint_idx is using gravity or not.

### get_mass {#get_mass}

```gdscript
float get_mass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stiffness {#get_stiffness}

```gdscript
float get_stiffness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_colliders {#get_use_colliders}

```gdscript
bool get_use_colliders() const
```

Returns whether the jiggle modifier is taking physics colliders into account when solving.

### get_use_gravity {#get_use_gravity}

```gdscript
bool get_use_gravity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### reset {#reset}

```gdscript
void reset()
```

Resets the internal jiggle simulation state to the current bone positions, clearing velocity, acceleration, and accumulated forces.

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(collision_mask: int)
```

Sets the collision mask that the Jiggle modifier will use when reacting to colliders, if the Jiggle modifier is set to take colliders into account.

### set_damping {#set_damping}

```gdscript
void set_damping(damping: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity {#set_gravity}

```gdscript
void set_gravity(gravity: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_jiggle_data_chain_length {#set_jiggle_data_chain_length}

```gdscript
void set_jiggle_data_chain_length(length: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_jiggle_joint_bone2d_node {#set_jiggle_joint_bone2d_node}

```gdscript
void set_jiggle_joint_bone2d_node(joint_idx: int, bone2d_node: NodePath)
```

Sets the Bone2D node assigned to the Jiggle joint at joint_idx.

### set_jiggle_joint_bone_index {#set_jiggle_joint_bone_index}

```gdscript
void set_jiggle_joint_bone_index(joint_idx: int, bone_idx: int)
```

Sets the bone index, bone_idx, of the Jiggle joint at joint_idx. When possible, this will also update the bone2d_node of the Jiggle joint based on data provided by the linked skeleton.

### set_jiggle_joint_damping {#set_jiggle_joint_damping}

```gdscript
void set_jiggle_joint_damping(joint_idx: int, damping: float)
```

Sets the amount of damping of the Jiggle joint at joint_idx.

### set_jiggle_joint_gravity {#set_jiggle_joint_gravity}

```gdscript
void set_jiggle_joint_gravity(joint_idx: int, gravity: Vector2)
```

Sets the gravity vector of the Jiggle joint at joint_idx.

### set_jiggle_joint_mass {#set_jiggle_joint_mass}

```gdscript
void set_jiggle_joint_mass(joint_idx: int, mass: float)
```

Sets the of mass of the Jiggle joint at joint_idx.

### set_jiggle_joint_override {#set_jiggle_joint_override}

```gdscript
void set_jiggle_joint_override(joint_idx: int, override: bool)
```

Sets whether the Jiggle joint at joint_idx should override the default Jiggle joint settings. Setting this to true will make the joint use its own settings rather than the default ones attached to the modification.

### set_jiggle_joint_stiffness {#set_jiggle_joint_stiffness}

```gdscript
void set_jiggle_joint_stiffness(joint_idx: int, stiffness: float)
```

Sets the of stiffness of the Jiggle joint at joint_idx.

### set_jiggle_joint_use_gravity {#set_jiggle_joint_use_gravity}

```gdscript
void set_jiggle_joint_use_gravity(joint_idx: int, use_gravity: bool)
```

Sets whether the Jiggle joint at joint_idx should use gravity.

### set_mass {#set_mass}

```gdscript
void set_mass(mass: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stiffness {#set_stiffness}

```gdscript
void set_stiffness(stiffness: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_node {#set_target_node}

```gdscript
void set_target_node(target_nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_colliders {#set_use_colliders}

```gdscript
void set_use_colliders(use_colliders: bool)
```

If true, the Jiggle modifier will take colliders into account, keeping them from entering into these collision objects.

### set_use_gravity {#set_use_gravity}

```gdscript
void set_use_gravity(use_gravity: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### target_nodepath {#target_nodepath}

```gdscript
target_nodepath: NodePath
```

- void set_target_node(value: NodePath)
- NodePath get_target_node()

The NodePath to the node that is the target for the Jiggle modification. This node is what the Jiggle chain will attempt to rotate the bone chain to.

### jiggle_data_chain_length {#jiggle_data_chain_length}

```gdscript
jiggle_data_chain_length: int
```

- void set_jiggle_data_chain_length(value: int)
- int get_jiggle_data_chain_length()

The amount of Jiggle joints in the Jiggle modification.

### stiffness {#stiffness}

```gdscript
stiffness: float
```

- void set_stiffness(value: float)
- float get_stiffness()

The default amount of stiffness assigned to the Jiggle joints, if they are not overridden. Higher values act more like springs, quickly moving into the correct position.

### mass {#mass}

```gdscript
mass: float
```

- void set_mass(value: float)
- float get_mass()

The default amount of mass assigned to the Jiggle joints, if they are not overridden. Higher values lead to faster movements and more overshooting.

### damping {#damping}

```gdscript
damping: float
```

- void set_damping(value: float)
- float get_damping()

The default amount of damping applied to the Jiggle joints, if they are not overridden. Higher values lead to more of the calculated velocity being applied.

### use_gravity {#use_gravity}

```gdscript
use_gravity: bool
```

- void set_use_gravity(value: bool)
- bool get_use_gravity()

Whether the gravity vector, gravity, should be applied to the Jiggle joints, assuming they are not overriding the default settings.

### gravity {#gravity}

```gdscript
gravity: Vector2
```

- void set_gravity(value: Vector2)
- Vector2 get_gravity()

The default amount of gravity applied to the Jiggle joints, if they are not overridden.
