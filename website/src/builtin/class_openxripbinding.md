# OpenXRIPBinding

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRIPBinding**

> This binding resource binds an OpenXRAction to an input or output. As most controllers have left hand and right versions that are handled by the same interaction profile we can specify multiple bindings. For instance an action "Fire" could be bound to both "/user/hand/left/input/trigger" and "/user/hand/right/input/trigger". This would require two binding entries.

<details><summary>官方英文描述</summary>

This binding resource binds an OpenXRAction to an input or output. As most controllers have left hand and right versions that are handled by the same interaction profile we can specify multiple bindings. For instance an action "Fire" could be bound to both "/user/hand/left/input/trigger" and "/user/hand/right/input/trigger". This would require two binding entries.

</details>

## 方法

### add_path {#add_path}

```gdscript
void add_path(path: String)
```

Deprecated: Binding is for a single path.

Add an input/output path to this binding.

### get_action {#get_action}

```gdscript
OpenXRAction get_action() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_binding_modifier {#get_binding_modifier}

```gdscript
OpenXRActionBindingModifier get_binding_modifier(index: int) const
```

Get the OpenXRBindingModifier at this index.

### get_binding_modifier_count {#get_binding_modifier_count}

```gdscript
int get_binding_modifier_count() const
```

Get the number of binding modifiers for this binding.

### get_binding_modifiers {#get_binding_modifiers}

```gdscript
Array get_binding_modifiers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_binding_path {#get_binding_path}

```gdscript
String get_binding_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_count {#get_path_count}

```gdscript
int get_path_count() const
```

Deprecated: Binding is for a single path.

Get the number of input/output paths in this binding.

### get_paths {#get_paths}

```gdscript
PackedStringArray get_paths() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_path {#has_path}

```gdscript
bool has_path(path: String) const
```

Deprecated: Binding is for a single path.

Returns true if this input/output path is part of this binding.

### remove_path {#remove_path}

```gdscript
void remove_path(path: String)
```

Deprecated: Binding is for a single path.

Removes this input/output path from this binding.

### set_action {#set_action}

```gdscript
void set_action(action: OpenXRAction)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_binding_modifiers {#set_binding_modifiers}

```gdscript
void set_binding_modifiers(binding_modifiers: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_binding_path {#set_binding_path}

```gdscript
void set_binding_path(binding_path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_paths {#set_paths}

```gdscript
void set_paths(paths: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### action {#action}

```gdscript
action: OpenXRAction
```

- void set_action(value: OpenXRAction)
- OpenXRAction get_action()

OpenXRAction that is bound to binding_path.

### binding_path {#binding_path}

```gdscript
binding_path: String
```

- void set_binding_path(value: String)
- String get_binding_path()

Binding path that defines the input or output bound to action.

Note: Binding paths are suggestions, an XR runtime may choose to bind the action to a different input or output emulating this input or output.

### binding_modifiers {#binding_modifiers}

```gdscript
binding_modifiers: OpenXRActionBindingModifier
```

- void set_binding_modifiers(value: Array)
- Array get_binding_modifiers()

Binding modifiers for this binding.

### paths {#paths}

```gdscript
paths: PackedStringArray
```

- void set_paths(value: PackedStringArray)
- PackedStringArray get_paths()

Deprecated: Use binding_path instead.

Paths that define the inputs or outputs bound on the device.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedStringArray for more details.
