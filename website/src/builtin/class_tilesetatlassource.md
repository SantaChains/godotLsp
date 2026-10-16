# TileSetAtlasSource

**继承**：[TileSetSource](class_tilesetsource.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TileSetAtlasSource**

> An atlas is a grid of tiles laid out on a texture. Each tile in the grid must be exposed using create_tile(). Those tiles are then indexed using their coordinates in the grid.

<details><summary>官方英文描述</summary>

An atlas is a grid of tiles laid out on a texture. Each tile in the grid must be exposed using create_tile(). Those tiles are then indexed using their coordinates in the grid.

Each tile can also have a size in the grid coordinates, making it more or less cells in the atlas.

Alternatives version of a tile can be created using create_alternative_tile(), which are then indexed using an alternative ID. The main tile (the one in the grid), is accessed with an alternative ID equal to 0.

Each tile alternate has a set of properties that is defined by the source's TileSet layers. Those properties are stored in a TileData object that can be accessed and modified using get_tile_data().

As TileData properties are stored directly in the TileSetAtlasSource resource, their properties might also be set using TileSetAtlasSource.set("://").

</details>

## 方法

### clear_tiles_outside_texture {#clear_tiles_outside_texture}

```gdscript
void clear_tiles_outside_texture()
```

Removes all tiles that don't fit the available texture area. This method iterates over all the source's tiles, so it's advised to use has_tiles_outside_texture() beforehand.

### create_alternative_tile {#create_alternative_tile}

```gdscript
int create_alternative_tile(atlas_coords: Vector2i, alternative_id_override: int = -1)
```

Creates an alternative tile for the tile at coordinates atlas_coords. If alternative_id_override is -1, give it an automatically generated unique ID, or assigns it the given ID otherwise.

Returns the new alternative identifier, or -1 if the alternative could not be created with a provided alternative_id_override.

### create_tile {#create_tile}

```gdscript
void create_tile(atlas_coords: Vector2i, size: Vector2i = Vector2i(1, 1))
```

Creates a new tile at coordinates atlas_coords with the given size.

### get_atlas_grid_size {#get_atlas_grid_size}

```gdscript
Vector2i get_atlas_grid_size() const
```

Returns the atlas grid size, which depends on how many tiles can fit in the texture. It thus depends on the texture's size, the atlas margins, and the tiles' texture_region_size.

### get_margins {#get_margins}

```gdscript
Vector2i get_margins() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_next_alternative_tile_id {#get_next_alternative_tile_id}

```gdscript
int get_next_alternative_tile_id(atlas_coords: Vector2i) const
```

Returns the alternative ID a following call to create_alternative_tile() would return.

### get_runtime_texture {#get_runtime_texture}

```gdscript
Texture2D get_runtime_texture() const
```

If use_texture_padding is false, returns texture. Otherwise, returns an internal ImageTexture created that includes the padding.

### get_runtime_tile_texture_region {#get_runtime_tile_texture_region}

```gdscript
Rect2i get_runtime_tile_texture_region(atlas_coords: Vector2i, frame: int) const
```

Returns the region of the tile at coordinates atlas_coords for the given frame inside the texture returned by get_runtime_texture().

Note: If use_texture_padding is false, returns the same as get_tile_texture_region().

### get_separation {#get_separation}

```gdscript
Vector2i get_separation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_region_size {#get_texture_region_size}

```gdscript
Vector2i get_texture_region_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tile_animation_columns {#get_tile_animation_columns}

```gdscript
int get_tile_animation_columns(atlas_coords: Vector2i) const
```

Returns how many columns the tile at atlas_coords has in its animation layout.

### get_tile_animation_frame_duration {#get_tile_animation_frame_duration}

```gdscript
float get_tile_animation_frame_duration(atlas_coords: Vector2i, frame_index: int) const
```

Returns the animation frame duration of frame frame_index for the tile at coordinates atlas_coords.

### get_tile_animation_frames_count {#get_tile_animation_frames_count}

```gdscript
int get_tile_animation_frames_count(atlas_coords: Vector2i) const
```

Returns how many animation frames has the tile at coordinates atlas_coords.

### get_tile_animation_mode {#get_tile_animation_mode}

```gdscript
TileSetAtlasSource.TileAnimationMode get_tile_animation_mode(atlas_coords: Vector2i) const
```

Returns the tile animation mode of the tile at atlas_coords. See also set_tile_animation_mode().

### get_tile_animation_separation {#get_tile_animation_separation}

```gdscript
Vector2i get_tile_animation_separation(atlas_coords: Vector2i) const
```

Returns the separation (as in the atlas grid) between each frame of an animated tile at coordinates atlas_coords.

### get_tile_animation_speed {#get_tile_animation_speed}

```gdscript
float get_tile_animation_speed(atlas_coords: Vector2i) const
```

Returns the animation speed of the tile at coordinates atlas_coords.

### get_tile_animation_total_duration {#get_tile_animation_total_duration}

```gdscript
float get_tile_animation_total_duration(atlas_coords: Vector2i) const
```

Returns the sum of the sum of the frame durations of the tile at coordinates atlas_coords. This value needs to be divided by the animation speed to get the actual animation loop duration.

### get_tile_at_coords {#get_tile_at_coords}

```gdscript
Vector2i get_tile_at_coords(atlas_coords: Vector2i) const
```

If there is a tile covering the atlas_coords coordinates, returns the top-left coordinates of the tile (thus its coordinate ID). Returns Vector2i(-1, -1) otherwise.

### get_tile_data {#get_tile_data}

```gdscript
TileData get_tile_data(atlas_coords: Vector2i, alternative_tile: int) const
```

Returns the TileData object for the given atlas coordinates and alternative ID.

### get_tile_size_in_atlas {#get_tile_size_in_atlas}

```gdscript
Vector2i get_tile_size_in_atlas(atlas_coords: Vector2i) const
```

Returns the size of the tile (in the grid coordinates system) at coordinates atlas_coords.

### get_tile_texture_region {#get_tile_texture_region}

```gdscript
Rect2i get_tile_texture_region(atlas_coords: Vector2i, frame: int = 0) const
```

Returns a tile's texture region in the atlas texture. For animated tiles, a frame argument might be provided for the different frames of the animation.

### get_tiles_to_be_removed_on_change {#get_tiles_to_be_removed_on_change}

```gdscript
PackedVector2Array get_tiles_to_be_removed_on_change(texture: Texture2D, margins: Vector2i, separation: Vector2i, texture_region_size: Vector2i)
```

Returns an array of tiles coordinates ID that will be automatically removed when modifying one or several of those properties: texture, margins, separation or texture_region_size. This can be used to undo changes that would have caused tiles data loss.

### get_use_texture_padding {#get_use_texture_padding}

```gdscript
bool get_use_texture_padding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_room_for_tile {#has_room_for_tile}

```gdscript
bool has_room_for_tile(atlas_coords: Vector2i, size: Vector2i, animation_columns: int, animation_separation: Vector2i, frames_count: int, ignored_tile: Vector2i = Vector2i(-1, -1)) const
```

Returns whether there is enough room in an atlas to create/modify a tile with the given properties. If ignored_tile is provided, act as is the given tile was not present in the atlas. This may be used when you want to modify a tile's properties.

### has_tiles_outside_texture {#has_tiles_outside_texture}

```gdscript
bool has_tiles_outside_texture() const
```

Checks if the source has any tiles that don't fit the texture area (either partially or completely).

### move_tile_in_atlas {#move_tile_in_atlas}

```gdscript
void move_tile_in_atlas(atlas_coords: Vector2i, new_atlas_coords: Vector2i = Vector2i(-1, -1), new_size: Vector2i = Vector2i(-1, -1))
```

Move the tile and its alternatives at the atlas_coords coordinates to the new_atlas_coords coordinates with the new_size size. This functions will fail if a tile is already present in the given area.

If new_atlas_coords is Vector2i(-1, -1), keeps the tile's coordinates. If new_size is Vector2i(-1, -1), keeps the tile's size.

To avoid an error, first check if a move is possible using has_room_for_tile().

### remove_alternative_tile {#remove_alternative_tile}

```gdscript
void remove_alternative_tile(atlas_coords: Vector2i, alternative_tile: int)
```

Remove a tile's alternative with alternative ID alternative_tile.

Calling this function with alternative_tile equals to 0 will fail, as the base tile alternative cannot be removed.

### remove_tile {#remove_tile}

```gdscript
void remove_tile(atlas_coords: Vector2i)
```

Remove a tile and its alternative at coordinates atlas_coords.

### set_alternative_tile_id {#set_alternative_tile_id}

```gdscript
void set_alternative_tile_id(atlas_coords: Vector2i, alternative_tile: int, new_id: int)
```

Change a tile's alternative ID from alternative_tile to new_id.

Calling this function with new_id of 0 will fail, as the base tile alternative cannot be moved.

### set_margins {#set_margins}

```gdscript
void set_margins(margins: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_separation {#set_separation}

```gdscript
void set_separation(separation: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_region_size {#set_texture_region_size}

```gdscript
void set_texture_region_size(texture_region_size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_animation_columns {#set_tile_animation_columns}

```gdscript
void set_tile_animation_columns(atlas_coords: Vector2i, frame_columns: int)
```

Sets the number of columns in the animation layout of the tile at coordinates atlas_coords. If set to 0, then the different frames of the animation are laid out as a single horizontal line in the atlas.

### set_tile_animation_frame_duration {#set_tile_animation_frame_duration}

```gdscript
void set_tile_animation_frame_duration(atlas_coords: Vector2i, frame_index: int, duration: float)
```

Sets the animation frame duration of frame frame_index for the tile at coordinates atlas_coords.

### set_tile_animation_frames_count {#set_tile_animation_frames_count}

```gdscript
void set_tile_animation_frames_count(atlas_coords: Vector2i, frames_count: int)
```

Sets how many animation frames the tile at coordinates atlas_coords has.

### set_tile_animation_mode {#set_tile_animation_mode}

```gdscript
void set_tile_animation_mode(atlas_coords: Vector2i, mode: TileSetAtlasSource.TileAnimationMode)
```

Sets the tile animation mode of the tile at atlas_coords to mode. See also get_tile_animation_mode().

### set_tile_animation_separation {#set_tile_animation_separation}

```gdscript
void set_tile_animation_separation(atlas_coords: Vector2i, separation: Vector2i)
```

Sets the margin (in grid tiles) between each tile in the animation layout of the tile at coordinates atlas_coords has.

### set_tile_animation_speed {#set_tile_animation_speed}

```gdscript
void set_tile_animation_speed(atlas_coords: Vector2i, speed: float)
```

Sets the animation speed of the tile at coordinates atlas_coords has.

### set_use_texture_padding {#set_use_texture_padding}

```gdscript
void set_use_texture_padding(use_texture_padding: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The atlas texture.

### margins {#margins}

```gdscript
margins: Vector2i
```

- void set_margins(value: Vector2i)
- Vector2i get_margins()

Margins, in pixels, to offset the origin of the grid in the texture.

### separation {#separation}

```gdscript
separation: Vector2i
```

- void set_separation(value: Vector2i)
- Vector2i get_separation()

Separation, in pixels, between each tile texture region of the grid.

### texture_region_size {#texture_region_size}

```gdscript
texture_region_size: Vector2i
```

- void set_texture_region_size(value: Vector2i)
- Vector2i get_texture_region_size()

The base tile size in the texture (in pixel). This size must be bigger than or equal to the TileSet's tile_size value.

### use_texture_padding {#use_texture_padding}

```gdscript
use_texture_padding: bool
```

- void set_use_texture_padding(value: bool)
- bool get_use_texture_padding()

If true, generates an internal texture with an additional one pixel padding around each tile. Texture padding avoids a common artifact where lines appear between tiles.

Disabling this setting might lead a small performance improvement, as generating the internal texture requires both memory and processing time when the TileSetAtlasSource resource is modified.

## 枚举

### TileAnimationMode {#enum-TileAnimationMode}

```gdscript
enum TileAnimationMode { TILE_ANIMATION_MODE_DEFAULT = 0, TILE_ANIMATION_MODE_RANDOM_START_TIMES = 1, TILE_ANIMATION_MODE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TILE_ANIMATION_MODE_DEFAULT {#TILE_ANIMATION_MODE_DEFAULT}

```gdscript
TILE_ANIMATION_MODE_DEFAULT = 0
```

Tile animations start at same time, looking identical.

#### TILE_ANIMATION_MODE_RANDOM_START_TIMES {#TILE_ANIMATION_MODE_RANDOM_START_TIMES}

```gdscript
TILE_ANIMATION_MODE_RANDOM_START_TIMES = 1
```

Tile animations start at random times, looking varied.

#### TILE_ANIMATION_MODE_MAX {#TILE_ANIMATION_MODE_MAX}

```gdscript
TILE_ANIMATION_MODE_MAX = 2
```

Represents the size of the TileAnimationMode enum.

---

## 常量

### TRANSFORM_FLIP_H {#TRANSFORM_FLIP_H}

```gdscript
const TRANSFORM_FLIP_H = 4096
```

Represents cell's horizontal flip flag. Should be used directly with TileMapLayer to flip placed tiles by altering their alternative IDs.

```gdscript
var alternate_id = $TileMapLayer.get_cell_alternative_tile(Vector2i(2, 2))
if not alternate_id & TileSetAtlasSource.TRANSFORM_FLIP_H:
    # If tile is not already flipped, flip it.
    $TileMapLayer.set_cell(Vector2i(2, 2), source_id, atlas_coords, alternate_id | TileSetAtlasSource.TRANSFORM_FLIP_H)
```

Note: These transformations can be combined to do the equivalent of 0, 90, 180, and 270 degree rotations, as shown below:

```gdscript
enum TileTransform {
    ROTATE_0 = 0,
    ROTATE_90 = TileSetAtlasSource.TRANSFORM_TRANSPOSE | TileSetAtlasSource.TRANSFORM_FLIP_H,
    ROTATE_180 = TileSetAtlasSource.TRANSFORM_FLIP_H | TileSetAtlasSource.TRANSFORM_FLIP_V,
    ROTATE_270 = TileSetAtlasSource.TRANSFORM_TRANSPOSE | TileSetAtlasSource.TRANSFORM_FLIP_V,
}
```

TRANSFORM_FLIP_V = 8192 

Represents cell's vertical flip flag. See TRANSFORM_FLIP_H for usage.

TRANSFORM_TRANSPOSE = 16384 

Represents cell's transposed flag. See TRANSFORM_FLIP_H for usage.

### TRANSFORM_FLIP_V {#TRANSFORM_FLIP_V}

```gdscript
const TRANSFORM_FLIP_V = 8192
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### TRANSFORM_TRANSPOSE {#TRANSFORM_TRANSPOSE}

```gdscript
const TRANSFORM_TRANSPOSE = 16384
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
