# PhysicalBoneSimulator3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **PhysicalBoneSimulator3D**

> Node that can be the parent of PhysicalBone3D and can apply the simulation results to Skeleton3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Node that can be the parent of PhysicalBone3D and can apply the simulation results to Skeleton3D.

</details>

## 方法

### is_simulating_physics {#is_simulating_physics}

```gdscript
bool is_simulating_physics() const
```

Returns a boolean that indicates whether the PhysicalBoneSimulator3D is running and simulating.

### physical_bones_add_collision_exception {#physical_bones_add_collision_exception}

```gdscript
void physical_bones_add_collision_exception(exception: RID)
```

Adds a collision exception to the physical bone.

Works just like the RigidBody3D node.

### physical_bones_remove_collision_exception {#physical_bones_remove_collision_exception}

```gdscript
void physical_bones_remove_collision_exception(exception: RID)
```

Removes a collision exception to the physical bone.

Works just like the RigidBody3D node.

### physical_bones_start_simulation {#physical_bones_start_simulation}

```gdscript
void physical_bones_start_simulation(bones: typedarray::StringName = [])
```

Tells the PhysicalBone3D nodes in the Skeleton to start simulating and reacting to the physics world.

Optionally, a list of bone names can be passed-in, allowing only the passed-in bones to be simulated.

### physical_bones_stop_simulation {#physical_bones_stop_simulation}

```gdscript
void physical_bones_stop_simulation()
```

Tells the PhysicalBone3D nodes in the Skeleton to stop simulating.
