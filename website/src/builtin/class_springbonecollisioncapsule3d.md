# SpringBoneCollisionCapsule3D

**继承**：[SpringBoneCollision3D](class_springbonecollision3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SpringBoneCollisionCapsule3D**

> A capsule shape collision that interacts with SpringBoneSimulator3D.

<details><summary>官方英文描述</summary>

A capsule shape collision that interacts with SpringBoneSimulator3D.

</details>

## 方法

### get_height {#get_height}

```gdscript
float get_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mid_height {#get_mid_height}

```gdscript
float get_mid_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_inside {#is_inside}

```gdscript
bool is_inside() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inside {#set_inside}

```gdscript
void set_inside(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mid_height {#set_mid_height}

```gdscript
void set_mid_height(mid_height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The capsule's radius.

Note: The radius of a capsule cannot be greater than half of its height. Otherwise, the capsule becomes a sphere. If the radius is greater than half of the height, the properties adjust to a valid value.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

The capsule's full height, including the hemispheres.

Note: The height of a capsule must be at least twice its radius. Otherwise, the capsule becomes a sphere. If the height is less than twice the radius, the properties adjust to a valid value.

### mid_height {#mid_height}

```gdscript
mid_height: float
```

- void set_mid_height(value: float)
- float get_mid_height()

The capsule's height, excluding the hemispheres. This is the height of the central cylindrical part in the middle of the capsule, and is the distance between the centers of the two hemispheres. This is a wrapper for height.

### inside {#inside}

```gdscript
inside: bool
```

- void set_inside(value: bool)
- bool is_inside()

If true, the collision acts to trap the joint within the collision.
