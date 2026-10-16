# Node2D

**继承**：[CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Node2D**

> 2D 游戏对象的基类。包含 position、rotation、scale 变换，CanvasItem 层级中的节点。

<details><summary>官方英文描述</summary>

A 2D game object, with a transform (position, rotation, and scale). All 2D nodes, including physics objects and sprites, inherit from Node2D. Use Node2D as a parent node to move, scale and rotate children in a 2D project. Also gives control of the node's render order.

Note: Since both Node2D and Control inherit from CanvasItem, they share several concepts from the class such as the CanvasItem.z_index and CanvasItem.visible properties.

</details>

## 方法

### apply_scale {#apply_scale}

```gdscript
void apply_scale(ratio: Vector2)
```

Multiplies the current scale by the ratio vector.

### get_angle_to {#get_angle_to}

```gdscript
float get_angle_to(point: Vector2) const
```

Returns the angle between the node and the point in radians. See also look_at().

Illustration of the returned angle. [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/node2d_get_angle_to.png]

### get_global_position {#get_global_position}

```gdscript
Vector2 get_global_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_rotation {#get_global_rotation}

```gdscript
float get_global_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_rotation_degrees {#get_global_rotation_degrees}

```gdscript
float get_global_rotation_degrees() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_scale {#get_global_scale}

```gdscript
Vector2 get_global_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_skew {#get_global_skew}

```gdscript
float get_global_skew() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position {#get_position}

```gdscript
Vector2 get_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_relative_transform_to_parent {#get_relative_transform_to_parent}

```gdscript
Transform2D get_relative_transform_to_parent(parent: Node) const
```

Returns the Transform2D relative to this node's parent.

### get_rotation {#get_rotation}

```gdscript
float get_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_degrees {#get_rotation_degrees}

```gdscript
float get_rotation_degrees() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale {#get_scale}

```gdscript
Vector2 get_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skew {#get_skew}

```gdscript
float get_skew() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### global_translate {#global_translate}

```gdscript
void global_translate(offset: Vector2)
```

Adds the offset vector to the node's global position.

### look_at {#look_at}

```gdscript
void look_at(point: Vector2)
```

Rotates the node so that its local +X axis points towards the point, which is expected to use global coordinates. This method is a combination of both rotate() and get_angle_to().

point should not be the same as the node's position, otherwise the node always looks to the right.

### move_local_x {#move_local_x}

```gdscript
void move_local_x(delta: float, scaled: bool = false)
```

Applies a local translation on the node's X axis with the amount specified in delta. If scaled is false, normalizes the movement to occur independently of the node's scale.

### move_local_y {#move_local_y}

```gdscript
void move_local_y(delta: float, scaled: bool = false)
```

Applies a local translation on the node's Y axis with the amount specified in delta. If scaled is false, normalizes the movement to occur independently of the node's scale.

### rotate {#rotate}

```gdscript
void rotate(radians: float)
```

Applies a rotation to the node, in radians, starting from its current rotation. This is equivalent to rotation += radians.

### set_global_position {#set_global_position}

```gdscript
void set_global_position(position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_rotation {#set_global_rotation}

```gdscript
void set_global_rotation(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_rotation_degrees {#set_global_rotation_degrees}

```gdscript
void set_global_rotation_degrees(degrees: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_scale {#set_global_scale}

```gdscript
void set_global_scale(scale: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_skew {#set_global_skew}

```gdscript
void set_global_skew(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_transform {#set_global_transform}

```gdscript
void set_global_transform(xform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position {#set_position}

```gdscript
void set_position(position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation {#set_rotation}

```gdscript
void set_rotation(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_degrees {#set_rotation_degrees}

```gdscript
void set_rotation_degrees(degrees: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale {#set_scale}

```gdscript
void set_scale(scale: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skew {#set_skew}

```gdscript
void set_skew(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform {#set_transform}

```gdscript
void set_transform(xform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_global {#to_global}

```gdscript
Vector2 to_global(local_point: Vector2) const
```

Transforms the provided local position into a position in global coordinate space. The input is expected to be local relative to the Node2D it is called on. e.g. Applying this method to the positions of child nodes will correctly transform their positions into the global coordinate space, but applying it to a node's own position will give an incorrect result, as it will incorporate the node's own transformation into its global position.

### to_local {#to_local}

```gdscript
Vector2 to_local(global_point: Vector2) const
```

Transforms the provided global position into a position in local coordinate space. The output will be local relative to the Node2D it is called on. e.g. It is appropriate for determining the positions of child nodes, but it is not appropriate for determining its own position relative to its parent.

### translate {#translate}

```gdscript
void translate(offset: Vector2)
```

Translates the node by the given offset in local coordinates. This is equivalent to position += offset.

## 属性

### position {#position}

```gdscript
position: Vector2
```

节点在父节点坐标系中的位置。

### rotation {#rotation}

```gdscript
rotation: float
```

节点绕 Z 轴的旋转角度（弧度）。

### rotation_degrees {#rotation_degrees}

```gdscript
rotation_degrees: float
```

- void set_rotation_degrees(value: float)
- float get_rotation_degrees()

Helper property to access rotation in degrees instead of radians. See also global_rotation_degrees.

### scale {#scale}

```gdscript
scale: Vector2
```

节点的缩放因子。(1,1) 为原始大小。

### skew {#skew}

```gdscript
skew: float
```

- void set_skew(value: float)
- float get_skew()

If set to a non-zero value, slants the node in one direction or another. This can be used for pseudo-3D effects. See also global_skew.

Note: Skew is performed on the X axis only, and between rotation and scaling.

Note: This property is edited in the inspector in degrees. If you want to use degrees in a script, use skew = deg_to_rad(value_in_degrees).

### transform {#transform}

```gdscript
transform: Transform2D
```

- void set_transform(value: Transform2D)
- Transform2D get_transform()

The node's Transform2D, relative to the node's parent. See also global_transform.

### global_position {#global_position}

```gdscript
global_position: Vector2
```

全局位置（世界坐标系）。

### global_rotation {#global_rotation}

```gdscript
global_rotation: float
```

全局旋转角度。

### global_rotation_degrees {#global_rotation_degrees}

```gdscript
global_rotation_degrees: float
```

- void set_global_rotation_degrees(value: float)
- float get_global_rotation_degrees()

Helper property to access global_rotation in degrees instead of radians. See also rotation_degrees.

### global_scale {#global_scale}

```gdscript
global_scale: Vector2
```

全局缩放。

### global_skew {#global_skew}

```gdscript
global_skew: float
```

- void set_global_skew(value: float)
- float get_global_skew()

Global skew in radians. See also skew.

### global_transform {#global_transform}

```gdscript
global_transform: Transform2D
```

- void set_global_transform(value: Transform2D)
- Transform2D get_global_transform()

Global Transform2D. See also transform.
