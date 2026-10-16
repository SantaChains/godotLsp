# RenderSceneBuffersExtension

**继承**：[RenderSceneBuffers](class_renderscenebuffers.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **RenderSceneBuffersExtension**

> This class allows for a RenderSceneBuffer implementation to be made in GDExtension.

<details><summary>官方英文描述</summary>

This class allows for a RenderSceneBuffer implementation to be made in GDExtension.

</details>

## 方法

### _configure {#_configure}

```gdscript
void _configure(config: RenderSceneBuffersConfiguration) virtual
```

Implement this in GDExtension to handle the (re)sizing of a viewport.

### _set_anisotropic_filtering_level {#_set_anisotropic_filtering_level}

```gdscript
void _set_anisotropic_filtering_level(anisotropic_filtering_level: int) virtual
```

Implement this in GDExtension to change the anisotropic filtering level.

### _set_fsr_sharpness {#_set_fsr_sharpness}

```gdscript
void _set_fsr_sharpness(fsr_sharpness: float) virtual
```

Implement this in GDExtension to record a new FSR sharpness value.

### _set_texture_mipmap_bias {#_set_texture_mipmap_bias}

```gdscript
void _set_texture_mipmap_bias(texture_mipmap_bias: float) virtual
```

Implement this in GDExtension to change the texture mipmap bias.

### _set_use_debanding {#_set_use_debanding}

```gdscript
void _set_use_debanding(use_debanding: bool) virtual
```

Implement this in GDExtension to react to the debanding flag changing.
