# EditorFileSystemDirectory

**继承**：[Object](class_object.md) → **EditorFileSystemDirectory**

> A more generalized, low-level variation of the directory concept.

<details><summary>官方英文描述</summary>

A more generalized, low-level variation of the directory concept.

</details>

## 方法

### find_dir_index {#find_dir_index}

```gdscript
int find_dir_index(name: String) const
```

Returns the index of the directory with name name or -1 if not found.

### find_file_index {#find_file_index}

```gdscript
int find_file_index(name: String) const
```

Returns the index of the file with name name or -1 if not found.

### get_file {#get_file}

```gdscript
String get_file(idx: int) const
```

Returns the name of the file at index idx.

### get_file_count {#get_file_count}

```gdscript
int get_file_count() const
```

Returns the number of files in this directory.

### get_file_import_is_valid {#get_file_import_is_valid}

```gdscript
bool get_file_import_is_valid(idx: int) const
```

Returns true if the file at index idx imported properly.

### get_file_path {#get_file_path}

```gdscript
String get_file_path(idx: int) const
```

Returns the path to the file at index idx.

### get_file_script_class_extends {#get_file_script_class_extends}

```gdscript
String get_file_script_class_extends(idx: int) const
```

Returns the base class of the script class defined in the file at index idx. If the file doesn't define a script class using the class_name syntax, this will return an empty string.

### get_file_script_class_name {#get_file_script_class_name}

```gdscript
String get_file_script_class_name(idx: int) const
```

Returns the name of the script class defined in the file at index idx. If the file doesn't define a script class using the class_name syntax, this will return an empty string.

### get_file_type {#get_file_type}

```gdscript
StringName get_file_type(idx: int) const
```

Returns the resource type of the file at index idx. This returns a string such as "Resource" or "GDScript", not a file extension such as ".gd".

### get_name {#get_name}

```gdscript
String get_name()
```

Returns the name of this directory.

### get_parent {#get_parent}

```gdscript
EditorFileSystemDirectory get_parent()
```

Returns the parent directory for this directory or null if called on a directory at res:// or user://.

### get_path {#get_path}

```gdscript
String get_path() const
```

Returns the path to this directory.

### get_subdir {#get_subdir}

```gdscript
EditorFileSystemDirectory get_subdir(idx: int)
```

Returns the subdirectory at index idx.

### get_subdir_count {#get_subdir_count}

```gdscript
int get_subdir_count() const
```

Returns the number of subdirectories in this directory.
