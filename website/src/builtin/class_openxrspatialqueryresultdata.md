# OpenXRSpatialQueryResultData

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialQueryResultData**

> Object for storing the main query result data when calling OpenXRSpatialEntityExtension.query_snapshot(). This must always be the first component requested.

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the main query result data when calling OpenXRSpatialEntityExtension.query_snapshot(). This must always be the first component requested.

</details>

## 方法

### get_capacity {#get_capacity}

```gdscript
int get_capacity() const
```

Returns the number of entities that were retrieved.

### get_entity_id {#get_entity_id}

```gdscript
int get_entity_id(index: int) const
```

Returns the entity id (XrSpatialEntityIdEXT) for the entity at this index.

### get_entity_state {#get_entity_state}

```gdscript
OpenXRSpatialEntityTracker.EntityTrackingState get_entity_state(index: int) const
```

Returns the entity state for the entity at this index.
