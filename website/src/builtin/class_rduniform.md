# RDUniform

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDUniform**

> This object is used by RenderingDevice.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object is used by RenderingDevice.

</details>

## 方法

### add_id {#add_id}

```gdscript
void add_id(id: RID)
```

Binds the given id to the uniform. The data associated with the id is then used when the uniform is passed to a shader.

### clear_ids {#clear_ids}

```gdscript
void clear_ids()
```

Unbinds all ids currently bound to the uniform.

### get_binding {#get_binding}

```gdscript
int get_binding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ids {#get_ids}

```gdscript
typedarray::RID get_ids() const
```

Returns an array of all ids currently bound to the uniform.

### get_uniform_type {#get_uniform_type}

```gdscript
RenderingDevice.UniformType get_uniform_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_binding {#set_binding}

```gdscript
void set_binding(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_uniform_type {#set_uniform_type}

```gdscript
void set_uniform_type(p_member: RenderingDevice.UniformType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### uniform_type {#uniform_type}

```gdscript
uniform_type: int
```

- void set_uniform_type(value: UniformType)
- UniformType get_uniform_type()

The uniform's data type.

### binding {#binding}

```gdscript
binding: int
```

- void set_binding(value: int)
- int get_binding()

The uniform's binding.
