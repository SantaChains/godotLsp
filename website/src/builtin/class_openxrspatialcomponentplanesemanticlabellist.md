# OpenXRSpatialComponentPlaneSemanticLabelList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentPlaneSemanticLabelList**

> Object for storing the queries plane semantic label result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the queries plane semantic label result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_plane_semantic_label {#get_plane_semantic_label}

```gdscript
OpenXRSpatialComponentPlaneSemanticLabelList.PlaneSemanticLabel get_plane_semantic_label(index: int) const
```

Returns the plane semantic label for the parent entity at this index.

## 枚举

### PlaneSemanticLabel {#enum-PlaneSemanticLabel}

```gdscript
enum PlaneSemanticLabel { PLANE_SEMANTIC_LABEL_UNCATEGORIZED = 1, PLANE_SEMANTIC_LABEL_FLOOR = 2, PLANE_SEMANTIC_LABEL_WALL = 3, PLANE_SEMANTIC_LABEL_CEILING = 4, PLANE_SEMANTIC_LABEL_TABLE = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLANE_SEMANTIC_LABEL_UNCATEGORIZED {#PLANE_SEMANTIC_LABEL_UNCATEGORIZED}

```gdscript
PLANE_SEMANTIC_LABEL_UNCATEGORIZED = 1
```

Uncategorized plane.

#### PLANE_SEMANTIC_LABEL_FLOOR {#PLANE_SEMANTIC_LABEL_FLOOR}

```gdscript
PLANE_SEMANTIC_LABEL_FLOOR = 2
```

Plane represents a floor.

#### PLANE_SEMANTIC_LABEL_WALL {#PLANE_SEMANTIC_LABEL_WALL}

```gdscript
PLANE_SEMANTIC_LABEL_WALL = 3
```

Plane represents a wall.

#### PLANE_SEMANTIC_LABEL_CEILING {#PLANE_SEMANTIC_LABEL_CEILING}

```gdscript
PLANE_SEMANTIC_LABEL_CEILING = 4
```

Plane represents a ceiling.

#### PLANE_SEMANTIC_LABEL_TABLE {#PLANE_SEMANTIC_LABEL_TABLE}

```gdscript
PLANE_SEMANTIC_LABEL_TABLE = 5
```

Plane represents the surface of a table.

---
