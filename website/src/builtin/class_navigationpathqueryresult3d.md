# NavigationPathQueryResult3D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **NavigationPathQueryResult3D**

> This class stores the result of a 3D navigation path query from the NavigationServer3D.

<details><summary>官方英文描述</summary>

This class stores the result of a 3D navigation path query from the NavigationServer3D.

</details>

## 方法

### get_path {#get_path}

```gdscript
PackedVector3Array get_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_length {#get_path_length}

```gdscript
float get_path_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_owner_ids {#get_path_owner_ids}

```gdscript
PackedInt64Array get_path_owner_ids() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_rids {#get_path_rids}

```gdscript
typedarray::RID get_path_rids() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_types {#get_path_types}

```gdscript
PackedInt32Array get_path_types() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### reset {#reset}

```gdscript
void reset()
```

Reset the result object to its initial state. This is useful to reuse the object across multiple queries.

### set_path {#set_path}

```gdscript
void set_path(path: PackedVector3Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_length {#set_path_length}

```gdscript
void set_path_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_owner_ids {#set_path_owner_ids}

```gdscript
void set_path_owner_ids(path_owner_ids: PackedInt64Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_rids {#set_path_rids}

```gdscript
void set_path_rids(path_rids: typedarray::RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_types {#set_path_types}

```gdscript
void set_path_types(path_types: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### path {#path}

```gdscript
path: PackedVector3Array
```

- void set_path(value: PackedVector3Array)
- PackedVector3Array get_path()

The resulting path array from the navigation query. All path array positions are in global coordinates. Without customized query parameters this is the same path as returned by NavigationServer3D.map_get_path().

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.

### path_types {#path_types}

```gdscript
path_types: PackedInt32Array
```

- void set_path_types(value: PackedInt32Array)
- PackedInt32Array get_path_types()

The type of navigation primitive (region or link) that each point of the path goes through.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### path_rids {#path_rids}

```gdscript
path_rids: typedarray::RID
```

- void set_path_rids(value: Array[RID])
- Array[RID] get_path_rids()

The RIDs of the regions and links that each point of the path goes through.

### path_owner_ids {#path_owner_ids}

```gdscript
path_owner_ids: PackedInt64Array
```

- void set_path_owner_ids(value: PackedInt64Array)
- PackedInt64Array get_path_owner_ids()

The ObjectIDs of the Objects which manage the regions and links each point of the path goes through.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt64Array for more details.

### path_length {#path_length}

```gdscript
path_length: float
```

- void set_path_length(value: float)
- float get_path_length()

Returns the length of the path.

## 枚举

### PathSegmentType {#enum-PathSegmentType}

```gdscript
enum PathSegmentType { PATH_SEGMENT_TYPE_REGION = 0, PATH_SEGMENT_TYPE_LINK = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PATH_SEGMENT_TYPE_REGION {#PATH_SEGMENT_TYPE_REGION}

```gdscript
PATH_SEGMENT_TYPE_REGION = 0
```

This segment of the path goes through a region.

#### PATH_SEGMENT_TYPE_LINK {#PATH_SEGMENT_TYPE_LINK}

```gdscript
PATH_SEGMENT_TYPE_LINK = 1
```

This segment of the path goes through a link.

---
