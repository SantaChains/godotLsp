# NavigationObstacle3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **NavigationObstacle3D**

> An obstacle needs a navigation map and outline vertices defined to work correctly. The outlines can not cross or overlap and are restricted to a plane projection. This means the y-axis of the vertices is ignored, instead the obstacle's global y-axis position is used for placement. The projected shape is extruded by the obstacles height along the y-axis.

<details><summary>官方英文描述（中文精译进行中）</summary>

An obstacle needs a navigation map and outline vertices defined to work correctly. The outlines can not cross or overlap and are restricted to a plane projection. This means the y-axis of the vertices is ignored, instead the obstacle's global y-axis position is used for placement. The projected shape is extruded by the obstacles height along the y-axis.

Obstacles can be included in the navigation mesh baking process when affect_navigation_mesh is enabled. They do not add walkable geometry, instead their role is to discard other source geometry inside the shape. This can be used to prevent navigation mesh from appearing in unwanted places, e.g. inside "solid" geometry or on top of it. If carve_navigation_mesh is enabled the baked shape will not be affected by offsets of the navigation mesh baking, e.g. the agent radius.

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

### get_height {#get_height}

```gdscript
float get_height() const
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

Returns the RID of this obstacle on the NavigationServer3D.

### get_use_3d_avoidance {#get_use_3d_avoidance}

```gdscript
bool get_use_3d_avoidance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_velocity {#get_velocity}

```gdscript
Vector3 get_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertices {#get_vertices}

```gdscript
PackedVector3Array get_vertices() const
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

### set_height {#set_height}

```gdscript
void set_height(height: float)
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

### set_use_3d_avoidance {#set_use_3d_avoidance}

```gdscript
void set_use_3d_avoidance(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity {#set_velocity}

```gdscript
void set_velocity(velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertices {#set_vertices}

```gdscript
void set_vertices(vertices: PackedVector3Array)
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

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

Sets the obstacle height used in 2D avoidance. 2D avoidance using agent's ignore obstacles that are below or above them.

### vertices {#vertices}

```gdscript
vertices: PackedVector3Array
```

- void set_vertices(value: PackedVector3Array)
- PackedVector3Array get_vertices()

The outline vertices of the obstacle. If the vertices are winded in clockwise order agents will be pushed in by the obstacle, else they will be pushed out. Outlines can not be crossed or overlap. Should the vertices using obstacle be warped to a new position agent's can not predict this movement and may get trapped inside the obstacle.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.

### affect_navigation_mesh {#affect_navigation_mesh}

```gdscript
affect_navigation_mesh: bool
```

- void set_affect_navigation_mesh(value: bool)
- bool get_affect_navigation_mesh()

If enabled and parsed in a navigation mesh baking process the obstacle will discard source geometry inside its vertices and height defined shape.

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
velocity: Vector3
```

- void set_velocity(value: Vector3)
- Vector3 get_velocity()

Sets the wanted velocity for the obstacle so other agent's can better predict the obstacle if it is moved with a velocity regularly (every frame) instead of warped to a new position. Does only affect avoidance for the obstacles radius. Does nothing for the obstacles static vertices.

### avoidance_layers {#avoidance_layers}

```gdscript
avoidance_layers: int
```

- void set_avoidance_layers(value: int)
- int get_avoidance_layers()

A bitfield determining the avoidance layers for this obstacle. Agents with a matching bit on the their avoidance mask will avoid this obstacle.

### use_3d_avoidance {#use_3d_avoidance}

```gdscript
use_3d_avoidance: bool
```

- void set_use_3d_avoidance(value: bool)
- bool get_use_3d_avoidance()

If true the obstacle affects 3D avoidance using agent's with obstacle radius.

If false the obstacle affects 2D avoidance using agent's with both obstacle vertices as well as obstacle radius.
