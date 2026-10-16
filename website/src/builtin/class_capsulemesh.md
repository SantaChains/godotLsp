# CapsuleMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CapsuleMesh**

> Class representing a capsule-shaped PrimitiveMesh.

<details><summary>官方英文描述（中文精译进行中）</summary>

Class representing a capsule-shaped PrimitiveMesh.

</details>

## 方法

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

### set_radial_segments {#set_radial_segments}

```gdscript
void set_radial_segments(segments: int)
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

Radius of the capsule mesh.

Note: The radius of a capsule cannot be greater than half of its height. Otherwise, the capsule becomes a circle. If the radius is greater than half of the height, the properties adjust to a valid value.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

Total height of the capsule mesh (including the hemispherical ends).

Note: The height of a capsule must be at least twice its radius. Otherwise, the capsule becomes a circle. If the height is less than twice the radius, the properties adjust to a valid value.

### radial_segments {#radial_segments}

```gdscript
radial_segments: int
```

- void set_radial_segments(value: int)
- int get_radial_segments()

Number of radial segments on the capsule mesh.

### rings {#rings}

```gdscript
rings: int
```

- void set_rings(value: int)
- int get_rings()

Number of rings along the height of the capsule.
