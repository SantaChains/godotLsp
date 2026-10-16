# AnimationNodeBlendTree

**继承**：[AnimationRootNode](class_animationrootnode.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeBlendTree**

> This animation node may contain a sub-tree of any other type animation nodes, such as AnimationNodeTransition, AnimationNodeBlend2, AnimationNodeBlend3, AnimationNodeOneShot, etc. This is one of the most commonly used animation node roots.

<details><summary>官方英文描述（中文精译进行中）</summary>

This animation node may contain a sub-tree of any other type animation nodes, such as AnimationNodeTransition, AnimationNodeBlend2, AnimationNodeBlend3, AnimationNodeOneShot, etc. This is one of the most commonly used animation node roots.

An AnimationNodeOutput node named output is created by default.

</details>

## 方法

### add_node {#add_node}

```gdscript
void add_node(name: StringName, node: AnimationNode, position: Vector2 = Vector2(0, 0))
```

Adds an AnimationNode at the given position. The name is used to identify the created sub animation node later.

### connect_node {#connect_node}

```gdscript
void connect_node(input_node: StringName, input_index: int, output_node: StringName)
```

Connects the output of an AnimationNode as input for another AnimationNode, at the input port specified by input_index.

### disconnect_node {#disconnect_node}

```gdscript
void disconnect_node(input_node: StringName, input_index: int)
```

Disconnects the animation node connected to the specified input.

### get_graph_offset {#get_graph_offset}

```gdscript
Vector2 get_graph_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node {#get_node}

```gdscript
AnimationNode get_node(name: StringName) const
```

Returns the sub animation node with the specified name.

### get_node_list {#get_node_list}

```gdscript
typedarray::StringName get_node_list() const
```

Returns a list containing the names of all sub animation nodes in this blend tree.

### get_node_position {#get_node_position}

```gdscript
Vector2 get_node_position(name: StringName) const
```

Returns the position of the sub animation node with the specified name.

### has_node {#has_node}

```gdscript
bool has_node(name: StringName) const
```

Returns true if a sub animation node with specified name exists.

### remove_node {#remove_node}

```gdscript
void remove_node(name: StringName)
```

Removes a sub animation node.

### rename_node {#rename_node}

```gdscript
void rename_node(name: StringName, new_name: StringName)
```

Changes the name of a sub animation node.

### set_graph_offset {#set_graph_offset}

```gdscript
void set_graph_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_node_position {#set_node_position}

```gdscript
void set_node_position(name: StringName, position: Vector2)
```

Modifies the position of a sub animation node.

## 属性

### graph_offset {#graph_offset}

```gdscript
graph_offset: Vector2
```

- void set_graph_offset(value: Vector2)
- Vector2 get_graph_offset()

The global offset of all sub animation nodes.

## 信号

### node_changed {#node_changed}

```gdscript
signal node_changed(node_name: StringName)
```

Emitted when the input port information is changed.

## 常量

### CONNECTION_OK {#CONNECTION_OK}

```gdscript
const CONNECTION_OK = 0
```

The connection was successful.

CONNECTION_ERROR_NO_INPUT = 1 

The input node is null.

CONNECTION_ERROR_NO_INPUT_INDEX = 2 

The specified input port is out of range.

CONNECTION_ERROR_NO_OUTPUT = 3 

The output node is null.

CONNECTION_ERROR_SAME_NODE = 4 

Input and output nodes are the same.

CONNECTION_ERROR_CONNECTION_EXISTS = 5 

The specified connection already exists.

### CONNECTION_ERROR_NO_INPUT {#CONNECTION_ERROR_NO_INPUT}

```gdscript
const CONNECTION_ERROR_NO_INPUT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CONNECTION_ERROR_NO_INPUT_INDEX {#CONNECTION_ERROR_NO_INPUT_INDEX}

```gdscript
const CONNECTION_ERROR_NO_INPUT_INDEX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CONNECTION_ERROR_NO_OUTPUT {#CONNECTION_ERROR_NO_OUTPUT}

```gdscript
const CONNECTION_ERROR_NO_OUTPUT = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CONNECTION_ERROR_SAME_NODE {#CONNECTION_ERROR_SAME_NODE}

```gdscript
const CONNECTION_ERROR_SAME_NODE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CONNECTION_ERROR_CONNECTION_EXISTS {#CONNECTION_ERROR_CONNECTION_EXISTS}

```gdscript
const CONNECTION_ERROR_CONNECTION_EXISTS = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
