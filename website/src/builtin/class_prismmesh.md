# PrismMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PrismMesh**

> Class representing a prism-shaped PrimitiveMesh.

<details><summary>官方英文描述（中文精译进行中）</summary>

Class representing a prism-shaped PrimitiveMesh.

</details>

## 方法

### get_left_to_right {#get_left_to_right}

```gdscript
float get_left_to_right() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subdivide_depth {#get_subdivide_depth}

```gdscript
int get_subdivide_depth() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subdivide_height {#get_subdivide_height}

```gdscript
int get_subdivide_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subdivide_width {#get_subdivide_width}

```gdscript
int get_subdivide_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_left_to_right {#set_left_to_right}

```gdscript
void set_left_to_right(left_to_right: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_depth {#set_subdivide_depth}

```gdscript
void set_subdivide_depth(segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_height {#set_subdivide_height}

```gdscript
void set_subdivide_height(segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_width {#set_subdivide_width}

```gdscript
void set_subdivide_width(segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### left_to_right {#left_to_right}

```gdscript
left_to_right: float
```

- void set_left_to_right(value: float)
- float get_left_to_right()

Displacement of the upper edge along the X axis. 0.0 positions edge straight above the bottom-left edge.

### size {#size}

```gdscript
size: Vector3
```

- void set_size(value: Vector3)
- Vector3 get_size()

Size of the prism.

### subdivide_width {#subdivide_width}

```gdscript
subdivide_width: int
```

- void set_subdivide_width(value: int)
- int get_subdivide_width()

Number of added edge loops along the X axis.

### subdivide_height {#subdivide_height}

```gdscript
subdivide_height: int
```

- void set_subdivide_height(value: int)
- int get_subdivide_height()

Number of added edge loops along the Y axis.

### subdivide_depth {#subdivide_depth}

```gdscript
subdivide_depth: int
```

- void set_subdivide_depth(value: int)
- int get_subdivide_depth()

Number of added edge loops along the Z axis.
