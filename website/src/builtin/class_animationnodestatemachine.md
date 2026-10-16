# AnimationNodeStateMachine

**继承**：[AnimationRootNode](class_animationrootnode.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeStateMachine**

> Contains multiple AnimationRootNodes representing animation states, connected in a graph. State transitions can be configured to happen automatically or via code, using a shortest-path algorithm. Retrieve the AnimationNodeStateMachinePlayback object from the AnimationTree node to control it programmatically.

<details><summary>官方英文描述（中文精译进行中）</summary>

Contains multiple AnimationRootNodes representing animation states, connected in a graph. State transitions can be configured to happen automatically or via code, using a shortest-path algorithm. Retrieve the AnimationNodeStateMachinePlayback object from the AnimationTree node to control it programmatically.

```
var state_machine = $AnimationTree.get("parameters/playback")
state_machine.travel("some_state")
```

```
var stateMachine = GetNode("AnimationTree").Get("parameters/playback") as AnimationNodeStateMachinePlayback;
stateMachine.Travel("some_state");
```

</details>

## 方法

### add_node {#add_node}

```gdscript
void add_node(name: StringName, node: AnimationNode, position: Vector2 = Vector2(0, 0))
```

Adds a new animation node to the graph. The position is used for display in the editor.

### add_transition {#add_transition}

```gdscript
void add_transition(from: StringName, to: StringName, transition: AnimationNodeStateMachineTransition)
```

Adds a transition between the given animation nodes.

### are_ends_reset {#are_ends_reset}

```gdscript
bool are_ends_reset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_graph_offset {#get_graph_offset}

```gdscript
Vector2 get_graph_offset() const
```

Returns the draw offset of the graph. Used for display in the editor.

### get_node {#get_node}

```gdscript
AnimationNode get_node(name: StringName) const
```

Returns the animation node with the given name.

### get_node_list {#get_node_list}

```gdscript
typedarray::StringName get_node_list() const
```

Returns a list containing the names of all animation nodes in this state machine.

### get_node_name {#get_node_name}

```gdscript
StringName get_node_name(node: AnimationNode) const
```

Returns the given animation node's name.

### get_node_position {#get_node_position}

```gdscript
Vector2 get_node_position(name: StringName) const
```

Returns the given animation node's coordinates. Used for display in the editor.

### get_state_machine_type {#get_state_machine_type}

```gdscript
AnimationNodeStateMachine.StateMachineType get_state_machine_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transition {#get_transition}

```gdscript
AnimationNodeStateMachineTransition get_transition(idx: int) const
```

Returns the given transition.

### get_transition_count {#get_transition_count}

```gdscript
int get_transition_count() const
```

Returns the number of connections in the graph.

### get_transition_from {#get_transition_from}

```gdscript
StringName get_transition_from(idx: int) const
```

Returns the given transition's start node.

### get_transition_to {#get_transition_to}

```gdscript
StringName get_transition_to(idx: int) const
```

Returns the given transition's end node.

### has_node {#has_node}

```gdscript
bool has_node(name: StringName) const
```

Returns true if the graph contains the given animation node.

### has_transition {#has_transition}

```gdscript
bool has_transition(from: StringName, to: StringName) const
```

Returns true if there is a transition between the given animation nodes.

### is_allow_transition_to_self {#is_allow_transition_to_self}

```gdscript
bool is_allow_transition_to_self() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_node {#remove_node}

```gdscript
void remove_node(name: StringName)
```

Deletes the given animation node from the graph.

### remove_transition {#remove_transition}

```gdscript
void remove_transition(from: StringName, to: StringName)
```

Deletes the transition between the two specified animation nodes.

### remove_transition_by_index {#remove_transition_by_index}

```gdscript
void remove_transition_by_index(idx: int)
```

Deletes the given transition by index.

### rename_node {#rename_node}

```gdscript
void rename_node(name: StringName, new_name: StringName)
```

Renames the given animation node.

### replace_node {#replace_node}

```gdscript
void replace_node(name: StringName, node: AnimationNode)
```

Replaces the given animation node with a new animation node.

### set_allow_transition_to_self {#set_allow_transition_to_self}

```gdscript
void set_allow_transition_to_self(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_graph_offset {#set_graph_offset}

```gdscript
void set_graph_offset(offset: Vector2)
```

Sets the draw offset of the graph. Used for display in the editor.

### set_node_position {#set_node_position}

```gdscript
void set_node_position(name: StringName, position: Vector2)
```

Sets the animation node's coordinates. Used for display in the editor.

### set_reset_ends {#set_reset_ends}

```gdscript
void set_reset_ends(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_state_machine_type {#set_state_machine_type}

```gdscript
void set_state_machine_type(state_machine_type: AnimationNodeStateMachine.StateMachineType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### state_machine_type {#state_machine_type}

```gdscript
state_machine_type: int
```

- void set_state_machine_type(value: StateMachineType)
- StateMachineType get_state_machine_type()

This property can define the process of transitions for different use cases. See also StateMachineType.

### allow_transition_to_self {#allow_transition_to_self}

```gdscript
allow_transition_to_self: bool
```

- void set_allow_transition_to_self(value: bool)
- bool is_allow_transition_to_self()

If true, allows teleport to the self state with AnimationNodeStateMachinePlayback.travel(). When the reset option is enabled in AnimationNodeStateMachinePlayback.travel(), the animation is restarted. If false, nothing happens on the teleportation to the self state.

### reset_ends {#reset_ends}

```gdscript
reset_ends: bool
```

- void set_reset_ends(value: bool)
- bool are_ends_reset()

If true, treat the cross-fade to the start and end nodes as a blend with the RESET animation.

In most cases, when additional cross-fades are performed in the parent AnimationNode of the state machine, setting this property to false and matching the cross-fade time of the parent AnimationNode and the state machine's start node and end node gives good results.

## 枚举

### StateMachineType {#enum-StateMachineType}

```gdscript
enum StateMachineType { STATE_MACHINE_TYPE_ROOT = 0, STATE_MACHINE_TYPE_NESTED = 1, STATE_MACHINE_TYPE_GROUPED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STATE_MACHINE_TYPE_ROOT {#STATE_MACHINE_TYPE_ROOT}

```gdscript
STATE_MACHINE_TYPE_ROOT = 0
```

Seeking to the beginning is treated as playing from the start state. Transition to the end state is treated as exiting the state machine.

#### STATE_MACHINE_TYPE_NESTED {#STATE_MACHINE_TYPE_NESTED}

```gdscript
STATE_MACHINE_TYPE_NESTED = 1
```

Seeking to the beginning is treated as seeking to the beginning of the animation in the current state. Transition to the end state, or the absence of transitions in each state, is treated as exiting the state machine.

#### STATE_MACHINE_TYPE_GROUPED {#STATE_MACHINE_TYPE_GROUPED}

```gdscript
STATE_MACHINE_TYPE_GROUPED = 2
```

This is a grouped state machine that can be controlled from a parent state machine. It does not work independently. There must be a state machine with state_machine_type of STATE_MACHINE_TYPE_ROOT or STATE_MACHINE_TYPE_NESTED in the parent or ancestor.

---
