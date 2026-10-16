# RDPipelineShader

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDPipelineShader**

> Wraps a shader resource and allows specialization constants to be applied at pipeline creation time.

<details><summary>官方英文描述（中文精译进行中）</summary>

Wraps a shader resource and allows specialization constants to be applied at pipeline creation time.

Used by RenderingDevice.raytracing_pipeline_create() for ray generation, miss, and hit shaders. The pipeline selects the required shader stage automatically.

</details>

## 方法

### get_shader {#get_shader}

```gdscript
RID get_shader() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_specialization_constants {#get_specialization_constants}

```gdscript
typedarray::RDPipelineSpecializationConstant get_specialization_constants() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shader {#set_shader}

```gdscript
void set_shader(p_member: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_specialization_constants {#set_specialization_constants}

```gdscript
void set_specialization_constants(specialization_constants: typedarray::RDPipelineSpecializationConstant)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### shader {#shader}

```gdscript
shader: RID
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### specialization_constants {#specialization_constants}

```gdscript
specialization_constants: typedarray::RDPipelineSpecializationConstant
```

- void set_specialization_constants(value: Array[RDPipelineSpecializationConstant])
- Array[RDPipelineSpecializationConstant] get_specialization_constants()

Specialization constants applied to the selected shader stage at pipeline creation time.
