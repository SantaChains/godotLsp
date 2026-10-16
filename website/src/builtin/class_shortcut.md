# Shortcut

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Shortcut**

> Shortcuts (also known as hotkeys) are containers of InputEvent resources. They are commonly used to interact with a Control element from an InputEvent.

<details><summary>官方英文描述</summary>

Shortcuts (also known as hotkeys) are containers of InputEvent resources. They are commonly used to interact with a Control element from an InputEvent.

One shortcut can contain multiple InputEvent resources, making it possible to trigger one action with multiple different inputs.

Example: Capture the Ctrl + S shortcut using a Shortcut resource:

```
extends Node
var save_shortcut = Shortcut.new()
func _ready():
    var key_event = InputEventKey.new()
    key_event.keycode = KEY_S
    key_event.ctrl_pressed = true
    key_event.command_or_control_autoremap = true # Swaps Ctrl for Command on Mac.
    save_shortcut.events = [key_event]
func _input(event):
    if save_shortcut.matches_event(event) and event.is_pressed() and not event.is_echo():
        print("Save shortcut pressed!")
        get_viewport().set_input_as_handled()
```

```
using Godot;
public partial class MyNode : Node
{
    private readonly Shortcut _saveShortcut = new Shortcut();
    public override void _Ready()
    {
        InputEventKey keyEvent = new InputEventKey
        {
            Keycode = Key.S,
            CtrlPressed = true,
            CommandOrControlAutoremap = true, // Swaps Ctrl for Command on Mac.
        };
        _saveShortcut.Events = [keyEvent];
    }
    public override void _Input(InputEvent @event)
    {
        if (@event is InputEventKey keyEvent &&
            _saveShortcut.MatchesEvent(@event) &&
            keyEvent.Pressed && !keyEvent.Echo)
        {
            GD.Print("Save shortcut pressed!");
            GetViewport().SetInputAsHandled();
        }
    }
}
```

</details>

## 方法

### get_as_text {#get_as_text}

```gdscript
String get_as_text() const
```

Returns the shortcut's first valid InputEvent as a String.

### get_events {#get_events}

```gdscript
Array get_events() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_valid_event {#has_valid_event}

```gdscript
bool has_valid_event() const
```

Returns whether events contains an InputEvent which is valid.

### matches_event {#matches_event}

```gdscript
bool matches_event(event: InputEvent) const
```

Returns whether any InputEvent in events equals event. This uses InputEvent.is_match() to compare events.

### set_events {#set_events}

```gdscript
void set_events(events: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### events {#events}

```gdscript
events: typedarray::24/17:InputEvent
```

- void set_events(value: Array)
- Array get_events()

The shortcut's InputEvent array.

Generally the InputEvent used is an InputEventKey, though it can be any InputEvent, including an InputEventAction.
