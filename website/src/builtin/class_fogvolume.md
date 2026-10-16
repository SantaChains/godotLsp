# FogVolume

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **FogVolume**

> FogVolumes are used to add localized fog into the global volumetric fog effect. FogVolumes can also remove volumetric fog from specific areas if using a FogMaterial with a negative FogMaterial.density.

<details><summary>官方英文描述（中文精译进行中）</summary>

FogVolumes are used to add localized fog into the global volumetric fog effect. FogVolumes can also remove volumetric fog from specific areas if using a FogMaterial with a negative FogMaterial.density.

Performance of FogVolumes is directly related to their relative size on the screen and the complexity of their attached FogMaterial. It is best to keep FogVolumes relatively small and simple where possible.

Note: FogVolumes only have a visible effect if Environment.volumetric_fog_enabled is true. If you don't want fog to be globally visible (but only within FogVolume nodes), set Environment.volumetric_fog_density to 0.0.

</details>

## 方法

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
RenderingServer.FogVolumeShape get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: RenderingServer.FogVolumeShape)
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

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### shape {#shape}

```gdscript
shape: int
```

- void set_shape(value: FogVolumeShape)
- FogVolumeShape get_shape()

The shape of the FogVolume. This can be set to either RenderingServer.FOG_VOLUME_SHAPE_ELLIPSOID, RenderingServer.FOG_VOLUME_SHAPE_CONE, RenderingServer.FOG_VOLUME_SHAPE_CYLINDER, RenderingServer.FOG_VOLUME_SHAPE_BOX or RenderingServer.FOG_VOLUME_SHAPE_WORLD.

### material {#material}

```gdscript
material: FogMaterial,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The Material used by the FogVolume. Can be either a built-in FogMaterial or a custom ShaderMaterial.
