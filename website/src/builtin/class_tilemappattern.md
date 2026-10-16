# TileMapPattern

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TileMapPattern**

> This resource holds a set of cells to help bulk manipulations of TileMap.

<details><summary>官方英文描述</summary>

This resource holds a set of cells to help bulk manipulations of TileMap.

A pattern always starts at the (0, 0) coordinates and cannot have cells with negative coordinates.

</details>

## 方法

### get_cell_alternative_tile {#get_cell_alternative_tile}

```gdscript
int get_cell_alternative_tile(coords: Vector2i) const
```

Returns the tile alternative ID of the cell at coords.

### get_cell_atlas_coords {#get_cell_atlas_coords}

```gdscript
Vector2i get_cell_atlas_coords(coords: Vector2i) const
```

Returns the tile atlas coordinates ID of the cell at coords.

### get_cell_source_id {#get_cell_source_id}

```gdscript
int get_cell_source_id(coords: Vector2i) const
```

Returns the tile source ID of the cell at coords.

### get_size {#get_size}

```gdscript
Vector2i get_size() const
```

Returns the size, in cells, of the pattern.

### get_used_cells {#get_used_cells}

```gdscript
typedarray::Vector2i get_used_cells() const
```

Returns the list of used cell coordinates in the pattern.

### has_cell {#has_cell}

```gdscript
bool has_cell(coords: Vector2i) const
```

Returns whether the pattern has a tile at the given coordinates.

### is_empty {#is_empty}

```gdscript
bool is_empty() const
```

Returns whether the pattern is empty or not.

### remove_cell {#remove_cell}

```gdscript
void remove_cell(coords: Vector2i, update_size: bool)
```

Remove the cell at the given coordinates.

### set_cell {#set_cell}

```gdscript
void set_cell(coords: Vector2i, source_id: int = -1, atlas_coords: Vector2i = Vector2i(-1, -1), alternative_tile: int = -1)
```

Sets the tile identifiers for the cell at coordinates coords. See TileMap.set_cell().

### set_size {#set_size}

```gdscript
void set_size(size: Vector2i)
```

Sets the size of the pattern.
