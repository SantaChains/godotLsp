# RootMotionView

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **RootMotionView**

> Root motion refers to an animation technique where a mesh's skeleton is used to give impulse to a character. When working with 3D animations, a popular technique is for animators to use the root skeleton bone to give motion to the rest of the skeleton. This allows animating characters in a way where steps actually match the floor below. It also allows precise interaction with objects during cinematics. See also AnimationMixer.

<details><summary>官方英文描述</summary>

Root motion refers to an animation technique where a mesh's skeleton is used to give impulse to a character. When working with 3D animations, a popular technique is for animators to use the root skeleton bone to give motion to the rest of the skeleton. This allows animating characters in a way where steps actually match the floor below. It also allows precise interaction with objects during cinematics. See also AnimationMixer.

Note: RootMotionView is only visible in the editor. It will be hidden automatically in the running project.

</details>

## 方法

### get_animation_path {#get_animation_path}

```gdscript
NodePath get_animation_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cell_size {#get_cell_size}

```gdscript
float get_cell_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color {#get_color}

```gdscript
Color get_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_zero_y {#get_zero_y}

```gdscript
bool get_zero_y() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_animation_path {#set_animation_path}

```gdscript
void set_animation_path(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cell_size {#set_cell_size}

```gdscript
void set_cell_size(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color {#set_color}

```gdscript
void set_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_zero_y {#set_zero_y}

```gdscript
void set_zero_y(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### animation_path {#animation_path}

```gdscript
animation_path: NodePath
```

- void set_animation_path(value: NodePath)
- NodePath get_animation_path()

Path to an AnimationMixer node to use as a basis for root motion.

### color {#color}

```gdscript
color: Color
```

- void set_color(value: Color)
- Color get_color()

The grid's color.

### cell_size {#cell_size}

```gdscript
cell_size: float
```

- void set_cell_size(value: float)
- float get_cell_size()

The grid's cell size in 3D units.

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The grid's radius in 3D units. The grid's opacity will fade gradually as the distance from the origin increases until this radius is reached.

### zero_y {#zero_y}

```gdscript
zero_y: bool
```

- void set_zero_y(value: bool)
- bool get_zero_y()

If true, the grid's points will all be on the same Y coordinate (local Y = 0). If false, the points' original Y coordinate is preserved.
