# InputEventMouse

**继承**：[InputEventWithModifiers](class_inputeventwithmodifiers.md) → [InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventMouse**

> Stores general information about mouse events.

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores general information about mouse events.

</details>

## 方法

### get_button_mask {#get_button_mask}

```gdscript
bitfield::MouseButtonMask get_button_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_position {#get_global_position}

```gdscript
Vector2 get_global_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position {#get_position}

```gdscript
Vector2 get_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_button_mask {#set_button_mask}

```gdscript
void set_button_mask(button_mask: bitfield::MouseButtonMask)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_position {#set_global_position}

```gdscript
void set_global_position(global_position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position {#set_position}

```gdscript
void set_position(position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### button_mask {#button_mask}

```gdscript
button_mask: int
```

- void set_button_mask(value: BitField[MouseButtonMask])
- BitField[MouseButtonMask] get_button_mask()

The mouse button mask identifier, one of or a bitwise combination of the MouseButton button masks.

### position {#position}

```gdscript
position: Vector2
```

- void set_position(value: Vector2)
- Vector2 get_position()

When received in Node._input() or Node._unhandled_input(), returns the mouse's position in the Viewport this Node is in using the coordinate system of this Viewport.

When received in Control._gui_input(), returns the mouse's position in the Control using the local coordinate system of the Control.

### global_position {#global_position}

```gdscript
global_position: Vector2
```

- void set_global_position(value: Vector2)
- Vector2 get_global_position()

When received in Node._input() or Node._unhandled_input(), returns the mouse's position in the root Viewport using the coordinate system of the root Viewport.

When received in Control._gui_input(), returns the mouse's position in the CanvasLayer that the Control is in using the coordinate system of the CanvasLayer.
