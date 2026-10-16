# PhysicalBone2D

**继承**：[RigidBody2D](class_rigidbody2d.md) → [PhysicsBody2D](class_physicsbody2d.md) → [CollisionObject2D](class_collisionobject2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **PhysicalBone2D**

> The PhysicalBone2D node is a RigidBody2D-based node that can be used to make Bone2Ds in a Skeleton2D react to physics.

<details><summary>官方英文描述</summary>

The PhysicalBone2D node is a RigidBody2D-based node that can be used to make Bone2Ds in a Skeleton2D react to physics.

Note: To make the Bone2Ds visually follow the PhysicalBone2D node, use a SkeletonModification2DPhysicalBones modification on the Skeleton2D parent.

Note: The PhysicalBone2D node does not automatically create a Joint2D node to keep PhysicalBone2D nodes together. They must be created manually. For most cases, you want to use a PinJoint2D node. The PhysicalBone2D node will automatically configure the Joint2D node once it's been added as a child node.

</details>

## 方法

### get_auto_configure_joint {#get_auto_configure_joint}

```gdscript
bool get_auto_configure_joint() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone2d_index {#get_bone2d_index}

```gdscript
int get_bone2d_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone2d_nodepath {#get_bone2d_nodepath}

```gdscript
NodePath get_bone2d_nodepath() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_follow_bone_when_simulating {#get_follow_bone_when_simulating}

```gdscript
bool get_follow_bone_when_simulating() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint {#get_joint}

```gdscript
Joint2D get_joint() const
```

Returns the first Joint2D child node, if one exists. This is mainly a helper function to make it easier to get the Joint2D that the PhysicalBone2D is autoconfiguring.

### get_simulate_physics {#get_simulate_physics}

```gdscript
bool get_simulate_physics() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_simulating_physics {#is_simulating_physics}

```gdscript
bool is_simulating_physics() const
```

Returns a boolean that indicates whether the PhysicalBone2D is running and simulating using the Godot 2D physics engine. When true, the PhysicalBone2D node is using physics.

### set_auto_configure_joint {#set_auto_configure_joint}

```gdscript
void set_auto_configure_joint(auto_configure_joint: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone2d_index {#set_bone2d_index}

```gdscript
void set_bone2d_index(bone_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone2d_nodepath {#set_bone2d_nodepath}

```gdscript
void set_bone2d_nodepath(nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_follow_bone_when_simulating {#set_follow_bone_when_simulating}

```gdscript
void set_follow_bone_when_simulating(follow_bone: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_simulate_physics {#set_simulate_physics}

```gdscript
void set_simulate_physics(simulate_physics: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### bone2d_nodepath {#bone2d_nodepath}

```gdscript
bone2d_nodepath: NodePath
```

- void set_bone2d_nodepath(value: NodePath)
- NodePath get_bone2d_nodepath()

The NodePath to the Bone2D that this PhysicalBone2D should simulate.

### bone2d_index {#bone2d_index}

```gdscript
bone2d_index: int
```

- void set_bone2d_index(value: int)
- int get_bone2d_index()

The index of the Bone2D that this PhysicalBone2D should simulate.

### auto_configure_joint {#auto_configure_joint}

```gdscript
auto_configure_joint: bool
```

- void set_auto_configure_joint(value: bool)
- bool get_auto_configure_joint()

If true, the PhysicalBone2D will automatically configure the first Joint2D child node. The automatic configuration is limited to setting up the node properties and positioning the Joint2D.

### simulate_physics {#simulate_physics}

```gdscript
simulate_physics: bool
```

- void set_simulate_physics(value: bool)
- bool get_simulate_physics()

If true, the PhysicalBone2D will start simulating using physics. If false, the PhysicalBone2D will follow the transform of the Bone2D node.

Note: To have the Bone2Ds visually follow the PhysicalBone2D, use a SkeletonModification2DPhysicalBones modification on the Skeleton2D node with the Bone2D nodes.

### follow_bone_when_simulating {#follow_bone_when_simulating}

```gdscript
follow_bone_when_simulating: bool
```

- void set_follow_bone_when_simulating(value: bool)
- bool get_follow_bone_when_simulating()

If true, the PhysicalBone2D will keep the transform of the bone it is bound to when simulating physics.
