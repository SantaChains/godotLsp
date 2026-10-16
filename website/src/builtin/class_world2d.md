# World2D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **World2D**

> Class that has everything pertaining to a 2D world: A physics space, a canvas, and a sound space. 2D nodes register their resources into the current 2D world.

<details><summary>官方英文描述</summary>

Class that has everything pertaining to a 2D world: A physics space, a canvas, and a sound space. 2D nodes register their resources into the current 2D world.

</details>

## 方法

### get_canvas {#get_canvas}

```gdscript
RID get_canvas() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_direct_space_state {#get_direct_space_state}

```gdscript
PhysicsDirectSpaceState2D get_direct_space_state()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_space {#get_space}

```gdscript
RID get_space() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### canvas {#canvas}

```gdscript
canvas: RID
```

- RID get_canvas()

The RID of this world's canvas resource. Used by the RenderingServer for 2D drawing.

### navigation_map {#navigation_map}

```gdscript
navigation_map: RID
```

- RID get_navigation_map()

The RID of this world's navigation map. Used by the NavigationServer2D.

### space {#space}

```gdscript
space: RID
```

- RID get_space()

The RID of this world's physics space resource. Used by the PhysicsServer2D for 2D physics, treating it as both a space and an area.

### direct_space_state {#direct_space_state}

```gdscript
direct_space_state: PhysicsDirectSpaceState2D
```

- PhysicsDirectSpaceState2D get_direct_space_state()

Direct access to the world's physics 2D space state. Used for querying current and potential collisions. When using multi-threaded physics, access is limited to Node._physics_process() in the main thread.
