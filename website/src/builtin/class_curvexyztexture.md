# CurveXYZTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CurveXYZTexture**

> A 1D texture where the red, green, and blue color channels correspond to points on 3 unit Curve resources. Compared to using separate CurveTextures, this further simplifies the task of saving curves as image files.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 1D texture where the red, green, and blue color channels correspond to points on 3 unit Curve resources. Compared to using separate CurveTextures, this further simplifies the task of saving curves as image files.

If you only need to store one curve within a single texture, use CurveTexture instead. See also GradientTexture1D and GradientTexture2D.

</details>

## 方法

### get_curve_x {#get_curve_x}

```gdscript
Curve get_curve_x() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_curve_y {#get_curve_y}

```gdscript
Curve get_curve_y() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_curve_z {#get_curve_z}

```gdscript
Curve get_curve_z() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve_x {#set_curve_x}

```gdscript
void set_curve_x(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve_y {#set_curve_y}

```gdscript
void set_curve_y(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve_z {#set_curve_z}

```gdscript
void set_curve_z(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_width {#set_width}

```gdscript
void set_width(width: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### width {#width}

```gdscript
width: int
```

- void set_width(value: int)
- int get_width()

The width of the texture (in pixels). Higher values make it possible to represent high-frequency data better (such as sudden direction changes), at the cost of increased generation time and memory usage.

### curve_x {#curve_x}

```gdscript
curve_x: Curve
```

- void set_curve_x(value: Curve)
- Curve get_curve_x()

The Curve that is rendered onto the texture's red channel. Should be a unit Curve.

### curve_y {#curve_y}

```gdscript
curve_y: Curve
```

- void set_curve_y(value: Curve)
- Curve get_curve_y()

The Curve that is rendered onto the texture's green channel. Should be a unit Curve.

### curve_z {#curve_z}

```gdscript
curve_z: Curve
```

- void set_curve_z(value: Curve)
- Curve get_curve_z()

The Curve that is rendered onto the texture's blue channel. Should be a unit Curve.
