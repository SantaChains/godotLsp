# NavigationPolygon

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **NavigationPolygon**

> A navigation mesh can be created either by baking it with the help of the NavigationServer2D, or by adding vertices and convex polygon indices arrays manually.

<details><summary>官方英文描述（中文精译进行中）</summary>

A navigation mesh can be created either by baking it with the help of the NavigationServer2D, or by adding vertices and convex polygon indices arrays manually.

To bake a navigation mesh at least one outline needs to be added that defines the outer bounds of the baked area.

```
var new_navigation_mesh = NavigationPolygon.new()
var bounding_outline = PackedVector2Array([Vector2(0, 0), Vector2(0, 50), Vector2(50, 50), Vector2(50, 0)])
new_navigation_mesh.add_outline(bounding_outline)
NavigationServer2D.bake_from_source_geometry_data(new_navigation_mesh, NavigationMeshSourceGeometryData2D.new());
$NavigationRegion2D.navigation_polygon = new_navigation_mesh
```

```
var newNavigationMesh = new NavigationPolygon();
Vector2[] boundingOutline = [new Vector2(0, 0), new Vector2(0, 50), new Vector2(50, 50), new Vector2(50, 0)];
newNavigationMesh.AddOutline(boundingOutline);
NavigationServer2D.BakeFromSourceGeometryData(newNavigationMesh, new NavigationMeshSourceGeometryData2D());
GetNode("NavigationRegion2D").NavigationPolygon = newNavigationMesh;
```

Adding vertices and polygon indices manually.

```
var new_navigation_mesh = NavigationPolygon.new()
var new_vertices = PackedVector2Array([Vector2(0, 0), Vector2(0, 50), Vector2(50, 50), Vector2(50, 0)])
new_navigation_mesh.vertices = new_vertices
var new_polygon_indices = PackedInt32Array([0, 1, 2, 3])
new_navigation_mesh.add_polygon(new_polygon_indices)
$NavigationRegion2D.navigation_polygon = new_navigation_mesh
```

```
var newNavigationMesh = new NavigationPolygon();
Vector2[] newVertices = [new Vector2(0, 0), new Vector2(0, 50), new Vector2(50, 50), new Vector2(50, 0)];
newNavigationMesh.Vertices = newVertices;
int[] newPolygonIndices = [0, 1, 2, 3];
newNavigationMesh.AddPolygon(newPolygonIndices);
GetNode("NavigationRegion2D").NavigationPolygon = newNavigationMesh;
```

</details>

## 方法

### add_outline {#add_outline}

```gdscript
void add_outline(outline: PackedVector2Array)
```

Appends a PackedVector2Array that contains the vertices of an outline to the internal array that contains all the outlines.

### add_outline_at_index {#add_outline_at_index}

```gdscript
void add_outline_at_index(outline: PackedVector2Array, index: int)
```

Adds a PackedVector2Array that contains the vertices of an outline to the internal array that contains all the outlines at a fixed position.

### add_polygon {#add_polygon}

```gdscript
void add_polygon(polygon: PackedInt32Array)
```

Adds a polygon using the indices of the vertices you get when calling get_vertices().

### clear {#clear}

```gdscript
void clear()
```

Clears the internal arrays for vertices and polygon indices.

### clear_outlines {#clear_outlines}

```gdscript
void clear_outlines()
```

Clears the array of the outlines, but it doesn't clear the vertices and the polygons that were created by them.

### clear_polygons {#clear_polygons}

```gdscript
void clear_polygons()
```

Clears the array of polygons, but it doesn't clear the array of outlines and vertices.

### get_agent_radius {#get_agent_radius}

```gdscript
float get_agent_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_baking_rect {#get_baking_rect}

```gdscript
Rect2 get_baking_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_baking_rect_offset {#get_baking_rect_offset}

```gdscript
Vector2 get_baking_rect_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_border_size {#get_border_size}

```gdscript
float get_border_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cell_size {#get_cell_size}

```gdscript
float get_cell_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_mesh {#get_navigation_mesh}

```gdscript
NavigationMesh get_navigation_mesh()
```

Returns the NavigationMesh resulting from this navigation polygon. This navigation mesh can be used to update the navigation mesh of a region with the NavigationServer3D.region_set_navigation_mesh() API directly.

### get_outline {#get_outline}

```gdscript
PackedVector2Array get_outline(idx: int) const
```

Returns a PackedVector2Array containing the vertices of an outline that was created in the editor or by script.

### get_outline_count {#get_outline_count}

```gdscript
int get_outline_count() const
```

Returns the number of outlines that were created in the editor or by script.

### get_parsed_collision_mask {#get_parsed_collision_mask}

```gdscript
int get_parsed_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parsed_collision_mask_value {#get_parsed_collision_mask_value}

```gdscript
bool get_parsed_collision_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the parsed_collision_mask is enabled, given a layer_number between 1 and 32.

### get_parsed_geometry_type {#get_parsed_geometry_type}

```gdscript
NavigationPolygon.ParsedGeometryType get_parsed_geometry_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_polygon {#get_polygon}

```gdscript
PackedInt32Array get_polygon(idx: int)
```

Returns a PackedInt32Array containing the indices of the vertices of a created polygon.

### get_polygon_count {#get_polygon_count}

```gdscript
int get_polygon_count() const
```

Returns the count of all polygons.

### get_sample_partition_type {#get_sample_partition_type}

```gdscript
NavigationPolygon.SamplePartitionType get_sample_partition_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_source_geometry_group_name {#get_source_geometry_group_name}

```gdscript
StringName get_source_geometry_group_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_source_geometry_mode {#get_source_geometry_mode}

```gdscript
NavigationPolygon.SourceGeometryMode get_source_geometry_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertices {#get_vertices}

```gdscript
PackedVector2Array get_vertices() const
```

Returns a PackedVector2Array containing all the vertices being used to create the polygons.

### make_polygons_from_outlines {#make_polygons_from_outlines}

```gdscript
void make_polygons_from_outlines()
```

Deprecated: Use NavigationServer2D.parse_source_geometry_data() and NavigationServer2D.bake_from_source_geometry_data() instead.

Creates polygons from the outlines added in the editor or by script.

### remove_outline {#remove_outline}

```gdscript
void remove_outline(idx: int)
```

Removes an outline created in the editor or by script. You have to call make_polygons_from_outlines() for the polygons to update.

### set_agent_radius {#set_agent_radius}

```gdscript
void set_agent_radius(agent_radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_baking_rect {#set_baking_rect}

```gdscript
void set_baking_rect(rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_baking_rect_offset {#set_baking_rect_offset}

```gdscript
void set_baking_rect_offset(rect_offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_border_size {#set_border_size}

```gdscript
void set_border_size(border_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_size {#set_cell_size}

```gdscript
void set_cell_size(cell_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outline {#set_outline}

```gdscript
void set_outline(idx: int, outline: PackedVector2Array)
```

Changes an outline created in the editor or by script. You have to call make_polygons_from_outlines() for the polygons to update.

### set_parsed_collision_mask {#set_parsed_collision_mask}

```gdscript
void set_parsed_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_parsed_collision_mask_value {#set_parsed_collision_mask_value}

```gdscript
void set_parsed_collision_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the parsed_collision_mask, given a layer_number between 1 and 32.

### set_parsed_geometry_type {#set_parsed_geometry_type}

```gdscript
void set_parsed_geometry_type(geometry_type: NavigationPolygon.ParsedGeometryType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sample_partition_type {#set_sample_partition_type}

```gdscript
void set_sample_partition_type(sample_partition_type: NavigationPolygon.SamplePartitionType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source_geometry_group_name {#set_source_geometry_group_name}

```gdscript
void set_source_geometry_group_name(group_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source_geometry_mode {#set_source_geometry_mode}

```gdscript
void set_source_geometry_mode(geometry_mode: NavigationPolygon.SourceGeometryMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertices {#set_vertices}

```gdscript
void set_vertices(vertices: PackedVector2Array)
```

Sets the vertices that can be then indexed to create polygons with the add_polygon() method.

## 属性

### vertices {#vertices}

```gdscript
vertices: PackedVector2Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### polygons {#polygons}

```gdscript
polygons: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### outlines {#outlines}

```gdscript
outlines: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### sample_partition_type {#sample_partition_type}

```gdscript
sample_partition_type: int
```

- void set_sample_partition_type(value: SamplePartitionType)
- SamplePartitionType get_sample_partition_type()

Partitioning algorithm for creating the navigation mesh polys.

### parsed_geometry_type {#parsed_geometry_type}

```gdscript
parsed_geometry_type: int
```

- void set_parsed_geometry_type(value: ParsedGeometryType)
- ParsedGeometryType get_parsed_geometry_type()

Determines which type of nodes will be parsed as geometry.

### parsed_collision_mask {#parsed_collision_mask}

```gdscript
parsed_collision_mask: int
```

- void set_parsed_collision_mask(value: int)
- int get_parsed_collision_mask()

The physics layers to scan for static colliders.

Only used when parsed_geometry_type is PARSED_GEOMETRY_STATIC_COLLIDERS or PARSED_GEOMETRY_BOTH.

### source_geometry_mode {#source_geometry_mode}

```gdscript
source_geometry_mode: int
```

- void set_source_geometry_mode(value: SourceGeometryMode)
- SourceGeometryMode get_source_geometry_mode()

The source of the geometry used when baking.

### source_geometry_group_name {#source_geometry_group_name}

```gdscript
source_geometry_group_name: String
```

- void set_source_geometry_group_name(value: StringName)
- StringName get_source_geometry_group_name()

The group name of nodes that should be parsed for baking source geometry.

Only used when source_geometry_mode is SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN or SOURCE_GEOMETRY_GROUPS_EXPLICIT.

### cell_size {#cell_size}

```gdscript
cell_size: float
```

- void set_cell_size(value: float)
- float get_cell_size()

The cell size used to rasterize the navigation mesh vertices. Must match with the cell size on the navigation map.

### border_size {#border_size}

```gdscript
border_size: float
```

- void set_border_size(value: float)
- float get_border_size()

The size of the non-navigable border around the bake bounding area defined by the baking_rect Rect2.

In conjunction with the baking_rect the border size can be used to bake tile aligned navigation meshes without the tile edges being shrunk by agent_radius.

### agent_radius {#agent_radius}

```gdscript
agent_radius: float
```

- void set_agent_radius(value: float)
- float get_agent_radius()

The distance to erode/shrink the walkable surface when baking the navigation mesh.

Note: The radius must be equal or higher than 0.0. If the radius is 0.0, it won't be possible to fix invalid outline overlaps and other precision errors during the baking process. As a result, some obstacles may be excluded incorrectly from the final navigation mesh, or may delete the navigation mesh's polygons.

### baking_rect {#baking_rect}

```gdscript
baking_rect: Rect2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### baking_rect_offset {#baking_rect_offset}

```gdscript
baking_rect_offset: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### SamplePartitionType {#enum-SamplePartitionType}

```gdscript
enum SamplePartitionType { SAMPLE_PARTITION_CONVEX_PARTITION = 0, SAMPLE_PARTITION_TRIANGULATE = 1, SAMPLE_PARTITION_MAX = 2 }
```

SamplePartitionType SAMPLE_PARTITION_CONVEX_PARTITION = 0

Convex partitioning that results in a navigation mesh with convex polygons.

SamplePartitionType SAMPLE_PARTITION_TRIANGULATE = 1

Triangulation partitioning that results in a navigation mesh with triangle polygons.

SamplePartitionType SAMPLE_PARTITION_MAX = 2

Represents the size of the SamplePartitionType enum.

#### SAMPLE_PARTITION_CONVEX_PARTITION {#SAMPLE_PARTITION_CONVEX_PARTITION}

```gdscript
SAMPLE_PARTITION_CONVEX_PARTITION = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SAMPLE_PARTITION_TRIANGULATE {#SAMPLE_PARTITION_TRIANGULATE}

```gdscript
SAMPLE_PARTITION_TRIANGULATE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SAMPLE_PARTITION_MAX {#SAMPLE_PARTITION_MAX}

```gdscript
SAMPLE_PARTITION_MAX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ParsedGeometryType {#enum-ParsedGeometryType}

```gdscript
enum ParsedGeometryType { PARSED_GEOMETRY_MESH_INSTANCES = 0, PARSED_GEOMETRY_STATIC_COLLIDERS = 1, PARSED_GEOMETRY_BOTH = 2, PARSED_GEOMETRY_MAX = 3 }
```

ParsedGeometryType PARSED_GEOMETRY_MESH_INSTANCES = 0

Parses mesh instances as obstruction geometry. This includes Polygon2D, MeshInstance2D, MultiMeshInstance2D, and TileMap nodes.

Meshes are only parsed when they use a 2D vertices surface format.

ParsedGeometryType PARSED_GEOMETRY_STATIC_COLLIDERS = 1

Parses StaticBody2D and TileMap colliders as obstruction geometry. The collider should be in any of the layers specified by parsed_collision_mask.

ParsedGeometryType PARSED_GEOMETRY_BOTH = 2

Both PARSED_GEOMETRY_MESH_INSTANCES and PARSED_GEOMETRY_STATIC_COLLIDERS.

ParsedGeometryType PARSED_GEOMETRY_MAX = 3

Represents the size of the ParsedGeometryType enum.

#### PARSED_GEOMETRY_MESH_INSTANCES {#PARSED_GEOMETRY_MESH_INSTANCES}

```gdscript
PARSED_GEOMETRY_MESH_INSTANCES = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARSED_GEOMETRY_STATIC_COLLIDERS {#PARSED_GEOMETRY_STATIC_COLLIDERS}

```gdscript
PARSED_GEOMETRY_STATIC_COLLIDERS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARSED_GEOMETRY_BOTH {#PARSED_GEOMETRY_BOTH}

```gdscript
PARSED_GEOMETRY_BOTH = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARSED_GEOMETRY_MAX {#PARSED_GEOMETRY_MAX}

```gdscript
PARSED_GEOMETRY_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### SourceGeometryMode {#enum-SourceGeometryMode}

```gdscript
enum SourceGeometryMode { SOURCE_GEOMETRY_ROOT_NODE_CHILDREN = 0, SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN = 1, SOURCE_GEOMETRY_GROUPS_EXPLICIT = 2, SOURCE_GEOMETRY_MAX = 3 }
```

SourceGeometryMode SOURCE_GEOMETRY_ROOT_NODE_CHILDREN = 0

Scans the child nodes of the root node recursively for geometry.

SourceGeometryMode SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN = 1

Scans nodes in a group and their child nodes recursively for geometry. The group is specified by source_geometry_group_name.

SourceGeometryMode SOURCE_GEOMETRY_GROUPS_EXPLICIT = 2

Uses nodes in a group for geometry. The group is specified by source_geometry_group_name.

SourceGeometryMode SOURCE_GEOMETRY_MAX = 3

Represents the size of the SourceGeometryMode enum.

#### SOURCE_GEOMETRY_ROOT_NODE_CHILDREN {#SOURCE_GEOMETRY_ROOT_NODE_CHILDREN}

```gdscript
SOURCE_GEOMETRY_ROOT_NODE_CHILDREN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN {#SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN}

```gdscript
SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_GEOMETRY_GROUPS_EXPLICIT {#SOURCE_GEOMETRY_GROUPS_EXPLICIT}

```gdscript
SOURCE_GEOMETRY_GROUPS_EXPLICIT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_GEOMETRY_MAX {#SOURCE_GEOMETRY_MAX}

```gdscript
SOURCE_GEOMETRY_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
