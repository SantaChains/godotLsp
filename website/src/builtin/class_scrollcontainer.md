# ScrollContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ScrollContainer**

> A container used to provide a child control with scrollbars when needed. Scrollbars will automatically be drawn at the right (for vertical) or bottom (for horizontal) and will enable dragging to move the viewable Control (and its children) within the ScrollContainer. Scrollbars will also automatically resize the grabber based on the Control.custom_minimum_size of the Control relative to the ScrollContainer.

<details><summary>官方英文描述</summary>

A container used to provide a child control with scrollbars when needed. Scrollbars will automatically be drawn at the right (for vertical) or bottom (for horizontal) and will enable dragging to move the viewable Control (and its children) within the ScrollContainer. Scrollbars will also automatically resize the grabber based on the Control.custom_minimum_size of the Control relative to the ScrollContainer.

</details>

## 方法

### ensure_control_visible {#ensure_control_visible}

```gdscript
void ensure_control_visible(control: Control)
```

Ensures the given control is visible (must be a direct or indirect child of the ScrollContainer). Used by follow_focus.

Note: This will not work on a node that was just added during the same frame. If you want to scroll to a newly added child, you must wait until the next frame using SceneTree.process_frame:

```gdscript
add_child(child_node)
await get_tree().process_frame
ensure_control_visible(child_node)
```

### get_deadzone {#get_deadzone}

```gdscript
int get_deadzone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_focus_border {#get_draw_focus_border}

```gdscript
bool get_draw_focus_border()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_scroll {#get_h_scroll}

```gdscript
int get_h_scroll() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_scroll_bar {#get_h_scroll_bar}

```gdscript
HScrollBar get_h_scroll_bar()
```

Returns the horizontal scrollbar HScrollBar of this ScrollContainer.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to disable or hide a scrollbar, you can use horizontal_scroll_mode.

### get_horizontal_custom_step {#get_horizontal_custom_step}

```gdscript
float get_horizontal_custom_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_horizontal_scroll_mode {#get_horizontal_scroll_mode}

```gdscript
ScrollContainer.ScrollMode get_horizontal_scroll_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_hint_mode {#get_scroll_hint_mode}

```gdscript
ScrollContainer.ScrollHintMode get_scroll_hint_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_scroll {#get_v_scroll}

```gdscript
int get_v_scroll() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_scroll_bar {#get_v_scroll_bar}

```gdscript
VScrollBar get_v_scroll_bar()
```

Returns the vertical scrollbar VScrollBar of this ScrollContainer.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to disable or hide a scrollbar, you can use vertical_scroll_mode.

### get_vertical_custom_step {#get_vertical_custom_step}

```gdscript
float get_vertical_custom_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertical_scroll_mode {#get_vertical_scroll_mode}

```gdscript
ScrollContainer.ScrollMode get_vertical_scroll_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_following_focus {#is_following_focus}

```gdscript
bool is_following_focus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scroll_hint_tiled {#is_scroll_hint_tiled}

```gdscript
bool is_scroll_hint_tiled()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scroll_horizontal_by_default {#is_scroll_horizontal_by_default}

```gdscript
bool is_scroll_horizontal_by_default() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deadzone {#set_deadzone}

```gdscript
void set_deadzone(deadzone: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_focus_border {#set_draw_focus_border}

```gdscript
void set_draw_focus_border(draw: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_follow_focus {#set_follow_focus}

```gdscript
void set_follow_focus(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_h_scroll {#set_h_scroll}

```gdscript
void set_h_scroll(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_custom_step {#set_horizontal_custom_step}

```gdscript
void set_horizontal_custom_step(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_scroll_mode {#set_horizontal_scroll_mode}

```gdscript
void set_horizontal_scroll_mode(enable: ScrollContainer.ScrollMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_hint_mode {#set_scroll_hint_mode}

```gdscript
void set_scroll_hint_mode(scroll_hint_mode: ScrollContainer.ScrollHintMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_horizontal_by_default {#set_scroll_horizontal_by_default}

```gdscript
void set_scroll_horizontal_by_default(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_scroll_hint {#set_tile_scroll_hint}

```gdscript
void set_tile_scroll_hint(tile_scroll_hint: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_scroll {#set_v_scroll}

```gdscript
void set_v_scroll(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical_custom_step {#set_vertical_custom_step}

```gdscript
void set_vertical_custom_step(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical_scroll_mode {#set_vertical_scroll_mode}

```gdscript
void set_vertical_scroll_mode(enable: ScrollContainer.ScrollMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### follow_focus {#follow_focus}

```gdscript
follow_focus: bool
```

- void set_follow_focus(value: bool)
- bool is_following_focus()

If true, the ScrollContainer will automatically scroll to focused children (including indirect children) to make sure they are fully visible.

### draw_focus_border {#draw_focus_border}

```gdscript
draw_focus_border: bool
```

- void set_draw_focus_border(value: bool)
- bool get_draw_focus_border()

If true, focus is drawn when the ScrollContainer or one of its descendant nodes is focused.

### scroll_horizontal {#scroll_horizontal}

```gdscript
scroll_horizontal: int
```

- void set_h_scroll(value: int)
- int get_h_scroll()

The current horizontal scroll value.

Note: If you are setting this value in the Node._ready() function or earlier, it needs to be wrapped with Object.set_deferred(), since scroll bar's Range.max_value is not initialized yet.

```gdscript
func _ready():
    set_deferred("scroll_horizontal", 600)
```

### scroll_vertical {#scroll_vertical}

```gdscript
scroll_vertical: int
```

- void set_v_scroll(value: int)
- int get_v_scroll()

The current vertical scroll value.

Note: Setting it early needs to be deferred, just like in scroll_horizontal.

```gdscript
func _ready():
    set_deferred("scroll_vertical", 600)
```

### scroll_horizontal_custom_step {#scroll_horizontal_custom_step}

```gdscript
scroll_horizontal_custom_step: float
```

- void set_horizontal_custom_step(value: float)
- float get_horizontal_custom_step()

Overrides the ScrollBar.custom_step used when clicking the internal scroll bar's horizontal increment and decrement buttons or when using arrow keys when the ScrollBar is focused.

### scroll_vertical_custom_step {#scroll_vertical_custom_step}

```gdscript
scroll_vertical_custom_step: float
```

- void set_vertical_custom_step(value: float)
- float get_vertical_custom_step()

Overrides the ScrollBar.custom_step used when clicking the internal scroll bar's vertical increment and decrement buttons or when using arrow keys when the ScrollBar is focused.

### horizontal_scroll_mode {#horizontal_scroll_mode}

```gdscript
horizontal_scroll_mode: int
```

- void set_horizontal_scroll_mode(value: ScrollMode)
- ScrollMode get_horizontal_scroll_mode()

Controls whether horizontal scrollbar can be used and when it should be visible.

### vertical_scroll_mode {#vertical_scroll_mode}

```gdscript
vertical_scroll_mode: int
```

- void set_vertical_scroll_mode(value: ScrollMode)
- ScrollMode get_vertical_scroll_mode()

Controls whether vertical scrollbar can be used and when it should be visible.

### scroll_horizontal_by_default {#scroll_horizontal_by_default}

```gdscript
scroll_horizontal_by_default: bool
```

- void set_scroll_horizontal_by_default(value: bool)
- bool is_scroll_horizontal_by_default()

If true, the mouse wheel scrolls the view horizontally, and holding Shift scrolls vertically.

If false (default), the mouse wheel scrolls the view vertically, and holding Shift scrolls horizontally.

### scroll_deadzone {#scroll_deadzone}

```gdscript
scroll_deadzone: int
```

- void set_deadzone(value: int)
- int get_deadzone()

Deadzone for touch scrolling. Lower deadzone makes the scrolling more sensitive.

### scroll_hint_mode {#scroll_hint_mode}

```gdscript
scroll_hint_mode: int
```

- void set_scroll_hint_mode(value: ScrollHintMode)
- ScrollHintMode get_scroll_hint_mode()

The way which scroll hints (indicators that show that the content can still be scrolled in a certain direction) will be shown.

Note: Hints won't be shown if the content can be scrolled both vertically and horizontally.

### tile_scroll_hint {#tile_scroll_hint}

```gdscript
tile_scroll_hint: bool
```

- void set_tile_scroll_hint(value: bool)
- bool is_scroll_hint_tiled()

If true, the scroll hint texture will be tiled instead of stretched. See scroll_hint_mode.

## 信号

### scroll_started {#scroll_started}

```gdscript
signal scroll_started()
```

Emitted when scrolling starts when dragging the scrollable area with a touch event. This signal is not emitted when scrolling by dragging the scrollbar, scrolling with the mouse wheel or scrolling with keyboard/gamepad events.

Note: This signal is only emitted on Android or iOS, or on desktop/web platforms when ProjectSettings.input_devices/pointing/emulate_touch_from_mouse is enabled.

### scroll_ended {#scroll_ended}

```gdscript
signal scroll_ended()
```

Emitted when scrolling stops when dragging the scrollable area with a touch event. This signal is not emitted when scrolling by dragging the scrollbar, scrolling with the mouse wheel or scrolling with keyboard/gamepad events.

Note: This signal is only emitted on Android or iOS, or on desktop/web platforms when ProjectSettings.input_devices/pointing/emulate_touch_from_mouse is enabled.

## 枚举

### ScrollMode {#enum-ScrollMode}

```gdscript
enum ScrollMode { SCROLL_MODE_DISABLED = 0, SCROLL_MODE_AUTO = 1, SCROLL_MODE_SHOW_ALWAYS = 2, SCROLL_MODE_SHOW_NEVER = 3, SCROLL_MODE_RESERVE = 4, SCROLL_MODE_MAXIMIZE_FIRST = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCROLL_MODE_DISABLED {#SCROLL_MODE_DISABLED}

```gdscript
SCROLL_MODE_DISABLED = 0
```

Scrolling disabled, scrollbar will be invisible.

#### SCROLL_MODE_AUTO {#SCROLL_MODE_AUTO}

```gdscript
SCROLL_MODE_AUTO = 1
```

Scrolling enabled, scrollbar will be visible only if necessary, i.e. container's content is bigger than the container.

#### SCROLL_MODE_SHOW_ALWAYS {#SCROLL_MODE_SHOW_ALWAYS}

```gdscript
SCROLL_MODE_SHOW_ALWAYS = 2
```

Scrolling enabled, scrollbar will be always visible.

#### SCROLL_MODE_SHOW_NEVER {#SCROLL_MODE_SHOW_NEVER}

```gdscript
SCROLL_MODE_SHOW_NEVER = 3
```

Scrolling enabled, scrollbar will be hidden.

#### SCROLL_MODE_RESERVE {#SCROLL_MODE_RESERVE}

```gdscript
SCROLL_MODE_RESERVE = 4
```

Combines SCROLL_MODE_AUTO and SCROLL_MODE_SHOW_ALWAYS. The scrollbar is only visible if necessary, but the content size is adjusted as if it was always visible. It's useful for ensuring that content size stays the same regardless if the scrollbar is visible.

#### SCROLL_MODE_MAXIMIZE_FIRST {#SCROLL_MODE_MAXIMIZE_FIRST}

```gdscript
SCROLL_MODE_MAXIMIZE_FIRST = 5
```

Behaves like SCROLL_MODE_AUTO, but makes the ScrollContainer report a minimum size based on its content (limited by Control.custom_maximum_size when set on the corresponding axis). This allows it to grow first and only start scrolling once constrained.

---

### ScrollHintMode {#enum-ScrollHintMode}

```gdscript
enum ScrollHintMode { SCROLL_HINT_MODE_DISABLED = 0, SCROLL_HINT_MODE_ALL = 1, SCROLL_HINT_MODE_TOP_AND_LEFT = 2, SCROLL_HINT_MODE_BOTTOM_AND_RIGHT = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCROLL_HINT_MODE_DISABLED {#SCROLL_HINT_MODE_DISABLED}

```gdscript
SCROLL_HINT_MODE_DISABLED = 0
```

Scroll hints will never be shown.

#### SCROLL_HINT_MODE_ALL {#SCROLL_HINT_MODE_ALL}

```gdscript
SCROLL_HINT_MODE_ALL = 1
```

Scroll hints will be shown at the top and bottom (if vertical), or left and right (if horizontal).

#### SCROLL_HINT_MODE_TOP_AND_LEFT {#SCROLL_HINT_MODE_TOP_AND_LEFT}

```gdscript
SCROLL_HINT_MODE_TOP_AND_LEFT = 2
```

Scroll hints will be shown at the top (if vertical), or the left (if horizontal).

#### SCROLL_HINT_MODE_BOTTOM_AND_RIGHT {#SCROLL_HINT_MODE_BOTTOM_AND_RIGHT}

```gdscript
SCROLL_HINT_MODE_BOTTOM_AND_RIGHT = 3
```

Scroll hints will be shown at the bottom (if horizontal), or the right (if horizontal).

---
