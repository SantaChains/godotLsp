# ThemeDB

**继承**：[Object](class_object.md) → **ThemeDB**

> This singleton provides access to static information about Theme resources used by the engine and by your projects. You can fetch the default engine theme, as well as your project configured theme.

<details><summary>官方英文描述（中文精译进行中）</summary>

This singleton provides access to static information about Theme resources used by the engine and by your projects. You can fetch the default engine theme, as well as your project configured theme.

ThemeDB also contains fallback values for theme properties.

</details>

## 方法

### get_default_theme {#get_default_theme}

```gdscript
Theme get_default_theme()
```

Returns a reference to the default engine Theme. This theme resource is responsible for the out-of-the-box look of Control nodes and cannot be overridden.

### get_fallback_base_scale {#get_fallback_base_scale}

```gdscript
float get_fallback_base_scale()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fallback_font {#get_fallback_font}

```gdscript
Font get_fallback_font()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fallback_font_size {#get_fallback_font_size}

```gdscript
int get_fallback_font_size()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fallback_icon {#get_fallback_icon}

```gdscript
Texture2D get_fallback_icon()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fallback_stylebox {#get_fallback_stylebox}

```gdscript
StyleBox get_fallback_stylebox()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_project_theme {#get_project_theme}

```gdscript
Theme get_project_theme()
```

Returns a reference to the custom project Theme. This theme resources allows to override the default engine theme for every control node in the project.

To set the project theme, see ProjectSettings.gui/theme/custom.

### set_fallback_base_scale {#set_fallback_base_scale}

```gdscript
void set_fallback_base_scale(base_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fallback_font {#set_fallback_font}

```gdscript
void set_fallback_font(font: Font)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fallback_font_size {#set_fallback_font_size}

```gdscript
void set_fallback_font_size(font_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fallback_icon {#set_fallback_icon}

```gdscript
void set_fallback_icon(icon: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fallback_stylebox {#set_fallback_stylebox}

```gdscript
void set_fallback_stylebox(stylebox: StyleBox)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### fallback_base_scale {#fallback_base_scale}

```gdscript
fallback_base_scale: float
```

- void set_fallback_base_scale(value: float)
- float get_fallback_base_scale()

The fallback base scale factor of every Control node and Theme resource. Used when no other value is available to the control.

See also Theme.default_base_scale.

### fallback_font {#fallback_font}

```gdscript
fallback_font: Font
```

- void set_fallback_font(value: Font)
- Font get_fallback_font()

The fallback font of every Control node and Theme resource. Used when no other value is available to the control.

See also Theme.default_font.

### fallback_font_size {#fallback_font_size}

```gdscript
fallback_font_size: int
```

- void set_fallback_font_size(value: int)
- int get_fallback_font_size()

The fallback font size of every Control node and Theme resource. Used when no other value is available to the control.

See also Theme.default_font_size.

### fallback_icon {#fallback_icon}

```gdscript
fallback_icon: Texture2D
```

- void set_fallback_icon(value: Texture2D)
- Texture2D get_fallback_icon()

The fallback icon of every Control node and Theme resource. Used when no other value is available to the control.

### fallback_stylebox {#fallback_stylebox}

```gdscript
fallback_stylebox: StyleBox
```

- void set_fallback_stylebox(value: StyleBox)
- StyleBox get_fallback_stylebox()

The fallback stylebox of every Control node and Theme resource. Used when no other value is available to the control.

## 信号

### fallback_changed {#fallback_changed}

```gdscript
signal fallback_changed()
```

Emitted when one of the fallback values had been changed. Use it to refresh the look of controls that may rely on the fallback theme items.
