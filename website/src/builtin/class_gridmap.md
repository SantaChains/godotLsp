# GridMap

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **GridMap**

> GridMap lets you place meshes on a grid interactively. It works both from the editor and from scripts, which can help you create in-game level editors.

<details><summary>官方英文描述（中文精译进行中）</summary>

GridMap lets you place meshes on a grid interactively. It works both from the editor and from scripts, which can help you create in-game level editors.

GridMaps use a MeshLibrary which contains a list of tiles. Each tile is a mesh with materials plus optional collision and navigation shapes.

A GridMap contains a collection of cells. Each grid cell refers to a tile in the MeshLibrary. All cells in the map have the same dimensions.

Internally, a GridMap is split into a sparse collection of octants for efficient rendering and physics processing. Every octant has the same dimensions and can contain several cells.

Note: GridMap doesn't extend VisualInstance3D and therefore can't be hidden or cull masked based on VisualInstance3D.layers. If you make a light not affect the first layer, the whole GridMap won't be lit by the light in question.

</details>

## 方法

### clear {#clear}

```gdscript
void clear()
```

Clear all cells.

### clear_baked_meshes {#clear_baked_meshes}

```gdscript
void clear_baked_meshes()
```

Clears all baked meshes. See make_baked_meshes().

### get_bake_mesh_instance {#get_bake_mesh_instance}

```gdscript
RID get_bake_mesh_instance(idx: int)
```

Returns RID of a baked mesh with the given idx.

### get_bake_meshes {#get_bake_meshes}

```gdscript
Array get_bake_meshes()
```

Returns an array of ArrayMeshes and Transform3D references of all bake meshes that exist within the current GridMap. Even indices contain ArrayMeshes, while odd indices contain Transform3Ds that are always equal to Transform3D.IDENTITY.

This method relies on the output of make_baked_meshes(), which will be called with gen_lightmap_uv set to true and lightmap_uv_texel_size set to 0.1 if it hasn't been called yet.

### get_basis_with_orthogonal_index {#get_basis_with_orthogonal_index}

```gdscript
Basis get_basis_with_orthogonal_index(index: int) const
```

Returns one of 24 possible rotations that lie along the vectors (x,y,z) with each component being either -1, 0, or 1. For further details, refer to the Godot source code.

### get_cell_item {#get_cell_item}

```gdscript
int get_cell_item(position: Vector3i) const
```

The MeshLibrary item index located at the given grid coordinates. If the cell is empty, INVALID_CELL_ITEM will be returned.

### get_cell_item_basis {#get_cell_item_basis}

```gdscript
Basis get_cell_item_basis(position: Vector3i) const
```

Returns the basis that gives the specified cell its orientation.

### get_cell_item_orientation {#get_cell_item_orientation}

```gdscript
int get_cell_item_orientation(position: Vector3i) const
```

The orientation of the cell at the given grid coordinates. -1 is returned if the cell is empty.

### get_cell_scale {#get_cell_scale}

```gdscript
float get_cell_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cell_size {#get_cell_size}

```gdscript
Vector3 get_cell_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_x {#get_center_x}

```gdscript
bool get_center_x() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_y {#get_center_y}

```gdscript
bool get_center_y() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_z {#get_center_z}

```gdscript
bool get_center_z() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_layer {#get_collision_layer}

```gdscript
int get_collision_layer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_layer_value {#get_collision_layer_value}

```gdscript
bool get_collision_layer_value(layer_number: int) const
```

Returns whether or not the specified layer of the collision_layer is enabled, given a layer_number between 1 and 32.

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_mask_value {#get_collision_mask_value}

```gdscript
bool get_collision_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the collision_mask is enabled, given a layer_number between 1 and 32.

### get_collision_priority {#get_collision_priority}

```gdscript
float get_collision_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_visibility_mode {#get_collision_visibility_mode}

```gdscript
GridMap.DebugVisibilityMode get_collision_visibility_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh_library {#get_mesh_library}

```gdscript
MeshLibrary get_mesh_library() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_meshes {#get_meshes}

```gdscript
Array get_meshes() const
```

Returns an array of Transform3D and Mesh references corresponding to the non-empty cells in the grid. The transforms are specified in local space. Even indices contain Transform3Ds, while odd indices contain Meshes related to the Transform3D in the index preceding it.

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

Returns the RID of the navigation map this GridMap node uses for its cell baked navigation meshes.

This function returns always the map set on the GridMap node and not the map on the NavigationServer. If the map is changed directly with the NavigationServer API the GridMap node will not be aware of the map change.

### get_octant_coords_from_cell_coords {#get_octant_coords_from_cell_coords}

```gdscript
Vector3i get_octant_coords_from_cell_coords(cell_coords: Vector3i) const
```

Returns the Vector3i octant coordinates of the octant that the cell at cell_coords belongs to.

### get_octant_size {#get_octant_size}

```gdscript
int get_octant_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_octants_in_bounds {#get_octants_in_bounds}

```gdscript
typedarray::Vector3i get_octants_in_bounds(bounds: AABB) const
```

Returns an array of Vector3i octant coordinates that are inside the given bounds, including octants that have no cells in use.

### get_orthogonal_index_from_basis {#get_orthogonal_index_from_basis}

```gdscript
int get_orthogonal_index_from_basis(basis: Basis) const
```

This function considers a discretization of rotations into 24 points on unit sphere, lying along the vectors (x,y,z) with each component being either -1, 0, or 1, and returns the index (in the range from 0 to 23) of the point best representing the orientation of the object. For further details, refer to the Godot source code.

### get_physics_material {#get_physics_material}

```gdscript
PhysicsMaterial get_physics_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_used_cells {#get_used_cells}

```gdscript
typedarray::Vector3i get_used_cells() const
```

Returns an array of Vector3 with the non-empty cell coordinates in the grid map.

### get_used_cells_by_item {#get_used_cells_by_item}

```gdscript
typedarray::Vector3i get_used_cells_by_item(item: int) const
```

Returns an array of all cells with the given item index specified in item.

### get_used_cells_in_octant {#get_used_cells_in_octant}

```gdscript
typedarray::Vector3i get_used_cells_in_octant(octant_coords: Vector3i) const
```

Returns an array of Vector3is with the cell coordinates of non-empty cells inside the octant at octant_coords.

### get_used_cells_in_octant_by_item {#get_used_cells_in_octant_by_item}

```gdscript
typedarray::Vector3i get_used_cells_in_octant_by_item(octant_coords: Vector3i, item: int) const
```

Returns an array of Vector3is with the cell coordinates of cells inside the octant at octant_coords that use the specified cell item.

### get_used_octants {#get_used_octants}

```gdscript
typedarray::Vector3i get_used_octants() const
```

Returns an array of Vector3is with the octant coordinates of the non-empty octants in the grid map.

### get_used_octants_by_item {#get_used_octants_by_item}

```gdscript
typedarray::Vector3i get_used_octants_by_item(item: int) const
```

Returns an array of Vector3is with the octant coordinates of the octants that use the specified item in the grid map.

### get_used_octants_in_bounds {#get_used_octants_in_bounds}

```gdscript
typedarray::Vector3i get_used_octants_in_bounds(bounds: AABB) const
```

Returns an array of Vector3is with the octant coordinates of non-empty octants that are inside the local bounds.

### is_baking_navigation {#is_baking_navigation}

```gdscript
bool is_baking_navigation()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### local_to_map {#local_to_map}

```gdscript
Vector3i local_to_map(local_position: Vector3) const
```

Returns the map coordinates of the cell containing the given local_position. If local_position is in global coordinates, consider using Node3D.to_local() before passing it to this method. See also map_to_local().

### make_baked_meshes {#make_baked_meshes}

```gdscript
void make_baked_meshes(gen_lightmap_uv: bool = false, lightmap_uv_texel_size: float = 0.1)
```

Generates a baked mesh that represents all meshes in the assigned MeshLibrary for use with LightmapGI. If gen_lightmap_uv is true, UV2 data will be generated for each mesh currently used in the GridMap. Otherwise, only meshes that already have UV2 data present will be able to use baked lightmaps. When generating UV2, lightmap_uv_texel_size controls the texel density for lightmaps, with lower values resulting in more detailed lightmaps. lightmap_uv_texel_size is ignored if gen_lightmap_uv is false. See also get_bake_meshes(), which relies on the output of this method.

Note: Calling this method will not actually bake lightmaps, as lightmap baking is performed using the LightmapGI node.

### map_to_local {#map_to_local}

```gdscript
Vector3 map_to_local(map_position: Vector3i) const
```

Returns the position of a grid cell in the GridMap's local coordinate space. To convert the returned value into global coordinates, use Node3D.to_global(). See also local_to_map().

### resource_changed {#resource_changed}

```gdscript
void resource_changed(resource: Resource)
```

Deprecated: Use Resource.changed instead.

This method does nothing.

### set_bake_navigation {#set_bake_navigation}

```gdscript
void set_bake_navigation(bake_navigation: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_item {#set_cell_item}

```gdscript
void set_cell_item(position: Vector3i, item: int, orientation: int = 0)
```

Sets the mesh index for the cell referenced by its grid coordinates.

A negative item index such as INVALID_CELL_ITEM will clear the cell.

Optionally, the item's orientation can be passed. For valid orientation values, see get_orthogonal_index_from_basis().

### set_cell_scale {#set_cell_scale}

```gdscript
void set_cell_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_size {#set_cell_size}

```gdscript
void set_cell_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_x {#set_center_x}

```gdscript
void set_center_x(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_y {#set_center_y}

```gdscript
void set_center_y(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_z {#set_center_z}

```gdscript
void set_center_z(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_layer {#set_collision_layer}

```gdscript
void set_collision_layer(layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_layer_value {#set_collision_layer_value}

```gdscript
void set_collision_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_layer, given a layer_number between 1 and 32.

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask_value {#set_collision_mask_value}

```gdscript
void set_collision_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_mask, given a layer_number between 1 and 32.

### set_collision_priority {#set_collision_priority}

```gdscript
void set_collision_priority(priority: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_visibility_mode {#set_collision_visibility_mode}

```gdscript
void set_collision_visibility_mode(visibility_mode: GridMap.DebugVisibilityMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh_library {#set_mesh_library}

```gdscript
void set_mesh_library(mesh_library: MeshLibrary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_map {#set_navigation_map}

```gdscript
void set_navigation_map(navigation_map: RID)
```

Sets the RID of the navigation map this GridMap node should use for its cell baked navigation meshes.

### set_octant_size {#set_octant_size}

```gdscript
void set_octant_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_material {#set_physics_material}

```gdscript
void set_physics_material(material: PhysicsMaterial)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mesh_library {#mesh_library}

```gdscript
mesh_library: MeshLibrary
```

- void set_mesh_library(value: MeshLibrary)
- MeshLibrary get_mesh_library()

The assigned MeshLibrary.

### physics_material {#physics_material}

```gdscript
physics_material: PhysicsMaterial
```

- void set_physics_material(value: PhysicsMaterial)
- PhysicsMaterial get_physics_material()

Overrides the default friction and bounce physics properties for the whole GridMap.

### cell_size {#cell_size}

```gdscript
cell_size: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### cell_octant_size {#cell_octant_size}

```gdscript
cell_octant_size: int
```

- void set_octant_size(value: int)
- int get_octant_size()

The size of each octant measured in number of cells. This applies to all three axis.

### cell_center_x {#cell_center_x}

```gdscript
cell_center_x: bool
```

- void set_center_x(value: bool)
- bool get_center_x()

If true, grid items are centered on the X axis.

### cell_center_y {#cell_center_y}

```gdscript
cell_center_y: bool
```

- void set_center_y(value: bool)
- bool get_center_y()

If true, grid items are centered on the Y axis.

### cell_center_z {#cell_center_z}

```gdscript
cell_center_z: bool
```

- void set_center_z(value: bool)
- bool get_center_z()

If true, grid items are centered on the Z axis.

### cell_scale {#cell_scale}

```gdscript
cell_scale: float
```

- void set_cell_scale(value: float)
- float get_cell_scale()

The scale of the cell items.

This does not affect the size of the grid cells themselves, only the items in them. This can be used to make cell items overlap their neighbors.

### collision_layer {#collision_layer}

```gdscript
collision_layer: int
```

- void set_collision_layer(value: int)
- int get_collision_layer()

The physics layers this GridMap is in.

GridMaps act as static bodies, meaning they aren't affected by gravity or other forces. They only affect other physics bodies that collide with them.

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The physics layers this GridMap detects collisions in. See Collision layers and masks in the documentation for more information.

### collision_priority {#collision_priority}

```gdscript
collision_priority: float
```

- void set_collision_priority(value: float)
- float get_collision_priority()

The priority used to solve colliding when occurring penetration. The higher the priority is, the lower the penetration into the object will be. This can for example be used to prevent the player from breaking through the boundaries of a level.

### collision_visibility_mode {#collision_visibility_mode}

```gdscript
collision_visibility_mode: int
```

- void set_collision_visibility_mode(value: DebugVisibilityMode)
- DebugVisibilityMode get_collision_visibility_mode()

Show or hide the GridMap's collision shapes. If set to DEBUG_VISIBILITY_MODE_DEFAULT, this depends on the show collision debug settings.

### bake_navigation {#bake_navigation}

```gdscript
bake_navigation: bool
```

- void set_bake_navigation(value: bool)
- bool is_baking_navigation()

If true, this GridMap creates a navigation region for each cell that uses a mesh_library item with a navigation mesh. The created navigation region will use the navigation layers bitmask assigned to the MeshLibrary's item.

## 信号

### cell_size_changed {#cell_size_changed}

```gdscript
signal cell_size_changed(cell_size: Vector3)
```

Emitted when cell_size changes.

### changed {#changed}

```gdscript
signal changed()
```

Emitted when the MeshLibrary of this GridMap changes.

## 枚举

### DebugVisibilityMode {#enum-DebugVisibilityMode}

```gdscript
enum DebugVisibilityMode { DEBUG_VISIBILITY_MODE_DEFAULT = 0, DEBUG_VISIBILITY_MODE_FORCE_SHOW = 1, DEBUG_VISIBILITY_MODE_FORCE_HIDE = 2 }
```

DebugVisibilityMode DEBUG_VISIBILITY_MODE_DEFAULT = 0

Hide the collisions debug shapes in the editor, and use the debug settings to determine their visibility in game (i.e. SceneTree.debug_collisions_hint or SceneTree.debug_navigation_hint).

DebugVisibilityMode DEBUG_VISIBILITY_MODE_FORCE_SHOW = 1

Always show the collisions debug shapes.

DebugVisibilityMode DEBUG_VISIBILITY_MODE_FORCE_HIDE = 2

Always hide the collisions debug shapes.

#### DEBUG_VISIBILITY_MODE_DEFAULT {#DEBUG_VISIBILITY_MODE_DEFAULT}

```gdscript
DEBUG_VISIBILITY_MODE_DEFAULT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_VISIBILITY_MODE_FORCE_SHOW {#DEBUG_VISIBILITY_MODE_FORCE_SHOW}

```gdscript
DEBUG_VISIBILITY_MODE_FORCE_SHOW = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_VISIBILITY_MODE_FORCE_HIDE {#DEBUG_VISIBILITY_MODE_FORCE_HIDE}

```gdscript
DEBUG_VISIBILITY_MODE_FORCE_HIDE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 常量

### INVALID_CELL_ITEM {#INVALID_CELL_ITEM}

```gdscript
const INVALID_CELL_ITEM = -1
```

Invalid cell item that can be used in set_cell_item() to clear cells (or represent an empty cell in get_cell_item()).
