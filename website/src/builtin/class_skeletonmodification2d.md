# SkeletonModification2D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2D**

> This resource provides an interface that can be expanded so code that operates on Bone2D nodes in a Skeleton2D can be mixed and matched together to create complex interactions.

<details><summary>官方英文描述（中文精译进行中）</summary>

This resource provides an interface that can be expanded so code that operates on Bone2D nodes in a Skeleton2D can be mixed and matched together to create complex interactions.

This is used to provide Godot with a flexible and powerful Inverse Kinematics solution that can be adapted for many different uses.

</details>

## 方法

### _draw_editor_gizmo {#_draw_editor_gizmo}

```gdscript
void _draw_editor_gizmo() virtual
```

Used for drawing editor-only modification gizmos. This function will only be called in the Godot editor and can be overridden to draw custom gizmos.

Note: You will need to use the Skeleton2D from SkeletonModificationStack2D.get_skeleton() and it's draw functions, as the SkeletonModification2D resource cannot draw on its own.

### _execute {#_execute}

```gdscript
void _execute(delta: float) virtual
```

Executes the given modification. This is where the modification performs whatever function it is designed to do.

### _setup_modification {#_setup_modification}

```gdscript
void _setup_modification(modification_stack: SkeletonModificationStack2D) virtual
```

Called when the modification is setup. This is where the modification performs initialization.

### clamp_angle {#clamp_angle}

```gdscript
float clamp_angle(angle: float, min: float, max: float, invert: bool)
```

Takes an angle and clamps it so it is within the passed-in min and max range. invert will inversely clamp the angle, clamping it to the range outside of the given bounds.

### get_editor_draw_gizmo {#get_editor_draw_gizmo}

```gdscript
bool get_editor_draw_gizmo() const
```

Returns whether this modification will call _draw_editor_gizmo() in the Godot editor to draw modification-specific gizmos.

### get_enabled {#get_enabled}

```gdscript
bool get_enabled()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_execution_mode {#get_execution_mode}

```gdscript
int get_execution_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_is_setup {#get_is_setup}

```gdscript
bool get_is_setup() const
```

Returns whether this modification has been successfully setup or not.

### get_modification_stack {#get_modification_stack}

```gdscript
SkeletonModificationStack2D get_modification_stack()
```

Returns the SkeletonModificationStack2D that this modification is bound to. Through the modification stack, you can access the Skeleton2D the modification is operating on.

### set_editor_draw_gizmo {#set_editor_draw_gizmo}

```gdscript
void set_editor_draw_gizmo(draw_gizmo: bool)
```

Sets whether this modification will call _draw_editor_gizmo() in the Godot editor to draw modification-specific gizmos.

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_execution_mode {#set_execution_mode}

```gdscript
void set_execution_mode(execution_mode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_is_setup {#set_is_setup}

```gdscript
void set_is_setup(is_setup: bool)
```

Manually allows you to set the setup state of the modification. This function should only rarely be used, as the SkeletonModificationStack2D the modification is bound to should handle setting the modification up.

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool get_enabled()

If true, the modification's _execute() function will be called by the SkeletonModificationStack2D.

### execution_mode {#execution_mode}

```gdscript
execution_mode: int
```

- void set_execution_mode(value: int)
- int get_execution_mode()

The execution mode for the modification. This tells the modification stack when to execute the modification. Some modifications have settings that are only available in certain execution modes.
