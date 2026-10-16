# VisualShaderNodeSample3D

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeSample3D**

> A virtual class, use the descendants instead.

<details><summary>官方英文描述（中文精译进行中）</summary>

A virtual class, use the descendants instead.

</details>

## 方法

### get_source {#get_source}

```gdscript
VisualShaderNodeSample3D.Source get_source() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_source {#set_source}

```gdscript
void set_source(value: VisualShaderNodeSample3D.Source)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### source {#source}

```gdscript
source: int
```

- void set_source(value: Source)
- Source get_source()

An input source type.

## 枚举

### Source {#enum-Source}

```gdscript
enum Source { SOURCE_TEXTURE = 0, SOURCE_PORT = 1, SOURCE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SOURCE_TEXTURE {#SOURCE_TEXTURE}

```gdscript
SOURCE_TEXTURE = 0
```

Creates internal uniform and provides a way to assign it within node.

#### SOURCE_PORT {#SOURCE_PORT}

```gdscript
SOURCE_PORT = 1
```

Use the uniform texture from sampler port.

#### SOURCE_MAX {#SOURCE_MAX}

```gdscript
SOURCE_MAX = 2
```

Represents the size of the Source enum.

---
