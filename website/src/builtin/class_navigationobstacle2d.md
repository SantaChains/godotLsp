# NavigationObstacle2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **NavigationObstacle2D**

> An obstacle needs a navigation map and outline vertices defined to work correctly. The outlines can not cross or overlap.

<details><summary>官方英文描述（中文精译进行中）</summary>

An obstacle needs a navigation map and outline vertices defined to work correctly. The outlines can not cross or overlap.

Obstacles can be included in the navigation mesh baking process when affect_navigation_mesh is enabled. They do not add walkable geometry, instead their role is to discard other source geometry inside the shape. This can be used to prevent navigation mesh from appearing in unwanted places. If carve_navigation_mesh is enabled the baked shape will not be affected by offsets of the navigation mesh baking, e.g. the agent radius.

With avoidance_enabled the obstacle can constrain the avoidance velocities of avoidance using agents. If the obstacle's vertices are wound in clockwise order, avoidance agents will be pushed in by the obstacle, otherwise, avoidance agents will be pushed out. Obstacles using vertices and avoidance can warp to a new position but should not be moved every single frame as each change requires a rebuild of the avoidance map.

</details>

## 方法

### get_affect_navigation_mesh {#get_affect_navigation_mesh}

```gdscript
bool get_affect_navigation_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_avoidance_enabled {#get_avoidance_enabled}

```gdscript
bool get_avoidance_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_avoidance_layer_value {#get_avoidance_layer_value}

```gdscript
bool get_avoidance_layer_value(layer_number: int) const
```

Returns whether or not the specified layer of the avoidance_layers bitmask is enabled, given a layer_number between 1 and 32.

### get_avoidance_layers {#get_avoidance_layers}

```gdscript
int get_avoidance_layers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_carve_navigation_mesh {#get_carve_navigation_mesh}

```gdscript
bool get_carve_navigation_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

Returns the RID of the navigation map for this NavigationObstacle node. This function returns always the map set on the NavigationObstacle node and not the map of the abstract obstacle on the NavigationServer. If the obstacle map is changed directly with the NavigationServer API the NavigationObstacle node will not be aware of the map change. Use set_navigation_map() to change the navigation map for the NavigationObstacle and also update the obstacle on the NavigationServer.

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the RID of this obstacle on the NavigationServer2D.

### get_velocity {#get_velocity}

```gdscript
Vector2 get_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertices {#get_vertices}

```gdscript
PackedVector2Array get_vertices() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_affect_navigation_mesh {#set_affect_navigation_mesh}

```gdscript
void set_affect_navigation_mesh(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_avoidance_enabled {#set_avoidance_enabled}

```gdscript
void set_avoidance_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_avoidance_layer_value {#set_avoidance_layer_value}

```gdscript
void set_avoidance_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the avoidance_layers bitmask, given a layer_number between 1 and 32.

### set_avoidance_layers {#set_avoidance_layers}

```gdscript
void set_avoidance_layers(layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_carve_navigation_mesh {#set_carve_navigation_mesh}

```gdscript
void set_carve_navigation_mesh(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_map {#set_navigation_map}

```gdscript
void set_navigation_map(navigation_map: RID)
```

Sets the RID of the navigation map this NavigationObstacle node should use and also updates the obstacle on the NavigationServer.

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity {#set_velocity}

```gdscript
void set_velocity(velocity: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertices {#set_vertices}

```gdscript
void set_vertices(vertices: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

Sets the avoidance radius for the obstacle.

### vertices {#vertices}

```gdscript
vertices: PackedVector2Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### affect_navigation_mesh {#affect_navigation_mesh}

```gdscript
affect_navigation_mesh: bool
```

- void set_affect_navigation_mesh(value: bool)
- bool get_affect_navigation_mesh()

If enabled and parsed in a navigation mesh baking process the obstacle will discard source geometry inside its vertices defined shape.

### carve_navigation_mesh {#carve_navigation_mesh}

```gdscript
carve_navigation_mesh: bool
```

- void set_carve_navigation_mesh(value: bool)
- bool get_carve_navigation_mesh()

If enabled the obstacle vertices will carve into the baked navigation mesh with the shape unaffected by additional offsets (e.g. agent radius).

It will still be affected by further postprocessing of the baking process, like edge and polygon simplification.

Requires affect_navigation_mesh to be enabled.

### avoidance_enabled {#avoidance_enabled}

```gdscript
avoidance_enabled: bool
```

- void set_avoidance_enabled(value: bool)
- bool get_avoidance_enabled()

If true the obstacle affects avoidance using agents.

### velocity {#velocity}

```gdscript
velocity: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### avoidance_layers {#avoidance_layers}

```gdscript
avoidance_layers: int
```

- void set_avoidance_layers(value: int)
- int get_avoidance_layers()

A bitfield determining the avoidance layers for this obstacle. Agents with a matching bit on the their avoidance mask will avoid this obstacle.
