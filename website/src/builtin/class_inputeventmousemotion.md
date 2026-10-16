# InputEventMouseMotion

**继承**：[InputEventMouse](class_inputeventmouse.md) → [InputEventWithModifiers](class_inputeventwithmodifiers.md) → [InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventMouseMotion**

> Stores information about a mouse or a pen motion. This includes relative position, absolute position, and velocity. See Node._input().

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores information about a mouse or a pen motion. This includes relative position, absolute position, and velocity. See Node._input().

Note: By default, this event is only emitted once per frame rendered at most. If you need more precise input reporting, set Input.use_accumulated_input to false to make events emitted as often as possible. If you use InputEventMouseMotion to draw lines, consider using Geometry2D.bresenham_line() as well to avoid visible gaps in lines if the user is moving the mouse quickly.

Note: This event may be emitted even when the mouse hasn't moved, either by the operating system or by Godot itself. If you really need to know if the mouse has moved (e.g. to suppress displaying a tooltip), you should check that relative.is_zero_approx() is false.

</details>

## 方法

### get_pen_inverted {#get_pen_inverted}

```gdscript
bool get_pen_inverted() const
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

### set_pen_inverted {#set_pen_inverted}

```gdscript
void set_pen_inverted(pen_inverted: bool)
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

Note: This property is implemented on Linux, macOS and Windows.

### relative {#relative}

```gdscript
relative: Vector2
```

- void set_relative(value: Vector2)
- Vector2 get_relative()

The mouse position relative to the previous position (position at the last frame).

Note: Since InputEventMouseMotion may only be emitted when the mouse moves, it is not possible to reliably detect when the mouse has stopped moving by checking this property. A separate, short timer may be necessary.

Note: relative is automatically scaled according to the content scale factor, which is defined by the project's stretch mode settings. This means mouse sensitivity will appear different depending on resolution when using relative in a script that handles mouse aiming with the Input.MOUSE_MODE_CAPTURED mouse mode. To avoid this, use screen_relative instead.

### screen_relative {#screen_relative}

```gdscript
screen_relative: Vector2
```

- void set_screen_relative(value: Vector2)
- Vector2 get_screen_relative()

The unscaled mouse position relative to the previous position in the coordinate system of the screen (position at the last frame).

Note: Since InputEventMouseMotion may only be emitted when the mouse moves, it is not possible to reliably detect when the mouse has stopped moving by checking this property. A separate, short timer may be necessary.

Note: This coordinate is not scaled according to the content scale factor or calls to InputEvent.xformed_by(). This should be preferred over relative for mouse aiming when using the Input.MOUSE_MODE_CAPTURED mouse mode, regardless of the project's stretch mode.

### velocity {#velocity}

```gdscript
velocity: Vector2
```

- void set_velocity(value: Vector2)
- Vector2 get_velocity()

The mouse velocity in pixels per second.

Note: velocity is automatically scaled according to the content scale factor, which is defined by the project's stretch mode settings. That means mouse sensitivity may appear different depending on resolution.

Note: In Input.MOUSE_MODE_CAPTURED mode, velocity returns (0, 0) because the mouse cursor is hidden and locked. Use screen_relative for mouse aiming using the Input.MOUSE_MODE_CAPTURED mouse mode.

### screen_velocity {#screen_velocity}

```gdscript
screen_velocity: Vector2
```

- void set_screen_velocity(value: Vector2)
- Vector2 get_screen_velocity()

The unscaled mouse velocity in pixels per second in screen coordinates. This velocity is not scaled according to the content scale factor or calls to InputEvent.xformed_by().

Note: In Input.MOUSE_MODE_CAPTURED mode, screen_velocity returns (0, 0) because the mouse cursor is hidden and locked. Use screen_relative for mouse aiming using the Input.MOUSE_MODE_CAPTURED mouse mode.
