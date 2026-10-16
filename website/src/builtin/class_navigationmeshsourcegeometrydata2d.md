# NavigationMeshSourceGeometryData2D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **NavigationMeshSourceGeometryData2D**

> Container for parsed source geometry data used in navigation mesh baking.

<details><summary>官方英文描述</summary>

Container for parsed source geometry data used in navigation mesh baking.

</details>

## 方法

### add_obstruction_outline {#add_obstruction_outline}

```gdscript
void add_obstruction_outline(shape_outline: PackedVector2Array)
```

Adds the outline points of a shape as obstructed area.

### add_projected_obstruction {#add_projected_obstruction}

```gdscript
void add_projected_obstruction(vertices: PackedVector2Array, carve: bool)
```

Adds a projected obstruction shape to the source geometry. If carve is true the carved shape will not be affected by additional offsets (e.g. agent radius) of the navigation mesh baking process.

### add_traversable_outline {#add_traversable_outline}

```gdscript
void add_traversable_outline(shape_outline: PackedVector2Array)
```

Adds the outline points of a shape as traversable area.

### append_obstruction_outlines {#append_obstruction_outlines}

```gdscript
void append_obstruction_outlines(obstruction_outlines: typedarray::PackedVector2Array)
```

Appends another array of obstruction_outlines at the end of the existing obstruction outlines array.

### append_traversable_outlines {#append_traversable_outlines}

```gdscript
void append_traversable_outlines(traversable_outlines: typedarray::PackedVector2Array)
```

Appends another array of traversable_outlines at the end of the existing traversable outlines array.

### clear {#clear}

```gdscript
void clear()
```

Clears the internal data.

### clear_projected_obstructions {#clear_projected_obstructions}

```gdscript
void clear_projected_obstructions()
```

Clears all projected obstructions.

### get_bounds {#get_bounds}

```gdscript
Rect2 get_bounds()
```

Returns an axis-aligned bounding box that covers all the stored geometry data. The bounds are calculated when calling this function with the result cached until further geometry changes are made.

### get_obstruction_outlines {#get_obstruction_outlines}

```gdscript
typedarray::PackedVector2Array get_obstruction_outlines() const
```

Returns all the obstructed area outlines arrays.

### get_projected_obstructions {#get_projected_obstructions}

```gdscript
Array get_projected_obstructions() const
```

Returns the projected obstructions as an Array of dictionaries. Each Dictionary contains the following entries:

- vertices - A PackedFloat32Array that defines the outline points of the projected shape.
- carve - A bool that defines how the projected shape affects the navigation mesh baking. If true the projected shape will not be affected by addition offsets, e.g. agent radius.

### get_traversable_outlines {#get_traversable_outlines}

```gdscript
typedarray::PackedVector2Array get_traversable_outlines() const
```

Returns all the traversable area outlines arrays.

### has_data {#has_data}

```gdscript
bool has_data()
```

Returns true when parsed source geometry data exists.

### merge {#merge}

```gdscript
void merge(other_geometry: NavigationMeshSourceGeometryData2D)
```

Adds the geometry data of another NavigationMeshSourceGeometryData2D to the navigation mesh baking data.

### set_obstruction_outlines {#set_obstruction_outlines}

```gdscript
void set_obstruction_outlines(obstruction_outlines: typedarray::PackedVector2Array)
```

Sets all the obstructed area outlines arrays.

### set_projected_obstructions {#set_projected_obstructions}

```gdscript
void set_projected_obstructions(projected_obstructions: Array)
```

Sets the projected obstructions with an Array of Dictionaries with the following key value pairs:

```
"vertices" : PackedFloat32Array
"carve" : bool
```

### set_traversable_outlines {#set_traversable_outlines}

```gdscript
void set_traversable_outlines(traversable_outlines: typedarray::PackedVector2Array)
```

Sets all the traversable area outlines arrays.

## 属性

### traversable_outlines {#traversable_outlines}

```gdscript
traversable_outlines: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### obstruction_outlines {#obstruction_outlines}

```gdscript
obstruction_outlines: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### projected_obstructions {#projected_obstructions}

```gdscript
projected_obstructions: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
