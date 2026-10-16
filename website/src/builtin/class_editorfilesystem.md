# EditorFileSystem

**继承**：[Node](class_node.md) → [Object](class_object.md) → **EditorFileSystem**

> This object holds information of all resources in the filesystem, their types, etc.

<details><summary>官方英文描述</summary>

This object holds information of all resources in the filesystem, their types, etc.

Note: This class shouldn't be instantiated directly. Instead, access the singleton using EditorInterface.get_resource_filesystem().

</details>

## 方法

### get_file_type {#get_file_type}

```gdscript
String get_file_type(path: String) const
```

Returns the resource type of the file, given the full path. This returns a string such as "Resource" or "GDScript", not a file extension such as ".gd".

### get_filesystem {#get_filesystem}

```gdscript
EditorFileSystemDirectory get_filesystem()
```

Gets the root directory object.

### get_filesystem_path {#get_filesystem_path}

```gdscript
EditorFileSystemDirectory get_filesystem_path(path: String)
```

Returns a view into the filesystem at path.

### get_scanning_progress {#get_scanning_progress}

```gdscript
float get_scanning_progress() const
```

Returns the scan progress for 0 to 1 if the FS is being scanned.

### is_importing {#is_importing}

```gdscript
bool is_importing() const
```

Returns true if resources are currently being imported.

### is_scanning {#is_scanning}

```gdscript
bool is_scanning() const
```

Returns true if the filesystem is being scanned.

### reimport_files {#reimport_files}

```gdscript
void reimport_files(files: PackedStringArray)
```

Reimports a set of files. Call this if these files or their .import files were directly edited by script or an external program.

If the file type changed or the file was newly created, use update_file() or scan().

Note: This function blocks until the import is finished. However, the main loop iteration, including timers and Node._process(), will occur during the import process due to progress bar updates. Avoid calls to reimport_files() or scan() while an import is in progress.

### scan {#scan}

```gdscript
void scan()
```

Scan the filesystem for changes.

### scan_sources {#scan_sources}

```gdscript
void scan_sources()
```

Check if the source of any imported resource changed.

### update_file {#update_file}

```gdscript
void update_file(path: String)
```

Add a file in an existing directory, or schedule file information to be updated on editor restart. Can be used to update text files saved by an external program.

This will not import the file. To reimport, call reimport_files() or scan() methods.

## 信号

### filesystem_changed {#filesystem_changed}

```gdscript
signal filesystem_changed()
```

Emitted if the filesystem changed.

### script_classes_updated {#script_classes_updated}

```gdscript
signal script_classes_updated()
```

Emitted when the list of global script classes gets updated.

### sources_changed {#sources_changed}

```gdscript
signal sources_changed(exist: bool)
```

Emitted if the source of any imported file changed.

### resources_reimporting {#resources_reimporting}

```gdscript
signal resources_reimporting(resources: PackedStringArray)
```

Emitted before a resource is reimported.

### resources_reimported {#resources_reimported}

```gdscript
signal resources_reimported(resources: PackedStringArray)
```

Emitted if a resource is reimported.

### resources_reload {#resources_reload}

```gdscript
signal resources_reload(resources: PackedStringArray)
```

Emitted if at least one resource is reloaded when the filesystem is scanned.
