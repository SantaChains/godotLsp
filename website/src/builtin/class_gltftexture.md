# GLTFTexture

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFTexture**

## 方法

### get_sampler {#get_sampler}

```gdscript
int get_sampler() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_src_image {#get_src_image}

```gdscript
int get_src_image() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sampler {#set_sampler}

```gdscript
void set_sampler(sampler: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_src_image {#set_src_image}

```gdscript
void set_src_image(src_image: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### src_image {#src_image}

```gdscript
src_image: int
```

- void set_src_image(value: int)
- int get_src_image()

The index of the image associated with this texture, see GLTFState.get_images(). If -1, then this texture does not have an image assigned.

### sampler {#sampler}

```gdscript
sampler: int
```

- void set_sampler(value: int)
- int get_sampler()

ID of the texture sampler to use when sampling the image. If -1, then the default texture sampler is used (linear filtering, and repeat wrapping in both axes).
