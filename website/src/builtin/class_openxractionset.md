# OpenXRActionSet

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRActionSet**

> Action sets in OpenXR define a collection of actions that can be activated in unison. This allows games to easily change between different states that require different inputs or need to reinterpret inputs. For instance we could have an action set that is active when a menu is open, an action set that is active when the player is freely walking around and an action set that is active when the player is controlling a vehicle.

<details><summary>官方英文描述</summary>

Action sets in OpenXR define a collection of actions that can be activated in unison. This allows games to easily change between different states that require different inputs or need to reinterpret inputs. For instance we could have an action set that is active when a menu is open, an action set that is active when the player is freely walking around and an action set that is active when the player is controlling a vehicle.

Action sets can contain the same action with the same name, if such action sets are active at the same time the action set with the highest priority defines which binding is active.

</details>

## 方法

### add_action {#add_action}

```gdscript
void add_action(action: OpenXRAction)
```

Add an action to this action set.

### get_action_count {#get_action_count}

```gdscript
int get_action_count() const
```

Retrieve the number of actions in our action set.

### get_actions {#get_actions}

```gdscript
Array get_actions() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_localized_name {#get_localized_name}

```gdscript
String get_localized_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_priority {#get_priority}

```gdscript
int get_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_action {#remove_action}

```gdscript
void remove_action(action: OpenXRAction)
```

Remove an action from this action set.

### set_actions {#set_actions}

```gdscript
void set_actions(actions: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_localized_name {#set_localized_name}

```gdscript
void set_localized_name(localized_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_priority {#set_priority}

```gdscript
void set_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### localized_name {#localized_name}

```gdscript
localized_name: String
```

- void set_localized_name(value: String)
- String get_localized_name()

The localized name of this action set.

### priority {#priority}

```gdscript
priority: int
```

- void set_priority(value: int)
- int get_priority()

The priority for this action set.

### actions {#actions}

```gdscript
actions: OpenXRAction
```

- void set_actions(value: Array)
- Array get_actions()

Collection of actions for this action set.
