# GLTFCamera

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFCamera**

> Represents a camera as defined by the base glTF spec.

<details><summary>官方英文描述</summary>

Represents a camera as defined by the base glTF spec.

</details>

## 方法

### from_dictionary {#from_dictionary}

```gdscript
static GLTFCamera from_dictionary(dictionary: Dictionary)
```

Creates a new GLTFCamera instance by parsing the given Dictionary.

### from_node {#from_node}

```gdscript
static GLTFCamera from_node(camera_node: Camera3D)
```

Create a new GLTFCamera instance from the given Godot Camera3D node.

### get_depth_far {#get_depth_far}

```gdscript
float get_depth_far() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_depth_near {#get_depth_near}

```gdscript
float get_depth_near() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fov {#get_fov}

```gdscript
float get_fov() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_perspective {#get_perspective}

```gdscript
bool get_perspective() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size_mag {#get_size_mag}

```gdscript
float get_size_mag() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_depth_far {#set_depth_far}

```gdscript
void set_depth_far(zdepth_far: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_depth_near {#set_depth_near}

```gdscript
void set_depth_near(zdepth_near: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fov {#set_fov}

```gdscript
void set_fov(fov: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_perspective {#set_perspective}

```gdscript
void set_perspective(perspective: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size_mag {#set_size_mag}

```gdscript
void set_size_mag(size_mag: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_dictionary {#to_dictionary}

```gdscript
Dictionary to_dictionary() const
```

Serializes this GLTFCamera instance into a Dictionary.

### to_node {#to_node}

```gdscript
Camera3D to_node() const
```

Converts this GLTFCamera instance into a Godot Camera3D node.

## 属性

### perspective {#perspective}

```gdscript
perspective: bool
```

- void set_perspective(value: bool)
- bool get_perspective()

If true, the camera is in perspective mode. Otherwise, the camera is in orthographic/orthogonal mode. This maps to glTF's camera type property. See Camera3D.projection and the glTF spec for more information.

### fov {#fov}

```gdscript
fov: float
```

- void set_fov(value: float)
- float get_fov()

The FOV of the camera. This class and glTF define the camera FOV in radians, while Godot uses degrees. This maps to glTF's yfov property. This value is only used for perspective cameras, when perspective is true.

### size_mag {#size_mag}

```gdscript
size_mag: float
```

- void set_size_mag(value: float)
- float get_size_mag()

The size of the camera. This class and glTF define the camera size magnitude as a radius in meters, while Godot defines it as a diameter in meters. This maps to glTF's ymag property. This value is only used for orthographic/orthogonal cameras, when perspective is false.

### depth_far {#depth_far}

```gdscript
depth_far: float
```

- void set_depth_far(value: float)
- float get_depth_far()

The distance to the far culling boundary for this camera relative to its local Z axis, in meters. This maps to glTF's zfar property.

### depth_near {#depth_near}

```gdscript
depth_near: float
```

- void set_depth_near(value: float)
- float get_depth_near()

The distance to the near culling boundary for this camera relative to its local Z axis, in meters. This maps to glTF's znear property.
