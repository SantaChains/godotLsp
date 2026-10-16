# VisualShaderNodeParticleMeshEmitter

**继承**：[VisualShaderNodeParticleEmitter](class_visualshadernodeparticleemitter.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeParticleMeshEmitter**

> VisualShaderNodeParticleEmitter that makes the particles emitted in a shape of the assigned mesh. It will emit from the mesh's surfaces, either all or only the specified one.

<details><summary>官方英文描述（中文精译进行中）</summary>

VisualShaderNodeParticleEmitter that makes the particles emitted in a shape of the assigned mesh. It will emit from the mesh's surfaces, either all or only the specified one.

</details>

## 方法

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_surface_index {#get_surface_index}

```gdscript
int get_surface_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_use_all_surfaces {#is_use_all_surfaces}

```gdscript
bool is_use_all_surfaces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: Mesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_surface_index {#set_surface_index}

```gdscript
void set_surface_index(surface_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_all_surfaces {#set_use_all_surfaces}

```gdscript
void set_use_all_surfaces(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mesh {#mesh}

```gdscript
mesh: Mesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

The Mesh that defines emission shape.

### use_all_surfaces {#use_all_surfaces}

```gdscript
use_all_surfaces: bool
```

- void set_use_all_surfaces(value: bool)
- bool is_use_all_surfaces()

If true, the particles will emit from all surfaces of the mesh.

### surface_index {#surface_index}

```gdscript
surface_index: int
```

- void set_surface_index(value: int)
- int get_surface_index()

Index of the surface that emits particles. use_all_surfaces must be false for this to take effect.
