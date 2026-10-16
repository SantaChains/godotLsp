# ShaderMaterial

**继承**：[Material](class_material.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ShaderMaterial**

> A material that uses a custom Shader program to render visual items (canvas items, meshes, skies, fog), or to process particles. Compared to other materials, ShaderMaterial gives deeper control over the generated shader code. For more information, see the shaders documentation index below.

<details><summary>官方英文描述</summary>

A material that uses a custom Shader program to render visual items (canvas items, meshes, skies, fog), or to process particles. Compared to other materials, ShaderMaterial gives deeper control over the generated shader code. For more information, see the shaders documentation index below.

Multiple ShaderMaterials can use the same shader and configure different values for the shader uniforms.

Note: For performance reasons, the Resource.changed signal is only emitted when the Resource.resource_name changes. Only in editor, it is also emitted for shader changes.

</details>

## 方法

### get_shader {#get_shader}

```gdscript
Shader get_shader() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shader_parameter {#get_shader_parameter}

```gdscript
Variant get_shader_parameter(param: StringName) const
```

Returns the current value set for this material of a uniform in the shader.

### set_shader {#set_shader}

```gdscript
void set_shader(shader: Shader)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shader_parameter {#set_shader_parameter}

```gdscript
void set_shader_parameter(param: StringName, value: Variant)
```

Changes the value set for this material of a uniform in the shader.

Note: param is case-sensitive and must match the name of the uniform in the code exactly (not the capitalized name in the inspector).

Note: Changes to the shader uniform will be effective on all instances using this ShaderMaterial. To prevent this, use per-instance uniforms with CanvasItem.set_instance_shader_parameter(), GeometryInstance3D.set_instance_shader_parameter() or duplicate the ShaderMaterial resource using Resource.duplicate(). Per-instance uniforms allow for better shader reuse and are therefore faster, so they should be preferred over duplicating the ShaderMaterial when possible.

## 属性

### shader {#shader}

```gdscript
shader: Shader
```

- void set_shader(value: Shader)
- Shader get_shader()

The Shader program used to render this material.
