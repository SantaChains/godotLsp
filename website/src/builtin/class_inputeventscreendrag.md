# InputEventScreenDrag

**继承**：[InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventScreenDrag**

> Stores information about screen drag events. See Node._input().

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores information about screen drag events. See Node._input().

</details>

## 方法

### get_index {#get_index}

```gdscript
int get_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pen_inverted {#get_pen_inverted}

```gdscript
bool get_pen_inverted() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position {#get_position}

```gdscript
Vector2 get_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pressure {#get_pressure}

```gdscript
float get_pressure() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_relative {#get_relative}

```gdscript
Vector2 get_relative() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_screen_relative {#get_screen_relative}

```gdscript
Vector2 get_screen_relative() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_screen_velocity {#get_screen_velocity}

```gdscript
Vector2 get_screen_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tilt {#get_tilt}

```gdscript
Vector2 get_tilt() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_velocity {#get_velocity}

```gdscript
Vector2 get_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_index {#set_index}

```gdscript
void set_index(index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pen_inverted {#set_pen_inverted}

```gdscript
void set_pen_inverted(pen_inverted: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position {#set_position}

```gdscript
void set_position(position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressure {#set_pressure}

```gdscript
void set_pressure(pressure: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_relative {#set_relative}

```gdscript
void set_relative(relative: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_screen_relative {#set_screen_relative}

```gdscript
void set_screen_relative(relative: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_screen_velocity {#set_screen_velocity}

```gdscript
void set_screen_velocity(velocity: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tilt {#set_tilt}

```gdscript
void set_tilt(tilt: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity {#set_velocity}

```gdscript
void set_velocity(velocity: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### index {#index}

```gdscript
index: int
```

- void set_index(value: int)
- int get_index()

The drag event index in the case of a multi-drag event.

### tilt {#tilt}

```gdscript
tilt: Vector2
```

- void set_tilt(value: Vector2)
- Vector2 get_tilt()

Represents the angles of tilt of the pen. Positive X-coordinate value indicates a tilt to the right. Positive Y-coordinate value indicates a tilt toward the user. Ranges from -1.0 to 1.0 for both axes.

### pressure {#pressure}

```gdscript
pressure: float
```

- void set_pressure(value: float)
- float get_pressure()

Represents the pressure the user puts on the pen. Ranges from 0.0 to 1.0.

### pen_inverted {#pen_inverted}

```gdscript
pen_inverted: bool
```

- void set_pen_inverted(value: bool)
- bool get_pen_inverted()

Returns true when using the eraser end of a stylus pen.

### position {#position}

```gdscript
position: Vector2
```

- void set_position(value: Vector2)
- Vector2 get_position()

The drag position in the viewport the node is in, using the coordinate system of this viewport.

### relative {#relative}

```gdscript
relative: Vector2
```

- void set_relative(value: Vector2)
- Vector2 get_relative()

The drag position relative to the previous position (position at the last frame).

Note: relative is automatically scaled according to the content scale factor, which is defined by the project's stretch mode settings. This means touch sensitivity will appear different depending on resolution when using relative in a script that handles touch aiming. To avoid this, use screen_relative instead.

### screen_relative {#screen_relative}

```gdscript
screen_relative: Vector2
```

- void set_screen_relative(value: Vector2)
- Vector2 get_screen_relative()

The unscaled drag position relative to the previous position in screen coordinates (position at the last frame). This position is not scaled according to the content scale factor or calls to InputEvent.xformed_by(). This should be preferred over relative for touch aiming regardless of the project's stretch mode.

### velocity {#velocity}

```gdscript
velocity: Vector2
```

- void set_velocity(value: Vector2)
- Vector2 get_velocity()

The drag velocity.

Note: velocity is automatically scaled according to the content scale factor, which is defined by the project's stretch mode settings. This means touch sensitivity will appear different depending on resolution when using velocity in a script that handles touch aiming. To avoid this, use screen_velocity instead.

### screen_velocity {#screen_velocity}

```gdscript
screen_velocity: Vector2
```

- void set_screen_velocity(value: Vector2)
- Vector2 get_screen_velocity()

The unscaled drag velocity in pixels per second in screen coordinates. This velocity is not scaled according to the content scale factor or calls to InputEvent.xformed_by(). This should be preferred over velocity for touch aiming regardless of the project's stretch mode.
