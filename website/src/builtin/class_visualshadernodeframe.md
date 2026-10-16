# VisualShaderNodeFrame

**继承**：[VisualShaderNodeResizableBase](class_visualshadernoderesizablebase.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeFrame**

> A rectangular frame that can be used to group visual shader nodes together to improve organization.

<details><summary>官方英文描述</summary>

A rectangular frame that can be used to group visual shader nodes together to improve organization.

Nodes attached to the frame will move with it when it is dragged and it can automatically resize to enclose all attached nodes.

Its title, description and color can be customized.

</details>

## 方法

### add_attached_node {#add_attached_node}

```gdscript
void add_attached_node(node: int)
```

Adds a node to the list of nodes attached to the frame. Should not be called directly, use the VisualShader.attach_node_to_frame() method instead.

### get_attached_nodes {#get_attached_nodes}

```gdscript
PackedInt32Array get_attached_nodes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tint_color {#get_tint_color}

```gdscript
Color get_tint_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title {#get_title}

```gdscript
String get_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_autoshrink_enabled {#is_autoshrink_enabled}

```gdscript
bool is_autoshrink_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_tint_color_enabled {#is_tint_color_enabled}

```gdscript
bool is_tint_color_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_attached_node {#remove_attached_node}

```gdscript
void remove_attached_node(node: int)
```

Removes a node from the list of nodes attached to the frame. Should not be called directly, use the VisualShader.detach_node_from_frame() method instead.

### set_attached_nodes {#set_attached_nodes}

```gdscript
void set_attached_nodes(attached_nodes: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autoshrink_enabled {#set_autoshrink_enabled}

```gdscript
void set_autoshrink_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tint_color {#set_tint_color}

```gdscript
void set_tint_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tint_color_enabled {#set_tint_color_enabled}

```gdscript
void set_tint_color_enabled(enable: bool)
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

The title of the node.

### tint_color_enabled {#tint_color_enabled}

```gdscript
tint_color_enabled: bool
```

- void set_tint_color_enabled(value: bool)
- bool is_tint_color_enabled()

If true, the frame will be tinted with the color specified in tint_color.

### tint_color {#tint_color}

```gdscript
tint_color: Color
```

- void set_tint_color(value: Color)
- Color get_tint_color()

The color of the frame when tint_color_enabled is true.

### autoshrink {#autoshrink}

```gdscript
autoshrink: bool
```

- void set_autoshrink_enabled(value: bool)
- bool is_autoshrink_enabled()

If true, the frame will automatically resize to enclose all attached nodes.

### attached_nodes {#attached_nodes}

```gdscript
attached_nodes: PackedInt32Array
```

- void set_attached_nodes(value: PackedInt32Array)
- PackedInt32Array get_attached_nodes()

The list of nodes attached to the frame.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.
