# OpenXRSpatialComponentData

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentData**

> Object for storing OpenXR spatial entity component data.

<details><summary>官方英文描述</summary>

Object for storing OpenXR spatial entity component data.

</details>

## 方法

### _get_component_type {#_get_component_type}

```gdscript
int _get_component_type() virtual const
```

Return the component type for the component we store data for.

### _get_structure_data {#_get_structure_data}

```gdscript
int _get_structure_data(next: int) virtual
```

Return a pointer to the structure data that will be submitted along with the snapshot query. This pointer must remain valid as long as this object is instantiated.

### _set_capacity {#_set_capacity}

```gdscript
void _set_capacity(capacity: int) virtual
```

Sets the expected capacity as provided by the spatial entities query system. Buffers should be initialized with the correct storage.

### get_component_type {#get_component_type}

```gdscript
int get_component_type() const
```

Gets this OpenXRSpatialComponentData's XrSpatialComponentTypeEXT.

### set_capacity {#set_capacity}

```gdscript
void set_capacity(capacity: int)
```

Sets the expected capacity as provided by the spatial entities query system. Buffers should be initialized with the correct storage.
