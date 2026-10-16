# VisualShaderNodeBillboard

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeBillboard**

> The output port of this node needs to be connected to Model View Matrix port of VisualShaderNodeOutput.

<details><summary>官方英文描述（中文精译进行中）</summary>

The output port of this node needs to be connected to Model View Matrix port of VisualShaderNodeOutput.

</details>

## 方法

### get_billboard_type {#get_billboard_type}

```gdscript
VisualShaderNodeBillboard.BillboardType get_billboard_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_keep_scale_enabled {#is_keep_scale_enabled}

```gdscript
bool is_keep_scale_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_billboard_type {#set_billboard_type}

```gdscript
void set_billboard_type(billboard_type: VisualShaderNodeBillboard.BillboardType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_scale_enabled {#set_keep_scale_enabled}

```gdscript
void set_keep_scale_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### billboard_type {#billboard_type}

```gdscript
billboard_type: int
```

- void set_billboard_type(value: BillboardType)
- BillboardType get_billboard_type()

Controls how the object faces the camera.

### keep_scale {#keep_scale}

```gdscript
keep_scale: bool
```

- void set_keep_scale_enabled(value: bool)
- bool is_keep_scale_enabled()

If true, the shader will keep the scale set for the mesh. Otherwise, the scale is lost when billboarding.

## 枚举

### BillboardType {#enum-BillboardType}

```gdscript
enum BillboardType { BILLBOARD_TYPE_DISABLED = 0, BILLBOARD_TYPE_ENABLED = 1, BILLBOARD_TYPE_FIXED_Y = 2, BILLBOARD_TYPE_PARTICLES = 3, BILLBOARD_TYPE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BILLBOARD_TYPE_DISABLED {#BILLBOARD_TYPE_DISABLED}

```gdscript
BILLBOARD_TYPE_DISABLED = 0
```

Billboarding is disabled and the node does nothing.

#### BILLBOARD_TYPE_ENABLED {#BILLBOARD_TYPE_ENABLED}

```gdscript
BILLBOARD_TYPE_ENABLED = 1
```

A standard billboarding algorithm is enabled.

#### BILLBOARD_TYPE_FIXED_Y {#BILLBOARD_TYPE_FIXED_Y}

```gdscript
BILLBOARD_TYPE_FIXED_Y = 2
```

A billboarding algorithm to rotate around Y-axis is enabled.

#### BILLBOARD_TYPE_PARTICLES {#BILLBOARD_TYPE_PARTICLES}

```gdscript
BILLBOARD_TYPE_PARTICLES = 3
```

A billboarding algorithm designed to use on particles is enabled.

#### BILLBOARD_TYPE_MAX {#BILLBOARD_TYPE_MAX}

```gdscript
BILLBOARD_TYPE_MAX = 4
```

Represents the size of the BillboardType enum.

---
