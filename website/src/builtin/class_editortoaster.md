# EditorToaster

**继承**：[HBoxContainer](class_hboxcontainer.md) → [BoxContainer](class_boxcontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorToaster**

> This object manages the functionality and display of toast notifications within the editor, ensuring immediate and informative alerts are presented to the user.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object manages the functionality and display of toast notifications within the editor, ensuring immediate and informative alerts are presented to the user.

Note: This class shouldn't be instantiated directly. Instead, access the singleton using EditorInterface.get_editor_toaster().

</details>

## 方法

### push_toast {#push_toast}

```gdscript
void push_toast(message: String, severity: EditorToaster.Severity = 0, tooltip: String = "")
```

Pushes a toast notification to the editor for display.

## 枚举

### Severity {#enum-Severity}

```gdscript
enum Severity { SEVERITY_INFO = 0, SEVERITY_WARNING = 1, SEVERITY_ERROR = 2 }
```

Severity SEVERITY_INFO = 0

Toast will display with an INFO severity.

Severity SEVERITY_WARNING = 1

Toast will display with a WARNING severity and have a corresponding color.

Severity SEVERITY_ERROR = 2

Toast will display with an ERROR severity and have a corresponding color.

#### SEVERITY_INFO {#SEVERITY_INFO}

```gdscript
SEVERITY_INFO = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SEVERITY_WARNING {#SEVERITY_WARNING}

```gdscript
SEVERITY_WARNING = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SEVERITY_ERROR {#SEVERITY_ERROR}

```gdscript
SEVERITY_ERROR = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
