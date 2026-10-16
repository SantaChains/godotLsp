# NavigationRegion2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **NavigationRegion2D**

> A traversable 2D region based on a NavigationPolygon that NavigationAgent2Ds can use for pathfinding.

<details><summary>官方英文描述（中文精译进行中）</summary>

A traversable 2D region based on a NavigationPolygon that NavigationAgent2Ds can use for pathfinding.

Two regions can be connected to each other if they share a similar edge. You can set the minimum distance between two vertices required to connect two edges by using NavigationServer2D.map_set_edge_connection_margin().

Note: Overlapping two regions' navigation polygons is not enough for connecting two regions. They must share a similar edge.

The pathfinding cost of entering a region from another region can be controlled with the enter_cost value.

Note: This value is not added to the path cost when the start position is already inside this region.

The pathfinding cost of traveling distances inside this region can be controlled with the travel_cost multiplier.

Note: This node caches changes to its properties, so if you make changes to the underlying region RID in NavigationServer2D, they will not be reflected in this node's properties.

</details>

## 方法

### bake_navigation_polygon {#bake_navigation_polygon}

```gdscript
void bake_navigation_polygon(on_thread: bool = true)
```

Bakes the NavigationPolygon. If on_thread is set to true (default), the baking is done on a separate thread.

### get_bounds {#get_bounds}

```gdscript
Rect2 get_bounds() const
```

Returns the axis-aligned rectangle for the region's transformed navigation mesh.

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

### get_navigation_polygon {#get_navigation_polygon}

```gdscript
NavigationPolygon get_navigation_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_region_rid {#get_region_rid}

```gdscript
RID get_region_rid() const
```

Deprecated: Use get_rid() instead.

Returns the RID of this region on the NavigationServer2D.

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the RID of this region on the NavigationServer2D. Combined with NavigationServer2D.map_get_closest_point_owner() can be used to identify the NavigationRegion2D closest to a point on the merged navigation map.

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

Returns true when the NavigationPolygon is being baked on a background thread.

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

Sets the RID of the navigation map this region should use. By default the region will automatically join the World2D default navigation map so this function is only required to override the default map.

### set_navigation_polygon {#set_navigation_polygon}

```gdscript
void set_navigation_polygon(navigation_polygon: NavigationPolygon)
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

### navigation_polygon {#navigation_polygon}

```gdscript
navigation_polygon: NavigationPolygon
```

- void set_navigation_polygon(value: NavigationPolygon)
- NavigationPolygon get_navigation_polygon()

The NavigationPolygon resource to use.

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

Determines if the NavigationRegion2D is enabled or disabled.

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

A bitfield determining all navigation layers the region belongs to. These navigation layers can be checked upon when requesting a path with NavigationServer2D.map_get_path().

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

### navigation_polygon_changed {#navigation_polygon_changed}

```gdscript
signal navigation_polygon_changed()
```

Emitted when the used navigation polygon is replaced or changes to the internals of the current navigation polygon are committed.

### bake_finished {#bake_finished}

```gdscript
signal bake_finished()
```

Emitted when a navigation polygon bake operation is completed.
