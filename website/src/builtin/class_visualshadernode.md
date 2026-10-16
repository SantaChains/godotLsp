# VisualShaderNode

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNode**

> Visual shader graphs consist of various nodes. Each node in the graph is a separate object and they are represented as a rectangular boxes with title and a set of properties. Each node also has connection ports that allow to connect it to another nodes and control the flow of the shader.

<details><summary>官方英文描述</summary>

Visual shader graphs consist of various nodes. Each node in the graph is a separate object and they are represented as a rectangular boxes with title and a set of properties. Each node also has connection ports that allow to connect it to another nodes and control the flow of the shader.

</details>

## 方法

### clear_default_input_values {#clear_default_input_values}

```gdscript
void clear_default_input_values()
```

Clears the default input ports value.

### get_default_input_port {#get_default_input_port}

```gdscript
int get_default_input_port(type: VisualShaderNode.PortType) const
```

Returns the input port which should be connected by default when this node is created as a result of dragging a connection from an existing node to the empty space on the graph.

### get_default_input_values {#get_default_input_values}

```gdscript
Array get_default_input_values() const
```

Returns an Array containing default values for all of the input ports of the node in the form [index0, value0, index1, value1, ...].

### get_frame {#get_frame}

```gdscript
int get_frame() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_input_port_default_value {#get_input_port_default_value}

```gdscript
Variant get_input_port_default_value(port: int) const
```

Returns the default value of the input port.

### get_output_port_for_preview {#get_output_port_for_preview}

```gdscript
int get_output_port_for_preview() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_input_port_default_value {#remove_input_port_default_value}

```gdscript
void remove_input_port_default_value(port: int)
```

Removes the default value of the input port.

### set_default_input_values {#set_default_input_values}

```gdscript
void set_default_input_values(values: Array)
```

Sets the default input ports values using an Array of the form [index0, value0, index1, value1, ...]. For example: [0, Vector3(0, 0, 0), 1, Vector3(0, 0, 0)].

### set_frame {#set_frame}

```gdscript
void set_frame(frame: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_port_default_value {#set_input_port_default_value}

```gdscript
void set_input_port_default_value(port: int, value: Variant, prev_value: Variant = null)
```

Sets the default value for the selected input port.

### set_output_port_for_preview {#set_output_port_for_preview}

```gdscript
void set_output_port_for_preview(port: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### output_port_for_preview {#output_port_for_preview}

```gdscript
output_port_for_preview: int
```

- void set_output_port_for_preview(value: int)
- int get_output_port_for_preview()

Sets the output port index which will be showed for preview. If set to -1 no port will be open for preview.

### default_input_values {#default_input_values}

```gdscript
default_input_values: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### expanded_output_ports {#expanded_output_ports}

```gdscript
expanded_output_ports: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### linked_parent_graph_frame {#linked_parent_graph_frame}

```gdscript
linked_parent_graph_frame: int
```

- void set_frame(value: int)
- int get_frame()

Represents the index of the frame this node is linked to. If set to -1 the node is not linked to any frame.

## 枚举

### PortType {#enum-PortType}

```gdscript
enum PortType { PORT_TYPE_SCALAR = 0, PORT_TYPE_SCALAR_INT = 1, PORT_TYPE_SCALAR_UINT = 2, PORT_TYPE_VECTOR_2D = 3, PORT_TYPE_VECTOR_3D = 4, PORT_TYPE_VECTOR_4D = 5, PORT_TYPE_BOOLEAN = 6, PORT_TYPE_TRANSFORM = 7, PORT_TYPE_SAMPLER = 8, PORT_TYPE_MAX = 9 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PORT_TYPE_SCALAR {#PORT_TYPE_SCALAR}

```gdscript
PORT_TYPE_SCALAR = 0
```

Floating-point scalar. Translated to float type in shader code.

#### PORT_TYPE_SCALAR_INT {#PORT_TYPE_SCALAR_INT}

```gdscript
PORT_TYPE_SCALAR_INT = 1
```

Integer scalar. Translated to int type in shader code.

#### PORT_TYPE_SCALAR_UINT {#PORT_TYPE_SCALAR_UINT}

```gdscript
PORT_TYPE_SCALAR_UINT = 2
```

Unsigned integer scalar. Translated to uint type in shader code.

#### PORT_TYPE_VECTOR_2D {#PORT_TYPE_VECTOR_2D}

```gdscript
PORT_TYPE_VECTOR_2D = 3
```

2D vector of floating-point values. Translated to vec2 type in shader code.

#### PORT_TYPE_VECTOR_3D {#PORT_TYPE_VECTOR_3D}

```gdscript
PORT_TYPE_VECTOR_3D = 4
```

3D vector of floating-point values. Translated to vec3 type in shader code.

#### PORT_TYPE_VECTOR_4D {#PORT_TYPE_VECTOR_4D}

```gdscript
PORT_TYPE_VECTOR_4D = 5
```

4D vector of floating-point values. Translated to vec4 type in shader code.

#### PORT_TYPE_BOOLEAN {#PORT_TYPE_BOOLEAN}

```gdscript
PORT_TYPE_BOOLEAN = 6
```

Boolean type. Translated to bool type in shader code.

#### PORT_TYPE_TRANSFORM {#PORT_TYPE_TRANSFORM}

```gdscript
PORT_TYPE_TRANSFORM = 7
```

Transform type. Translated to mat4 type in shader code.

#### PORT_TYPE_SAMPLER {#PORT_TYPE_SAMPLER}

```gdscript
PORT_TYPE_SAMPLER = 8
```

Sampler type. Translated to reference of sampler uniform in shader code. Can only be used for input ports in non-uniform nodes.

#### PORT_TYPE_MAX {#PORT_TYPE_MAX}

```gdscript
PORT_TYPE_MAX = 9
```

Represents the size of the PortType enum.

---
