# StyleBoxLine

**继承**：[StyleBox](class_stylebox.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StyleBoxLine**

> A StyleBox that displays a single line of a given color and thickness. The line can be either horizontal or vertical. Useful for separators.

<details><summary>官方英文描述（中文精译进行中）</summary>

A StyleBox that displays a single line of a given color and thickness. The line can be either horizontal or vertical. Useful for separators.

</details>

## 方法

### get_color {#get_color}

```gdscript
Color get_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_grow_begin {#get_grow_begin}

```gdscript
float get_grow_begin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_grow_end {#get_grow_end}

```gdscript
float get_grow_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_thickness {#get_thickness}

```gdscript
int get_thickness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_vertical {#is_vertical}

```gdscript
bool is_vertical() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color {#set_color}

```gdscript
void set_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_grow_begin {#set_grow_begin}

```gdscript
void set_grow_begin(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_grow_end {#set_grow_end}

```gdscript
void set_grow_end(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_thickness {#set_thickness}

```gdscript
void set_thickness(thickness: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical {#set_vertical}

```gdscript
void set_vertical(vertical: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### color {#color}

```gdscript
color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### grow_begin {#grow_begin}

```gdscript
grow_begin: float
```

- void set_grow_begin(value: float)
- float get_grow_begin()

The number of pixels the line will extend before the StyleBoxLine's bounds. If set to a negative value, the line will begin inside the StyleBoxLine's bounds.

### grow_end {#grow_end}

```gdscript
grow_end: float
```

- void set_grow_end(value: float)
- float get_grow_end()

The number of pixels the line will extend past the StyleBoxLine's bounds. If set to a negative value, the line will end inside the StyleBoxLine's bounds.

### thickness {#thickness}

```gdscript
thickness: int
```

- void set_thickness(value: int)
- int get_thickness()

The line's thickness in pixels.

### vertical {#vertical}

```gdscript
vertical: bool
```

- void set_vertical(value: bool)
- bool is_vertical()

If true, the line will be vertical. If false, the line will be horizontal.
