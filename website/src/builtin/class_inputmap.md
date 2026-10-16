# InputMap

**继承**：[Object](class_object.md) → **InputMap**

> Manages all InputEventAction which can be created/modified from the project settings menu Project > Project Settings > Input Map or in code with add_action() and action_add_event(). See Node._input().

<details><summary>官方英文描述</summary>

Manages all InputEventAction which can be created/modified from the project settings menu Project > Project Settings > Input Map or in code with add_action() and action_add_event(). See Node._input().

</details>

## 方法

### action_add_event {#action_add_event}

```gdscript
void action_add_event(action: StringName, event: InputEvent)
```

Adds an InputEvent to an action. This InputEvent will trigger the action.

### action_erase_event {#action_erase_event}

```gdscript
void action_erase_event(action: StringName, event: InputEvent)
```

Removes an InputEvent from an action.

### action_erase_events {#action_erase_events}

```gdscript
void action_erase_events(action: StringName)
```

Removes all events from an action.

### action_get_deadzone {#action_get_deadzone}

```gdscript
float action_get_deadzone(action: StringName)
```

Returns a deadzone value for the action.

### action_get_events {#action_get_events}

```gdscript
typedarray::InputEvent action_get_events(action: StringName)
```

Returns an array of InputEvents associated with a given action.

Note: When used in the editor (e.g. a tool script or EditorPlugin), this method will return events for the editor action. If you want to access your project's input binds from the editor, read the input/* settings from ProjectSettings.

### action_has_event {#action_has_event}

```gdscript
bool action_has_event(action: StringName, event: InputEvent)
```

Returns true if the action has the given InputEvent associated with it.

### action_set_deadzone {#action_set_deadzone}

```gdscript
void action_set_deadzone(action: StringName, deadzone: float)
```

Sets a deadzone value for the action.

### add_action {#add_action}

```gdscript
void add_action(action: StringName, deadzone: float = 0.2)
```

Adds an empty action to the InputMap with a configurable deadzone.

An InputEvent can then be added to this action with action_add_event().

### erase_action {#erase_action}

```gdscript
void erase_action(action: StringName)
```

Removes an action from the InputMap.

### event_is_action {#event_is_action}

```gdscript
bool event_is_action(event: InputEvent, action: StringName, exact_match: bool = false) const
```

Returns true if the given event is part of an existing action. This method ignores keyboard modifiers if the given InputEvent is not pressed (for proper release detection). See action_has_event() if you don't want this behavior.

If exact_match is false, it ignores additional input modifiers for InputEventKey and InputEventMouseButton events, and the direction for InputEventJoypadMotion events.

### get_action_description {#get_action_description}

```gdscript
String get_action_description(action: StringName) const
```

Returns the human-readable description of the given action.

### get_actions {#get_actions}

```gdscript
typedarray::StringName get_actions()
```

Returns an array of all actions in the InputMap.

### has_action {#has_action}

```gdscript
bool has_action(action: StringName) const
```

Returns true if the InputMap has a registered action with the given name.

### load_from_project_settings {#load_from_project_settings}

```gdscript
void load_from_project_settings()
```

Clears all InputEventAction in the InputMap and load it anew from ProjectSettings.

## 信号

### project_settings_loaded {#project_settings_loaded}

```gdscript
signal project_settings_loaded()
```

Emitted when the ProjectSettings InputMap has been loaded.
