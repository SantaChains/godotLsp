# ShaderInclude

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ShaderInclude**

> A shader include file, saved with the .gdshaderinc extension. This class allows you to define a custom shader snippet that can be included in a Shader by using the preprocessor directive #include, followed by the file path (e.g. #include "res://shader_lib.gdshaderinc"). The snippet doesn't have to be a valid shader on its own.

<details><summary>官方英文描述</summary>

A shader include file, saved with the .gdshaderinc extension. This class allows you to define a custom shader snippet that can be included in a Shader by using the preprocessor directive #include, followed by the file path (e.g. #include "res://shader_lib.gdshaderinc"). The snippet doesn't have to be a valid shader on its own.

</details>

## 方法

### get_code {#get_code}

```gdscript
String get_code() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_code {#set_code}

```gdscript
void set_code(code: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### code {#code}

```gdscript
code: String
```

- void set_code(value: String)
- String get_code()

Returns the code of the shader include file. The returned text is what the user has written, not the full generated code used internally.
