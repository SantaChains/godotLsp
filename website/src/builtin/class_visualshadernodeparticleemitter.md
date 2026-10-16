# VisualShaderNodeParticleEmitter

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeParticleEmitter**

> Particle emitter nodes can be used in "start" step of particle shaders and they define the starting position of the particles. Connect them to the Position output port.

<details><summary>官方英文描述（中文精译进行中）</summary>

Particle emitter nodes can be used in "start" step of particle shaders and they define the starting position of the particles. Connect them to the Position output port.

</details>

## 方法

### is_mode_2d {#is_mode_2d}

```gdscript
bool is_mode_2d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mode_2d {#set_mode_2d}

```gdscript
void set_mode_2d(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mode_2d {#mode_2d}

```gdscript
mode_2d: bool
```

- void set_mode_2d(value: bool)
- bool is_mode_2d()

If true, the result of this emitter is projected to 2D space. By default it is false and meant for use in 3D space.
