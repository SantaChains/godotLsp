# VisualShaderNodeParticleAccelerator

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeParticleAccelerator**

> Particle accelerator can be used in "process" step of particle shader. It will accelerate the particles. Connect it to the Velocity output port.

<details><summary>官方英文描述</summary>

Particle accelerator can be used in "process" step of particle shader. It will accelerate the particles. Connect it to the Velocity output port.

</details>

## 方法

### get_mode {#get_mode}

```gdscript
VisualShaderNodeParticleAccelerator.Mode get_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mode {#set_mode}

```gdscript
void set_mode(mode: VisualShaderNodeParticleAccelerator.Mode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mode {#mode}

```gdscript
mode: int
```

- void set_mode(value: Mode)
- Mode get_mode()

Defines in what manner the particles will be accelerated.

## 枚举

### Mode {#enum-Mode}

```gdscript
enum Mode { MODE_LINEAR = 0, MODE_RADIAL = 1, MODE_TANGENTIAL = 2, MODE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_LINEAR {#MODE_LINEAR}

```gdscript
MODE_LINEAR = 0
```

The particles will be accelerated based on their velocity.

#### MODE_RADIAL {#MODE_RADIAL}

```gdscript
MODE_RADIAL = 1
```

The particles will be accelerated towards or away from the center.

#### MODE_TANGENTIAL {#MODE_TANGENTIAL}

```gdscript
MODE_TANGENTIAL = 2
```

The particles will be accelerated tangentially to the radius vector from center to their position.

#### MODE_MAX {#MODE_MAX}

```gdscript
MODE_MAX = 3
```

Represents the size of the Mode enum.

---
