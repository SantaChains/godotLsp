# LinkButton

**继承**：[BaseButton](class_basebutton.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **LinkButton**

> A button that represents a link. This type of button is primarily used for interactions that cause a context change (like linking to a web page).

<details><summary>官方英文描述</summary>

A button that represents a link. This type of button is primarily used for interactions that cause a context change (like linking to a web page).

See also BaseButton which contains common properties and methods associated with this node.

</details>

## 方法

### get_ellipsis_char {#get_ellipsis_char}

```gdscript
String get_ellipsis_char() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_structured_text_bidi_override {#get_structured_text_bidi_override}

```gdscript
TextServer.StructuredTextParser get_structured_text_bidi_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_structured_text_bidi_override_options {#get_structured_text_bidi_override_options}

```gdscript
Array get_structured_text_bidi_override_options() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text {#get_text}

```gdscript
String get_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_direction {#get_text_direction}

```gdscript
Control.TextDirection get_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_overrun_behavior {#get_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_text_overrun_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_underline_mode {#get_underline_mode}

```gdscript
LinkButton.UnderlineMode get_underline_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_uri {#get_uri}

```gdscript
String get_uri() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ellipsis_char {#set_ellipsis_char}

```gdscript
void set_ellipsis_char(char: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_structured_text_bidi_override {#set_structured_text_bidi_override}

```gdscript
void set_structured_text_bidi_override(parser: TextServer.StructuredTextParser)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_structured_text_bidi_override_options {#set_structured_text_bidi_override_options}

```gdscript
void set_structured_text_bidi_override_options(args: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text {#set_text}

```gdscript
void set_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_direction {#set_text_direction}

```gdscript
void set_text_direction(direction: Control.TextDirection)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_overrun_behavior {#set_text_overrun_behavior}

```gdscript
void set_text_overrun_behavior(overrun_behavior: TextServer.OverrunBehavior)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_underline_mode {#set_underline_mode}

```gdscript
void set_underline_mode(underline_mode: LinkButton.UnderlineMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_uri {#set_uri}

```gdscript
void set_uri(uri: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

The button's text that will be displayed inside the button's area.

### underline {#underline}

```gdscript
underline: int
```

- void set_underline_mode(value: UnderlineMode)
- UnderlineMode get_underline_mode()

The underline mode to use for the text.

### uri {#uri}

```gdscript
uri: String
```

- void set_uri(value: String)
- String get_uri()

The URI [https://en.wikipedia.org/wiki/Uniform_Resource_Identifier] for this LinkButton. If set to a valid URI, pressing the button opens the URI using the operating system's default program for the protocol (via OS.shell_open()). HTTP and HTTPS URLs open the default web browser.

```
uri = "https://godotengine.org"  # Opens the URL in the default web browser.
uri = "C:\SomeFolder"  # Opens the file explorer at the given path.
uri = "C:\SomeImage.png"  # Opens the given image in the default viewing app.
```

```
Uri = "https://godotengine.org"; // Opens the URL in the default web browser.
Uri = "C:\SomeFolder"; // Opens the file explorer at the given path.
Uri = "C:\SomeImage.png"; // Opens the given image in the default viewing app.
```

### text_overrun_behavior {#text_overrun_behavior}

```gdscript
text_overrun_behavior: int
```

- void set_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_text_overrun_behavior()

Sets the clipping behavior when the text exceeds the node's bounding rectangle.

### ellipsis_char {#ellipsis_char}

```gdscript
ellipsis_char: String
```

- void set_ellipsis_char(value: String)
- String get_ellipsis_char()

Ellipsis character used for text clipping.

### text_direction {#text_direction}

```gdscript
text_direction: int
```

- void set_text_direction(value: TextDirection)
- TextDirection get_text_direction()

Base text writing direction.

### language {#language}

```gdscript
language: String
```

- void set_language(value: String)
- String get_language()

Language code used for line-breaking and text shaping algorithms. If left empty, the current locale is used instead.

### structured_text_bidi_override {#structured_text_bidi_override}

```gdscript
structured_text_bidi_override: int
```

- void set_structured_text_bidi_override(value: StructuredTextParser)
- StructuredTextParser get_structured_text_bidi_override()

Set BiDi algorithm override for the structured text.

### structured_text_bidi_override_options {#structured_text_bidi_override_options}

```gdscript
structured_text_bidi_override_options: Array
```

- void set_structured_text_bidi_override_options(value: Array)
- Array get_structured_text_bidi_override_options()

Set additional options for BiDi override.

## 枚举

### UnderlineMode {#enum-UnderlineMode}

```gdscript
enum UnderlineMode { UNDERLINE_MODE_ALWAYS = 0, UNDERLINE_MODE_ON_HOVER = 1, UNDERLINE_MODE_NEVER = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UNDERLINE_MODE_ALWAYS {#UNDERLINE_MODE_ALWAYS}

```gdscript
UNDERLINE_MODE_ALWAYS = 0
```

The LinkButton will always show an underline at the bottom of its text.

#### UNDERLINE_MODE_ON_HOVER {#UNDERLINE_MODE_ON_HOVER}

```gdscript
UNDERLINE_MODE_ON_HOVER = 1
```

The LinkButton will show an underline at the bottom of its text when the mouse cursor is over it.

#### UNDERLINE_MODE_NEVER {#UNDERLINE_MODE_NEVER}

```gdscript
UNDERLINE_MODE_NEVER = 2
```

The LinkButton will never show an underline at the bottom of its text.

---
