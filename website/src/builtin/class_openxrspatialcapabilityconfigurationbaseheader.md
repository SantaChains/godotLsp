# OpenXRSpatialCapabilityConfigurationBaseHeader

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialCapabilityConfigurationBaseHeader**

> Wrapper base class for OpenXR Spatial Capability Configuration headers. This class needs to be implemented for each capability configuration structure usable within OpenXR's spatial entities system.

<details><summary>官方英文描述</summary>

Wrapper base class for OpenXR Spatial Capability Configuration headers. This class needs to be implemented for each capability configuration structure usable within OpenXR's spatial entities system.

</details>

## 方法

### _get_configuration {#_get_configuration}

```gdscript
int _get_configuration() virtual
```

Return a pointer (encoded as an int64_t) to a struct holding the spatial capability configuration data. The memory for this struct should remain accessible as long as this object remains instantiated.

### _has_valid_configuration {#_has_valid_configuration}

```gdscript
bool _has_valid_configuration() virtual const
```

Return true if this object contains a valid configuration that can be retrieved when calling _get_configuration().

### get_configuration {#get_configuration}

```gdscript
int get_configuration()
```

Gets a pointer to the XrSpatialCapabilityConfigurationBaseHeaderEXT struct.

Note: This method is intended to be used from GDExtensions.

### has_valid_configuration {#has_valid_configuration}

```gdscript
bool has_valid_configuration() const
```

Returns true if this object contains a valid configuration that can be used when calling OpenXRSpatialEntityExtension.create_spatial_context().
