# GPUParticlesAttractorVectorField3D

**继承**：[GPUParticlesAttractor3D](class_gpuparticlesattractor3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **GPUParticlesAttractorVectorField3D**

> A box-shaped attractor with varying directions and strengths defined in it that influences particles from GPUParticles3D nodes.

<details><summary>官方英文描述（中文精译进行中）</summary>

A box-shaped attractor with varying directions and strengths defined in it that influences particles from GPUParticles3D nodes.

Unlike GPUParticlesAttractorBox3D, GPUParticlesAttractorVectorField3D uses a texture to affect attraction strength within the box. This can be used to create complex attraction scenarios where particles travel in different directions depending on their location. This can be useful for weather effects such as sandstorms.

Particle attractors work in real-time and can be moved, rotated and scaled during gameplay. Unlike collision shapes, non-uniform scaling of attractors is also supported.

Note: Particle attractors only affect GPUParticles3D, not CPUParticles3D.

</details>

## 方法

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture3D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector3
```

- void set_size(value: Vector3)
- Vector3 get_size()

The size of the vector field box in 3D units.

### texture {#texture}

```gdscript
texture: Texture3D
```

- void set_texture(value: Texture3D)
- Texture3D get_texture()

The 3D texture to be used. Values are linearly interpolated between the texture's pixels.

Note: To get better performance, the 3D texture's resolution should reflect the size of the attractor. Since particle attraction is usually low-frequency data, the texture can be kept at a low resolution such as 64×64×64.
