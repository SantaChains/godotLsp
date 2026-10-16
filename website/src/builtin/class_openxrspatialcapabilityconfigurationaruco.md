# OpenXRSpatialCapabilityConfigurationAruco

**继承**：[OpenXRSpatialCapabilityConfigurationBaseHeader](class_openxrspatialcapabilityconfigurationbaseheader.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialCapabilityConfigurationAruco**

> Configuration header for Aruco markers. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context that can detect Aruco markers.

<details><summary>官方英文描述</summary>

Configuration header for Aruco markers. Pass this to OpenXRSpatialEntityExtension.create_spatial_context() to create a spatial context that can detect Aruco markers.

</details>

## 方法

### get_aruco_dict {#get_aruco_dict}

```gdscript
OpenXRSpatialCapabilityConfigurationAruco.ArucoDict get_aruco_dict() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enabled_components {#get_enabled_components}

```gdscript
PackedInt64Array get_enabled_components() const
```

Returns the components enabled by this configuration.

Note: Only valid after this configuration was used to create a spatial context.

### set_aruco_dict {#set_aruco_dict}

```gdscript
void set_aruco_dict(aruco_dict: OpenXRSpatialCapabilityConfigurationAruco.ArucoDict)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### aruco_dict {#aruco_dict}

```gdscript
aruco_dict: int
```

- void set_aruco_dict(value: ArucoDict)
- ArucoDict get_aruco_dict()

Dictionary to use to decode Aruco markers.

Note: Must be set before using this configuration to create a spatial context.

## 枚举

### ArucoDict {#enum-ArucoDict}

```gdscript
enum ArucoDict { ARUCO_DICT_4X4_50 = 1, ARUCO_DICT_4X4_100 = 2, ARUCO_DICT_4X4_250 = 3, ARUCO_DICT_4X4_1000 = 4, ARUCO_DICT_5X5_50 = 5, ARUCO_DICT_5X5_100 = 6, ARUCO_DICT_5X5_250 = 7, ARUCO_DICT_5X5_1000 = 8, ARUCO_DICT_6X6_50 = 9, ARUCO_DICT_6X6_100 = 10, ARUCO_DICT_6X6_250 = 11, ARUCO_DICT_6X6_1000 = 12, ARUCO_DICT_7X7_50 = 13, ARUCO_DICT_7X7_100 = 14, ARUCO_DICT_7X7_250 = 15, ARUCO_DICT_7X7_1000 = 16 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARUCO_DICT_4X4_50 {#ARUCO_DICT_4X4_50}

```gdscript
ARUCO_DICT_4X4_50 = 1
```

4 by 4 pixel Aruco marker dictionary with 50 IDs.

#### ARUCO_DICT_4X4_100 {#ARUCO_DICT_4X4_100}

```gdscript
ARUCO_DICT_4X4_100 = 2
```

4 by 4 pixel Aruco marker dictionary with 100 IDs.

#### ARUCO_DICT_4X4_250 {#ARUCO_DICT_4X4_250}

```gdscript
ARUCO_DICT_4X4_250 = 3
```

4 by 4 pixel Aruco marker dictionary with 250 IDs.

#### ARUCO_DICT_4X4_1000 {#ARUCO_DICT_4X4_1000}

```gdscript
ARUCO_DICT_4X4_1000 = 4
```

4 by 4 pixel Aruco marker dictionary with 1000 IDs.

#### ARUCO_DICT_5X5_50 {#ARUCO_DICT_5X5_50}

```gdscript
ARUCO_DICT_5X5_50 = 5
```

5 by 5 pixel Aruco marker dictionary with 50 IDs.

#### ARUCO_DICT_5X5_100 {#ARUCO_DICT_5X5_100}

```gdscript
ARUCO_DICT_5X5_100 = 6
```

5 by 5 pixel Aruco marker dictionary with 100 IDs.

#### ARUCO_DICT_5X5_250 {#ARUCO_DICT_5X5_250}

```gdscript
ARUCO_DICT_5X5_250 = 7
```

5 by 5 pixel Aruco marker dictionary with 250 IDs.

#### ARUCO_DICT_5X5_1000 {#ARUCO_DICT_5X5_1000}

```gdscript
ARUCO_DICT_5X5_1000 = 8
```

5 by 5 pixel Aruco marker dictionary with 1000 IDs.

#### ARUCO_DICT_6X6_50 {#ARUCO_DICT_6X6_50}

```gdscript
ARUCO_DICT_6X6_50 = 9
```

6 by 6 pixel Aruco marker dictionary with 50 IDs.

#### ARUCO_DICT_6X6_100 {#ARUCO_DICT_6X6_100}

```gdscript
ARUCO_DICT_6X6_100 = 10
```

6 by 6 pixel Aruco marker dictionary with 100 IDs.

#### ARUCO_DICT_6X6_250 {#ARUCO_DICT_6X6_250}

```gdscript
ARUCO_DICT_6X6_250 = 11
```

6 by 6 pixel Aruco marker dictionary with 250 IDs.

#### ARUCO_DICT_6X6_1000 {#ARUCO_DICT_6X6_1000}

```gdscript
ARUCO_DICT_6X6_1000 = 12
```

6 by 6 pixel Aruco marker dictionary with 1000 IDs.

#### ARUCO_DICT_7X7_50 {#ARUCO_DICT_7X7_50}

```gdscript
ARUCO_DICT_7X7_50 = 13
```

7 by 7 pixel Aruco marker dictionary with 50 IDs.

#### ARUCO_DICT_7X7_100 {#ARUCO_DICT_7X7_100}

```gdscript
ARUCO_DICT_7X7_100 = 14
```

7 by 7 pixel Aruco marker dictionary with 100 IDs.

#### ARUCO_DICT_7X7_250 {#ARUCO_DICT_7X7_250}

```gdscript
ARUCO_DICT_7X7_250 = 15
```

7 by 7 pixel Aruco marker dictionary with 250 IDs.

#### ARUCO_DICT_7X7_1000 {#ARUCO_DICT_7X7_1000}

```gdscript
ARUCO_DICT_7X7_1000 = 16
```

7 by 7 pixel Aruco marker dictionary with 1000 IDs.

---
