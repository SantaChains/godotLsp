# OpenXRSpatialContextPersistenceConfig

**继承**：[OpenXRStructureBase](class_openxrstructurebase.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialContextPersistenceConfig**

> Configuration header for spatial persistence. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() as the next parameter to create a spatial context with spatial persistence capabilities.

<details><summary>官方英文描述</summary>

Configuration header for spatial persistence. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() as the next parameter to create a spatial context with spatial persistence capabilities.

</details>

## 方法

### add_persistence_context {#add_persistence_context}

```gdscript
void add_persistence_context(persistence_context: RID)
```

Adds a persistence context to this configuration. You must add at least one persistence context to create a valid configuration. You can create a persistence context by calling OpenXRSpatialAnchorCapability.create_persistence_context().

### get_persistence_contexts {#get_persistence_contexts}

```gdscript
Array get_persistence_contexts() const
```

Gets the persistence context(s) (as RIDs) received by add_persistence_context().

### remove_persistence_context {#remove_persistence_context}

```gdscript
void remove_persistence_context(persistence_context: RID)
```

Removes a persistence context.
