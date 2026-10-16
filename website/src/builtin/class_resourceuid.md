# ResourceUID

**继承**：[Object](class_object.md) → **ResourceUID**

> Resource UIDs (Unique IDentifiers) allow the engine to keep references between resources intact, even if files are renamed or moved. They can be accessed with uid://.

<details><summary>官方英文描述（中文精译进行中）</summary>

Resource UIDs (Unique IDentifiers) allow the engine to keep references between resources intact, even if files are renamed or moved. They can be accessed with uid://.

ResourceUID keeps track of all registered resource UIDs in a project, generates new UIDs, and converts between their string and integer representations.

</details>

## 方法

### add_id {#add_id}

```gdscript
void add_id(id: int, path: String)
```

Adds a new UID value which is mapped to the given resource path.

Fails with an error if the UID already exists, so be sure to check has_id() beforehand, or use set_id() instead.

### create_id {#create_id}

```gdscript
int create_id()
```

Generates a random resource UID which is guaranteed to be unique within the list of currently loaded UIDs.

In order for this UID to be registered, you must call add_id() or set_id().

### create_id_for_path {#create_id_for_path}

```gdscript
int create_id_for_path(path: String)
```

Like create_id(), but the UID is seeded with the provided path and project name. UIDs generated for that path will be always the same within the current project.

### ensure_path {#ensure_path}

```gdscript
static String ensure_path(path_or_uid: String)
```

Returns a path, converting path_or_uid if necessary. Fails and returns an empty string if an invalid UID is provided.

### get_id_path {#get_id_path}

```gdscript
String get_id_path(id: int) const
```

Returns the path that the given UID value refers to.

Fails with an error if the UID does not exist, so be sure to check has_id() beforehand.

### has_id {#has_id}

```gdscript
bool has_id(id: int) const
```

Returns whether the given UID value is known to the cache.

### id_to_text {#id_to_text}

```gdscript
String id_to_text(id: int) const
```

Converts the given UID to a uid:// string value.

### path_to_uid {#path_to_uid}

```gdscript
static String path_to_uid(path: String)
```

Converts the provided resource path to a UID. Returns the unchanged path if it has no associated UID.

### remove_id {#remove_id}

```gdscript
void remove_id(id: int)
```

Removes a loaded UID value from the cache.

Fails with an error if the UID does not exist, so be sure to check has_id() beforehand.

### set_id {#set_id}

```gdscript
void set_id(id: int, path: String)
```

Updates the resource path of an existing UID.

Fails with an error if the UID does not exist, so be sure to check has_id() beforehand, or use add_id() instead.

### text_to_id {#text_to_id}

```gdscript
int text_to_id(text_id: String) const
```

Extracts the UID value from the given uid:// string.

### uid_to_path {#uid_to_path}

```gdscript
static String uid_to_path(uid: String)
```

Converts the provided uid to a path. Prints an error if the UID is invalid.

## 常量

### INVALID_ID {#INVALID_ID}

```gdscript
const INVALID_ID = -1
```

The value to use for an invalid UID, for example if the resource could not be loaded.

Its text representation is uid://.
