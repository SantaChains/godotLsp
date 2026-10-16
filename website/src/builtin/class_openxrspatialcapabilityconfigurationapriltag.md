# OpenXRSpatialCapabilityConfigurationAprilTag

**继承**：[OpenXRSpatialCapabilityConfigurationBaseHeader](class_openxrspatialcapabilityconfigurationbaseheader.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialCapabilityConfigurationAprilTag**

> Configuration header for April tag markers. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context that can detect April tags.

<details><summary>官方英文描述（中文精译进行中）</summary>

Configuration header for April tag markers. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context that can detect April tags.

</details>

## 方法

### get_april_dict {#get_april_dict}

```gdscript
OpenXRSpatialCapabilityConfigurationAprilTag.AprilTagDict get_april_dict() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enabled_components {#get_enabled_components}

```gdscript
PackedInt64Array get_enabled_components() const
```

Returns the components enabled by this configuration.

Note: Only valid after this configuration was used to create a spatial context.

### set_april_dict {#set_april_dict}

```gdscript
void set_april_dict(april_dict: OpenXRSpatialCapabilityConfigurationAprilTag.AprilTagDict)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### april_dict {#april_dict}

```gdscript
april_dict: int
```

- void set_april_dict(value: AprilTagDict)
- AprilTagDict get_april_dict()

Dictionary to use to decode April tags.

Note: Must be set before using this configuration to create a spatial context.

## 枚举

### AprilTagDict {#enum-AprilTagDict}

```gdscript
enum AprilTagDict { APRIL_TAG_DICT_16H5 = 1, APRIL_TAG_DICT_25H9 = 2, APRIL_TAG_DICT_36H10 = 3, APRIL_TAG_DICT_36H11 = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### APRIL_TAG_DICT_16H5 {#APRIL_TAG_DICT_16H5}

```gdscript
APRIL_TAG_DICT_16H5 = 1
```

4 by 4 bits, minimum Hamming distance between any two codes = 5, 30 codes.

#### APRIL_TAG_DICT_25H9 {#APRIL_TAG_DICT_25H9}

```gdscript
APRIL_TAG_DICT_25H9 = 2
```

5 by 5 bits, minimum Hamming distance between any two codes = 9, 35 codes.

#### APRIL_TAG_DICT_36H10 {#APRIL_TAG_DICT_36H10}

```gdscript
APRIL_TAG_DICT_36H10 = 3
```

6 by 6 bits, minimum Hamming distance between any two codes = 10, 2320 codes.

#### APRIL_TAG_DICT_36H11 {#APRIL_TAG_DICT_36H11}

```gdscript
APRIL_TAG_DICT_36H11 = 4
```

6 by 6 bits, minimum Hamming distance between any two codes = 11, 587 codes.

---
