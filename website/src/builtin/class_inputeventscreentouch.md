# InputEventScreenTouch

**继承**：[InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventScreenTouch**

> Stores information about multi-touch press/release input events. Supports touch press, touch release and index for multi-touch count and order.

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores information about multi-touch press/release input events. Supports touch press, touch release and index for multi-touch count and order.

</details>

## 方法

### get_index {#get_index}

```gdscript
int get_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position {#get_position}

```gdscript
Vector2 get_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_double_tap {#is_double_tap}

```gdscript
bool is_double_tap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_canceled {#set_canceled}

```gdscript
void set_canceled(canceled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_double_tap {#set_double_tap}

```gdscript
void set_double_tap(double_tap: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_index {#set_index}

```gdscript
void set_index(index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position {#set_position}

```gdscript
void set_position(position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressed {#set_pressed}

```gdscript
void set_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### index {#index}

```gdscript
index: int
```

- void set_index(value: int)
- int get_index()

The touch index in the case of a multi-touch event. One index = one finger.

### position {#position}

```gdscript
position: Vector2
```

- void set_position(value: Vector2)
- Vector2 get_position()

The touch position in the viewport the node is in, using the coordinate system of this viewport.

### canceled {#canceled}

```gdscript
canceled: bool
```

- void set_canceled(value: bool)
- bool is_canceled()

If true, the touch event has been canceled.

### pressed {#pressed}

```gdscript
pressed: bool
```

- void set_pressed(value: bool)
- bool is_pressed()

If true, the touch's state is pressed. If false, the touch's state is released.

### double_tap {#double_tap}

```gdscript
double_tap: bool
```

- void set_double_tap(value: bool)
- bool is_double_tap()

If true, the touch's state is a double tap.
