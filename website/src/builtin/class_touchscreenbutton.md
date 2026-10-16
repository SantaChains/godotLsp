# TouchScreenButton

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TouchScreenButton**

> TouchScreenButton allows you to create on-screen buttons for touch devices. It's intended for gameplay use, such as a unit you have to touch to move. Unlike Button, TouchScreenButton supports multitouch out of the box. Several TouchScreenButtons can be pressed at the same time with touch input.

<details><summary>官方英文描述（中文精译进行中）</summary>

TouchScreenButton allows you to create on-screen buttons for touch devices. It's intended for gameplay use, such as a unit you have to touch to move. Unlike Button, TouchScreenButton supports multitouch out of the box. Several TouchScreenButtons can be pressed at the same time with touch input.

This node inherits from Node2D. Unlike with Control nodes, you cannot set anchors on it. If you want to create menus or user interfaces, you may want to use Button nodes instead. To make button nodes react to touch events, you can enable ProjectSettings.input_devices/pointing/emulate_mouse_from_touch in the Project Settings.

You can configure TouchScreenButton to be visible only on touch devices, helping you develop your game both for desktop and mobile devices.

</details>

## 方法

### get_action {#get_action}

```gdscript
String get_action() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bitmask {#get_bitmask}

```gdscript
BitMap get_bitmask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
Shape2D get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_normal {#get_texture_normal}

```gdscript
Texture2D get_texture_normal() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_pressed {#get_texture_pressed}

```gdscript
Texture2D get_texture_pressed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visibility_mode {#get_visibility_mode}

```gdscript
TouchScreenButton.VisibilityMode get_visibility_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_passby_press_enabled {#is_passby_press_enabled}

```gdscript
bool is_passby_press_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pressed {#is_pressed}

```gdscript
bool is_pressed() const
```

Returns true if this button is currently pressed.

### is_shape_centered {#is_shape_centered}

```gdscript
bool is_shape_centered() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shape_visible {#is_shape_visible}

```gdscript
bool is_shape_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_action {#set_action}

```gdscript
void set_action(action: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bitmask {#set_bitmask}

```gdscript
void set_bitmask(bitmask: BitMap)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_passby_press {#set_passby_press}

```gdscript
void set_passby_press(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: Shape2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape_centered {#set_shape_centered}

```gdscript
void set_shape_centered(bool: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape_visible {#set_shape_visible}

```gdscript
void set_shape_visible(bool: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_normal {#set_texture_normal}

```gdscript
void set_texture_normal(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_pressed {#set_texture_pressed}

```gdscript
void set_texture_pressed(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visibility_mode {#set_visibility_mode}

```gdscript
void set_visibility_mode(mode: TouchScreenButton.VisibilityMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture_normal {#texture_normal}

```gdscript
texture_normal: Texture2D
```

- void set_texture_normal(value: Texture2D)
- Texture2D get_texture_normal()

The button's texture for the normal state.

### texture_pressed {#texture_pressed}

```gdscript
texture_pressed: Texture2D
```

- void set_texture_pressed(value: Texture2D)
- Texture2D get_texture_pressed()

The button's texture for the pressed state.

### bitmask {#bitmask}

```gdscript
bitmask: BitMap
```

- void set_bitmask(value: BitMap)
- BitMap get_bitmask()

The button's bitmask.

### shape {#shape}

```gdscript
shape: Shape2D
```

- void set_shape(value: Shape2D)
- Shape2D get_shape()

The button's shape.

### shape_centered {#shape_centered}

```gdscript
shape_centered: bool
```

- void set_shape_centered(value: bool)
- bool is_shape_centered()

If true, the button's shape is centered in the provided texture. If no texture is used, this property has no effect.

### shape_visible {#shape_visible}

```gdscript
shape_visible: bool
```

- void set_shape_visible(value: bool)
- bool is_shape_visible()

If true, the button's shape is visible in the editor.

### passby_press {#passby_press}

```gdscript
passby_press: bool
```

- void set_passby_press(value: bool)
- bool is_passby_press_enabled()

If true, the pressed and released signals are emitted whenever a pressed finger goes in and out of the button, even if the pressure started outside the active area of the button.

Note: This is a "pass-by" (not "bypass") press mode.

### action {#action}

```gdscript
action: StringName
```

- void set_action(value: String)
- String get_action()

The button's action. Actions can be handled with InputEventAction.

### visibility_mode {#visibility_mode}

```gdscript
visibility_mode: int
```

- void set_visibility_mode(value: VisibilityMode)
- VisibilityMode get_visibility_mode()

The button's visibility mode.

## 信号

### pressed {#pressed}

```gdscript
signal pressed()
```

Emitted when the button is pressed (down).

### released {#released}

```gdscript
signal released()
```

Emitted when the button is released (up).

## 枚举

### VisibilityMode {#enum-VisibilityMode}

```gdscript
enum VisibilityMode { VISIBILITY_ALWAYS = 0, VISIBILITY_TOUCHSCREEN_ONLY = 1 }
```

VisibilityMode VISIBILITY_ALWAYS = 0

Always visible.

VisibilityMode VISIBILITY_TOUCHSCREEN_ONLY = 1

Visible on touch screens only.

#### VISIBILITY_ALWAYS {#VISIBILITY_ALWAYS}

```gdscript
VISIBILITY_ALWAYS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VISIBILITY_TOUCHSCREEN_ONLY {#VISIBILITY_TOUCHSCREEN_ONLY}

```gdscript
VISIBILITY_TOUCHSCREEN_ONLY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
