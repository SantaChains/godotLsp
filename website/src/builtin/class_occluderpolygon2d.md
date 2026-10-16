# OccluderPolygon2D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OccluderPolygon2D**

> Editor facility that helps you draw a 2D polygon used as resource for LightOccluder2D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Editor facility that helps you draw a 2D polygon used as resource for LightOccluder2D.

</details>

## 方法

### get_cull_mode {#get_cull_mode}

```gdscript
OccluderPolygon2D.CullMode get_cull_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_polygon {#get_polygon}

```gdscript
PackedVector2Array get_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_closed {#is_closed}

```gdscript
bool is_closed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_closed {#set_closed}

```gdscript
void set_closed(closed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cull_mode {#set_cull_mode}

```gdscript
void set_cull_mode(cull_mode: OccluderPolygon2D.CullMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polygon {#set_polygon}

```gdscript
void set_polygon(polygon: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### closed {#closed}

```gdscript
closed: bool
```

- void set_closed(value: bool)
- bool is_closed()

If true, closes the polygon. A closed OccluderPolygon2D occludes the light coming from any direction. An opened OccluderPolygon2D occludes the light only at its outline's direction.

### cull_mode {#cull_mode}

```gdscript
cull_mode: int
```

- void set_cull_mode(value: CullMode)
- CullMode get_cull_mode()

The culling mode to use.

### polygon {#polygon}

```gdscript
polygon: PackedVector2Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### CullMode {#enum-CullMode}

```gdscript
enum CullMode { CULL_DISABLED = 0, CULL_CLOCKWISE = 1, CULL_COUNTER_CLOCKWISE = 2 }
```

CullMode CULL_DISABLED = 0

Culling is disabled. See cull_mode.

CullMode CULL_CLOCKWISE = 1

Culling is performed in the clockwise direction. See cull_mode.

CullMode CULL_COUNTER_CLOCKWISE = 2

Culling is performed in the counterclockwise direction. See cull_mode.

#### CULL_DISABLED {#CULL_DISABLED}

```gdscript
CULL_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CULL_CLOCKWISE {#CULL_CLOCKWISE}

```gdscript
CULL_CLOCKWISE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CULL_COUNTER_CLOCKWISE {#CULL_COUNTER_CLOCKWISE}

```gdscript
CULL_COUNTER_CLOCKWISE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
