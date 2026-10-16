# SubViewport

**继承**：[Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **SubViewport**

> SubViewport Isolates a rectangular region of a scene to be displayed independently. This can be used, for example, to display UI in 3D space.

<details><summary>官方英文描述（中文精译进行中）</summary>

SubViewport Isolates a rectangular region of a scene to be displayed independently. This can be used, for example, to display UI in 3D space.

Note: SubViewport is a Viewport that isn't a Window, i.e. it doesn't draw anything by itself. To display anything, SubViewport must have a non-zero size and be either put inside a SubViewportContainer or assigned to a ViewportTexture.

Note: InputEvents are not passed to a standalone SubViewport by default. To ensure InputEvent propagation, a SubViewport can be placed inside of a SubViewportContainer.

</details>

## 方法

### get_clear_mode {#get_clear_mode}

```gdscript
SubViewport.ClearMode get_clear_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector2i get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size_2d_override {#get_size_2d_override}

```gdscript
Vector2i get_size_2d_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_mode {#get_update_mode}

```gdscript
SubViewport.UpdateMode get_update_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_view_count {#get_view_count}

```gdscript
int get_view_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_size_2d_override_stretch_enabled {#is_size_2d_override_stretch_enabled}

```gdscript
bool is_size_2d_override_stretch_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_clear_mode {#set_clear_mode}

```gdscript
void set_clear_mode(mode: SubViewport.ClearMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size_2d_override {#set_size_2d_override}

```gdscript
void set_size_2d_override(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size_2d_override_stretch {#set_size_2d_override_stretch}

```gdscript
void set_size_2d_override_stretch(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_mode {#set_update_mode}

```gdscript
void set_update_mode(mode: SubViewport.UpdateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_view_count {#set_view_count}

```gdscript
void set_view_count(view_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector2i
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### size_2d_override {#size_2d_override}

```gdscript
size_2d_override: Vector2i
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### size_2d_override_stretch {#size_2d_override_stretch}

```gdscript
size_2d_override_stretch: bool
```

- void set_size_2d_override_stretch(value: bool)
- bool is_size_2d_override_stretch_enabled()

If true, the 2D size override affects stretch as well.

### view_count {#view_count}

```gdscript
view_count: int
```

- void set_view_count(value: int)
- int get_view_count()

The number of view layers we are rendering to. Set this to 2 to enable stereo rendering.

### render_target_clear_mode {#render_target_clear_mode}

```gdscript
render_target_clear_mode: int
```

- void set_clear_mode(value: ClearMode)
- ClearMode get_clear_mode()

The clear mode when the sub-viewport is used as a render target.

Note: This property is intended for 2D usage.

### render_target_update_mode {#render_target_update_mode}

```gdscript
render_target_update_mode: int
```

- void set_update_mode(value: UpdateMode)
- UpdateMode get_update_mode()

The update mode when the sub-viewport is used as a render target.

## 枚举

### ClearMode {#enum-ClearMode}

```gdscript
enum ClearMode { CLEAR_MODE_ALWAYS = 0, CLEAR_MODE_NEVER = 1, CLEAR_MODE_ONCE = 2 }
```

ClearMode CLEAR_MODE_ALWAYS = 0

Always clear the render target before drawing.

ClearMode CLEAR_MODE_NEVER = 1

Never clear the render target.

ClearMode CLEAR_MODE_ONCE = 2

Clear the render target on the next frame, then switch to CLEAR_MODE_NEVER.

#### CLEAR_MODE_ALWAYS {#CLEAR_MODE_ALWAYS}

```gdscript
CLEAR_MODE_ALWAYS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CLEAR_MODE_NEVER {#CLEAR_MODE_NEVER}

```gdscript
CLEAR_MODE_NEVER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CLEAR_MODE_ONCE {#CLEAR_MODE_ONCE}

```gdscript
CLEAR_MODE_ONCE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### UpdateMode {#enum-UpdateMode}

```gdscript
enum UpdateMode { UPDATE_DISABLED = 0, UPDATE_ONCE = 1, UPDATE_WHEN_VISIBLE = 2, UPDATE_WHEN_PARENT_VISIBLE = 3, UPDATE_ALWAYS = 4 }
```

UpdateMode UPDATE_DISABLED = 0

Do not update the render target.

UpdateMode UPDATE_ONCE = 1

Update the render target once, then switch to UPDATE_DISABLED.

UpdateMode UPDATE_WHEN_VISIBLE = 2

Update the render target only when it is visible. This is the default value.

UpdateMode UPDATE_WHEN_PARENT_VISIBLE = 3

Update the render target only when its parent is visible.

UpdateMode UPDATE_ALWAYS = 4

Always update the render target.

#### UPDATE_DISABLED {#UPDATE_DISABLED}

```gdscript
UPDATE_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_ONCE {#UPDATE_ONCE}

```gdscript
UPDATE_ONCE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_WHEN_VISIBLE {#UPDATE_WHEN_VISIBLE}

```gdscript
UPDATE_WHEN_VISIBLE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_WHEN_PARENT_VISIBLE {#UPDATE_WHEN_PARENT_VISIBLE}

```gdscript
UPDATE_WHEN_PARENT_VISIBLE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_ALWAYS {#UPDATE_ALWAYS}

```gdscript
UPDATE_ALWAYS = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
