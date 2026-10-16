# ClassDB

**继承**：[Object](class_object.md) → **ClassDB**

> Provides access to metadata stored for every available engine class.

<details><summary>官方英文描述（中文精译进行中）</summary>

Provides access to metadata stored for every available engine class.

Note: Script-defined classes with class_name are not part of ClassDB, so they will not return reflection data such as a method or property list. However, GDExtension-defined classes are part of ClassDB, so they will return reflection data.

</details>

## 方法

### can_instantiate {#can_instantiate}

```gdscript
bool can_instantiate(class: StringName) const
```

Returns true if objects can be instantiated from the specified class, otherwise returns false.

### class_call_static {#class_call_static}

```gdscript
Variant class_call_static(class: StringName, method: StringName, ...)
```

Calls a static method on a class.

### class_exists {#class_exists}

```gdscript
bool class_exists(class: StringName) const
```

Returns whether the specified class is available or not.

### class_get_api_type {#class_get_api_type}

```gdscript
ClassDB.APIType class_get_api_type(class: StringName) const
```

Returns the API type of the specified class.

### class_get_enum_constants {#class_get_enum_constants}

```gdscript
PackedStringArray class_get_enum_constants(class: StringName, enum: StringName, no_inheritance: bool = false) const
```

Returns an array with all the keys in enum of class or its ancestry.

### class_get_enum_list {#class_get_enum_list}

```gdscript
PackedStringArray class_get_enum_list(class: StringName, no_inheritance: bool = false) const
```

Returns an array with all the enums of class or its ancestry.

### class_get_integer_constant {#class_get_integer_constant}

```gdscript
int class_get_integer_constant(class: StringName, name: StringName) const
```

Returns the value of the integer constant name of class or its ancestry. Always returns 0 when the constant could not be found.

### class_get_integer_constant_enum {#class_get_integer_constant_enum}

```gdscript
StringName class_get_integer_constant_enum(class: StringName, name: StringName, no_inheritance: bool = false) const
```

Returns which enum the integer constant name of class or its ancestry belongs to.

### class_get_integer_constant_list {#class_get_integer_constant_list}

```gdscript
PackedStringArray class_get_integer_constant_list(class: StringName, no_inheritance: bool = false) const
```

Returns an array with the names all the integer constants of class or its ancestry.

### class_get_method_argument_count {#class_get_method_argument_count}

```gdscript
int class_get_method_argument_count(class: StringName, method: StringName, no_inheritance: bool = false) const
```

Returns the number of arguments of the method method of class or its ancestry if no_inheritance is false.

### class_get_method_list {#class_get_method_list}

```gdscript
typedarray::Dictionary class_get_method_list(class: StringName, no_inheritance: bool = false) const
```

Returns an array with all the methods of class or its ancestry if no_inheritance is false. Every element of the array is a Dictionary with the following keys: args, default_args, flags, id, name, return: (class_name, hint, hint_string, name, type, usage).

Note: In exported release builds the debug info is not available, so the returned dictionaries will contain only method names.

### class_get_property {#class_get_property}

```gdscript
Variant class_get_property(object: Object, property: StringName) const
```

Returns the value of property of object or its ancestry.

### class_get_property_default_value {#class_get_property_default_value}

```gdscript
Variant class_get_property_default_value(class: StringName, property: StringName) const
```

Returns the default value of property of class or its ancestor classes.

### class_get_property_getter {#class_get_property_getter}

```gdscript
StringName class_get_property_getter(class: StringName, property: StringName)
```

Returns the getter method name of property of class.

### class_get_property_list {#class_get_property_list}

```gdscript
typedarray::Dictionary class_get_property_list(class: StringName, no_inheritance: bool = false) const
```

Returns an array with all the properties of class or its ancestry if no_inheritance is false.

### class_get_property_setter {#class_get_property_setter}

```gdscript
StringName class_get_property_setter(class: StringName, property: StringName)
```

Returns the setter method name of property of class.

### class_get_signal {#class_get_signal}

```gdscript
Dictionary class_get_signal(class: StringName, signal: StringName) const
```

Returns the signal data of class or its ancestry. The returned value is a Dictionary with the following keys: args, default_args, flags, id, name, return: (class_name, hint, hint_string, name, type, usage).

### class_get_signal_list {#class_get_signal_list}

```gdscript
typedarray::Dictionary class_get_signal_list(class: StringName, no_inheritance: bool = false) const
```

Returns an array with all the signals of class or its ancestry if no_inheritance is false. Every element of the array is a Dictionary as described in class_get_signal().

### class_has_enum {#class_has_enum}

```gdscript
bool class_has_enum(class: StringName, name: StringName, no_inheritance: bool = false) const
```

Returns whether class or its ancestry has an enum called name or not.

### class_has_integer_constant {#class_has_integer_constant}

```gdscript
bool class_has_integer_constant(class: StringName, name: StringName) const
```

Returns whether class or its ancestry has an integer constant called name or not.

### class_has_method {#class_has_method}

```gdscript
bool class_has_method(class: StringName, method: StringName, no_inheritance: bool = false) const
```

Returns whether class (or its ancestry if no_inheritance is false) has a method called method or not.

### class_has_signal {#class_has_signal}

```gdscript
bool class_has_signal(class: StringName, signal: StringName) const
```

Returns whether class or its ancestry has a signal called signal or not.

### class_set_property {#class_set_property}

```gdscript
Error class_set_property(object: Object, property: StringName, value: Variant) const
```

Sets property value of object to value.

### get_class_list {#get_class_list}

```gdscript
PackedStringArray get_class_list() const
```

Returns the names of all engine classes available.

Note: Script-defined classes with class_name are not included in this list. Use ProjectSettings.get_global_class_list() to get a list of script-defined classes instead.

### get_inheriters_from_class {#get_inheriters_from_class}

```gdscript
PackedStringArray get_inheriters_from_class(class: StringName) const
```

Returns the names of all engine classes that directly or indirectly inherit from class.

### get_parent_class {#get_parent_class}

```gdscript
StringName get_parent_class(class: StringName) const
```

Returns the parent class of class.

### instantiate {#instantiate}

```gdscript
Variant instantiate(class: StringName) const
```

Creates an instance of class.

### is_class_enabled {#is_class_enabled}

```gdscript
bool is_class_enabled(class: StringName) const
```

Returns whether this class is enabled or not.

### is_class_enum_bitfield {#is_class_enum_bitfield}

```gdscript
bool is_class_enum_bitfield(class: StringName, enum: StringName, no_inheritance: bool = false) const
```

Returns whether class (or its ancestor classes if no_inheritance is false) has an enum called enum that is a bitfield.

### is_parent_class {#is_parent_class}

```gdscript
bool is_parent_class(class: StringName, inherits: StringName) const
```

Returns whether inherits is an ancestor of class or not.

## 枚举

### APIType {#enum-APIType}

```gdscript
enum APIType { API_CORE = 0, API_EDITOR = 1, API_EXTENSION = 2, API_EDITOR_EXTENSION = 3, API_NONE = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### API_CORE {#API_CORE}

```gdscript
API_CORE = 0
```

Native Core class type.

#### API_EDITOR {#API_EDITOR}

```gdscript
API_EDITOR = 1
```

Native Editor class type.

#### API_EXTENSION {#API_EXTENSION}

```gdscript
API_EXTENSION = 2
```

GDExtension class type.

#### API_EDITOR_EXTENSION {#API_EDITOR_EXTENSION}

```gdscript
API_EDITOR_EXTENSION = 3
```

GDExtension Editor class type.

#### API_NONE {#API_NONE}

```gdscript
API_NONE = 4
```

Unknown class type.

---
