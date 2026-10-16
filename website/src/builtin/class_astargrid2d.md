# AStarGrid2D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **AStarGrid2D**

> AStarGrid2D is a variant of AStar2D that is specialized for partial 2D grids. It is simpler to use because it doesn't require you to manually create points and connect them together. This class also supports multiple types of heuristics, modes for diagonal movement, and a jumping mode to speed up calculations.

<details><summary>官方英文描述（中文精译进行中）</summary>

AStarGrid2D is a variant of AStar2D that is specialized for partial 2D grids. It is simpler to use because it doesn't require you to manually create points and connect them together. This class also supports multiple types of heuristics, modes for diagonal movement, and a jumping mode to speed up calculations.

To use AStarGrid2D, you only need to set the region of the grid, optionally set the cell_size, and then call the update() method:

```
var astar_grid = AStarGrid2D.new()
astar_grid.region = Rect2i(0, 0, 32, 32)
astar_grid.cell_size = Vector2(16, 16)
astar_grid.update()
print(astar_grid.get_id_path(Vector2i(0, 0), Vector2i(3, 4))) # Prints [(0, 0), (1, 1), (2, 2), (3, 3), (3, 4)]
print(astar_grid.get_point_path(Vector2i(0, 0), Vector2i(3, 4))) # Prints [(0, 0), (16, 16), (32, 32), (48, 48), (48, 64)]
```

```
AStarGrid2D astarGrid = new AStarGrid2D();
astarGrid.Region = new Rect2I(0, 0, 32, 32);
astarGrid.CellSize = new Vector2I(16, 16);
astarGrid.Update();
GD.Print(astarGrid.GetIdPath(Vector2I.Zero, new Vector2I(3, 4))); // Prints [(0, 0), (1, 1), (2, 2), (3, 3), (3, 4)]
GD.Print(astarGrid.GetPointPath(Vector2I.Zero, new Vector2I(3, 4))); // Prints [(0, 0), (16, 16), (32, 32), (48, 48), (48, 64)]
```

To remove a point from the pathfinding grid, it must be set as "solid" with set_point_solid().

</details>

## 方法

### _compute_cost {#_compute_cost}

```gdscript
float _compute_cost(from_id: Vector2i, to_id: Vector2i) virtual const
```

Called when computing the cost between two connected points.

Note that this function is hidden in the default AStarGrid2D class.

### _estimate_cost {#_estimate_cost}

```gdscript
float _estimate_cost(from_id: Vector2i, end_id: Vector2i) virtual const
```

Called when estimating the cost between a point and the path's ending point.

Note that this function is hidden in the default AStarGrid2D class.

### clear {#clear}

```gdscript
void clear()
```

Clears the grid and sets the region to Rect2i(0, 0, 0, 0).

### fill_solid_region {#fill_solid_region}

```gdscript
void fill_solid_region(region: Rect2i, solid: bool = true)
```

Fills the given region on the grid with the specified value for the solid flag.

Note: Calling update() is not needed after the call of this function.

### fill_weight_scale_region {#fill_weight_scale_region}

```gdscript
void fill_weight_scale_region(region: Rect2i, weight_scale: float)
```

Fills the given region on the grid with the specified value for the weight scale.

Note: Calling update() is not needed after the call of this function.

### get_cell_shape {#get_cell_shape}

```gdscript
AStarGrid2D.CellShape get_cell_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cell_size {#get_cell_size}

```gdscript
Vector2 get_cell_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_compute_heuristic {#get_default_compute_heuristic}

```gdscript
AStarGrid2D.Heuristic get_default_compute_heuristic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_estimate_heuristic {#get_default_estimate_heuristic}

```gdscript
AStarGrid2D.Heuristic get_default_estimate_heuristic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_diagonal_mode {#get_diagonal_mode}

```gdscript
AStarGrid2D.DiagonalMode get_diagonal_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_id_path {#get_id_path}

```gdscript
typedarray::Vector2i get_id_path(from_id: Vector2i, to_id: Vector2i, allow_partial_path: bool = false)
```

Returns an array with the IDs of the points that form the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.

If from_id point is disabled, returns an empty array (even if from_id == to_id).

If from_id point is not disabled, there is no valid path to the target, and allow_partial_path is true, returns a path to the point closest to the target that can be reached.

Note: When allow_partial_path is true and to_id is solid the search may take an unusually long time to finish.

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_point_data_in_region {#get_point_data_in_region}

```gdscript
typedarray::Dictionary get_point_data_in_region(region: Rect2i) const
```

Returns an array of dictionaries with point data (id: Vector2i, position: Vector2, solid: bool, weight_scale: float) within a region.

### get_point_path {#get_point_path}

```gdscript
PackedVector2Array get_point_path(from_id: Vector2i, to_id: Vector2i, allow_partial_path: bool = false)
```

Returns an array with the points that are in the path found by AStarGrid2D between the given points. The array is ordered from the starting point to the ending point of the path.

If from_id point is disabled, returns an empty array (even if from_id == to_id).

If from_id point is not disabled, there is no valid path to the target, and allow_partial_path is true, returns a path to the point closest to the target that can be reached.

Note: This method is not thread-safe; it can only be used from a single Thread at a given time. Consider using Mutex to ensure exclusive access to one thread to avoid race conditions.

Additionally, when allow_partial_path is true and to_id is solid the search may take an unusually long time to finish.

### get_point_position {#get_point_position}

```gdscript
Vector2 get_point_position(id: Vector2i) const
```

Returns the position of the point associated with the given id.

### get_point_weight_scale {#get_point_weight_scale}

```gdscript
float get_point_weight_scale(id: Vector2i) const
```

Returns the weight scale of the point associated with the given id.

### get_region {#get_region}

```gdscript
Rect2i get_region() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector2i get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_dirty {#is_dirty}

```gdscript
bool is_dirty() const
```

Indicates that the grid parameters were changed and update() needs to be called.

### is_in_bounds {#is_in_bounds}

```gdscript
bool is_in_bounds(x: int, y: int) const
```

Returns true if the x and y is a valid grid coordinate (id), i.e. if it is inside region. Equivalent to region.has_point(Vector2i(x, y)).

### is_in_boundsv {#is_in_boundsv}

```gdscript
bool is_in_boundsv(id: Vector2i) const
```

Returns true if the id vector is a valid grid coordinate, i.e. if it is inside region. Equivalent to region.has_point(id).

### is_jumping_enabled {#is_jumping_enabled}

```gdscript
bool is_jumping_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_point_solid {#is_point_solid}

```gdscript
bool is_point_solid(id: Vector2i) const
```

Returns true if a point is disabled for pathfinding. By default, all points are enabled.

### set_cell_shape {#set_cell_shape}

```gdscript
void set_cell_shape(cell_shape: AStarGrid2D.CellShape)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_size {#set_cell_size}

```gdscript
void set_cell_size(cell_size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_compute_heuristic {#set_default_compute_heuristic}

```gdscript
void set_default_compute_heuristic(heuristic: AStarGrid2D.Heuristic)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_estimate_heuristic {#set_default_estimate_heuristic}

```gdscript
void set_default_estimate_heuristic(heuristic: AStarGrid2D.Heuristic)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_diagonal_mode {#set_diagonal_mode}

```gdscript
void set_diagonal_mode(mode: AStarGrid2D.DiagonalMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_jumping_enabled {#set_jumping_enabled}

```gdscript
void set_jumping_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_solid {#set_point_solid}

```gdscript
void set_point_solid(id: Vector2i, solid: bool = true)
```

Disables or enables the specified point for pathfinding. Useful for making an obstacle. By default, all points are enabled.

Note: Calling update() is not needed after the call of this function.

### set_point_weight_scale {#set_point_weight_scale}

```gdscript
void set_point_weight_scale(id: Vector2i, weight_scale: float)
```

Sets the weight_scale for the point with the given id. The weight_scale is multiplied by the result of _compute_cost() when determining the overall cost of traveling across a segment from a neighboring point to this point.

Note: Calling update() is not needed after the call of this function.

### set_region {#set_region}

```gdscript
void set_region(region: Rect2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update {#update}

```gdscript
void update()
```

Updates the internal state of the grid according to the parameters to prepare it to search the path. Needs to be called if parameters like region, cell_size or offset are changed. is_dirty() will return true if this is the case and this needs to be called.

Note: All point data (solidity and weight scale) will be cleared.

## 属性

### region {#region}

```gdscript
region: Rect2i
```

- void set_region(value: Rect2i)
- Rect2i get_region()

The region of grid cells available for pathfinding. If changed, update() needs to be called before finding the next path.

### size {#size}

```gdscript
size: Vector2i
```

- void set_size(value: Vector2i)
- Vector2i get_size()

Deprecated: Use region instead.

The size of the grid (number of cells of size cell_size on each axis). If changed, update() needs to be called before finding the next path.

### offset {#offset}

```gdscript
offset: Vector2
```

- void set_offset(value: Vector2)
- Vector2 get_offset()

The offset of the grid which will be applied to calculate the resulting point position returned by get_point_path(). If changed, update() needs to be called before finding the next path.

### cell_size {#cell_size}

```gdscript
cell_size: Vector2
```

- void set_cell_size(value: Vector2)
- Vector2 get_cell_size()

The size of the point cell which will be applied to calculate the resulting point position returned by get_point_path(). If changed, update() needs to be called before finding the next path.

### cell_shape {#cell_shape}

```gdscript
cell_shape: int
```

- void set_cell_shape(value: CellShape)
- CellShape get_cell_shape()

The cell shape. Affects how the positions are placed in the grid. If changed, update() needs to be called before finding the next path.

### jumping_enabled {#jumping_enabled}

```gdscript
jumping_enabled: bool
```

- void set_jumping_enabled(value: bool)
- bool is_jumping_enabled()

Enables or disables jumping to skip up the intermediate points and speeds up the searching algorithm.

Note: Currently, toggling it on disables the consideration of weight scaling in pathfinding.

### default_compute_heuristic {#default_compute_heuristic}

```gdscript
default_compute_heuristic: int
```

- void set_default_compute_heuristic(value: Heuristic)
- Heuristic get_default_compute_heuristic()

The default Heuristic which will be used to calculate the cost between two points if _compute_cost() was not overridden.

### default_estimate_heuristic {#default_estimate_heuristic}

```gdscript
default_estimate_heuristic: int
```

- void set_default_estimate_heuristic(value: Heuristic)
- Heuristic get_default_estimate_heuristic()

The default Heuristic which will be used to calculate the cost between the point and the end point if _estimate_cost() was not overridden.

### diagonal_mode {#diagonal_mode}

```gdscript
diagonal_mode: int
```

- void set_diagonal_mode(value: DiagonalMode)
- DiagonalMode get_diagonal_mode()

A specific DiagonalMode mode which will force the path to avoid or accept the specified diagonals.

## 枚举

### Heuristic {#enum-Heuristic}

```gdscript
enum Heuristic { HEURISTIC_EUCLIDEAN = 0, HEURISTIC_MANHATTAN = 1, HEURISTIC_OCTILE = 2, HEURISTIC_CHEBYSHEV = 3, HEURISTIC_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HEURISTIC_EUCLIDEAN {#HEURISTIC_EUCLIDEAN}

```gdscript
HEURISTIC_EUCLIDEAN = 0
```

The Euclidean heuristic [https://en.wikipedia.org/wiki/Euclidean_distance] to be used for the pathfinding using the following formula:

```gdscript
dx = abs(to_id.x - from_id.x)
dy = abs(to_id.y - from_id.y)
result = sqrt(dx * dx + dy * dy)
```

Note: This is also the internal heuristic used in AStar3D and AStar2D by default (with the inclusion of possible z-axis coordinate).

#### HEURISTIC_MANHATTAN {#HEURISTIC_MANHATTAN}

```gdscript
HEURISTIC_MANHATTAN = 1
```

The Manhattan heuristic [https://en.wikipedia.org/wiki/Taxicab_geometry] to be used for the pathfinding using the following formula:

```gdscript
dx = abs(to_id.x - from_id.x)
dy = abs(to_id.y - from_id.y)
result = dx + dy
```

Note: This heuristic is intended to be used with 4-side orthogonal movements, provided by setting the diagonal_mode to DIAGONAL_MODE_NEVER.

#### HEURISTIC_OCTILE {#HEURISTIC_OCTILE}

```gdscript
HEURISTIC_OCTILE = 2
```

The Octile heuristic to be used for the pathfinding using the following formula:

```gdscript
dx = abs(to_id.x - from_id.x)
dy = abs(to_id.y - from_id.y)
f = sqrt(2) - 1
result = (dx < dy) ? f * dx + dy : f * dy + dx;
```

#### HEURISTIC_CHEBYSHEV {#HEURISTIC_CHEBYSHEV}

```gdscript
HEURISTIC_CHEBYSHEV = 3
```

The Chebyshev heuristic [https://en.wikipedia.org/wiki/Chebyshev_distance] to be used for the pathfinding using the following formula:

```gdscript
dx = abs(to_id.x - from_id.x)
dy = abs(to_id.y - from_id.y)
result = max(dx, dy)
```

#### HEURISTIC_MAX {#HEURISTIC_MAX}

```gdscript
HEURISTIC_MAX = 4
```

Represents the size of the Heuristic enum.

---

### DiagonalMode {#enum-DiagonalMode}

```gdscript
enum DiagonalMode { DIAGONAL_MODE_ALWAYS = 0, DIAGONAL_MODE_NEVER = 1, DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE = 2, DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES = 3, DIAGONAL_MODE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DIAGONAL_MODE_ALWAYS {#DIAGONAL_MODE_ALWAYS}

```gdscript
DIAGONAL_MODE_ALWAYS = 0
```

The pathfinding algorithm will ignore solid neighbors around the target cell and allow passing using diagonals.

#### DIAGONAL_MODE_NEVER {#DIAGONAL_MODE_NEVER}

```gdscript
DIAGONAL_MODE_NEVER = 1
```

The pathfinding algorithm will ignore all diagonals and the way will be always orthogonal.

#### DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE {#DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE}

```gdscript
DIAGONAL_MODE_AT_LEAST_ONE_WALKABLE = 2
```

The pathfinding algorithm will avoid using diagonals if at least two obstacles have been placed around the neighboring cells of the specific path segment.

#### DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES {#DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES}

```gdscript
DIAGONAL_MODE_ONLY_IF_NO_OBSTACLES = 3
```

The pathfinding algorithm will avoid using diagonals if any obstacle has been placed around the neighboring cells of the specific path segment.

#### DIAGONAL_MODE_MAX {#DIAGONAL_MODE_MAX}

```gdscript
DIAGONAL_MODE_MAX = 4
```

Represents the size of the DiagonalMode enum.

---

### CellShape {#enum-CellShape}

```gdscript
enum CellShape { CELL_SHAPE_SQUARE = 0, CELL_SHAPE_ISOMETRIC_RIGHT = 1, CELL_SHAPE_ISOMETRIC_DOWN = 2, CELL_SHAPE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CELL_SHAPE_SQUARE {#CELL_SHAPE_SQUARE}

```gdscript
CELL_SHAPE_SQUARE = 0
```

Rectangular cell shape.

#### CELL_SHAPE_ISOMETRIC_RIGHT {#CELL_SHAPE_ISOMETRIC_RIGHT}

```gdscript
CELL_SHAPE_ISOMETRIC_RIGHT = 1
```

Diamond cell shape (for isometric look). Cell coordinates layout where the horizontal axis goes up-right, and the vertical one goes down-right.

#### CELL_SHAPE_ISOMETRIC_DOWN {#CELL_SHAPE_ISOMETRIC_DOWN}

```gdscript
CELL_SHAPE_ISOMETRIC_DOWN = 2
```

Diamond cell shape (for isometric look). Cell coordinates layout where the horizontal axis goes down-right, and the vertical one goes down-left.

#### CELL_SHAPE_MAX {#CELL_SHAPE_MAX}

```gdscript
CELL_SHAPE_MAX = 3
```

Represents the size of the CellShape enum.

---
