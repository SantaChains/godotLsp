# GPUParticlesCollisionBox3D

**继承**：[GPUParticlesCollision3D](class_gpuparticlescollision3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **GPUParticlesCollisionBox3D**

> A box-shaped 3D particle collision shape affecting GPUParticles3D nodes.

<details><summary>官方英文描述</summary>

A box-shaped 3D particle collision shape affecting GPUParticles3D nodes.

Particle collision shapes work in real-time and can be moved, rotated and scaled during gameplay. Unlike attractors, non-uniform scaling of collision shapes is not supported.

Note: ParticleProcessMaterial.collision_mode must be ParticleProcessMaterial.COLLISION_RIGID or ParticleProcessMaterial.COLLISION_HIDE_ON_CONTACT on the GPUParticles3D's process material for collision to work.

Note: Particle collision only affects GPUParticles3D, not CPUParticles3D.

</details>

## 方法

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector3
```

- void set_size(value: Vector3)
- Vector3 get_size()

The collision box's size in 3D units.
