# RDPipelineSpecializationConstant

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDPipelineSpecializationConstant**

> A specialization constant is a way to create additional variants of shaders without actually increasing the number of shader versions that are compiled. This allows improving performance by reducing the number of shader versions and reducing if branching, while still allowing shaders to be flexible for different use cases.

<details><summary>官方英文描述（中文精译进行中）</summary>

A specialization constant is a way to create additional variants of shaders without actually increasing the number of shader versions that are compiled. This allows improving performance by reducing the number of shader versions and reducing if branching, while still allowing shaders to be flexible for different use cases.

This object is used by RenderingDevice.

</details>

## 方法

### get_constant_id {#get_constant_id}

```gdscript
int get_constant_id() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_value {#get_value}

```gdscript
Variant get_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constant_id {#set_constant_id}

```gdscript
void set_constant_id(constant_id: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_value {#set_value}

```gdscript
void set_value(value: Variant)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### value {#value}

```gdscript
value: Variant
```

- void set_value(value: Variant)
- Variant get_value()

The specialization constant's value. Only bool, int and float types are valid for specialization constants.

### constant_id {#constant_id}

```gdscript
constant_id: int
```

- void set_constant_id(value: int)
- int get_constant_id()

The identifier of the specialization constant. This is a value starting from 0 and that increments for every different specialization constant for a given shader.
