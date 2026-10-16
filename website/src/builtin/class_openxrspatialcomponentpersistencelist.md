# OpenXRSpatialComponentPersistenceList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentPersistenceList**

> Object for storing the query persistence result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the query persistence result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_persistent_state {#get_persistent_state}

```gdscript
int get_persistent_state(index: int) const
```

Returns the persistent state (XrSpatialPersistenceStateEXT) for the entity at this index.

### get_persistent_uuid {#get_persistent_uuid}

```gdscript
String get_persistent_uuid(index: int) const
```

Returns the persistent uuid for the entity at this index.
