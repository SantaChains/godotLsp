# GLTFPhysicsBody

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFPhysicsBody**

> Represents a physics body as an intermediary between the OMI_physics_body glTF data and Godot's nodes, and it's abstracted in a way that allows adding support for different glTF physics extensions in the future.

<details><summary>官方英文描述（中文精译进行中）</summary>

Represents a physics body as an intermediary between the OMI_physics_body glTF data and Godot's nodes, and it's abstracted in a way that allows adding support for different glTF physics extensions in the future.

</details>

## 方法

### from_dictionary {#from_dictionary}

```gdscript
static GLTFPhysicsBody from_dictionary(dictionary: Dictionary)
```

Creates a new GLTFPhysicsBody instance by parsing the given Dictionary in the OMI_physics_body glTF extension format.

### from_node {#from_node}

```gdscript
static GLTFPhysicsBody from_node(body_node: CollisionObject3D)
```

Creates a new GLTFPhysicsBody instance from the given Godot CollisionObject3D node.

### get_angular_velocity {#get_angular_velocity}

```gdscript
Vector3 get_angular_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_body_type {#get_body_type}

```gdscript
String get_body_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_of_mass {#get_center_of_mass}

```gdscript
Vector3 get_center_of_mass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inertia_diagonal {#get_inertia_diagonal}

```gdscript
Vector3 get_inertia_diagonal() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inertia_orientation {#get_inertia_orientation}

```gdscript
Quaternion get_inertia_orientation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inertia_tensor {#get_inertia_tensor}

```gdscript
Basis get_inertia_tensor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_velocity {#get_linear_velocity}

```gdscript
Vector3 get_linear_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mass {#get_mass}

```gdscript
float get_mass() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_velocity {#set_angular_velocity}

```gdscript
void set_angular_velocity(angular_velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_body_type {#set_body_type}

```gdscript
void set_body_type(body_type: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_of_mass {#set_center_of_mass}

```gdscript
void set_center_of_mass(center_of_mass: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inertia_diagonal {#set_inertia_diagonal}

```gdscript
void set_inertia_diagonal(inertia_diagonal: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inertia_orientation {#set_inertia_orientation}

```gdscript
void set_inertia_orientation(inertia_orientation: Quaternion)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inertia_tensor {#set_inertia_tensor}

```gdscript
void set_inertia_tensor(inertia_tensor: Basis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_velocity {#set_linear_velocity}

```gdscript
void set_linear_velocity(linear_velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mass {#set_mass}

```gdscript
void set_mass(mass: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_dictionary {#to_dictionary}

```gdscript
Dictionary to_dictionary() const
```

Serializes this GLTFPhysicsBody instance into a Dictionary. It will be in the format expected by the OMI_physics_body glTF extension.

### to_node {#to_node}

```gdscript
CollisionObject3D to_node() const
```

Converts this GLTFPhysicsBody instance into a Godot CollisionObject3D node.

## 属性

### body_type {#body_type}

```gdscript
body_type: String
```

- void set_body_type(value: String)
- String get_body_type()

The type of the body.

When importing, this controls what type of CollisionObject3D node Godot should generate. Valid values are "static", "animatable", "character", "rigid", "vehicle", and "trigger".

When exporting, this will be squashed down to one of "static", "kinematic", or "dynamic" motion types, or the "trigger" property.

### mass {#mass}

```gdscript
mass: float
```

- void set_mass(value: float)
- float get_mass()

The mass of the physics body, in kilograms. This is only used when the body type is "rigid" or "vehicle".

### linear_velocity {#linear_velocity}

```gdscript
linear_velocity: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### angular_velocity {#angular_velocity}

```gdscript
angular_velocity: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### center_of_mass {#center_of_mass}

```gdscript
center_of_mass: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### inertia_diagonal {#inertia_diagonal}

```gdscript
inertia_diagonal: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### inertia_orientation {#inertia_orientation}

```gdscript
inertia_orientation: Quaternion
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### inertia_tensor {#inertia_tensor}

```gdscript
inertia_tensor: Basis
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
