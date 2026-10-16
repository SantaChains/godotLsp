# VisualShader

**继承**：[Shader](class_shader.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShader**

> This class provides a graph-like visual editor for creating a Shader. Although VisualShaders do not require coding, they share the same logic with script shaders. They use VisualShaderNodes that can be connected to each other to control the flow of the shader. The visual shader graph is converted to a script shader behind the scenes.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class provides a graph-like visual editor for creating a Shader. Although VisualShaders do not require coding, they share the same logic with script shaders. They use VisualShaderNodes that can be connected to each other to control the flow of the shader. The visual shader graph is converted to a script shader behind the scenes.

</details>

## 方法

### add_node {#add_node}

```gdscript
void add_node(type: VisualShader.Type, node: VisualShaderNode, position: Vector2, id: int)
```

Adds the specified node to the shader.

### add_varying {#add_varying}

```gdscript
void add_varying(name: String, mode: VisualShader.VaryingMode, type: VisualShader.VaryingType)
```

Adds a new varying value node to the shader.

### attach_node_to_frame {#attach_node_to_frame}

```gdscript
void attach_node_to_frame(type: VisualShader.Type, id: int, frame: int)
```

Attaches the given node to the given frame.

### can_connect_nodes {#can_connect_nodes}

```gdscript
bool can_connect_nodes(type: VisualShader.Type, from_node: int, from_port: int, to_node: int, to_port: int) const
```

Returns true if the specified nodes and ports can be connected together.

### connect_nodes {#connect_nodes}

```gdscript
Error connect_nodes(type: VisualShader.Type, from_node: int, from_port: int, to_node: int, to_port: int)
```

Connects the specified nodes and ports.

### connect_nodes_forced {#connect_nodes_forced}

```gdscript
void connect_nodes_forced(type: VisualShader.Type, from_node: int, from_port: int, to_node: int, to_port: int)
```

Connects the specified nodes and ports, even if they can't be connected. Such connection is invalid and will not function properly.

### detach_node_from_frame {#detach_node_from_frame}

```gdscript
void detach_node_from_frame(type: VisualShader.Type, id: int)
```

Detaches the given node from the frame it is attached to.

### disconnect_nodes {#disconnect_nodes}

```gdscript
void disconnect_nodes(type: VisualShader.Type, from_node: int, from_port: int, to_node: int, to_port: int)
```

Connects the specified nodes and ports.

### get_graph_offset {#get_graph_offset}

```gdscript
Vector2 get_graph_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node {#get_node}

```gdscript
VisualShaderNode get_node(type: VisualShader.Type, id: int) const
```

Returns the shader node instance with specified type and id.

### get_node_connections {#get_node_connections}

```gdscript
typedarray::Dictionary get_node_connections(type: VisualShader.Type) const
```

Returns the list of connected nodes with the specified type.

### get_node_list {#get_node_list}

```gdscript
PackedInt32Array get_node_list(type: VisualShader.Type) const
```

Returns the list of all nodes in the shader with the specified type.

### get_node_position {#get_node_position}

```gdscript
Vector2 get_node_position(type: VisualShader.Type, id: int) const
```

Returns the position of the specified node within the shader graph.

### get_valid_node_id {#get_valid_node_id}

```gdscript
int get_valid_node_id(type: VisualShader.Type) const
```

Returns next valid node ID that can be added to the shader graph.

### has_varying {#has_varying}

```gdscript
bool has_varying(name: String) const
```

Returns true if the shader has a varying with the given name.

### is_node_connection {#is_node_connection}

```gdscript
bool is_node_connection(type: VisualShader.Type, from_node: int, from_port: int, to_node: int, to_port: int) const
```

Returns true if the specified node and port connection exist.

### remove_node {#remove_node}

```gdscript
void remove_node(type: VisualShader.Type, id: int)
```

Removes the specified node from the shader.

### remove_varying {#remove_varying}

```gdscript
void remove_varying(name: String)
```

Removes a varying value node with the given name. Prints an error if a node with this name is not found.

### replace_node {#replace_node}

```gdscript
void replace_node(type: VisualShader.Type, id: int, new_class: StringName)
```

Replaces the specified node with a node of new class type.

### set_graph_offset {#set_graph_offset}

```gdscript
void set_graph_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mode {#set_mode}

```gdscript
void set_mode(mode: Shader.Mode)
```

Sets the mode of this shader.

### set_node_position {#set_node_position}

```gdscript
void set_node_position(type: VisualShader.Type, id: int, position: Vector2)
```

Sets the position of the specified node.

## 属性

### graph_offset {#graph_offset}

```gdscript
graph_offset: Vector2
```

- void set_graph_offset(value: Vector2)
- Vector2 get_graph_offset()

Deprecated: This property does nothing and always equals to zero.

Deprecated.

## 枚举

### Type {#enum-Type}

```gdscript
enum Type { TYPE_VERTEX = 0, TYPE_FRAGMENT = 1, TYPE_LIGHT = 2, TYPE_START = 3, TYPE_PROCESS = 4, TYPE_COLLIDE = 5, TYPE_START_CUSTOM = 6, TYPE_PROCESS_CUSTOM = 7, TYPE_SKY = 8, TYPE_FOG = 9, TYPE_TEXTURE_BLIT = 10, TYPE_MAX = 11 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_VERTEX {#TYPE_VERTEX}

```gdscript
TYPE_VERTEX = 0
```

A vertex shader, operating on vertices.

#### TYPE_FRAGMENT {#TYPE_FRAGMENT}

```gdscript
TYPE_FRAGMENT = 1
```

A fragment shader, operating on fragments (pixels).

#### TYPE_LIGHT {#TYPE_LIGHT}

```gdscript
TYPE_LIGHT = 2
```

A shader for light calculations.

#### TYPE_START {#TYPE_START}

```gdscript
TYPE_START = 3
```

A function for the "start" stage of particle shader.

#### TYPE_PROCESS {#TYPE_PROCESS}

```gdscript
TYPE_PROCESS = 4
```

A function for the "process" stage of particle shader.

#### TYPE_COLLIDE {#TYPE_COLLIDE}

```gdscript
TYPE_COLLIDE = 5
```

A function for the "collide" stage (particle collision handler) of particle shader.

#### TYPE_START_CUSTOM {#TYPE_START_CUSTOM}

```gdscript
TYPE_START_CUSTOM = 6
```

A function for the "start" stage of particle shader, with customized output.

#### TYPE_PROCESS_CUSTOM {#TYPE_PROCESS_CUSTOM}

```gdscript
TYPE_PROCESS_CUSTOM = 7
```

A function for the "process" stage of particle shader, with customized output.

#### TYPE_SKY {#TYPE_SKY}

```gdscript
TYPE_SKY = 8
```

A shader for 3D environment's sky.

#### TYPE_FOG {#TYPE_FOG}

```gdscript
TYPE_FOG = 9
```

A compute shader that runs for each froxel of the volumetric fog map.

#### TYPE_TEXTURE_BLIT {#TYPE_TEXTURE_BLIT}

```gdscript
TYPE_TEXTURE_BLIT = 10
```

A shader used to process blit calls to a DrawableTexture.

#### TYPE_MAX {#TYPE_MAX}

```gdscript
TYPE_MAX = 11
```

Represents the size of the Type enum.

---

### VaryingMode {#enum-VaryingMode}

```gdscript
enum VaryingMode { VARYING_MODE_VERTEX_TO_FRAG_LIGHT = 0, VARYING_MODE_FRAG_TO_LIGHT = 1, VARYING_MODE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VARYING_MODE_VERTEX_TO_FRAG_LIGHT {#VARYING_MODE_VERTEX_TO_FRAG_LIGHT}

```gdscript
VARYING_MODE_VERTEX_TO_FRAG_LIGHT = 0
```

Varying is passed from Vertex function to Fragment and Light functions.

#### VARYING_MODE_FRAG_TO_LIGHT {#VARYING_MODE_FRAG_TO_LIGHT}

```gdscript
VARYING_MODE_FRAG_TO_LIGHT = 1
```

Varying is passed from Fragment function to Light function.

#### VARYING_MODE_MAX {#VARYING_MODE_MAX}

```gdscript
VARYING_MODE_MAX = 2
```

Represents the size of the VaryingMode enum.

---

### VaryingType {#enum-VaryingType}

```gdscript
enum VaryingType { VARYING_TYPE_FLOAT = 0, VARYING_TYPE_INT = 1, VARYING_TYPE_UINT = 2, VARYING_TYPE_VECTOR_2D = 3, VARYING_TYPE_VECTOR_3D = 4, VARYING_TYPE_VECTOR_4D = 5, VARYING_TYPE_BOOLEAN = 6, VARYING_TYPE_TRANSFORM = 7, VARYING_TYPE_MAX = 8 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VARYING_TYPE_FLOAT {#VARYING_TYPE_FLOAT}

```gdscript
VARYING_TYPE_FLOAT = 0
```

Varying is of type float.

#### VARYING_TYPE_INT {#VARYING_TYPE_INT}

```gdscript
VARYING_TYPE_INT = 1
```

Varying is of type int.

#### VARYING_TYPE_UINT {#VARYING_TYPE_UINT}

```gdscript
VARYING_TYPE_UINT = 2
```

Varying is of type unsigned int.

#### VARYING_TYPE_VECTOR_2D {#VARYING_TYPE_VECTOR_2D}

```gdscript
VARYING_TYPE_VECTOR_2D = 3
```

Varying is of type Vector2.

#### VARYING_TYPE_VECTOR_3D {#VARYING_TYPE_VECTOR_3D}

```gdscript
VARYING_TYPE_VECTOR_3D = 4
```

Varying is of type Vector3.

#### VARYING_TYPE_VECTOR_4D {#VARYING_TYPE_VECTOR_4D}

```gdscript
VARYING_TYPE_VECTOR_4D = 5
```

Varying is of type Vector4.

#### VARYING_TYPE_BOOLEAN {#VARYING_TYPE_BOOLEAN}

```gdscript
VARYING_TYPE_BOOLEAN = 6
```

Varying is of type bool.

#### VARYING_TYPE_TRANSFORM {#VARYING_TYPE_TRANSFORM}

```gdscript
VARYING_TYPE_TRANSFORM = 7
```

Varying is of type Transform3D.

#### VARYING_TYPE_MAX {#VARYING_TYPE_MAX}

```gdscript
VARYING_TYPE_MAX = 8
```

Represents the size of the VaryingType enum.

---

## 常量

### NODE_ID_INVALID {#NODE_ID_INVALID}

```gdscript
const NODE_ID_INVALID = -1
```

Indicates an invalid VisualShader node.

NODE_ID_OUTPUT = 0 

Indicates an output node of VisualShader.

### NODE_ID_OUTPUT {#NODE_ID_OUTPUT}

```gdscript
const NODE_ID_OUTPUT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
