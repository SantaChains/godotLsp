# Window

**继承**：[Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **Window**

> A node that creates a window. The window can either be a native system window or embedded inside another Window (see Viewport.gui_embed_subwindows).

<details><summary>官方英文描述</summary>

A node that creates a window. The window can either be a native system window or embedded inside another Window (see Viewport.gui_embed_subwindows).

At runtime, Windows will not close automatically when requested. You need to handle it manually using the close_requested signal (this applies both to pressing the close button and clicking outside of a popup).

</details>

## 方法

### _get_contents_minimum_size {#_get_contents_minimum_size}

```gdscript
Vector2 _get_contents_minimum_size() virtual const
```

Virtual method to be implemented by the user. Overrides the value returned by get_contents_minimum_size().

### add_theme_color_override {#add_theme_color_override}

```gdscript
void add_theme_color_override(name: StringName, color: Color)
```

Creates a local override for a theme Color with the specified name. Local overrides always take precedence when fetching theme items for the control. An override can be removed with remove_theme_color_override().

See also get_theme_color() and Control.add_theme_color_override() for more details.

### add_theme_constant_override {#add_theme_constant_override}

```gdscript
void add_theme_constant_override(name: StringName, constant: int)
```

Creates a local override for a theme constant with the specified name. Local overrides always take precedence when fetching theme items for the control. An override can be removed with remove_theme_constant_override().

See also get_theme_constant().

### add_theme_font_override {#add_theme_font_override}

```gdscript
void add_theme_font_override(name: StringName, font: Font)
```

Creates a local override for a theme Font with the specified name. Local overrides always take precedence when fetching theme items for the control. An override can be removed with remove_theme_font_override().

See also get_theme_font().

### add_theme_font_size_override {#add_theme_font_size_override}

```gdscript
void add_theme_font_size_override(name: StringName, font_size: int)
```

Creates a local override for a theme font size with the specified name. Local overrides always take precedence when fetching theme items for the control. An override can be removed with remove_theme_font_size_override().

See also get_theme_font_size().

### add_theme_icon_override {#add_theme_icon_override}

```gdscript
void add_theme_icon_override(name: StringName, texture: Texture2D)
```

Creates a local override for a theme icon with the specified name. Local overrides always take precedence when fetching theme items for the control. An override can be removed with remove_theme_icon_override().

See also get_theme_icon().

### add_theme_stylebox_override {#add_theme_stylebox_override}

```gdscript
void add_theme_stylebox_override(name: StringName, stylebox: StyleBox)
```

Creates a local override for a theme StyleBox with the specified name. Local overrides always take precedence when fetching theme items for the control. An override can be removed with remove_theme_stylebox_override().

See also get_theme_stylebox() and Control.add_theme_stylebox_override() for more details.

### begin_bulk_theme_override {#begin_bulk_theme_override}

```gdscript
void begin_bulk_theme_override()
```

Prevents *_theme_*_override methods from emitting NOTIFICATION_THEME_CHANGED until end_bulk_theme_override() is called.

### can_draw {#can_draw}

```gdscript
bool can_draw() const
```

Returns whether the window is being drawn to the screen.

### child_controls_changed {#child_controls_changed}

```gdscript
void child_controls_changed()
```

Requests an update of the Window size to fit underlying Control nodes.

### end_bulk_theme_override {#end_bulk_theme_override}

```gdscript
void end_bulk_theme_override()
```

Ends a bulk theme override update. See begin_bulk_theme_override().

### get_accessibility_description {#get_accessibility_description}

```gdscript
String get_accessibility_description() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_accessibility_name {#get_accessibility_name}

```gdscript
String get_accessibility_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_content_scale_aspect {#get_content_scale_aspect}

```gdscript
Window.ContentScaleAspect get_content_scale_aspect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_content_scale_factor {#get_content_scale_factor}

```gdscript
float get_content_scale_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_content_scale_mode {#get_content_scale_mode}

```gdscript
Window.ContentScaleMode get_content_scale_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_content_scale_size {#get_content_scale_size}

```gdscript
Vector2i get_content_scale_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_content_scale_stretch {#get_content_scale_stretch}

```gdscript
Window.ContentScaleStretch get_content_scale_stretch() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_contents_minimum_size {#get_contents_minimum_size}

```gdscript
Vector2 get_contents_minimum_size() const
```

Returns the combined minimum size from the child Control nodes of the window. Use child_controls_changed() to update it when child nodes have changed.

The value returned by this method can be overridden with _get_contents_minimum_size().

### get_current_screen {#get_current_screen}

```gdscript
int get_current_screen() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_flag {#get_flag}

```gdscript
bool get_flag(flag: Window.Flags) const
```

Returns true if the flag is set.

### get_focused_window {#get_focused_window}

```gdscript
static Window get_focused_window()
```

Returns the focused window.

### get_force_native {#get_force_native}

```gdscript
bool get_force_native() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_initial_position {#get_initial_position}

```gdscript
Window.WindowInitialPosition get_initial_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_keep_title_visible {#get_keep_title_visible}

```gdscript
bool get_keep_title_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_layout_direction {#get_layout_direction}

```gdscript
Window.LayoutDirection get_layout_direction() const
```

Returns layout direction and text writing direction.

### get_max_size {#get_max_size}

```gdscript
Vector2i get_max_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_size {#get_min_size}

```gdscript
Vector2i get_min_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mode {#get_mode}

```gdscript
Window.Mode get_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mouse_passthrough_polygon {#get_mouse_passthrough_polygon}

```gdscript
PackedVector2Array get_mouse_passthrough_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_nonclient_area {#get_nonclient_area}

```gdscript
Rect2i get_nonclient_area() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_output_max_linear_value {#get_output_max_linear_value}

```gdscript
float get_output_max_linear_value() const
```

Returns the maximum value for linear color components that can be displayed in this window, regardless of SDR or HDR output. Returns 1.0 if HDR is not enabled or not supported. The output_max_linear_value_changed signal will be emitted whenever this value changes.

This value is used by tonemapping and other Environment effects to ensure that bright colors are presented in the range that can be displayed by this window. When using this maximum linear value in your project, it should only be used to present colors directly to the screen without tonemapping and without influencing lighting, post-processing effects, or surrounding color. The following is an example that produces the brightest purple color that the screen can produce:

```
func _process(_delta):
    # output_max_linear_value may change often, so do this every frame.
    var max_linear_value = get_window().get_output_max_linear_value()
    # Replace this with your color:
    var original_color = Color.PURPLE
    # Normalize to max_linear_value to produce the brightest color possible,
    # regardless of SDR or HDR output:
    var bright_color = normalize_color(original_color, max_linear_value)
func normalize_color(srgb_color, max_linear_value = 1.0):
    # Color must be linear-encoded to use math operations.
    var linear_color = srgb_color.srgb_to_linear()
    var max_rgb_value = maxf(linear_color.r, maxf(linear_color.g, linear_color.b))
    var brightness_scale = max_linear_value / max_rgb_value
    linear_color *= brightness_scale
    # Undo changes to the alpha channel, which should not be modified.
    linear_color.a = srgb_color.a
    # Convert back to nonlinear sRGB encoding, which is required for Color in
    # Godot unless stated otherwise.
    return linear_color.linear_to_srgb()
```

Note: You will need to convert sRGB colors to linear before multiplying by this value to get correct results.

### get_position {#get_position}

```gdscript
Vector2i get_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position_with_decorations {#get_position_with_decorations}

```gdscript
Vector2i get_position_with_decorations() const
```

Returns the window's position including its border.

Note: If visible is false, this method returns the same value as position.

### get_size {#get_size}

```gdscript
Vector2i get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size_with_decorations {#get_size_with_decorations}

```gdscript
Vector2i get_size_with_decorations() const
```

Returns the window's size including its border.

Note: If visible is false, this method returns the same value as size.

### get_theme {#get_theme}

```gdscript
Theme get_theme() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_theme_color {#get_theme_color}

```gdscript
Color get_theme_color(name: StringName, theme_type: StringName = &"") const
```

Returns a Color from the first matching Theme in the tree if that Theme has a color item with the specified name and theme_type.

See Control.get_theme_color() for more details.

### get_theme_constant {#get_theme_constant}

```gdscript
int get_theme_constant(name: StringName, theme_type: StringName = &"") const
```

Returns a constant from the first matching Theme in the tree if that Theme has a constant item with the specified name and theme_type.

See Control.get_theme_color() for more details.

### get_theme_default_base_scale {#get_theme_default_base_scale}

```gdscript
float get_theme_default_base_scale() const
```

Returns the default base scale value from the first matching Theme in the tree if that Theme has a valid Theme.default_base_scale value.

See Control.get_theme_color() for details.

### get_theme_default_font {#get_theme_default_font}

```gdscript
Font get_theme_default_font() const
```

Returns the default font from the first matching Theme in the tree if that Theme has a valid Theme.default_font value.

See Control.get_theme_color() for details.

### get_theme_default_font_size {#get_theme_default_font_size}

```gdscript
int get_theme_default_font_size() const
```

Returns the default font size value from the first matching Theme in the tree if that Theme has a valid Theme.default_font_size value.

See Control.get_theme_color() for details.

### get_theme_font {#get_theme_font}

```gdscript
Font get_theme_font(name: StringName, theme_type: StringName = &"") const
```

Returns a Font from the first matching Theme in the tree if that Theme has a font item with the specified name and theme_type.

See Control.get_theme_color() for details.

### get_theme_font_size {#get_theme_font_size}

```gdscript
int get_theme_font_size(name: StringName, theme_type: StringName = &"") const
```

Returns a font size from the first matching Theme in the tree if that Theme has a font size item with the specified name and theme_type.

See Control.get_theme_color() for details.

### get_theme_icon {#get_theme_icon}

```gdscript
Texture2D get_theme_icon(name: StringName, theme_type: StringName = &"") const
```

Returns an icon from the first matching Theme in the tree if that Theme has an icon item with the specified name and theme_type.

See Control.get_theme_color() for details.

### get_theme_stylebox {#get_theme_stylebox}

```gdscript
StyleBox get_theme_stylebox(name: StringName, theme_type: StringName = &"") const
```

Returns a StyleBox from the first matching Theme in the tree if that Theme has a stylebox item with the specified name and theme_type.

See Control.get_theme_color() for details.

### get_theme_type_variation {#get_theme_type_variation}

```gdscript
StringName get_theme_type_variation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title {#get_title}

```gdscript
String get_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_window_id {#get_window_id}

```gdscript
int get_window_id() const
```

Returns the ID of the window.

### grab_focus {#grab_focus}

```gdscript
void grab_focus()
```

Causes the window to grab focus, allowing it to receive user input.

### has_focus {#has_focus}

```gdscript
bool has_focus() const
```

Returns true if the window is focused.

### has_theme_color {#has_theme_color}

```gdscript
bool has_theme_color(name: StringName, theme_type: StringName = &"") const
```

Returns true if there is a matching Theme in the tree that has a color item with the specified name and theme_type.

See Control.get_theme_color() for details.

### has_theme_color_override {#has_theme_color_override}

```gdscript
bool has_theme_color_override(name: StringName) const
```

Returns true if there is a local override for a theme Color with the specified name in this Control node.

See add_theme_color_override().

### has_theme_constant {#has_theme_constant}

```gdscript
bool has_theme_constant(name: StringName, theme_type: StringName = &"") const
```

Returns true if there is a matching Theme in the tree that has a constant item with the specified name and theme_type.

See Control.get_theme_color() for details.

### has_theme_constant_override {#has_theme_constant_override}

```gdscript
bool has_theme_constant_override(name: StringName) const
```

Returns true if there is a local override for a theme constant with the specified name in this Control node.

See add_theme_constant_override().

### has_theme_font {#has_theme_font}

```gdscript
bool has_theme_font(name: StringName, theme_type: StringName = &"") const
```

Returns true if there is a matching Theme in the tree that has a font item with the specified name and theme_type.

See Control.get_theme_color() for details.

### has_theme_font_override {#has_theme_font_override}

```gdscript
bool has_theme_font_override(name: StringName) const
```

Returns true if there is a local override for a theme Font with the specified name in this Control node.

See add_theme_font_override().

### has_theme_font_size {#has_theme_font_size}

```gdscript
bool has_theme_font_size(name: StringName, theme_type: StringName = &"") const
```

Returns true if there is a matching Theme in the tree that has a font size item with the specified name and theme_type.

See Control.get_theme_color() for details.

### has_theme_font_size_override {#has_theme_font_size_override}

```gdscript
bool has_theme_font_size_override(name: StringName) const
```

Returns true if there is a local override for a theme font size with the specified name in this Control node.

See add_theme_font_size_override().

### has_theme_icon {#has_theme_icon}

```gdscript
bool has_theme_icon(name: StringName, theme_type: StringName = &"") const
```

Returns true if there is a matching Theme in the tree that has an icon item with the specified name and theme_type.

See Control.get_theme_color() for details.

### has_theme_icon_override {#has_theme_icon_override}

```gdscript
bool has_theme_icon_override(name: StringName) const
```

Returns true if there is a local override for a theme icon with the specified name in this Control node.

See add_theme_icon_override().

### has_theme_stylebox {#has_theme_stylebox}

```gdscript
bool has_theme_stylebox(name: StringName, theme_type: StringName = &"") const
```

Returns true if there is a matching Theme in the tree that has a stylebox item with the specified name and theme_type.

See Control.get_theme_color() for details.

### has_theme_stylebox_override {#has_theme_stylebox_override}

```gdscript
bool has_theme_stylebox_override(name: StringName) const
```

Returns true if there is a local override for a theme StyleBox with the specified name in this Control node.

See add_theme_stylebox_override().

### hide {#hide}

```gdscript
void hide()
```

Hides the window. This is not the same as minimized state. Hidden window can't be interacted with and needs to be made visible with show().

### is_auto_translating {#is_auto_translating}

```gdscript
bool is_auto_translating() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_embedded {#is_embedded}

```gdscript
bool is_embedded() const
```

Returns true if the window is currently embedded in another window.

### is_exclusive {#is_exclusive}

```gdscript
bool is_exclusive() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hdr_output_requested {#is_hdr_output_requested}

```gdscript
bool is_hdr_output_requested() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_layout_rtl {#is_layout_rtl}

```gdscript
bool is_layout_rtl() const
```

Returns true if the layout is right-to-left.

### is_maximize_allowed {#is_maximize_allowed}

```gdscript
bool is_maximize_allowed() const
```

Returns true if the window can be maximized (the maximize button is enabled).

### is_transient {#is_transient}

```gdscript
bool is_transient() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_transient_to_focused {#is_transient_to_focused}

```gdscript
bool is_transient_to_focused() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_font_oversampling {#is_using_font_oversampling}

```gdscript
bool is_using_font_oversampling() const
```

Returns true if font oversampling is enabled. See set_use_font_oversampling().

### is_visible {#is_visible}

```gdscript
bool is_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_wrapping_controls {#is_wrapping_controls}

```gdscript
bool is_wrapping_controls() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### move_to_center {#move_to_center}

```gdscript
void move_to_center()
```

Centers the window in the current screen. If the window is embedded, it is centered in the embedder Viewport instead.

### move_to_foreground {#move_to_foreground}

```gdscript
void move_to_foreground()
```

Deprecated: Use grab_focus() instead.

Causes the window to grab focus, allowing it to receive user input.

### popup {#popup}

```gdscript
void popup(rect: Rect2i = Rect2i(0, 0, 0, 0))
```

Shows the Window and makes it transient (see transient). If rect is provided, it will be set as the Window's size. Fails if called on the main window.

If ProjectSettings.display/window/subwindows/embed_subwindows is true (single-window mode), rect's coordinates are global and relative to the main window's top-left corner (excluding window decorations). If rect's position coordinates are negative, the window will be located outside the main window and may not be visible as a result.

If ProjectSettings.display/window/subwindows/embed_subwindows is false (multi-window mode), rect's coordinates are global and relative to the top-left corner of the leftmost screen. If rect's position coordinates are negative, the window will be placed at the top-left corner of the screen.

Note: rect must be in global coordinates if specified.

### popup_centered {#popup_centered}

```gdscript
void popup_centered(minsize: Vector2i = Vector2i(0, 0))
```

Popups the Window at the center of the current screen, with optionally given minimum size. If the Window is embedded, it will be centered in the parent Viewport instead.

Note: Calling it with the default value of minsize is equivalent to calling it with size.

### popup_centered_clamped {#popup_centered_clamped}

```gdscript
void popup_centered_clamped(minsize: Vector2i = Vector2i(0, 0), fallback_ratio: float = 0.75)
```

Popups the Window centered inside its parent Window. fallback_ratio determines the maximum size of the Window, in relation to its parent.

Note: Calling it with the default value of minsize is equivalent to calling it with size.

### popup_centered_ratio {#popup_centered_ratio}

```gdscript
void popup_centered_ratio(ratio: float = 0.8)
```

If Window is embedded, popups the Window centered inside its embedder and sets its size as a ratio of embedder's size.

If Window is a native window, popups the Window centered inside the screen of its parent Window and sets its size as a ratio of the screen size.

### popup_exclusive {#popup_exclusive}

```gdscript
void popup_exclusive(from_node: Node, rect: Rect2i = Rect2i(0, 0, 0, 0))
```

Attempts to parent this dialog to the last exclusive window relative to from_node, and then calls popup() on it. The dialog must have no current parent, otherwise the method fails.

See also set_unparent_when_invisible() and Node.get_last_exclusive_window().

### popup_exclusive_centered {#popup_exclusive_centered}

```gdscript
void popup_exclusive_centered(from_node: Node, minsize: Vector2i = Vector2i(0, 0))
```

Attempts to parent this dialog to the last exclusive window relative to from_node, and then calls popup_centered() on it. The dialog must have no current parent, otherwise the method fails.

See also set_unparent_when_invisible() and Node.get_last_exclusive_window().

### popup_exclusive_centered_clamped {#popup_exclusive_centered_clamped}

```gdscript
void popup_exclusive_centered_clamped(from_node: Node, minsize: Vector2i = Vector2i(0, 0), fallback_ratio: float = 0.75)
```

Attempts to parent this dialog to the last exclusive window relative to from_node, and then calls popup_centered_clamped() on it. The dialog must have no current parent, otherwise the method fails.

See also set_unparent_when_invisible() and Node.get_last_exclusive_window().

### popup_exclusive_centered_ratio {#popup_exclusive_centered_ratio}

```gdscript
void popup_exclusive_centered_ratio(from_node: Node, ratio: float = 0.8)
```

Attempts to parent this dialog to the last exclusive window relative to from_node, and then calls popup_centered_ratio() on it. The dialog must have no current parent, otherwise the method fails.

See also set_unparent_when_invisible() and Node.get_last_exclusive_window().

### popup_exclusive_on_parent {#popup_exclusive_on_parent}

```gdscript
void popup_exclusive_on_parent(from_node: Node, parent_rect: Rect2i)
```

Attempts to parent this dialog to the last exclusive window relative to from_node, and then calls popup_on_parent() on it. The dialog must have no current parent, otherwise the method fails.

See also set_unparent_when_invisible() and Node.get_last_exclusive_window().

### popup_on_parent {#popup_on_parent}

```gdscript
void popup_on_parent(parent_rect: Rect2i)
```

Popups the Window with a position shifted by parent Window's position. If the Window is embedded, has the same effect as popup().

### remove_theme_color_override {#remove_theme_color_override}

```gdscript
void remove_theme_color_override(name: StringName)
```

Removes a local override for a theme Color with the specified name previously added by add_theme_color_override() or via the Inspector dock.

### remove_theme_constant_override {#remove_theme_constant_override}

```gdscript
void remove_theme_constant_override(name: StringName)
```

Removes a local override for a theme constant with the specified name previously added by add_theme_constant_override() or via the Inspector dock.

### remove_theme_font_override {#remove_theme_font_override}

```gdscript
void remove_theme_font_override(name: StringName)
```

Removes a local override for a theme Font with the specified name previously added by add_theme_font_override() or via the Inspector dock.

### remove_theme_font_size_override {#remove_theme_font_size_override}

```gdscript
void remove_theme_font_size_override(name: StringName)
```

Removes a local override for a theme font size with the specified name previously added by add_theme_font_size_override() or via the Inspector dock.

### remove_theme_icon_override {#remove_theme_icon_override}

```gdscript
void remove_theme_icon_override(name: StringName)
```

Removes a local override for a theme icon with the specified name previously added by add_theme_icon_override() or via the Inspector dock.

### remove_theme_stylebox_override {#remove_theme_stylebox_override}

```gdscript
void remove_theme_stylebox_override(name: StringName)
```

Removes a local override for a theme StyleBox with the specified name previously added by add_theme_stylebox_override() or via the Inspector dock.

### request_attention {#request_attention}

```gdscript
void request_attention()
```

Tells the OS that the Window needs an attention. This makes the window stand out in some way depending on the system, e.g. it might blink on the task bar.

### reset_size {#reset_size}

```gdscript
void reset_size()
```

Resets the size to the minimum size, which is the max of min_size and (if wrap_controls is enabled) get_contents_minimum_size(). This is equivalent to calling set_size(Vector2i()) (or any size below the minimum).

### set_accessibility_description {#set_accessibility_description}

```gdscript
void set_accessibility_description(description: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_accessibility_name {#set_accessibility_name}

```gdscript
void set_accessibility_name(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_translate {#set_auto_translate}

```gdscript
void set_auto_translate(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_content_scale_aspect {#set_content_scale_aspect}

```gdscript
void set_content_scale_aspect(aspect: Window.ContentScaleAspect)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_content_scale_factor {#set_content_scale_factor}

```gdscript
void set_content_scale_factor(factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_content_scale_mode {#set_content_scale_mode}

```gdscript
void set_content_scale_mode(mode: Window.ContentScaleMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_content_scale_size {#set_content_scale_size}

```gdscript
void set_content_scale_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_content_scale_stretch {#set_content_scale_stretch}

```gdscript
void set_content_scale_stretch(stretch: Window.ContentScaleStretch)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current_screen {#set_current_screen}

```gdscript
void set_current_screen(index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclusive {#set_exclusive}

```gdscript
void set_exclusive(exclusive: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flag {#set_flag}

```gdscript
void set_flag(flag: Window.Flags, enabled: bool)
```

Sets a specified window flag.

### set_force_native {#set_force_native}

```gdscript
void set_force_native(force_native: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hdr_output_requested {#set_hdr_output_requested}

```gdscript
void set_hdr_output_requested(requested: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ime_active {#set_ime_active}

```gdscript
void set_ime_active(active: bool)
```

If active is true, enables system's native IME (Input Method Editor).

### set_ime_position {#set_ime_position}

```gdscript
void set_ime_position(position: Vector2i)
```

Moves IME to the given position.

### set_initial_position {#set_initial_position}

```gdscript
void set_initial_position(initial_position: Window.WindowInitialPosition)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_title_visible {#set_keep_title_visible}

```gdscript
void set_keep_title_visible(title_visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layout_direction {#set_layout_direction}

```gdscript
void set_layout_direction(direction: Window.LayoutDirection)
```

Sets layout direction and text writing direction. Right-to-left layouts are necessary for certain languages (e.g. Arabic and Hebrew).

### set_max_size {#set_max_size}

```gdscript
void set_max_size(max_size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_size {#set_min_size}

```gdscript
void set_min_size(min_size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mode {#set_mode}

```gdscript
void set_mode(mode: Window.Mode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mouse_passthrough_polygon {#set_mouse_passthrough_polygon}

```gdscript
void set_mouse_passthrough_polygon(polygon: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_nonclient_area {#set_nonclient_area}

```gdscript
void set_nonclient_area(area: Rect2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position {#set_position}

```gdscript
void set_position(position: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_taskbar_progress_state {#set_taskbar_progress_state}

```gdscript
void set_taskbar_progress_state(state: DisplayServer.ProgressState)
```

Sets the type and state of the progress bar on the taskbar/dock icon of the Window. See ProgressState for possible values and how each mode behaves.

Note: This method is implemented only on Windows and macOS.

### set_taskbar_progress_value {#set_taskbar_progress_value}

```gdscript
void set_taskbar_progress_value(value: float)
```

Creates a progress bar on the taskbar/dock icon of the Window if it does not exist, sets the progress of the icon.

value acts as a relative percentage value, ranges from 0.0 (lowest) to 1.0 (highest).

Note: This method is implemented only on Windows and macOS.

### set_theme {#set_theme}

```gdscript
void set_theme(theme: Theme)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_theme_type_variation {#set_theme_type_variation}

```gdscript
void set_theme_type_variation(theme_type: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title {#set_title}

```gdscript
void set_title(title: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transient {#set_transient}

```gdscript
void set_transient(transient: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transient_to_focused {#set_transient_to_focused}

```gdscript
void set_transient_to_focused(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_unparent_when_invisible {#set_unparent_when_invisible}

```gdscript
void set_unparent_when_invisible(unparent: bool)
```

If unparent is true, the window is automatically unparented when going invisible.

Note: Make sure to keep a reference to the node, otherwise it will be orphaned. You also need to manually call Node.queue_free() to free the window if it's not parented.

### set_use_font_oversampling {#set_use_font_oversampling}

```gdscript
void set_use_font_oversampling(enable: bool)
```

Enables font oversampling. This makes fonts look better when they are scaled up.

### set_visible {#set_visible}

```gdscript
void set_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wrap_controls {#set_wrap_controls}

```gdscript
void set_wrap_controls(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### show {#show}

```gdscript
void show()
```

Makes the Window appear. This enables interactions with the Window and doesn't change any of its property other than visibility (unlike e.g. popup()).

### start_drag {#start_drag}

```gdscript
void start_drag()
```

Starts an interactive drag operation on the window, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's title bar. Using this method allows the window to participate in space switching, tiling, and other system features.

### start_resize {#start_resize}

```gdscript
void start_resize(edge: DisplayServer.WindowResizeEdge)
```

Starts an interactive resize operation on the window, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's edge.

## 属性

### mode {#mode}

```gdscript
mode: int
```

- void set_mode(value: Mode)
- Mode get_mode()

Set's the window's current mode.

Note: Fullscreen mode is not exclusive full screen on Windows and Linux.

Note: This method only works with native windows, i.e. the main window and Window-derived nodes when Viewport.gui_embed_subwindows is disabled in the main viewport.

### title {#title}

```gdscript
title: String
```

- void set_title(value: String)
- String get_title()

The window's title. If the Window is native, title styles set in Theme will have no effect.

### initial_position {#initial_position}

```gdscript
initial_position: int
```

- void set_initial_position(value: WindowInitialPosition)
- WindowInitialPosition get_initial_position()

Specifies the initial type of position for the Window.

### position {#position}

```gdscript
position: Vector2i
```

- void set_position(value: Vector2i)
- Vector2i get_position()

The window's position in pixels.

If ProjectSettings.display/window/subwindows/embed_subwindows is false, the position is in absolute screen coordinates. This typically applies to editor plugins. If the setting is true, the window's position is in the coordinates of its parent Viewport.

Note: This property only works if initial_position is set to WINDOW_INITIAL_POSITION_ABSOLUTE.

### size {#size}

```gdscript
size: Vector2i
```

- void set_size(value: Vector2i)
- Vector2i get_size()

The window's size in pixels. See also content_scale_size, which doesn't set the window's physical size but affects how scaling works relative to the current content_scale_mode.

### current_screen {#current_screen}

```gdscript
current_screen: int
```

- void set_current_screen(value: int)
- int get_current_screen()

The screen the window is currently on.

### nonclient_area {#nonclient_area}

```gdscript
nonclient_area: Rect2i
```

- void set_nonclient_area(value: Rect2i)
- Rect2i get_nonclient_area()

If set, defines the window's custom decoration area which will receive mouse input, even if normal input to the window is blocked (such as when it has an exclusive child opened). See also nonclient_window_input.

### mouse_passthrough_polygon {#mouse_passthrough_polygon}

```gdscript
mouse_passthrough_polygon: PackedVector2Array
```

- void set_mouse_passthrough_polygon(value: PackedVector2Array)
- PackedVector2Array get_mouse_passthrough_polygon()

Sets a polygonal region of the window which accepts mouse events. Mouse events outside the region will be passed through.

Passing an empty array will disable passthrough support (all mouse events will be intercepted by the window, which is the default behavior).

```
# Set region, using Path2D node.
$Window.mouse_passthrough_polygon = $Path2D.curve.get_baked_points()
# Set region, using Polygon2D node.
$Window.mouse_passthrough_polygon = $Polygon2D.polygon
# Reset region to default.
$Window.mouse_passthrough_polygon = []
```

```
// Set region, using Path2D node.
GetNode("Window").MousePassthroughPolygon = GetNode("Path2D").Curve.GetBakedPoints();
// Set region, using Polygon2D node.
GetNode("Window").MousePassthroughPolygon = GetNode("Polygon2D").Polygon;
// Reset region to default.
GetNode("Window").MousePassthroughPolygon = [];
```

Note: This property is ignored if mouse_passthrough is set to true.

Note: On Windows, the portion of a window that lies outside the region is not drawn, while on Linux (X11) and macOS it is.

Note: This property is implemented on Linux (X11), macOS and Windows.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.

### visible {#visible}

```gdscript
visible: bool
```

- void set_visible(value: bool)
- bool is_visible()

If true, the window is visible.

### wrap_controls {#wrap_controls}

```gdscript
wrap_controls: bool
```

- void set_wrap_controls(value: bool)
- bool is_wrapping_controls()

If true, the window's size will automatically update when a child node is added or removed, ignoring min_size if the new size is bigger.

If false, you need to call child_controls_changed() manually.

### transient {#transient}

```gdscript
transient: bool
```

- void set_transient(value: bool)
- bool is_transient()

If true, the Window is transient, i.e. it's considered a child of another Window. The transient window will be destroyed with its transient parent and will return focus to their parent when closed. The transient window is displayed on top of a non-exclusive full-screen parent window. Transient windows can't enter full-screen mode.

Note that behavior might be different depending on the platform.

### transient_to_focused {#transient_to_focused}

```gdscript
transient_to_focused: bool
```

- void set_transient_to_focused(value: bool)
- bool is_transient_to_focused()

If true, and the Window is transient, this window will (at the time of becoming visible) become transient to the currently focused window instead of the immediate parent window in the hierarchy. Note that the transient parent is assigned at the time this window becomes visible, so changing it afterwards has no effect until re-shown.

### exclusive {#exclusive}

```gdscript
exclusive: bool
```

- void set_exclusive(value: bool)
- bool is_exclusive()

If true, the Window will be in exclusive mode. Exclusive windows are always on top of their parent and will block all input going to the parent Window.

Needs transient enabled to work.

### unresizable {#unresizable}

```gdscript
unresizable: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the window can't be resized.

### borderless {#borderless}

```gdscript
borderless: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the window will have no borders.

### always_on_top {#always_on_top}

```gdscript
always_on_top: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the window will be on top of all other windows. Does not work if transient is enabled.

### transparent {#transparent}

```gdscript
transparent: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window's background can be transparent. This is best used with embedded windows.

Note: Transparency support is implemented on Linux, macOS and Windows, but availability might vary depending on GPU driver, display manager, and compositor capabilities.

Note: This property has no effect if ProjectSettings.display/window/per_pixel_transparency/allowed is set to false.

### unfocusable {#unfocusable}

```gdscript
unfocusable: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window can't be focused nor interacted with. It can still be visible.

### popup_window {#popup_window}

```gdscript
popup_window: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window will be considered a popup. Popups are sub-windows that don't show as separate windows in system's window manager's window list and will send close request when anything is clicked outside of them (unless exclusive is enabled).

### extend_to_title {#extend_to_title}

```gdscript
extend_to_title: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window contents is expanded to the full size of the window, window title bar is transparent.

Note: This property is implemented only on macOS.

Note: This property only works with native windows.

### mouse_passthrough {#mouse_passthrough}

```gdscript
mouse_passthrough: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, all mouse events will be passed to the underlying window of the same application. See also mouse_passthrough_polygon.

Note: This property is implemented on Linux (X11), macOS and Windows.

Note: This property only works with native windows.

### sharp_corners {#sharp_corners}

```gdscript
sharp_corners: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window will override the OS window style to display sharp corners.

Note: This property is implemented only on Windows (11).

Note: This property only works with native windows.

### exclude_from_capture {#exclude_from_capture}

```gdscript
exclude_from_capture: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window is excluded from screenshots taken by DisplayServer.screen_get_image(), DisplayServer.screen_get_image_rect(), and DisplayServer.screen_get_pixel().

Note: This property is implemented on macOS and Windows.

Note: Enabling this setting will prevent standard screenshot methods from capturing a window image, but does NOT guarantee that other apps won't be able to capture an image. It should not be used as a DRM or security measure.

### popup_wm_hint {#popup_wm_hint}

```gdscript
popup_wm_hint: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window will signal to the window manager that it is supposed to be an implementation-defined "popup" (usually a floating, borderless, untileable and immovable child window).

### minimize_disabled {#minimize_disabled}

```gdscript
minimize_disabled: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window's minimize button is disabled.

Note: If both minimize and maximize buttons are disabled, buttons are fully hidden, and only close button is visible.

Note: This property is implemented only on macOS and Windows.

### maximize_disabled {#maximize_disabled}

```gdscript
maximize_disabled: bool
```

- void set_flag(flag: Flags, enabled: bool)
- bool get_flag(flag: Flags) const

If true, the Window's maximize button is disabled.

Note: If both minimize and maximize buttons are disabled, buttons are fully hidden, and only close button is visible.

Note: This property is implemented only on macOS and Windows.

### force_native {#force_native}

```gdscript
force_native: bool
```

- void set_force_native(value: bool)
- bool get_force_native()

If true, native window will be used regardless of parent viewport and project settings.

### min_size {#min_size}

```gdscript
min_size: Vector2i
```

- void set_min_size(value: Vector2i)
- Vector2i get_min_size()

If non-zero, the Window can't be resized to be smaller than this size.

Note: This property will be ignored in favor of get_contents_minimum_size() if wrap_controls is enabled and if its size is bigger.

### max_size {#max_size}

```gdscript
max_size: Vector2i
```

- void set_max_size(value: Vector2i)
- Vector2i get_max_size()

If non-zero, the Window can't be resized to be bigger than this size.

Note: This property will be ignored if the value is lower than min_size.

### keep_title_visible {#keep_title_visible}

```gdscript
keep_title_visible: bool
```

- void set_keep_title_visible(value: bool)
- bool get_keep_title_visible()

If true, the Window width is expanded to keep the title bar text fully visible.

### content_scale_size {#content_scale_size}

```gdscript
content_scale_size: Vector2i
```

- void set_content_scale_size(value: Vector2i)
- Vector2i get_content_scale_size()

The content's base size in "virtual" pixels. Not to be confused with size, which sets the actual window's physical size in pixels. If set to a value greater than 0 and content_scale_mode is set to a value other than CONTENT_SCALE_MODE_DISABLED, the Window's content will be scaled when the window is resized to a different size. Higher values will make the content appear smaller, as it will be able to fit more of the project in view. On the root Window, this is set to match ProjectSettings.display/window/size/viewport_width and ProjectSettings.display/window/size/viewport_height by default.

For example, when using CONTENT_SCALE_MODE_CANVAS_ITEMS and content_scale_size set to Vector2i(1280, 720), using a window size of 2560×1440 will make 2D elements appear at double their original size, as the content is scaled by a factor of 2.0 (2560.0 / 1280.0 = 2.0, 1440.0 / 720.0 = 2.0).

See the Base size section of the Multiple resolutions documentation for details.

### content_scale_mode {#content_scale_mode}

```gdscript
content_scale_mode: int
```

- void set_content_scale_mode(value: ContentScaleMode)
- ContentScaleMode get_content_scale_mode()

Specifies how the content is scaled when the Window is resized.

### content_scale_aspect {#content_scale_aspect}

```gdscript
content_scale_aspect: int
```

- void set_content_scale_aspect(value: ContentScaleAspect)
- ContentScaleAspect get_content_scale_aspect()

Specifies how the content's aspect behaves when the Window is resized. The base aspect is determined by content_scale_size.

### content_scale_stretch {#content_scale_stretch}

```gdscript
content_scale_stretch: int
```

- void set_content_scale_stretch(value: ContentScaleStretch)
- ContentScaleStretch get_content_scale_stretch()

The policy to use to determine the final scale factor for 2D elements. This affects how content_scale_factor is applied, in addition to the automatic scale factor determined by content_scale_size.

### content_scale_factor {#content_scale_factor}

```gdscript
content_scale_factor: float
```

- void set_content_scale_factor(value: float)
- float get_content_scale_factor()

Specifies the base scale of Window's content when its size is equal to content_scale_size. See also Viewport.get_stretch_transform().

### hdr_output_requested {#hdr_output_requested}

```gdscript
hdr_output_requested: bool
```

- void set_hdr_output_requested(value: bool)
- bool is_hdr_output_requested()

If true, requests HDR output for the Window, falling back to SDR if not supported, and automatically switching between HDR and SDR as the window moves between screens, screen capabilities change, or system settings are modified. This will internally force Viewport.use_hdr_2d to be enabled on the main Viewport. All other SubViewport of this Window must have their Viewport.use_hdr_2d property enabled to produce HDR output.

### auto_translate {#auto_translate}

```gdscript
auto_translate: bool
```

- void set_auto_translate(value: bool)
- bool is_auto_translating()

Deprecated: Use Node.auto_translate_mode and Node.can_auto_translate() instead.

Toggles if any text should automatically change to its translated version depending on the current locale.

### accessibility_name {#accessibility_name}

```gdscript
accessibility_name: String
```

- void set_accessibility_name(value: String)
- String get_accessibility_name()

The human-readable node name that is reported to assistive apps.

### accessibility_description {#accessibility_description}

```gdscript
accessibility_description: String
```

- void set_accessibility_description(value: String)
- String get_accessibility_description()

The human-readable node description that is reported to assistive apps.

### theme {#theme}

```gdscript
theme: Theme
```

- void set_theme(value: Theme)
- Theme get_theme()

The Theme resource this node and all its Control and Window children use. If a child node has its own Theme resource set, theme items are merged with child's definitions having higher priority.

Note: Window styles will have no effect unless the window is embedded.

### theme_type_variation {#theme_type_variation}

```gdscript
theme_type_variation: String
```

- void set_theme_type_variation(value: StringName)
- StringName get_theme_type_variation()

The name of a theme type variation used by this Window to look up its own theme items. See Control.theme_type_variation for more details.

## 信号

### window_input {#window_input}

```gdscript
signal window_input(event: InputEvent)
```

Emitted when the Window is currently focused and receives any input, passing the received event as an argument. The event's position, if present, is in the embedder's coordinate system.

### nonclient_window_input {#nonclient_window_input}

```gdscript
signal nonclient_window_input(event: InputEvent)
```

Emitted when the mouse event is received by the custom decoration area defined by nonclient_area, and normal input to the window is blocked (such as when it has an exclusive child opened). event's position is in the embedder's coordinate system.

### files_dropped {#files_dropped}

```gdscript
signal files_dropped(files: PackedStringArray)
```

Emitted when files are dragged from the OS file manager and dropped in the game window. The argument is a list of file paths.

```gdscript
func _ready():
    get_window().files_dropped.connect(on_files_dropped)
func on_files_dropped(files):
    print(files)
```

Note: This signal only works with native windows, i.e. the main window and Window-derived nodes when Viewport.gui_embed_subwindows is disabled in the main viewport.

### mouse_entered {#mouse_entered}

```gdscript
signal mouse_entered()
```

Emitted when the mouse cursor enters the Window's visible area, that is not occluded behind other Controls or windows, provided its Viewport.gui_disable_input is false and regardless if it's currently focused or not.

### mouse_exited {#mouse_exited}

```gdscript
signal mouse_exited()
```

Emitted when the mouse cursor leaves the Window's visible area, that is not occluded behind other Controls or windows, provided its Viewport.gui_disable_input is false and regardless if it's currently focused or not.

### focus_entered {#focus_entered}

```gdscript
signal focus_entered()
```

Emitted when the Window gains focus.

### focus_exited {#focus_exited}

```gdscript
signal focus_exited()
```

Emitted when the Window loses its focus.

### close_requested {#close_requested}

```gdscript
signal close_requested()
```

Emitted when the Window's close button is pressed or when popup_window is enabled and user clicks outside the window.

This signal can be used to handle window closing, e.g. by connecting it to hide().

### go_back_requested {#go_back_requested}

```gdscript
signal go_back_requested()
```

Emitted when a go back request is sent (e.g. pressing the "Back" button on Android), right after Node.NOTIFICATION_WM_GO_BACK_REQUEST.

### visibility_changed {#visibility_changed}

```gdscript
signal visibility_changed()
```

Emitted when Window is made visible or disappears.

### about_to_popup {#about_to_popup}

```gdscript
signal about_to_popup()
```

Emitted right after popup() call, before the Window appears or does anything.

### theme_changed {#theme_changed}

```gdscript
signal theme_changed()
```

Emitted when the NOTIFICATION_THEME_CHANGED notification is sent.

### dpi_changed {#dpi_changed}

```gdscript
signal dpi_changed()
```

Emitted when the Window's DPI changes as a result of OS-level changes (e.g. moving the window from a Retina display to a lower resolution one).

Note: Only implemented on macOS and Linux (Wayland).

### titlebar_changed {#titlebar_changed}

```gdscript
signal titlebar_changed()
```

Emitted when window title bar decorations are changed, e.g. macOS window enter/exit full screen mode, or extend-to-title flag is changed.

### title_changed {#title_changed}

```gdscript
signal title_changed()
```

Emitted when window title bar text is changed.

### output_max_linear_value_changed {#output_max_linear_value_changed}

```gdscript
signal output_max_linear_value_changed(output_max_linear_value: float)
```

Emitted when the output max linear value returned by get_output_max_linear_value() has changed. This occurs when HDR output is enabled or disabled and when any HDR output luminance values of the window have changed, such as when the player adjusts their screen brightness setting or moves the window to a different screen. output_max_linear_value is the new value.

## 枚举

### Mode {#enum-Mode}

```gdscript
enum Mode { MODE_WINDOWED = 0, MODE_MINIMIZED = 1, MODE_MAXIMIZED = 2, MODE_FULLSCREEN = 3, MODE_EXCLUSIVE_FULLSCREEN = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_WINDOWED {#MODE_WINDOWED}

```gdscript
MODE_WINDOWED = 0
```

Windowed mode, i.e. Window doesn't occupy the whole screen (unless set to the size of the screen).

#### MODE_MINIMIZED {#MODE_MINIMIZED}

```gdscript
MODE_MINIMIZED = 1
```

Minimized window mode, i.e. Window is not visible and available on window manager's window list. Normally happens when the minimize button is pressed.

#### MODE_MAXIMIZED {#MODE_MAXIMIZED}

```gdscript
MODE_MAXIMIZED = 2
```

Maximized window mode, i.e. Window will occupy whole screen area except task bar and still display its borders. Normally happens when the maximize button is pressed.

#### MODE_FULLSCREEN {#MODE_FULLSCREEN}

```gdscript
MODE_FULLSCREEN = 3
```

Full screen mode with full multi-window support.

Full screen window covers the entire display area of a screen and has no decorations. The display's video mode is not changed.

On Android: This enables immersive mode.

On macOS: A new desktop is used to display the running project.

Note: Regardless of the platform, enabling full screen will change the window size to match the monitor's size. Therefore, make sure your project supports multiple resolutions when enabling full screen mode.

#### MODE_EXCLUSIVE_FULLSCREEN {#MODE_EXCLUSIVE_FULLSCREEN}

```gdscript
MODE_EXCLUSIVE_FULLSCREEN = 4
```

A single window full screen mode. This mode has less overhead, but only one window can be open on a given screen at a time (opening a child window or application switching will trigger a full screen transition).

Full screen window covers the entire display area of a screen and has no border or decorations. The display's video mode is not changed.

Note: This mode might not work with screen recording software.

On Android: This enables immersive mode.

On Windows: Depending on video driver, full screen transition might cause screens to go black for a moment.

On macOS: A new desktop is used to display the running project. Exclusive full screen mode prevents Dock and Menu from showing up when the mouse pointer is hovering the edge of the screen.

On Linux (X11): Exclusive full screen mode bypasses compositor.

On Linux (Wayland): Equivalent to MODE_FULLSCREEN.

Note: Regardless of the platform, enabling full screen will change the window size to match the monitor's size. Therefore, make sure your project supports multiple resolutions when enabling full screen mode.

---

### Flags {#enum-Flags}

```gdscript
enum Flags { FLAG_RESIZE_DISABLED = 0, FLAG_BORDERLESS = 1, FLAG_ALWAYS_ON_TOP = 2, FLAG_TRANSPARENT = 3, FLAG_NO_FOCUS = 4, FLAG_POPUP = 5, FLAG_EXTEND_TO_TITLE = 6, FLAG_MOUSE_PASSTHROUGH = 7, FLAG_SHARP_CORNERS = 8, FLAG_EXCLUDE_FROM_CAPTURE = 9, FLAG_POPUP_WM_HINT = 10, FLAG_MINIMIZE_DISABLED = 11, FLAG_MAXIMIZE_DISABLED = 12, FLAG_MAX = 13 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_RESIZE_DISABLED {#FLAG_RESIZE_DISABLED}

```gdscript
FLAG_RESIZE_DISABLED = 0
```

The window can't be resized by dragging its resize grip. It's still possible to resize the window using size. This flag is ignored for full screen windows. Set with unresizable.

Note: This flag is implemented on Linux (X11), macOS, Windows, and embedded windows.

#### FLAG_BORDERLESS {#FLAG_BORDERLESS}

```gdscript
FLAG_BORDERLESS = 1
```

The window do not have native title bar and other decorations. This flag is ignored for full-screen windows. Set with borderless.

Note: This flag is implemented on Linux (X11/Wayland), macOS, Windows, and embedded windows.

#### FLAG_ALWAYS_ON_TOP {#FLAG_ALWAYS_ON_TOP}

```gdscript
FLAG_ALWAYS_ON_TOP = 2
```

The window is floating on top of all other windows. This flag is ignored for full-screen windows. Set with always_on_top.

Note: This flag is implemented on Linux (X11), macOS, Windows, and embedded windows.

#### FLAG_TRANSPARENT {#FLAG_TRANSPARENT}

```gdscript
FLAG_TRANSPARENT = 3
```

The window background can be transparent. Set with transparent.

Note: This flag has no effect if either ProjectSettings.display/window/per_pixel_transparency/allowed, or the window's Viewport.transparent_bg is set to false.

Note: Transparency support is implemented on Linux (X11/Wayland), macOS, Windows, and embedded windows.

#### FLAG_NO_FOCUS {#FLAG_NO_FOCUS}

```gdscript
FLAG_NO_FOCUS = 4
```

The window can't be focused. No-focus window will ignore all input, except mouse clicks. Set with unfocusable.

Note: This flag is implemented on Linux (X11), macOS, Windows, and embedded windows.

#### FLAG_POPUP {#FLAG_POPUP}

```gdscript
FLAG_POPUP = 5
```

Window is part of menu or OptionButton dropdown. This flag can't be changed when the window is visible. An active popup window will exclusively receive all input, without stealing focus from its parent. Popup windows are automatically closed when uses click outside it, or when an application is switched. Popup window must have transient parent set (see transient).

Note: This flag is implemented on Linux (X11/Wayland), macOS, Windows, and embedded Popup windows.

#### FLAG_EXTEND_TO_TITLE {#FLAG_EXTEND_TO_TITLE}

```gdscript
FLAG_EXTEND_TO_TITLE = 6
```

Window content is expanded to the full size of the window. Unlike borderless window, the frame is left intact and can be used to resize the window, title bar is transparent, but have minimize/maximize/close buttons. Set with extend_to_title.

Note: This flag has no effect in embedded windows.

Note: This flag is implemented only on macOS.

#### FLAG_MOUSE_PASSTHROUGH {#FLAG_MOUSE_PASSTHROUGH}

```gdscript
FLAG_MOUSE_PASSTHROUGH = 7
```

All mouse events are passed to the underlying window of the same application.

Note: This flag has no effect in embedded windows.

Note: This flag is implemented on Linux (X11), macOS, Windows.

#### FLAG_SHARP_CORNERS {#FLAG_SHARP_CORNERS}

```gdscript
FLAG_SHARP_CORNERS = 8
```

Window style is overridden, forcing sharp corners.

Note: This flag has no effect in embedded windows.

Note: This flag is implemented only on Windows (11).

#### FLAG_EXCLUDE_FROM_CAPTURE {#FLAG_EXCLUDE_FROM_CAPTURE}

```gdscript
FLAG_EXCLUDE_FROM_CAPTURE = 9
```

Windows is excluded from screenshots taken by DisplayServer.screen_get_image(), DisplayServer.screen_get_image_rect(), and DisplayServer.screen_get_pixel().

Note: This flag has no effect in embedded windows.

Note: This flag is implemented on macOS and Windows (10, 20H1).

Note: Setting this flag will prevent standard screenshot methods from capturing a window image, but does NOT guarantee that other apps won't be able to capture an image. It should not be used as a DRM or security measure.

#### FLAG_POPUP_WM_HINT {#FLAG_POPUP_WM_HINT}

```gdscript
FLAG_POPUP_WM_HINT = 10
```

Signals the window manager that this window is supposed to be an implementation-defined "popup" (usually a floating, borderless, untileable and immovable child window).

Note: This flag has no effect in embedded windows.

Note: This flag is implemented on Linux (Wayland).

#### FLAG_MINIMIZE_DISABLED {#FLAG_MINIMIZE_DISABLED}

```gdscript
FLAG_MINIMIZE_DISABLED = 11
```

Window minimize button is disabled.

Note: This flag has no effect in embedded windows.

Note: This flag is implemented on Linux (X11), macOS, and Windows.

#### FLAG_MAXIMIZE_DISABLED {#FLAG_MAXIMIZE_DISABLED}

```gdscript
FLAG_MAXIMIZE_DISABLED = 12
```

Window maximize button is disabled.

Note: This flag has no effect in embedded windows.

Note: This flag is implemented on Linux (X11), macOS, and Windows.

#### FLAG_MAX {#FLAG_MAX}

```gdscript
FLAG_MAX = 13
```

Max value of the Flags.

---

### ContentScaleMode {#enum-ContentScaleMode}

```gdscript
enum ContentScaleMode { CONTENT_SCALE_MODE_DISABLED = 0, CONTENT_SCALE_MODE_CANVAS_ITEMS = 1, CONTENT_SCALE_MODE_VIEWPORT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONTENT_SCALE_MODE_DISABLED {#CONTENT_SCALE_MODE_DISABLED}

```gdscript
CONTENT_SCALE_MODE_DISABLED = 0
```

The content will not be scaled to match the Window's size (content_scale_size is ignored).

#### CONTENT_SCALE_MODE_CANVAS_ITEMS {#CONTENT_SCALE_MODE_CANVAS_ITEMS}

```gdscript
CONTENT_SCALE_MODE_CANVAS_ITEMS = 1
```

The content will be rendered at the target size. This is more performance-expensive than CONTENT_SCALE_MODE_VIEWPORT, but provides better results.

#### CONTENT_SCALE_MODE_VIEWPORT {#CONTENT_SCALE_MODE_VIEWPORT}

```gdscript
CONTENT_SCALE_MODE_VIEWPORT = 2
```

The content will be rendered at the base size and then scaled to the target size. More performant than CONTENT_SCALE_MODE_CANVAS_ITEMS, but results in pixelated image.

---

### ContentScaleAspect {#enum-ContentScaleAspect}

```gdscript
enum ContentScaleAspect { CONTENT_SCALE_ASPECT_IGNORE = 0, CONTENT_SCALE_ASPECT_KEEP = 1, CONTENT_SCALE_ASPECT_KEEP_WIDTH = 2, CONTENT_SCALE_ASPECT_KEEP_HEIGHT = 3, CONTENT_SCALE_ASPECT_EXPAND = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONTENT_SCALE_ASPECT_IGNORE {#CONTENT_SCALE_ASPECT_IGNORE}

```gdscript
CONTENT_SCALE_ASPECT_IGNORE = 0
```

The aspect will be ignored. Scaling will simply stretch the content to fit the target size.

#### CONTENT_SCALE_ASPECT_KEEP {#CONTENT_SCALE_ASPECT_KEEP}

```gdscript
CONTENT_SCALE_ASPECT_KEEP = 1
```

The content's aspect will be preserved. If the target size has different aspect from the base one, the image will be centered and black bars will appear on left and right sides.

#### CONTENT_SCALE_ASPECT_KEEP_WIDTH {#CONTENT_SCALE_ASPECT_KEEP_WIDTH}

```gdscript
CONTENT_SCALE_ASPECT_KEEP_WIDTH = 2
```

The content can be expanded vertically. Scaling horizontally will result in keeping the width ratio and then black bars on left and right sides.

#### CONTENT_SCALE_ASPECT_KEEP_HEIGHT {#CONTENT_SCALE_ASPECT_KEEP_HEIGHT}

```gdscript
CONTENT_SCALE_ASPECT_KEEP_HEIGHT = 3
```

The content can be expanded horizontally. Scaling vertically will result in keeping the height ratio and then black bars on top and bottom sides.

#### CONTENT_SCALE_ASPECT_EXPAND {#CONTENT_SCALE_ASPECT_EXPAND}

```gdscript
CONTENT_SCALE_ASPECT_EXPAND = 4
```

The content's aspect will be preserved. If the target size has different aspect from the base one, the content will stay in the top-left corner and add an extra visible area in the stretched space.

---

### ContentScaleStretch {#enum-ContentScaleStretch}

```gdscript
enum ContentScaleStretch { CONTENT_SCALE_STRETCH_FRACTIONAL = 0, CONTENT_SCALE_STRETCH_INTEGER = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONTENT_SCALE_STRETCH_FRACTIONAL {#CONTENT_SCALE_STRETCH_FRACTIONAL}

```gdscript
CONTENT_SCALE_STRETCH_FRACTIONAL = 0
```

The content will be stretched according to a fractional factor. This fills all the space available in the window, but allows "pixel wobble" to occur due to uneven pixel scaling.

#### CONTENT_SCALE_STRETCH_INTEGER {#CONTENT_SCALE_STRETCH_INTEGER}

```gdscript
CONTENT_SCALE_STRETCH_INTEGER = 1
```

The content will be stretched only according to an integer factor, preserving sharp pixels. This may leave a black background visible on the window's edges depending on the window size.

---

### LayoutDirection {#enum-LayoutDirection}

```gdscript
enum LayoutDirection { LAYOUT_DIRECTION_INHERITED = 0, LAYOUT_DIRECTION_APPLICATION_LOCALE = 1, LAYOUT_DIRECTION_LTR = 2, LAYOUT_DIRECTION_RTL = 3, LAYOUT_DIRECTION_SYSTEM_LOCALE = 4, LAYOUT_DIRECTION_MAX = 5, LAYOUT_DIRECTION_LOCALE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LAYOUT_DIRECTION_INHERITED {#LAYOUT_DIRECTION_INHERITED}

```gdscript
LAYOUT_DIRECTION_INHERITED = 0
```

Automatic layout direction, determined from the parent window layout direction.

#### LAYOUT_DIRECTION_APPLICATION_LOCALE {#LAYOUT_DIRECTION_APPLICATION_LOCALE}

```gdscript
LAYOUT_DIRECTION_APPLICATION_LOCALE = 1
```

Automatic layout direction, determined from the current locale.

#### LAYOUT_DIRECTION_LTR {#LAYOUT_DIRECTION_LTR}

```gdscript
LAYOUT_DIRECTION_LTR = 2
```

Left-to-right layout direction.

#### LAYOUT_DIRECTION_RTL {#LAYOUT_DIRECTION_RTL}

```gdscript
LAYOUT_DIRECTION_RTL = 3
```

Right-to-left layout direction.

#### LAYOUT_DIRECTION_SYSTEM_LOCALE {#LAYOUT_DIRECTION_SYSTEM_LOCALE}

```gdscript
LAYOUT_DIRECTION_SYSTEM_LOCALE = 4
```

Automatic layout direction, determined from the system locale.

#### LAYOUT_DIRECTION_MAX {#LAYOUT_DIRECTION_MAX}

```gdscript
LAYOUT_DIRECTION_MAX = 5
```

Represents the size of the LayoutDirection enum.

#### LAYOUT_DIRECTION_LOCALE {#LAYOUT_DIRECTION_LOCALE}

```gdscript
LAYOUT_DIRECTION_LOCALE = 1
```

Deprecated: Use LAYOUT_DIRECTION_APPLICATION_LOCALE instead.

---

### WindowInitialPosition {#enum-WindowInitialPosition}

```gdscript
enum WindowInitialPosition { WINDOW_INITIAL_POSITION_ABSOLUTE = 0, WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN = 1, WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN = 2, WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN = 3, WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS = 4, WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### WINDOW_INITIAL_POSITION_ABSOLUTE {#WINDOW_INITIAL_POSITION_ABSOLUTE}

```gdscript
WINDOW_INITIAL_POSITION_ABSOLUTE = 0
```

Initial window position is determined by position.

#### WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN {#WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN}

```gdscript
WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN = 1
```

Initial window position is the center of the primary screen.

#### WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN {#WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN}

```gdscript
WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN = 2
```

Initial window position is the center of the main window screen.

#### WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN {#WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN}

```gdscript
WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN = 3
```

Initial window position is the center of current_screen screen.

#### WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS {#WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS}

```gdscript
WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS = 4
```

Initial window position is the center of the screen containing the mouse pointer.

#### WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS {#WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS}

```gdscript
WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS = 5
```

Initial window position is the center of the screen containing the window with the keyboard focus.

---

## 常量

### NOTIFICATION_VISIBILITY_CHANGED {#NOTIFICATION_VISIBILITY_CHANGED}

```gdscript
const NOTIFICATION_VISIBILITY_CHANGED = 30
```

Emitted when Window's visibility changes, right before visibility_changed.

NOTIFICATION_THEME_CHANGED = 32 

Sent when the node needs to refresh its theme items. This happens in one of the following cases:

- The theme property is changed on this node or any of its ancestors.
- The theme_type_variation property is changed on this node.
- The node enters the scene tree.

Note: As an optimization, this notification won't be sent from changes that occur while this node is outside of the scene tree. Instead, all of the theme item updates can be applied at once when the node enters the scene tree.

### NOTIFICATION_THEME_CHANGED {#NOTIFICATION_THEME_CHANGED}

```gdscript
const NOTIFICATION_THEME_CHANGED = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
