# EditorFileSystemImportFormatSupportQuery

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorFileSystemImportFormatSupportQuery**

> This class is used to query and configure a certain import format. It is used in conjunction with asset format import plugins.

<details><summary>官方英文描述</summary>

This class is used to query and configure a certain import format. It is used in conjunction with asset format import plugins.

</details>

## 方法

### _get_file_extensions {#_get_file_extensions}

```gdscript
PackedStringArray _get_file_extensions() virtual const
```

Return the file extensions supported.

### _is_active {#_is_active}

```gdscript
bool _is_active() virtual const
```

Return whether this importer is active.

### _query {#_query}

```gdscript
bool _query() virtual const
```

Query support. Return false if import must not continue.
