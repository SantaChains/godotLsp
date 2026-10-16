# NavigationRegion3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **NavigationRegion3D**

> A traversable 3D region based on a NavigationMesh that NavigationAgent3Ds can use for pathfinding.

<details><summary>官方英文描述</summary>

A traversable 3D region based on a NavigationMesh that NavigationAgent3Ds can use for pathfinding.

Two regions can be connected to each other if they share a similar edge. You can set the minimum distance between two vertices required to connect two edges by using NavigationServer3D.map_set_edge_connection_margin().

Note: Overlapping two regions' navigation meshes is not enough for connecting two regions. They must share a similar edge.

The cost of entering this region from another region can be controlled with the enter_cost value.

Note: This value is not added to the path cost when the start position is already inside this region.

The cost of traveling distances inside this region can be controlled with the travel_cost multiplier.

Note: This node caches changes to its properties, so if you make changes to the underlying region RID in NavigationServer3D, they will not be reflected in this node's properties.

</details>

## 方法

### bake_navigation_mesh {#bake_navigation_mesh}

```gdscript
void bake_navigation_mesh(on_thread: bool = true)
```

Bakes the NavigationMesh. If on_thread is set to true (default), the baking is done on a separate thread. Baking on separate thread is useful because navigation baking is not a cheap operation. When it is completed, it automatically sets the new NavigationMesh. Please note that baking on separate thread may be very slow if geometry is parsed from meshes as async access to each mesh involves heavy synchronization. Also, please note that baking on a separate thread is automatically disabled on operating systems that cannot use threads (such as Web with threads disabled).

### get_bounds {#get_bounds}

```gdscript
AABB get_bounds() const
```

Returns the axis-aligned bounding box for the region's transformed navigation mesh.

### get_enter_cost {#get_enter_cost}

```gdscript
float get_enter_cost() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

Returns the current navigation map RID used by this region.

### get_navigation_mesh {#get_navigation_mesh}

```gdscript
NavigationMesh get_navigation_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_region_rid {#get_region_rid}

```gdscript
RID get_region_rid() const
```

Deprecated: Use get_rid() instead.

Returns the RID of this region on the NavigationServer3D.

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the RID of this region on the NavigationServer3D. Combined with NavigationServer3D.map_get_closest_point_owner() can be used to identify the NavigationRegion3D closest to a point on the merged navigation map.

### get_travel_cost {#get_travel_cost}

```gdscript
float get_travel_cost() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_edge_connections {#get_use_edge_connections}

```gdscript
bool get_use_edge_connections() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_baking {#is_baking}

```gdscript
bool is_baking() const
```

Returns true when the NavigationMesh is being baked on a background thread.

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enter_cost {#set_enter_cost}

```gdscript
void set_enter_cost(enter_cost: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

Sets the RID of the navigation map this region should use. By default the region will automatically join the World3D default navigation map so this function is only required to override the default map.

### set_navigation_mesh {#set_navigation_mesh}

```gdscript
void set_navigation_mesh(navigation_mesh: NavigationMesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_travel_cost {#set_travel_cost}

```gdscript
void set_travel_cost(travel_cost: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_edge_connections {#set_use_edge_connections}

```gdscript
void set_use_edge_connections(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### navigation_mesh {#navigation_mesh}

```gdscript
navigation_mesh: NavigationMesh
```

- void set_navigation_mesh(value: NavigationMesh)
- NavigationMesh get_navigation_mesh()

The NavigationMesh resource to use.

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

Determines if the NavigationRegion3D is enabled or disabled.

### use_edge_connections {#use_edge_connections}

```gdscript
use_edge_connections: bool
```

- void set_use_edge_connections(value: bool)
- bool get_use_edge_connections()

If enabled the navigation region will use edge connections to connect with other navigation regions within proximity of the navigation map edge connection margin.

### navigation_layers {#navigation_layers}

```gdscript
navigation_layers: int
```

- void set_navigation_layers(value: int)
- int get_navigation_layers()

A bitfield determining all navigation layers the region belongs to. These navigation layers can be checked upon when requesting a path with NavigationServer3D.map_get_path().

### enter_cost {#enter_cost}

```gdscript
enter_cost: float
```

- void set_enter_cost(value: float)
- float get_enter_cost()

When pathfinding enters this region's navigation mesh from another regions navigation mesh the enter_cost value is added to the path distance for determining the shortest path.

### travel_cost {#travel_cost}

```gdscript
travel_cost: float
```

- void set_travel_cost(value: float)
- float get_travel_cost()

When pathfinding moves inside this region's navigation mesh the traveled distances are multiplied with travel_cost for determining the shortest path.

## 信号

### navigation_mesh_changed {#navigation_mesh_changed}

```gdscript
signal navigation_mesh_changed()
```

Notifies when the NavigationMesh has changed.

### bake_finished {#bake_finished}

```gdscript
signal bake_finished()
```

Notifies when the navigation mesh bake operation is completed.
