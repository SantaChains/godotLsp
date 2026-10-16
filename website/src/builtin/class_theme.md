# Theme

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Theme**

> A resource used for styling/skinning Control and Window nodes. While individual controls can be styled using their local theme overrides (see Control.add_theme_color_override()), theme resources allow you to store and apply the same settings across all controls sharing the same type (e.g. style all Buttons the same). One theme resource can be used for the entire project, but you can also set a separate theme resource to a branch of control nodes. A theme resource assigned to a control applies to the control itself, as well as all of its direct and indirect children (as long as a chain of controls is uninterrupted).

<details><summary>官方英文描述（中文精译进行中）</summary>

A resource used for styling/skinning Control and Window nodes. While individual controls can be styled using their local theme overrides (see Control.add_theme_color_override()), theme resources allow you to store and apply the same settings across all controls sharing the same type (e.g. style all Buttons the same). One theme resource can be used for the entire project, but you can also set a separate theme resource to a branch of control nodes. A theme resource assigned to a control applies to the control itself, as well as all of its direct and indirect children (as long as a chain of controls is uninterrupted).

Use ProjectSettings.gui/theme/custom to set up a project-scope theme that will be available to every control in your project.

Use Control.theme of any control node to set up a theme that will be available to that control and all of its direct and indirect children.

</details>

## 方法

### add_type {#add_type}

```gdscript
void add_type(theme_type: StringName)
```

Adds an empty theme type for every valid data type.

Note: Empty types are not saved with the theme. This method only exists to perform in-memory changes to the resource. Use available set_* methods to add theme items.

### clear {#clear}

```gdscript
void clear()
```

Removes all the theme properties defined on the theme resource.

### clear_color {#clear_color}

```gdscript
void clear_color(name: StringName, theme_type: StringName)
```

Removes the Color property defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_color() to check for existence.

### clear_constant {#clear_constant}

```gdscript
void clear_constant(name: StringName, theme_type: StringName)
```

Removes the constant property defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_constant() to check for existence.

### clear_font {#clear_font}

```gdscript
void clear_font(name: StringName, theme_type: StringName)
```

Removes the Font property defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_font() to check for existence.

### clear_font_size {#clear_font_size}

```gdscript
void clear_font_size(name: StringName, theme_type: StringName)
```

Removes the font size property defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_font_size() to check for existence.

### clear_icon {#clear_icon}

```gdscript
void clear_icon(name: StringName, theme_type: StringName)
```

Removes the icon property defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_icon() to check for existence.

### clear_stylebox {#clear_stylebox}

```gdscript
void clear_stylebox(name: StringName, theme_type: StringName)
```

Removes the StyleBox property defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_stylebox() to check for existence.

### clear_theme_item {#clear_theme_item}

```gdscript
void clear_theme_item(data_type: Theme.DataType, name: StringName, theme_type: StringName)
```

Removes the theme property of data_type defined by name and theme_type, if it exists.

Fails if it doesn't exist. Use has_theme_item() to check for existence.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### clear_type_variation {#clear_type_variation}

```gdscript
void clear_type_variation(theme_type: StringName)
```

Unmarks theme_type as being a variation of another theme type. See set_type_variation().

### get_color {#get_color}

```gdscript
Color get_color(name: StringName, theme_type: StringName) const
```

Returns the Color property defined by name and theme_type, if it exists.

Returns the default color value if the property doesn't exist. Use has_color() to check for existence.

### get_color_list {#get_color_list}

```gdscript
PackedStringArray get_color_list(theme_type: String) const
```

Returns a list of names for Color properties defined with theme_type. Use get_color_type_list() to get a list of possible theme type names.

### get_color_type_list {#get_color_type_list}

```gdscript
PackedStringArray get_color_type_list() const
```

Returns a list of all unique theme type names for Color properties. Use get_type_list() to get a list of all unique theme types.

### get_constant {#get_constant}

```gdscript
int get_constant(name: StringName, theme_type: StringName) const
```

Returns the constant property defined by name and theme_type, if it exists.

Returns 0 if the property doesn't exist. Use has_constant() to check for existence.

### get_constant_list {#get_constant_list}

```gdscript
PackedStringArray get_constant_list(theme_type: String) const
```

Returns a list of names for constant properties defined with theme_type. Use get_constant_type_list() to get a list of possible theme type names.

### get_constant_type_list {#get_constant_type_list}

```gdscript
PackedStringArray get_constant_type_list() const
```

Returns a list of all unique theme type names for constant properties. Use get_type_list() to get a list of all unique theme types.

### get_default_base_scale {#get_default_base_scale}

```gdscript
float get_default_base_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_font {#get_default_font}

```gdscript
Font get_default_font() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_font_size {#get_default_font_size}

```gdscript
int get_default_font_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_font {#get_font}

```gdscript
Font get_font(name: StringName, theme_type: StringName) const
```

Returns the Font property defined by name and theme_type, if it exists.

Returns the default theme font if the property doesn't exist and the default theme font is set up (see default_font). Use has_font() to check for existence of the property and has_default_font() to check for existence of the default theme font.

Returns the engine fallback font value, if neither exist (see ThemeDB.fallback_font).

### get_font_list {#get_font_list}

```gdscript
PackedStringArray get_font_list(theme_type: String) const
```

Returns a list of names for Font properties defined with theme_type. Use get_font_type_list() to get a list of possible theme type names.

### get_font_size {#get_font_size}

```gdscript
int get_font_size(name: StringName, theme_type: StringName) const
```

Returns the font size property defined by name and theme_type, if it exists.

Returns the default theme font size if the property doesn't exist and the default theme font size is set up (see default_font_size). Use has_font_size() to check for existence of the property and has_default_font_size() to check for existence of the default theme font.

Returns the engine fallback font size value, if neither exist (see ThemeDB.fallback_font_size).

### get_font_size_list {#get_font_size_list}

```gdscript
PackedStringArray get_font_size_list(theme_type: String) const
```

Returns a list of names for font size properties defined with theme_type. Use get_font_size_type_list() to get a list of possible theme type names.

### get_font_size_type_list {#get_font_size_type_list}

```gdscript
PackedStringArray get_font_size_type_list() const
```

Returns a list of all unique theme type names for font size properties. Use get_type_list() to get a list of all unique theme types.

### get_font_type_list {#get_font_type_list}

```gdscript
PackedStringArray get_font_type_list() const
```

Returns a list of all unique theme type names for Font properties. Use get_type_list() to get a list of all unique theme types.

### get_icon {#get_icon}

```gdscript
Texture2D get_icon(name: StringName, theme_type: StringName) const
```

Returns the icon property defined by name and theme_type, if it exists.

Returns the engine fallback icon value if the property doesn't exist (see ThemeDB.fallback_icon). Use has_icon() to check for existence.

### get_icon_list {#get_icon_list}

```gdscript
PackedStringArray get_icon_list(theme_type: String) const
```

Returns a list of names for icon properties defined with theme_type. Use get_icon_type_list() to get a list of possible theme type names.

### get_icon_type_list {#get_icon_type_list}

```gdscript
PackedStringArray get_icon_type_list() const
```

Returns a list of all unique theme type names for icon properties. Use get_type_list() to get a list of all unique theme types.

### get_stylebox {#get_stylebox}

```gdscript
StyleBox get_stylebox(name: StringName, theme_type: StringName) const
```

Returns the StyleBox property defined by name and theme_type, if it exists.

Returns the engine fallback stylebox value if the property doesn't exist (see ThemeDB.fallback_stylebox). Use has_stylebox() to check for existence.

### get_stylebox_list {#get_stylebox_list}

```gdscript
PackedStringArray get_stylebox_list(theme_type: String) const
```

Returns a list of names for StyleBox properties defined with theme_type. Use get_stylebox_type_list() to get a list of possible theme type names.

### get_stylebox_type_list {#get_stylebox_type_list}

```gdscript
PackedStringArray get_stylebox_type_list() const
```

Returns a list of all unique theme type names for StyleBox properties. Use get_type_list() to get a list of all unique theme types.

### get_theme_item {#get_theme_item}

```gdscript
Variant get_theme_item(data_type: Theme.DataType, name: StringName, theme_type: StringName) const
```

Returns the theme property of data_type defined by name and theme_type, if it exists.

Returns the engine fallback value if the property doesn't exist (see ThemeDB). Use has_theme_item() to check for existence.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### get_theme_item_list {#get_theme_item_list}

```gdscript
PackedStringArray get_theme_item_list(data_type: Theme.DataType, theme_type: String) const
```

Returns a list of names for properties of data_type defined with theme_type. Use get_theme_item_type_list() to get a list of possible theme type names.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### get_theme_item_type_list {#get_theme_item_type_list}

```gdscript
PackedStringArray get_theme_item_type_list(data_type: Theme.DataType) const
```

Returns a list of all unique theme type names for data_type properties. Use get_type_list() to get a list of all unique theme types.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### get_type_list {#get_type_list}

```gdscript
PackedStringArray get_type_list() const
```

Returns a list of all unique theme type names. Use the appropriate get_*_type_list method to get a list of unique theme types for a single data type.

### get_type_variation_base {#get_type_variation_base}

```gdscript
StringName get_type_variation_base(theme_type: StringName) const
```

Returns the name of the base theme type if theme_type is a valid variation type. Returns an empty string otherwise.

### get_type_variation_list {#get_type_variation_list}

```gdscript
PackedStringArray get_type_variation_list(base_type: StringName) const
```

Returns a list of all type variations for the given base_type.

### has_color {#has_color}

```gdscript
bool has_color(name: StringName, theme_type: StringName) const
```

Returns true if the Color property defined by name and theme_type exists.

Returns false if it doesn't exist. Use set_color() to define it.

### has_constant {#has_constant}

```gdscript
bool has_constant(name: StringName, theme_type: StringName) const
```

Returns true if the constant property defined by name and theme_type exists.

Returns false if it doesn't exist. Use set_constant() to define it.

### has_default_base_scale {#has_default_base_scale}

```gdscript
bool has_default_base_scale() const
```

Returns true if default_base_scale has a valid value.

Returns false if it doesn't. The value must be greater than 0.0 to be considered valid.

### has_default_font {#has_default_font}

```gdscript
bool has_default_font() const
```

Returns true if default_font has a valid value.

Returns false if it doesn't.

### has_default_font_size {#has_default_font_size}

```gdscript
bool has_default_font_size() const
```

Returns true if default_font_size has a valid value.

Returns false if it doesn't. The value must be greater than 0 to be considered valid.

### has_font {#has_font}

```gdscript
bool has_font(name: StringName, theme_type: StringName) const
```

Returns true if the Font property defined by name and theme_type exists, or if the default theme font is set up (see has_default_font()).

Returns false if neither exist. Use set_font() to define the property.

### has_font_size {#has_font_size}

```gdscript
bool has_font_size(name: StringName, theme_type: StringName) const
```

Returns true if the font size property defined by name and theme_type exists, or if the default theme font size is set up (see has_default_font_size()).

Returns false if neither exist. Use set_font_size() to define the property.

### has_icon {#has_icon}

```gdscript
bool has_icon(name: StringName, theme_type: StringName) const
```

Returns true if the icon property defined by name and theme_type exists.

Returns false if it doesn't exist. Use set_icon() to define it.

### has_stylebox {#has_stylebox}

```gdscript
bool has_stylebox(name: StringName, theme_type: StringName) const
```

Returns true if the StyleBox property defined by name and theme_type exists.

Returns false if it doesn't exist. Use set_stylebox() to define it.

### has_theme_item {#has_theme_item}

```gdscript
bool has_theme_item(data_type: Theme.DataType, name: StringName, theme_type: StringName) const
```

Returns true if the theme property of data_type defined by name and theme_type exists.

Returns false if it doesn't exist. Use set_theme_item() to define it.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### is_type_variation {#is_type_variation}

```gdscript
bool is_type_variation(theme_type: StringName, base_type: StringName) const
```

Returns true if theme_type is marked as a variation of base_type.

### merge_with {#merge_with}

```gdscript
void merge_with(other: Theme)
```

Adds missing and overrides existing definitions with values from the other theme resource.

Note: This modifies the current theme. If you want to merge two themes together without modifying either one, create a new empty theme and merge the other two into it one after another.

### remove_type {#remove_type}

```gdscript
void remove_type(theme_type: StringName)
```

Removes the theme type, gracefully discarding defined theme items. If the type is a variation, this information is also erased. If the type is a base for type variations, those variations lose their base.

### rename_color {#rename_color}

```gdscript
void rename_color(old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the Color property defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_color() to check for existence, and clear_color() to remove the existing property.

### rename_constant {#rename_constant}

```gdscript
void rename_constant(old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the constant property defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_constant() to check for existence, and clear_constant() to remove the existing property.

### rename_font {#rename_font}

```gdscript
void rename_font(old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the Font property defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_font() to check for existence, and clear_font() to remove the existing property.

### rename_font_size {#rename_font_size}

```gdscript
void rename_font_size(old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the font size property defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_font_size() to check for existence, and clear_font_size() to remove the existing property.

### rename_icon {#rename_icon}

```gdscript
void rename_icon(old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the icon property defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_icon() to check for existence, and clear_icon() to remove the existing property.

### rename_stylebox {#rename_stylebox}

```gdscript
void rename_stylebox(old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the StyleBox property defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_stylebox() to check for existence, and clear_stylebox() to remove the existing property.

### rename_theme_item {#rename_theme_item}

```gdscript
void rename_theme_item(data_type: Theme.DataType, old_name: StringName, name: StringName, theme_type: StringName)
```

Renames the theme property of data_type defined by old_name and theme_type to name, if it exists.

Fails if it doesn't exist, or if a similar property with the new name already exists. Use has_theme_item() to check for existence, and clear_theme_item() to remove the existing property.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### rename_type {#rename_type}

```gdscript
void rename_type(old_theme_type: StringName, theme_type: StringName)
```

Renames the theme type old_theme_type to theme_type, if the old type exists and the new one doesn't exist.

Note: Renaming a theme type to an empty name or a variation to a type associated with a built-in class removes type variation connections in a way that cannot be undone by reversing the rename alone.

### set_color {#set_color}

```gdscript
void set_color(name: StringName, theme_type: StringName, color: Color)
```

Creates or changes the value of the Color property defined by name and theme_type. Use clear_color() to remove the property.

### set_constant {#set_constant}

```gdscript
void set_constant(name: StringName, theme_type: StringName, constant: int)
```

Creates or changes the value of the constant property defined by name and theme_type. Use clear_constant() to remove the property.

### set_default_base_scale {#set_default_base_scale}

```gdscript
void set_default_base_scale(base_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_font {#set_default_font}

```gdscript
void set_default_font(font: Font)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_font_size {#set_default_font_size}

```gdscript
void set_default_font_size(font_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font {#set_font}

```gdscript
void set_font(name: StringName, theme_type: StringName, font: Font)
```

Creates or changes the value of the Font property defined by name and theme_type. Use clear_font() to remove the property.

### set_font_size {#set_font_size}

```gdscript
void set_font_size(name: StringName, theme_type: StringName, font_size: int)
```

Creates or changes the value of the font size property defined by name and theme_type. Use clear_font_size() to remove the property.

### set_icon {#set_icon}

```gdscript
void set_icon(name: StringName, theme_type: StringName, texture: Texture2D)
```

Creates or changes the value of the icon property defined by name and theme_type. Use clear_icon() to remove the property.

### set_stylebox {#set_stylebox}

```gdscript
void set_stylebox(name: StringName, theme_type: StringName, texture: StyleBox)
```

Creates or changes the value of the StyleBox property defined by name and theme_type. Use clear_stylebox() to remove the property.

### set_theme_item {#set_theme_item}

```gdscript
void set_theme_item(data_type: Theme.DataType, name: StringName, theme_type: StringName, value: Variant)
```

Creates or changes the value of the theme property of data_type defined by name and theme_type. Use clear_theme_item() to remove the property.

Fails if the value type is not accepted by data_type.

Note: This method is analogous to calling the corresponding data type specific method, but can be used for more generalized logic.

### set_type_variation {#set_type_variation}

```gdscript
void set_type_variation(theme_type: StringName, base_type: StringName)
```

Marks theme_type as a variation of base_type.

This adds theme_type as a suggested option for Control.theme_type_variation on a Control that is of the base_type class.

Variations can also be nested, i.e. base_type can be another variation. If a chain of variations ends with a base_type matching the class of the Control, the whole chain is going to be suggested as options.

Note: Suggestions only show up if this theme resource is set as the project default theme. See ProjectSettings.gui/theme/custom.

## 属性

### default_base_scale {#default_base_scale}

```gdscript
default_base_scale: float
```

- void set_default_base_scale(value: float)
- float get_default_base_scale()

The default base scale factor of this theme resource. Used by some controls to scale their visual properties based on the global scale factor. If this value is set to 0.0, the global scale factor is used (see ThemeDB.fallback_base_scale).

Use has_default_base_scale() to check if this value is valid.

### default_font {#default_font}

```gdscript
default_font: Font
```

- void set_default_font(value: Font)
- Font get_default_font()

The default font of this theme resource. Used as the default value when trying to fetch a font resource that doesn't exist in this theme or is in invalid state. If the default font is also missing or invalid, the engine fallback value is used (see ThemeDB.fallback_font).

Use has_default_font() to check if this value is valid.

### default_font_size {#default_font_size}

```gdscript
default_font_size: int
```

- void set_default_font_size(value: int)
- int get_default_font_size()

The default font size of this theme resource. Used as the default value when trying to fetch a font size value that doesn't exist in this theme or is in invalid state. If the default font size is also missing or invalid, the engine fallback value is used (see ThemeDB.fallback_font_size).

Values below 1 are invalid and can be used to unset the property. Use has_default_font_size() to check if this value is valid.

## 枚举

### DataType {#enum-DataType}

```gdscript
enum DataType { DATA_TYPE_COLOR = 0, DATA_TYPE_CONSTANT = 1, DATA_TYPE_FONT = 2, DATA_TYPE_FONT_SIZE = 3, DATA_TYPE_ICON = 4, DATA_TYPE_STYLEBOX = 5, DATA_TYPE_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DATA_TYPE_COLOR {#DATA_TYPE_COLOR}

```gdscript
DATA_TYPE_COLOR = 0
```

Theme's Color item type.

#### DATA_TYPE_CONSTANT {#DATA_TYPE_CONSTANT}

```gdscript
DATA_TYPE_CONSTANT = 1
```

Theme's constant item type.

#### DATA_TYPE_FONT {#DATA_TYPE_FONT}

```gdscript
DATA_TYPE_FONT = 2
```

Theme's Font item type.

#### DATA_TYPE_FONT_SIZE {#DATA_TYPE_FONT_SIZE}

```gdscript
DATA_TYPE_FONT_SIZE = 3
```

Theme's font size item type.

#### DATA_TYPE_ICON {#DATA_TYPE_ICON}

```gdscript
DATA_TYPE_ICON = 4
```

Theme's icon Texture2D item type.

#### DATA_TYPE_STYLEBOX {#DATA_TYPE_STYLEBOX}

```gdscript
DATA_TYPE_STYLEBOX = 5
```

Theme's StyleBox item type.

#### DATA_TYPE_MAX {#DATA_TYPE_MAX}

```gdscript
DATA_TYPE_MAX = 6
```

Maximum value for the DataType enum.

---
