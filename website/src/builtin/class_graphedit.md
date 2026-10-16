# GraphEdit

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GraphEdit**

> GraphEdit provides tools for creation, manipulation, and display of various graphs. Its main purpose in the engine is to power the visual programming systems, such as visual shaders, but it is also available for use in user projects.

<details><summary>官方英文描述（中文精译进行中）</summary>

GraphEdit provides tools for creation, manipulation, and display of various graphs. Its main purpose in the engine is to power the visual programming systems, such as visual shaders, but it is also available for use in user projects.

GraphEdit by itself is only an empty container, representing an infinite grid where GraphNodes can be placed. Each GraphNode represents a node in the graph, a single unit of data in the connected scheme. GraphEdit, in turn, helps to control various interactions with nodes and between nodes. When the user attempts to connect, disconnect, or delete a GraphNode, a signal is emitted in the GraphEdit, but no action is taken by default. It is the responsibility of the programmer utilizing this control to implement the necessary logic to determine how each request should be handled.

Performance: It is greatly advised to enable low-processor usage mode (see OS.low_processor_usage_mode) when using GraphEdits.

Note: Keep in mind that Node.get_children() will also return the connection layer node named _connection_layer due to technical limitations. This behavior may change in future releases.

</details>

## 方法

### _get_connection_line {#_get_connection_line}

```gdscript
PackedVector2Array _get_connection_line(from_position: Vector2, to_position: Vector2) virtual const
```

Virtual method which can be overridden to customize how connections are drawn.

### _is_in_input_hotzone {#_is_in_input_hotzone}

```gdscript
bool _is_in_input_hotzone(in_node: Object, in_port: int, mouse_position: Vector2) virtual
```

Returns whether the mouse_position is in the input hot zone.

By default, a hot zone is a Rect2 positioned such that its center is at in_node.GraphNode.get_input_port_position()(in_port) (For output's case, call GraphNode.get_output_port_position() instead). The hot zone's width is twice the Theme Property port_grab_distance_horizontal, and its height is twice the port_grab_distance_vertical.

Below is a sample code to help get started:

```gdscript
func _is_in_input_hotzone(in_node, in_port, mouse_position):
    var port_size = Vector2(get_theme_constant("port_grab_distance_horizontal"), get_theme_constant("port_grab_distance_vertical"))
    var port_pos = in_node.get_position() + in_node.get_input_port_position(in_port) - port_size / 2
    var rect = Rect2(port_pos, port_size)
    return rect.has_point(mouse_position)
```

### _is_in_output_hotzone {#_is_in_output_hotzone}

```gdscript
bool _is_in_output_hotzone(in_node: Object, in_port: int, mouse_position: Vector2) virtual
```

Returns whether the mouse_position is in the output hot zone. For more information on hot zones, see _is_in_input_hotzone().

Below is a sample code to help get started:

```gdscript
func _is_in_output_hotzone(in_node, in_port, mouse_position):
    var port_size = Vector2(get_theme_constant("port_grab_distance_horizontal"), get_theme_constant("port_grab_distance_vertical"))
    var port_pos = in_node.get_position() + in_node.get_output_port_position(in_port) - port_size / 2
    var rect = Rect2(port_pos, port_size)
    return rect.has_point(mouse_position)
```

### _is_node_hover_valid {#_is_node_hover_valid}

```gdscript
bool _is_node_hover_valid(from_node: StringName, from_port: int, to_node: StringName, to_port: int) virtual
```

This virtual method can be used to insert additional error detection while the user is dragging a connection over a valid port.

Return true if the connection is indeed valid or return false if the connection is impossible. If the connection is impossible, no snapping to the port and thus no connection request to that port will happen.

In this example a connection to same node is suppressed:

```
func _is_node_hover_valid(from, from_port, to, to_port):
    return from != to
```

```
public override bool _IsNodeHoverValid(StringName fromNode, int fromPort, StringName toNode, int toPort)
{
    return fromNode != toNode;
}
```

### add_valid_connection_type {#add_valid_connection_type}

```gdscript
void add_valid_connection_type(from_type: int, to_type: int)
```

Allows the connection between two different port types. The port type is defined individually for the left and the right port of each slot with the GraphNode.set_slot() method.

See also is_valid_connection_type() and remove_valid_connection_type().

### add_valid_left_disconnect_type {#add_valid_left_disconnect_type}

```gdscript
void add_valid_left_disconnect_type(type: int)
```

Allows to disconnect nodes when dragging from the left port of the GraphNode's slot if it has the specified type. See also remove_valid_left_disconnect_type().

### add_valid_right_disconnect_type {#add_valid_right_disconnect_type}

```gdscript
void add_valid_right_disconnect_type(type: int)
```

Allows to disconnect nodes when dragging from the right port of the GraphNode's slot if it has the specified type. See also remove_valid_right_disconnect_type().

### arrange_nodes {#arrange_nodes}

```gdscript
void arrange_nodes()
```

Rearranges selected nodes in a layout with minimum crossings between connections and uniform horizontal and vertical gap between nodes.

### attach_graph_element_to_frame {#attach_graph_element_to_frame}

```gdscript
void attach_graph_element_to_frame(element: StringName, frame: StringName)
```

Attaches the element GraphElement to the frame GraphFrame.

### clear_connections {#clear_connections}

```gdscript
void clear_connections()
```

Removes all connections between nodes.

### connect_node {#connect_node}

```gdscript
Error connect_node(from_node: StringName, from_port: int, to_node: StringName, to_port: int, keep_alive: bool = false)
```

Create a connection between the from_port of the from_node GraphNode and the to_port of the to_node GraphNode. If the connection already exists, no connection is created.

Connections with keep_alive set to false may be deleted automatically if invalid during a redraw.

### detach_graph_element_from_frame {#detach_graph_element_from_frame}

```gdscript
void detach_graph_element_from_frame(element: StringName)
```

Detaches the element GraphElement from the GraphFrame it is currently attached to.

### disconnect_node {#disconnect_node}

```gdscript
void disconnect_node(from_node: StringName, from_port: int, to_node: StringName, to_port: int)
```

Removes the connection between the from_port of the from_node GraphNode and the to_port of the to_node GraphNode. If the connection does not exist, no connection is removed.

### force_connection_drag_end {#force_connection_drag_end}

```gdscript
void force_connection_drag_end()
```

Ends the creation of the current connection. In other words, if you are dragging a connection you can use this method to abort the process and remove the line that followed your cursor.

This is best used together with connection_drag_started and connection_drag_ended to add custom behavior like node addition through shortcuts.

Note: This method suppresses any other connection request signals apart from connection_drag_ended.

### get_attached_nodes_of_frame {#get_attached_nodes_of_frame}

```gdscript
typedarray::StringName get_attached_nodes_of_frame(frame: StringName)
```

Returns an array of node names that are attached to the GraphFrame with the given name.

### get_closest_connection_at_point {#get_closest_connection_at_point}

```gdscript
Dictionary get_closest_connection_at_point(point: Vector2, max_distance: float = 4.0) const
```

Returns the closest connection to the given point in screen space. If no connection is found within max_distance pixels, an empty Dictionary is returned.

A connection is represented as a Dictionary in the form of:

```gdscript
{
    from_node: StringName,
    from_port: int,
    to_node: StringName,
    to_port: int,
    keep_alive: bool
}
```

For example, getting a connection at a given mouse position can be achieved like this:

```
var connection = get_closest_connection_at_point(mouse_event.get_position())
```

### get_connection_count {#get_connection_count}

```gdscript
int get_connection_count(from_node: StringName, from_port: int)
```

Returns the number of connections from from_port of from_node.

### get_connection_line {#get_connection_line}

```gdscript
PackedVector2Array get_connection_line(from_node: Vector2, to_node: Vector2) const
```

Returns the points which would make up a connection between from_node and to_node.

### get_connection_lines_curvature {#get_connection_lines_curvature}

```gdscript
float get_connection_lines_curvature() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_connection_lines_thickness {#get_connection_lines_thickness}

```gdscript
float get_connection_lines_thickness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_connection_list {#get_connection_list}

```gdscript
typedarray::Dictionary get_connection_list() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_connection_list_from_node {#get_connection_list_from_node}

```gdscript
typedarray::Dictionary get_connection_list_from_node(node: StringName) const
```

Returns an Array containing a list of all connections for node.

A connection is represented as a Dictionary in the form of:

```gdscript
{
    from_node: StringName,
    from_port: int,
    to_node: StringName,
    to_port: int,
    keep_alive: bool
}
```

Example: Get all connections on a specific port:

```gdscript
func get_connection_list_from_port(node, port):
    var connections = get_connection_list_from_node(node)
    var result = []
    for connection in connections:
        var dict = {}
        if connection["from_node"] == node and connection["from_port"] == port:
            dict["node"] = connection["to_node"]
            dict["port"] = connection["to_port"]
            dict["type"] = "left"
            result.push_back(dict)
        elif connection["to_node"] == node and connection["to_port"] == port:
            dict["node"] = connection["from_node"]
            dict["port"] = connection["from_port"]
            dict["type"] = "right"
            result.push_back(dict)
    return result
```

### get_connections_intersecting_with_rect {#get_connections_intersecting_with_rect}

```gdscript
typedarray::Dictionary get_connections_intersecting_with_rect(rect: Rect2) const
```

Returns an Array containing the list of connections that intersect with the given Rect2.

A connection is represented as a Dictionary in the form of:

```gdscript
{
    from_node: StringName,
    from_port: int,
    to_node: StringName,
    to_port: int,
    keep_alive: bool
}
```

### get_element_frame {#get_element_frame}

```gdscript
GraphFrame get_element_frame(element: StringName)
```

Returns the GraphFrame that contains the GraphElement with the given name.

### get_grid_pattern {#get_grid_pattern}

```gdscript
GraphEdit.GridPattern get_grid_pattern() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_menu_hbox {#get_menu_hbox}

```gdscript
HBoxContainer get_menu_hbox()
```

Gets the HBoxContainer that contains the zooming and grid snap controls in the top left of the graph. You can use this method to reposition the toolbar or to add your own custom controls to it.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_minimap_opacity {#get_minimap_opacity}

```gdscript
float get_minimap_opacity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_minimap_size {#get_minimap_size}

```gdscript
Vector2 get_minimap_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_panning_scheme {#get_panning_scheme}

```gdscript
GraphEdit.PanningScheme get_panning_scheme() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_offset {#get_scroll_offset}

```gdscript
Vector2 get_scroll_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_snapping_distance {#get_snapping_distance}

```gdscript
int get_snapping_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_type_names {#get_type_names}

```gdscript
Dictionary get_type_names() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_zoom {#get_zoom}

```gdscript
float get_zoom() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_zoom_max {#get_zoom_max}

```gdscript
float get_zoom_max() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_zoom_min {#get_zoom_min}

```gdscript
float get_zoom_min() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_zoom_step {#get_zoom_step}

```gdscript
float get_zoom_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_connection_lines_antialiased {#is_connection_lines_antialiased}

```gdscript
bool is_connection_lines_antialiased() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_minimap_enabled {#is_minimap_enabled}

```gdscript
bool is_minimap_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_node_connected {#is_node_connected}

```gdscript
bool is_node_connected(from_node: StringName, from_port: int, to_node: StringName, to_port: int)
```

Returns true if the from_port of the from_node GraphNode is connected to the to_port of the to_node GraphNode.

### is_right_disconnects_enabled {#is_right_disconnects_enabled}

```gdscript
bool is_right_disconnects_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_arrange_button {#is_showing_arrange_button}

```gdscript
bool is_showing_arrange_button() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_grid {#is_showing_grid}

```gdscript
bool is_showing_grid() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_grid_buttons {#is_showing_grid_buttons}

```gdscript
bool is_showing_grid_buttons() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_menu {#is_showing_menu}

```gdscript
bool is_showing_menu() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_minimap_button {#is_showing_minimap_button}

```gdscript
bool is_showing_minimap_button() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_zoom_buttons {#is_showing_zoom_buttons}

```gdscript
bool is_showing_zoom_buttons() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_zoom_label {#is_showing_zoom_label}

```gdscript
bool is_showing_zoom_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_snapping_enabled {#is_snapping_enabled}

```gdscript
bool is_snapping_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_valid_connection_type {#is_valid_connection_type}

```gdscript
bool is_valid_connection_type(from_type: int, to_type: int) const
```

Returns whether it's possible to make a connection between two different port types. The port type is defined individually for the left and the right port of each slot with the GraphNode.set_slot() method.

See also add_valid_connection_type() and remove_valid_connection_type().

### remove_valid_connection_type {#remove_valid_connection_type}

```gdscript
void remove_valid_connection_type(from_type: int, to_type: int)
```

Disallows the connection between two different port types previously allowed by add_valid_connection_type(). The port type is defined individually for the left and the right port of each slot with the GraphNode.set_slot() method.

See also is_valid_connection_type().

### remove_valid_left_disconnect_type {#remove_valid_left_disconnect_type}

```gdscript
void remove_valid_left_disconnect_type(type: int)
```

Disallows to disconnect nodes when dragging from the left port of the GraphNode's slot if it has the specified type. Use this to disable a disconnection previously allowed with add_valid_left_disconnect_type().

### remove_valid_right_disconnect_type {#remove_valid_right_disconnect_type}

```gdscript
void remove_valid_right_disconnect_type(type: int)
```

Disallows to disconnect nodes when dragging from the right port of the GraphNode's slot if it has the specified type. Use this to disable a disconnection previously allowed with add_valid_right_disconnect_type().

### set_connection_activity {#set_connection_activity}

```gdscript
void set_connection_activity(from_node: StringName, from_port: int, to_node: StringName, to_port: int, amount: float)
```

Sets the coloration of the connection between from_node's from_port and to_node's to_port with the color provided in the activity theme property. The color is linearly interpolated between the connection color and the activity color using amount as weight.

### set_connection_lines_antialiased {#set_connection_lines_antialiased}

```gdscript
void set_connection_lines_antialiased(pixels: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_connection_lines_curvature {#set_connection_lines_curvature}

```gdscript
void set_connection_lines_curvature(curvature: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_connection_lines_thickness {#set_connection_lines_thickness}

```gdscript
void set_connection_lines_thickness(pixels: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_connections {#set_connections}

```gdscript
void set_connections(connections: typedarray::Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_grid_pattern {#set_grid_pattern}

```gdscript
void set_grid_pattern(pattern: GraphEdit.GridPattern)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_minimap_enabled {#set_minimap_enabled}

```gdscript
void set_minimap_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_minimap_opacity {#set_minimap_opacity}

```gdscript
void set_minimap_opacity(opacity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_minimap_size {#set_minimap_size}

```gdscript
void set_minimap_size(size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_panning_scheme {#set_panning_scheme}

```gdscript
void set_panning_scheme(scheme: GraphEdit.PanningScheme)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_right_disconnects {#set_right_disconnects}

```gdscript
void set_right_disconnects(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_offset {#set_scroll_offset}

```gdscript
void set_scroll_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selected {#set_selected}

```gdscript
void set_selected(node: Node)
```

Sets the specified node as the one selected.

### set_show_arrange_button {#set_show_arrange_button}

```gdscript
void set_show_arrange_button(hidden: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_grid {#set_show_grid}

```gdscript
void set_show_grid(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_grid_buttons {#set_show_grid_buttons}

```gdscript
void set_show_grid_buttons(hidden: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_menu {#set_show_menu}

```gdscript
void set_show_menu(hidden: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_minimap_button {#set_show_minimap_button}

```gdscript
void set_show_minimap_button(hidden: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_zoom_buttons {#set_show_zoom_buttons}

```gdscript
void set_show_zoom_buttons(hidden: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_zoom_label {#set_show_zoom_label}

```gdscript
void set_show_zoom_label(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snapping_distance {#set_snapping_distance}

```gdscript
void set_snapping_distance(pixels: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snapping_enabled {#set_snapping_enabled}

```gdscript
void set_snapping_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_type_names {#set_type_names}

```gdscript
void set_type_names(type_names: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_zoom {#set_zoom}

```gdscript
void set_zoom(zoom: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_zoom_max {#set_zoom_max}

```gdscript
void set_zoom_max(zoom_max: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_zoom_min {#set_zoom_min}

```gdscript
void set_zoom_min(zoom_min: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_zoom_step {#set_zoom_step}

```gdscript
void set_zoom_step(zoom_step: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### scroll_offset {#scroll_offset}

```gdscript
scroll_offset: Vector2
```

- void set_scroll_offset(value: Vector2)
- Vector2 get_scroll_offset()

The scroll offset.

### show_grid {#show_grid}

```gdscript
show_grid: bool
```

- void set_show_grid(value: bool)
- bool is_showing_grid()

If true, the grid is visible.

### grid_pattern {#grid_pattern}

```gdscript
grid_pattern: int
```

- void set_grid_pattern(value: GridPattern)
- GridPattern get_grid_pattern()

The pattern used for drawing the grid.

### snapping_enabled {#snapping_enabled}

```gdscript
snapping_enabled: bool
```

- void set_snapping_enabled(value: bool)
- bool is_snapping_enabled()

If true, enables snapping.

### snapping_distance {#snapping_distance}

```gdscript
snapping_distance: int
```

- void set_snapping_distance(value: int)
- int get_snapping_distance()

The snapping distance in pixels, also determines the grid line distance.

### panning_scheme {#panning_scheme}

```gdscript
panning_scheme: int
```

- void set_panning_scheme(value: PanningScheme)
- PanningScheme get_panning_scheme()

Defines the control scheme for panning with mouse wheel.

### right_disconnects {#right_disconnects}

```gdscript
right_disconnects: bool
```

- void set_right_disconnects(value: bool)
- bool is_right_disconnects_enabled()

If true, enables disconnection of existing connections in the GraphEdit by dragging the right end.

### type_names {#type_names}

```gdscript
type_names: typeddictionary::int;String
```

- void set_type_names(value: Dictionary)
- Dictionary get_type_names()

Dictionary of human-readable port type names.

### connection_lines_curvature {#connection_lines_curvature}

```gdscript
connection_lines_curvature: float
```

- void set_connection_lines_curvature(value: float)
- float get_connection_lines_curvature()

The curvature of the lines between the nodes. 0 results in straight lines.

### connection_lines_thickness {#connection_lines_thickness}

```gdscript
connection_lines_thickness: float
```

- void set_connection_lines_thickness(value: float)
- float get_connection_lines_thickness()

The thickness of the lines between the nodes.

### connection_lines_antialiased {#connection_lines_antialiased}

```gdscript
connection_lines_antialiased: bool
```

- void set_connection_lines_antialiased(value: bool)
- bool is_connection_lines_antialiased()

If true, the lines between nodes will use antialiasing.

### connections {#connections}

```gdscript
connections: typedarray::27/0:
```

- void set_connections(value: Array[Dictionary])
- Array[Dictionary] get_connection_list()

The connections between GraphNodes.

A connection is represented as a Dictionary in the form of:

```gdscript
{
    from_node: StringName,
    from_port: int,
    to_node: StringName,
    to_port: int,
    keep_alive: bool
}
```

Connections with keep_alive set to false may be deleted automatically if invalid during a redraw.

### zoom {#zoom}

```gdscript
zoom: float
```

- void set_zoom(value: float)
- float get_zoom()

The current zoom value.

### zoom_min {#zoom_min}

```gdscript
zoom_min: float
```

- void set_zoom_min(value: float)
- float get_zoom_min()

The lower zoom limit.

### zoom_max {#zoom_max}

```gdscript
zoom_max: float
```

- void set_zoom_max(value: float)
- float get_zoom_max()

The upper zoom limit.

### zoom_step {#zoom_step}

```gdscript
zoom_step: float
```

- void set_zoom_step(value: float)
- float get_zoom_step()

The step of each zoom level.

### minimap_enabled {#minimap_enabled}

```gdscript
minimap_enabled: bool
```

- void set_minimap_enabled(value: bool)
- bool is_minimap_enabled()

If true, the minimap is visible.

### minimap_size {#minimap_size}

```gdscript
minimap_size: Vector2
```

- void set_minimap_size(value: Vector2)
- Vector2 get_minimap_size()

The size of the minimap rectangle. The map itself is based on the size of the grid area and is scaled to fit this rectangle.

### minimap_opacity {#minimap_opacity}

```gdscript
minimap_opacity: float
```

- void set_minimap_opacity(value: float)
- float get_minimap_opacity()

The opacity of the minimap rectangle.

### show_menu {#show_menu}

```gdscript
show_menu: bool
```

- void set_show_menu(value: bool)
- bool is_showing_menu()

If true, the menu toolbar is visible.

### show_zoom_label {#show_zoom_label}

```gdscript
show_zoom_label: bool
```

- void set_show_zoom_label(value: bool)
- bool is_showing_zoom_label()

If true, the label with the current zoom level is visible. The zoom level is displayed in percents.

### show_zoom_buttons {#show_zoom_buttons}

```gdscript
show_zoom_buttons: bool
```

- void set_show_zoom_buttons(value: bool)
- bool is_showing_zoom_buttons()

If true, buttons that allow to change and reset the zoom level are visible.

### show_grid_buttons {#show_grid_buttons}

```gdscript
show_grid_buttons: bool
```

- void set_show_grid_buttons(value: bool)
- bool is_showing_grid_buttons()

If true, buttons that allow to configure grid and snapping options are visible.

### show_minimap_button {#show_minimap_button}

```gdscript
show_minimap_button: bool
```

- void set_show_minimap_button(value: bool)
- bool is_showing_minimap_button()

If true, the button to toggle the minimap is visible.

### show_arrange_button {#show_arrange_button}

```gdscript
show_arrange_button: bool
```

- void set_show_arrange_button(value: bool)
- bool is_showing_arrange_button()

If true, the button to automatically arrange graph nodes is visible.

## 信号

### connection_request {#connection_request}

```gdscript
signal connection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int)
```

Emitted to the GraphEdit when the connection between the from_port of the from_node GraphNode and the to_port of the to_node GraphNode is attempted to be created.

### disconnection_request {#disconnection_request}

```gdscript
signal disconnection_request(from_node: StringName, from_port: int, to_node: StringName, to_port: int)
```

Emitted to the GraphEdit when the connection between from_port of from_node GraphNode and to_port of to_node GraphNode is attempted to be removed.

### connection_to_empty {#connection_to_empty}

```gdscript
signal connection_to_empty(from_node: StringName, from_port: int, release_position: Vector2)
```

Emitted when user drags a connection from an output port into the empty space of the graph.

### connection_from_empty {#connection_from_empty}

```gdscript
signal connection_from_empty(to_node: StringName, to_port: int, release_position: Vector2)
```

Emitted when user drags a connection from an input port into the empty space of the graph.

### connection_drag_started {#connection_drag_started}

```gdscript
signal connection_drag_started(from_node: StringName, from_port: int, is_output: bool)
```

Emitted at the beginning of a connection drag.

### connection_drag_ended {#connection_drag_ended}

```gdscript
signal connection_drag_ended()
```

Emitted at the end of a connection drag.

### copy_nodes_request {#copy_nodes_request}

```gdscript
signal copy_nodes_request()
```

Emitted when this GraphEdit captures a ui_copy action (Ctrl + C by default). In general, this signal indicates that the selected GraphElements should be copied.

### cut_nodes_request {#cut_nodes_request}

```gdscript
signal cut_nodes_request()
```

Emitted when this GraphEdit captures a ui_cut action (Ctrl + X by default). In general, this signal indicates that the selected GraphElements should be cut.

### paste_nodes_request {#paste_nodes_request}

```gdscript
signal paste_nodes_request()
```

Emitted when this GraphEdit captures a ui_paste action (Ctrl + V by default). In general, this signal indicates that previously copied GraphElements should be pasted.

### duplicate_nodes_request {#duplicate_nodes_request}

```gdscript
signal duplicate_nodes_request()
```

Emitted when this GraphEdit captures a ui_graph_duplicate action (Ctrl + D by default). In general, this signal indicates that the selected GraphElements should be duplicated.

### delete_nodes_request {#delete_nodes_request}

```gdscript
signal delete_nodes_request(nodes: typedarray::StringName)
```

Emitted when this GraphEdit captures a ui_graph_delete action (Delete by default).

nodes is an array of node names that should be removed. These usually include all selected nodes.

### node_selected {#node_selected}

```gdscript
signal node_selected(node: Node)
```

Emitted when the given GraphElement node is selected.

### node_deselected {#node_deselected}

```gdscript
signal node_deselected(node: Node)
```

Emitted when the given GraphElement node is deselected.

### frame_rect_changed {#frame_rect_changed}

```gdscript
signal frame_rect_changed(frame: GraphFrame, new_rect: Rect2)
```

Emitted when the GraphFrame frame is resized to new_rect.

### popup_request {#popup_request}

```gdscript
signal popup_request(at_position: Vector2)
```

Emitted when a popup is requested. Happens on right-clicking in the GraphEdit. at_position is the position of the mouse pointer when the signal is sent.

### begin_node_move {#begin_node_move}

```gdscript
signal begin_node_move()
```

Emitted at the beginning of a GraphElement's movement.

### end_node_move {#end_node_move}

```gdscript
signal end_node_move()
```

Emitted at the end of a GraphElement's movement.

### graph_elements_linked_to_frame_request {#graph_elements_linked_to_frame_request}

```gdscript
signal graph_elements_linked_to_frame_request(elements: Array, frame: StringName)
```

Emitted when one or more GraphElements are dropped onto the GraphFrame named frame, when they were not previously attached to any other one.

elements is an array of GraphElements to be attached.

### scroll_offset_changed {#scroll_offset_changed}

```gdscript
signal scroll_offset_changed(offset: Vector2)
```

Emitted when the scroll offset is changed by the user. It will not be emitted when changed in code.

## 枚举

### PanningScheme {#enum-PanningScheme}

```gdscript
enum PanningScheme { SCROLL_ZOOMS = 0, SCROLL_PANS = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCROLL_ZOOMS {#SCROLL_ZOOMS}

```gdscript
SCROLL_ZOOMS = 0
```

Mouse Wheel will zoom, Ctrl + Mouse Wheel will move the view.

#### SCROLL_PANS {#SCROLL_PANS}

```gdscript
SCROLL_PANS = 1
```

Mouse Wheel will move the view, Ctrl + Mouse Wheel will zoom.

---

### GridPattern {#enum-GridPattern}

```gdscript
enum GridPattern { GRID_PATTERN_LINES = 0, GRID_PATTERN_DOTS = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GRID_PATTERN_LINES {#GRID_PATTERN_LINES}

```gdscript
GRID_PATTERN_LINES = 0
```

Draw the grid using solid lines.

#### GRID_PATTERN_DOTS {#GRID_PATTERN_DOTS}

```gdscript
GRID_PATTERN_DOTS = 1
```

Draw the grid using dots.

---
