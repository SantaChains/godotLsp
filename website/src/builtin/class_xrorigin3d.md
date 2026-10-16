# XROrigin3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **XROrigin3D**

> This is a special node within the AR/VR system that maps the physical location of the center of our tracking space to the virtual location within our game world.

<details><summary>官方英文描述</summary>

This is a special node within the AR/VR system that maps the physical location of the center of our tracking space to the virtual location within our game world.

Multiple origin points can be added to the scene tree, but only one can used at a time. All the XRCamera3D, XRController3D, and XRAnchor3D nodes should be direct children of this node for spatial tracking to work correctly.

It is the position of this node that you update when your character needs to move through your game world while we're not moving in the real world. Movement in the real world is always in relation to this origin point.

For example, if your character is driving a car, the XROrigin3D node should be a child node of this car. Or, if you're implementing a teleport system to move your character, you should change the position of this node.

</details>

## 方法

### get_world_scale {#get_world_scale}

```gdscript
float get_world_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_current {#is_current}

```gdscript
bool is_current() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current {#set_current}

```gdscript
void set_current(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_world_scale {#set_world_scale}

```gdscript
void set_world_scale(world_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### world_scale {#world_scale}

```gdscript
world_scale: float
```

- void set_world_scale(value: float)
- float get_world_scale()

The scale of the game world compared to the real world. This is the same as XRServer.world_scale. By default, most AR/VR platforms assume that 1 game unit corresponds to 1 real world meter.

### current {#current}

```gdscript
current: bool
```

- void set_current(value: bool)
- bool is_current()

If true, this origin node is currently being used by the XRServer. Only one origin point can be used at a time.
