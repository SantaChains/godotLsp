# LightOccluder2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **LightOccluder2D**

> Occludes light cast by a Light2D, casting shadows. The LightOccluder2D must be provided with an OccluderPolygon2D in order for the shadow to be computed.

<details><summary>官方英文描述（中文精译进行中）</summary>

Occludes light cast by a Light2D, casting shadows. The LightOccluder2D must be provided with an OccluderPolygon2D in order for the shadow to be computed.

</details>

## 方法

### get_occluder_light_mask {#get_occluder_light_mask}

```gdscript
int get_occluder_light_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_occluder_polygon {#get_occluder_polygon}

```gdscript
OccluderPolygon2D get_occluder_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_set_as_sdf_collision {#is_set_as_sdf_collision}

```gdscript
bool is_set_as_sdf_collision() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_as_sdf_collision {#set_as_sdf_collision}

```gdscript
void set_as_sdf_collision(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_occluder_light_mask {#set_occluder_light_mask}

```gdscript
void set_occluder_light_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_occluder_polygon {#set_occluder_polygon}

```gdscript
void set_occluder_polygon(polygon: OccluderPolygon2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### occluder {#occluder}

```gdscript
occluder: OccluderPolygon2D
```

- void set_occluder_polygon(value: OccluderPolygon2D)
- OccluderPolygon2D get_occluder_polygon()

The OccluderPolygon2D used to compute the shadow.

### sdf_collision {#sdf_collision}

```gdscript
sdf_collision: bool
```

- void set_as_sdf_collision(value: bool)
- bool is_set_as_sdf_collision()

If enabled, the occluder will be part of a real-time generated signed distance field that can be used in custom shaders.

### occluder_light_mask {#occluder_light_mask}

```gdscript
occluder_light_mask: int
```

- void set_occluder_light_mask(value: int)
- int get_occluder_light_mask()

The LightOccluder2D's occluder light mask. The LightOccluder2D will cast shadows only from Light2D(s) that have the same light mask(s).
