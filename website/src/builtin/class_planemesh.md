# PlaneMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PlaneMesh**

> Class representing a planar PrimitiveMesh. This flat mesh does not have a thickness. By default, this mesh is aligned on the X and Z axes; this default rotation isn't suited for use with billboarded materials. For billboarded materials, change orientation to FACE_Z.

<details><summary>官方英文描述（中文精译进行中）</summary>

Class representing a planar PrimitiveMesh. This flat mesh does not have a thickness. By default, this mesh is aligned on the X and Z axes; this default rotation isn't suited for use with billboarded materials. For billboarded materials, change orientation to FACE_Z.

Note: When using a large textured PlaneMesh (e.g. as a floor), you may stumble upon UV jittering issues depending on the camera angle. To solve this, increase subdivide_depth and subdivide_width until you no longer notice UV jittering.

</details>

## 方法

### get_center_offset {#get_center_offset}

```gdscript
Vector3 get_center_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_orientation {#get_orientation}

```gdscript
PlaneMesh.Orientation get_orientation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector2 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subdivide_depth {#get_subdivide_depth}

```gdscript
int get_subdivide_depth() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subdivide_width {#get_subdivide_width}

```gdscript
int get_subdivide_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_offset {#set_center_offset}

```gdscript
void set_center_offset(offset: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_orientation {#set_orientation}

```gdscript
void set_orientation(orientation: PlaneMesh.Orientation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_depth {#set_subdivide_depth}

```gdscript
void set_subdivide_depth(subdivide: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_width {#set_subdivide_width}

```gdscript
void set_subdivide_width(subdivide: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### subdivide_width {#subdivide_width}

```gdscript
subdivide_width: int
```

- void set_subdivide_width(value: int)
- int get_subdivide_width()

Number of subdivision along the X axis.

### subdivide_depth {#subdivide_depth}

```gdscript
subdivide_depth: int
```

- void set_subdivide_depth(value: int)
- int get_subdivide_depth()

Number of subdivision along the Z axis.

### center_offset {#center_offset}

```gdscript
center_offset: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### orientation {#orientation}

```gdscript
orientation: int
```

- void set_orientation(value: Orientation)
- Orientation get_orientation()

Direction that the PlaneMesh is facing.

## 枚举

### Orientation {#enum-Orientation}

```gdscript
enum Orientation { FACE_X = 0, FACE_Y = 1, FACE_Z = 2 }
```

Orientation FACE_X = 0

PlaneMesh will face the positive X-axis.

Orientation FACE_Y = 1

PlaneMesh will face the positive Y-axis. This matches the behavior of the PlaneMesh in Godot 3.x.

Orientation FACE_Z = 2

PlaneMesh will face the positive Z-axis. This matches the behavior of the QuadMesh in Godot 3.x.

#### FACE_X {#FACE_X}

```gdscript
FACE_X = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FACE_Y {#FACE_Y}

```gdscript
FACE_Y = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FACE_Z {#FACE_Z}

```gdscript
FACE_Z = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
