# OpenXRSpatialCapabilityConfigurationPlaneTracking

**继承**：[OpenXRSpatialCapabilityConfigurationBaseHeader](class_openxrspatialcapabilityconfigurationbaseheader.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialCapabilityConfigurationPlaneTracking**

> Configuration header for plane tracking. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context with plane tracking capabilities.

<details><summary>官方英文描述（中文精译进行中）</summary>

Configuration header for plane tracking. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context with plane tracking capabilities.

</details>

## 方法

### get_enabled_components {#get_enabled_components}

```gdscript
PackedInt64Array get_enabled_components() const
```

Returns the components enabled by this configuration.

Note: Only valid after this configuration was used to create a spatial context.

### supports_labels {#supports_labels}

```gdscript
bool supports_labels()
```

Returns true if we support the plane semantic label component (only valid after the OpenXR session has started). You can query these using the OpenXRSpatialComponentPlaneSemanticLabelList data object.

### supports_mesh_2d {#supports_mesh_2d}

```gdscript
bool supports_mesh_2d()
```

Returns true if we support the mesh 2D component (only valid after the OpenXR session has started). You can query these using the OpenXRSpatialComponentMesh2DList data object.

### supports_polygons {#supports_polygons}

```gdscript
bool supports_polygons()
```

Returns true if we support the polygon 2D component (only valid after the OpenXR session has started). You can query these using the OpenXRSpatialComponentPolygon2DList data object.
