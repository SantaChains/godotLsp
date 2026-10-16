# GLTFSpecGloss

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFSpecGloss**

> KHR_materials_pbrSpecularGlossiness is an archived glTF extension. This means that it is deprecated and not recommended for new files. However, it is still supported for loading old files.

<details><summary>官方英文描述（中文精译进行中）</summary>

KHR_materials_pbrSpecularGlossiness is an archived glTF extension. This means that it is deprecated and not recommended for new files. However, it is still supported for loading old files.

</details>

## 方法

### get_diffuse_factor {#get_diffuse_factor}

```gdscript
Color get_diffuse_factor()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_diffuse_img {#get_diffuse_img}

```gdscript
Image get_diffuse_img()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gloss_factor {#get_gloss_factor}

```gdscript
float get_gloss_factor()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spec_gloss_img {#get_spec_gloss_img}

```gdscript
Image get_spec_gloss_img()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_specular_factor {#get_specular_factor}

```gdscript
Color get_specular_factor()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_diffuse_factor {#set_diffuse_factor}

```gdscript
void set_diffuse_factor(diffuse_factor: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_diffuse_img {#set_diffuse_img}

```gdscript
void set_diffuse_img(diffuse_img: Image)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gloss_factor {#set_gloss_factor}

```gdscript
void set_gloss_factor(gloss_factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spec_gloss_img {#set_spec_gloss_img}

```gdscript
void set_spec_gloss_img(spec_gloss_img: Image)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_specular_factor {#set_specular_factor}

```gdscript
void set_specular_factor(specular_factor: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### diffuse_img {#diffuse_img}

```gdscript
diffuse_img: Object
```

- void set_diffuse_img(value: Image)
- Image get_diffuse_img()

The diffuse texture.

### diffuse_factor {#diffuse_factor}

```gdscript
diffuse_factor: Color
```

- void set_diffuse_factor(value: Color)
- Color get_diffuse_factor()

The reflected diffuse factor of the material.

### gloss_factor {#gloss_factor}

```gdscript
gloss_factor: float
```

- void set_gloss_factor(value: float)
- float get_gloss_factor()

The glossiness or smoothness of the material.

### specular_factor {#specular_factor}

```gdscript
specular_factor: Color
```

- void set_specular_factor(value: Color)
- Color get_specular_factor()

The specular RGB color of the material. The alpha channel is unused.

### spec_gloss_img {#spec_gloss_img}

```gdscript
spec_gloss_img: Object
```

- void set_spec_gloss_img(value: Image)
- Image get_spec_gloss_img()

The specular-glossiness texture.
