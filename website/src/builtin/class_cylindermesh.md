# CylinderMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CylinderMesh**

> Class representing a cylindrical PrimitiveMesh. This class can be used to create cones by setting either the top_radius or bottom_radius properties to 0.0.

<details><summary>官方英文描述（中文精译进行中）</summary>

Class representing a cylindrical PrimitiveMesh. This class can be used to create cones by setting either the top_radius or bottom_radius properties to 0.0.

</details>

## 方法

### get_bottom_radius {#get_bottom_radius}

```gdscript
float get_bottom_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_height {#get_height}

```gdscript
float get_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radial_segments {#get_radial_segments}

```gdscript
int get_radial_segments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rings {#get_rings}

```gdscript
int get_rings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_top_radius {#get_top_radius}

```gdscript
float get_top_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_cap_bottom {#is_cap_bottom}

```gdscript
bool is_cap_bottom() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_cap_top {#is_cap_top}

```gdscript
bool is_cap_top() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bottom_radius {#set_bottom_radius}

```gdscript
void set_bottom_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cap_bottom {#set_cap_bottom}

```gdscript
void set_cap_bottom(cap_bottom: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cap_top {#set_cap_top}

```gdscript
void set_cap_top(cap_top: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radial_segments {#set_radial_segments}

```gdscript
void set_radial_segments(segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rings {#set_rings}

```gdscript
void set_rings(rings: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_top_radius {#set_top_radius}

```gdscript
void set_top_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### top_radius {#top_radius}

```gdscript
top_radius: float
```

- void set_top_radius(value: float)
- float get_top_radius()

Top radius of the cylinder. If set to 0.0, the top faces will not be generated, resulting in a conic shape. See also cap_top.

### bottom_radius {#bottom_radius}

```gdscript
bottom_radius: float
```

- void set_bottom_radius(value: float)
- float get_bottom_radius()

Bottom radius of the cylinder. If set to 0.0, the bottom faces will not be generated, resulting in a conic shape. See also cap_bottom.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

Full height of the cylinder.

### radial_segments {#radial_segments}

```gdscript
radial_segments: int
```

- void set_radial_segments(value: int)
- int get_radial_segments()

Number of radial segments on the cylinder. Higher values result in a more detailed cylinder/cone at the cost of performance.

### rings {#rings}

```gdscript
rings: int
```

- void set_rings(value: int)
- int get_rings()

Number of edge rings along the height of the cylinder. Changing rings does not have any visual impact unless a shader or procedural mesh tool is used to alter the vertex data. Higher values result in more subdivisions, which can be used to create smoother-looking effects with shaders or procedural mesh tools (at the cost of performance). When not altering the vertex data using a shader or procedural mesh tool, rings should be kept to its default value.

### cap_top {#cap_top}

```gdscript
cap_top: bool
```

- void set_cap_top(value: bool)
- bool is_cap_top()

If true, generates a cap at the top of the cylinder. This can be set to false to speed up generation and rendering when the cap is never seen by the camera. See also top_radius.

Note: If top_radius is 0.0, cap generation is always skipped even if cap_top is true.

### cap_bottom {#cap_bottom}

```gdscript
cap_bottom: bool
```

- void set_cap_bottom(value: bool)
- bool is_cap_bottom()

If true, generates a cap at the bottom of the cylinder. This can be set to false to speed up generation and rendering when the cap is never seen by the camera. See also bottom_radius.

Note: If bottom_radius is 0.0, cap generation is always skipped even if cap_bottom is true.
