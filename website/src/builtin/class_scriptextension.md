# ScriptExtension

**继承**：[Script](class_script.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ScriptExtension**

## 方法

### _can_instantiate {#_can_instantiate}

```gdscript
bool _can_instantiate() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _editor_can_reload_from_file {#_editor_can_reload_from_file}

```gdscript
bool _editor_can_reload_from_file() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_base_script {#_get_base_script}

```gdscript
Script _get_base_script() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_class_icon_path {#_get_class_icon_path}

```gdscript
String _get_class_icon_path() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_constants {#_get_constants}

```gdscript
Dictionary _get_constants() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_doc_class_name {#_get_doc_class_name}

```gdscript
StringName _get_doc_class_name() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_documentation {#_get_documentation}

```gdscript
typedarray::Dictionary _get_documentation() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_global_name {#_get_global_name}

```gdscript
StringName _get_global_name() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_instance_base_type {#_get_instance_base_type}

```gdscript
StringName _get_instance_base_type() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_language {#_get_language}

```gdscript
ScriptLanguage _get_language() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_member_line {#_get_member_line}

```gdscript
int _get_member_line(member: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_members {#_get_members}

```gdscript
typedarray::StringName _get_members() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_method_info {#_get_method_info}

```gdscript
Dictionary _get_method_info(method: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_property_default_value {#_get_property_default_value}

```gdscript
Variant _get_property_default_value(property: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_rpc_config {#_get_rpc_config}

```gdscript
Variant _get_rpc_config() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_script_method_argument_count {#_get_script_method_argument_count}

```gdscript
Variant _get_script_method_argument_count(method: StringName) virtual const
```

Return the expected argument count for the given method, or null if it can't be determined (which will then fall back to the default behavior).

### _get_script_method_list {#_get_script_method_list}

```gdscript
typedarray::Dictionary _get_script_method_list() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_script_property_list {#_get_script_property_list}

```gdscript
typedarray::Dictionary _get_script_property_list() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_script_signal_list {#_get_script_signal_list}

```gdscript
typedarray::Dictionary _get_script_signal_list() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _get_source_code {#_get_source_code}

```gdscript
String _get_source_code() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _has_method {#_has_method}

```gdscript
bool _has_method(method: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _has_property_default_value {#_has_property_default_value}

```gdscript
bool _has_property_default_value(property: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _has_script_signal {#_has_script_signal}

```gdscript
bool _has_script_signal(signal: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _has_source_code {#_has_source_code}

```gdscript
bool _has_source_code() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _has_static_method {#_has_static_method}

```gdscript
bool _has_static_method(method: StringName) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _inherits_script {#_inherits_script}

```gdscript
bool _inherits_script(script: Script) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _instance_create {#_instance_create}

```gdscript
void* _instance_create(for_object: Object) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _instance_has {#_instance_has}

```gdscript
bool _instance_has(object: Object) virtual const
```

Deprecated: This method is not called by the engine.

### _is_abstract {#_is_abstract}

```gdscript
bool _is_abstract() virtual const
```

Returns true if the script is an abstract script. Abstract scripts cannot be instantiated directly, instead other scripts should inherit them. Abstract scripts will be either unselectable or hidden in the Create New Node dialog (unselectable if there are non-abstract classes inheriting it, otherwise hidden).

### _is_placeholder_fallback_enabled {#_is_placeholder_fallback_enabled}

```gdscript
bool _is_placeholder_fallback_enabled() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _is_tool {#_is_tool}

```gdscript
bool _is_tool() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _is_valid {#_is_valid}

```gdscript
bool _is_valid() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _placeholder_erased {#_placeholder_erased}

```gdscript
void _placeholder_erased(placeholder: void*) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _placeholder_instance_create {#_placeholder_instance_create}

```gdscript
void* _placeholder_instance_create(for_object: Object) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _reload {#_reload}

```gdscript
Error _reload(keep_state: bool) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _set_source_code {#_set_source_code}

```gdscript
void _set_source_code(code: String) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _update_exports {#_update_exports}

```gdscript
void _update_exports() virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!
