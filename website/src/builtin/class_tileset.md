# TileSet

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TileSet**

> A TileSet is a library of tiles for a TileMapLayer. A TileSet handles a list of TileSetSource, each of them storing a set of tiles.

<details><summary>官方英文描述（中文精译进行中）</summary>

A TileSet is a library of tiles for a TileMapLayer. A TileSet handles a list of TileSetSource, each of them storing a set of tiles.

Tiles can either be from a TileSetAtlasSource, which renders tiles out of a texture with support for physics, navigation, etc., or from a TileSetScenesCollectionSource, which exposes scene-based tiles.

Tiles are referenced by using three IDs: their source ID, their atlas coordinates ID, and their alternative tile ID.

A TileSet can be configured so that its tiles expose more or fewer properties. To do so, the TileSet resources use property layers, which you can add or remove depending on your needs.

For example, adding a physics layer allows giving collision shapes to your tiles. Each layer has dedicated properties (physics layer and mask), so you may add several TileSet physics layers for each type of collision you need.

See the functions to add new layers for more information.

</details>

## 方法

### add_custom_data_layer {#add_custom_data_layer}

```gdscript
void add_custom_data_layer(to_position: int = -1)
```

Adds a custom data layer to the TileSet at the given position to_position in the array. If to_position is -1, adds it at the end of the array.

Custom data layers allow assigning custom properties to atlas tiles.

### add_navigation_layer {#add_navigation_layer}

```gdscript
void add_navigation_layer(to_position: int = -1)
```

Adds a navigation layer to the TileSet at the given position to_position in the array. If to_position is -1, adds it at the end of the array.

Navigation layers allow assigning a navigable area to atlas tiles.

### add_occlusion_layer {#add_occlusion_layer}

```gdscript
void add_occlusion_layer(to_position: int = -1)
```

Adds an occlusion layer to the TileSet at the given position to_position in the array. If to_position is -1, adds it at the end of the array.

Occlusion layers allow assigning occlusion polygons to atlas tiles.

### add_pattern {#add_pattern}

```gdscript
int add_pattern(pattern: TileMapPattern, index: int = -1)
```

Adds a TileMapPattern to be stored in the TileSet resource. If provided, insert it at the given index.

### add_physics_layer {#add_physics_layer}

```gdscript
void add_physics_layer(to_position: int = -1)
```

Adds a physics layer to the TileSet at the given position to_position in the array. If to_position is -1, adds it at the end of the array.

Physics layers allow assigning collision polygons to atlas tiles.

### add_source {#add_source}

```gdscript
int add_source(source: TileSetSource, atlas_source_id_override: int = -1)
```

Adds a TileSetSource to the TileSet. If atlas_source_id_override is not -1, also set its source ID. Otherwise, a unique identifier is automatically generated.

The function returns the added source ID or -1 if the source could not be added.

Warning: A source cannot belong to two TileSets at the same time. If the added source was attached to another TileSet, it will be removed from that one.

### add_terrain {#add_terrain}

```gdscript
void add_terrain(terrain_set: int, to_position: int = -1)
```

Adds a new terrain to the given terrain set terrain_set at the given position to_position in the array. If to_position is -1, adds it at the end of the array.

### add_terrain_set {#add_terrain_set}

```gdscript
void add_terrain_set(to_position: int = -1)
```

Adds a new terrain set at the given position to_position in the array. If to_position is -1, adds it at the end of the array.

### cleanup_invalid_tile_proxies {#cleanup_invalid_tile_proxies}

```gdscript
void cleanup_invalid_tile_proxies()
```

Clears tile proxies pointing to invalid tiles.

### clear_terrains {#clear_terrains}

```gdscript
void clear_terrains(terrain_set: int)
```

Clears all terrain properties for the given terrain set.

### clear_tile_proxies {#clear_tile_proxies}

```gdscript
void clear_tile_proxies()
```

Clears all tile proxies.

### get_alternative_level_tile_proxy {#get_alternative_level_tile_proxy}

```gdscript
Array get_alternative_level_tile_proxy(source_from: int, coords_from: Vector2i, alternative_from: int)
```

Returns the alternative-level proxy for the given identifiers. The returned array contains the three proxie's target identifiers (source ID, atlas coords ID and alternative tile ID).

If the TileSet has no proxy for the given identifiers, returns an empty Array.

### get_coords_level_tile_proxy {#get_coords_level_tile_proxy}

```gdscript
Array get_coords_level_tile_proxy(source_from: int, coords_from: Vector2i)
```

Returns the coordinate-level proxy for the given identifiers. The returned array contains the two target identifiers of the proxy (source ID and atlas coordinates ID).

If the TileSet has no proxy for the given identifiers, returns an empty Array.

### get_custom_data_layer_by_name {#get_custom_data_layer_by_name}

```gdscript
int get_custom_data_layer_by_name(layer_name: String) const
```

Returns the index of the custom data layer identified by the given name.

### get_custom_data_layer_name {#get_custom_data_layer_name}

```gdscript
String get_custom_data_layer_name(layer_index: int) const
```

Returns the name of the custom data layer identified by the given index.

### get_custom_data_layer_type {#get_custom_data_layer_type}

```gdscript
Variant.Type get_custom_data_layer_type(layer_index: int) const
```

Returns the type of the custom data layer identified by the given index.

### get_custom_data_layers_count {#get_custom_data_layers_count}

```gdscript
int get_custom_data_layers_count() const
```

Returns the custom data layers count.

### get_navigation_layer_layer_value {#get_navigation_layer_layer_value}

```gdscript
bool get_navigation_layer_layer_value(layer_index: int, layer_number: int) const
```

Returns whether or not the specified navigation layer of the TileSet navigation data layer identified by the given layer_index is enabled, given a navigation_layers layer_number between 1 and 32.

### get_navigation_layer_layers {#get_navigation_layer_layers}

```gdscript
int get_navigation_layer_layers(layer_index: int) const
```

Returns the navigation layers (as in the Navigation server) of the given TileSet navigation layer.

### get_navigation_layers_count {#get_navigation_layers_count}

```gdscript
int get_navigation_layers_count() const
```

Returns the navigation layers count.

### get_next_source_id {#get_next_source_id}

```gdscript
int get_next_source_id() const
```

Returns a new unused source ID. This generated ID is the same that a call to add_source() would return.

### get_occlusion_layer_light_mask {#get_occlusion_layer_light_mask}

```gdscript
int get_occlusion_layer_light_mask(layer_index: int) const
```

Returns the light mask of the occlusion layer.

### get_occlusion_layer_sdf_collision {#get_occlusion_layer_sdf_collision}

```gdscript
bool get_occlusion_layer_sdf_collision(layer_index: int) const
```

Returns if the occluders from this layer use sdf_collision.

### get_occlusion_layers_count {#get_occlusion_layers_count}

```gdscript
int get_occlusion_layers_count() const
```

Returns the occlusion layers count.

### get_pattern {#get_pattern}

```gdscript
TileMapPattern get_pattern(index: int = -1)
```

Returns the TileMapPattern at the given index.

### get_patterns_count {#get_patterns_count}

```gdscript
int get_patterns_count()
```

Returns the number of TileMapPattern this tile set handles.

### get_physics_layer_collision_layer {#get_physics_layer_collision_layer}

```gdscript
int get_physics_layer_collision_layer(layer_index: int) const
```

Returns the collision layer (as in the physics server) bodies on the given TileSet's physics layer are in.

### get_physics_layer_collision_mask {#get_physics_layer_collision_mask}

```gdscript
int get_physics_layer_collision_mask(layer_index: int) const
```

Returns the collision mask of bodies on the given TileSet's physics layer.

### get_physics_layer_collision_priority {#get_physics_layer_collision_priority}

```gdscript
float get_physics_layer_collision_priority(layer_index: int) const
```

Returns the collision priority of bodies on the given TileSet's physics layer.

### get_physics_layer_physics_material {#get_physics_layer_physics_material}

```gdscript
PhysicsMaterial get_physics_layer_physics_material(layer_index: int) const
```

Returns the physics material of bodies on the given TileSet's physics layer.

### get_physics_layers_count {#get_physics_layers_count}

```gdscript
int get_physics_layers_count() const
```

Returns the physics layers count.

### get_source {#get_source}

```gdscript
TileSetSource get_source(source_id: int) const
```

Returns the TileSetSource with ID source_id.

### get_source_count {#get_source_count}

```gdscript
int get_source_count() const
```

Returns the number of TileSetSource in this TileSet.

### get_source_id {#get_source_id}

```gdscript
int get_source_id(index: int) const
```

Returns the source ID for source with index index.

### get_source_level_tile_proxy {#get_source_level_tile_proxy}

```gdscript
int get_source_level_tile_proxy(source_from: int)
```

Returns the source-level proxy for the given source identifier.

If the TileSet has no proxy for the given identifier, returns -1.

### get_terrain_color {#get_terrain_color}

```gdscript
Color get_terrain_color(terrain_set: int, terrain_index: int) const
```

Returns a terrain's color.

### get_terrain_name {#get_terrain_name}

```gdscript
String get_terrain_name(terrain_set: int, terrain_index: int) const
```

Returns a terrain's name.

### get_terrain_set_mode {#get_terrain_set_mode}

```gdscript
TileSet.TerrainMode get_terrain_set_mode(terrain_set: int) const
```

Returns a terrain set mode.

### get_terrain_sets_count {#get_terrain_sets_count}

```gdscript
int get_terrain_sets_count() const
```

Returns the terrain sets count.

### get_terrains_count {#get_terrains_count}

```gdscript
int get_terrains_count(terrain_set: int) const
```

Returns the number of terrains in the given terrain set.

### get_tile_layout {#get_tile_layout}

```gdscript
TileSet.TileLayout get_tile_layout() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tile_offset_axis {#get_tile_offset_axis}

```gdscript
TileSet.TileOffsetAxis get_tile_offset_axis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tile_shape {#get_tile_shape}

```gdscript
TileSet.TileShape get_tile_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tile_size {#get_tile_size}

```gdscript
Vector2i get_tile_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_alternative_level_tile_proxy {#has_alternative_level_tile_proxy}

```gdscript
bool has_alternative_level_tile_proxy(source_from: int, coords_from: Vector2i, alternative_from: int)
```

Returns if there is an alternative-level proxy for the given identifiers.

### has_coords_level_tile_proxy {#has_coords_level_tile_proxy}

```gdscript
bool has_coords_level_tile_proxy(source_from: int, coords_from: Vector2i)
```

Returns if there is a coodinates-level proxy for the given identifiers.

### has_custom_data_layer_by_name {#has_custom_data_layer_by_name}

```gdscript
bool has_custom_data_layer_by_name(layer_name: String) const
```

Returns if there is a custom data layer named layer_name.

### has_source {#has_source}

```gdscript
bool has_source(source_id: int) const
```

Returns if this TileSet has a source for the given source ID.

### has_source_level_tile_proxy {#has_source_level_tile_proxy}

```gdscript
bool has_source_level_tile_proxy(source_from: int)
```

Returns if there is a source-level proxy for the given source ID.

### is_uv_clipping {#is_uv_clipping}

```gdscript
bool is_uv_clipping() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### map_tile_proxy {#map_tile_proxy}

```gdscript
Array map_tile_proxy(source_from: int, coords_from: Vector2i, alternative_from: int) const
```

According to the configured proxies, maps the provided identifiers to a new set of identifiers. The source ID, atlas coordinates ID and alternative tile ID are returned as a 3 elements Array.

This function first look for matching alternative-level proxies, then coordinates-level proxies, then source-level proxies.

If no proxy corresponding to provided identifiers are found, returns the same values the ones used as arguments.

### move_custom_data_layer {#move_custom_data_layer}

```gdscript
void move_custom_data_layer(layer_index: int, to_position: int)
```

Moves the custom data layer at index layer_index to the given position to_position in the array. Also updates the atlas tiles accordingly.

### move_navigation_layer {#move_navigation_layer}

```gdscript
void move_navigation_layer(layer_index: int, to_position: int)
```

Moves the navigation layer at index layer_index to the given position to_position in the array. Also updates the atlas tiles accordingly.

### move_occlusion_layer {#move_occlusion_layer}

```gdscript
void move_occlusion_layer(layer_index: int, to_position: int)
```

Moves the occlusion layer at index layer_index to the given position to_position in the array. Also updates the atlas tiles accordingly.

### move_physics_layer {#move_physics_layer}

```gdscript
void move_physics_layer(layer_index: int, to_position: int)
```

Moves the physics layer at index layer_index to the given position to_position in the array. Also updates the atlas tiles accordingly.

### move_terrain {#move_terrain}

```gdscript
void move_terrain(terrain_set: int, terrain_index: int, to_position: int)
```

Moves the terrain at index terrain_index for terrain set terrain_set to the given position to_position in the array. Also updates the atlas tiles accordingly.

### move_terrain_set {#move_terrain_set}

```gdscript
void move_terrain_set(terrain_set: int, to_position: int)
```

Moves the terrain set at index terrain_set to the given position to_position in the array. Also updates the atlas tiles accordingly.

### remove_alternative_level_tile_proxy {#remove_alternative_level_tile_proxy}

```gdscript
void remove_alternative_level_tile_proxy(source_from: int, coords_from: Vector2i, alternative_from: int)
```

Removes an alternative-level proxy for the given identifiers.

### remove_coords_level_tile_proxy {#remove_coords_level_tile_proxy}

```gdscript
void remove_coords_level_tile_proxy(source_from: int, coords_from: Vector2i)
```

Removes a coordinates-level proxy for the given identifiers.

### remove_custom_data_layer {#remove_custom_data_layer}

```gdscript
void remove_custom_data_layer(layer_index: int)
```

Removes the custom data layer at index layer_index. Also updates the atlas tiles accordingly.

### remove_navigation_layer {#remove_navigation_layer}

```gdscript
void remove_navigation_layer(layer_index: int)
```

Removes the navigation layer at index layer_index. Also updates the atlas tiles accordingly.

### remove_occlusion_layer {#remove_occlusion_layer}

```gdscript
void remove_occlusion_layer(layer_index: int)
```

Removes the occlusion layer at index layer_index. Also updates the atlas tiles accordingly.

### remove_pattern {#remove_pattern}

```gdscript
void remove_pattern(index: int)
```

Remove the TileMapPattern at the given index.

### remove_physics_layer {#remove_physics_layer}

```gdscript
void remove_physics_layer(layer_index: int)
```

Removes the physics layer at index layer_index. Also updates the atlas tiles accordingly.

### remove_source {#remove_source}

```gdscript
void remove_source(source_id: int)
```

Removes the source with the given source ID.

### remove_source_level_tile_proxy {#remove_source_level_tile_proxy}

```gdscript
void remove_source_level_tile_proxy(source_from: int)
```

Removes a source-level tile proxy.

### remove_terrain {#remove_terrain}

```gdscript
void remove_terrain(terrain_set: int, terrain_index: int)
```

Removes the terrain at index terrain_index in the given terrain set terrain_set. Also updates the atlas tiles accordingly.

### remove_terrain_set {#remove_terrain_set}

```gdscript
void remove_terrain_set(terrain_set: int)
```

Removes the terrain set at index terrain_set. Also updates the atlas tiles accordingly.

### set_alternative_level_tile_proxy {#set_alternative_level_tile_proxy}

```gdscript
void set_alternative_level_tile_proxy(source_from: int, coords_from: Vector2i, alternative_from: int, source_to: int, coords_to: Vector2i, alternative_to: int)
```

Create an alternative-level proxy for the given identifiers. A proxy will map set of tile identifiers to another set of identifiers.

Proxied tiles can be automatically replaced in TileMapLayer nodes using the editor.

### set_coords_level_tile_proxy {#set_coords_level_tile_proxy}

```gdscript
void set_coords_level_tile_proxy(source_from: int, coords_from: Vector2i, source_to: int, coords_to: Vector2i)
```

Creates a coordinates-level proxy for the given identifiers. A proxy will map set of tile identifiers to another set of identifiers. The alternative tile ID is kept the same when using coordinates-level proxies.

Proxied tiles can be automatically replaced in TileMapLayer nodes using the editor.

### set_custom_data_layer_name {#set_custom_data_layer_name}

```gdscript
void set_custom_data_layer_name(layer_index: int, layer_name: String)
```

Sets the name of the custom data layer identified by the given index. Names are identifiers of the layer therefore if the name is already taken it will fail and raise an error.

### set_custom_data_layer_type {#set_custom_data_layer_type}

```gdscript
void set_custom_data_layer_type(layer_index: int, layer_type: Variant.Type)
```

Sets the type of the custom data layer identified by the given index.

### set_navigation_layer_layer_value {#set_navigation_layer_layer_value}

```gdscript
void set_navigation_layer_layer_value(layer_index: int, layer_number: int, value: bool)
```

Based on value, enables or disables the specified navigation layer of the TileSet navigation data layer identified by the given layer_index, given a navigation_layers layer_number between 1 and 32.

### set_navigation_layer_layers {#set_navigation_layer_layers}

```gdscript
void set_navigation_layer_layers(layer_index: int, layers: int)
```

Sets the navigation layers (as in the navigation server) for navigation regions in the given TileSet navigation layer.

### set_occlusion_layer_light_mask {#set_occlusion_layer_light_mask}

```gdscript
void set_occlusion_layer_light_mask(layer_index: int, light_mask: int)
```

Sets the occlusion layer (as in the rendering server) for occluders in the given TileSet occlusion layer.

### set_occlusion_layer_sdf_collision {#set_occlusion_layer_sdf_collision}

```gdscript
void set_occlusion_layer_sdf_collision(layer_index: int, sdf_collision: bool)
```

Enables or disables SDF collision for occluders in the given TileSet occlusion layer.

### set_physics_layer_collision_layer {#set_physics_layer_collision_layer}

```gdscript
void set_physics_layer_collision_layer(layer_index: int, layer: int)
```

Sets the collision layer (as in the physics server) for bodies in the given TileSet physics layer.

### set_physics_layer_collision_mask {#set_physics_layer_collision_mask}

```gdscript
void set_physics_layer_collision_mask(layer_index: int, mask: int)
```

Sets the collision mask for bodies in the given TileSet physics layer.

### set_physics_layer_collision_priority {#set_physics_layer_collision_priority}

```gdscript
void set_physics_layer_collision_priority(layer_index: int, priority: float)
```

Sets the collision priority for bodies in the given TileSet physics layer.

### set_physics_layer_physics_material {#set_physics_layer_physics_material}

```gdscript
void set_physics_layer_physics_material(layer_index: int, physics_material: PhysicsMaterial)
```

Sets the physics material for bodies in the given TileSet physics layer.

### set_source_id {#set_source_id}

```gdscript
void set_source_id(source_id: int, new_source_id: int)
```

Changes a source's ID.

### set_source_level_tile_proxy {#set_source_level_tile_proxy}

```gdscript
void set_source_level_tile_proxy(source_from: int, source_to: int)
```

Creates a source-level proxy for the given source ID. A proxy will map set of tile identifiers to another set of identifiers. Both the atlas coordinates ID and the alternative tile ID are kept the same when using source-level proxies.

Proxied tiles can be automatically replaced in TileMapLayer nodes using the editor.

### set_terrain_color {#set_terrain_color}

```gdscript
void set_terrain_color(terrain_set: int, terrain_index: int, color: Color)
```

Sets a terrain's color. This color is used for identifying the different terrains in the TileSet editor.

### set_terrain_name {#set_terrain_name}

```gdscript
void set_terrain_name(terrain_set: int, terrain_index: int, name: String)
```

Sets a terrain's name.

### set_terrain_set_mode {#set_terrain_set_mode}

```gdscript
void set_terrain_set_mode(terrain_set: int, mode: TileSet.TerrainMode)
```

Sets a terrain mode. Each mode determines which bits of a tile shape is used to match the neighboring tiles' terrains.

### set_tile_layout {#set_tile_layout}

```gdscript
void set_tile_layout(layout: TileSet.TileLayout)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_offset_axis {#set_tile_offset_axis}

```gdscript
void set_tile_offset_axis(alignment: TileSet.TileOffsetAxis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_shape {#set_tile_shape}

```gdscript
void set_tile_shape(shape: TileSet.TileShape)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_size {#set_tile_size}

```gdscript
void set_tile_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_uv_clipping {#set_uv_clipping}

```gdscript
void set_uv_clipping(uv_clipping: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### tile_shape {#tile_shape}

```gdscript
tile_shape: int
```

- void set_tile_shape(value: TileShape)
- TileShape get_tile_shape()

The tile shape.

### tile_layout {#tile_layout}

```gdscript
tile_layout: int
```

- void set_tile_layout(value: TileLayout)
- TileLayout get_tile_layout()

For all half-offset shapes (Isometric, Hexagonal and Half-Offset square), changes the way tiles are indexed in the TileMapLayer grid.

### tile_offset_axis {#tile_offset_axis}

```gdscript
tile_offset_axis: int
```

- void set_tile_offset_axis(value: TileOffsetAxis)
- TileOffsetAxis get_tile_offset_axis()

For all half-offset shapes (Isometric, Hexagonal and Half-Offset square), determines the offset axis.

### tile_size {#tile_size}

```gdscript
tile_size: Vector2i
```

- void set_tile_size(value: Vector2i)
- Vector2i get_tile_size()

The tile size, in pixels. For all tile shapes, this size corresponds to the encompassing rectangle of the tile shape. This is thus the minimal cell size required in an atlas.

### uv_clipping {#uv_clipping}

```gdscript
uv_clipping: bool
```

- void set_uv_clipping(value: bool)
- bool is_uv_clipping()

Enables/Disable uv clipping when rendering the tiles.

## 枚举

### TileShape {#enum-TileShape}

```gdscript
enum TileShape { TILE_SHAPE_SQUARE = 0, TILE_SHAPE_ISOMETRIC = 1, TILE_SHAPE_HALF_OFFSET_SQUARE = 2, TILE_SHAPE_HEXAGON = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TILE_SHAPE_SQUARE {#TILE_SHAPE_SQUARE}

```gdscript
TILE_SHAPE_SQUARE = 0
```

Rectangular tile shape.

#### TILE_SHAPE_ISOMETRIC {#TILE_SHAPE_ISOMETRIC}

```gdscript
TILE_SHAPE_ISOMETRIC = 1
```

Diamond tile shape (for isometric look).

Note: Isometric TileSet works best if all sibling TileMapLayers and their parent inheriting from Node2D have Y-sort enabled.

#### TILE_SHAPE_HALF_OFFSET_SQUARE {#TILE_SHAPE_HALF_OFFSET_SQUARE}

```gdscript
TILE_SHAPE_HALF_OFFSET_SQUARE = 2
```

Rectangular tile shape with one row/column out of two offset by half a tile.

#### TILE_SHAPE_HEXAGON {#TILE_SHAPE_HEXAGON}

```gdscript
TILE_SHAPE_HEXAGON = 3
```

Hexagonal tile shape.

---

### TileLayout {#enum-TileLayout}

```gdscript
enum TileLayout { TILE_LAYOUT_STACKED = 0, TILE_LAYOUT_STACKED_OFFSET = 1, TILE_LAYOUT_STAIRS_RIGHT = 2, TILE_LAYOUT_STAIRS_DOWN = 3, TILE_LAYOUT_DIAMOND_RIGHT = 4, TILE_LAYOUT_DIAMOND_DOWN = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TILE_LAYOUT_STACKED {#TILE_LAYOUT_STACKED}

```gdscript
TILE_LAYOUT_STACKED = 0
```

Tile coordinates layout where both axis stay consistent with their respective local horizontal and vertical axis.

#### TILE_LAYOUT_STACKED_OFFSET {#TILE_LAYOUT_STACKED_OFFSET}

```gdscript
TILE_LAYOUT_STACKED_OFFSET = 1
```

Same as TILE_LAYOUT_STACKED, but the first half-offset is negative instead of positive.

#### TILE_LAYOUT_STAIRS_RIGHT {#TILE_LAYOUT_STAIRS_RIGHT}

```gdscript
TILE_LAYOUT_STAIRS_RIGHT = 2
```

Tile coordinates layout where the horizontal axis stay horizontal, and the vertical one goes down-right.

#### TILE_LAYOUT_STAIRS_DOWN {#TILE_LAYOUT_STAIRS_DOWN}

```gdscript
TILE_LAYOUT_STAIRS_DOWN = 3
```

Tile coordinates layout where the vertical axis stay vertical, and the horizontal one goes down-right.

#### TILE_LAYOUT_DIAMOND_RIGHT {#TILE_LAYOUT_DIAMOND_RIGHT}

```gdscript
TILE_LAYOUT_DIAMOND_RIGHT = 4
```

Tile coordinates layout where the horizontal axis goes up-right, and the vertical one goes down-right.

#### TILE_LAYOUT_DIAMOND_DOWN {#TILE_LAYOUT_DIAMOND_DOWN}

```gdscript
TILE_LAYOUT_DIAMOND_DOWN = 5
```

Tile coordinates layout where the horizontal axis goes down-right, and the vertical one goes down-left.

---

### TileOffsetAxis {#enum-TileOffsetAxis}

```gdscript
enum TileOffsetAxis { TILE_OFFSET_AXIS_HORIZONTAL = 0, TILE_OFFSET_AXIS_VERTICAL = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TILE_OFFSET_AXIS_HORIZONTAL {#TILE_OFFSET_AXIS_HORIZONTAL}

```gdscript
TILE_OFFSET_AXIS_HORIZONTAL = 0
```

Horizontal half-offset.

#### TILE_OFFSET_AXIS_VERTICAL {#TILE_OFFSET_AXIS_VERTICAL}

```gdscript
TILE_OFFSET_AXIS_VERTICAL = 1
```

Vertical half-offset.

---

### CellNeighbor {#enum-CellNeighbor}

```gdscript
enum CellNeighbor { CELL_NEIGHBOR_RIGHT_SIDE = 0, CELL_NEIGHBOR_RIGHT_CORNER = 1, CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE = 2, CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER = 3, CELL_NEIGHBOR_BOTTOM_SIDE = 4, CELL_NEIGHBOR_BOTTOM_CORNER = 5, CELL_NEIGHBOR_BOTTOM_LEFT_SIDE = 6, CELL_NEIGHBOR_BOTTOM_LEFT_CORNER = 7, CELL_NEIGHBOR_LEFT_SIDE = 8, CELL_NEIGHBOR_LEFT_CORNER = 9, CELL_NEIGHBOR_TOP_LEFT_SIDE = 10, CELL_NEIGHBOR_TOP_LEFT_CORNER = 11, CELL_NEIGHBOR_TOP_SIDE = 12, CELL_NEIGHBOR_TOP_CORNER = 13, CELL_NEIGHBOR_TOP_RIGHT_SIDE = 14, CELL_NEIGHBOR_TOP_RIGHT_CORNER = 15 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CELL_NEIGHBOR_RIGHT_SIDE {#CELL_NEIGHBOR_RIGHT_SIDE}

```gdscript
CELL_NEIGHBOR_RIGHT_SIDE = 0
```

Neighbor on the right side.

#### CELL_NEIGHBOR_RIGHT_CORNER {#CELL_NEIGHBOR_RIGHT_CORNER}

```gdscript
CELL_NEIGHBOR_RIGHT_CORNER = 1
```

Neighbor in the right corner.

#### CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE {#CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE}

```gdscript
CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE = 2
```

Neighbor on the bottom right side.

#### CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER {#CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER}

```gdscript
CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER = 3
```

Neighbor in the bottom right corner.

#### CELL_NEIGHBOR_BOTTOM_SIDE {#CELL_NEIGHBOR_BOTTOM_SIDE}

```gdscript
CELL_NEIGHBOR_BOTTOM_SIDE = 4
```

Neighbor on the bottom side.

#### CELL_NEIGHBOR_BOTTOM_CORNER {#CELL_NEIGHBOR_BOTTOM_CORNER}

```gdscript
CELL_NEIGHBOR_BOTTOM_CORNER = 5
```

Neighbor in the bottom corner.

#### CELL_NEIGHBOR_BOTTOM_LEFT_SIDE {#CELL_NEIGHBOR_BOTTOM_LEFT_SIDE}

```gdscript
CELL_NEIGHBOR_BOTTOM_LEFT_SIDE = 6
```

Neighbor on the bottom left side.

#### CELL_NEIGHBOR_BOTTOM_LEFT_CORNER {#CELL_NEIGHBOR_BOTTOM_LEFT_CORNER}

```gdscript
CELL_NEIGHBOR_BOTTOM_LEFT_CORNER = 7
```

Neighbor in the bottom left corner.

#### CELL_NEIGHBOR_LEFT_SIDE {#CELL_NEIGHBOR_LEFT_SIDE}

```gdscript
CELL_NEIGHBOR_LEFT_SIDE = 8
```

Neighbor on the left side.

#### CELL_NEIGHBOR_LEFT_CORNER {#CELL_NEIGHBOR_LEFT_CORNER}

```gdscript
CELL_NEIGHBOR_LEFT_CORNER = 9
```

Neighbor in the left corner.

#### CELL_NEIGHBOR_TOP_LEFT_SIDE {#CELL_NEIGHBOR_TOP_LEFT_SIDE}

```gdscript
CELL_NEIGHBOR_TOP_LEFT_SIDE = 10
```

Neighbor on the top left side.

#### CELL_NEIGHBOR_TOP_LEFT_CORNER {#CELL_NEIGHBOR_TOP_LEFT_CORNER}

```gdscript
CELL_NEIGHBOR_TOP_LEFT_CORNER = 11
```

Neighbor in the top left corner.

#### CELL_NEIGHBOR_TOP_SIDE {#CELL_NEIGHBOR_TOP_SIDE}

```gdscript
CELL_NEIGHBOR_TOP_SIDE = 12
```

Neighbor on the top side.

#### CELL_NEIGHBOR_TOP_CORNER {#CELL_NEIGHBOR_TOP_CORNER}

```gdscript
CELL_NEIGHBOR_TOP_CORNER = 13
```

Neighbor in the top corner.

#### CELL_NEIGHBOR_TOP_RIGHT_SIDE {#CELL_NEIGHBOR_TOP_RIGHT_SIDE}

```gdscript
CELL_NEIGHBOR_TOP_RIGHT_SIDE = 14
```

Neighbor on the top right side.

#### CELL_NEIGHBOR_TOP_RIGHT_CORNER {#CELL_NEIGHBOR_TOP_RIGHT_CORNER}

```gdscript
CELL_NEIGHBOR_TOP_RIGHT_CORNER = 15
```

Neighbor in the top right corner.

---

### TerrainMode {#enum-TerrainMode}

```gdscript
enum TerrainMode { TERRAIN_MODE_MATCH_CORNERS_AND_SIDES = 0, TERRAIN_MODE_MATCH_CORNERS = 1, TERRAIN_MODE_MATCH_SIDES = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TERRAIN_MODE_MATCH_CORNERS_AND_SIDES {#TERRAIN_MODE_MATCH_CORNERS_AND_SIDES}

```gdscript
TERRAIN_MODE_MATCH_CORNERS_AND_SIDES = 0
```

Requires both corners and side to match with neighboring tiles' terrains.

#### TERRAIN_MODE_MATCH_CORNERS {#TERRAIN_MODE_MATCH_CORNERS}

```gdscript
TERRAIN_MODE_MATCH_CORNERS = 1
```

Requires corners to match with neighboring tiles' terrains.

#### TERRAIN_MODE_MATCH_SIDES {#TERRAIN_MODE_MATCH_SIDES}

```gdscript
TERRAIN_MODE_MATCH_SIDES = 2
```

Requires sides to match with neighboring tiles' terrains.

---
