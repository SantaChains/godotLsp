# MarginContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **MarginContainer**

> MarginContainer adds an adjustable margin on each side of its child controls. The margins are added around all children, not around each individual one. To control the MarginContainer's margins, use the margin_* theme properties listed below.

<details><summary>官方英文描述（中文精译进行中）</summary>

MarginContainer adds an adjustable margin on each side of its child controls. The margins are added around all children, not around each individual one. To control the MarginContainer's margins, use the margin_* theme properties listed below.

Note: The margin sizes are theme overrides, not normal properties. This is an example of how to change them in code:

```
# This code sample assumes the current script is extending MarginContainer.
var margin_value = 100
add_theme_constant_override("margin_top", margin_value)
add_theme_constant_override("margin_left", margin_value)
add_theme_constant_override("margin_bottom", margin_value)
add_theme_constant_override("margin_right", margin_value)
```

```
// This code sample assumes the current script is extending MarginContainer.
int marginValue = 100;
AddThemeConstantOverride("margin_top", marginValue);
AddThemeConstantOverride("margin_left", marginValue);
AddThemeConstantOverride("margin_bottom", marginValue);
AddThemeConstantOverride("margin_right", marginValue);
```

</details>
