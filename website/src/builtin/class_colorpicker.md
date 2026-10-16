# ColorPicker

**继承**：[VBoxContainer](class_vboxcontainer.md) → [BoxContainer](class_boxcontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ColorPicker**

> A widget that provides an interface for selecting or modifying a color. It can optionally provide functionalities like a color sampler (eyedropper), color modes, and presets.

<details><summary>官方英文描述（中文精译进行中）</summary>

A widget that provides an interface for selecting or modifying a color. It can optionally provide functionalities like a color sampler (eyedropper), color modes, and presets.

Note: This control is the color picker widget itself. You can use a ColorPickerButton instead if you need a button that brings up a ColorPicker in a popup.

</details>

## 方法

### add_preset {#add_preset}

```gdscript
void add_preset(color: Color)
```

Adds the given color to a list of color presets. The presets are displayed in the color picker and the user will be able to select them.

Note: The presets list is only for this color picker.

### add_recent_preset {#add_recent_preset}

```gdscript
void add_recent_preset(color: Color)
```

Adds the given color to a list of color recent presets so that it can be picked later. Recent presets are the colors that were picked recently, a new preset is automatically created and added to recent presets when you pick a new color.

Note: The recent presets list is only for this color picker.

### are_modes_visible {#are_modes_visible}

```gdscript
bool are_modes_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### are_presets_visible {#are_presets_visible}

```gdscript
bool are_presets_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### are_sliders_visible {#are_sliders_visible}

```gdscript
bool are_sliders_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### are_swatches_enabled {#are_swatches_enabled}

```gdscript
bool are_swatches_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### erase_preset {#erase_preset}

```gdscript
void erase_preset(color: Color)
```

Removes the given color from the list of color presets of this color picker.

### erase_recent_preset {#erase_recent_preset}

```gdscript
void erase_recent_preset(color: Color)
```

Removes the given color from the list of color recent presets of this color picker.

### get_color_mode {#get_color_mode}

```gdscript
ColorPicker.ColorModeType get_color_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pick_color {#get_pick_color}

```gdscript
Color get_pick_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_picker_shape {#get_picker_shape}

```gdscript
ColorPicker.PickerShapeType get_picker_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_presets {#get_presets}

```gdscript
PackedColorArray get_presets() const
```

Returns the list of colors in the presets of the color picker.

### get_recent_presets {#get_recent_presets}

```gdscript
PackedColorArray get_recent_presets() const
```

Returns the list of colors in the recent presets of the color picker.

### is_deferred_mode {#is_deferred_mode}

```gdscript
bool is_deferred_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editing_alpha {#is_editing_alpha}

```gdscript
bool is_editing_alpha() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editing_intensity {#is_editing_intensity}

```gdscript
bool is_editing_intensity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hex_visible {#is_hex_visible}

```gdscript
bool is_hex_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_sampler_visible {#is_sampler_visible}

```gdscript
bool is_sampler_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_can_add_swatches {#set_can_add_swatches}

```gdscript
void set_can_add_swatches(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_mode {#set_color_mode}

```gdscript
void set_color_mode(color_mode: ColorPicker.ColorModeType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deferred_mode {#set_deferred_mode}

```gdscript
void set_deferred_mode(mode: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edit_alpha {#set_edit_alpha}

```gdscript
void set_edit_alpha(show: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edit_intensity {#set_edit_intensity}

```gdscript
void set_edit_intensity(show: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hex_visible {#set_hex_visible}

```gdscript
void set_hex_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_modes_visible {#set_modes_visible}

```gdscript
void set_modes_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pick_color {#set_pick_color}

```gdscript
void set_pick_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_picker_shape {#set_picker_shape}

```gdscript
void set_picker_shape(shape: ColorPicker.PickerShapeType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_presets_visible {#set_presets_visible}

```gdscript
void set_presets_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sampler_visible {#set_sampler_visible}

```gdscript
void set_sampler_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sliders_visible {#set_sliders_visible}

```gdscript
void set_sliders_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### color {#color}

```gdscript
color: Color
```

- void set_pick_color(value: Color)
- Color get_pick_color()

The currently selected color.

### edit_alpha {#edit_alpha}

```gdscript
edit_alpha: bool
```

- void set_edit_alpha(value: bool)
- bool is_editing_alpha()

If true, shows an alpha channel slider (opacity).

### edit_intensity {#edit_intensity}

```gdscript
edit_intensity: bool
```

- void set_edit_intensity(value: bool)
- bool is_editing_intensity()

If true, shows an intensity slider. The intensity is applied as follows: convert the color to linear encoding, multiply it by 2 ** intensity, and then convert it back to nonlinear sRGB encoding.

### color_mode {#color_mode}

```gdscript
color_mode: int
```

- void set_color_mode(value: ColorModeType)
- ColorModeType get_color_mode()

The currently selected color mode.

### deferred_mode {#deferred_mode}

```gdscript
deferred_mode: bool
```

- void set_deferred_mode(value: bool)
- bool is_deferred_mode()

If true, the color will apply only after the user releases the mouse button, otherwise it will apply immediately even in mouse motion event (which can cause performance issues).

### picker_shape {#picker_shape}

```gdscript
picker_shape: int
```

- void set_picker_shape(value: PickerShapeType)
- PickerShapeType get_picker_shape()

The shape of the color space view.

### can_add_swatches {#can_add_swatches}

```gdscript
can_add_swatches: bool
```

- void set_can_add_swatches(value: bool)
- bool are_swatches_enabled()

If true, it's possible to add presets under Swatches. If false, the button to add presets is disabled.

### sampler_visible {#sampler_visible}

```gdscript
sampler_visible: bool
```

- void set_sampler_visible(value: bool)
- bool is_sampler_visible()

If true, the color sampler and color preview are visible.

### color_modes_visible {#color_modes_visible}

```gdscript
color_modes_visible: bool
```

- void set_modes_visible(value: bool)
- bool are_modes_visible()

If true, the color mode buttons are visible.

### sliders_visible {#sliders_visible}

```gdscript
sliders_visible: bool
```

- void set_sliders_visible(value: bool)
- bool are_sliders_visible()

If true, the color sliders are visible.

### hex_visible {#hex_visible}

```gdscript
hex_visible: bool
```

- void set_hex_visible(value: bool)
- bool is_hex_visible()

If true, the hex color code input field is visible.

### presets_visible {#presets_visible}

```gdscript
presets_visible: bool
```

- void set_presets_visible(value: bool)
- bool are_presets_visible()

If true, the Swatches and Recent Colors presets are visible.

## 信号

### color_changed {#color_changed}

```gdscript
signal color_changed(color: Color)
```

Emitted when the color is changed.

### preset_added {#preset_added}

```gdscript
signal preset_added(color: Color)
```

Emitted when a preset is added.

### preset_removed {#preset_removed}

```gdscript
signal preset_removed(color: Color)
```

Emitted when a preset is removed.

## 枚举

### ColorModeType {#enum-ColorModeType}

```gdscript
enum ColorModeType { MODE_RGB = 0, MODE_HSV = 1, MODE_RAW = 2, MODE_LINEAR = 2, MODE_OKHSL = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_RGB {#MODE_RGB}

```gdscript
MODE_RGB = 0
```

Allows editing the color with Red/Green/Blue sliders in sRGB color space.

#### MODE_HSV {#MODE_HSV}

```gdscript
MODE_HSV = 1
```

Allows editing the color with Hue/Saturation/Value sliders.

#### MODE_RAW {#MODE_RAW}

```gdscript
MODE_RAW = 2
```

Deprecated: This is replaced by MODE_LINEAR.

#### MODE_LINEAR {#MODE_LINEAR}

```gdscript
MODE_LINEAR = 2
```

Allows editing the color with Red/Green/Blue sliders in linear color space.

#### MODE_OKHSL {#MODE_OKHSL}

```gdscript
MODE_OKHSL = 3
```

Allows editing the color with Hue/Saturation/Lightness sliders.

OKHSL is a new color space similar to HSL but that better match perception by leveraging the Oklab color space which is designed to be simple to use, while doing a good job at predicting perceived lightness, chroma and hue.

Okhsv and Okhsl color spaces [https://bottosson.github.io/posts/colorpicker/]

---

### PickerShapeType {#enum-PickerShapeType}

```gdscript
enum PickerShapeType { SHAPE_HSV_RECTANGLE = 0, SHAPE_HSV_WHEEL = 1, SHAPE_VHS_CIRCLE = 2, SHAPE_OKHSL_CIRCLE = 3, SHAPE_NONE = 4, SHAPE_OK_HS_RECTANGLE = 5, SHAPE_OK_HL_RECTANGLE = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHAPE_HSV_RECTANGLE {#SHAPE_HSV_RECTANGLE}

```gdscript
SHAPE_HSV_RECTANGLE = 0
```

HSV Color Model rectangle color space.

#### SHAPE_HSV_WHEEL {#SHAPE_HSV_WHEEL}

```gdscript
SHAPE_HSV_WHEEL = 1
```

HSV Color Model rectangle color space with a wheel.

#### SHAPE_VHS_CIRCLE {#SHAPE_VHS_CIRCLE}

```gdscript
SHAPE_VHS_CIRCLE = 2
```

HSV Color Model circle color space. Use Saturation as a radius.

#### SHAPE_OKHSL_CIRCLE {#SHAPE_OKHSL_CIRCLE}

```gdscript
SHAPE_OKHSL_CIRCLE = 3
```

HSL OK Color Model circle color space.

#### SHAPE_NONE {#SHAPE_NONE}

```gdscript
SHAPE_NONE = 4
```

The color space shape and the shape select button are hidden. Can't be selected from the shapes popup.

#### SHAPE_OK_HS_RECTANGLE {#SHAPE_OK_HS_RECTANGLE}

```gdscript
SHAPE_OK_HS_RECTANGLE = 5
```

OKHSL Color Model rectangle with constant lightness.

#### SHAPE_OK_HL_RECTANGLE {#SHAPE_OK_HL_RECTANGLE}

```gdscript
SHAPE_OK_HL_RECTANGLE = 6
```

OKHSL Color Model rectangle with constant saturation.

---
