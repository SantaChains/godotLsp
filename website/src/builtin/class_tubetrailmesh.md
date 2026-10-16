# TubeTrailMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TubeTrailMesh**

> TubeTrailMesh represents a straight tube-shaped mesh with variable width. The tube is composed of a number of cylindrical sections, each with the same section_length and number of section_rings. A curve is sampled along the total length of the tube, meaning that the curve determines the radius of the tube along its length.

<details><summary>官方英文描述</summary>

TubeTrailMesh represents a straight tube-shaped mesh with variable width. The tube is composed of a number of cylindrical sections, each with the same section_length and number of section_rings. A curve is sampled along the total length of the tube, meaning that the curve determines the radius of the tube along its length.

This primitive mesh is usually used for particle trails.

</details>

## 方法

### get_curve {#get_curve}

```gdscript
Curve get_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radial_steps {#get_radial_steps}

```gdscript
int get_radial_steps() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_section_length {#get_section_length}

```gdscript
float get_section_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_section_rings {#get_section_rings}

```gdscript
int get_section_rings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sections {#get_sections}

```gdscript
int get_sections() const
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

### set_curve {#set_curve}

```gdscript
void set_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radial_steps {#set_radial_steps}

```gdscript
void set_radial_steps(radial_steps: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_section_length {#set_section_length}

```gdscript
void set_section_length(section_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_section_rings {#set_section_rings}

```gdscript
void set_section_rings(section_rings: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sections {#set_sections}

```gdscript
void set_sections(sections: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The baseline radius of the tube. The radius of a particular section ring is obtained by multiplying this radius by the value of the curve at the given distance.

### radial_steps {#radial_steps}

```gdscript
radial_steps: int
```

- void set_radial_steps(value: int)
- int get_radial_steps()

The number of sides on the tube. For example, a value of 5 means the tube will be pentagonal. Higher values result in a more detailed tube at the cost of performance.

### sections {#sections}

```gdscript
sections: int
```

- void set_sections(value: int)
- int get_sections()

The total number of sections on the tube.

### section_length {#section_length}

```gdscript
section_length: float
```

- void set_section_length(value: float)
- float get_section_length()

The length of a section of the tube.

### section_rings {#section_rings}

```gdscript
section_rings: int
```

- void set_section_rings(value: int)
- int get_section_rings()

The number of rings in a section. The curve is sampled on each ring to determine its radius. Higher values result in a more detailed tube at the cost of performance.

### cap_top {#cap_top}

```gdscript
cap_top: bool
```

- void set_cap_top(value: bool)
- bool is_cap_top()

If true, generates a cap at the top of the tube. This can be set to false to speed up generation and rendering when the cap is never seen by the camera.

### cap_bottom {#cap_bottom}

```gdscript
cap_bottom: bool
```

- void set_cap_bottom(value: bool)
- bool is_cap_bottom()

If true, generates a cap at the bottom of the tube. This can be set to false to speed up generation and rendering when the cap is never seen by the camera.

### curve {#curve}

```gdscript
curve: Curve
```

- void set_curve(value: Curve)
- Curve get_curve()

Determines the radius of the tube along its length. The radius of a particular section ring is obtained by multiplying the baseline radius by the value of this curve at the given distance. For values smaller than 0, the faces will be inverted. Should be a unit Curve.
