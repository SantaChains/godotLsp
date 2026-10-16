# GLTFPhysicsShape

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFPhysicsShape**

> Represents a physics shape as defined by the OMI_physics_shape or OMI_collider glTF extensions. This class is an intermediary between the glTF data and Godot's nodes, and it's abstracted in a way that allows adding support for different glTF physics extensions in the future.

<details><summary>官方英文描述（中文精译进行中）</summary>

Represents a physics shape as defined by the OMI_physics_shape or OMI_collider glTF extensions. This class is an intermediary between the glTF data and Godot's nodes, and it's abstracted in a way that allows adding support for different glTF physics extensions in the future.

</details>

## 方法

### from_dictionary {#from_dictionary}

```gdscript
static GLTFPhysicsShape from_dictionary(dictionary: Dictionary)
```

Creates a new GLTFPhysicsShape instance by parsing the given Dictionary.

### from_node {#from_node}

```gdscript
static GLTFPhysicsShape from_node(shape_node: CollisionShape3D)
```

Creates a new GLTFPhysicsShape instance from the given Godot CollisionShape3D node.

### from_resource {#from_resource}

```gdscript
static GLTFPhysicsShape from_resource(shape_resource: Shape3D)
```

Creates a new GLTFPhysicsShape instance from the given Godot Shape3D resource.

### get_height {#get_height}

```gdscript
float get_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_importer_mesh {#get_importer_mesh}

```gdscript
ImporterMesh get_importer_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_is_trigger {#get_is_trigger}

```gdscript
bool get_is_trigger() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh_index {#get_mesh_index}

```gdscript
int get_mesh_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape_type {#get_shape_type}

```gdscript
String get_shape_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_importer_mesh {#set_importer_mesh}

```gdscript
void set_importer_mesh(importer_mesh: ImporterMesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_is_trigger {#set_is_trigger}

```gdscript
void set_is_trigger(is_trigger: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh_index {#set_mesh_index}

```gdscript
void set_mesh_index(mesh_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape_type {#set_shape_type}

```gdscript
void set_shape_type(shape_type: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_dictionary {#to_dictionary}

```gdscript
Dictionary to_dictionary() const
```

Serializes this GLTFPhysicsShape instance into a Dictionary in the format defined by OMI_physics_shape.

### to_node {#to_node}

```gdscript
CollisionShape3D to_node(cache_shapes: bool = false)
```

Converts this GLTFPhysicsShape instance into a Godot CollisionShape3D node.

### to_resource {#to_resource}

```gdscript
Shape3D to_resource(cache_shapes: bool = false)
```

Converts this GLTFPhysicsShape instance into a Godot Shape3D resource.

## 属性

### shape_type {#shape_type}

```gdscript
shape_type: String
```

- void set_shape_type(value: String)
- String get_shape_type()

The type of shape this shape represents. Valid values are "box", "capsule", "cylinder", "sphere", "hull", and "trimesh".

### size {#size}

```gdscript
size: Vector3
```

- void set_size(value: Vector3)
- Vector3 get_size()

The size of the shape, in meters. This is only used when the shape type is "box", and it represents the "diameter" of the box. This value should not be negative.

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The radius of the shape, in meters. This is only used when the shape type is "capsule", "cylinder", or "sphere". This value should not be negative.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

The height of the shape, in meters. This is only used when the shape type is "capsule" or "cylinder". This value should not be negative, and for "capsule" it should be at least twice the radius.

### is_trigger {#is_trigger}

```gdscript
is_trigger: bool
```

- void set_is_trigger(value: bool)
- bool get_is_trigger()

If true, indicates that this shape is a trigger. For Godot, this means that the shape should be a child of an Area3D node.

This is the only variable not used in the to_node() method, it's intended to be used alongside when deciding where to add the generated node as a child.

### mesh_index {#mesh_index}

```gdscript
mesh_index: int
```

- void set_mesh_index(value: int)
- int get_mesh_index()

The index of the shape's mesh in the glTF file. This is only used when the shape type is "hull" (convex hull) or "trimesh" (concave trimesh).

### importer_mesh {#importer_mesh}

```gdscript
importer_mesh: ImporterMesh
```

- void set_importer_mesh(value: ImporterMesh)
- ImporterMesh get_importer_mesh()

The ImporterMesh resource of the shape. This is only used when the shape type is "hull" (convex hull) or "trimesh" (concave trimesh).
