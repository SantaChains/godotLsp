# OpenXRAction

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRAction**

> This resource defines an OpenXR action. Actions can be used both for inputs (buttons, joysticks, triggers, etc.) and outputs (haptics).

<details><summary>官方英文描述（中文精译进行中）</summary>

This resource defines an OpenXR action. Actions can be used both for inputs (buttons, joysticks, triggers, etc.) and outputs (haptics).

OpenXR performs automatic conversion between action type and input type whenever possible. An analog trigger bound to a boolean action will thus return false if the trigger is depressed and true if pressed fully.

Actions are not directly bound to specific devices, instead OpenXR recognizes a limited number of top level paths that identify devices by usage. We can restrict which devices an action can be bound to by these top level paths. For instance an action that should only be used for hand held controllers can have the top level paths "/user/hand/left" and "/user/hand/right" associated with them. See the reserved path section in the OpenXR specification [https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#semantic-path-reserved] for more info on the top level paths.

Note that the name of the resource is used to register the action with.

</details>

## 方法

### get_action_type {#get_action_type}

```gdscript
OpenXRAction.ActionType get_action_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_localized_name {#get_localized_name}

```gdscript
String get_localized_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_toplevel_paths {#get_toplevel_paths}

```gdscript
PackedStringArray get_toplevel_paths() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_action_type {#set_action_type}

```gdscript
void set_action_type(action_type: OpenXRAction.ActionType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_localized_name {#set_localized_name}

```gdscript
void set_localized_name(localized_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_toplevel_paths {#set_toplevel_paths}

```gdscript
void set_toplevel_paths(toplevel_paths: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### localized_name {#localized_name}

```gdscript
localized_name: String
```

- void set_localized_name(value: String)
- String get_localized_name()

The localized description of this action.

### action_type {#action_type}

```gdscript
action_type: int
```

- void set_action_type(value: ActionType)
- ActionType get_action_type()

The type of action.

### toplevel_paths {#toplevel_paths}

```gdscript
toplevel_paths: PackedStringArray
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### ActionType {#enum-ActionType}

```gdscript
enum ActionType { OPENXR_ACTION_BOOL = 0, OPENXR_ACTION_FLOAT = 1, OPENXR_ACTION_VECTOR2 = 2, OPENXR_ACTION_POSE = 3 }
```

ActionType OPENXR_ACTION_BOOL = 0

This action provides a boolean value.

ActionType OPENXR_ACTION_FLOAT = 1

This action provides a float value between 0.0 and 1.0 for any analog input such as triggers.

ActionType OPENXR_ACTION_VECTOR2 = 2

This action provides a Vector2 value and can be bound to embedded trackpads and joysticks.

ActionType OPENXR_ACTION_POSE = 3

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### OPENXR_ACTION_BOOL {#OPENXR_ACTION_BOOL}

```gdscript
OPENXR_ACTION_BOOL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OPENXR_ACTION_FLOAT {#OPENXR_ACTION_FLOAT}

```gdscript
OPENXR_ACTION_FLOAT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OPENXR_ACTION_VECTOR2 {#OPENXR_ACTION_VECTOR2}

```gdscript
OPENXR_ACTION_VECTOR2 = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OPENXR_ACTION_POSE {#OPENXR_ACTION_POSE}

```gdscript
OPENXR_ACTION_POSE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
