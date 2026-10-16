# ScriptLanguageExtension

**继承**：[ScriptLanguage](class_scriptlanguage.md) → [Object](class_object.md) → **ScriptLanguageExtension**

## 方法

### _add_global_constant {#_add_global_constant}

```gdscript
void _add_global_constant(name: StringName, value: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _add_named_global_constant {#_add_named_global_constant}

```gdscript
void _add_named_global_constant(name: StringName, value: Variant) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _auto_indent_code {#_auto_indent_code}

```gdscript
String _auto_indent_code(code: String, from_line: int, to_line: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _can_inherit_from_file {#_can_inherit_from_file}

```gdscript
bool _can_inherit_from_file() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _can_make_function {#_can_make_function}

```gdscript
bool _can_make_function() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _complete_code {#_complete_code}

```gdscript
Dictionary _complete_code(code: String, path: String, owner: Object) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _create_script {#_create_script}

```gdscript
Object _create_script() virtual const
```

Deprecated: This method is not called by the engine.

### _debug_get_current_stack_info {#_debug_get_current_stack_info}

```gdscript
typedarray::Dictionary _debug_get_current_stack_info() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_error {#_debug_get_error}

```gdscript
String _debug_get_error() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_globals {#_debug_get_globals}

```gdscript
Dictionary _debug_get_globals(max_subitems: int, max_depth: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_count {#_debug_get_stack_level_count}

```gdscript
int _debug_get_stack_level_count() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_function {#_debug_get_stack_level_function}

```gdscript
String _debug_get_stack_level_function(level: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_instance {#_debug_get_stack_level_instance}

```gdscript
void* _debug_get_stack_level_instance(level: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_line {#_debug_get_stack_level_line}

```gdscript
int _debug_get_stack_level_line(level: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_locals {#_debug_get_stack_level_locals}

```gdscript
Dictionary _debug_get_stack_level_locals(level: int, max_subitems: int, max_depth: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_members {#_debug_get_stack_level_members}

```gdscript
Dictionary _debug_get_stack_level_members(level: int, max_subitems: int, max_depth: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _debug_get_stack_level_source {#_debug_get_stack_level_source}

```gdscript
String _debug_get_stack_level_source(level: int) virtual const
```

Returns the source associated with a given debug stack position.

### _debug_parse_stack_level_expression {#_debug_parse_stack_level_expression}

```gdscript
String _debug_parse_stack_level_expression(level: int, expression: String, max_subitems: int, max_depth: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _find_function {#_find_function}

```gdscript
int _find_function(function: String, code: String) virtual const
```

Returns the line where the function is defined in the code, or -1 if the function is not present.

### _finish {#_finish}

```gdscript
void _finish() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _frame {#_frame}

```gdscript
void _frame() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_built_in_templates {#_get_built_in_templates}

```gdscript
typedarray::Dictionary _get_built_in_templates(object: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_comment_delimiters {#_get_comment_delimiters}

```gdscript
PackedStringArray _get_comment_delimiters() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_doc_comment_delimiters {#_get_doc_comment_delimiters}

```gdscript
PackedStringArray _get_doc_comment_delimiters() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_extension {#_get_extension}

```gdscript
String _get_extension() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_global_class_name {#_get_global_class_name}

```gdscript
Dictionary _get_global_class_name(path: String) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_name {#_get_name}

```gdscript
String _get_name() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_public_annotations {#_get_public_annotations}

```gdscript
typedarray::Dictionary _get_public_annotations() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_public_constants {#_get_public_constants}

```gdscript
Dictionary _get_public_constants() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_public_functions {#_get_public_functions}

```gdscript
typedarray::Dictionary _get_public_functions() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_recognized_extensions {#_get_recognized_extensions}

```gdscript
PackedStringArray _get_recognized_extensions() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_reserved_words {#_get_reserved_words}

```gdscript
PackedStringArray _get_reserved_words() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_string_delimiters {#_get_string_delimiters}

```gdscript
PackedStringArray _get_string_delimiters() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_type {#_get_type}

```gdscript
String _get_type() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _handles_global_class_type {#_handles_global_class_type}

```gdscript
bool _handles_global_class_type(type: String) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _has_named_classes {#_has_named_classes}

```gdscript
bool _has_named_classes() virtual const
```

Deprecated: This method is not called by the engine.

### _init {#_init}

```gdscript
void _init() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _is_control_flow_keyword {#_is_control_flow_keyword}

```gdscript
bool _is_control_flow_keyword(keyword: String) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _is_using_templates {#_is_using_templates}

```gdscript
bool _is_using_templates() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _lookup_code {#_lookup_code}

```gdscript
Dictionary _lookup_code(code: String, symbol: String, path: String, owner: Object) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _make_function {#_make_function}

```gdscript
String _make_function(class_name: String, function_name: String, function_args: PackedStringArray) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _make_template {#_make_template}

```gdscript
Script _make_template(template: String, class_name: String, base_class_name: String) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _open_in_external_editor {#_open_in_external_editor}

```gdscript
Error _open_in_external_editor(script: Script, line: int, column: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _overrides_external_editor {#_overrides_external_editor}

```gdscript
bool _overrides_external_editor() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _preferred_file_name_casing {#_preferred_file_name_casing}

```gdscript
ScriptLanguage.ScriptNameCasing _preferred_file_name_casing() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _profiling_get_accumulated_data {#_profiling_get_accumulated_data}

```gdscript
int _profiling_get_accumulated_data(info_array: ScriptLanguageExtensionProfilingInfo*, info_max: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _profiling_get_frame_data {#_profiling_get_frame_data}

```gdscript
int _profiling_get_frame_data(info_array: ScriptLanguageExtensionProfilingInfo*, info_max: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _profiling_set_save_native_calls {#_profiling_set_save_native_calls}

```gdscript
void _profiling_set_save_native_calls(enable: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _profiling_start {#_profiling_start}

```gdscript
void _profiling_start() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _profiling_stop {#_profiling_stop}

```gdscript
void _profiling_stop() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _reload_all_scripts {#_reload_all_scripts}

```gdscript
void _reload_all_scripts() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _reload_scripts {#_reload_scripts}

```gdscript
void _reload_scripts(scripts: Array, soft_reload: bool) virtual
```

Reloads all scripts from disk and the specifics of how that happens is ScriptLanguageExtension specific.

### _reload_tool_script {#_reload_tool_script}

```gdscript
void _reload_tool_script(script: Script, soft_reload: bool) virtual
```

Reloads the given script from disk and the specifics of how that happens is ScriptLanguageExtension specific.

### _remove_named_global_constant {#_remove_named_global_constant}

```gdscript
void _remove_named_global_constant(name: StringName) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _supports_builtin_mode {#_supports_builtin_mode}

```gdscript
bool _supports_builtin_mode() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _supports_documentation {#_supports_documentation}

```gdscript
bool _supports_documentation() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _thread_enter {#_thread_enter}

```gdscript
void _thread_enter() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _thread_exit {#_thread_exit}

```gdscript
void _thread_exit() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _validate {#_validate}

```gdscript
Dictionary _validate(script: String, path: String, validate_functions: bool, validate_errors: bool, validate_warnings: bool, validate_safe_lines: bool) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _validate_path {#_validate_path}

```gdscript
String _validate_path(path: String) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

## 枚举

### LookupResultType {#enum-LookupResultType}

```gdscript
enum LookupResultType { LOOKUP_RESULT_SCRIPT_LOCATION = 0, LOOKUP_RESULT_CLASS = 1, LOOKUP_RESULT_CLASS_CONSTANT = 2, LOOKUP_RESULT_CLASS_PROPERTY = 3, LOOKUP_RESULT_CLASS_METHOD = 4, LOOKUP_RESULT_CLASS_SIGNAL = 5, LOOKUP_RESULT_CLASS_ENUM = 6, LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE = 7, LOOKUP_RESULT_CLASS_ANNOTATION = 8, LOOKUP_RESULT_LOCAL_CONSTANT = 9, LOOKUP_RESULT_LOCAL_VARIABLE = 10, LOOKUP_RESULT_MAX = 11 }
```

LookupResultType LOOKUP_RESULT_SCRIPT_LOCATION = 0

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS = 1

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS_CONSTANT = 2

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS_PROPERTY = 3

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS_METHOD = 4

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS_SIGNAL = 5

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS_ENUM = 6

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE = 7

Deprecated: This constant may be changed or removed in future versions.

LookupResultType LOOKUP_RESULT_CLASS_ANNOTATION = 8

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_LOCAL_CONSTANT = 9

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_LOCAL_VARIABLE = 10

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

LookupResultType LOOKUP_RESULT_MAX = 11

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### LOOKUP_RESULT_SCRIPT_LOCATION {#LOOKUP_RESULT_SCRIPT_LOCATION}

```gdscript
LOOKUP_RESULT_SCRIPT_LOCATION = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS {#LOOKUP_RESULT_CLASS}

```gdscript
LOOKUP_RESULT_CLASS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_CONSTANT {#LOOKUP_RESULT_CLASS_CONSTANT}

```gdscript
LOOKUP_RESULT_CLASS_CONSTANT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_PROPERTY {#LOOKUP_RESULT_CLASS_PROPERTY}

```gdscript
LOOKUP_RESULT_CLASS_PROPERTY = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_METHOD {#LOOKUP_RESULT_CLASS_METHOD}

```gdscript
LOOKUP_RESULT_CLASS_METHOD = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_SIGNAL {#LOOKUP_RESULT_CLASS_SIGNAL}

```gdscript
LOOKUP_RESULT_CLASS_SIGNAL = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_ENUM {#LOOKUP_RESULT_CLASS_ENUM}

```gdscript
LOOKUP_RESULT_CLASS_ENUM = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE {#LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE}

```gdscript
LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_CLASS_ANNOTATION {#LOOKUP_RESULT_CLASS_ANNOTATION}

```gdscript
LOOKUP_RESULT_CLASS_ANNOTATION = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_LOCAL_CONSTANT {#LOOKUP_RESULT_LOCAL_CONSTANT}

```gdscript
LOOKUP_RESULT_LOCAL_CONSTANT = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_LOCAL_VARIABLE {#LOOKUP_RESULT_LOCAL_VARIABLE}

```gdscript
LOOKUP_RESULT_LOCAL_VARIABLE = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOKUP_RESULT_MAX {#LOOKUP_RESULT_MAX}

```gdscript
LOOKUP_RESULT_MAX = 11
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

### CodeCompletionKind {#enum-CodeCompletionKind}

```gdscript
enum CodeCompletionKind { CODE_COMPLETION_KIND_CLASS = 0, CODE_COMPLETION_KIND_FUNCTION = 1, CODE_COMPLETION_KIND_SIGNAL = 2, CODE_COMPLETION_KIND_VARIABLE = 3, CODE_COMPLETION_KIND_MEMBER = 4, CODE_COMPLETION_KIND_ENUM = 5, CODE_COMPLETION_KIND_CONSTANT = 6, CODE_COMPLETION_KIND_NODE_PATH = 7, CODE_COMPLETION_KIND_FILE_PATH = 8, CODE_COMPLETION_KIND_PLAIN_TEXT = 9, CODE_COMPLETION_KIND_KEYWORD = 10, CODE_COMPLETION_KIND_MAX = 11 }
```

CodeCompletionKind CODE_COMPLETION_KIND_CLASS = 0

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_FUNCTION = 1

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_SIGNAL = 2

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_VARIABLE = 3

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_MEMBER = 4

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_ENUM = 5

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_CONSTANT = 6

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_NODE_PATH = 7

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_FILE_PATH = 8

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_PLAIN_TEXT = 9

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_KEYWORD = 10

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

CodeCompletionKind CODE_COMPLETION_KIND_MAX = 11

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### CODE_COMPLETION_KIND_CLASS {#CODE_COMPLETION_KIND_CLASS}

```gdscript
CODE_COMPLETION_KIND_CLASS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_FUNCTION {#CODE_COMPLETION_KIND_FUNCTION}

```gdscript
CODE_COMPLETION_KIND_FUNCTION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_SIGNAL {#CODE_COMPLETION_KIND_SIGNAL}

```gdscript
CODE_COMPLETION_KIND_SIGNAL = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_VARIABLE {#CODE_COMPLETION_KIND_VARIABLE}

```gdscript
CODE_COMPLETION_KIND_VARIABLE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_MEMBER {#CODE_COMPLETION_KIND_MEMBER}

```gdscript
CODE_COMPLETION_KIND_MEMBER = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_ENUM {#CODE_COMPLETION_KIND_ENUM}

```gdscript
CODE_COMPLETION_KIND_ENUM = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_CONSTANT {#CODE_COMPLETION_KIND_CONSTANT}

```gdscript
CODE_COMPLETION_KIND_CONSTANT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_NODE_PATH {#CODE_COMPLETION_KIND_NODE_PATH}

```gdscript
CODE_COMPLETION_KIND_NODE_PATH = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_FILE_PATH {#CODE_COMPLETION_KIND_FILE_PATH}

```gdscript
CODE_COMPLETION_KIND_FILE_PATH = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_PLAIN_TEXT {#CODE_COMPLETION_KIND_PLAIN_TEXT}

```gdscript
CODE_COMPLETION_KIND_PLAIN_TEXT = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_KEYWORD {#CODE_COMPLETION_KIND_KEYWORD}

```gdscript
CODE_COMPLETION_KIND_KEYWORD = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CODE_COMPLETION_KIND_MAX {#CODE_COMPLETION_KIND_MAX}

```gdscript
CODE_COMPLETION_KIND_MAX = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
