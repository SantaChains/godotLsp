# VisualShaderNodeParameter

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeParameter**

> A parameter represents a variable in the shader which is set externally, i.e. from the ShaderMaterial. Parameters are exposed as properties in the ShaderMaterial and can be assigned from the Inspector or from a script.

<details><summary>官方英文描述（中文精译进行中）</summary>

A parameter represents a variable in the shader which is set externally, i.e. from the ShaderMaterial. Parameters are exposed as properties in the ShaderMaterial and can be assigned from the Inspector or from a script.

</details>

## 方法

### get_instance_index {#get_instance_index}

```gdscript
int get_instance_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parameter_name {#get_parameter_name}

```gdscript
String get_parameter_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_qualifier {#get_qualifier}

```gdscript
VisualShaderNodeParameter.Qualifier get_qualifier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_instance_index {#set_instance_index}

```gdscript
void set_instance_index(instance_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_parameter_name {#set_parameter_name}

```gdscript
void set_parameter_name(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_qualifier {#set_qualifier}

```gdscript
void set_qualifier(qualifier: VisualShaderNodeParameter.Qualifier)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### parameter_name {#parameter_name}

```gdscript
parameter_name: StringName
```

- void set_parameter_name(value: String)
- String get_parameter_name()

Name of the parameter, by which it can be accessed through the ShaderMaterial properties.

### qualifier {#qualifier}

```gdscript
qualifier: int
```

- void set_qualifier(value: Qualifier)
- Qualifier get_qualifier()

Defines the scope of the parameter.

### instance_index {#instance_index}

```gdscript
instance_index: int
```

- void set_instance_index(value: int)
- int get_instance_index()

The index within 0-15 range, which is used to avoid clashes when shader used on multiple materials.

## 枚举

### Qualifier {#enum-Qualifier}

```gdscript
enum Qualifier { QUAL_NONE = 0, QUAL_GLOBAL = 1, QUAL_INSTANCE = 2, QUAL_INSTANCE_INDEX = 3, QUAL_MAX = 4 }
```

Qualifier QUAL_NONE = 0

The parameter will be tied to the ShaderMaterial using this shader.

Qualifier QUAL_GLOBAL = 1

The parameter will use a global value, defined in Project Settings.

Qualifier QUAL_INSTANCE = 2

The parameter will be tied to the node with attached ShaderMaterial using this shader.

Qualifier QUAL_INSTANCE_INDEX = 3

The parameter will be tied to the node with attached ShaderMaterial using this shader. Enables setting a instance_index property.

Qualifier QUAL_MAX = 4

Represents the size of the Qualifier enum.

#### QUAL_NONE {#QUAL_NONE}

```gdscript
QUAL_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### QUAL_GLOBAL {#QUAL_GLOBAL}

```gdscript
QUAL_GLOBAL = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### QUAL_INSTANCE {#QUAL_INSTANCE}

```gdscript
QUAL_INSTANCE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### QUAL_INSTANCE_INDEX {#QUAL_INSTANCE_INDEX}

```gdscript
QUAL_INSTANCE_INDEX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### QUAL_MAX {#QUAL_MAX}

```gdscript
QUAL_MAX = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
