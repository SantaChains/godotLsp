# EditorResourcePreview

**继承**：[Node](class_node.md) → [Object](class_object.md) → **EditorResourcePreview**

> This node is used to generate previews for resources or files.

<details><summary>官方英文描述</summary>

This node is used to generate previews for resources or files.

Note: This class shouldn't be instantiated directly. Instead, access the singleton using EditorInterface.get_resource_previewer().

</details>

## 方法

### add_preview_generator {#add_preview_generator}

```gdscript
void add_preview_generator(generator: EditorResourcePreviewGenerator)
```

Create an own, custom preview generator.

### check_for_invalidation {#check_for_invalidation}

```gdscript
void check_for_invalidation(path: String)
```

Check if the resource changed, if so, it will be invalidated and the corresponding signal emitted.

### queue_edited_resource_preview {#queue_edited_resource_preview}

```gdscript
void queue_edited_resource_preview(resource: Resource, receiver: Object, receiver_func: StringName, userdata: Variant)
```

Queue the resource being edited for preview. Once the preview is ready, the receiver's receiver_func will be called. The receiver_func must take the following four arguments: String path, Texture2D preview, Texture2D thumbnail_preview, Variant userdata. userdata can be anything, and will be returned when receiver_func is called.

Note: If it was not possible to create the preview the receiver_func will still be called, but the preview will be null.

### queue_resource_preview {#queue_resource_preview}

```gdscript
void queue_resource_preview(path: String, receiver: Object, receiver_func: StringName, userdata: Variant)
```

Queue a resource file located at path for preview. Once the preview is ready, the receiver's receiver_func will be called. The receiver_func must take the following four arguments: String path, Texture2D preview, Texture2D thumbnail_preview, Variant userdata. userdata can be anything, and will be returned when receiver_func is called.

Note: If it was not possible to create the preview the receiver_func will still be called, but the preview will be null.

### remove_preview_generator {#remove_preview_generator}

```gdscript
void remove_preview_generator(generator: EditorResourcePreviewGenerator)
```

Removes a custom preview generator.

## 信号

### preview_invalidated {#preview_invalidated}

```gdscript
signal preview_invalidated(path: String)
```

Emitted if a preview was invalidated (changed). path corresponds to the path of the preview.
