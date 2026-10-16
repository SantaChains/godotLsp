# RDHitGroup

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDHitGroup**

> Defines a hit group for use with RenderingDevice.raytracing_pipeline_create().

<details><summary>官方英文描述（中文精译进行中）</summary>

Defines a hit group for use with RenderingDevice.raytracing_pipeline_create().

A hit group combines shaders that are executed when a ray intersects geometry. It may include a closest-hit shader, any-hit shader, and intersection shader.

Hit groups are referenced by index when populating hit shader binding tables using RenderingDevice.hit_sbt_range_update().

</details>

## 方法

### get_any_hit_shader {#get_any_hit_shader}

```gdscript
RDPipelineShader get_any_hit_shader() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_closest_hit_shader {#get_closest_hit_shader}

```gdscript
RDPipelineShader get_closest_hit_shader() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_intersection_shader {#get_intersection_shader}

```gdscript
RDPipelineShader get_intersection_shader() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_any_hit_shader {#set_any_hit_shader}

```gdscript
void set_any_hit_shader(p_member: RDPipelineShader)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_closest_hit_shader {#set_closest_hit_shader}

```gdscript
void set_closest_hit_shader(p_member: RDPipelineShader)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_intersection_shader {#set_intersection_shader}

```gdscript
void set_intersection_shader(p_member: RDPipelineShader)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### closest_hit_shader {#closest_hit_shader}

```gdscript
closest_hit_shader: Object
```

- void set_closest_hit_shader(value: RDPipelineShader)
- RDPipelineShader get_closest_hit_shader()

Closest-hit shader for this hit group. Executed for the closest intersection. Can be null.

### any_hit_shader {#any_hit_shader}

```gdscript
any_hit_shader: Object
```

- void set_any_hit_shader(value: RDPipelineShader)
- RDPipelineShader get_any_hit_shader()

Any-hit shader for this hit group. Executed for each potential intersection. Can be null.

### intersection_shader {#intersection_shader}

```gdscript
intersection_shader: Object
```

- void set_intersection_shader(value: RDPipelineShader)
- RDPipelineShader get_intersection_shader()

Intersection shader for this hit group. Required for non-triangle geometry. Must be null when using for triangle geometry.
