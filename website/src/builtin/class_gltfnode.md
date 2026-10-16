# GLTFNode

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFNode**

> Represents a glTF node. glTF nodes may have names, transforms, children (other glTF nodes), and more specialized properties (represented by their own classes).

<details><summary>官方英文描述（中文精译进行中）</summary>

Represents a glTF node. glTF nodes may have names, transforms, children (other glTF nodes), and more specialized properties (represented by their own classes).

glTF nodes generally exist inside of GLTFState which represents all data of a glTF file. Most of GLTFNode's properties are indices of other data in the glTF file. You can extend a glTF node with additional properties by using get_additional_data() and set_additional_data().

</details>

## 方法

### append_child_index {#append_child_index}

```gdscript
void append_child_index(child_index: int)
```

Appends the given child node index to the children array.

### get_additional_data {#get_additional_data}

```gdscript
Variant get_additional_data(extension_name: StringName)
```

Gets additional arbitrary data in this GLTFNode instance. This can be used to keep per-node state data in GLTFDocumentExtension classes, which is important because they are stateless.

The argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the return value can be anything you set. If nothing was set, the return value is null.

### get_camera {#get_camera}

```gdscript
int get_camera()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_children {#get_children}

```gdscript
PackedInt32Array get_children()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_height {#get_height}

```gdscript
int get_height()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_light {#get_light}

```gdscript
int get_light()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh {#get_mesh}

```gdscript
int get_mesh()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_original_name {#get_original_name}

```gdscript
String get_original_name()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parent {#get_parent}

```gdscript
int get_parent()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position {#get_position}

```gdscript
Vector3 get_position()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation {#get_rotation}

```gdscript
Quaternion get_rotation()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale {#get_scale}

```gdscript
Vector3 get_scale()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scene_node_path {#get_scene_node_path}

```gdscript
NodePath get_scene_node_path(gltf_state: GLTFState, handle_skeletons: bool = true)
```

Returns the NodePath that this GLTF node will have in the Godot scene tree after being imported. This is useful when importing glTF object model pointers with GLTFObjectModelProperty, for handling extensions such as KHR_animation_pointer or KHR_interactivity.

If handle_skeletons is true, paths to skeleton bone glTF nodes will be resolved properly. For example, a path that would be ^"A/B/C/Bone1/Bone2/Bone3" if false will become ^"A/B/C/Skeleton3D:Bone3".

### get_skeleton {#get_skeleton}

```gdscript
int get_skeleton()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skin {#get_skin}

```gdscript
int get_skin()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visible {#get_visible}

```gdscript
bool get_visible()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_xform {#get_xform}

```gdscript
Transform3D get_xform()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_additional_data {#set_additional_data}

```gdscript
void set_additional_data(extension_name: StringName, additional_data: Variant)
```

Sets additional arbitrary data in this GLTFNode instance. This can be used to keep per-node state data in GLTFDocumentExtension classes, which is important because they are stateless.

The first argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the second argument can be anything you want.

### set_camera {#set_camera}

```gdscript
void set_camera(camera: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_children {#set_children}

```gdscript
void set_children(children: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_light {#set_light}

```gdscript
void set_light(light: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_original_name {#set_original_name}

```gdscript
void set_original_name(original_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_parent {#set_parent}

```gdscript
void set_parent(parent: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position {#set_position}

```gdscript
void set_position(position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation {#set_rotation}

```gdscript
void set_rotation(rotation: Quaternion)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale {#set_scale}

```gdscript
void set_scale(scale: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skeleton {#set_skeleton}

```gdscript
void set_skeleton(skeleton: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skin {#set_skin}

```gdscript
void set_skin(skin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible {#set_visible}

```gdscript
void set_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_xform {#set_xform}

```gdscript
void set_xform(xform: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### original_name {#original_name}

```gdscript
original_name: String
```

- void set_original_name(value: String)
- String get_original_name()

The original name of the node.

### parent {#parent}

```gdscript
parent: int
```

- void set_parent(value: int)
- int get_parent()

The index of the parent node in the GLTFState. If -1, this node is a root node.

### height {#height}

```gdscript
height: int
```

- void set_height(value: int)
- int get_height()

How deep into the node hierarchy this node is. A root node will have a height of 0, its children will have a height of 1, and so on. If -1, the height has not been calculated.

### xform {#xform}

```gdscript
xform: Transform3D
```

- void set_xform(value: Transform3D)
- Transform3D get_xform()

The transform of the glTF node relative to its parent. This property is usually unused since the position, rotation, and scale properties are preferred.

### mesh {#mesh}

```gdscript
mesh: int
```

- void set_mesh(value: int)
- int get_mesh()

If this glTF node is a mesh, the index of the GLTFMesh in the GLTFState that describes the mesh's properties. If -1, this node is not a mesh.

### camera {#camera}

```gdscript
camera: int
```

- void set_camera(value: int)
- int get_camera()

If this glTF node is a camera, the index of the GLTFCamera in the GLTFState that describes the camera's properties. If -1, this node is not a camera.

### skin {#skin}

```gdscript
skin: int
```

- void set_skin(value: int)
- int get_skin()

If this glTF node has a skin, the index of the GLTFSkin in the GLTFState that describes the skin's properties. If -1, this node does not have a skin.

### skeleton {#skeleton}

```gdscript
skeleton: int
```

- void set_skeleton(value: int)
- int get_skeleton()

If this glTF node has a skeleton, the index of the GLTFSkeleton in the GLTFState that describes the skeleton's properties. If -1, this node does not have a skeleton.

### position {#position}

```gdscript
position: Vector3
```

- void set_position(value: Vector3)
- Vector3 get_position()

The position of the glTF node relative to its parent.

### rotation {#rotation}

```gdscript
rotation: Quaternion
```

- void set_rotation(value: Quaternion)
- Quaternion get_rotation()

The rotation of the glTF node relative to its parent.

### scale {#scale}

```gdscript
scale: Vector3
```

- void set_scale(value: Vector3)
- Vector3 get_scale()

The scale of the glTF node relative to its parent.

### children {#children}

```gdscript
children: PackedInt32Array
```

- void set_children(value: PackedInt32Array)
- PackedInt32Array get_children()

The indices of the child nodes in the GLTFState. If this glTF node has no children, this will be an empty array.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### light {#light}

```gdscript
light: int
```

- void set_light(value: int)
- int get_light()

If this glTF node is a light, the index of the GLTFLight in the GLTFState that describes the light's properties. If -1, this node is not a light.

### visible {#visible}

```gdscript
visible: bool
```

- void set_visible(value: bool)
- bool get_visible()

If true, the GLTF node is visible. If false, the GLTF node is not visible. This is converted to the Node3D.visible property in the Godot scene, and is exported to KHR_node_visibility when false.
