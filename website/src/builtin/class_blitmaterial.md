# BlitMaterial

**继承**：[Material](class_material.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **BlitMaterial**

> A material resource that can be used by DrawableTextures when processing blit calls to draw.

<details><summary>官方英文描述</summary>

A material resource that can be used by DrawableTextures when processing blit calls to draw.

</details>

## 方法

### get_blend_mode {#get_blend_mode}

```gdscript
BlitMaterial.BlendMode get_blend_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_mode {#set_blend_mode}

```gdscript
void set_blend_mode(blend_mode: BlitMaterial.BlendMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### blend_mode {#blend_mode}

```gdscript
blend_mode: int
```

- void set_blend_mode(value: BlendMode)
- BlendMode get_blend_mode()

The manner in which the newly blitted texture is blended with the original DrawableTexture.

## 枚举

### BlendMode {#enum-BlendMode}

```gdscript
enum BlendMode { BLEND_MODE_MIX = 0, BLEND_MODE_ADD = 1, BLEND_MODE_SUB = 2, BLEND_MODE_MUL = 3, BLEND_MODE_DISABLED = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BLEND_MODE_MIX {#BLEND_MODE_MIX}

```gdscript
BLEND_MODE_MIX = 0
```

Mix blending mode. Colors are assumed to be independent of the alpha (opacity) value.

#### BLEND_MODE_ADD {#BLEND_MODE_ADD}

```gdscript
BLEND_MODE_ADD = 1
```

Additive blending mode.

#### BLEND_MODE_SUB {#BLEND_MODE_SUB}

```gdscript
BLEND_MODE_SUB = 2
```

Subtractive blending mode.

#### BLEND_MODE_MUL {#BLEND_MODE_MUL}

```gdscript
BLEND_MODE_MUL = 3
```

Multiplicative blending mode.

#### BLEND_MODE_DISABLED {#BLEND_MODE_DISABLED}

```gdscript
BLEND_MODE_DISABLED = 4
```

No blending mode, direct color copy.

---
