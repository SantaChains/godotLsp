# ShaderIncludeDB

**继承**：[Object](class_object.md) → **ShaderIncludeDB**

> This object contains shader fragments from Godot's internal shaders. These can be used when access to internal uniform buffers and/or internal functions is required for instance when composing compositor effects or compute shaders. Only fragments for the current rendering device are loaded.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object contains shader fragments from Godot's internal shaders. These can be used when access to internal uniform buffers and/or internal functions is required for instance when composing compositor effects or compute shaders. Only fragments for the current rendering device are loaded.

</details>

## 方法

### get_built_in_include_file {#get_built_in_include_file}

```gdscript
static String get_built_in_include_file(filename: String)
```

Returns the code for the built-in shader fragment. You can also access this in your shader code through #include "filename".

### has_built_in_include_file {#has_built_in_include_file}

```gdscript
static bool has_built_in_include_file(filename: String)
```

Returns true if an include file with this name exists.

### list_built_in_include_files {#list_built_in_include_files}

```gdscript
static PackedStringArray list_built_in_include_files()
```

Returns a list of built-in include files that are currently registered.
