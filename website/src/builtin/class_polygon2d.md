# Polygon2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Polygon2D**

> A Polygon2D is defined by a set of points. Each point is connected to the next, with the final point being connected to the first, resulting in a closed polygon. Polygon2Ds can be filled with color (solid or gradient) or filled with a given texture.

<details><summary>官方英文描述</summary>

A Polygon2D is defined by a set of points. Each point is connected to the next, with the final point being connected to the first, resulting in a closed polygon. Polygon2Ds can be filled with color (solid or gradient) or filled with a given texture.

</details>

## 方法

### add_bone {#add_bone}

```gdscript
void add_bone(path: NodePath, weights: PackedFloat32Array)
```

Adds a bone with the specified path and weights.

### clear_bones {#clear_bones}

```gdscript
void clear_bones()
```

Removes all bones from this Polygon2D.

### erase_bone {#erase_bone}

```gdscript
void erase_bone(index: int)
```

Removes the specified bone from this Polygon2D.

### get_antialiased {#get_antialiased}

```gdscript
bool get_antialiased() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_count {#get_bone_count}

```gdscript
int get_bone_count() const
```

Returns the number of bones in this Polygon2D.

### get_bone_path {#get_bone_path}

```gdscript
NodePath get_bone_path(index: int) const
```

Returns the path to the node associated with the specified bone.

### get_bone_weights {#get_bone_weights}

```gdscript
PackedFloat32Array get_bone_weights(index: int) const
```

Returns the weight values of the specified bone.

### get_color {#get_color}

```gdscript
Color get_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_internal_vertex_count {#get_internal_vertex_count}

```gdscript
int get_internal_vertex_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_invert_border {#get_invert_border}

```gdscript
float get_invert_border() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_invert_enabled {#get_invert_enabled}

```gdscript
bool get_invert_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_polygon {#get_polygon}

```gdscript
PackedVector2Array get_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_polygons {#get_polygons}

```gdscript
Array get_polygons() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton {#get_skeleton}

```gdscript
NodePath get_skeleton() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_offset {#get_texture_offset}

```gdscript
Vector2 get_texture_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_rotation {#get_texture_rotation}

```gdscript
float get_texture_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_scale {#get_texture_scale}

```gdscript
Vector2 get_texture_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_uv {#get_uv}

```gdscript
PackedVector2Array get_uv() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_colors {#get_vertex_colors}

```gdscript
PackedColorArray get_vertex_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_antialiased {#set_antialiased}

```gdscript
void set_antialiased(antialiased: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_path {#set_bone_path}

```gdscript
void set_bone_path(index: int, path: NodePath)
```

Sets the path to the node associated with the specified bone.

### set_bone_weights {#set_bone_weights}

```gdscript
void set_bone_weights(index: int, weights: PackedFloat32Array)
```

Sets the weight values for the specified bone.

### set_color {#set_color}

```gdscript
void set_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_internal_vertex_count {#set_internal_vertex_count}

```gdscript
void set_internal_vertex_count(internal_vertex_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_invert_border {#set_invert_border}

```gdscript
void set_invert_border(invert_border: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_invert_enabled {#set_invert_enabled}

```gdscript
void set_invert_enabled(invert: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polygon {#set_polygon}

```gdscript
void set_polygon(polygon: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polygons {#set_polygons}

```gdscript
void set_polygons(polygons: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skeleton {#set_skeleton}

```gdscript
void set_skeleton(skeleton: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_offset {#set_texture_offset}

```gdscript
void set_texture_offset(texture_offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_rotation {#set_texture_rotation}

```gdscript
void set_texture_rotation(texture_rotation: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_scale {#set_texture_scale}

```gdscript
void set_texture_scale(texture_scale: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_uv {#set_uv}

```gdscript
void set_uv(uv: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_colors {#set_vertex_colors}

```gdscript
void set_vertex_colors(vertex_colors: PackedColorArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### color {#color}

```gdscript
color: Color
```

- void set_color(value: Color)
- Color get_color()

The polygon's fill color. If texture is set, it will be multiplied by this color. It will also be the default color for vertices not set in vertex_colors.

### offset {#offset}

```gdscript
offset: Vector2
```

- void set_offset(value: Vector2)
- Vector2 get_offset()

The offset applied to each vertex.

### antialiased {#antialiased}

```gdscript
antialiased: bool
```

- void set_antialiased(value: bool)
- bool get_antialiased()

If true, polygon edges will be anti-aliased.

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The polygon's fill texture. Use uv to set texture coordinates.

### texture_offset {#texture_offset}

```gdscript
texture_offset: Vector2
```

- void set_texture_offset(value: Vector2)
- Vector2 get_texture_offset()

Amount to offset the polygon's texture. If set to Vector2(0, 0), the texture's origin (its top-left corner) will be placed at the polygon's position.

### texture_scale {#texture_scale}

```gdscript
texture_scale: Vector2
```

- void set_texture_scale(value: Vector2)
- Vector2 get_texture_scale()

Amount to multiply the uv coordinates when using texture. Larger values make the texture smaller, and vice versa.

### texture_rotation {#texture_rotation}

```gdscript
texture_rotation: float
```

- void set_texture_rotation(value: float)
- float get_texture_rotation()

The texture's rotation in radians.

### skeleton {#skeleton}

```gdscript
skeleton: NodePath
```

- void set_skeleton(value: NodePath)
- NodePath get_skeleton()

Path to a Skeleton2D node used for skeleton-based deformations of this polygon. If empty or invalid, skeletal deformations will not be used.

### invert_enabled {#invert_enabled}

```gdscript
invert_enabled: bool
```

- void set_invert_enabled(value: bool)
- bool get_invert_enabled()

If true, the polygon will be inverted, containing the area outside the defined points and extending to the invert_border.

### invert_border {#invert_border}

```gdscript
invert_border: float
```

- void set_invert_border(value: float)
- float get_invert_border()

Added padding applied to the bounding box when invert_enabled is set to true. Setting this value too small may result in a "Bad Polygon" error.

### polygon {#polygon}

```gdscript
polygon: PackedVector2Array
```

- void set_polygon(value: PackedVector2Array)
- PackedVector2Array get_polygon()

The polygon's list of vertices. The final point will be connected to the first.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.

### uv {#uv}

```gdscript
uv: PackedVector2Array
```

- void set_uv(value: PackedVector2Array)
- PackedVector2Array get_uv()

Texture coordinates for each vertex of the polygon. There should be one UV value per polygon vertex. If there are fewer, undefined vertices will use Vector2(0, 0).

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.

### vertex_colors {#vertex_colors}

```gdscript
vertex_colors: PackedColorArray
```

- void set_vertex_colors(value: PackedColorArray)
- PackedColorArray get_vertex_colors()

Color for each vertex. Colors are interpolated between vertices, resulting in smooth gradients. There should be one per polygon vertex. If there are fewer, undefined vertices will use color.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedColorArray for more details.

### polygons {#polygons}

```gdscript
polygons: Array
```

- void set_polygons(value: Array)
- Array get_polygons()

The list of polygons, in case more than one is being represented. Every individual polygon is stored as a PackedInt32Array where each int is an index to a point in polygon. If empty, this property will be ignored, and the resulting single polygon will be composed of all points in polygon, using the order they are stored in.

### bones {#bones}

```gdscript
bones: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### internal_vertex_count {#internal_vertex_count}

```gdscript
internal_vertex_count: int
```

- void set_internal_vertex_count(value: int)
- int get_internal_vertex_count()

Number of internal vertices, used for UV mapping.
