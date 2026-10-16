# TextureButton

**继承**：[BaseButton](class_basebutton.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TextureButton**

> TextureButton has the same functionality as Button, except it uses sprites instead of Godot's Theme resource. It is faster to create, but it doesn't support localization like more complex Controls.

<details><summary>官方英文描述</summary>

TextureButton has the same functionality as Button, except it uses sprites instead of Godot's Theme resource. It is faster to create, but it doesn't support localization like more complex Controls.

See also BaseButton which contains common properties and methods associated with this node.

Note: Setting a texture for the "normal" state (texture_normal) is recommended. If texture_normal is not set, the TextureButton will still receive input events and be clickable, but the user will not be able to see it unless they activate another one of its states with a texture assigned (e.g., hover over it to show texture_hover).

</details>

## 方法

### get_click_mask {#get_click_mask}

```gdscript
BitMap get_click_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ignore_texture_size {#get_ignore_texture_size}

```gdscript
bool get_ignore_texture_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stretch_mode {#get_stretch_mode}

```gdscript
TextureButton.StretchMode get_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_disabled {#get_texture_disabled}

```gdscript
Texture2D get_texture_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_focused {#get_texture_focused}

```gdscript
Texture2D get_texture_focused() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_hover {#get_texture_hover}

```gdscript
Texture2D get_texture_hover() const
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

### is_flipped_h {#is_flipped_h}

```gdscript
bool is_flipped_h() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flipped_v {#is_flipped_v}

```gdscript
bool is_flipped_v() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_click_mask {#set_click_mask}

```gdscript
void set_click_mask(mask: BitMap)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_h {#set_flip_h}

```gdscript
void set_flip_h(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_v {#set_flip_v}

```gdscript
void set_flip_v(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ignore_texture_size {#set_ignore_texture_size}

```gdscript
void set_ignore_texture_size(ignore: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stretch_mode {#set_stretch_mode}

```gdscript
void set_stretch_mode(mode: TextureButton.StretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_disabled {#set_texture_disabled}

```gdscript
void set_texture_disabled(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_focused {#set_texture_focused}

```gdscript
void set_texture_focused(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_hover {#set_texture_hover}

```gdscript
void set_texture_hover(texture: Texture2D)
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

## 属性

### texture_normal {#texture_normal}

```gdscript
texture_normal: Texture2D
```

- void set_texture_normal(value: Texture2D)
- Texture2D get_texture_normal()

Texture to display by default, when the node is not in the disabled, hover or pressed state. This texture is still displayed in the focused state, with texture_focused drawn on top.

### texture_pressed {#texture_pressed}

```gdscript
texture_pressed: Texture2D
```

- void set_texture_pressed(value: Texture2D)
- Texture2D get_texture_pressed()

Texture to display on mouse down over the node, if the node has keyboard focus and the player presses the Enter key or if the player presses the BaseButton.shortcut key. If not assigned, the TextureButton displays texture_hover instead when pressed.

### texture_hover {#texture_hover}

```gdscript
texture_hover: Texture2D
```

- void set_texture_hover(value: Texture2D)
- Texture2D get_texture_hover()

Texture to display when the mouse hovers over the node. If not assigned, the TextureButton displays texture_normal instead when hovered over.

### texture_disabled {#texture_disabled}

```gdscript
texture_disabled: Texture2D
```

- void set_texture_disabled(value: Texture2D)
- Texture2D get_texture_disabled()

Texture to display when the node is disabled. See BaseButton.disabled. If not assigned, the TextureButton displays texture_normal instead.

### texture_focused {#texture_focused}

```gdscript
texture_focused: Texture2D
```

- void set_texture_focused(value: Texture2D)
- Texture2D get_texture_focused()

Texture to overlay on the base texture when the node has mouse or keyboard focus. Because texture_focused is displayed on top of the base texture, a partially transparent texture should be used to ensure the base texture remains visible. A texture that represents an outline or an underline works well for this purpose. To disable the focus visual effect, assign a fully transparent texture of any size. Note that disabling the focus visual effect will harm keyboard/controller navigation usability, so this is not recommended for accessibility reasons.

### texture_click_mask {#texture_click_mask}

```gdscript
texture_click_mask: BitMap
```

- void set_click_mask(value: BitMap)
- BitMap get_click_mask()

Pure black and white BitMap image to use for click detection. On the mask, white pixels represent the button's clickable area. Use it to create buttons with curved shapes.

### ignore_texture_size {#ignore_texture_size}

```gdscript
ignore_texture_size: bool
```

- void set_ignore_texture_size(value: bool)
- bool get_ignore_texture_size()

If true, the size of the texture won't be considered for minimum size calculation, so the TextureButton can be shrunk down past the texture size.

### stretch_mode {#stretch_mode}

```gdscript
stretch_mode: int
```

- void set_stretch_mode(value: StretchMode)
- StretchMode get_stretch_mode()

Controls the texture's behavior when you resize the node's bounding rectangle. See the StretchMode constants for available options.

### flip_h {#flip_h}

```gdscript
flip_h: bool
```

- void set_flip_h(value: bool)
- bool is_flipped_h()

If true, texture is flipped horizontally.

### flip_v {#flip_v}

```gdscript
flip_v: bool
```

- void set_flip_v(value: bool)
- bool is_flipped_v()

If true, texture is flipped vertically.

## 枚举

### StretchMode {#enum-StretchMode}

```gdscript
enum StretchMode { STRETCH_SCALE = 0, STRETCH_TILE = 1, STRETCH_KEEP = 2, STRETCH_KEEP_CENTERED = 3, STRETCH_KEEP_ASPECT = 4, STRETCH_KEEP_ASPECT_CENTERED = 5, STRETCH_KEEP_ASPECT_COVERED = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_SCALE {#STRETCH_SCALE}

```gdscript
STRETCH_SCALE = 0
```

Scale to fit the node's bounding rectangle.

#### STRETCH_TILE {#STRETCH_TILE}

```gdscript
STRETCH_TILE = 1
```

Tile inside the node's bounding rectangle.

#### STRETCH_KEEP {#STRETCH_KEEP}

```gdscript
STRETCH_KEEP = 2
```

The texture keeps its original size and stays in the bounding rectangle's top-left corner.

#### STRETCH_KEEP_CENTERED {#STRETCH_KEEP_CENTERED}

```gdscript
STRETCH_KEEP_CENTERED = 3
```

The texture keeps its original size and stays centered in the node's bounding rectangle.

#### STRETCH_KEEP_ASPECT {#STRETCH_KEEP_ASPECT}

```gdscript
STRETCH_KEEP_ASPECT = 4
```

Scale the texture to fit the node's bounding rectangle, but maintain the texture's aspect ratio.

#### STRETCH_KEEP_ASPECT_CENTERED {#STRETCH_KEEP_ASPECT_CENTERED}

```gdscript
STRETCH_KEEP_ASPECT_CENTERED = 5
```

Scale the texture to fit the node's bounding rectangle, center it, and maintain its aspect ratio.

#### STRETCH_KEEP_ASPECT_COVERED {#STRETCH_KEEP_ASPECT_COVERED}

```gdscript
STRETCH_KEEP_ASPECT_COVERED = 6
```

Scale the texture so that the shorter side fits the bounding rectangle. The other side clips to the node's limits.

---
