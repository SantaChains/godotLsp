# SphereMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SphereMesh**

> Class representing a spherical PrimitiveMesh.

<details><summary>官方英文描述</summary>

Class representing a spherical PrimitiveMesh.

</details>

## 方法

### get_height {#get_height}

```gdscript
float get_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_is_hemisphere {#get_is_hemisphere}

```gdscript
bool get_is_hemisphere() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radial_segments {#get_radial_segments}

```gdscript
int get_radial_segments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rings {#get_rings}

```gdscript
int get_rings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_is_hemisphere {#set_is_hemisphere}

```gdscript
void set_is_hemisphere(is_hemisphere: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radial_segments {#set_radial_segments}

```gdscript
void set_radial_segments(radial_segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rings {#set_rings}

```gdscript
void set_rings(rings: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

Radius of sphere.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

Full height of the sphere.

### radial_segments {#radial_segments}

```gdscript
radial_segments: int
```

- void set_radial_segments(value: int)
- int get_radial_segments()

Number of radial segments on the sphere.

### rings {#rings}

```gdscript
rings: int
```

- void set_rings(value: int)
- int get_rings()

Number of segments along the height of the sphere.

### is_hemisphere {#is_hemisphere}

```gdscript
is_hemisphere: bool
```

- void set_is_hemisphere(value: bool)
- bool get_is_hemisphere()

If true, a hemisphere is created rather than a full sphere.

Note: To get a regular hemisphere, the height and radius of the sphere must be equal.
