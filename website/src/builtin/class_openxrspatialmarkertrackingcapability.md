# OpenXRSpatialMarkerTrackingCapability

**继承**：[OpenXRExtensionWrapper](class_openxrextensionwrapper.md) → [Object](class_object.md) → **OpenXRSpatialMarkerTrackingCapability**

> This class handles the OpenXR marker tracking spatial entity extension.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class handles the OpenXR marker tracking spatial entity extension.

</details>

## 方法

### do_entity_update {#do_entity_update}

```gdscript
void do_entity_update(spatial_context: RID, component_data: typedarray::OpenXRSpatialComponentData, next_snapshot_create: OpenXRStructureBase = null, next_snapshot_query: OpenXRStructureBase = null)
```

Calls OpenXRSpatialEntityExtension.update_spatial_entities() and OpenXRSpatialEntityExtension.query_snapshot() with the marker entities associated with spatial_context.

component_data are the OpenXRSpatialComponentDatas to update for this marker capability.

If next_snapshot_create is non-null, then pass this to the next parameter in OpenXRSpatialEntityExtension.update_spatial_entities().

If next_snapshot_query is non-null, then pass this to the next parameter in OpenXRSpatialEntityExtension.query_snapshot().

### is_april_tag_supported {#is_april_tag_supported}

```gdscript
bool is_april_tag_supported()
```

Returns true if April tag marker tracking is supported by the current device.

### is_aruco_supported {#is_aruco_supported}

```gdscript
bool is_aruco_supported()
```

Returns true if Aruco marker tracking is supported by the current device.

### is_micro_qrcode_supported {#is_micro_qrcode_supported}

```gdscript
bool is_micro_qrcode_supported()
```

Returns true if micro QR code marker tracking is supported by the current device.

### is_qrcode_supported {#is_qrcode_supported}

```gdscript
bool is_qrcode_supported()
```

Returns true if QR code marker tracking is supported by the current device.

### start_entity_discovery {#start_entity_discovery}

```gdscript
OpenXRFutureResult start_entity_discovery(spatial_context: RID, component_data: typedarray::OpenXRSpatialComponentData, next_snapshot_create: OpenXRStructureBase = null, next_snapshot_query: OpenXRStructureBase = null, user_callback: Callable = Callable())
```

Calls OpenXRSpatialEntityExtension.discover_spatial_entities() and OpenXRSpatialEntityExtension.query_snapshot() with the marker entities associated with spatial_context.

component_data are the OpenXRSpatialComponentDatas to discover for this marker capability.

If next_snapshot_create is non-null, then pass this to the next parameter in OpenXRSpatialEntityExtension.discover_spatial_entities().

If next_snapshot_query is non-null, then pass this to the next parameter in OpenXRSpatialEntityExtension.query_snapshot().

user_callback, when non-null, is called with two parameters usually twice. The first parameter is the RID of the discovery snapshot and the second parameter is a boolean where false indicates the discovery snapshot is about to be processed, and true indicates the discovery snapshot has been processed and component_data has valid data. The second call is skipped if an error was encountered.

The returned OpenXRFutureResult is identical to the return from OpenXRSpatialEntityExtension.discover_spatial_entities().
