# RichTextEffect

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **RichTextEffect**

> A custom effect for a RichTextLabel, which can be loaded in the RichTextLabel inspector or using RichTextLabel.install_effect().

<details><summary>官方英文描述（中文精译进行中）</summary>

A custom effect for a RichTextLabel, which can be loaded in the RichTextLabel inspector or using RichTextLabel.install_effect().

Note: For a RichTextEffect to be usable, a BBCode tag must be defined as a member variable called bbcode in the script.

```
# The RichTextEffect will be usable like this: `[example]Some text[/example]`
var bbcode = "example"
```

```
// The RichTextEffect will be usable like this: `[example]Some text[/example]`
string bbcode = "example";
```

Note: As soon as a RichTextLabel contains at least one RichTextEffect, it will continuously process the effect unless the project is paused. This may impact battery life negatively.

</details>

## 方法

### _process_custom_fx {#_process_custom_fx}

```gdscript
bool _process_custom_fx(char_fx: CharFXTransform) virtual const
```

Override this method to modify properties in char_fx. The method must return true if the character could be transformed successfully. If the method returns false, it will skip transformation to avoid displaying broken text.
