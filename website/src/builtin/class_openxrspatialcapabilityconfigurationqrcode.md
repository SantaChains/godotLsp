# OpenXRSpatialCapabilityConfigurationQrCode

**继承**：[OpenXRSpatialCapabilityConfigurationBaseHeader](class_openxrspatialcapabilityconfigurationbaseheader.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialCapabilityConfigurationQrCode**

> Configuration header for micro QR code markers. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context that can detect micro QR code markers.

<details><summary>官方英文描述</summary>

Configuration header for micro QR code markers. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context that can detect micro QR code markers.

</details>

## 方法

### get_enabled_components {#get_enabled_components}

```gdscript
PackedInt64Array get_enabled_components() const
```

Returns the components enabled by this configuration.

Note: Only valid after this configuration was used to create a spatial context.
