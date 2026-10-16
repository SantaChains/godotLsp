# VoxelGIData

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VoxelGIData**

> VoxelGIData contains baked voxel global illumination for use in a VoxelGI node. VoxelGIData also offers several properties to adjust the final appearance of the global illumination. These properties can be adjusted at run-time without having to bake the VoxelGI node again.

<details><summary>官方英文描述（中文精译进行中）</summary>

VoxelGIData contains baked voxel global illumination for use in a VoxelGI node. VoxelGIData also offers several properties to adjust the final appearance of the global illumination. These properties can be adjusted at run-time without having to bake the VoxelGI node again.

Note: To prevent text-based scene files (.tscn) from growing too much and becoming slow to load and save, always save VoxelGIData to an external binary resource file (.res) instead of embedding it within the scene. This can be done by clicking the dropdown arrow next to the VoxelGIData resource, choosing Edit, clicking the floppy disk icon at the top of the Inspector then choosing Save As....

</details>

## 方法

### allocate {#allocate}

```gdscript
void allocate(to_cell_xform: Transform3D, aabb: AABB, octree_size: Vector3, octree_cells: PackedByteArray, data_cells: PackedByteArray, distance_field: PackedByteArray, level_counts: PackedInt32Array)
```

Initializes this VoxelGIData with the specified data. octree_cells must be a multiple of 32. octree_cells must be double the size of data_cells. The allocated data can be retrieved later using the various getter methods.

### get_bias {#get_bias}

```gdscript
float get_bias() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bounds {#get_bounds}

```gdscript
AABB get_bounds() const
```

Returns the bounds of the baked voxel data as an AABB, which should match VoxelGI.size after being baked (which only contains the size as a Vector3).

Note: If the size was modified without baking the VoxelGI data, then the value of get_bounds() and VoxelGI.size will not match.

### get_data_cells {#get_data_cells}

```gdscript
PackedByteArray get_data_cells() const
```

Returns the baked cell data for this VoxelGIData.

### get_dynamic_range {#get_dynamic_range}

```gdscript
float get_dynamic_range() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_energy {#get_energy}

```gdscript
float get_energy() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_level_counts {#get_level_counts}

```gdscript
PackedInt32Array get_level_counts() const
```

Returns the baked level counts for this VoxelGIData.

### get_normal_bias {#get_normal_bias}

```gdscript
float get_normal_bias() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_octree_cells {#get_octree_cells}

```gdscript
PackedByteArray get_octree_cells() const
```

Returns the baked octree cell data for this VoxelGIData.

### get_octree_size {#get_octree_size}

```gdscript
Vector3 get_octree_size() const
```

Returns the baked octree size for this VoxelGIData, which corresponds to the number of subdivisions per axis. This can be viewed in the editor by hovering the Bake VoxelGI button at the top of the 3D editor viewport when a VoxelGI node is selected and looking at the Subdivisions field in the tooltip.

### get_propagation {#get_propagation}

```gdscript
float get_propagation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_to_cell_xform {#get_to_cell_xform}

```gdscript
Transform3D get_to_cell_xform() const
```

Returns the baked cell transform for this VoxelGIData.

### is_interior {#is_interior}

```gdscript
bool is_interior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_two_bounces {#is_using_two_bounces}

```gdscript
bool is_using_two_bounces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bias {#set_bias}

```gdscript
void set_bias(bias: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dynamic_range {#set_dynamic_range}

```gdscript
void set_dynamic_range(dynamic_range: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_energy {#set_energy}

```gdscript
void set_energy(energy: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interior {#set_interior}

```gdscript
void set_interior(interior: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_normal_bias {#set_normal_bias}

```gdscript
void set_normal_bias(bias: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_propagation {#set_propagation}

```gdscript
void set_propagation(propagation: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_two_bounces {#set_use_two_bounces}

```gdscript
void set_use_two_bounces(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### dynamic_range {#dynamic_range}

```gdscript
dynamic_range: float
```

- void set_dynamic_range(value: float)
- float get_dynamic_range()

The dynamic range to use (1.0 represents a low dynamic range scene brightness). Higher values can be used to provide brighter indirect lighting, at the cost of more visible color banding in dark areas (both in indirect lighting and reflections). To avoid color banding, it's recommended to use the lowest value that does not result in visible light clipping.

### energy {#energy}

```gdscript
energy: float
```

- void set_energy(value: float)
- float get_energy()

The energy of the indirect lighting and reflections produced by the VoxelGI node. Higher values result in brighter indirect lighting. If indirect lighting looks too flat, try decreasing propagation while increasing energy at the same time. See also use_two_bounces which influences the indirect lighting's effective brightness.

### bias {#bias}

```gdscript
bias: float
```

- void set_bias(value: float)
- float get_bias()

The normal bias to use for indirect lighting and reflections. Higher values reduce self-reflections visible in non-rough materials, at the cost of more visible light leaking and flatter-looking indirect lighting. To prioritize hiding self-reflections over lighting quality, set bias to 0.0 and normal_bias to a value between 1.0 and 2.0.

### normal_bias {#normal_bias}

```gdscript
normal_bias: float
```

- void set_normal_bias(value: float)
- float get_normal_bias()

The normal bias to use for indirect lighting and reflections. Higher values reduce self-reflections visible in non-rough materials, at the cost of more visible light leaking and flatter-looking indirect lighting. See also bias. To prioritize hiding self-reflections over lighting quality, set bias to 0.0 and normal_bias to a value between 1.0 and 2.0.

### propagation {#propagation}

```gdscript
propagation: float
```

- void set_propagation(value: float)
- float get_propagation()

The multiplier to use when light bounces off a surface. Higher values result in brighter indirect lighting. If indirect lighting looks too flat, try decreasing propagation while increasing energy at the same time. See also use_two_bounces which influences the indirect lighting's effective brightness.

### use_two_bounces {#use_two_bounces}

```gdscript
use_two_bounces: bool
```

- void set_use_two_bounces(value: bool)
- bool is_using_two_bounces()

If true, performs two bounces of indirect lighting instead of one. This makes indirect lighting look more natural and brighter at a small performance cost. The second bounce is also visible in reflections. If the scene appears too bright after enabling use_two_bounces, adjust propagation and energy.

### interior {#interior}

```gdscript
interior: bool
```

- void set_interior(value: bool)
- bool is_interior()

If true, Environment lighting is ignored by the VoxelGI node. If false, Environment lighting is taken into account by the VoxelGI node. Environment lighting updates in real-time, which means it can be changed without having to bake the VoxelGI node again.
