# PathFollow2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **PathFollow2D**

> This node takes its parent Path2D, and returns the coordinates of a point within it, given a distance from the first vertex.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node takes its parent Path2D, and returns the coordinates of a point within it, given a distance from the first vertex.

It is useful for making other nodes follow a path, without coding the movement pattern. For that, the nodes must be children of this node. The descendant nodes will then move accordingly when setting the progress in this node.

</details>

## 方法

### get_cubic_interpolation {#get_cubic_interpolation}

```gdscript
bool get_cubic_interpolation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_offset {#get_h_offset}

```gdscript
float get_h_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_progress {#get_progress}

```gdscript
float get_progress() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_progress_ratio {#get_progress_ratio}

```gdscript
float get_progress_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_offset {#get_v_offset}

```gdscript
float get_v_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_loop {#has_loop}

```gdscript
bool has_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_rotating {#is_rotating}

```gdscript
bool is_rotating() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cubic_interpolation {#set_cubic_interpolation}

```gdscript
void set_cubic_interpolation(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_h_offset {#set_h_offset}

```gdscript
void set_h_offset(h_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop {#set_loop}

```gdscript
void set_loop(loop: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_progress {#set_progress}

```gdscript
void set_progress(progress: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_progress_ratio {#set_progress_ratio}

```gdscript
void set_progress_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotates {#set_rotates}

```gdscript
void set_rotates(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_offset {#set_v_offset}

```gdscript
void set_v_offset(v_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### progress {#progress}

```gdscript
progress: float
```

- void set_progress(value: float)
- float get_progress()

The distance along the path, in pixels. Changing this value sets this node's position to a point within the path.

### progress_ratio {#progress_ratio}

```gdscript
progress_ratio: float
```

- void set_progress_ratio(value: float)
- float get_progress_ratio()

The distance along the path as a number in the range 0.0 (for the first vertex) to 1.0 (for the last). This is just another way of expressing the progress within the path, as the offset supplied is multiplied internally by the path's length.

It can be set or get only if the PathFollow2D is the child of a Path2D which is part of the scene tree, and that this Path2D has a Curve2D with a non-zero length. Otherwise, trying to set this field will print an error, and getting this field will return 0.0.

### h_offset {#h_offset}

```gdscript
h_offset: float
```

- void set_h_offset(value: float)
- float get_h_offset()

The node's offset along the curve.

### v_offset {#v_offset}

```gdscript
v_offset: float
```

- void set_v_offset(value: float)
- float get_v_offset()

The node's offset perpendicular to the curve.

### rotates {#rotates}

```gdscript
rotates: bool
```

- void set_rotates(value: bool)
- bool is_rotating()

If true, this node rotates to follow the path, with the +X direction facing forward on the path.

### cubic_interp {#cubic_interp}

```gdscript
cubic_interp: bool
```

- void set_cubic_interpolation(value: bool)
- bool get_cubic_interpolation()

If true, the position between two cached points is interpolated cubically, and linearly otherwise.

The points along the Curve2D of the Path2D are precomputed before use, for faster calculations. The point at the requested offset is then calculated interpolating between two adjacent cached points. This may present a problem if the curve makes sharp turns, as the cached points may not follow the curve closely enough.

There are two answers to this problem: either increase the number of cached points and increase memory consumption, or make a cubic interpolation between two points at the cost of (slightly) slower calculations.

### loop {#loop}

```gdscript
loop: bool
```

- void set_loop(value: bool)
- bool has_loop()

If true, any offset outside the path's length will wrap around, instead of stopping at the ends. Use it for cyclic paths.
