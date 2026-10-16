# ButtonGroup

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ButtonGroup**

> A group of BaseButton-derived buttons. The buttons in a ButtonGroup are treated like radio buttons: No more than one button can be pressed at a time. Some types of buttons (such as CheckBox) may have a special appearance in this state.

<details><summary>官方英文描述</summary>

A group of BaseButton-derived buttons. The buttons in a ButtonGroup are treated like radio buttons: No more than one button can be pressed at a time. Some types of buttons (such as CheckBox) may have a special appearance in this state.

Every member of a ButtonGroup should have BaseButton.toggle_mode set to true.

</details>

## 方法

### get_buttons {#get_buttons}

```gdscript
typedarray::BaseButton get_buttons()
```

Returns an Array of Buttons who have this as their ButtonGroup (see BaseButton.button_group).

### get_pressed_button {#get_pressed_button}

```gdscript
BaseButton get_pressed_button()
```

Returns the current pressed button.

### is_allow_unpress {#is_allow_unpress}

```gdscript
bool is_allow_unpress()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_unpress {#set_allow_unpress}

```gdscript
void set_allow_unpress(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### allow_unpress {#allow_unpress}

```gdscript
allow_unpress: bool
```

- void set_allow_unpress(value: bool)
- bool is_allow_unpress()

If true, it is possible to unpress all buttons in this ButtonGroup.

## 信号

### pressed {#pressed}

```gdscript
signal pressed(button: BaseButton)
```

Emitted when one of the buttons of the group is pressed.
