# GLTFTextureSampler

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFTextureSampler**

> Represents a texture sampler as defined by the base glTF spec. Texture samplers in glTF specify how to sample data from the texture's base image, when rendering the texture on an object.

<details><summary>官方英文描述（中文精译进行中）</summary>

Represents a texture sampler as defined by the base glTF spec. Texture samplers in glTF specify how to sample data from the texture's base image, when rendering the texture on an object.

</details>

## 方法

### get_mag_filter {#get_mag_filter}

```gdscript
int get_mag_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_filter {#get_min_filter}

```gdscript
int get_min_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wrap_s {#get_wrap_s}

```gdscript
int get_wrap_s() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wrap_t {#get_wrap_t}

```gdscript
int get_wrap_t() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mag_filter {#set_mag_filter}

```gdscript
void set_mag_filter(filter_mode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_filter {#set_min_filter}

```gdscript
void set_min_filter(filter_mode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wrap_s {#set_wrap_s}

```gdscript
void set_wrap_s(wrap_mode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wrap_t {#set_wrap_t}

```gdscript
void set_wrap_t(wrap_mode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mag_filter {#mag_filter}

```gdscript
mag_filter: int
```

- void set_mag_filter(value: int)
- int get_mag_filter()

Texture's magnification filter, used when texture appears larger on screen than the source image.

### min_filter {#min_filter}

```gdscript
min_filter: int
```

- void set_min_filter(value: int)
- int get_min_filter()

Texture's minification filter, used when the texture appears smaller on screen than the source image.

### wrap_s {#wrap_s}

```gdscript
wrap_s: int
```

- void set_wrap_s(value: int)
- int get_wrap_s()

Wrapping mode to use for S-axis (horizontal) texture coordinates.

### wrap_t {#wrap_t}

```gdscript
wrap_t: int
```

- void set_wrap_t(value: int)
- int get_wrap_t()

Wrapping mode to use for T-axis (vertical) texture coordinates.
