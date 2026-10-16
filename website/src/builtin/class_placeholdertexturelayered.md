# PlaceholderTextureLayered

**继承**：[TextureLayered](class_texturelayered.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PlaceholderTextureLayered**

> This class is used when loading a project that uses a TextureLayered subclass in 2 conditions:

<details><summary>官方英文描述（中文精译进行中）</summary>

This class is used when loading a project that uses a TextureLayered subclass in 2 conditions:

- When running the project exported in dedicated server mode, only the texture's dimensions are kept (as they may be relied upon for gameplay purposes or positioning of other elements). This allows reducing the exported PCK's size significantly.
- When this subclass is missing due to using a different engine version or build (e.g. modules disabled).

Note: This is not intended to be used as an actual texture for rendering. It is not guaranteed to work like one in shaders or materials (for example when calculating UV).

</details>

## 方法

### get_size {#get_size}

```gdscript
Vector2i get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layers {#set_layers}

```gdscript
void set_layers(layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector2i
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### layers {#layers}

```gdscript
layers: int
```

- void set_layers(value: int)
- int get_layers()

The number of layers in the texture array.
