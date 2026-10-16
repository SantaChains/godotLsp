# OpenXRCompositionLayerEquirect

**继承**：[OpenXRCompositionLayer](class_openxrcompositionlayer.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRCompositionLayerEquirect**

> An OpenXR composition layer that allows rendering a SubViewport on an internal slice of a sphere.

<details><summary>官方英文描述（中文精译进行中）</summary>

An OpenXR composition layer that allows rendering a SubViewport on an internal slice of a sphere.

</details>

## 方法

### get_central_horizontal_angle {#get_central_horizontal_angle}

```gdscript
float get_central_horizontal_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fallback_segments {#get_fallback_segments}

```gdscript
int get_fallback_segments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_lower_vertical_angle {#get_lower_vertical_angle}

```gdscript
float get_lower_vertical_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_upper_vertical_angle {#get_upper_vertical_angle}

```gdscript
float get_upper_vertical_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_central_horizontal_angle {#set_central_horizontal_angle}

```gdscript
void set_central_horizontal_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fallback_segments {#set_fallback_segments}

```gdscript
void set_fallback_segments(segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lower_vertical_angle {#set_lower_vertical_angle}

```gdscript
void set_lower_vertical_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_upper_vertical_angle {#set_upper_vertical_angle}

```gdscript
void set_upper_vertical_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The radius of the sphere.

### central_horizontal_angle {#central_horizontal_angle}

```gdscript
central_horizontal_angle: float
```

- void set_central_horizontal_angle(value: float)
- float get_central_horizontal_angle()

The central horizontal angle of the sphere. Used to set the width.

### upper_vertical_angle {#upper_vertical_angle}

```gdscript
upper_vertical_angle: float
```

- void set_upper_vertical_angle(value: float)
- float get_upper_vertical_angle()

The upper vertical angle of the sphere. Used (together with lower_vertical_angle) to set the height.

### lower_vertical_angle {#lower_vertical_angle}

```gdscript
lower_vertical_angle: float
```

- void set_lower_vertical_angle(value: float)
- float get_lower_vertical_angle()

The lower vertical angle of the sphere. Used (together with upper_vertical_angle) to set the height.

### fallback_segments {#fallback_segments}

```gdscript
fallback_segments: int
```

- void set_fallback_segments(value: int)
- int get_fallback_segments()

The number of segments to use in the fallback mesh.
