# GraphNode

**继承**：[GraphElement](class_graphelement.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GraphNode**

> GraphNode allows to create nodes for a GraphEdit graph with customizable content based on its child controls. GraphNode is derived from Container and it is responsible for placing its children on screen. This works similar to VBoxContainer. Children, in turn, provide GraphNode with so-called slots, each of which can have a connection port on either side.

<details><summary>官方英文描述（中文精译进行中）</summary>

GraphNode allows to create nodes for a GraphEdit graph with customizable content based on its child controls. GraphNode is derived from Container and it is responsible for placing its children on screen. This works similar to VBoxContainer. Children, in turn, provide GraphNode with so-called slots, each of which can have a connection port on either side.

Each GraphNode slot is defined by its index and can provide the node with up to two ports: one on the left, and one on the right. By convention the left port is also referred to as the input port and the right port is referred to as the output port. Each port can be enabled and configured individually, using different type and color. The type is an arbitrary value that you can define using your own considerations. The parent GraphEdit will receive this information on each connect and disconnect request.

Slots can be configured in the Inspector dock once you add at least one child Control. The properties are grouped by each slot's index in the "Slot" section.

Note: While GraphNode is set up using slots and slot indices, connections are made between the ports which are enabled. Because of that GraphEdit uses the port's index and not the slot's index. You can use get_input_port_slot() and get_output_port_slot() to get the slot index from the port index.

</details>

## 方法

### _draw_port {#_draw_port}

```gdscript
void _draw_port(slot_index: int, position: Vector2i, left: bool, color: Color) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### clear_all_slots {#clear_all_slots}

```gdscript
void clear_all_slots()
```

Disables all slots of the GraphNode. This will remove all input/output ports from the GraphNode.

### clear_slot {#clear_slot}

```gdscript
void clear_slot(slot_index: int)
```

Disables the slot with the given slot_index. This will remove the corresponding input and output port from the GraphNode.

### get_input_port_color {#get_input_port_color}

```gdscript
Color get_input_port_color(port_idx: int)
```

Returns the Color of the input port with the given port_idx.

### get_input_port_count {#get_input_port_count}

```gdscript
int get_input_port_count()
```

Returns the number of slots with an enabled input port.

### get_input_port_position {#get_input_port_position}

```gdscript
Vector2 get_input_port_position(port_idx: int)
```

Returns the position of the input port with the given port_idx.

### get_input_port_slot {#get_input_port_slot}

```gdscript
int get_input_port_slot(port_idx: int)
```

Returns the corresponding slot index of the input port with the given port_idx.

### get_input_port_type {#get_input_port_type}

```gdscript
int get_input_port_type(port_idx: int)
```

Returns the type of the input port with the given port_idx.

### get_output_port_color {#get_output_port_color}

```gdscript
Color get_output_port_color(port_idx: int)
```

Returns the Color of the output port with the given port_idx.

### get_output_port_count {#get_output_port_count}

```gdscript
int get_output_port_count()
```

Returns the number of slots with an enabled output port.

### get_output_port_position {#get_output_port_position}

```gdscript
Vector2 get_output_port_position(port_idx: int)
```

Returns the position of the output port with the given port_idx.

### get_output_port_slot {#get_output_port_slot}

```gdscript
int get_output_port_slot(port_idx: int)
```

Returns the corresponding slot index of the output port with the given port_idx.

### get_output_port_type {#get_output_port_type}

```gdscript
int get_output_port_type(port_idx: int)
```

Returns the type of the output port with the given port_idx.

### get_slot_color_left {#get_slot_color_left}

```gdscript
Color get_slot_color_left(slot_index: int) const
```

Returns the left (input) Color of the slot with the given slot_index.

### get_slot_color_right {#get_slot_color_right}

```gdscript
Color get_slot_color_right(slot_index: int) const
```

Returns the right (output) Color of the slot with the given slot_index.

### get_slot_custom_icon_left {#get_slot_custom_icon_left}

```gdscript
Texture2D get_slot_custom_icon_left(slot_index: int) const
```

Returns the left (input) custom Texture2D of the slot with the given slot_index.

### get_slot_custom_icon_right {#get_slot_custom_icon_right}

```gdscript
Texture2D get_slot_custom_icon_right(slot_index: int) const
```

Returns the right (output) custom Texture2D of the slot with the given slot_index.

### get_slot_metadata_left {#get_slot_metadata_left}

```gdscript
Variant get_slot_metadata_left(slot_index: int) const
```

Returns the left (input) metadata of the slot with the given slot_index.

### get_slot_metadata_right {#get_slot_metadata_right}

```gdscript
Variant get_slot_metadata_right(slot_index: int) const
```

Returns the right (output) metadata of the slot with the given slot_index.

### get_slot_type_left {#get_slot_type_left}

```gdscript
int get_slot_type_left(slot_index: int) const
```

Returns the left (input) type of the slot with the given slot_index.

### get_slot_type_right {#get_slot_type_right}

```gdscript
int get_slot_type_right(slot_index: int) const
```

Returns the right (output) type of the slot with the given slot_index.

### get_slots_focus_mode {#get_slots_focus_mode}

```gdscript
Control.FocusMode get_slots_focus_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title {#get_title}

```gdscript
String get_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_titlebar_hbox {#get_titlebar_hbox}

```gdscript
HBoxContainer get_titlebar_hbox()
```

Returns the HBoxContainer used for the title bar, only containing a Label for displaying the title by default. This can be used to add custom controls to the title bar such as option or close buttons.

### is_ignoring_valid_connection_type {#is_ignoring_valid_connection_type}

```gdscript
bool is_ignoring_valid_connection_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_slot_draw_stylebox {#is_slot_draw_stylebox}

```gdscript
bool is_slot_draw_stylebox(slot_index: int) const
```

Returns true if the background StyleBox of the slot with the given slot_index is drawn.

### is_slot_enabled_left {#is_slot_enabled_left}

```gdscript
bool is_slot_enabled_left(slot_index: int) const
```

Returns true if left (input) side of the slot with the given slot_index is enabled.

### is_slot_enabled_right {#is_slot_enabled_right}

```gdscript
bool is_slot_enabled_right(slot_index: int) const
```

Returns true if right (output) side of the slot with the given slot_index is enabled.

### set_ignore_invalid_connection_type {#set_ignore_invalid_connection_type}

```gdscript
void set_ignore_invalid_connection_type(ignore: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_slot {#set_slot}

```gdscript
void set_slot(slot_index: int, enable_left_port: bool, type_left: int, color_left: Color, enable_right_port: bool, type_right: int, color_right: Color, custom_icon_left: Texture2D = null, custom_icon_right: Texture2D = null, draw_stylebox: bool = true)
```

Sets properties of the slot with the given slot_index.

If enable_left_port/enable_right_port is true, a port will appear and the slot will be able to be connected from this side.

With type_left/type_right an arbitrary type can be assigned to each port. Two ports can be connected if they share the same type, or if the connection between their types is allowed in the parent GraphEdit (see GraphEdit.add_valid_connection_type()). Keep in mind that the GraphEdit has the final say in accepting the connection. Type compatibility simply allows the GraphEdit.connection_request signal to be emitted.

Ports can be further customized using color_left/color_right and custom_icon_left/custom_icon_right. The color parameter adds a tint to the icon. The custom icon can be used to override the default port dot.

Additionally, draw_stylebox can be used to enable or disable drawing of the background stylebox for each slot. See slot.

Individual properties can also be set using one of the set_slot_* methods.

Note: This method only sets properties of the slot. To create the slot itself, add a Control-derived child to the GraphNode.

### set_slot_color_left {#set_slot_color_left}

```gdscript
void set_slot_color_left(slot_index: int, color: Color)
```

Sets the Color of the left (input) side of the slot with the given slot_index to color.

### set_slot_color_right {#set_slot_color_right}

```gdscript
void set_slot_color_right(slot_index: int, color: Color)
```

Sets the Color of the right (output) side of the slot with the given slot_index to color.

### set_slot_custom_icon_left {#set_slot_custom_icon_left}

```gdscript
void set_slot_custom_icon_left(slot_index: int, custom_icon: Texture2D)
```

Sets the custom Texture2D of the left (input) side of the slot with the given slot_index to custom_icon.

### set_slot_custom_icon_right {#set_slot_custom_icon_right}

```gdscript
void set_slot_custom_icon_right(slot_index: int, custom_icon: Texture2D)
```

Sets the custom Texture2D of the right (output) side of the slot with the given slot_index to custom_icon.

### set_slot_draw_stylebox {#set_slot_draw_stylebox}

```gdscript
void set_slot_draw_stylebox(slot_index: int, enable: bool)
```

Toggles the background StyleBox of the slot with the given slot_index.

### set_slot_enabled_left {#set_slot_enabled_left}

```gdscript
void set_slot_enabled_left(slot_index: int, enable: bool)
```

Toggles the left (input) side of the slot with the given slot_index. If enable is true, a port will appear on the left side and the slot will be able to be connected from this side.

### set_slot_enabled_right {#set_slot_enabled_right}

```gdscript
void set_slot_enabled_right(slot_index: int, enable: bool)
```

Toggles the right (output) side of the slot with the given slot_index. If enable is true, a port will appear on the right side and the slot will be able to be connected from this side.

### set_slot_metadata_left {#set_slot_metadata_left}

```gdscript
void set_slot_metadata_left(slot_index: int, value: Variant)
```

Sets the custom metadata for the left (input) side of the slot with the given slot_index to value.

### set_slot_metadata_right {#set_slot_metadata_right}

```gdscript
void set_slot_metadata_right(slot_index: int, value: Variant)
```

Sets the custom metadata for the right (output) side of the slot with the given slot_index to value.

### set_slot_type_left {#set_slot_type_left}

```gdscript
void set_slot_type_left(slot_index: int, type: int)
```

Sets the left (input) type of the slot with the given slot_index to type. If the value is negative, all connections will be disallowed to be created via user inputs.

### set_slot_type_right {#set_slot_type_right}

```gdscript
void set_slot_type_right(slot_index: int, type: int)
```

Sets the right (output) type of the slot with the given slot_index to type. If the value is negative, all connections will be disallowed to be created via user inputs.

### set_slots_focus_mode {#set_slots_focus_mode}

```gdscript
void set_slots_focus_mode(focus_mode: Control.FocusMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title {#set_title}

```gdscript
void set_title(title: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### title {#title}

```gdscript
title: String
```

- void set_title(value: String)
- String get_title()

The text displayed in the GraphNode's title bar.

### ignore_invalid_connection_type {#ignore_invalid_connection_type}

```gdscript
ignore_invalid_connection_type: bool
```

- void set_ignore_invalid_connection_type(value: bool)
- bool is_ignoring_valid_connection_type()

If true, you can connect ports with different types, even if the connection was not explicitly allowed in the parent GraphEdit.

### slots_focus_mode {#slots_focus_mode}

```gdscript
slots_focus_mode: int
```

- void set_slots_focus_mode(value: FocusMode)
- FocusMode get_slots_focus_mode()

Determines how connection slots can be focused.

- If set to Control.FOCUS_CLICK, connections can only be made with the mouse.
- If set to Control.FOCUS_ALL, slots can also be focused using the ProjectSettings.input/ui_up and ProjectSettings.input/ui_down and connected using ProjectSettings.input/ui_left and ProjectSettings.input/ui_right input actions.
- If set to Control.FOCUS_ACCESSIBILITY, slot input actions are only enabled when the screen reader is active.

## 信号

### slot_updated {#slot_updated}

```gdscript
signal slot_updated(slot_index: int)
```

Emitted when any GraphNode's slot is updated.

### slot_sizes_changed {#slot_sizes_changed}

```gdscript
signal slot_sizes_changed()
```

Emitted when any slot's size might have changed.
