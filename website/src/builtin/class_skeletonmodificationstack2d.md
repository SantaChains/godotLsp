# SkeletonModificationStack2D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModificationStack2D**

> This resource is used by the Skeleton and holds a stack of SkeletonModification2Ds.

<details><summary>官方英文描述</summary>

This resource is used by the Skeleton and holds a stack of SkeletonModification2Ds.

This controls the order of the modifications and how they are applied. Modification order is especially important for full-body IK setups, as you need to execute the modifications in the correct order to get the desired results. For example, you want to execute a modification on the spine before the arms on a humanoid skeleton.

This resource also controls how strongly all of the modifications are applied to the Skeleton2D.

</details>

## 方法

### add_modification {#add_modification}

```gdscript
void add_modification(modification: SkeletonModification2D)
```

Adds the passed-in SkeletonModification2D to the stack.

### delete_modification {#delete_modification}

```gdscript
void delete_modification(mod_idx: int)
```

Deletes the SkeletonModification2D at the index position mod_idx, if it exists.

### enable_all_modifications {#enable_all_modifications}

```gdscript
void enable_all_modifications(enabled: bool)
```

Enables all SkeletonModification2Ds in the stack.

### execute {#execute}

```gdscript
void execute(delta: float, execution_mode: int)
```

Executes all of the SkeletonModification2Ds in the stack that use the same execution mode as the passed-in execution_mode, starting from index 0 to modification_count.

Note: The order of the modifications can matter depending on the modifications. For example, modifications on a spine should operate before modifications on the arms in order to get proper results.

### get_enabled {#get_enabled}

```gdscript
bool get_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_is_setup {#get_is_setup}

```gdscript
bool get_is_setup() const
```

Returns a boolean that indicates whether the modification stack is setup and can execute.

### get_modification {#get_modification}

```gdscript
SkeletonModification2D get_modification(mod_idx: int) const
```

Returns the SkeletonModification2D at the passed-in index, mod_idx.

### get_modification_count {#get_modification_count}

```gdscript
int get_modification_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton {#get_skeleton}

```gdscript
Skeleton2D get_skeleton() const
```

Returns the Skeleton2D node that the SkeletonModificationStack2D is bound to.

### get_strength {#get_strength}

```gdscript
float get_strength() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_modification {#set_modification}

```gdscript
void set_modification(mod_idx: int, modification: SkeletonModification2D)
```

Sets the modification at mod_idx to the passed-in modification, modification.

### set_modification_count {#set_modification_count}

```gdscript
void set_modification_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_strength {#set_strength}

```gdscript
void set_strength(strength: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### setup {#setup}

```gdscript
void setup()
```

Sets up the modification stack so it can execute. This function should be called by Skeleton2D and shouldn't be manually called unless you know what you are doing.

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool get_enabled()

If true, the modification's in the stack will be called. This is handled automatically through the Skeleton2D node.

### strength {#strength}

```gdscript
strength: float
```

- void set_strength(value: float)
- float get_strength()

The interpolation strength of the modifications in stack. A value of 0 will make it where the modifications are not applied, a strength of 0.5 will be half applied, and a strength of 1 will allow the modifications to be fully applied and override the Skeleton2D Bone2D poses.

### modification_count {#modification_count}

```gdscript
modification_count: int
```

- void set_modification_count(value: int)
- int get_modification_count()

The number of modifications in the stack.
