# CodeEdit

**继承**：[TextEdit](class_textedit.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **CodeEdit**

> CodeEdit is a specialized TextEdit designed for editing plain text code files. It has many features commonly found in code editors such as line numbers, line folding, code completion, indent management, and string/comment management.

<details><summary>官方英文描述（中文精译进行中）</summary>

CodeEdit is a specialized TextEdit designed for editing plain text code files. It has many features commonly found in code editors such as line numbers, line folding, code completion, indent management, and string/comment management.

Note: Regardless of locale, CodeEdit will by default always use left-to-right text direction to correctly display source code.

</details>

## 方法

### _confirm_code_completion {#_confirm_code_completion}

```gdscript
void _confirm_code_completion(replace: bool) virtual
```

Override this method to define how the selected entry should be inserted. If replace is true, any existing text should be replaced.

### _filter_code_completion_candidates {#_filter_code_completion_candidates}

```gdscript
typedarray::Dictionary _filter_code_completion_candidates(candidates: typedarray::Dictionary) virtual const
```

Override this method to define what items in candidates should be displayed.

Both candidates and the return is an Array of Dictionary, see get_code_completion_option() for Dictionary content.

### _request_code_completion {#_request_code_completion}

```gdscript
void _request_code_completion(force: bool) virtual
```

Override this method to define what happens when the user requests code completion. If force is true, any checks should be bypassed.

### add_auto_brace_completion_pair {#add_auto_brace_completion_pair}

```gdscript
void add_auto_brace_completion_pair(start_key: String, end_key: String)
```

Adds a brace pair.

Both the start and end keys must be symbols. Only the start key has to be unique.

### add_code_completion_option {#add_code_completion_option}

```gdscript
void add_code_completion_option(type: CodeEdit.CodeCompletionKind, display_text: String, insert_text: String, text_color: Color = Color(1, 1, 1, 1), icon: Resource = null, value: Variant = null, location: int = 1024)
```

Submits an item to the queue of potential candidates for the autocomplete menu. Call update_code_completion_options() to update the list.

location indicates location of the option relative to the location of the code completion query. See CodeCompletionLocation for how to set this value.

Note: This list will replace all current candidates.

### add_comment_delimiter {#add_comment_delimiter}

```gdscript
void add_comment_delimiter(start_key: String, end_key: String, line_only: bool = false)
```

Adds a comment delimiter from start_key to end_key. Both keys should be symbols, and start_key must not be shared with other delimiters.

If line_only is true or end_key is an empty String, the region does not carry over to the next line.

### add_string_delimiter {#add_string_delimiter}

```gdscript
void add_string_delimiter(start_key: String, end_key: String, line_only: bool = false)
```

Defines a string delimiter from start_key to end_key. Both keys should be symbols, and start_key must not be shared with other delimiters.

If line_only is true or end_key is an empty String, the region does not carry over to the next line.

### can_fold_line {#can_fold_line}

```gdscript
bool can_fold_line(line: int) const
```

Returns true if the given line is foldable. A line is foldable if it is the start of a valid code region (see get_code_region_start_tag()), if it is the start of a comment or string block, or if the next non-empty line is more indented (see TextEdit.get_indent_level()).

### cancel_code_completion {#cancel_code_completion}

```gdscript
void cancel_code_completion()
```

Cancels the autocomplete menu.

### clear_bookmarked_lines {#clear_bookmarked_lines}

```gdscript
void clear_bookmarked_lines()
```

Clears all bookmarked lines.

### clear_breakpointed_lines {#clear_breakpointed_lines}

```gdscript
void clear_breakpointed_lines()
```

Clears all breakpointed lines.

### clear_comment_delimiters {#clear_comment_delimiters}

```gdscript
void clear_comment_delimiters()
```

Removes all comment delimiters.

### clear_executing_lines {#clear_executing_lines}

```gdscript
void clear_executing_lines()
```

Clears all executed lines.

### clear_string_delimiters {#clear_string_delimiters}

```gdscript
void clear_string_delimiters()
```

Removes all string delimiters.

### confirm_code_completion {#confirm_code_completion}

```gdscript
void confirm_code_completion(replace: bool = false)
```

Inserts the selected entry into the text. If replace is true, any existing text is replaced rather than merged.

### convert_indent {#convert_indent}

```gdscript
void convert_indent(from_line: int = -1, to_line: int = -1)
```

Converts the indents of lines between from_line and to_line to tabs or spaces as set by indent_use_spaces.

Values of -1 convert the entire text.

### create_code_region {#create_code_region}

```gdscript
void create_code_region()
```

Creates a new code region with the selection. At least one single line comment delimiter have to be defined (see add_comment_delimiter()).

A code region is a part of code that is highlighted when folded and can help organize your script.

Code region start and end tags can be customized (see set_code_region_tags()).

Code regions are delimited using start and end tags (respectively region and endregion by default) preceded by one line comment delimiter. (eg. #region and #endregion)

### delete_lines {#delete_lines}

```gdscript
void delete_lines()
```

Deletes all lines that are selected or have a caret on them.

### do_indent {#do_indent}

```gdscript
void do_indent()
```

If there is no selection, indentation is inserted at the caret. Otherwise, the selected lines are indented like indent_lines(). Equivalent to the ProjectSettings.input/ui_text_indent action. The indentation characters used depend on indent_use_spaces and indent_size.

### duplicate_lines {#duplicate_lines}

```gdscript
void duplicate_lines()
```

Duplicates all lines currently selected with any caret. Duplicates the entire line beneath the current one no matter where the caret is within the line.

### duplicate_selection {#duplicate_selection}

```gdscript
void duplicate_selection()
```

Duplicates all selected text and duplicates all lines with a caret on them.

### fold_all_lines {#fold_all_lines}

```gdscript
void fold_all_lines()
```

Folds all lines that are possible to be folded (see can_fold_line()).

### fold_line {#fold_line}

```gdscript
void fold_line(line: int)
```

Folds the given line, if possible (see can_fold_line()).

### get_auto_brace_completion_close_key {#get_auto_brace_completion_close_key}

```gdscript
String get_auto_brace_completion_close_key(open_key: String) const
```

Gets the matching auto brace close key for open_key.

### get_auto_brace_completion_pairs {#get_auto_brace_completion_pairs}

```gdscript
Dictionary get_auto_brace_completion_pairs() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_auto_indent_prefixes {#get_auto_indent_prefixes}

```gdscript
typedarray::String get_auto_indent_prefixes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bookmarked_lines {#get_bookmarked_lines}

```gdscript
PackedInt32Array get_bookmarked_lines() const
```

Gets all bookmarked lines.

### get_breakpointed_lines {#get_breakpointed_lines}

```gdscript
PackedInt32Array get_breakpointed_lines() const
```

Gets all breakpointed lines.

### get_code_completion_option {#get_code_completion_option}

```gdscript
Dictionary get_code_completion_option(index: int) const
```

Gets the completion option at index. The return Dictionary has the following key-values:

kind: CodeCompletionKind

display_text: Text that is shown on the autocomplete menu.

insert_text: Text that is to be inserted when this item is selected.

font_color: Color of the text on the autocomplete menu.

icon: Icon to draw on the autocomplete menu.

default_value: Value of the symbol.

### get_code_completion_options {#get_code_completion_options}

```gdscript
typedarray::Dictionary get_code_completion_options() const
```

Gets all completion options, see get_code_completion_option() for return content.

### get_code_completion_prefixes {#get_code_completion_prefixes}

```gdscript
typedarray::String get_code_completion_prefixes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_code_completion_selected_index {#get_code_completion_selected_index}

```gdscript
int get_code_completion_selected_index() const
```

Gets the index of the current selected completion option.

### get_code_region_end_tag {#get_code_region_end_tag}

```gdscript
String get_code_region_end_tag() const
```

Returns the code region end tag (without comment delimiter).

### get_code_region_start_tag {#get_code_region_start_tag}

```gdscript
String get_code_region_start_tag() const
```

Returns the code region start tag (without comment delimiter).

### get_comment_delimiters {#get_comment_delimiters}

```gdscript
typedarray::String get_comment_delimiters() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_delimiter_end_key {#get_delimiter_end_key}

```gdscript
String get_delimiter_end_key(delimiter_index: int) const
```

Gets the end key for a string or comment region index.

### get_delimiter_end_position {#get_delimiter_end_position}

```gdscript
Vector2 get_delimiter_end_position(line: int, column: int) const
```

If line column is in a string or comment, returns the end position of the region. If not or no end could be found, both Vector2 values will be -1.

### get_delimiter_start_key {#get_delimiter_start_key}

```gdscript
String get_delimiter_start_key(delimiter_index: int) const
```

Gets the start key for a string or comment region index.

### get_delimiter_start_position {#get_delimiter_start_position}

```gdscript
Vector2 get_delimiter_start_position(line: int, column: int) const
```

If line column is in a string or comment, returns the start position of the region. If not or no start could be found, both Vector2 values will be -1.

### get_executing_lines {#get_executing_lines}

```gdscript
PackedInt32Array get_executing_lines() const
```

Gets all executing lines.

### get_folded_lines {#get_folded_lines}

```gdscript
typedarray::int get_folded_lines() const
```

Returns all lines that are currently folded.

### get_indent_size {#get_indent_size}

```gdscript
int get_indent_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_length_guidelines {#get_line_length_guidelines}

```gdscript
typedarray::int get_line_length_guidelines() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_numbers_min_digits {#get_line_numbers_min_digits}

```gdscript
int get_line_numbers_min_digits() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_string_delimiters {#get_string_delimiters}

```gdscript
typedarray::String get_string_delimiters() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_for_code_completion {#get_text_for_code_completion}

```gdscript
String get_text_for_code_completion() const
```

Returns the full text with char 0xFFFF at the caret location.

### get_text_for_symbol_lookup {#get_text_for_symbol_lookup}

```gdscript
String get_text_for_symbol_lookup() const
```

Returns the full text with char 0xFFFF at the cursor location.

### get_text_with_cursor_char {#get_text_with_cursor_char}

```gdscript
String get_text_with_cursor_char(line: int, column: int) const
```

Returns the full text with char 0xFFFF at the specified location.

### has_auto_brace_completion_close_key {#has_auto_brace_completion_close_key}

```gdscript
bool has_auto_brace_completion_close_key(close_key: String) const
```

Returns true if close key close_key exists.

### has_auto_brace_completion_open_key {#has_auto_brace_completion_open_key}

```gdscript
bool has_auto_brace_completion_open_key(open_key: String) const
```

Returns true if open key open_key exists.

### has_comment_delimiter {#has_comment_delimiter}

```gdscript
bool has_comment_delimiter(start_key: String) const
```

Returns true if comment start_key exists.

### has_string_delimiter {#has_string_delimiter}

```gdscript
bool has_string_delimiter(start_key: String) const
```

Returns true if string start_key exists.

### indent_lines {#indent_lines}

```gdscript
void indent_lines()
```

Indents all lines that are selected or have a caret on them. Uses spaces or a tab depending on indent_use_spaces. See unindent_lines().

### is_auto_brace_completion_enabled {#is_auto_brace_completion_enabled}

```gdscript
bool is_auto_brace_completion_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_auto_indent_enabled {#is_auto_indent_enabled}

```gdscript
bool is_auto_indent_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_code_completion_enabled {#is_code_completion_enabled}

```gdscript
bool is_code_completion_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_line_numbers_enabled {#is_draw_line_numbers_enabled}

```gdscript
bool is_draw_line_numbers_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_bookmarks_gutter {#is_drawing_bookmarks_gutter}

```gdscript
bool is_drawing_bookmarks_gutter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_breakpoints_gutter {#is_drawing_breakpoints_gutter}

```gdscript
bool is_drawing_breakpoints_gutter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_executing_lines_gutter {#is_drawing_executing_lines_gutter}

```gdscript
bool is_drawing_executing_lines_gutter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_fold_gutter {#is_drawing_fold_gutter}

```gdscript
bool is_drawing_fold_gutter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_highlight_matching_braces_enabled {#is_highlight_matching_braces_enabled}

```gdscript
bool is_highlight_matching_braces_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_in_comment {#is_in_comment}

```gdscript
int is_in_comment(line: int, column: int = -1) const
```

Returns delimiter index if line column is in a comment. If column is not provided, will return delimiter index if the entire line is a comment. Otherwise -1.

### is_in_string {#is_in_string}

```gdscript
int is_in_string(line: int, column: int = -1) const
```

Returns the delimiter index if line column is in a string. If column is not provided, will return the delimiter index if the entire line is a string. Otherwise -1.

### is_indent_using_spaces {#is_indent_using_spaces}

```gdscript
bool is_indent_using_spaces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_line_bookmarked {#is_line_bookmarked}

```gdscript
bool is_line_bookmarked(line: int) const
```

Returns true if the given line is bookmarked. See set_line_as_bookmarked().

### is_line_breakpointed {#is_line_breakpointed}

```gdscript
bool is_line_breakpointed(line: int) const
```

Returns true if the given line is breakpointed. See set_line_as_breakpoint().

### is_line_code_region_end {#is_line_code_region_end}

```gdscript
bool is_line_code_region_end(line: int) const
```

Returns true if the given line is a code region end. See set_code_region_tags().

### is_line_code_region_start {#is_line_code_region_start}

```gdscript
bool is_line_code_region_start(line: int) const
```

Returns true if the given line is a code region start. See set_code_region_tags().

### is_line_executing {#is_line_executing}

```gdscript
bool is_line_executing(line: int) const
```

Returns true if the given line is marked as executing. See set_line_as_executing().

### is_line_folded {#is_line_folded}

```gdscript
bool is_line_folded(line: int) const
```

Returns true if the given line is folded. See fold_line().

### is_line_folding_enabled {#is_line_folding_enabled}

```gdscript
bool is_line_folding_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_line_numbers_zero_padded {#is_line_numbers_zero_padded}

```gdscript
bool is_line_numbers_zero_padded() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_symbol_lookup_on_click_enabled {#is_symbol_lookup_on_click_enabled}

```gdscript
bool is_symbol_lookup_on_click_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_symbol_tooltip_on_hover_enabled {#is_symbol_tooltip_on_hover_enabled}

```gdscript
bool is_symbol_tooltip_on_hover_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### join_lines {#join_lines}

```gdscript
void join_lines(line_ending: String = " ")
```

Joins all selected lines or lines containing a caret with their next line. Whitespace in between will be removed. If the next line has content, the line_ending will be inserted in between.

### move_lines_down {#move_lines_down}

```gdscript
void move_lines_down()
```

Moves all lines down that are selected or have a caret on them.

### move_lines_up {#move_lines_up}

```gdscript
void move_lines_up()
```

Moves all lines up that are selected or have a caret on them.

### remove_comment_delimiter {#remove_comment_delimiter}

```gdscript
void remove_comment_delimiter(start_key: String)
```

Removes the comment delimiter with start_key.

### remove_string_delimiter {#remove_string_delimiter}

```gdscript
void remove_string_delimiter(start_key: String)
```

Removes the string delimiter with start_key.

### request_code_completion {#request_code_completion}

```gdscript
void request_code_completion(force: bool = false)
```

Emits code_completion_requested, if force is true will bypass all checks. Otherwise will check that the caret is in a word or in front of a prefix. Will ignore the request if all current options are of type file path, node path, or signal.

### set_auto_brace_completion_enabled {#set_auto_brace_completion_enabled}

```gdscript
void set_auto_brace_completion_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_brace_completion_pairs {#set_auto_brace_completion_pairs}

```gdscript
void set_auto_brace_completion_pairs(pairs: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_indent_enabled {#set_auto_indent_enabled}

```gdscript
void set_auto_indent_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_indent_prefixes {#set_auto_indent_prefixes}

```gdscript
void set_auto_indent_prefixes(prefixes: typedarray::String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_code_completion_enabled {#set_code_completion_enabled}

```gdscript
void set_code_completion_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_code_completion_prefixes {#set_code_completion_prefixes}

```gdscript
void set_code_completion_prefixes(prefixes: typedarray::String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_code_completion_selected_index {#set_code_completion_selected_index}

```gdscript
void set_code_completion_selected_index(index: int)
```

Sets the current selected completion option.

### set_code_hint {#set_code_hint}

```gdscript
void set_code_hint(code_hint: String)
```

Sets the code hint text. Pass an empty string to clear.

### set_code_hint_draw_below {#set_code_hint_draw_below}

```gdscript
void set_code_hint_draw_below(draw_below: bool)
```

If true, the code hint will draw below the main caret. If false, the code hint will draw above the main caret. See set_code_hint().

### set_code_region_tags {#set_code_region_tags}

```gdscript
void set_code_region_tags(start: String = "region", end: String = "endregion")
```

Sets the code region start and end tags (without comment delimiter).

### set_comment_delimiters {#set_comment_delimiters}

```gdscript
void set_comment_delimiters(comment_delimiters: typedarray::String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_bookmarks_gutter {#set_draw_bookmarks_gutter}

```gdscript
void set_draw_bookmarks_gutter(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_breakpoints_gutter {#set_draw_breakpoints_gutter}

```gdscript
void set_draw_breakpoints_gutter(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_executing_lines_gutter {#set_draw_executing_lines_gutter}

```gdscript
void set_draw_executing_lines_gutter(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_fold_gutter {#set_draw_fold_gutter}

```gdscript
void set_draw_fold_gutter(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_line_numbers {#set_draw_line_numbers}

```gdscript
void set_draw_line_numbers(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_highlight_matching_braces_enabled {#set_highlight_matching_braces_enabled}

```gdscript
void set_highlight_matching_braces_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_indent_size {#set_indent_size}

```gdscript
void set_indent_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_indent_using_spaces {#set_indent_using_spaces}

```gdscript
void set_indent_using_spaces(use_spaces: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_as_bookmarked {#set_line_as_bookmarked}

```gdscript
void set_line_as_bookmarked(line: int, bookmarked: bool)
```

Sets the given line as bookmarked. If true and gutters_draw_bookmarks is true, draws the bookmark icon in the gutter for this line. See get_bookmarked_lines() and is_line_bookmarked().

### set_line_as_breakpoint {#set_line_as_breakpoint}

```gdscript
void set_line_as_breakpoint(line: int, breakpointed: bool)
```

Sets the given line as a breakpoint. If true and gutters_draw_breakpoints_gutter is true, draws the breakpoint icon in the gutter for this line. See get_breakpointed_lines() and is_line_breakpointed().

### set_line_as_executing {#set_line_as_executing}

```gdscript
void set_line_as_executing(line: int, executing: bool)
```

Sets the given line as executing. If true and gutters_draw_executing_lines is true, draws the executing_line icon in the gutter for this line. See get_executing_lines() and is_line_executing().

### set_line_folding_enabled {#set_line_folding_enabled}

```gdscript
void set_line_folding_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_length_guidelines {#set_line_length_guidelines}

```gdscript
void set_line_length_guidelines(guideline_columns: typedarray::int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_numbers_min_digits {#set_line_numbers_min_digits}

```gdscript
void set_line_numbers_min_digits(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_numbers_zero_padded {#set_line_numbers_zero_padded}

```gdscript
void set_line_numbers_zero_padded(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_string_delimiters {#set_string_delimiters}

```gdscript
void set_string_delimiters(string_delimiters: typedarray::String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_symbol_lookup_on_click_enabled {#set_symbol_lookup_on_click_enabled}

```gdscript
void set_symbol_lookup_on_click_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_symbol_lookup_word_as_valid {#set_symbol_lookup_word_as_valid}

```gdscript
void set_symbol_lookup_word_as_valid(valid: bool)
```

Sets the symbol emitted by symbol_validate as a valid lookup.

### set_symbol_tooltip_on_hover_enabled {#set_symbol_tooltip_on_hover_enabled}

```gdscript
void set_symbol_tooltip_on_hover_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### toggle_foldable_line {#toggle_foldable_line}

```gdscript
void toggle_foldable_line(line: int)
```

Toggle the folding of the code block at the given line.

### toggle_foldable_lines_at_carets {#toggle_foldable_lines_at_carets}

```gdscript
void toggle_foldable_lines_at_carets()
```

Toggle the folding of the code block on all lines with a caret on them.

### unfold_all_lines {#unfold_all_lines}

```gdscript
void unfold_all_lines()
```

Unfolds all lines that are folded.

### unfold_line {#unfold_line}

```gdscript
void unfold_line(line: int)
```

Unfolds the given line if it is folded or if it is hidden under a folded line.

### unindent_lines {#unindent_lines}

```gdscript
void unindent_lines()
```

Unindents all lines that are selected or have a caret on them. Uses spaces or a tab depending on indent_use_spaces. Equivalent to the ProjectSettings.input/ui_text_dedent action. See indent_lines().

### update_code_completion_options {#update_code_completion_options}

```gdscript
void update_code_completion_options(force: bool)
```

Submits all completion options added with add_code_completion_option(). Will try to force the autocomplete menu to popup, if force is true.

Note: This will replace all current candidates.

## 属性

### symbol_lookup_on_click {#symbol_lookup_on_click}

```gdscript
symbol_lookup_on_click: bool
```

- void set_symbol_lookup_on_click_enabled(value: bool)
- bool is_symbol_lookup_on_click_enabled()

Set when a validated word from symbol_validate is clicked, the symbol_lookup should be emitted.

### symbol_tooltip_on_hover {#symbol_tooltip_on_hover}

```gdscript
symbol_tooltip_on_hover: bool
```

- void set_symbol_tooltip_on_hover_enabled(value: bool)
- bool is_symbol_tooltip_on_hover_enabled()

If true, the symbol_hovered signal is emitted when hovering over a word.

### line_folding {#line_folding}

```gdscript
line_folding: bool
```

- void set_line_folding_enabled(value: bool)
- bool is_line_folding_enabled()

If true, lines can be folded. Otherwise, line folding methods like fold_line() will not work and can_fold_line() will always return false. See gutters_draw_fold_gutter.

### line_length_guidelines {#line_length_guidelines}

```gdscript
line_length_guidelines: PackedInt32Array
```

- void set_line_length_guidelines(value: Array[int])
- Array[int] get_line_length_guidelines()

Draws vertical lines at the provided columns. The first entry is considered a main hard guideline and is drawn more prominently.

### gutters_draw_breakpoints_gutter {#gutters_draw_breakpoints_gutter}

```gdscript
gutters_draw_breakpoints_gutter: bool
```

- void set_draw_breakpoints_gutter(value: bool)
- bool is_drawing_breakpoints_gutter()

If true, breakpoints are drawn in the gutter. This gutter is shared with bookmarks and executing lines. Clicking the gutter will toggle the breakpoint for the line, see set_line_as_breakpoint().

### gutters_draw_bookmarks {#gutters_draw_bookmarks}

```gdscript
gutters_draw_bookmarks: bool
```

- void set_draw_bookmarks_gutter(value: bool)
- bool is_drawing_bookmarks_gutter()

If true, bookmarks are drawn in the gutter. This gutter is shared with breakpoints and executing lines. See set_line_as_bookmarked().

### gutters_draw_executing_lines {#gutters_draw_executing_lines}

```gdscript
gutters_draw_executing_lines: bool
```

- void set_draw_executing_lines_gutter(value: bool)
- bool is_drawing_executing_lines_gutter()

If true, executing lines are marked in the gutter. This gutter is shared with breakpoints and bookmarks. See set_line_as_executing().

### gutters_draw_line_numbers {#gutters_draw_line_numbers}

```gdscript
gutters_draw_line_numbers: bool
```

- void set_draw_line_numbers(value: bool)
- bool is_draw_line_numbers_enabled()

If true, the line number gutter is drawn. Line numbers start at 1 and are incremented for each line of text. Clicking and dragging in the line number gutter will select entire lines of text.

### gutters_zero_pad_line_numbers {#gutters_zero_pad_line_numbers}

```gdscript
gutters_zero_pad_line_numbers: bool
```

- void set_line_numbers_zero_padded(value: bool)
- bool is_line_numbers_zero_padded()

If true, line numbers drawn in the gutter are zero padded based on the total line count. Requires gutters_draw_line_numbers to be set to true.

### gutters_line_numbers_min_digits {#gutters_line_numbers_min_digits}

```gdscript
gutters_line_numbers_min_digits: int
```

- void set_line_numbers_min_digits(value: int)
- int get_line_numbers_min_digits()

The minimum width in digits reserved for the line number gutter.

### gutters_draw_fold_gutter {#gutters_draw_fold_gutter}

```gdscript
gutters_draw_fold_gutter: bool
```

- void set_draw_fold_gutter(value: bool)
- bool is_drawing_fold_gutter()

If true, the fold gutter is drawn. In this gutter, the can_fold_code_region icon is drawn for each foldable line (see can_fold_line()) and the folded_code_region icon is drawn for each folded line (see is_line_folded()). These icons can be clicked to toggle the fold state, see toggle_foldable_line(). line_folding must be true to show icons.

### delimiter_strings {#delimiter_strings}

```gdscript
delimiter_strings: PackedStringArray
```

- void set_string_delimiters(value: Array[String])
- Array[String] get_string_delimiters()

Sets the string delimiters. All existing string delimiters will be removed.

### delimiter_comments {#delimiter_comments}

```gdscript
delimiter_comments: PackedStringArray
```

- void set_comment_delimiters(value: Array[String])
- Array[String] get_comment_delimiters()

Sets the comment delimiters. All existing comment delimiters will be removed.

### code_completion_enabled {#code_completion_enabled}

```gdscript
code_completion_enabled: bool
```

- void set_code_completion_enabled(value: bool)
- bool is_code_completion_enabled()

If true, the ProjectSettings.input/ui_text_completion_query action requests code completion. To handle it, see _request_code_completion() or code_completion_requested.

### code_completion_prefixes {#code_completion_prefixes}

```gdscript
code_completion_prefixes: PackedStringArray
```

- void set_code_completion_prefixes(value: Array[String])
- Array[String] get_code_completion_prefixes()

Sets prefixes that will trigger code completion.

### indent_size {#indent_size}

```gdscript
indent_size: int
```

- void set_indent_size(value: int)
- int get_indent_size()

Size of the tabulation indent (one Tab press) in characters. If indent_use_spaces is enabled the number of spaces to use.

### indent_use_spaces {#indent_use_spaces}

```gdscript
indent_use_spaces: bool
```

- void set_indent_using_spaces(value: bool)
- bool is_indent_using_spaces()

Use spaces instead of tabs for indentation.

### indent_automatic {#indent_automatic}

```gdscript
indent_automatic: bool
```

- void set_auto_indent_enabled(value: bool)
- bool is_auto_indent_enabled()

If true, an extra indent is automatically inserted when a new line is added and a prefix in indent_automatic_prefixes is found. If a brace pair opening key is found, the matching closing brace will be moved to another new line (see auto_brace_completion_pairs).

### indent_automatic_prefixes {#indent_automatic_prefixes}

```gdscript
indent_automatic_prefixes: PackedStringArray
```

- void set_auto_indent_prefixes(value: Array[String])
- Array[String] get_auto_indent_prefixes()

Prefixes to trigger an automatic indent. Used when indent_automatic is set to true.

### auto_brace_completion_enabled {#auto_brace_completion_enabled}

```gdscript
auto_brace_completion_enabled: bool
```

- void set_auto_brace_completion_enabled(value: bool)
- bool is_auto_brace_completion_enabled()

If true, uses auto_brace_completion_pairs to automatically insert the closing brace when the opening brace is inserted by typing or autocompletion. Also automatically removes the closing brace when using backspace on the opening brace.

### auto_brace_completion_highlight_matching {#auto_brace_completion_highlight_matching}

```gdscript
auto_brace_completion_highlight_matching: bool
```

- void set_highlight_matching_braces_enabled(value: bool)
- bool is_highlight_matching_braces_enabled()

If true, highlights brace pairs when the caret is on either one, using auto_brace_completion_pairs. If matching, the pairs will be underlined. If a brace is unmatched, it is colored with brace_mismatch_color.

### auto_brace_completion_pairs {#auto_brace_completion_pairs}

```gdscript
auto_brace_completion_pairs: Dictionary
```

- void set_auto_brace_completion_pairs(value: Dictionary)
- Dictionary get_auto_brace_completion_pairs()

Sets the brace pairs to be autocompleted. For each entry in the dictionary, the key is the opening brace and the value is the closing brace that matches it. A brace is a String made of symbols. See auto_brace_completion_enabled and auto_brace_completion_highlight_matching.

## 信号

### breakpoint_toggled {#breakpoint_toggled}

```gdscript
signal breakpoint_toggled(line: int)
```

Emitted when a breakpoint is added or removed from a line. If the line is removed via backspace, a signal is emitted at the old line.

### code_completion_requested {#code_completion_requested}

```gdscript
signal code_completion_requested()
```

Emitted when the user requests code completion. This signal will not be sent if _request_code_completion() is overridden or code_completion_enabled is false.

### symbol_lookup {#symbol_lookup}

```gdscript
signal symbol_lookup(symbol: String, line: int, column: int)
```

Emitted when the user has clicked on a valid symbol.

### symbol_validate {#symbol_validate}

```gdscript
signal symbol_validate(symbol: String)
```

Emitted when the user hovers over a symbol. The symbol should be validated and responded to, by calling set_symbol_lookup_word_as_valid().

Note: symbol_lookup_on_click must be true for this signal to be emitted.

### symbol_hovered {#symbol_hovered}

```gdscript
signal symbol_hovered(symbol: String, line: int, column: int)
```

Emitted when the user hovers over a symbol. Unlike Control.mouse_entered, this signal is not emitted immediately, but when the cursor is over the symbol for ProjectSettings.gui/timers/tooltip_delay_sec seconds.

Note: symbol_tooltip_on_hover must be true for this signal to be emitted.

## 枚举

### CodeCompletionKind {#enum-CodeCompletionKind}

```gdscript
enum CodeCompletionKind { KIND_CLASS = 0, KIND_FUNCTION = 1, KIND_SIGNAL = 2, KIND_VARIABLE = 3, KIND_MEMBER = 4, KIND_ENUM = 5, KIND_CONSTANT = 6, KIND_NODE_PATH = 7, KIND_FILE_PATH = 8, KIND_PLAIN_TEXT = 9, KIND_KEYWORD = 10 }
```

CodeCompletionKind KIND_CLASS = 0

Marks the option as a class.

CodeCompletionKind KIND_FUNCTION = 1

Marks the option as a function.

CodeCompletionKind KIND_SIGNAL = 2

Marks the option as a Godot signal.

CodeCompletionKind KIND_VARIABLE = 3

Marks the option as a variable.

CodeCompletionKind KIND_MEMBER = 4

Marks the option as a member.

CodeCompletionKind KIND_ENUM = 5

Marks the option as an enum entry.

CodeCompletionKind KIND_CONSTANT = 6

Marks the option as a constant.

CodeCompletionKind KIND_NODE_PATH = 7

Marks the option as a Godot node path.

CodeCompletionKind KIND_FILE_PATH = 8

Marks the option as a file path.

CodeCompletionKind KIND_PLAIN_TEXT = 9

Marks the option as unclassified or plain text.

CodeCompletionKind KIND_KEYWORD = 10

Marks the option as a keyword.

#### KIND_CLASS {#KIND_CLASS}

```gdscript
KIND_CLASS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_FUNCTION {#KIND_FUNCTION}

```gdscript
KIND_FUNCTION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_SIGNAL {#KIND_SIGNAL}

```gdscript
KIND_SIGNAL = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_VARIABLE {#KIND_VARIABLE}

```gdscript
KIND_VARIABLE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_MEMBER {#KIND_MEMBER}

```gdscript
KIND_MEMBER = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_ENUM {#KIND_ENUM}

```gdscript
KIND_ENUM = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_CONSTANT {#KIND_CONSTANT}

```gdscript
KIND_CONSTANT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_NODE_PATH {#KIND_NODE_PATH}

```gdscript
KIND_NODE_PATH = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_FILE_PATH {#KIND_FILE_PATH}

```gdscript
KIND_FILE_PATH = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_PLAIN_TEXT {#KIND_PLAIN_TEXT}

```gdscript
KIND_PLAIN_TEXT = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KIND_KEYWORD {#KIND_KEYWORD}

```gdscript
KIND_KEYWORD = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CodeCompletionLocation {#enum-CodeCompletionLocation}

```gdscript
enum CodeCompletionLocation { LOCATION_LOCAL = 0, LOCATION_PARENT_MASK = 256, LOCATION_OTHER_USER_CODE = 512, LOCATION_OTHER = 1024 }
```

CodeCompletionLocation LOCATION_LOCAL = 0

The option is local to the location of the code completion query - e.g. a local variable. Subsequent value of location represent options from the outer class, the exact value represent how far they are (in terms of inner classes).

CodeCompletionLocation LOCATION_PARENT_MASK = 256

The option is from the containing class or a parent class, relative to the location of the code completion query. Perform a bitwise OR with the class depth (e.g. 0 for the local class, 1 for the parent, 2 for the grandparent, etc.) to store the depth of an option in the class or a parent class.

CodeCompletionLocation LOCATION_OTHER_USER_CODE = 512

The option is from user code which is not local and not in a derived class (e.g. Autoload Singletons).

CodeCompletionLocation LOCATION_OTHER = 1024

The option is from other engine code, not covered by the other enum constants - e.g. built-in classes.

#### LOCATION_LOCAL {#LOCATION_LOCAL}

```gdscript
LOCATION_LOCAL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOCATION_PARENT_MASK {#LOCATION_PARENT_MASK}

```gdscript
LOCATION_PARENT_MASK = 256
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOCATION_OTHER_USER_CODE {#LOCATION_OTHER_USER_CODE}

```gdscript
LOCATION_OTHER_USER_CODE = 512
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOCATION_OTHER {#LOCATION_OTHER}

```gdscript
LOCATION_OTHER = 1024
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
