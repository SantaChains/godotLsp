# NavigationLink3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **NavigationLink3D**

> A link between two positions on NavigationRegion3Ds that agents can be routed through. These positions can be on the same NavigationRegion3D or on two different ones. Links are useful to express navigation methods other than traveling along the surface of the navigation mesh, such as ziplines, teleporters, or gaps that can be jumped across.

<details><summary>官方英文描述（中文精译进行中）</summary>

A link between two positions on NavigationRegion3Ds that agents can be routed through. These positions can be on the same NavigationRegion3D or on two different ones. Links are useful to express navigation methods other than traveling along the surface of the navigation mesh, such as ziplines, teleporters, or gaps that can be jumped across.

</details>

## 方法

### get_end_position {#get_end_position}

```gdscript
Vector3 get_end_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enter_cost {#get_enter_cost}

```gdscript
float get_enter_cost() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_end_position {#get_global_end_position}

```gdscript
Vector3 get_global_end_position() const
```

Returns the end_position that is relative to the link as a global position.

### get_global_start_position {#get_global_start_position}

```gdscript
Vector3 get_global_start_position() const
```

Returns the start_position that is relative to the link as a global position.

### get_navigation_layer_value {#get_navigation_layer_value}

```gdscript
bool get_navigation_layer_value(layer_number: int) const
```

Returns whether or not the specified layer of the navigation_layers bitmask is enabled, given a layer_number between 1 and 32.

### get_navigation_layers {#get_navigation_layers}

```gdscript
int get_navigation_layers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

Returns the current navigation map RID used by this link.

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the RID of this link on the NavigationServer3D.

### get_start_position {#get_start_position}

```gdscript
Vector3 get_start_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_travel_cost {#get_travel_cost}

```gdscript
float get_travel_cost() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_bidirectional {#is_bidirectional}

```gdscript
bool is_bidirectional() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bidirectional {#set_bidirectional}

```gdscript
void set_bidirectional(bidirectional: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_end_position {#set_end_position}

```gdscript
void set_end_position(position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enter_cost {#set_enter_cost}

```gdscript
void set_enter_cost(enter_cost: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_end_position {#set_global_end_position}

```gdscript
void set_global_end_position(position: Vector3)
```

Sets the end_position that is relative to the link from a global position.

### set_global_start_position {#set_global_start_position}

```gdscript
void set_global_start_position(position: Vector3)
```

Sets the start_position that is relative to the link from a global position.

### set_navigation_layer_value {#set_navigation_layer_value}

```gdscript
void set_navigation_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the navigation_layers bitmask, given a layer_number between 1 and 32.

### set_navigation_layers {#set_navigation_layers}

```gdscript
void set_navigation_layers(navigation_layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_map {#set_navigation_map}

```gdscript
void set_navigation_map(navigation_map: RID)
```

Sets the RID of the navigation map this link should use. By default the link will automatically join the World3D default navigation map so this function is only required to override the default map.

### set_start_position {#set_start_position}

```gdscript
void set_start_position(position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_travel_cost {#set_travel_cost}

```gdscript
void set_travel_cost(travel_cost: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

Whether this link is currently active. If false, NavigationServer3D.map_get_path() will ignore this link.

### bidirectional {#bidirectional}

```gdscript
bidirectional: bool
```

- void set_bidirectional(value: bool)
- bool is_bidirectional()

Whether this link can be traveled in both directions or only from start_position to end_position.

### navigation_layers {#navigation_layers}

```gdscript
navigation_layers: int
```

- void set_navigation_layers(value: int)
- int get_navigation_layers()

A bitfield determining all navigation layers the link belongs to. These navigation layers will be checked when requesting a path with NavigationServer3D.map_get_path().

### start_position {#start_position}

```gdscript
start_position: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### end_position {#end_position}

```gdscript
end_position: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### enter_cost {#enter_cost}

```gdscript
enter_cost: float
```

- void set_enter_cost(value: float)
- float get_enter_cost()

When pathfinding enters this link from another regions navigation mesh the enter_cost value is added to the path distance for determining the shortest path.

### travel_cost {#travel_cost}

```gdscript
travel_cost: float
```

- void set_travel_cost(value: float)
- float get_travel_cost()

When pathfinding moves along the link the traveled distance is multiplied with travel_cost for determining the shortest path.
