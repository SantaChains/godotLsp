# FBXState

**继承**：[GLTFState](class_gltfstate.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **FBXState**

> The FBXState handles the state data imported from FBX files.

<details><summary>官方英文描述</summary>

The FBXState handles the state data imported from FBX files.

</details>

## 方法

### get_allow_geometry_helper_nodes {#get_allow_geometry_helper_nodes}

```gdscript
bool get_allow_geometry_helper_nodes()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_geometry_helper_nodes {#set_allow_geometry_helper_nodes}

```gdscript
void set_allow_geometry_helper_nodes(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### allow_geometry_helper_nodes {#allow_geometry_helper_nodes}

```gdscript
allow_geometry_helper_nodes: bool
```

- void set_allow_geometry_helper_nodes(value: bool)
- bool get_allow_geometry_helper_nodes()

If true, the import process used auxiliary nodes called geometry helper nodes. These nodes help preserve the pivots and transformations of the original 3D model during import.
