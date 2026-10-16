# RDAccelerationStructureInstance

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDAccelerationStructureInstance**

> RDAccelerationStructureInstance describes an instance of a Bottom-Level Acceleration Structure (BLAS) used in the RenderingDevice.tlas_build() method.

<details><summary>官方英文描述（中文精译进行中）</summary>

RDAccelerationStructureInstance describes an instance of a Bottom-Level Acceleration Structure (BLAS) used in the RenderingDevice.tlas_build() method.

</details>

## 方法

### get_blas {#get_blas}

```gdscript
RID get_blas() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_flags {#get_flags}

```gdscript
bitfield::RenderingDevice.AccelerationStructureInstanceFlagBits get_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hit_sbt_range {#get_hit_sbt_range}

```gdscript
int get_hit_sbt_range() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_id {#get_id}

```gdscript
int get_id() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mask {#get_mask}

```gdscript
int get_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform {#get_transform}

```gdscript
Transform3D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blas {#set_blas}

```gdscript
void set_blas(p_member: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flags {#set_flags}

```gdscript
void set_flags(p_member: bitfield::RenderingDevice.AccelerationStructureInstanceFlagBits)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hit_sbt_range {#set_hit_sbt_range}

```gdscript
void set_hit_sbt_range(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_id {#set_id}

```gdscript
void set_id(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mask {#set_mask}

```gdscript
void set_mask(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform {#set_transform}

```gdscript
void set_transform(p_member: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### transform {#transform}

```gdscript
transform: Transform3D
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### id {#id}

```gdscript
id: int
```

- void set_id(value: int)
- int get_id()

Custom instance ID that can be accessed in GLSL using gl_InstanceCustomIndexEXT.

### mask {#mask}

```gdscript
mask: int
```

- void set_mask(value: int)
- int get_mask()

Visibility mask used to control which rays can intersect this instance.

### hit_sbt_range {#hit_sbt_range}

```gdscript
hit_sbt_range: int
```

- void set_hit_sbt_range(value: int)
- int get_hit_sbt_range()

Hit shader binding table range used for this instance, allocated using the RenderingDevice.hit_sbt_range_alloc() method.

### flags {#flags}

```gdscript
flags: int
```

- void set_flags(value: BitField[AccelerationStructureInstanceFlagBits])
- BitField[AccelerationStructureInstanceFlagBits] get_flags()

Flags for the instance.

### blas {#blas}

```gdscript
blas: RID
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
