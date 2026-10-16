# GLTFMesh

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFMesh**

> GLTFMesh handles 3D mesh data imported from glTF files. It includes properties for blend channels, blend weights, instance materials, and the mesh itself.

<details><summary>官方英文描述（中文精译进行中）</summary>

GLTFMesh handles 3D mesh data imported from glTF files. It includes properties for blend channels, blend weights, instance materials, and the mesh itself.

</details>

## 方法

### get_additional_data {#get_additional_data}

```gdscript
Variant get_additional_data(extension_name: StringName)
```

Gets additional arbitrary data in this GLTFMesh instance. This can be used to keep per-node state data in GLTFDocumentExtension classes, which is important because they are stateless.

The argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the return value can be anything you set. If nothing was set, the return value is null.

### get_blend_weights {#get_blend_weights}

```gdscript
PackedFloat32Array get_blend_weights()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_instance_materials {#get_instance_materials}

```gdscript
typedarray::Material get_instance_materials()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh {#get_mesh}

```gdscript
ImporterMesh get_mesh()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_original_name {#get_original_name}

```gdscript
String get_original_name()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_additional_data {#set_additional_data}

```gdscript
void set_additional_data(extension_name: StringName, additional_data: Variant)
```

Sets additional arbitrary data in this GLTFMesh instance. This can be used to keep per-node state data in GLTFDocumentExtension classes, which is important because they are stateless.

The first argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the second argument can be anything you want.

### set_blend_weights {#set_blend_weights}

```gdscript
void set_blend_weights(blend_weights: PackedFloat32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_instance_materials {#set_instance_materials}

```gdscript
void set_instance_materials(instance_materials: typedarray::Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: ImporterMesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_original_name {#set_original_name}

```gdscript
void set_original_name(original_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### original_name {#original_name}

```gdscript
original_name: String
```

- void set_original_name(value: String)
- String get_original_name()

The original name of the mesh.

### mesh {#mesh}

```gdscript
mesh: Object
```

- void set_mesh(value: ImporterMesh)
- ImporterMesh get_mesh()

The ImporterMesh object representing the mesh itself.

### blend_weights {#blend_weights}

```gdscript
blend_weights: PackedFloat32Array
```

- void set_blend_weights(value: PackedFloat32Array)
- PackedFloat32Array get_blend_weights()

An array of floats representing the blend weights of the mesh.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedFloat32Array for more details.

### instance_materials {#instance_materials}

```gdscript
instance_materials: Array
```

- void set_instance_materials(value: Array[Material])
- Array[Material] get_instance_materials()

An array of Material objects representing the materials used in the mesh.
