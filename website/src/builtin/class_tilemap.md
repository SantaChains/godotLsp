# TileMap

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TileMap**

> Node for 2D tile-based maps. Tilemaps use a TileSet which contain a list of tiles which are used to create grid-based maps. A TileMap may have several layers, layouting tiles on top of each other.

<details><summary>官方英文描述（中文精译进行中）</summary>

Node for 2D tile-based maps. Tilemaps use a TileSet which contain a list of tiles which are used to create grid-based maps. A TileMap may have several layers, layouting tiles on top of each other.

For performance reasons, all TileMap updates are batched at the end of a frame. Notably, this means that scene tiles from a TileSetScenesCollectionSource may be initialized after their parent. This is only queued when inside the scene tree.

To force an update earlier on, call update_internals().

Note: For performance and compatibility reasons, the coordinates serialized by TileMap are limited to 16-bit signed integers, i.e. the range for X and Y coordinates is from -32768 to 32767. When saving tile data, tiles outside this range are wrapped.

</details>

## 方法

### _tile_data_runtime_update {#_tile_data_runtime_update}

```gdscript
void _tile_data_runtime_update(layer: int, coords: Vector2i, tile_data: TileData) virtual
```

Called with a TileData object about to be used internally by the TileMap, allowing its modification at runtime.

This method is only called if _use_tile_data_runtime_update() is implemented and returns true for the given tile coords and layer.

Warning: The tile_data object's sub-resources are the same as the one in the TileSet. Modifying them might impact the whole TileSet. Instead, make sure to duplicate those resources.

Note: If the properties of tile_data object should change over time, use notify_runtime_tile_data_update() to notify the TileMap it needs an update.

### _use_tile_data_runtime_update {#_use_tile_data_runtime_update}

```gdscript
bool _use_tile_data_runtime_update(layer: int, coords: Vector2i) virtual
```

Should return true if the tile at coordinates coords on layer layer requires a runtime update.

Warning: Make sure this function only return true when needed. Any tile processed at runtime without a need for it will imply a significant performance penalty.

Note: If the result of this function should changed, use notify_runtime_tile_data_update() to notify the TileMap it needs an update.

### add_layer {#add_layer}

```gdscript
void add_layer(to_position: int)
```

Adds a layer at the given position to_position in the array. If to_position is negative, the position is counted from the end, with -1 adding the layer at the end of the array.

### clear {#clear}

```gdscript
void clear()
```

Clears all cells.

### clear_layer {#clear_layer}

```gdscript
void clear_layer(layer: int)
```

Clears all cells on the given layer.

If layer is negative, the layers are accessed from the last one.

### erase_cell {#erase_cell}

```gdscript
void erase_cell(layer: int, coords: Vector2i)
```

Erases the cell on layer layer at coordinates coords.

If layer is negative, the layers are accessed from the last one.

### fix_invalid_tiles {#fix_invalid_tiles}

```gdscript
void fix_invalid_tiles()
```

Clears cells that do not exist in the tileset.

### force_update {#force_update}

```gdscript
void force_update(layer: int = -1)
```

Deprecated: Use notify_runtime_tile_data_update() and/or update_internals() instead.

Forces the TileMap and the layer layer to update.

### get_cell_alternative_tile {#get_cell_alternative_tile}

```gdscript
int get_cell_alternative_tile(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns the tile alternative ID of the cell on layer layer at coords.

If use_proxies is false, ignores the TileSet's tile proxies, returning the raw alternative identifier. See TileSet.map_tile_proxy().

If layer is negative, the layers are accessed from the last one.

### get_cell_atlas_coords {#get_cell_atlas_coords}

```gdscript
Vector2i get_cell_atlas_coords(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns the tile atlas coordinates ID of the cell on layer layer at coordinates coords. Returns Vector2i(-1, -1) if the cell does not exist.

If use_proxies is false, ignores the TileSet's tile proxies, returning the raw atlas coordinate identifier. See TileSet.map_tile_proxy().

If layer is negative, the layers are accessed from the last one.

### get_cell_source_id {#get_cell_source_id}

```gdscript
int get_cell_source_id(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns the tile source ID of the cell on layer layer at coordinates coords. Returns -1 if the cell does not exist.

If use_proxies is false, ignores the TileSet's tile proxies, returning the raw source identifier. See TileSet.map_tile_proxy().

If layer is negative, the layers are accessed from the last one.

### get_cell_tile_data {#get_cell_tile_data}

```gdscript
TileData get_cell_tile_data(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns the TileData object associated with the given cell, or null if the cell does not exist or is not a TileSetAtlasSource.

If layer is negative, the layers are accessed from the last one.

```gdscript
func get_clicked_tile_power():
    var clicked_cell = tile_map.local_to_map(tile_map.get_local_mouse_position())
    var data = tile_map.get_cell_tile_data(0, clicked_cell)
    if data:
        return data.get_custom_data("power")
    else:
        return 0
```

If use_proxies is false, ignores the TileSet's tile proxies. See TileSet.map_tile_proxy().

### get_collision_visibility_mode {#get_collision_visibility_mode}

```gdscript
TileMap.VisibilityMode get_collision_visibility_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_coords_for_body_rid {#get_coords_for_body_rid}

```gdscript
Vector2i get_coords_for_body_rid(body: RID)
```

Returns the coordinates of the tile for given physics body RID. Such RID can be retrieved from KinematicCollision2D.get_collider_rid(), when colliding with a tile.

### get_layer_for_body_rid {#get_layer_for_body_rid}

```gdscript
int get_layer_for_body_rid(body: RID)
```

Returns the tilemap layer of the tile for given physics body RID. Such RID can be retrieved from KinematicCollision2D.get_collider_rid(), when colliding with a tile.

### get_layer_modulate {#get_layer_modulate}

```gdscript
Color get_layer_modulate(layer: int) const
```

Returns a TileMap layer's modulate.

If layer is negative, the layers are accessed from the last one.

### get_layer_name {#get_layer_name}

```gdscript
String get_layer_name(layer: int) const
```

Returns a TileMap layer's name.

If layer is negative, the layers are accessed from the last one.

### get_layer_navigation_map {#get_layer_navigation_map}

```gdscript
RID get_layer_navigation_map(layer: int) const
```

Returns the RID of the NavigationServer2D navigation map assigned to the specified TileMap layer layer.

By default the TileMap uses the default World2D navigation map for the first TileMap layer. For each additional TileMap layer a new navigation map is created for the additional layer.

In order to make NavigationAgent2D switch between TileMap layer navigation maps use NavigationAgent2D.set_navigation_map() with the navigation map received from get_layer_navigation_map().

If layer is negative, the layers are accessed from the last one.

### get_layer_y_sort_origin {#get_layer_y_sort_origin}

```gdscript
int get_layer_y_sort_origin(layer: int) const
```

Returns a TileMap layer's Y sort origin.

If layer is negative, the layers are accessed from the last one.

### get_layer_z_index {#get_layer_z_index}

```gdscript
int get_layer_z_index(layer: int) const
```

Returns a TileMap layer's Z-index value.

If layer is negative, the layers are accessed from the last one.

### get_layers_count {#get_layers_count}

```gdscript
int get_layers_count() const
```

Returns the number of layers in the TileMap.

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map(layer: int) const
```

Deprecated: Use get_layer_navigation_map() instead.

Returns the RID of the NavigationServer2D navigation map assigned to the specified TileMap layer layer.

### get_navigation_visibility_mode {#get_navigation_visibility_mode}

```gdscript
TileMap.VisibilityMode get_navigation_visibility_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_neighbor_cell {#get_neighbor_cell}

```gdscript
Vector2i get_neighbor_cell(coords: Vector2i, neighbor: TileSet.CellNeighbor) const
```

Returns the neighboring cell to the one at coordinates coords, identified by the neighbor direction. This method takes into account the different layouts a TileMap can take.

### get_pattern {#get_pattern}

```gdscript
TileMapPattern get_pattern(layer: int, coords_array: typedarray::Vector2i)
```

Creates a new TileMapPattern from the given layer and set of cells.

If layer is negative, the layers are accessed from the last one.

### get_rendering_quadrant_size {#get_rendering_quadrant_size}

```gdscript
int get_rendering_quadrant_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_surrounding_cells {#get_surrounding_cells}

```gdscript
typedarray::Vector2i get_surrounding_cells(coords: Vector2i)
```

Returns the list of all neighbourings cells to the one at coords.

### get_tileset {#get_tileset}

```gdscript
TileSet get_tileset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_used_cells {#get_used_cells}

```gdscript
typedarray::Vector2i get_used_cells(layer: int) const
```

Returns a Vector2i array with the positions of all cells containing a tile in the given layer. A cell is considered empty if its source identifier equals -1, its atlas coordinates identifiers is Vector2(-1, -1) and its alternative identifier is -1.

If layer is negative, the layers are accessed from the last one.

### get_used_cells_by_id {#get_used_cells_by_id}

```gdscript
typedarray::Vector2i get_used_cells_by_id(layer: int, source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = -1) const
```

Returns a Vector2i array with the positions of all cells containing a tile in the given layer. Tiles may be filtered according to their source (source_id), their atlas coordinates (atlas_coords) or alternative id (alternative_tile).

If a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default value, this method returns the same result as get_used_cells().

A cell is considered empty if its source identifier equals -1, its atlas coordinates identifiers is Vector2(-1, -1) and its alternative identifier is -1.

If layer is negative, the layers are accessed from the last one.

### get_used_rect {#get_used_rect}

```gdscript
Rect2i get_used_rect() const
```

Returns a rectangle enclosing the used (non-empty) tiles of the map, including all layers.

### is_cell_flipped_h {#is_cell_flipped_h}

```gdscript
bool is_cell_flipped_h(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns true if the cell on layer layer at coordinates coords is flipped horizontally. The result is valid only for atlas sources.

### is_cell_flipped_v {#is_cell_flipped_v}

```gdscript
bool is_cell_flipped_v(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns true if the cell on layer layer at coordinates coords is flipped vertically. The result is valid only for atlas sources.

### is_cell_transposed {#is_cell_transposed}

```gdscript
bool is_cell_transposed(layer: int, coords: Vector2i, use_proxies: bool = false) const
```

Returns true if the cell on layer layer at coordinates coords is transposed. The result is valid only for atlas sources.

### is_collision_animatable {#is_collision_animatable}

```gdscript
bool is_collision_animatable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_layer_enabled {#is_layer_enabled}

```gdscript
bool is_layer_enabled(layer: int) const
```

Returns if a layer is enabled.

If layer is negative, the layers are accessed from the last one.

### is_layer_navigation_enabled {#is_layer_navigation_enabled}

```gdscript
bool is_layer_navigation_enabled(layer: int) const
```

Returns if a layer's built-in navigation regions generation is enabled.

### is_layer_y_sort_enabled {#is_layer_y_sort_enabled}

```gdscript
bool is_layer_y_sort_enabled(layer: int) const
```

Returns if a layer Y-sorts its tiles.

If layer is negative, the layers are accessed from the last one.

### local_to_map {#local_to_map}

```gdscript
Vector2i local_to_map(local_position: Vector2) const
```

Returns the map coordinates of the cell containing the given local_position. If local_position is in global coordinates, consider using Node2D.to_local() before passing it to this method. See also map_to_local().

### map_pattern {#map_pattern}

```gdscript
Vector2i map_pattern(position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: TileMapPattern)
```

Returns for the given coordinate coords_in_pattern in a TileMapPattern the corresponding cell coordinates if the pattern was pasted at the position_in_tilemap coordinates (see set_pattern()). This mapping is required as in half-offset tile shapes, the mapping might not work by calculating position_in_tile_map + coords_in_pattern.

### map_to_local {#map_to_local}

```gdscript
Vector2 map_to_local(map_position: Vector2i) const
```

Returns the centered position of a cell in the TileMap's local coordinate space. To convert the returned value into global coordinates, use Node2D.to_global(). See also local_to_map().

Note: This may not correspond to the visual position of the tile, i.e. it ignores the TileData.texture_origin property of individual tiles.

### move_layer {#move_layer}

```gdscript
void move_layer(layer: int, to_position: int)
```

Moves the layer at index layer to the given position to_position in the array.

### notify_runtime_tile_data_update {#notify_runtime_tile_data_update}

```gdscript
void notify_runtime_tile_data_update(layer: int = -1)
```

Notifies the TileMap node that calls to _use_tile_data_runtime_update() or _tile_data_runtime_update() will lead to different results. This will thus trigger a TileMap update.

If layer is provided, only notifies changes for the given layer. Providing the layer argument (when applicable) is usually preferred for performance reasons.

Warning: Updating the TileMap is computationally expensive and may impact performance. Try to limit the number of calls to this function to avoid unnecessary update.

Note: This does not trigger a direct update of the TileMap, the update will be done at the end of the frame as usual (unless you call update_internals()).

### remove_layer {#remove_layer}

```gdscript
void remove_layer(layer: int)
```

Removes the layer at index layer.

### set_cell {#set_cell}

```gdscript
void set_cell(layer: int, coords: Vector2i, source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = 0)
```

Sets the tile identifiers for the cell on layer layer at coordinates coords. Each tile of the TileSet is identified using three parts:

- The source identifier source_id identifies a TileSetSource identifier. See TileSet.set_source_id(),
- The atlas coordinates identifier atlas_coords identifies a tile coordinates in the atlas (if the source is a TileSetAtlasSource). For TileSetScenesCollectionSource it should always be Vector2i(0, 0)),
- The alternative tile identifier alternative_tile identifies a tile alternative in the atlas (if the source is a TileSetAtlasSource), and the scene for a TileSetScenesCollectionSource.

If source_id is set to -1, atlas_coords to Vector2i(-1, -1) or alternative_tile to -1, the cell will be erased. An erased cell gets all its identifiers automatically set to their respective invalid values, namely -1, Vector2i(-1, -1) and -1.

If layer is negative, the layers are accessed from the last one.

### set_cells_terrain_connect {#set_cells_terrain_connect}

```gdscript
void set_cells_terrain_connect(layer: int, cells: typedarray::Vector2i, terrain_set: int, terrain: int, ignore_empty_terrains: bool = true)
```

Update all the cells in the cells coordinates array so that they use the given terrain for the given terrain_set. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.

If ignore_empty_terrains is true, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.

If layer is negative, the layers are accessed from the last one.

Note: To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results.

### set_cells_terrain_path {#set_cells_terrain_path}

```gdscript
void set_cells_terrain_path(layer: int, path: typedarray::Vector2i, terrain_set: int, terrain: int, ignore_empty_terrains: bool = true)
```

Update all the cells in the path coordinates array so that they use the given terrain for the given terrain_set. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.

If ignore_empty_terrains is true, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.

If layer is negative, the layers are accessed from the last one.

Note: To work correctly, this method requires the TileMap's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results.

### set_collision_animatable {#set_collision_animatable}

```gdscript
void set_collision_animatable(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_visibility_mode {#set_collision_visibility_mode}

```gdscript
void set_collision_visibility_mode(collision_visibility_mode: TileMap.VisibilityMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layer_enabled {#set_layer_enabled}

```gdscript
void set_layer_enabled(layer: int, enabled: bool)
```

Enables or disables the layer layer. A disabled layer is not processed at all (no rendering, no physics, etc.).

If layer is negative, the layers are accessed from the last one.

### set_layer_modulate {#set_layer_modulate}

```gdscript
void set_layer_modulate(layer: int, modulate: Color)
```

Sets a layer's color. It will be multiplied by tile's color and TileMap's modulate.

If layer is negative, the layers are accessed from the last one.

### set_layer_name {#set_layer_name}

```gdscript
void set_layer_name(layer: int, name: String)
```

Sets a layer's name. This is mostly useful in the editor.

If layer is negative, the layers are accessed from the last one.

### set_layer_navigation_enabled {#set_layer_navigation_enabled}

```gdscript
void set_layer_navigation_enabled(layer: int, enabled: bool)
```

Enables or disables a layer's built-in navigation regions generation. Disable this if you need to bake navigation regions from a TileMap using a NavigationRegion2D node.

### set_layer_navigation_map {#set_layer_navigation_map}

```gdscript
void set_layer_navigation_map(layer: int, map: RID)
```

Assigns map as a NavigationServer2D navigation map for the specified TileMap layer layer.

By default the TileMap uses the default World2D navigation map for the first TileMap layer. For each additional TileMap layer a new navigation map is created for the additional layer.

In order to make NavigationAgent2D switch between TileMap layer navigation maps use NavigationAgent2D.set_navigation_map() with the navigation map received from get_layer_navigation_map().

If layer is negative, the layers are accessed from the last one.

### set_layer_y_sort_enabled {#set_layer_y_sort_enabled}

```gdscript
void set_layer_y_sort_enabled(layer: int, y_sort_enabled: bool)
```

Enables or disables a layer's Y-sorting. If a layer is Y-sorted, the layer will behave as a CanvasItem node where each of its tile gets Y-sorted.

Y-sorted layers should usually be on different Z-index values than not Y-sorted layers, otherwise, each of those layer will be Y-sorted as whole with the Y-sorted one. This is usually an undesired behavior.

If layer is negative, the layers are accessed from the last one.

### set_layer_y_sort_origin {#set_layer_y_sort_origin}

```gdscript
void set_layer_y_sort_origin(layer: int, y_sort_origin: int)
```

Sets a layer's Y-sort origin value. This Y-sort origin value is added to each tile's Y-sort origin value.

This allows, for example, to fake a different height level on each layer. This can be useful for top-down view games.

If layer is negative, the layers are accessed from the last one.

### set_layer_z_index {#set_layer_z_index}

```gdscript
void set_layer_z_index(layer: int, z_index: int)
```

Sets a layers Z-index value. This Z-index is added to each tile's Z-index value.

If layer is negative, the layers are accessed from the last one.

### set_navigation_map {#set_navigation_map}

```gdscript
void set_navigation_map(layer: int, map: RID)
```

Deprecated: Use set_layer_navigation_map() instead.

Assigns map as a NavigationServer2D navigation map for the specified TileMap layer layer.

### set_navigation_visibility_mode {#set_navigation_visibility_mode}

```gdscript
void set_navigation_visibility_mode(navigation_visibility_mode: TileMap.VisibilityMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pattern {#set_pattern}

```gdscript
void set_pattern(layer: int, position: Vector2i, pattern: TileMapPattern)
```

Paste the given TileMapPattern at the given position and layer in the tile map.

If layer is negative, the layers are accessed from the last one.

### set_rendering_quadrant_size {#set_rendering_quadrant_size}

```gdscript
void set_rendering_quadrant_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tileset {#set_tileset}

```gdscript
void set_tileset(tileset: TileSet)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update_internals {#update_internals}

```gdscript
void update_internals()
```

Triggers a direct update of the TileMap. Usually, calling this function is not needed, as TileMap node updates automatically when one of its properties or cells is modified.

However, for performance reasons, those updates are batched and delayed to the end of the frame. Calling this function will force the TileMap to update right away instead.

Warning: Updating the TileMap is computationally expensive and may impact performance. Try to limit the number of updates and how many tiles they impact.

## 属性

### tile_set {#tile_set}

```gdscript
tile_set: TileSet
```

- void set_tileset(value: TileSet)
- TileSet get_tileset()

The TileSet used by this TileMap. The textures, collisions, and additional behavior of all available tiles are stored here.

### rendering_quadrant_size {#rendering_quadrant_size}

```gdscript
rendering_quadrant_size: int
```

- void set_rendering_quadrant_size(value: int)
- int get_rendering_quadrant_size()

The TileMap's quadrant size. A quadrant is a group of tiles to be drawn together on a single canvas item, for optimization purposes. rendering_quadrant_size defines the length of a square's side, in the map's coordinate system, that forms the quadrant. Thus, the default quadrant size groups together 16 * 16 = 256 tiles.

The quadrant size does not apply on Y-sorted layers, as tiles are grouped by Y position instead in that case.

Note: As quadrants are created according to the map's coordinate system, the quadrant's "square shape" might not look like square in the TileMap's local coordinate system.

### collision_animatable {#collision_animatable}

```gdscript
collision_animatable: bool
```

- void set_collision_animatable(value: bool)
- bool is_collision_animatable()

If enabled, the TileMap will see its collisions synced to the physics tick and change its collision type from static to kinematic. This is required to create TileMap-based moving platform.

Note: Enabling collision_animatable may have a small performance impact, only do it if the TileMap is moving and has colliding tiles.

### collision_visibility_mode {#collision_visibility_mode}

```gdscript
collision_visibility_mode: int
```

- void set_collision_visibility_mode(value: VisibilityMode)
- VisibilityMode get_collision_visibility_mode()

Show or hide the TileMap's collision shapes. If set to VISIBILITY_MODE_DEFAULT, this depends on the show collision debug settings.

### navigation_visibility_mode {#navigation_visibility_mode}

```gdscript
navigation_visibility_mode: int
```

- void set_navigation_visibility_mode(value: VisibilityMode)
- VisibilityMode get_navigation_visibility_mode()

Show or hide the TileMap's navigation meshes. If set to VISIBILITY_MODE_DEFAULT, this depends on the show navigation debug settings.

## 信号

### changed {#changed}

```gdscript
signal changed()
```

Emitted when the TileSet of this TileMap changes.

## 枚举

### VisibilityMode {#enum-VisibilityMode}

```gdscript
enum VisibilityMode { VISIBILITY_MODE_DEFAULT = 0, VISIBILITY_MODE_FORCE_HIDE = 2, VISIBILITY_MODE_FORCE_SHOW = 1 }
```

VisibilityMode VISIBILITY_MODE_DEFAULT = 0

Use the debug settings to determine visibility.

VisibilityMode VISIBILITY_MODE_FORCE_HIDE = 2

Always hide.

VisibilityMode VISIBILITY_MODE_FORCE_SHOW = 1

Always show.

#### VISIBILITY_MODE_DEFAULT {#VISIBILITY_MODE_DEFAULT}

```gdscript
VISIBILITY_MODE_DEFAULT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VISIBILITY_MODE_FORCE_HIDE {#VISIBILITY_MODE_FORCE_HIDE}

```gdscript
VISIBILITY_MODE_FORCE_HIDE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VISIBILITY_MODE_FORCE_SHOW {#VISIBILITY_MODE_FORCE_SHOW}

```gdscript
VISIBILITY_MODE_FORCE_SHOW = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
