# TileMapLayer

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TileMapLayer**

> Node for 2D tile-based maps. A TileMapLayer uses a TileSet which contain a list of tiles which are used to create grid-based maps. Unlike the TileMap node, which is deprecated, TileMapLayer has only one layer of tiles. You can use several TileMapLayer to achieve the same result as a TileMap node.

<details><summary>官方英文描述（中文精译进行中）</summary>

Node for 2D tile-based maps. A TileMapLayer uses a TileSet which contain a list of tiles which are used to create grid-based maps. Unlike the TileMap node, which is deprecated, TileMapLayer has only one layer of tiles. You can use several TileMapLayer to achieve the same result as a TileMap node.

For performance reasons, all TileMap updates are batched at the end of a frame. Notably, this means that scene tiles from a TileSetScenesCollectionSource are initialized after their parent. This is only queued when inside the scene tree.

To force an update earlier on, call update_internals().

Note: For performance and compatibility reasons, the coordinates serialized by TileMapLayer are limited to 16-bit signed integers, i.e. the range for X and Y coordinates is from -32768 to 32767. When saving tile data, tiles outside this range are wrapped.

</details>

## 方法

### _tile_data_runtime_update {#_tile_data_runtime_update}

```gdscript
void _tile_data_runtime_update(coords: Vector2i, tile_data: TileData) virtual
```

Called with a TileData object about to be used internally by the TileMapLayer, allowing its modification at runtime.

This method is only called if _use_tile_data_runtime_update() is implemented and returns true for the given tile coords.

Warning: The tile_data object's sub-resources are the same as the one in the TileSet. Modifying them might impact the whole TileSet. Instead, make sure to duplicate those resources.

Note: If the properties of tile_data object should change over time, use notify_runtime_tile_data_update() to notify the TileMapLayer it needs an update.

### _update_cells {#_update_cells}

```gdscript
void _update_cells(coords: typedarray::Vector2i, forced_cleanup: bool) virtual
```

Called when this TileMapLayer's cells need an internal update. This update may be caused from individual cells being modified or by a change in the tile_set (causing all cells to be queued for an update). The first call to this function is always for initializing all the TileMapLayer's cells. coords contains the coordinates of all modified cells, roughly in the order they were modified. forced_cleanup is true when the TileMapLayer's internals should be fully cleaned up. This is the case when:

- The layer is disabled;
- The layer is not visible;
- tile_set is set to null;
- The node is removed from the tree;
- The node is freed.

Note that any internal update happening while one of these conditions is verified is considered to be a "cleanup". See also update_internals().

Warning: Implementing this method may degrade the TileMapLayer's performance.

### _use_tile_data_runtime_update {#_use_tile_data_runtime_update}

```gdscript
bool _use_tile_data_runtime_update(coords: Vector2i) virtual
```

Should return true if the tile at coordinates coords requires a runtime update.

Warning: Make sure this function only returns true when needed. Any tile processed at runtime without a need for it will imply a significant performance penalty.

Note: If the result of this function should change, use notify_runtime_tile_data_update() to notify the TileMapLayer it needs an update.

### clear {#clear}

```gdscript
void clear()
```

Clears all cells.

### erase_cell {#erase_cell}

```gdscript
void erase_cell(coords: Vector2i)
```

Erases the cell at coordinates coords.

### fix_invalid_tiles {#fix_invalid_tiles}

```gdscript
void fix_invalid_tiles()
```

Clears cells containing tiles that do not exist in the tile_set.

### get_cell_alternative_tile {#get_cell_alternative_tile}

```gdscript
int get_cell_alternative_tile(coords: Vector2i) const
```

Returns the tile alternative ID of the cell at coordinates coords.

### get_cell_atlas_coords {#get_cell_atlas_coords}

```gdscript
Vector2i get_cell_atlas_coords(coords: Vector2i) const
```

Returns the tile atlas coordinates ID of the cell at coordinates coords. Returns Vector2i(-1, -1) if the cell does not exist.

### get_cell_source_id {#get_cell_source_id}

```gdscript
int get_cell_source_id(coords: Vector2i) const
```

Returns the tile source ID of the cell at coordinates coords. Returns -1 if the cell does not exist.

### get_cell_tile_data {#get_cell_tile_data}

```gdscript
TileData get_cell_tile_data(coords: Vector2i) const
```

Returns the TileData object associated with the given cell, or null if the cell does not exist or is not a TileSetAtlasSource.

```gdscript
func get_clicked_tile_power():
    var clicked_cell = tile_map_layer.local_to_map(tile_map_layer.get_local_mouse_position())
    var data = tile_map_layer.get_cell_tile_data(clicked_cell)
    if data:
        return data.get_custom_data("power")
    else:
        return 0
```

### get_collision_visibility_mode {#get_collision_visibility_mode}

```gdscript
TileMapLayer.DebugVisibilityMode get_collision_visibility_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_coords_for_body_rid {#get_coords_for_body_rid}

```gdscript
Vector2i get_coords_for_body_rid(body: RID) const
```

Returns the coordinates of the physics quadrant (see physics_quadrant_size) for given physics body RID. Such an RID can be retrieved from KinematicCollision2D.get_collider_rid(), when colliding with a tile.

Note: Higher values of physics_quadrant_size will make this function less precise. To get the exact cell coordinates, you need to set physics_quadrant_size to 1, which disables physics chunking.

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

Returns the RID of the NavigationServer2D navigation used by this TileMapLayer.

By default this returns the default World2D navigation map, unless a custom map was provided using set_navigation_map().

### get_navigation_visibility_mode {#get_navigation_visibility_mode}

```gdscript
TileMapLayer.DebugVisibilityMode get_navigation_visibility_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_neighbor_cell {#get_neighbor_cell}

```gdscript
Vector2i get_neighbor_cell(coords: Vector2i, neighbor: TileSet.CellNeighbor) const
```

Returns the neighboring cell to the one at coordinates coords, identified by the neighbor direction. This method takes into account the different layouts a TileMap can take.

### get_pattern {#get_pattern}

```gdscript
TileMapPattern get_pattern(coords_array: typedarray::Vector2i)
```

Creates and returns a new TileMapPattern from the given array of cells. See also set_pattern().

### get_physics_quadrant_size {#get_physics_quadrant_size}

```gdscript
int get_physics_quadrant_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rendering_quadrant_size {#get_rendering_quadrant_size}

```gdscript
int get_rendering_quadrant_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_surrounding_cells {#get_surrounding_cells}

```gdscript
typedarray::Vector2i get_surrounding_cells(coords: Vector2i)
```

Returns the list of all neighboring cells to the one at coords. Any neighboring cell is one that is touching edges, so for a square cell 4 cells would be returned, for a hexagon 6 cells are returned.

### get_tile_map_data_as_array {#get_tile_map_data_as_array}

```gdscript
PackedByteArray get_tile_map_data_as_array() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tile_set {#get_tile_set}

```gdscript
TileSet get_tile_set() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_used_cells {#get_used_cells}

```gdscript
typedarray::Vector2i get_used_cells() const
```

Returns a Vector2i array with the positions of all cells containing a tile. A cell is considered empty if its source identifier equals -1, its atlas coordinate identifier is Vector2(-1, -1) and its alternative identifier is -1.

### get_used_cells_by_id {#get_used_cells_by_id}

```gdscript
typedarray::Vector2i get_used_cells_by_id(source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = -1) const
```

Returns a Vector2i array with the positions of all cells containing a tile. Tiles may be filtered according to their source (source_id), their atlas coordinates (atlas_coords), or alternative id (alternative_tile).

If a parameter has its value set to the default one, this parameter is not used to filter a cell. Thus, if all parameters have their respective default values, this method returns the same result as get_used_cells().

A cell is considered empty if its source identifier equals -1, its atlas coordinate identifier is Vector2(-1, -1) and its alternative identifier is -1.

### get_used_rect {#get_used_rect}

```gdscript
Rect2i get_used_rect() const
```

Returns a rectangle enclosing the used (non-empty) tiles of the map.

### get_y_sort_origin {#get_y_sort_origin}

```gdscript
int get_y_sort_origin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_body_rid {#has_body_rid}

```gdscript
bool has_body_rid(body: RID) const
```

Returns whether the provided body RID belongs to one of this TileMapLayer's cells.

### is_cell_flipped_h {#is_cell_flipped_h}

```gdscript
bool is_cell_flipped_h(coords: Vector2i) const
```

Returns true if the cell at coordinates coords is flipped horizontally. The result is valid only for atlas sources.

### is_cell_flipped_v {#is_cell_flipped_v}

```gdscript
bool is_cell_flipped_v(coords: Vector2i) const
```

Returns true if the cell at coordinates coords is flipped vertically. The result is valid only for atlas sources.

### is_cell_transposed {#is_cell_transposed}

```gdscript
bool is_cell_transposed(coords: Vector2i) const
```

Returns true if the cell at coordinates coords is transposed. The result is valid only for atlas sources.

### is_collision_enabled {#is_collision_enabled}

```gdscript
bool is_collision_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_navigation_enabled {#is_navigation_enabled}

```gdscript
bool is_navigation_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_occlusion_enabled {#is_occlusion_enabled}

```gdscript
bool is_occlusion_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_kinematic_bodies {#is_using_kinematic_bodies}

```gdscript
bool is_using_kinematic_bodies() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_x_draw_order_reversed {#is_x_draw_order_reversed}

```gdscript
bool is_x_draw_order_reversed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### local_to_map {#local_to_map}

```gdscript
Vector2i local_to_map(local_position: Vector2) const
```

Returns the map coordinates of the cell containing the given local_position. If local_position is in global coordinates, consider using Node2D.to_local() before passing it to this method. See also map_to_local().

### map_pattern {#map_pattern}

```gdscript
Vector2i map_pattern(position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: TileMapPattern)
```

Returns for the given coordinates coords_in_pattern in a TileMapPattern the corresponding cell coordinates if the pattern was pasted at the position_in_tilemap coordinates (see set_pattern()). This mapping is required as in half-offset tile shapes, the mapping might not work by calculating position_in_tile_map + coords_in_pattern.

### map_to_local {#map_to_local}

```gdscript
Vector2 map_to_local(map_position: Vector2i) const
```

Returns the centered position of a cell in the TileMapLayer's local coordinate space. To convert the returned value into global coordinates, use Node2D.to_global(). See also local_to_map().

Note: This may not correspond to the visual position of the tile, i.e. it ignores the TileData.texture_origin property of individual tiles.

### notify_runtime_tile_data_update {#notify_runtime_tile_data_update}

```gdscript
void notify_runtime_tile_data_update()
```

Notifies the TileMapLayer node that calls to _use_tile_data_runtime_update() or _tile_data_runtime_update() will lead to different results. This will thus trigger a TileMapLayer update.

Warning: Updating the TileMapLayer is computationally expensive and may impact performance. Try to limit the number of calls to this function to avoid unnecessary update.

Note: This does not trigger a direct update of the TileMapLayer, the update will be done at the end of the frame as usual (unless you call update_internals()).

### set_cell {#set_cell}

```gdscript
void set_cell(coords: Vector2i, source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = 0)
```

Sets the tile identifiers for the cell at coordinates coords. Each tile of the TileSet is identified using three parts:

- The source identifier source_id identifies a TileSetSource identifier. See TileSet.set_source_id(),
- The atlas coordinate identifier atlas_coords identifies a tile coordinates in the atlas (if the source is a TileSetAtlasSource). For TileSetScenesCollectionSource it should always be Vector2i(0, 0),
- The alternative tile identifier alternative_tile identifies a tile alternative in the atlas (if the source is a TileSetAtlasSource), and the scene for a TileSetScenesCollectionSource.

If source_id is set to -1, atlas_coords to Vector2i(-1, -1), or alternative_tile to -1, the cell will be erased. An erased cell gets all its identifiers automatically set to their respective invalid values, namely -1, Vector2i(-1, -1) and -1.

### set_cells_terrain_connect {#set_cells_terrain_connect}

```gdscript
void set_cells_terrain_connect(cells: typedarray::Vector2i, terrain_set: int, terrain: int, ignore_empty_terrains: bool = true)
```

Update all the cells in the cells coordinates array so that they use the given terrain for the given terrain_set. If an updated cell has the same terrain as one of its neighboring cells, this function tries to join the two. This function might update neighboring tiles if needed to create correct terrain transitions.

If ignore_empty_terrains is true, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.

Note: To work correctly, this method requires the TileMapLayer's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results.

### set_cells_terrain_path {#set_cells_terrain_path}

```gdscript
void set_cells_terrain_path(path: typedarray::Vector2i, terrain_set: int, terrain: int, ignore_empty_terrains: bool = true)
```

Update all the cells in the path coordinates array so that they use the given terrain for the given terrain_set. The function will also connect two successive cell in the path with the same terrain. This function might update neighboring tiles if needed to create correct terrain transitions.

If ignore_empty_terrains is true, empty terrains will be ignored when trying to find the best fitting tile for the given terrain constraints.

Note: To work correctly, this method requires the TileMapLayer's TileSet to have terrains set up with all required terrain combinations. Otherwise, it may produce unexpected results.

### set_collision_enabled {#set_collision_enabled}

```gdscript
void set_collision_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_visibility_mode {#set_collision_visibility_mode}

```gdscript
void set_collision_visibility_mode(visibility_mode: TileMapLayer.DebugVisibilityMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_enabled {#set_navigation_enabled}

```gdscript
void set_navigation_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_map {#set_navigation_map}

```gdscript
void set_navigation_map(map: RID)
```

Sets a custom map as a NavigationServer2D navigation map. If not set, uses the default World2D navigation map instead.

### set_navigation_visibility_mode {#set_navigation_visibility_mode}

```gdscript
void set_navigation_visibility_mode(show_navigation: TileMapLayer.DebugVisibilityMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_occlusion_enabled {#set_occlusion_enabled}

```gdscript
void set_occlusion_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pattern {#set_pattern}

```gdscript
void set_pattern(position: Vector2i, pattern: TileMapPattern)
```

Pastes the TileMapPattern at the given position in the tile map. See also get_pattern().

### set_physics_quadrant_size {#set_physics_quadrant_size}

```gdscript
void set_physics_quadrant_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rendering_quadrant_size {#set_rendering_quadrant_size}

```gdscript
void set_rendering_quadrant_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_map_data_from_array {#set_tile_map_data_from_array}

```gdscript
void set_tile_map_data_from_array(tile_map_layer_data: PackedByteArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_set {#set_tile_set}

```gdscript
void set_tile_set(tile_set: TileSet)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_kinematic_bodies {#set_use_kinematic_bodies}

```gdscript
void set_use_kinematic_bodies(use_kinematic_bodies: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_x_draw_order_reversed {#set_x_draw_order_reversed}

```gdscript
void set_x_draw_order_reversed(x_draw_order_reversed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_y_sort_origin {#set_y_sort_origin}

```gdscript
void set_y_sort_origin(y_sort_origin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update_internals {#update_internals}

```gdscript
void update_internals()
```

Triggers a direct update of the TileMapLayer. Usually, calling this function is not needed, as TileMapLayer node updates automatically when one of its properties or cells is modified.

However, for performance reasons, those updates are batched and delayed to the end of the frame. Calling this function will force the TileMapLayer to update right away instead.

Warning: Updating the TileMapLayer is computationally expensive and may impact performance. Try to limit the number of updates and how many tiles they impact.

## 属性

### tile_map_data {#tile_map_data}

```gdscript
tile_map_data: PackedByteArray
```

- void set_tile_map_data_from_array(value: PackedByteArray)
- PackedByteArray get_tile_map_data_as_array()

The raw tile map data as a byte array.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

If false, disables this TileMapLayer completely (rendering, collision, navigation, scene tiles, etc.)

### tile_set {#tile_set}

```gdscript
tile_set: TileSet
```

- void set_tile_set(value: TileSet)
- TileSet get_tile_set()

The TileSet used by this layer. The textures, collisions, and additional behavior of all available tiles are stored here.

### occlusion_enabled {#occlusion_enabled}

```gdscript
occlusion_enabled: bool
```

- void set_occlusion_enabled(value: bool)
- bool is_occlusion_enabled()

Enable or disable light occlusion.

### y_sort_origin {#y_sort_origin}

```gdscript
y_sort_origin: int
```

- void set_y_sort_origin(value: int)
- int get_y_sort_origin()

This Y-sort origin value is added to each tile's Y-sort origin value. This allows, for example, to fake a different height level. This can be useful for top-down view games.

### x_draw_order_reversed {#x_draw_order_reversed}

```gdscript
x_draw_order_reversed: bool
```

- void set_x_draw_order_reversed(value: bool)
- bool is_x_draw_order_reversed()

If CanvasItem.y_sort_enabled is enabled, setting this to true will reverse the order the tiles are drawn on the X-axis.

### rendering_quadrant_size {#rendering_quadrant_size}

```gdscript
rendering_quadrant_size: int
```

- void set_rendering_quadrant_size(value: int)
- int get_rendering_quadrant_size()

The TileMapLayer's rendering quadrant size. A quadrant is a group of tiles to be drawn together on a single canvas item, for optimization purposes. rendering_quadrant_size defines the length of a square's side, in the map's coordinate system, that forms the quadrant. Thus, the default quadrant size groups together 16 * 16 = 256 tiles.

The quadrant size does not apply on a Y-sorted TileMapLayer, as tiles are grouped by Y position instead in that case.

Note: As quadrants are created according to the map's coordinate system, the quadrant's "square shape" might not look like square in the TileMapLayer's local coordinate system.

### collision_enabled {#collision_enabled}

```gdscript
collision_enabled: bool
```

- void set_collision_enabled(value: bool)
- bool is_collision_enabled()

Enable or disable collisions.

### use_kinematic_bodies {#use_kinematic_bodies}

```gdscript
use_kinematic_bodies: bool
```

- void set_use_kinematic_bodies(value: bool)
- bool is_using_kinematic_bodies()

If true, this TileMapLayer collision shapes will be instantiated as kinematic bodies. This can be needed for moving TileMapLayer nodes (i.e. moving platforms).

### collision_visibility_mode {#collision_visibility_mode}

```gdscript
collision_visibility_mode: int
```

- void set_collision_visibility_mode(value: DebugVisibilityMode)
- DebugVisibilityMode get_collision_visibility_mode()

Show or hide the TileMapLayer's collision shapes. If set to DEBUG_VISIBILITY_MODE_DEFAULT, this depends on the show collision debug settings.

### physics_quadrant_size {#physics_quadrant_size}

```gdscript
physics_quadrant_size: int
```

- void set_physics_quadrant_size(value: int)
- int get_physics_quadrant_size()

The TileMapLayer's physics quadrant size. Within a physics quadrant, cells with similar physics properties are grouped together and their collision shapes get merged. physics_quadrant_size defines the length of a square's side, in the map's coordinate system, that forms the quadrant. Thus, the default quadrant size groups together 16 * 16 = 256 tiles.

Note: As quadrants are created according to the map's coordinate system, the quadrant's "square shape" might not look like square in the TileMapLayer's local coordinate system.

Note: This impacts the value returned by get_coords_for_body_rid(). Higher values will make that function less precise. To get the exact cell coordinates, you need to set physics_quadrant_size to 1, which disables physics chunking.

### navigation_enabled {#navigation_enabled}

```gdscript
navigation_enabled: bool
```

- void set_navigation_enabled(value: bool)
- bool is_navigation_enabled()

If true, navigation regions are enabled.

### navigation_visibility_mode {#navigation_visibility_mode}

```gdscript
navigation_visibility_mode: int
```

- void set_navigation_visibility_mode(value: DebugVisibilityMode)
- DebugVisibilityMode get_navigation_visibility_mode()

Show or hide the TileMapLayer's navigation meshes. If set to DEBUG_VISIBILITY_MODE_DEFAULT, this depends on the show navigation debug settings.

## 信号

### changed {#changed}

```gdscript
signal changed()
```

Emitted when this TileMapLayer's properties changes. This includes modified cells, properties, or changes made to its assigned TileSet.

Note: This signal may be emitted very often when batch-modifying a TileMapLayer. Avoid executing complex processing in a connected function, and consider delaying it to the end of the frame instead (i.e. calling Object.call_deferred()).

## 枚举

### DebugVisibilityMode {#enum-DebugVisibilityMode}

```gdscript
enum DebugVisibilityMode { DEBUG_VISIBILITY_MODE_DEFAULT = 0, DEBUG_VISIBILITY_MODE_FORCE_HIDE = 2, DEBUG_VISIBILITY_MODE_FORCE_SHOW = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_VISIBILITY_MODE_DEFAULT {#DEBUG_VISIBILITY_MODE_DEFAULT}

```gdscript
DEBUG_VISIBILITY_MODE_DEFAULT = 0
```

Hide the collisions or navigation debug shapes in the editor, and use the debug settings to determine their visibility in game (i.e. SceneTree.debug_collisions_hint or SceneTree.debug_navigation_hint).

#### DEBUG_VISIBILITY_MODE_FORCE_HIDE {#DEBUG_VISIBILITY_MODE_FORCE_HIDE}

```gdscript
DEBUG_VISIBILITY_MODE_FORCE_HIDE = 2
```

Always hide the collisions or navigation debug shapes.

#### DEBUG_VISIBILITY_MODE_FORCE_SHOW {#DEBUG_VISIBILITY_MODE_FORCE_SHOW}

```gdscript
DEBUG_VISIBILITY_MODE_FORCE_SHOW = 1
```

Always show the collisions or navigation debug shapes.

---
