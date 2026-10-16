# VisualShaderNodeGroupBase

**继承**：[VisualShaderNodeResizableBase](class_visualshadernoderesizablebase.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeGroupBase**

> Currently, has no direct usage, use the derived classes instead.

<details><summary>官方英文描述</summary>

Currently, has no direct usage, use the derived classes instead.

</details>

## 方法

### add_input_port {#add_input_port}

```gdscript
void add_input_port(id: int, type: int, name: String)
```

Adds an input port with the specified type (see PortType) and name.

### add_output_port {#add_output_port}

```gdscript
void add_output_port(id: int, type: int, name: String)
```

Adds an output port with the specified type (see PortType) and name.

### clear_input_ports {#clear_input_ports}

```gdscript
void clear_input_ports()
```

Removes all previously specified input ports.

### clear_output_ports {#clear_output_ports}

```gdscript
void clear_output_ports()
```

Removes all previously specified output ports.

### get_free_input_port_id {#get_free_input_port_id}

```gdscript
int get_free_input_port_id() const
```

Returns a free input port ID which can be used in add_input_port().

### get_free_output_port_id {#get_free_output_port_id}

```gdscript
int get_free_output_port_id() const
```

Returns a free output port ID which can be used in add_output_port().

### get_input_port_count {#get_input_port_count}

```gdscript
int get_input_port_count() const
```

Returns the number of input ports in use. Alternative for get_free_input_port_id().

### get_inputs {#get_inputs}

```gdscript
String get_inputs() const
```

Returns a String description of the input ports as a colon-separated list using the format id,type,name; (see add_input_port()).

### get_output_port_count {#get_output_port_count}

```gdscript
int get_output_port_count() const
```

Returns the number of output ports in use. Alternative for get_free_output_port_id().

### get_outputs {#get_outputs}

```gdscript
String get_outputs() const
```

Returns a String description of the output ports as a colon-separated list using the format id,type,name; (see add_output_port()).

### has_input_port {#has_input_port}

```gdscript
bool has_input_port(id: int) const
```

Returns true if the specified input port exists.

### has_output_port {#has_output_port}

```gdscript
bool has_output_port(id: int) const
```

Returns true if the specified output port exists.

### is_valid_port_name {#is_valid_port_name}

```gdscript
bool is_valid_port_name(name: String) const
```

Returns true if the specified port name does not override an existed port name and is valid within the shader.

### remove_input_port {#remove_input_port}

```gdscript
void remove_input_port(id: int)
```

Removes the specified input port.

### remove_output_port {#remove_output_port}

```gdscript
void remove_output_port(id: int)
```

Removes the specified output port.

### set_input_port_name {#set_input_port_name}

```gdscript
void set_input_port_name(id: int, name: String)
```

Renames the specified input port.

### set_input_port_type {#set_input_port_type}

```gdscript
void set_input_port_type(id: int, type: int)
```

Sets the specified input port's type (see PortType).

### set_inputs {#set_inputs}

```gdscript
void set_inputs(inputs: String)
```

Defines all input ports using a String formatted as a colon-separated list: id,type,name; (see add_input_port()).

### set_output_port_name {#set_output_port_name}

```gdscript
void set_output_port_name(id: int, name: String)
```

Renames the specified output port.

### set_output_port_type {#set_output_port_type}

```gdscript
void set_output_port_type(id: int, type: int)
```

Sets the specified output port's type (see PortType).

### set_outputs {#set_outputs}

```gdscript
void set_outputs(outputs: String)
```

Defines all output ports using a String formatted as a colon-separated list: id,type,name; (see add_output_port()).
