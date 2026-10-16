# OpenXRAnchorTracker

**继承**：[OpenXRSpatialEntityTracker](class_openxrspatialentitytracker.md) → [XRPositionalTracker](class_xrpositionaltracker.md) → [XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRAnchorTracker**

> Positional tracker for our OpenXR spatial entity anchor extension, it tracks a user defined location in real space and maps it to our virtual space.

<details><summary>官方英文描述（中文精译进行中）</summary>

Positional tracker for our OpenXR spatial entity anchor extension, it tracks a user defined location in real space and maps it to our virtual space.

</details>

## 方法

### get_uuid {#get_uuid}

```gdscript
String get_uuid() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_uuid {#has_uuid}

```gdscript
bool has_uuid() const
```

Returns true if a non-zero UUID is set.

### set_uuid {#set_uuid}

```gdscript
void set_uuid(uuid: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### uuid {#uuid}

```gdscript
uuid: String
```

- void set_uuid(value: String)
- String get_uuid()

The UUID provided for persistent anchors.

## 信号

### uuid_changed {#uuid_changed}

```gdscript
signal uuid_changed()
```

Emitted when the UUID for this anchor was changed.
