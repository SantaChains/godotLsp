# MeshLibrary

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MeshLibrary**

> A library of meshes. Contains a list of Mesh resources, each with a name and ID. Each item can also include collision and navigation shapes. This resource is used in GridMap.

<details><summary>官方英文描述</summary>

A library of meshes. Contains a list of Mesh resources, each with a name and ID. Each item can also include collision and navigation shapes. This resource is used in GridMap.

</details>

## 方法

### clear {#clear}

```gdscript
void clear()
```

Clears the library.

### create_item {#create_item}

```gdscript
void create_item(id: int)
```

Creates a new item in the library with the given ID.

You can get an unused ID from get_last_unused_item_id().

### find_item_by_name {#find_item_by_name}

```gdscript
int find_item_by_name(name: String) const
```

Returns the first item with the given name, or -1 if no item is found.

### get_item_count {#get_item_count}

```gdscript
int get_item_count() const
```

Returns the number of items present in the library.

### get_item_list {#get_item_list}

```gdscript
PackedInt32Array get_item_list() const
```

Returns the list of item IDs in use.

### get_item_mesh {#get_item_mesh}

```gdscript
Mesh get_item_mesh(id: int) const
```

Returns the item's mesh.

### get_item_mesh_cast_shadow {#get_item_mesh_cast_shadow}

```gdscript
RenderingServer.ShadowCastingSetting get_item_mesh_cast_shadow(id: int) const
```

Returns the item's shadow casting mode.

### get_item_mesh_transform {#get_item_mesh_transform}

```gdscript
Transform3D get_item_mesh_transform(id: int) const
```

Returns the transform applied to the item's mesh.

### get_item_name {#get_item_name}

```gdscript
String get_item_name(id: int) const
```

Returns the item's name.

### get_item_navigation_layers {#get_item_navigation_layers}

```gdscript
int get_item_navigation_layers(id: int) const
```

Returns the item's navigation layers bitmask.

### get_item_navigation_mesh {#get_item_navigation_mesh}

```gdscript
NavigationMesh get_item_navigation_mesh(id: int) const
```

Returns the item's navigation mesh.

### get_item_navigation_mesh_transform {#get_item_navigation_mesh_transform}

```gdscript
Transform3D get_item_navigation_mesh_transform(id: int) const
```

Returns the transform applied to the item's navigation mesh.

### get_item_preview {#get_item_preview}

```gdscript
Texture2D get_item_preview(id: int) const
```

When running in the editor, returns a generated item preview (a 3D rendering in isometric perspective). When used in a running project, returns the manually-defined item preview which can be set using set_item_preview(). Returns an empty Texture2D if no preview was manually set in a running project.

### get_item_shapes {#get_item_shapes}

```gdscript
Array get_item_shapes(id: int) const
```

Returns an item's collision shapes.

The array consists of each Shape3D followed by its Transform3D.

### get_last_unused_item_id {#get_last_unused_item_id}

```gdscript
int get_last_unused_item_id() const
```

Gets an unused ID for a new item.

### remove_item {#remove_item}

```gdscript
void remove_item(id: int)
```

Removes the item.

### set_item_mesh {#set_item_mesh}

```gdscript
void set_item_mesh(id: int, mesh: Mesh)
```

Sets the item's mesh.

### set_item_mesh_cast_shadow {#set_item_mesh_cast_shadow}

```gdscript
void set_item_mesh_cast_shadow(id: int, shadow_casting_setting: RenderingServer.ShadowCastingSetting)
```

Sets the item's shadow casting mode to shadow_casting_setting.

### set_item_mesh_transform {#set_item_mesh_transform}

```gdscript
void set_item_mesh_transform(id: int, mesh_transform: Transform3D)
```

Sets the transform to apply to the item's mesh.

### set_item_name {#set_item_name}

```gdscript
void set_item_name(id: int, name: String)
```

Sets the item's name.

This name is shown in the editor. It can also be used to look up the item later using find_item_by_name().

### set_item_navigation_layers {#set_item_navigation_layers}

```gdscript
void set_item_navigation_layers(id: int, navigation_layers: int)
```

Sets the item's navigation layers bitmask.

### set_item_navigation_mesh {#set_item_navigation_mesh}

```gdscript
void set_item_navigation_mesh(id: int, navigation_mesh: NavigationMesh)
```

Sets the item's navigation mesh.

### set_item_navigation_mesh_transform {#set_item_navigation_mesh_transform}

```gdscript
void set_item_navigation_mesh_transform(id: int, navigation_mesh: Transform3D)
```

Sets the transform to apply to the item's navigation mesh.

### set_item_preview {#set_item_preview}

```gdscript
void set_item_preview(id: int, texture: Texture2D)
```

Sets a texture to use as the item's preview icon in the editor.

### set_item_shapes {#set_item_shapes}

```gdscript
void set_item_shapes(id: int, shapes: Array)
```

Sets an item's collision shapes.

The array should consist of Shape3D objects, each followed by a Transform3D that will be applied to it. For shapes that should not have a transform, use Transform3D.IDENTITY.
