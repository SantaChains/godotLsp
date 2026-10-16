# CodeHighlighter

**继承**：[SyntaxHighlighter](class_syntaxhighlighter.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CodeHighlighter**

> By adjusting various properties of this resource, you can change the colors of strings, comments, numbers, and other text patterns inside a TextEdit control.

<details><summary>官方英文描述（中文精译进行中）</summary>

By adjusting various properties of this resource, you can change the colors of strings, comments, numbers, and other text patterns inside a TextEdit control.

</details>

## 方法

### add_color_region {#add_color_region}

```gdscript
void add_color_region(start_key: String, end_key: String, color: Color, line_only: bool = false)
```

Adds a color region (such as for comments or strings) from start_key to end_key. Both keys should be symbols, and start_key must not be shared with other delimiters.

If line_only is true or end_key is an empty String, the region does not carry over to the next line.

### add_keyword_color {#add_keyword_color}

```gdscript
void add_keyword_color(keyword: String, color: Color)
```

Sets the color for a keyword.

The keyword cannot contain any symbols except '_'.

### add_member_keyword_color {#add_member_keyword_color}

```gdscript
void add_member_keyword_color(member_keyword: String, color: Color)
```

Sets the color for a member keyword.

The member keyword cannot contain any symbols except '_'.

It will not be highlighted if preceded by a '.'.

### clear_color_regions {#clear_color_regions}

```gdscript
void clear_color_regions()
```

Removes all color regions.

### clear_keyword_colors {#clear_keyword_colors}

```gdscript
void clear_keyword_colors()
```

Removes all keywords.

### clear_member_keyword_colors {#clear_member_keyword_colors}

```gdscript
void clear_member_keyword_colors()
```

Removes all member keywords.

### get_color_regions {#get_color_regions}

```gdscript
Dictionary get_color_regions() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_function_color {#get_function_color}

```gdscript
Color get_function_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_keyword_color {#get_keyword_color}

```gdscript
Color get_keyword_color(keyword: String) const
```

Returns the color for a keyword.

### get_keyword_colors {#get_keyword_colors}

```gdscript
Dictionary get_keyword_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_member_keyword_color {#get_member_keyword_color}

```gdscript
Color get_member_keyword_color(member_keyword: String) const
```

Returns the color for a member keyword.

### get_member_keyword_colors {#get_member_keyword_colors}

```gdscript
Dictionary get_member_keyword_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_member_variable_color {#get_member_variable_color}

```gdscript
Color get_member_variable_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_number_color {#get_number_color}

```gdscript
Color get_number_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_symbol_color {#get_symbol_color}

```gdscript
Color get_symbol_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_color_region {#has_color_region}

```gdscript
bool has_color_region(start_key: String) const
```

Returns true if the start key exists, else false.

### has_keyword_color {#has_keyword_color}

```gdscript
bool has_keyword_color(keyword: String) const
```

Returns true if the keyword exists, else false.

### has_member_keyword_color {#has_member_keyword_color}

```gdscript
bool has_member_keyword_color(member_keyword: String) const
```

Returns true if the member keyword exists, else false.

### remove_color_region {#remove_color_region}

```gdscript
void remove_color_region(start_key: String)
```

Removes the color region that uses that start key.

### remove_keyword_color {#remove_keyword_color}

```gdscript
void remove_keyword_color(keyword: String)
```

Removes the keyword.

### remove_member_keyword_color {#remove_member_keyword_color}

```gdscript
void remove_member_keyword_color(member_keyword: String)
```

Removes the member keyword.

### set_color_regions {#set_color_regions}

```gdscript
void set_color_regions(color_regions: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function_color {#set_function_color}

```gdscript
void set_function_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keyword_colors {#set_keyword_colors}

```gdscript
void set_keyword_colors(keywords: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_member_keyword_colors {#set_member_keyword_colors}

```gdscript
void set_member_keyword_colors(member_keyword: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_member_variable_color {#set_member_variable_color}

```gdscript
void set_member_variable_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_number_color {#set_number_color}

```gdscript
void set_number_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_symbol_color {#set_symbol_color}

```gdscript
void set_symbol_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### number_color {#number_color}

```gdscript
number_color: Color
```

- void set_number_color(value: Color)
- Color get_number_color()

Sets the color for numbers.

### symbol_color {#symbol_color}

```gdscript
symbol_color: Color
```

- void set_symbol_color(value: Color)
- Color get_symbol_color()

Sets the color for symbols.

### function_color {#function_color}

```gdscript
function_color: Color
```

- void set_function_color(value: Color)
- Color get_function_color()

Sets color for functions. A function is a non-keyword string followed by a '('.

### member_variable_color {#member_variable_color}

```gdscript
member_variable_color: Color
```

- void set_member_variable_color(value: Color)
- Color get_member_variable_color()

Sets color for member variables. A member variable is non-keyword, non-function string proceeded with a '.'.

### keyword_colors {#keyword_colors}

```gdscript
keyword_colors: Dictionary
```

- void set_keyword_colors(value: Dictionary)
- Dictionary get_keyword_colors()

Sets the keyword colors. All existing keywords will be removed. The Dictionary key is the keyword. The value is the keyword color.

### member_keyword_colors {#member_keyword_colors}

```gdscript
member_keyword_colors: Dictionary
```

- void set_member_keyword_colors(value: Dictionary)
- Dictionary get_member_keyword_colors()

Sets the member keyword colors. All existing member keyword will be removed. The Dictionary key is the member keyword. The value is the member keyword color.

### color_regions {#color_regions}

```gdscript
color_regions: Dictionary
```

- void set_color_regions(value: Dictionary)
- Dictionary get_color_regions()

Sets the color regions. All existing regions will be removed. The Dictionary key is the region start and end key, separated by a space. The value is the region color.
