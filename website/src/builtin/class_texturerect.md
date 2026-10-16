# TextureRect

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TextureRect**

> A control that displays a texture, for example an icon inside a GUI. The texture's placement can be controlled with the stretch_mode property. It can scale, tile, or stay centered inside its bounding rectangle.

<details><summary>官方英文描述（中文精译进行中）</summary>

A control that displays a texture, for example an icon inside a GUI. The texture's placement can be controlled with the stretch_mode property. It can scale, tile, or stay centered inside its bounding rectangle.

</details>

## 方法

### get_expand_mode {#get_expand_mode}

```gdscript
TextureRect.ExpandMode get_expand_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stretch_mode {#get_stretch_mode}

```gdscript
TextureRect.StretchMode get_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
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

### set_expand_mode {#set_expand_mode}

```gdscript
void set_expand_mode(expand_mode: TextureRect.ExpandMode)
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

### set_stretch_mode {#set_stretch_mode}

```gdscript
void set_stretch_mode(stretch_mode: TextureRect.StretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The node's Texture2D resource.

### expand_mode {#expand_mode}

```gdscript
expand_mode: int
```

- void set_expand_mode(value: ExpandMode)
- ExpandMode get_expand_mode()

Experimental: Using EXPAND_FIT_WIDTH, EXPAND_FIT_WIDTH_PROPORTIONAL, EXPAND_FIT_HEIGHT, or EXPAND_FIT_HEIGHT_PROPORTIONAL may result in unstable behavior in some Container controls. This behavior may be re-evaluated and changed in the future.

Defines how minimum size is determined based on the texture's size.

### stretch_mode {#stretch_mode}

```gdscript
stretch_mode: int
```

- void set_stretch_mode(value: StretchMode)
- StretchMode get_stretch_mode()

Controls the texture's behavior when resizing the node's bounding rectangle.

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

### ExpandMode {#enum-ExpandMode}

```gdscript
enum ExpandMode { EXPAND_KEEP_SIZE = 0, EXPAND_IGNORE_SIZE = 1, EXPAND_FIT_WIDTH = 2, EXPAND_FIT_WIDTH_PROPORTIONAL = 3, EXPAND_FIT_HEIGHT = 4, EXPAND_FIT_HEIGHT_PROPORTIONAL = 5 }
```

ExpandMode EXPAND_KEEP_SIZE = 0

The minimum size will be equal to texture size, i.e. TextureRect can't be smaller than the texture.

ExpandMode EXPAND_IGNORE_SIZE = 1

The size of the texture won't be considered for minimum size calculation, so the TextureRect can be shrunk down past the texture size.

ExpandMode EXPAND_FIT_WIDTH = 2

The height of the texture will be ignored. Minimum width will be equal to the current height. Useful for horizontal layouts, e.g. inside HBoxContainer.

ExpandMode EXPAND_FIT_WIDTH_PROPORTIONAL = 3

Same as EXPAND_FIT_WIDTH, but keeps texture's aspect ratio.

ExpandMode EXPAND_FIT_HEIGHT = 4

The width of the texture will be ignored. Minimum height will be equal to the current width. Useful for vertical layouts, e.g. inside VBoxContainer.

ExpandMode EXPAND_FIT_HEIGHT_PROPORTIONAL = 5

Same as EXPAND_FIT_HEIGHT, but keeps texture's aspect ratio.

#### EXPAND_KEEP_SIZE {#EXPAND_KEEP_SIZE}

```gdscript
EXPAND_KEEP_SIZE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPAND_IGNORE_SIZE {#EXPAND_IGNORE_SIZE}

```gdscript
EXPAND_IGNORE_SIZE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPAND_FIT_WIDTH {#EXPAND_FIT_WIDTH}

```gdscript
EXPAND_FIT_WIDTH = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPAND_FIT_WIDTH_PROPORTIONAL {#EXPAND_FIT_WIDTH_PROPORTIONAL}

```gdscript
EXPAND_FIT_WIDTH_PROPORTIONAL = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPAND_FIT_HEIGHT {#EXPAND_FIT_HEIGHT}

```gdscript
EXPAND_FIT_HEIGHT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPAND_FIT_HEIGHT_PROPORTIONAL {#EXPAND_FIT_HEIGHT_PROPORTIONAL}

```gdscript
EXPAND_FIT_HEIGHT_PROPORTIONAL = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### StretchMode {#enum-StretchMode}

```gdscript
enum StretchMode { STRETCH_SCALE = 0, STRETCH_TILE = 1, STRETCH_KEEP = 2, STRETCH_KEEP_CENTERED = 3, STRETCH_KEEP_ASPECT = 4, STRETCH_KEEP_ASPECT_CENTERED = 5, STRETCH_KEEP_ASPECT_COVERED = 6 }
```

StretchMode STRETCH_SCALE = 0

Scale to fit the node's bounding rectangle.

StretchMode STRETCH_TILE = 1

Tile inside the node's bounding rectangle.

Note: STRETCH_TILE mode is not supported for texture set to an AtlasTexture with non-zero AtlasTexture.margin.

StretchMode STRETCH_KEEP = 2

The texture keeps its original size and stays in the bounding rectangle's top-left corner.

StretchMode STRETCH_KEEP_CENTERED = 3

The texture keeps its original size and stays centered in the node's bounding rectangle.

StretchMode STRETCH_KEEP_ASPECT = 4

Scale the texture to fit the node's bounding rectangle, but maintain the texture's aspect ratio.

StretchMode STRETCH_KEEP_ASPECT_CENTERED = 5

Scale the texture to fit the node's bounding rectangle, center it and maintain its aspect ratio.

StretchMode STRETCH_KEEP_ASPECT_COVERED = 6

Scale the texture so that the shorter side fits the bounding rectangle. The other side clips to the node's limits.

#### STRETCH_SCALE {#STRETCH_SCALE}

```gdscript
STRETCH_SCALE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_TILE {#STRETCH_TILE}

```gdscript
STRETCH_TILE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_KEEP {#STRETCH_KEEP}

```gdscript
STRETCH_KEEP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_KEEP_CENTERED {#STRETCH_KEEP_CENTERED}

```gdscript
STRETCH_KEEP_CENTERED = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_KEEP_ASPECT {#STRETCH_KEEP_ASPECT}

```gdscript
STRETCH_KEEP_ASPECT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_KEEP_ASPECT_CENTERED {#STRETCH_KEEP_ASPECT_CENTERED}

```gdscript
STRETCH_KEEP_ASPECT_CENTERED = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_KEEP_ASPECT_COVERED {#STRETCH_KEEP_ASPECT_COVERED}

```gdscript
STRETCH_KEEP_ASPECT_COVERED = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
