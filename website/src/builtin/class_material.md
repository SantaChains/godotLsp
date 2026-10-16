# Material

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Material**

> Material is a base resource used for coloring and shading geometry. All materials inherit from it and almost all VisualInstance3D derived nodes carry a Material. A few flags and parameters are shared between all material types and are configured here.

<details><summary>官方英文描述</summary>

Material is a base resource used for coloring and shading geometry. All materials inherit from it and almost all VisualInstance3D derived nodes carry a Material. A few flags and parameters are shared between all material types and are configured here.

Importantly, you can inherit from Material to create your own custom material type in script or in GDExtension.

</details>

## 方法

### _can_do_next_pass {#_can_do_next_pass}

```gdscript
bool _can_do_next_pass() virtual const
```

Only exposed for the purpose of overriding. You cannot call this function directly. Used internally to determine if next_pass should be shown in the editor or not.

### _can_use_render_priority {#_can_use_render_priority}

```gdscript
bool _can_use_render_priority() virtual const
```

Only exposed for the purpose of overriding. You cannot call this function directly. Used internally to determine if render_priority should be shown in the editor or not.

### _get_shader_mode {#_get_shader_mode}

```gdscript
Shader.Mode _get_shader_mode() virtual const
```

Only exposed for the purpose of overriding. You cannot call this function directly. Used internally by various editor tools.

### _get_shader_rid {#_get_shader_rid}

```gdscript
RID _get_shader_rid() virtual const
```

Only exposed for the purpose of overriding. You cannot call this function directly. Used internally by various editor tools. Used to access the RID of the Material's Shader.

### create_placeholder {#create_placeholder}

```gdscript
Resource create_placeholder() const
```

Creates a placeholder version of this resource (PlaceholderMaterial).

### get_next_pass {#get_next_pass}

```gdscript
Material get_next_pass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_render_priority {#get_render_priority}

```gdscript
int get_render_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### inspect_native_shader_code {#inspect_native_shader_code}

```gdscript
void inspect_native_shader_code()
```

Only available when running in the editor. Opens a popup that visualizes the generated shader code, including all variants and internal shader code. See also Shader.inspect_native_shader_code().

### set_next_pass {#set_next_pass}

```gdscript
void set_next_pass(next_pass: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_render_priority {#set_render_priority}

```gdscript
void set_render_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### render_priority {#render_priority}

```gdscript
render_priority: int
```

- void set_render_priority(value: int)
- int get_render_priority()

Sets the render priority for objects in 3D scenes. Higher priority objects will be sorted in front of lower priority objects. In other words, all objects with render_priority 1 will render on top of all objects with render_priority 0.

Note: This only applies to StandardMaterial3Ds and ShaderMaterials with type "Spatial".

Note: This will not impact how transparent objects are sorted relative to opaque objects or how dynamic meshes will be sorted relative to other opaque meshes. This is because all transparent objects are drawn after all opaque objects and all dynamic opaque meshes are drawn before other opaque meshes.

### next_pass {#next_pass}

```gdscript
next_pass: Material
```

- void set_next_pass(value: Material)
- Material get_next_pass()

Sets the Material to be used for the next pass. This renders the object again using a different material.

Note: next_pass materials are not necessarily drawn immediately after the source Material. Draw order is determined by material properties, render_priority, and distance to camera.

Note: This only applies to StandardMaterial3Ds and ShaderMaterials with type "Spatial".

## 常量

### RENDER_PRIORITY_MAX {#RENDER_PRIORITY_MAX}

```gdscript
const RENDER_PRIORITY_MAX = 127
```

Maximum value for the render_priority parameter.

RENDER_PRIORITY_MIN = -128 

Minimum value for the render_priority parameter.

### RENDER_PRIORITY_MIN {#RENDER_PRIORITY_MIN}

```gdscript
const RENDER_PRIORITY_MIN = -128
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
