# NavigationMesh

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **NavigationMesh**

> A navigation mesh is a collection of polygons that define which areas of an environment are traversable to aid agents in pathfinding through complicated spaces.

<details><summary>官方英文描述（中文精译进行中）</summary>

A navigation mesh is a collection of polygons that define which areas of an environment are traversable to aid agents in pathfinding through complicated spaces.

</details>

## 方法

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

### clear_polygons {#clear_polygons}

```gdscript
void clear_polygons()
```

Clears the array of polygons, but it doesn't clear the array of vertices.

### create_from_mesh {#create_from_mesh}

```gdscript
void create_from_mesh(mesh: Mesh)
```

Initializes the navigation mesh by setting the vertices and indices according to a Mesh.

Note: The given mesh must be of type Mesh.PRIMITIVE_TRIANGLES and have an index array.

### get_agent_height {#get_agent_height}

```gdscript
float get_agent_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_agent_max_climb {#get_agent_max_climb}

```gdscript
float get_agent_max_climb() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_agent_max_slope {#get_agent_max_slope}

```gdscript
float get_agent_max_slope() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_agent_radius {#get_agent_radius}

```gdscript
float get_agent_radius()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_border_size {#get_border_size}

```gdscript
float get_border_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cell_height {#get_cell_height}

```gdscript
float get_cell_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cell_size {#get_cell_size}

```gdscript
float get_cell_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_mask_value {#get_collision_mask_value}

```gdscript
bool get_collision_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the geometry_collision_mask is enabled, given a layer_number between 1 and 32.

### get_detail_sample_distance {#get_detail_sample_distance}

```gdscript
float get_detail_sample_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_detail_sample_max_error {#get_detail_sample_max_error}

```gdscript
float get_detail_sample_max_error() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_edge_max_error {#get_edge_max_error}

```gdscript
float get_edge_max_error() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_edge_max_length {#get_edge_max_length}

```gdscript
float get_edge_max_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filter_baking_aabb {#get_filter_baking_aabb}

```gdscript
AABB get_filter_baking_aabb() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filter_baking_aabb_offset {#get_filter_baking_aabb_offset}

```gdscript
Vector3 get_filter_baking_aabb_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filter_ledge_spans {#get_filter_ledge_spans}

```gdscript
bool get_filter_ledge_spans() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filter_low_hanging_obstacles {#get_filter_low_hanging_obstacles}

```gdscript
bool get_filter_low_hanging_obstacles() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filter_walkable_low_height_spans {#get_filter_walkable_low_height_spans}

```gdscript
bool get_filter_walkable_low_height_spans() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parsed_geometry_type {#get_parsed_geometry_type}

```gdscript
NavigationMesh.ParsedGeometryType get_parsed_geometry_type() const
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

Returns the number of polygons in the navigation mesh.

### get_region_merge_size {#get_region_merge_size}

```gdscript
float get_region_merge_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_region_min_size {#get_region_min_size}

```gdscript
float get_region_min_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sample_partition_type {#get_sample_partition_type}

```gdscript
NavigationMesh.SamplePartitionType get_sample_partition_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_source_geometry_mode {#get_source_geometry_mode}

```gdscript
NavigationMesh.SourceGeometryMode get_source_geometry_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_source_group_name {#get_source_group_name}

```gdscript
StringName get_source_group_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertices {#get_vertices}

```gdscript
PackedVector3Array get_vertices() const
```

Returns a PackedVector3Array containing all the vertices being used to create the polygons.

### get_vertices_per_polygon {#get_vertices_per_polygon}

```gdscript
float get_vertices_per_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_agent_height {#set_agent_height}

```gdscript
void set_agent_height(agent_height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_agent_max_climb {#set_agent_max_climb}

```gdscript
void set_agent_max_climb(agent_max_climb: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_agent_max_slope {#set_agent_max_slope}

```gdscript
void set_agent_max_slope(agent_max_slope: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_agent_radius {#set_agent_radius}

```gdscript
void set_agent_radius(agent_radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_border_size {#set_border_size}

```gdscript
void set_border_size(border_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_height {#set_cell_height}

```gdscript
void set_cell_height(cell_height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_size {#set_cell_size}

```gdscript
void set_cell_size(cell_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask_value {#set_collision_mask_value}

```gdscript
void set_collision_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the geometry_collision_mask, given a layer_number between 1 and 32.

### set_detail_sample_distance {#set_detail_sample_distance}

```gdscript
void set_detail_sample_distance(detail_sample_dist: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_detail_sample_max_error {#set_detail_sample_max_error}

```gdscript
void set_detail_sample_max_error(detail_sample_max_error: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edge_max_error {#set_edge_max_error}

```gdscript
void set_edge_max_error(edge_max_error: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edge_max_length {#set_edge_max_length}

```gdscript
void set_edge_max_length(edge_max_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filter_baking_aabb {#set_filter_baking_aabb}

```gdscript
void set_filter_baking_aabb(baking_aabb: AABB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filter_baking_aabb_offset {#set_filter_baking_aabb_offset}

```gdscript
void set_filter_baking_aabb_offset(baking_aabb_offset: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filter_ledge_spans {#set_filter_ledge_spans}

```gdscript
void set_filter_ledge_spans(filter_ledge_spans: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filter_low_hanging_obstacles {#set_filter_low_hanging_obstacles}

```gdscript
void set_filter_low_hanging_obstacles(filter_low_hanging_obstacles: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filter_walkable_low_height_spans {#set_filter_walkable_low_height_spans}

```gdscript
void set_filter_walkable_low_height_spans(filter_walkable_low_height_spans: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_parsed_geometry_type {#set_parsed_geometry_type}

```gdscript
void set_parsed_geometry_type(geometry_type: NavigationMesh.ParsedGeometryType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region_merge_size {#set_region_merge_size}

```gdscript
void set_region_merge_size(region_merge_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region_min_size {#set_region_min_size}

```gdscript
void set_region_min_size(region_min_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sample_partition_type {#set_sample_partition_type}

```gdscript
void set_sample_partition_type(sample_partition_type: NavigationMesh.SamplePartitionType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source_geometry_mode {#set_source_geometry_mode}

```gdscript
void set_source_geometry_mode(mask: NavigationMesh.SourceGeometryMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source_group_name {#set_source_group_name}

```gdscript
void set_source_group_name(mask: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertices {#set_vertices}

```gdscript
void set_vertices(vertices: PackedVector3Array)
```

Sets the vertices that can be then indexed to create polygons with the add_polygon() method.

### set_vertices_per_polygon {#set_vertices_per_polygon}

```gdscript
void set_vertices_per_polygon(vertices_per_polygon: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### vertices {#vertices}

```gdscript
vertices: PackedVector3Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### polygons {#polygons}

```gdscript
polygons: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### sample_partition_type {#sample_partition_type}

```gdscript
sample_partition_type: int
```

- void set_sample_partition_type(value: SamplePartitionType)
- SamplePartitionType get_sample_partition_type()

Partitioning algorithm for creating the navigation mesh polys.

### geometry_parsed_geometry_type {#geometry_parsed_geometry_type}

```gdscript
geometry_parsed_geometry_type: int
```

- void set_parsed_geometry_type(value: ParsedGeometryType)
- ParsedGeometryType get_parsed_geometry_type()

Determines which type of nodes will be parsed as geometry.

### geometry_collision_mask {#geometry_collision_mask}

```gdscript
geometry_collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The physics layers to scan for static colliders.

Only used when geometry_parsed_geometry_type is PARSED_GEOMETRY_STATIC_COLLIDERS or PARSED_GEOMETRY_BOTH.

### geometry_source_geometry_mode {#geometry_source_geometry_mode}

```gdscript
geometry_source_geometry_mode: int
```

- void set_source_geometry_mode(value: SourceGeometryMode)
- SourceGeometryMode get_source_geometry_mode()

The source of the geometry used when baking.

### geometry_source_group_name {#geometry_source_group_name}

```gdscript
geometry_source_group_name: String
```

- void set_source_group_name(value: StringName)
- StringName get_source_group_name()

The name of the group to scan for geometry.

Only used when geometry_source_geometry_mode is SOURCE_GEOMETRY_GROUPS_WITH_CHILDREN or SOURCE_GEOMETRY_GROUPS_EXPLICIT.

### cell_size {#cell_size}

```gdscript
cell_size: float
```

- void set_cell_size(value: float)
- float get_cell_size()

The cell size used to rasterize the navigation mesh vertices on the XZ plane. Must match with the cell size on the navigation map.

### cell_height {#cell_height}

```gdscript
cell_height: float
```

- void set_cell_height(value: float)
- float get_cell_height()

The cell height used to rasterize the navigation mesh vertices on the Y axis. Must match with the cell height on the navigation map.

### border_size {#border_size}

```gdscript
border_size: float
```

- void set_border_size(value: float)
- float get_border_size()

The size of the non-navigable border around the bake bounding area.

In conjunction with the filter_baking_aabb and a edge_max_error value at 1.0 or below the border size can be used to bake tile aligned navigation meshes without the tile edges being shrunk by agent_radius.

Note: If this value is not 0.0, it will be rounded up to the nearest multiple of cell_size during baking.

### agent_height {#agent_height}

```gdscript
agent_height: float
```

- void set_agent_height(value: float)
- float get_agent_height()

The minimum floor to ceiling height that will still allow the floor area to be considered walkable.

Note: While baking, this value will be rounded up to the nearest multiple of cell_height.

### agent_radius {#agent_radius}

```gdscript
agent_radius: float
```

- void set_agent_radius(value: float)
- float get_agent_radius()

The distance to erode/shrink the walkable area of the heightfield away from obstructions.

Note: While baking, this value will be rounded up to the nearest multiple of cell_size.

Note: The radius must be equal or higher than 0.0. If the radius is 0.0, it won't be possible to fix invalid outline overlaps and other precision errors during the baking process. As a result, some obstacles may be excluded incorrectly from the final navigation mesh, or may delete the navigation mesh's polygons.

### agent_max_climb {#agent_max_climb}

```gdscript
agent_max_climb: float
```

- void set_agent_max_climb(value: float)
- float get_agent_max_climb()

The minimum ledge height that is considered to still be traversable.

Note: While baking, this value will be rounded down to the nearest multiple of cell_height.

### agent_max_slope {#agent_max_slope}

```gdscript
agent_max_slope: float
```

- void set_agent_max_slope(value: float)
- float get_agent_max_slope()

The maximum slope that is considered walkable, in degrees.

### region_min_size {#region_min_size}

```gdscript
region_min_size: float
```

- void set_region_min_size(value: float)
- float get_region_min_size()

The minimum size of a region for it to be created.

Note: This value will be squared to calculate the minimum number of cells allowed to form isolated island areas. For example, a value of 8 will set the number of cells to 64.

### region_merge_size {#region_merge_size}

```gdscript
region_merge_size: float
```

- void set_region_merge_size(value: float)
- float get_region_merge_size()

Any regions with a size smaller than this will be merged with larger regions if possible.

Note: This value will be squared to calculate the number of cells. For example, a value of 20 will set the number of cells to 400.

### edge_max_length {#edge_max_length}

```gdscript
edge_max_length: float
```

- void set_edge_max_length(value: float)
- float get_edge_max_length()

The maximum allowed length for contour edges along the border of the mesh. A value of 0.0 disables this feature.

Note: While baking, this value will be rounded up to the nearest multiple of cell_size.

### edge_max_error {#edge_max_error}

```gdscript
edge_max_error: float
```

- void set_edge_max_error(value: float)
- float get_edge_max_error()

The maximum distance a simplified contour's border edges should deviate the original raw contour.

### vertices_per_polygon {#vertices_per_polygon}

```gdscript
vertices_per_polygon: float
```

- void set_vertices_per_polygon(value: float)
- float get_vertices_per_polygon()

The maximum number of vertices allowed for polygons generated during the contour to polygon conversion process.

### detail_sample_distance {#detail_sample_distance}

```gdscript
detail_sample_distance: float
```

- void set_detail_sample_distance(value: float)
- float get_detail_sample_distance()

The sampling distance to use when generating the detail mesh, in cell unit.

### detail_sample_max_error {#detail_sample_max_error}

```gdscript
detail_sample_max_error: float
```

- void set_detail_sample_max_error(value: float)
- float get_detail_sample_max_error()

The maximum distance the detail mesh surface should deviate from heightfield, in cell unit.

### filter_low_hanging_obstacles {#filter_low_hanging_obstacles}

```gdscript
filter_low_hanging_obstacles: bool
```

- void set_filter_low_hanging_obstacles(value: bool)
- bool get_filter_low_hanging_obstacles()

If true, marks non-walkable spans as walkable if their maximum is within agent_max_climb of a walkable neighbor.

### filter_ledge_spans {#filter_ledge_spans}

```gdscript
filter_ledge_spans: bool
```

- void set_filter_ledge_spans(value: bool)
- bool get_filter_ledge_spans()

If true, marks spans that are ledges as non-walkable.

### filter_walkable_low_height_spans {#filter_walkable_low_height_spans}

```gdscript
filter_walkable_low_height_spans: bool
```

- void set_filter_walkable_low_height_spans(value: bool)
- bool get_filter_walkable_low_height_spans()

If true, marks walkable spans as not walkable if the clearance above the span is less than agent_height.

### filter_baking_aabb {#filter_baking_aabb}

```gdscript
filter_baking_aabb: AABB
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### filter_baking_aabb_offset {#filter_baking_aabb_offset}

```gdscript
filter_baking_aabb_offset: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### SamplePartitionType {#enum-SamplePartitionType}

```gdscript
enum SamplePartitionType { SAMPLE_PARTITION_WATERSHED = 0, SAMPLE_PARTITION_MONOTONE = 1, SAMPLE_PARTITION_LAYERS = 2, SAMPLE_PARTITION_MAX = 3 }
```

SamplePartitionType SAMPLE_PARTITION_WATERSHED = 0

Watershed partitioning. Generally the best choice if you precompute the navigation mesh, use this if you have large open areas.

SamplePartitionType SAMPLE_PARTITION_MONOTONE = 1

Monotone partitioning. Use this if you want fast navigation mesh generation.

SamplePartitionType SAMPLE_PARTITION_LAYERS = 2

Layer partitioning. Good choice to use for tiled navigation mesh with medium and small sized tiles.

SamplePartitionType SAMPLE_PARTITION_MAX = 3

Represents the size of the SamplePartitionType enum.

#### SAMPLE_PARTITION_WATERSHED {#SAMPLE_PARTITION_WATERSHED}

```gdscript
SAMPLE_PARTITION_WATERSHED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SAMPLE_PARTITION_MONOTONE {#SAMPLE_PARTITION_MONOTONE}

```gdscript
SAMPLE_PARTITION_MONOTONE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SAMPLE_PARTITION_LAYERS {#SAMPLE_PARTITION_LAYERS}

```gdscript
SAMPLE_PARTITION_LAYERS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SAMPLE_PARTITION_MAX {#SAMPLE_PARTITION_MAX}

```gdscript
SAMPLE_PARTITION_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ParsedGeometryType {#enum-ParsedGeometryType}

```gdscript
enum ParsedGeometryType { PARSED_GEOMETRY_MESH_INSTANCES = 0, PARSED_GEOMETRY_STATIC_COLLIDERS = 1, PARSED_GEOMETRY_BOTH = 2, PARSED_GEOMETRY_MAX = 3 }
```

ParsedGeometryType PARSED_GEOMETRY_MESH_INSTANCES = 0

Parses mesh instances as geometry. This includes MeshInstance3D, CSGShape3D, and GridMap nodes.

ParsedGeometryType PARSED_GEOMETRY_STATIC_COLLIDERS = 1

Parses StaticBody3D colliders as geometry. The collider should be in any of the layers specified by geometry_collision_mask.

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

Scans nodes in a group and their child nodes recursively for geometry. The group is specified by geometry_source_group_name.

SourceGeometryMode SOURCE_GEOMETRY_GROUPS_EXPLICIT = 2

Uses nodes in a group for geometry. The group is specified by geometry_source_group_name.

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
