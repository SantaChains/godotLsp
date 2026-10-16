# ColorPalette

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ColorPalette**

> The ColorPalette resource is designed to store and manage a collection of colors. This resource is useful in scenarios where a predefined set of colors is required, such as for creating themes, designing user interfaces, or managing game assets. The built-in ColorPicker control can also make use of ColorPalette without additional code.

<details><summary>官方英文描述（中文精译进行中）</summary>

The ColorPalette resource is designed to store and manage a collection of colors. This resource is useful in scenarios where a predefined set of colors is required, such as for creating themes, designing user interfaces, or managing game assets. The built-in ColorPicker control can also make use of ColorPalette without additional code.

</details>

## 方法

### get_colors {#get_colors}

```gdscript
PackedColorArray get_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_colors {#set_colors}

```gdscript
void set_colors(colors: PackedColorArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### colors {#colors}

```gdscript
colors: PackedColorArray
```

- void set_colors(value: PackedColorArray)
- PackedColorArray get_colors()

A PackedColorArray containing the colors in the palette.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedColorArray for more details.
