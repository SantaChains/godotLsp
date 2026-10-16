# OpenXRInteractionProfile

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRInteractionProfile**

> This object stores suggested bindings for an interaction profile. Interaction profiles define the metadata for a tracked XR device such as an XR controller.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object stores suggested bindings for an interaction profile. Interaction profiles define the metadata for a tracked XR device such as an XR controller.

For more information see the interaction profiles info in the OpenXR specification [https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#semantic-path-interaction-profiles].

</details>

## 方法

### get_binding {#get_binding}

```gdscript
OpenXRIPBinding get_binding(index: int) const
```

Retrieve the binding at this index.

### get_binding_count {#get_binding_count}

```gdscript
int get_binding_count() const
```

Get the number of bindings in this interaction profile.

### get_binding_modifier {#get_binding_modifier}

```gdscript
OpenXRIPBindingModifier get_binding_modifier(index: int) const
```

Get the OpenXRBindingModifier at this index.

### get_binding_modifier_count {#get_binding_modifier_count}

```gdscript
int get_binding_modifier_count() const
```

Get the number of binding modifiers in this interaction profile.

### get_binding_modifiers {#get_binding_modifiers}

```gdscript
Array get_binding_modifiers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bindings {#get_bindings}

```gdscript
Array get_bindings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interaction_profile_path {#get_interaction_profile_path}

```gdscript
String get_interaction_profile_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_binding_modifiers {#set_binding_modifiers}

```gdscript
void set_binding_modifiers(binding_modifiers: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bindings {#set_bindings}

```gdscript
void set_bindings(bindings: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interaction_profile_path {#set_interaction_profile_path}

```gdscript
void set_interaction_profile_path(interaction_profile_path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### interaction_profile_path {#interaction_profile_path}

```gdscript
interaction_profile_path: String
```

- void set_interaction_profile_path(value: String)
- String get_interaction_profile_path()

The interaction profile path identifying the XR device.

### bindings {#bindings}

```gdscript
bindings: OpenXRIPBinding
```

- void set_bindings(value: Array)
- Array get_bindings()

Action bindings for this interaction profile.

### binding_modifiers {#binding_modifiers}

```gdscript
binding_modifiers: OpenXRIPBindingModifier
```

- void set_binding_modifiers(value: Array)
- Array get_binding_modifiers()

Binding modifiers for this interaction profile.
