# TileData

**继承**：[Object](class_object.md) → **TileData**

> TileData object represents a single tile in a TileSet. It is usually edited using the tileset editor, but it can be modified at runtime using TileMapLayer._tile_data_runtime_update().

<details><summary>官方英文描述（中文精译进行中）</summary>

TileData object represents a single tile in a TileSet. It is usually edited using the tileset editor, but it can be modified at runtime using TileMapLayer._tile_data_runtime_update().

</details>

## 方法

### add_collision_polygon {#add_collision_polygon}

```gdscript
void add_collision_polygon(layer_id: int)
```

Adds a collision polygon to the tile on the given TileSet physics layer.

### add_occluder_polygon {#add_occluder_polygon}

```gdscript
void add_occluder_polygon(layer_id: int)
```

Adds an occlusion polygon to the tile on the TileSet occlusion layer with index layer_id.

### get_collision_polygon_one_way_margin {#get_collision_polygon_one_way_margin}

```gdscript
float get_collision_polygon_one_way_margin(layer_id: int, polygon_index: int) const
```

Returns the one-way margin (for one-way platforms) of the polygon at index polygon_index for TileSet physics layer with index layer_id.

### get_collision_polygon_points {#get_collision_polygon_points}

```gdscript
PackedVector2Array get_collision_polygon_points(layer_id: int, polygon_index: int) const
```

Returns the points of the polygon at index polygon_index for TileSet physics layer with index layer_id.

### get_collision_polygons_count {#get_collision_polygons_count}

```gdscript
int get_collision_polygons_count(layer_id: int) const
```

Returns how many polygons the tile has for TileSet physics layer with index layer_id.

### get_constant_angular_velocity {#get_constant_angular_velocity}

```gdscript
float get_constant_angular_velocity(layer_id: int) const
```

Returns the constant angular velocity applied to objects colliding with this tile.

### get_constant_linear_velocity {#get_constant_linear_velocity}

```gdscript
Vector2 get_constant_linear_velocity(layer_id: int) const
```

Returns the constant linear velocity applied to objects colliding with this tile.

### get_custom_data {#get_custom_data}

```gdscript
Variant get_custom_data(layer_name: String) const
```

Returns the custom data value for custom data layer named layer_name. To check if a custom data layer exists, use has_custom_data().

### get_custom_data_by_layer_id {#get_custom_data_by_layer_id}

```gdscript
Variant get_custom_data_by_layer_id(layer_id: int) const
```

Returns the custom data value for custom data layer with index layer_id.

### get_flip_h {#get_flip_h}

```gdscript
bool get_flip_h() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_flip_v {#get_flip_v}

```gdscript
bool get_flip_v() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_modulate {#get_modulate}

```gdscript
Color get_modulate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_polygon {#get_navigation_polygon}

```gdscript
NavigationPolygon get_navigation_polygon(layer_id: int, flip_h: bool = false, flip_v: bool = false, transpose: bool = false) const
```

Returns the navigation polygon of the tile for the TileSet navigation layer with index layer_id.

flip_h, flip_v, and transpose allow transforming the returned polygon.

### get_occluder {#get_occluder}

```gdscript
OccluderPolygon2D get_occluder(layer_id: int, flip_h: bool = false, flip_v: bool = false, transpose: bool = false) const
```

Deprecated: Use get_occluder_polygon() instead.

Returns the occluder polygon of the tile for the TileSet occlusion layer with index layer_id.

flip_h, flip_v, and transpose allow transforming the returned polygon.

### get_occluder_polygon {#get_occluder_polygon}

```gdscript
OccluderPolygon2D get_occluder_polygon(layer_id: int, polygon_index: int, flip_h: bool = false, flip_v: bool = false, transpose: bool = false) const
```

Returns the occluder polygon at index polygon_index from the TileSet occlusion layer with index layer_id.

The flip_h, flip_v, and transpose parameters can be true to transform the returned polygon.

### get_occluder_polygons_count {#get_occluder_polygons_count}

```gdscript
int get_occluder_polygons_count(layer_id: int) const
```

Returns the number of occluder polygons of the tile in the TileSet occlusion layer with index layer_id.

### get_probability {#get_probability}

```gdscript
float get_probability() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_terrain {#get_terrain}

```gdscript
int get_terrain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_terrain_peering_bit {#get_terrain_peering_bit}

```gdscript
int get_terrain_peering_bit(peering_bit: TileSet.CellNeighbor) const
```

Returns the tile's terrain bit for the given peering_bit direction. To check that a direction is valid, use is_valid_terrain_peering_bit().

### get_terrain_set {#get_terrain_set}

```gdscript
int get_terrain_set() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_origin {#get_texture_origin}

```gdscript
Vector2i get_texture_origin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transpose {#get_transpose}

```gdscript
bool get_transpose() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_y_sort_origin {#get_y_sort_origin}

```gdscript
int get_y_sort_origin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_z_index {#get_z_index}

```gdscript
int get_z_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_custom_data {#has_custom_data}

```gdscript
bool has_custom_data(layer_name: String) const
```

Returns whether there exists a custom data layer named layer_name.

### is_collision_polygon_one_way {#is_collision_polygon_one_way}

```gdscript
bool is_collision_polygon_one_way(layer_id: int, polygon_index: int) const
```

Returns whether one-way collisions are enabled for the polygon at index polygon_index for TileSet physics layer with index layer_id.

### is_valid_terrain_peering_bit {#is_valid_terrain_peering_bit}

```gdscript
bool is_valid_terrain_peering_bit(peering_bit: TileSet.CellNeighbor) const
```

Returns whether the given peering_bit direction is valid for this tile.

### remove_collision_polygon {#remove_collision_polygon}

```gdscript
void remove_collision_polygon(layer_id: int, polygon_index: int)
```

Removes the polygon at index polygon_index for TileSet physics layer with index layer_id.

### remove_occluder_polygon {#remove_occluder_polygon}

```gdscript
void remove_occluder_polygon(layer_id: int, polygon_index: int)
```

Removes the polygon at index polygon_index for TileSet occlusion layer with index layer_id.

### set_collision_polygon_one_way {#set_collision_polygon_one_way}

```gdscript
void set_collision_polygon_one_way(layer_id: int, polygon_index: int, one_way: bool)
```

Enables/disables one-way collisions on the polygon at index polygon_index for TileSet physics layer with index layer_id.

### set_collision_polygon_one_way_margin {#set_collision_polygon_one_way_margin}

```gdscript
void set_collision_polygon_one_way_margin(layer_id: int, polygon_index: int, one_way_margin: float)
```

Sets the one-way margin (for one-way platforms) of the polygon at index polygon_index for TileSet physics layer with index layer_id.

### set_collision_polygon_points {#set_collision_polygon_points}

```gdscript
void set_collision_polygon_points(layer_id: int, polygon_index: int, polygon: PackedVector2Array)
```

Sets the points of the polygon at index polygon_index for TileSet physics layer with index layer_id.

### set_collision_polygons_count {#set_collision_polygons_count}

```gdscript
void set_collision_polygons_count(layer_id: int, polygons_count: int)
```

Sets the polygons count for TileSet physics layer with index layer_id.

### set_constant_angular_velocity {#set_constant_angular_velocity}

```gdscript
void set_constant_angular_velocity(layer_id: int, velocity: float)
```

Sets the constant angular velocity. This does not rotate the tile. This angular velocity is applied to objects colliding with this tile.

### set_constant_linear_velocity {#set_constant_linear_velocity}

```gdscript
void set_constant_linear_velocity(layer_id: int, velocity: Vector2)
```

Sets the constant linear velocity. This does not move the tile. This linear velocity is applied to objects colliding with this tile. This is useful to create conveyor belts.

### set_custom_data {#set_custom_data}

```gdscript
void set_custom_data(layer_name: String, value: Variant)
```

Sets the tile's custom data value for the TileSet custom data layer with name layer_name.

### set_custom_data_by_layer_id {#set_custom_data_by_layer_id}

```gdscript
void set_custom_data_by_layer_id(layer_id: int, value: Variant)
```

Sets the tile's custom data value for the TileSet custom data layer with index layer_id.

### set_flip_h {#set_flip_h}

```gdscript
void set_flip_h(flip_h: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_v {#set_flip_v}

```gdscript
void set_flip_v(flip_v: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_modulate {#set_modulate}

```gdscript
void set_modulate(modulate: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_polygon {#set_navigation_polygon}

```gdscript
void set_navigation_polygon(layer_id: int, navigation_polygon: NavigationPolygon)
```

Sets the navigation polygon for the TileSet navigation layer with index layer_id.

### set_occluder {#set_occluder}

```gdscript
void set_occluder(layer_id: int, occluder_polygon: OccluderPolygon2D)
```

Deprecated: Use set_occluder_polygon() instead.

Sets the occluder for the TileSet occlusion layer with index layer_id.

### set_occluder_polygon {#set_occluder_polygon}

```gdscript
void set_occluder_polygon(layer_id: int, polygon_index: int, polygon: OccluderPolygon2D)
```

Sets the occluder for polygon with index polygon_index in the TileSet occlusion layer with index layer_id.

### set_occluder_polygons_count {#set_occluder_polygons_count}

```gdscript
void set_occluder_polygons_count(layer_id: int, polygons_count: int)
```

Sets the occluder polygon count in the TileSet occlusion layer with index layer_id.

### set_probability {#set_probability}

```gdscript
void set_probability(probability: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_terrain {#set_terrain}

```gdscript
void set_terrain(terrain: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_terrain_peering_bit {#set_terrain_peering_bit}

```gdscript
void set_terrain_peering_bit(peering_bit: TileSet.CellNeighbor, terrain: int)
```

Sets the tile's terrain bit for the given peering_bit direction. To check that a direction is valid, use is_valid_terrain_peering_bit().

### set_terrain_set {#set_terrain_set}

```gdscript
void set_terrain_set(terrain_set: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_origin {#set_texture_origin}

```gdscript
void set_texture_origin(texture_origin: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transpose {#set_transpose}

```gdscript
void set_transpose(transpose: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_y_sort_origin {#set_y_sort_origin}

```gdscript
void set_y_sort_origin(y_sort_origin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_z_index {#set_z_index}

```gdscript
void set_z_index(z_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### flip_h {#flip_h}

```gdscript
flip_h: bool
```

- void set_flip_h(value: bool)
- bool get_flip_h()

If true, the tile will have its texture flipped horizontally.

### flip_v {#flip_v}

```gdscript
flip_v: bool
```

- void set_flip_v(value: bool)
- bool get_flip_v()

If true, the tile will have its texture flipped vertically.

### transpose {#transpose}

```gdscript
transpose: bool
```

- void set_transpose(value: bool)
- bool get_transpose()

If true, the tile will display transposed, i.e. with horizontal and vertical texture UVs swapped.

### texture_origin {#texture_origin}

```gdscript
texture_origin: Vector2i
```

- void set_texture_origin(value: Vector2i)
- Vector2i get_texture_origin()

Offsets the position of where the tile is drawn.

### modulate {#modulate}

```gdscript
modulate: Color
```

- void set_modulate(value: Color)
- Color get_modulate()

Color modulation of the tile.

### material {#material}

```gdscript
material: CanvasItemMaterial,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The Material to use for this TileData. This can be a CanvasItemMaterial to use the default shader, or a ShaderMaterial to use a custom shader.

### z_index {#z_index}

```gdscript
z_index: int
```

- void set_z_index(value: int)
- int get_z_index()

Ordering index of this tile, relative to TileMapLayer.

### y_sort_origin {#y_sort_origin}

```gdscript
y_sort_origin: int
```

- void set_y_sort_origin(value: int)
- int get_y_sort_origin()

Vertical point of the tile used for determining y-sorted order.

### terrain_set {#terrain_set}

```gdscript
terrain_set: int
```

- void set_terrain_set(value: int)
- int get_terrain_set()

ID of the terrain set that the tile uses.

### terrain {#terrain}

```gdscript
terrain: int
```

- void set_terrain(value: int)
- int get_terrain()

ID of the terrain from the terrain set that the tile uses.

### probability {#probability}

```gdscript
probability: float
```

- void set_probability(value: float)
- float get_probability()

Relative probability of this tile being selected when drawing a pattern of random tiles.

## 信号

### changed {#changed}

```gdscript
signal changed()
```

Emitted when any of the properties are changed.
