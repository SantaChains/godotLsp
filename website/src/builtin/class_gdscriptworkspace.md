# GDScriptWorkspace

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **GDScriptWorkspace**

> Provides language server functionality related to the workspace.

<details><summary>官方英文描述（中文精译进行中）</summary>

Provides language server functionality related to the workspace.

</details>

## 方法

### apply_new_signal {#apply_new_signal}

```gdscript
void apply_new_signal(obj: Object, function: String, args: PackedStringArray)
```

Deprecated: Might result in unwanted side effects for connected clients.

### didDeleteFiles {#didDeleteFiles}

```gdscript
void didDeleteFiles(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### generate_script_api {#generate_script_api}

```gdscript
Dictionary generate_script_api(path: String)
```

Returns the interface of the script in a machine-readable format.

### get_file_path {#get_file_path}

```gdscript
String get_file_path(uri: String)
```

Converts a URI to a file path.

### get_file_uri {#get_file_uri}

```gdscript
String get_file_uri(path: String) const
```

Converts a file path to a URI.

### parse_local_script {#parse_local_script}

```gdscript
Error parse_local_script(path: String)
```

Deprecated: Might result in unwanted side effects for connected clients.

### parse_script {#parse_script}

```gdscript
Error parse_script(path: String, content: String)
```

Deprecated: Might result in unwanted side effects for connected clients.

### publish_diagnostics {#publish_diagnostics}

```gdscript
void publish_diagnostics(path: String)
```

Deprecated: Might result in unwanted side effects for connected clients.
