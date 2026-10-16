# BaseButton

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **BaseButton**

> BaseButton is an abstract base class for GUI buttons. It doesn't display anything by itself.

<details><summary>官方英文描述（中文精译进行中）</summary>

BaseButton is an abstract base class for GUI buttons. It doesn't display anything by itself.

</details>

## 方法

### _pressed {#_pressed}

```gdscript
void _pressed() virtual
```

Called when the button is pressed. If you need to know the button's pressed state (and toggle_mode is active), use _toggled() instead.

### _toggled {#_toggled}

```gdscript
void _toggled(toggled_on: bool) virtual
```

Called when the button is toggled (only if toggle_mode is active).

### get_action_mode {#get_action_mode}

```gdscript
BaseButton.ActionMode get_action_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_button_group {#get_button_group}

```gdscript
ButtonGroup get_button_group() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_button_mask {#get_button_mask}

```gdscript
bitfield::MouseButtonMask get_button_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_mode {#get_draw_mode}

```gdscript
BaseButton.DrawMode get_draw_mode() const
```

Returns the visual state used to draw the button. This is useful mainly when implementing your own draw code by either overriding _draw() or connecting to "draw" signal. The visual state of the button is defined by the DrawMode enum.

### get_shortcut {#get_shortcut}

```gdscript
Shortcut get_shortcut() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_disabled {#is_disabled}

```gdscript
bool is_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hovered {#is_hovered}

```gdscript
bool is_hovered() const
```

Returns true if the mouse has entered the button and has not left it yet.

### is_keep_pressed_outside {#is_keep_pressed_outside}

```gdscript
bool is_keep_pressed_outside() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pressed {#is_pressed}

```gdscript
bool is_pressed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shortcut_feedback {#is_shortcut_feedback}

```gdscript
bool is_shortcut_feedback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shortcut_in_tooltip_enabled {#is_shortcut_in_tooltip_enabled}

```gdscript
bool is_shortcut_in_tooltip_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_toggle_mode {#is_toggle_mode}

```gdscript
bool is_toggle_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_action_mode {#set_action_mode}

```gdscript
void set_action_mode(mode: BaseButton.ActionMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_button_group {#set_button_group}

```gdscript
void set_button_group(button_group: ButtonGroup)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_button_mask {#set_button_mask}

```gdscript
void set_button_mask(mask: bitfield::MouseButtonMask)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disabled {#set_disabled}

```gdscript
void set_disabled(disabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_pressed_outside {#set_keep_pressed_outside}

```gdscript
void set_keep_pressed_outside(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressed {#set_pressed}

```gdscript
void set_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressed_no_signal {#set_pressed_no_signal}

```gdscript
void set_pressed_no_signal(pressed: bool)
```

Changes the button_pressed state of the button, without emitting toggled. Use when you just want to change the state of the button without sending the pressed event (e.g. when initializing scene). Only works if toggle_mode is true.

Note: This method doesn't unpress other buttons in button_group.

### set_shortcut {#set_shortcut}

```gdscript
void set_shortcut(shortcut: Shortcut)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shortcut_feedback {#set_shortcut_feedback}

```gdscript
void set_shortcut_feedback(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shortcut_in_tooltip {#set_shortcut_in_tooltip}

```gdscript
void set_shortcut_in_tooltip(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_toggle_mode {#set_toggle_mode}

```gdscript
void set_toggle_mode(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### disabled {#disabled}

```gdscript
disabled: bool
```

- void set_disabled(value: bool)
- bool is_disabled()

If true, the button is in disabled state and can't be clicked or toggled.

Note: If the button is disabled while held down, button_up will be emitted.

### toggle_mode {#toggle_mode}

```gdscript
toggle_mode: bool
```

- void set_toggle_mode(value: bool)
- bool is_toggle_mode()

If true, the button is in toggle mode. Makes the button flip state between pressed and unpressed each time its area is clicked.

### button_pressed {#button_pressed}

```gdscript
button_pressed: bool
```

- void set_pressed(value: bool)
- bool is_pressed()

If true, the button's state is pressed. Means the button is pressed down or toggled (if toggle_mode is active). Only works if toggle_mode is true.

Note: Changing the value of button_pressed will result in toggled to be emitted. If you want to change the pressed state without emitting that signal, use set_pressed_no_signal().

### action_mode {#action_mode}

```gdscript
action_mode: int
```

- void set_action_mode(value: ActionMode)
- ActionMode get_action_mode()

Determines when the button is considered clicked.

### button_mask {#button_mask}

```gdscript
button_mask: int
```

- void set_button_mask(value: BitField[MouseButtonMask])
- BitField[MouseButtonMask] get_button_mask()

Binary mask to choose which mouse buttons this button will respond to.

To allow both left-click and right-click, use MOUSE_BUTTON_MASK_LEFT | MOUSE_BUTTON_MASK_RIGHT.

### keep_pressed_outside {#keep_pressed_outside}

```gdscript
keep_pressed_outside: bool
```

- void set_keep_pressed_outside(value: bool)
- bool is_keep_pressed_outside()

If true, the button stays pressed when moving the cursor outside the button while pressing it.

Note: This property only affects the button's visual appearance. Signals will be emitted at the same moment regardless of this property's value.

### button_group {#button_group}

```gdscript
button_group: ButtonGroup
```

- void set_button_group(value: ButtonGroup)
- ButtonGroup get_button_group()

The ButtonGroup associated with the button. Not to be confused with node groups.

Note: The button will be configured as a radio button if a ButtonGroup is assigned to it.

### shortcut {#shortcut}

```gdscript
shortcut: Shortcut
```

- void set_shortcut(value: Shortcut)
- Shortcut get_shortcut()

Shortcut associated to the button.

### shortcut_feedback {#shortcut_feedback}

```gdscript
shortcut_feedback: bool
```

- void set_shortcut_feedback(value: bool)
- bool is_shortcut_feedback()

If true, the button will highlight for a short amount of time when its shortcut is activated. If false and toggle_mode is false, the shortcut will activate without any visual feedback.

### shortcut_in_tooltip {#shortcut_in_tooltip}

```gdscript
shortcut_in_tooltip: bool
```

- void set_shortcut_in_tooltip(value: bool)
- bool is_shortcut_in_tooltip_enabled()

If true, the button will add information about its shortcut in the tooltip. This includes the shortcut's events and its Resource.resource_name. If both events and name are empty, the shortcut will not be included.

Note: This property does nothing when the tooltip control is customized using Control._make_custom_tooltip().

## 信号

### pressed {#pressed}

```gdscript
signal pressed()
```

Emitted when the button is toggled or pressed. This is on button_down if action_mode is ACTION_MODE_BUTTON_PRESS and on button_up otherwise.

If you need to know the button's pressed state (and toggle_mode is active), use toggled instead.

### button_up {#button_up}

```gdscript
signal button_up()
```

Emitted when the button stops being held down.

### button_down {#button_down}

```gdscript
signal button_down()
```

Emitted when the button starts being held down.

### toggled {#toggled}

```gdscript
signal toggled(toggled_on: bool)
```

Emitted when the button was just toggled between pressed and normal states (only if toggle_mode is active). The new state is contained in the toggled_on argument.

## 枚举

### DrawMode {#enum-DrawMode}

```gdscript
enum DrawMode { DRAW_NORMAL = 0, DRAW_PRESSED = 1, DRAW_HOVER = 2, DRAW_DISABLED = 3, DRAW_HOVER_PRESSED = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DRAW_NORMAL {#DRAW_NORMAL}

```gdscript
DRAW_NORMAL = 0
```

The normal state (i.e. not pressed, not hovered, not toggled and enabled) of buttons.

#### DRAW_PRESSED {#DRAW_PRESSED}

```gdscript
DRAW_PRESSED = 1
```

The state of buttons are pressed.

#### DRAW_HOVER {#DRAW_HOVER}

```gdscript
DRAW_HOVER = 2
```

The state of buttons are hovered.

#### DRAW_DISABLED {#DRAW_DISABLED}

```gdscript
DRAW_DISABLED = 3
```

The state of buttons are disabled.

#### DRAW_HOVER_PRESSED {#DRAW_HOVER_PRESSED}

```gdscript
DRAW_HOVER_PRESSED = 4
```

The state of buttons are both hovered and pressed.

---

### ActionMode {#enum-ActionMode}

```gdscript
enum ActionMode { ACTION_MODE_BUTTON_PRESS = 0, ACTION_MODE_BUTTON_RELEASE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACTION_MODE_BUTTON_PRESS {#ACTION_MODE_BUTTON_PRESS}

```gdscript
ACTION_MODE_BUTTON_PRESS = 0
```

Require just a press to consider the button clicked.

#### ACTION_MODE_BUTTON_RELEASE {#ACTION_MODE_BUTTON_RELEASE}

```gdscript
ACTION_MODE_BUTTON_RELEASE = 1
```

Require a press and a subsequent release before considering the button clicked.

---
