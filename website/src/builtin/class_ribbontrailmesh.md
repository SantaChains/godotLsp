# RibbonTrailMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **RibbonTrailMesh**

> RibbonTrailMesh represents a straight ribbon-shaped mesh with variable width. The ribbon is composed of a number of flat or cross-shaped sections, each with the same section_length and number of section_segments. A curve is sampled along the total length of the ribbon, meaning that the curve determines the size of the ribbon along its length.

<details><summary>官方英文描述</summary>

RibbonTrailMesh represents a straight ribbon-shaped mesh with variable width. The ribbon is composed of a number of flat or cross-shaped sections, each with the same section_length and number of section_segments. A curve is sampled along the total length of the ribbon, meaning that the curve determines the size of the ribbon along its length.

This primitive mesh is usually used for particle trails.

</details>

## 方法

### get_curve {#get_curve}

```gdscript
Curve get_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_section_length {#get_section_length}

```gdscript
float get_section_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_section_segments {#get_section_segments}

```gdscript
int get_section_segments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sections {#get_sections}

```gdscript
int get_sections() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
RibbonTrailMesh.Shape get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
float get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve {#set_curve}

```gdscript
void set_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_section_length {#set_section_length}

```gdscript
void set_section_length(section_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_section_segments {#set_section_segments}

```gdscript
void set_section_segments(section_segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sections {#set_sections}

```gdscript
void set_sections(sections: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: RibbonTrailMesh.Shape)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### shape {#shape}

```gdscript
shape: int
```

- void set_shape(value: Shape)
- Shape get_shape()

Determines the shape of the ribbon.

### size {#size}

```gdscript
size: float
```

- void set_size(value: float)
- float get_size()

The baseline size of the ribbon. The size of a particular section segment is obtained by multiplying this size by the value of the curve at the given distance.

### sections {#sections}

```gdscript
sections: int
```

- void set_sections(value: int)
- int get_sections()

The total number of sections on the ribbon.

### section_length {#section_length}

```gdscript
section_length: float
```

- void set_section_length(value: float)
- float get_section_length()

The length of a section of the ribbon.

### section_segments {#section_segments}

```gdscript
section_segments: int
```

- void set_section_segments(value: int)
- int get_section_segments()

The number of segments in a section. The curve is sampled on each segment to determine its size. Higher values result in a more detailed ribbon at the cost of performance.

### curve {#curve}

```gdscript
curve: Curve
```

- void set_curve(value: Curve)
- Curve get_curve()

Determines the size of the ribbon along its length. The size of a particular section segment is obtained by multiplying the baseline size by the value of this curve at the given distance. For values smaller than 0, the faces will be inverted. Should be a unit Curve.

## 枚举

### Shape {#enum-Shape}

```gdscript
enum Shape { SHAPE_FLAT = 0, SHAPE_CROSS = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHAPE_FLAT {#SHAPE_FLAT}

```gdscript
SHAPE_FLAT = 0
```

Gives the mesh a single flat face.

#### SHAPE_CROSS {#SHAPE_CROSS}

```gdscript
SHAPE_CROSS = 1
```

Gives the mesh two perpendicular flat faces, making a cross shape.

---
