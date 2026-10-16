# CanvasGroup

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **CanvasGroup**

> Child CanvasItem nodes of a CanvasGroup are drawn as a single object. It allows to e.g. draw overlapping translucent 2D nodes without causing the overlapping sections to be more opaque than intended (set the CanvasItem.self_modulate property on the CanvasGroup to achieve this effect).

<details><summary>官方英文描述</summary>

Child CanvasItem nodes of a CanvasGroup are drawn as a single object. It allows to e.g. draw overlapping translucent 2D nodes without causing the overlapping sections to be more opaque than intended (set the CanvasItem.self_modulate property on the CanvasGroup to achieve this effect).

Note: The CanvasGroup uses a custom shader to read from the backbuffer to draw its children. Assigning a Material to the CanvasGroup overrides the built-in shader. To duplicate the behavior of the built-in shader in a custom Shader, use the following:

```gdscript
shader_type canvas_item;
render_mode unshaded;
uniform sampler2D screen_texture : hint_screen_texture, repeat_disable, filter_nearest;
void fragment() {
    vec4 c = textureLod(screen_texture, SCREEN_UV, 0.0);
    if (c.a > 0.0001) {
        c.rgb /= c.a;
    }
    COLOR *= c;
}
```

Note: Since CanvasGroup and CanvasItem.clip_children both utilize the backbuffer, children of a CanvasGroup who have their CanvasItem.clip_children set to anything other than CanvasItem.CLIP_CHILDREN_DISABLED will not function correctly.

</details>

## 方法

### get_clear_margin {#get_clear_margin}

```gdscript
float get_clear_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fit_margin {#get_fit_margin}

```gdscript
float get_fit_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_mipmaps {#is_using_mipmaps}

```gdscript
bool is_using_mipmaps() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_clear_margin {#set_clear_margin}

```gdscript
void set_clear_margin(clear_margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fit_margin {#set_fit_margin}

```gdscript
void set_fit_margin(fit_margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_mipmaps {#set_use_mipmaps}

```gdscript
void set_use_mipmaps(use_mipmaps: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### fit_margin {#fit_margin}

```gdscript
fit_margin: float
```

- void set_fit_margin(value: float)
- float get_fit_margin()

Sets the size of a margin used to expand the drawable rect of this CanvasGroup. The size of the CanvasGroup is determined by fitting a rect around its children then expanding that rect by fit_margin. This increases both the backbuffer area used and the area covered by the CanvasGroup both of which can reduce performance. This should be kept as small as possible and should only be expanded when an increased size is needed (e.g. for custom shader effects).

### clear_margin {#clear_margin}

```gdscript
clear_margin: float
```

- void set_clear_margin(value: float)
- float get_clear_margin()

Sets the size of the margin used to expand the clearing rect of this CanvasGroup. This expands the area of the backbuffer that will be used by the CanvasGroup. A smaller margin will reduce the area of the backbuffer used which can increase performance, however if use_mipmaps is enabled, a small margin may result in mipmap errors at the edge of the CanvasGroup. Accordingly, this should be left as small as possible, but should be increased if artifacts appear along the edges of the canvas group.

### use_mipmaps {#use_mipmaps}

```gdscript
use_mipmaps: bool
```

- void set_use_mipmaps(value: bool)
- bool is_using_mipmaps()

If true, calculates mipmaps for the backbuffer before drawing the CanvasGroup so that mipmaps can be used in a custom ShaderMaterial attached to the CanvasGroup. Generating mipmaps has a performance cost so this should not be enabled unless required.
