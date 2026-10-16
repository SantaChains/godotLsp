# InputEventKey

**继承**：[InputEventWithModifiers](class_inputeventwithmodifiers.md) → [InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventKey**

> An input event for keys on a keyboard. Supports key presses, key releases and echo events. It can also be received in Node._unhandled_key_input().

<details><summary>官方英文描述</summary>

An input event for keys on a keyboard. Supports key presses, key releases and echo events. It can also be received in Node._unhandled_key_input().

Note: Events received from the keyboard usually have all properties set. Event mappings should have only one of the keycode, physical_keycode or unicode set.

When events are compared, properties are checked in the following priority - keycode, physical_keycode and unicode. Events with the first matching value will be considered equal.

</details>

## 方法

### as_text_key_label {#as_text_key_label}

```gdscript
String as_text_key_label() const
```

Returns a String representation of the event's key_label and modifiers.

### as_text_keycode {#as_text_keycode}

```gdscript
String as_text_keycode() const
```

Returns a String representation of the event's keycode and modifiers.

### as_text_location {#as_text_location}

```gdscript
String as_text_location() const
```

Returns a String representation of the event's location. This will be a blank string if the event is not specific to a location.

### as_text_physical_keycode {#as_text_physical_keycode}

```gdscript
String as_text_physical_keycode() const
```

Returns a String representation of the event's physical_keycode and modifiers.

### get_key_label {#get_key_label}

```gdscript
Key get_key_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_key_label_with_modifiers {#get_key_label_with_modifiers}

```gdscript
Key get_key_label_with_modifiers() const
```

Returns the localized key label combined with modifier keys such as Shift or Alt. See also InputEventWithModifiers.

To get a human-readable representation of the InputEventKey with modifiers, use OS.get_keycode_string(event.get_key_label_with_modifiers()) where event is the InputEventKey.

### get_keycode {#get_keycode}

```gdscript
Key get_keycode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_keycode_with_modifiers {#get_keycode_with_modifiers}

```gdscript
Key get_keycode_with_modifiers() const
```

Returns the Latin keycode combined with modifier keys such as Shift or Alt. See also InputEventWithModifiers.

To get a human-readable representation of the InputEventKey with modifiers, use OS.get_keycode_string(event.get_keycode_with_modifiers()) where event is the InputEventKey.

### get_location {#get_location}

```gdscript
KeyLocation get_location() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physical_keycode {#get_physical_keycode}

```gdscript
Key get_physical_keycode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physical_keycode_with_modifiers {#get_physical_keycode_with_modifiers}

```gdscript
Key get_physical_keycode_with_modifiers() const
```

Returns the physical keycode combined with modifier keys such as Shift or Alt. See also InputEventWithModifiers.

To get a human-readable representation of the InputEventKey with modifiers, use OS.get_keycode_string(event.get_physical_keycode_with_modifiers()) where event is the InputEventKey.

### get_unicode {#get_unicode}

```gdscript
int get_unicode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_echo {#set_echo}

```gdscript
void set_echo(echo: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_key_label {#set_key_label}

```gdscript
void set_key_label(key_label: Key)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keycode {#set_keycode}

```gdscript
void set_keycode(keycode: Key)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_location {#set_location}

```gdscript
void set_location(location: KeyLocation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physical_keycode {#set_physical_keycode}

```gdscript
void set_physical_keycode(physical_keycode: Key)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressed {#set_pressed}

```gdscript
void set_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_unicode {#set_unicode}

```gdscript
void set_unicode(unicode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### pressed {#pressed}

```gdscript
pressed: bool
```

- void set_pressed(value: bool)
- bool is_pressed()

If true, the key's state is pressed. If false, the key's state is released.

### keycode {#keycode}

```gdscript
keycode: int
```

- void set_keycode(value: Key)
- Key get_keycode()

Latin label printed on the key in the current keyboard layout, which corresponds to one of the Key constants. Key codes are meant for shortcuts expressed with a standard Latin keyboard, such as Ctrl + S for a "Save" shortcut.

To get a human-readable representation of the InputEventKey, use OS.get_keycode_string(event.keycode) where event is the InputEventKey.

```
+-----+ +-----+
| Q   | | Q   | - "Q" - keycode
|   Й | |  ض | - "Й" and "ض" - key_label
+-----+ +-----+
```

### physical_keycode {#physical_keycode}

```gdscript
physical_keycode: int
```

- void set_physical_keycode(value: Key)
- Key get_physical_keycode()

Represents the physical location of a key on the 101/102-key US QWERTY keyboard, which corresponds to one of the Key constants. Physical key codes meant for game input, such as WASD movement, where only the location of the keys is important.

To get a human-readable representation of the InputEventKey, use OS.get_keycode_string() in combination with DisplayServer.keyboard_get_keycode_from_physical() or DisplayServer.keyboard_get_label_from_physical():

```
func _input(event):
    if event is InputEventKey:
        var keycode = DisplayServer.keyboard_get_keycode_from_physical(event.physical_keycode)
        var label = DisplayServer.keyboard_get_label_from_physical(event.physical_keycode)
        print(OS.get_keycode_string(keycode))
        print(OS.get_keycode_string(label))
```

```
public override void _Input(InputEvent @event)
{
    if (@event is InputEventKey inputEventKey)
    {
        var keycode = DisplayServer.KeyboardGetKeycodeFromPhysical(inputEventKey.PhysicalKeycode);
        var label = DisplayServer.KeyboardGetLabelFromPhysical(inputEventKey.PhysicalKeycode);
        GD.Print(OS.GetKeycodeString(keycode));
        GD.Print(OS.GetKeycodeString(label));
    }
}
```

### key_label {#key_label}

```gdscript
key_label: int
```

- void set_key_label(value: Key)
- Key get_key_label()

Represents the localized label printed on the key in the current keyboard layout, which corresponds to one of the Key constants or any valid Unicode character. Key labels are meant for key prompts.

For keyboard layouts with a single label on the key, it is equivalent to keycode.

To get a human-readable representation of the InputEventKey, use OS.get_keycode_string(event.key_label) where event is the InputEventKey.

```
+-----+ +-----+
| Q   | | Q   | - "Q" - keycode
|   Й | |  ض | - "Й" and "ض" - key_label
+-----+ +-----+
```

### unicode {#unicode}

```gdscript
unicode: int
```

- void set_unicode(value: int)
- int get_unicode()

The key Unicode character code (when relevant), shifted by modifier keys. Unicode character codes for composite characters and complex scripts may not be available unless IME input mode is active. See Window.set_ime_active() for more information. Unicode character codes are meant for text input.

Note: This property is set by the engine only for a pressed event. If the event is sent by an IME or a virtual keyboard, no corresponding key released event is sent.

### location {#location}

```gdscript
location: int
```

- void set_location(value: KeyLocation)
- KeyLocation get_location()

Represents the location of a key which has both left and right versions, such as Shift or Alt.

### echo {#echo}

```gdscript
echo: bool
```

- void set_echo(value: bool)
- bool is_echo()

If true, the key was already pressed before this event. An echo event is a repeated key event sent when the user is holding down the key.

Note: The rate at which echo events are sent is typically around 20 events per second (after holding down the key for roughly half a second). However, the key repeat delay/speed can be changed by the user or disabled entirely in the operating system settings. To ensure your project works correctly on all configurations, do not assume the user has a specific key repeat configuration in your project's behavior.
