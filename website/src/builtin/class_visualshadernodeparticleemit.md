# VisualShaderNodeParticleEmit

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeParticleEmit**

> This node internally calls emit_subparticle shader method. It will emit a particle from the configured sub-emitter and also allows to customize how its emitted. Requires a sub-emitter assigned to the particles node with this shader.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node internally calls emit_subparticle shader method. It will emit a particle from the configured sub-emitter and also allows to customize how its emitted. Requires a sub-emitter assigned to the particles node with this shader.

</details>

## 方法

### get_flags {#get_flags}

```gdscript
VisualShaderNodeParticleEmit.EmitFlags get_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flags {#set_flags}

```gdscript
void set_flags(flags: VisualShaderNodeParticleEmit.EmitFlags)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### flags {#flags}

```gdscript
flags: int
```

- void set_flags(value: EmitFlags)
- EmitFlags get_flags()

Flags used to override the properties defined in the sub-emitter's process material.

## 枚举

### EmitFlags {#enum-EmitFlags}

```gdscript
enum EmitFlags { EMIT_FLAG_POSITION = 1, EMIT_FLAG_ROT_SCALE = 2, EMIT_FLAG_VELOCITY = 4, EMIT_FLAG_COLOR = 8, EMIT_FLAG_CUSTOM = 16 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EMIT_FLAG_POSITION {#EMIT_FLAG_POSITION}

```gdscript
EMIT_FLAG_POSITION = 1
```

If enabled, the particle starts with the position defined by this node.

#### EMIT_FLAG_ROT_SCALE {#EMIT_FLAG_ROT_SCALE}

```gdscript
EMIT_FLAG_ROT_SCALE = 2
```

If enabled, the particle starts with the rotation and scale defined by this node.

#### EMIT_FLAG_VELOCITY {#EMIT_FLAG_VELOCITY}

```gdscript
EMIT_FLAG_VELOCITY = 4
```

If enabled,the particle starts with the velocity defined by this node.

#### EMIT_FLAG_COLOR {#EMIT_FLAG_COLOR}

```gdscript
EMIT_FLAG_COLOR = 8
```

If enabled, the particle starts with the color defined by this node.

#### EMIT_FLAG_CUSTOM {#EMIT_FLAG_CUSTOM}

```gdscript
EMIT_FLAG_CUSTOM = 16
```

If enabled, the particle starts with the CUSTOM data defined by this node.

---
