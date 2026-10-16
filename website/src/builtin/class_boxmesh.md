# BoxMesh

**继承**：[PrimitiveMesh](class_primitivemesh.md) → [Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **BoxMesh**

> Generate an axis-aligned box PrimitiveMesh.

<details><summary>官方英文描述（中文精译进行中）</summary>

Generate an axis-aligned box PrimitiveMesh.

The box's UV layout is arranged in a 3×2 layout that allows texturing each face individually. To apply the same texture on all faces, change the material's UV property to Vector3(3, 2, 1). This is equivalent to adding UV *= vec2(3.0, 2.0) in a vertex shader.

Note: When using a large textured BoxMesh (e.g. as a floor), you may stumble upon UV jittering issues depending on the camera angle. To solve this, increase subdivide_depth, subdivide_height and subdivide_width until you no longer notice UV jittering.

</details>

## 方法

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

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_depth {#set_subdivide_depth}

```gdscript
void set_subdivide_depth(divisions: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdivide_height {#set_subdivide_height}

```gdscript
void set_subdivide_height(divisions: int)
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
size: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### subdivide_width {#subdivide_width}

```gdscript
subdivide_width: int
```

- void set_subdivide_width(value: int)
- int get_subdivide_width()

Number of extra edge loops inserted along the X axis.

### subdivide_height {#subdivide_height}

```gdscript
subdivide_height: int
```

- void set_subdivide_height(value: int)
- int get_subdivide_height()

Number of extra edge loops inserted along the Y axis.

### subdivide_depth {#subdivide_depth}

```gdscript
subdivide_depth: int
```

- void set_subdivide_depth(value: int)
- int get_subdivide_depth()

Number of extra edge loops inserted along the Z axis.
