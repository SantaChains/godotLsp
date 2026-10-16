# InputEventShortcut

**继承**：[InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventShortcut**

> InputEventShortcut is a special event that can be received in Node._input(), Node._shortcut_input(), and Node._unhandled_input(). It is typically sent by the editor's Command Palette to trigger actions, but can also be sent manually using Viewport.push_input().

<details><summary>官方英文描述（中文精译进行中）</summary>

InputEventShortcut is a special event that can be received in Node._input(), Node._shortcut_input(), and Node._unhandled_input(). It is typically sent by the editor's Command Palette to trigger actions, but can also be sent manually using Viewport.push_input().

</details>

## 方法

### get_shortcut {#get_shortcut}

```gdscript
Shortcut get_shortcut()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shortcut {#set_shortcut}

```gdscript
void set_shortcut(shortcut: Shortcut)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### shortcut {#shortcut}

```gdscript
shortcut: Shortcut
```

- void set_shortcut(value: Shortcut)
- Shortcut get_shortcut()

The Shortcut represented by this event. Its Shortcut.matches_event() method will always return true for this event.
