# GLTFState

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFState**

> Contains all nodes and resources of a glTF file. This is used by GLTFDocument as data storage, which allows GLTFDocument and all GLTFDocumentExtension classes to remain stateless.

<details><summary>官方英文描述</summary>

Contains all nodes and resources of a glTF file. This is used by GLTFDocument as data storage, which allows GLTFDocument and all GLTFDocumentExtension classes to remain stateless.

GLTFState can be populated by GLTFDocument reading a file or by converting a Godot scene. Then the data can either be used to create a Godot scene or save to a glTF file. The code that converts to/from a Godot scene can be intercepted at arbitrary points by GLTFDocumentExtension classes. This allows for custom data to be stored in the glTF file or for custom data to be converted to/from Godot nodes.

</details>

## 方法

### add_used_extension {#add_used_extension}

```gdscript
void add_used_extension(extension_name: String, required: bool)
```

Appends an extension to the list of extensions used by this glTF file during serialization. If required is true, the extension will also be added to the list of required extensions. Do not run this in GLTFDocumentExtension._export_post(), as that stage is too late to add extensions. The final list is sorted alphabetically.

### append_data_to_buffers {#append_data_to_buffers}

```gdscript
int append_data_to_buffers(data: PackedByteArray, deduplication: bool)
```

Appends the given byte array data to the buffers and creates a GLTFBufferView for it. The index of the destination GLTFBufferView is returned. If deduplication is true, the buffers are first searched for duplicate data, otherwise new bytes are always appended.

### append_gltf_node {#append_gltf_node}

```gdscript
int append_gltf_node(gltf_node: GLTFNode, godot_scene_node: Node, parent_node_index: int)
```

Appends the given GLTFNode to the state, and returns its new index. This can be used to export one Godot node as multiple glTF nodes, or inject new glTF nodes at import time. On import, this must be called before GLTFDocumentExtension._generate_scene_node() finishes for the parent node. On export, this must be called before GLTFDocumentExtension._export_node() runs for the parent node.

The godot_scene_node parameter is the Godot scene node that corresponds to this glTF node. This is highly recommended to be set to a valid node, but may be null if there is no corresponding Godot scene node. One Godot scene node may be used for multiple glTF nodes, so if exporting multiple glTF nodes for one Godot scene node, use the same Godot scene node for each.

The parent_node_index parameter is the index of the parent GLTFNode in the state. If -1, the node will be a root node, otherwise the new node will be added to the parent's list of children. The index will also be written to the GLTFNode.parent property of the new node.

### get_accessors {#get_accessors}

```gdscript
typedarray::GLTFAccessor get_accessors() const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_additional_data {#get_additional_data}

```gdscript
Variant get_additional_data(extension_name: StringName) const
```

Gets additional arbitrary data in this GLTFState instance. This can be used to keep per-file state data in GLTFDocumentExtension classes, which is important because they are stateless.

The argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the return value can be anything you set. If nothing was set, the return value is null.

### get_animation_player {#get_animation_player}

```gdscript
AnimationPlayer get_animation_player(anim_player_index: int) const
```

Returns the AnimationPlayer node with the given index. These nodes are only used during the export process when converting Godot AnimationPlayer nodes to glTF animations.

### get_animation_players_count {#get_animation_players_count}

```gdscript
int get_animation_players_count(anim_player_index: int) const
```

Returns the number of AnimationPlayer nodes in this GLTFState. These nodes are only used during the export process when converting Godot AnimationPlayer nodes to glTF animations.

### get_animations {#get_animations}

```gdscript
typedarray::GLTFAnimation get_animations() const
```

Returns an array of all GLTFAnimations in the glTF file. When importing, these will be generated as animations in an AnimationPlayer node. When exporting, these will be generated from Godot AnimationPlayer nodes.

### get_bake_fps {#get_bake_fps}

```gdscript
float get_bake_fps() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_base_path {#get_base_path}

```gdscript
String get_base_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_buffer_views {#get_buffer_views}

```gdscript
typedarray::GLTFBufferView get_buffer_views() const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_buffers {#get_buffers}

```gdscript
typedarray::PackedByteArray get_buffers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cameras {#get_cameras}

```gdscript
typedarray::GLTFCamera get_cameras() const
```

Returns an array of all GLTFCameras in the glTF file. These are the cameras that the GLTFNode.camera index refers to.

### get_copyright {#get_copyright}

```gdscript
String get_copyright() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_create_animations {#get_create_animations}

```gdscript
bool get_create_animations() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filename {#get_filename}

```gdscript
String get_filename() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_glb_data {#get_glb_data}

```gdscript
PackedByteArray get_glb_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_handle_binary_image {#get_handle_binary_image}

```gdscript
int get_handle_binary_image() const
```

Deprecated: Use handle_binary_image_mode instead.

Deprecated untyped alias for handle_binary_image_mode. When importing a glTF file with unimported raw binary images embedded inside of binary blob buffers, in data URIs, or separate files not imported by Godot, this controls how the images are handled.

### get_handle_binary_image_mode {#get_handle_binary_image_mode}

```gdscript
GLTFState.HandleBinaryImageMode get_handle_binary_image_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_images {#get_images}

```gdscript
typedarray::Texture2D get_images() const
```

Gets the images of the glTF file as an array of Texture2Ds. These are the images that the GLTFTexture.src_image index refers to.

### get_import_as_skeleton_bones {#get_import_as_skeleton_bones}

```gdscript
bool get_import_as_skeleton_bones() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_json {#get_json}

```gdscript
Dictionary get_json() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_lights {#get_lights}

```gdscript
typedarray::GLTFLight get_lights() const
```

Returns an array of all GLTFLights in the glTF file. These are the lights that the GLTFNode.light index refers to.

### get_major_version {#get_major_version}

```gdscript
int get_major_version() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_materials {#get_materials}

```gdscript
typedarray::Material get_materials() const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_meshes {#get_meshes}

```gdscript
typedarray::GLTFMesh get_meshes() const
```

Returns an array of all GLTFMeshes in the glTF file. These are the meshes that the GLTFNode.mesh index refers to.

### get_minor_version {#get_minor_version}

```gdscript
int get_minor_version() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node_index {#get_node_index}

```gdscript
int get_node_index(scene_node: Node) const
```

Returns the index of the GLTFNode corresponding to this Godot scene node. This is the inverse of get_scene_node(). Useful during the export process.

Note: Not every Godot scene node will have a corresponding GLTFNode, and not every GLTFNode will have a scene node generated. If there is no GLTFNode index for this scene node, -1 is returned.

### get_nodes {#get_nodes}

```gdscript
typedarray::GLTFNode get_nodes() const
```

Returns an array of all GLTFNodes in the glTF file. These are the nodes that GLTFNode.children and root_nodes refer to. This includes nodes that may not be generated in the Godot scene, or nodes that may generate multiple Godot scene nodes.

### get_root_nodes {#get_root_nodes}

```gdscript
PackedInt32Array get_root_nodes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scene_name {#get_scene_name}

```gdscript
String get_scene_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scene_node {#get_scene_node}

```gdscript
Node get_scene_node(gltf_node_index: int) const
```

Returns the Godot scene node that corresponds to the same index as the GLTFNode it was generated from. This is the inverse of get_node_index(). Useful during the import process.

Note: Not every GLTFNode will have a scene node generated, and not every generated scene node will have a corresponding GLTFNode. If there is no scene node for this GLTFNode index, null is returned.

### get_skeletons {#get_skeletons}

```gdscript
typedarray::GLTFSkeleton get_skeletons() const
```

Returns an array of all GLTFSkeletons in the glTF file. These are the skeletons that the GLTFNode.skeleton index refers to.

### get_skins {#get_skins}

```gdscript
typedarray::GLTFSkin get_skins() const
```

Returns an array of all GLTFSkins in the glTF file. These are the skins that the GLTFNode.skin index refers to.

### get_texture_samplers {#get_texture_samplers}

```gdscript
typedarray::GLTFTextureSampler get_texture_samplers() const
```

Retrieves the array of texture samplers that are used by the textures contained in the glTF.

### get_textures {#get_textures}

```gdscript
typedarray::GLTFTexture get_textures() const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_unique_animation_names {#get_unique_animation_names}

```gdscript
typedarray::String get_unique_animation_names() const
```

Returns an array of unique animation names. This is only used during the import process.

### get_unique_names {#get_unique_names}

```gdscript
typedarray::String get_unique_names() const
```

Returns an array of unique node names. This is used in both the import process and export process.

### get_use_named_skin_binds {#get_use_named_skin_binds}

```gdscript
bool get_use_named_skin_binds() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_accessors {#set_accessors}

```gdscript
void set_accessors(accessors: typedarray::GLTFAccessor)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_additional_data {#set_additional_data}

```gdscript
void set_additional_data(extension_name: StringName, additional_data: Variant)
```

Sets additional arbitrary data in this GLTFState instance. This can be used to keep per-file state data in GLTFDocumentExtension classes, which is important because they are stateless.

The first argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the second argument can be anything you want.

### set_animations {#set_animations}

```gdscript
void set_animations(animations: typedarray::GLTFAnimation)
```

Sets the GLTFAnimations in the state. When importing, these will be generated as animations in an AnimationPlayer node. When exporting, these will be generated from Godot AnimationPlayer nodes.

### set_bake_fps {#set_bake_fps}

```gdscript
void set_bake_fps(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_base_path {#set_base_path}

```gdscript
void set_base_path(base_path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_buffer_views {#set_buffer_views}

```gdscript
void set_buffer_views(buffer_views: typedarray::GLTFBufferView)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_buffers {#set_buffers}

```gdscript
void set_buffers(buffers: typedarray::PackedByteArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cameras {#set_cameras}

```gdscript
void set_cameras(cameras: typedarray::GLTFCamera)
```

Sets the GLTFCameras in the state. These are the cameras that the GLTFNode.camera index refers to.

### set_copyright {#set_copyright}

```gdscript
void set_copyright(copyright: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_create_animations {#set_create_animations}

```gdscript
void set_create_animations(create_animations: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filename {#set_filename}

```gdscript
void set_filename(filename: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_glb_data {#set_glb_data}

```gdscript
void set_glb_data(glb_data: PackedByteArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_handle_binary_image {#set_handle_binary_image}

```gdscript
void set_handle_binary_image(method: int)
```

Deprecated: Use handle_binary_image_mode instead.

Deprecated untyped alias for handle_binary_image_mode. When importing a glTF file with unimported raw binary images embedded inside of binary blob buffers, in data URIs, or separate files not imported by Godot, this controls how the images are handled.

### set_handle_binary_image_mode {#set_handle_binary_image_mode}

```gdscript
void set_handle_binary_image_mode(method: GLTFState.HandleBinaryImageMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_images {#set_images}

```gdscript
void set_images(images: typedarray::Texture2D)
```

Sets the images in the state stored as an array of Texture2Ds. This can be used during export. These are the images that the GLTFTexture.src_image index refers to.

### set_import_as_skeleton_bones {#set_import_as_skeleton_bones}

```gdscript
void set_import_as_skeleton_bones(import_as_skeleton_bones: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_json {#set_json}

```gdscript
void set_json(json: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lights {#set_lights}

```gdscript
void set_lights(lights: typedarray::GLTFLight)
```

Sets the GLTFLights in the state. These are the lights that the GLTFNode.light index refers to.

### set_major_version {#set_major_version}

```gdscript
void set_major_version(major_version: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_materials {#set_materials}

```gdscript
void set_materials(materials: typedarray::Material)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_meshes {#set_meshes}

```gdscript
void set_meshes(meshes: typedarray::GLTFMesh)
```

Sets the GLTFMeshes in the state. These are the meshes that the GLTFNode.mesh index refers to.

### set_minor_version {#set_minor_version}

```gdscript
void set_minor_version(minor_version: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_nodes {#set_nodes}

```gdscript
void set_nodes(nodes: typedarray::GLTFNode)
```

Sets the GLTFNodes in the state. These are the nodes that GLTFNode.children and root_nodes refer to. Some of the nodes set here may not be generated in the Godot scene, or may generate multiple Godot scene nodes.

### set_root_nodes {#set_root_nodes}

```gdscript
void set_root_nodes(root_nodes: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scene_name {#set_scene_name}

```gdscript
void set_scene_name(scene_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skeletons {#set_skeletons}

```gdscript
void set_skeletons(skeletons: typedarray::GLTFSkeleton)
```

Sets the GLTFSkeletons in the state. These are the skeletons that the GLTFNode.skeleton index refers to.

### set_skins {#set_skins}

```gdscript
void set_skins(skins: typedarray::GLTFSkin)
```

Sets the GLTFSkins in the state. These are the skins that the GLTFNode.skin index refers to.

### set_texture_samplers {#set_texture_samplers}

```gdscript
void set_texture_samplers(texture_samplers: typedarray::GLTFTextureSampler)
```

Sets the array of texture samplers that are used by the textures contained in the glTF.

### set_textures {#set_textures}

```gdscript
void set_textures(textures: typedarray::GLTFTexture)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_unique_animation_names {#set_unique_animation_names}

```gdscript
void set_unique_animation_names(unique_animation_names: typedarray::String)
```

Sets the unique animation names in the state. This is only used during the import process.

### set_unique_names {#set_unique_names}

```gdscript
void set_unique_names(unique_names: typedarray::String)
```

Sets the unique node names in the state. This is used in both the import process and export process.

### set_use_named_skin_binds {#set_use_named_skin_binds}

```gdscript
void set_use_named_skin_binds(use_named_skin_binds: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### json {#json}

```gdscript
json: Dictionary
```

- void set_json(value: Dictionary)
- Dictionary get_json()

The original raw JSON document corresponding to this GLTFState.

### major_version {#major_version}

```gdscript
major_version: int
```

- void set_major_version(value: int)
- int get_major_version()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### minor_version {#minor_version}

```gdscript
minor_version: int
```

- void set_minor_version(value: int)
- int get_minor_version()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### copyright {#copyright}

```gdscript
copyright: String
```

- void set_copyright(value: String)
- String get_copyright()

The copyright string in the asset header of the glTF file. This is set during import if present and export if non-empty. See the glTF asset header documentation for more information.

### glb_data {#glb_data}

```gdscript
glb_data: PackedByteArray
```

- void set_glb_data(value: PackedByteArray)
- PackedByteArray get_glb_data()

The binary buffer attached to a .glb file.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### use_named_skin_binds {#use_named_skin_binds}

```gdscript
use_named_skin_binds: bool
```

- void set_use_named_skin_binds(value: bool)
- bool get_use_named_skin_binds()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### nodes {#nodes}

```gdscript
nodes: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### buffers {#buffers}

```gdscript
buffers: Array
```

- void set_buffers(value: Array[PackedByteArray])
- Array[PackedByteArray] get_buffers()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### buffer_views {#buffer_views}

```gdscript
buffer_views: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### accessors {#accessors}

```gdscript
accessors: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### meshes {#meshes}

```gdscript
meshes: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### materials {#materials}

```gdscript
materials: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scene_name {#scene_name}

```gdscript
scene_name: String
```

- void set_scene_name(value: String)
- String get_scene_name()

The name of the scene. When importing, if not specified, this will be the file name. When exporting, if specified, the scene name will be saved to the glTF file.

### base_path {#base_path}

```gdscript
base_path: String
```

- void set_base_path(value: String)
- String get_base_path()

The folder path associated with this glTF data. This is used to find other files the glTF file references, like images or binary buffers. This will be set during import when appending from a file, and will be set during export when writing to a file.

### filename {#filename}

```gdscript
filename: String
```

- void set_filename(value: String)
- String get_filename()

The file name associated with this glTF data. If it ends with .gltf, this is text-based glTF, otherwise this is binary GLB. This will be set during import when appending from a file, and will be set during export when writing to a file. If writing to a buffer, this will be an empty string.

### root_nodes {#root_nodes}

```gdscript
root_nodes: PackedInt32Array
```

- void set_root_nodes(value: PackedInt32Array)
- PackedInt32Array get_root_nodes()

The root nodes of the glTF file. Typically, a glTF file will only have one scene, and therefore one root node. However, a glTF file may have multiple scenes and therefore multiple root nodes, which will be generated as siblings of each other and as children of the root node of the generated Godot scene.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### textures {#textures}

```gdscript
textures: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### texture_samplers {#texture_samplers}

```gdscript
texture_samplers: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### images {#images}

```gdscript
images: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### skins {#skins}

```gdscript
skins: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### cameras {#cameras}

```gdscript
cameras: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### lights {#lights}

```gdscript
lights: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### unique_names {#unique_names}

```gdscript
unique_names: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### unique_animation_names {#unique_animation_names}

```gdscript
unique_animation_names: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### skeletons {#skeletons}

```gdscript
skeletons: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### create_animations {#create_animations}

```gdscript
create_animations: bool
```

- void set_create_animations(value: bool)
- bool get_create_animations()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### import_as_skeleton_bones {#import_as_skeleton_bones}

```gdscript
import_as_skeleton_bones: bool
```

- void set_import_as_skeleton_bones(value: bool)
- bool get_import_as_skeleton_bones()

If true, forces all GLTFNodes in the document to be bones of a single Skeleton3D Godot node.

### animations {#animations}

```gdscript
animations: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### handle_binary_image_mode {#handle_binary_image_mode}

```gdscript
handle_binary_image_mode: int
```

- void set_handle_binary_image_mode(value: HandleBinaryImageMode)
- HandleBinaryImageMode get_handle_binary_image_mode()

When importing a glTF file with unimported raw binary images embedded inside of binary blob buffers, in data URIs, or separate files not imported by Godot, this controls how the images are handled. Images can be discarded, saved as separate files, or embedded in the scene lossily or losslessly. See HandleBinaryImageMode for options.

This property does nothing for image files in the res:// folder imported by Godot, as those are handled by Godot's image importer directly, and then the Godot scene generated from the glTF file will use the images as Godot imported them.

### bake_fps {#bake_fps}

```gdscript
bake_fps: float
```

- void set_bake_fps(value: float)
- float get_bake_fps()

The baking fps of the animation for either import or export.

### handle_binary_image {#handle_binary_image}

```gdscript
handle_binary_image: int
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### HandleBinaryImageMode {#enum-HandleBinaryImageMode}

```gdscript
enum HandleBinaryImageMode { HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES = 0, HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES = 1, HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU = 2, HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES {#HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES}

```gdscript
HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES = 0
```

When importing a glTF file with embedded binary images, discards all images and uses untextured materials in their place. Images stored as separate files in the res:// folder are not affected by this; those will be used as Godot imported them.

#### HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES {#HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES}

```gdscript
HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES = 1
```

When importing a glTF file with embedded binary images, extracts them and saves them to their own files. This allows the image to be imported by Godot's image importer, which can then have their import options customized by the user, including optionally compressing the image to VRAM texture formats.

This will save the images's bytes exactly as-is, without recompression. For image formats supplied by glTF extensions, the file will have a filename ending with the file extension supplied by GLTFDocumentExtension._get_image_file_extension() of the extension class.

Note: This option is editor-only. At runtime, this acts the same as HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED.

#### HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU {#HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU}

```gdscript
HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU = 2
```

When importing a glTF file with embedded binary images, embeds textures VRAM compressed with Basis Universal into the generated scene. Images stored as separate files in the res:// folder are not affected by this; those will be used as Godot imported them.

#### HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED {#HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED}

```gdscript
HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED = 3
```

When importing a glTF file with embedded binary images, embeds textures compressed losslessly into the generated scene. Images stored as separate files in the res:// folder are not affected by this; those will be used as Godot imported them.

---

## 常量

### HANDLE_BINARY_DISCARD_TEXTURES {#HANDLE_BINARY_DISCARD_TEXTURES}

```gdscript
const HANDLE_BINARY_DISCARD_TEXTURES = 0
```

Deprecated: Use HANDLE_BINARY_IMAGE_MODE_DISCARD_TEXTURES instead.

Discards all embedded textures and uses untextured materials.

HANDLE_BINARY_EXTRACT_TEXTURES = 1 

Deprecated: Use HANDLE_BINARY_IMAGE_MODE_EXTRACT_TEXTURES instead.

Extracts embedded textures to be reimported and compressed. Editor only. Acts as uncompressed at runtime.

HANDLE_BINARY_EMBED_AS_BASISU = 2 

Deprecated: Use HANDLE_BINARY_IMAGE_MODE_EMBED_AS_BASISU instead.

Embeds textures VRAM compressed with Basis Universal into the generated scene.

HANDLE_BINARY_EMBED_AS_UNCOMPRESSED = 3 

Deprecated: Use HANDLE_BINARY_IMAGE_MODE_EMBED_AS_UNCOMPRESSED instead.

Embeds textures compressed losslessly into the generated scene, matching old behavior.

### HANDLE_BINARY_EXTRACT_TEXTURES {#HANDLE_BINARY_EXTRACT_TEXTURES}

```gdscript
const HANDLE_BINARY_EXTRACT_TEXTURES = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### HANDLE_BINARY_EMBED_AS_BASISU {#HANDLE_BINARY_EMBED_AS_BASISU}

```gdscript
const HANDLE_BINARY_EMBED_AS_BASISU = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### HANDLE_BINARY_EMBED_AS_UNCOMPRESSED {#HANDLE_BINARY_EMBED_AS_UNCOMPRESSED}

```gdscript
const HANDLE_BINARY_EMBED_AS_UNCOMPRESSED = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
