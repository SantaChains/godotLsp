# RegExMatch

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RegExMatch**

> Contains the results of a single RegEx match returned by RegEx.search() and RegEx.search_all(). It can be used to find the position and range of the match and its capturing groups, and it can extract its substring for you.

<details><summary>官方英文描述（中文精译进行中）</summary>

Contains the results of a single RegEx match returned by RegEx.search() and RegEx.search_all(). It can be used to find the position and range of the match and its capturing groups, and it can extract its substring for you.

</details>

## 方法

### get_end {#get_end}

```gdscript
int get_end(name: Variant = 0) const
```

Returns the end position of the match within the source string. The end position of capturing groups can be retrieved by providing its group number as an integer or its string name (if it's a named group). The default value of 0 refers to the whole pattern.

Returns -1 if the group did not match or doesn't exist.

### get_group_count {#get_group_count}

```gdscript
int get_group_count() const
```

Returns the number of capturing groups.

### get_names {#get_names}

```gdscript
Dictionary get_names() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_start {#get_start}

```gdscript
int get_start(name: Variant = 0) const
```

Returns the starting position of the match within the source string. The starting position of capturing groups can be retrieved by providing its group number as an integer or its string name (if it's a named group). The default value of 0 refers to the whole pattern.

Returns -1 if the group did not match or doesn't exist.

### get_string {#get_string}

```gdscript
String get_string(name: Variant = 0) const
```

Returns the substring of the match from the source string. Capturing groups can be retrieved by providing its group number as an integer or its string name (if it's a named group). The default value of 0 refers to the whole pattern.

Returns an empty string if the group did not match or doesn't exist.

### get_strings {#get_strings}

```gdscript
PackedStringArray get_strings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subject {#get_subject}

```gdscript
String get_subject() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### subject {#subject}

```gdscript
subject: String
```

- String get_subject()

The source string used with the search pattern to find this matching result.

### names {#names}

```gdscript
names: Dictionary
```

- Dictionary get_names()

A dictionary of named groups and its corresponding group number. Only groups that were matched are included. If multiple groups have the same name, that name would refer to the first matching one.

### strings {#strings}

```gdscript
strings: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
