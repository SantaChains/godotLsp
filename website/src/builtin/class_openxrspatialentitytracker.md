# OpenXRSpatialEntityTracker

**继承**：[XRPositionalTracker](class_xrpositionaltracker.md) → [XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialEntityTracker**

> These are trackers created and managed by OpenXR's spatial entity extensions that give access to specific data related to OpenXR's spatial entities. They will always be of type TRACKER_ANCHOR.

<details><summary>官方英文描述（中文精译进行中）</summary>

These are trackers created and managed by OpenXR's spatial entity extensions that give access to specific data related to OpenXR's spatial entities. They will always be of type TRACKER_ANCHOR.

</details>

## 方法

### add_next {#add_next}

```gdscript
void add_next(next: OpenXRStructureBase)
```

Adds a new OpenXRStructureBase to the next-chain.

get_next() will return this next until either add_next() is called again or it's removed in remove_next().

### get_entity {#get_entity}

```gdscript
RID get_entity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_next {#get_next}

```gdscript
OpenXRStructureBase get_next() const
```

Gets the head OpenXRStructureBase in the next-chain.

See also add_next() and remove_next().

### get_spatial_context {#get_spatial_context}

```gdscript
RID get_spatial_context() const
```

Gets the spatial context used to create this OpenXRSpatialEntityTracker.

### get_spatial_tracking_state {#get_spatial_tracking_state}

```gdscript
OpenXRSpatialEntityTracker.EntityTrackingState get_spatial_tracking_state() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_next {#remove_next}

```gdscript
void remove_next(next: OpenXRStructureBase)
```

Removes a next object previously added in add_next() from the next-chain.

### set_entity {#set_entity}

```gdscript
void set_entity(entity: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spatial_context {#set_spatial_context}

```gdscript
void set_spatial_context(spatial_context: RID)
```

Sets the spatial context used to create this tracker.

### set_spatial_tracking_state {#set_spatial_tracking_state}

```gdscript
void set_spatial_tracking_state(spatial_tracking_state: OpenXRSpatialEntityTracker.EntityTrackingState)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### entity {#entity}

```gdscript
entity: RID
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### spatial_tracking_state {#spatial_tracking_state}

```gdscript
spatial_tracking_state: int
```

- void set_spatial_tracking_state(value: EntityTrackingState)
- EntityTrackingState get_spatial_tracking_state()

The spatial tracking state for this tracker.

## 信号

### next_changed {#next_changed}

```gdscript
signal next_changed()
```

Emitted when the next-chain changes, from either add_next() or remove_next().

### spatial_tracking_state_changed {#spatial_tracking_state_changed}

```gdscript
signal spatial_tracking_state_changed(spatial_tracking_state: int)
```

There is currently no description for this signal. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

## 枚举

### EntityTrackingState {#enum-EntityTrackingState}

```gdscript
enum EntityTrackingState { ENTITY_TRACKING_STATE_STOPPED = 1, ENTITY_TRACKING_STATE_PAUSED = 2, ENTITY_TRACKING_STATE_TRACKING = 3 }
```

EntityTrackingState ENTITY_TRACKING_STATE_STOPPED = 1

This anchor has stopped tracking.

EntityTrackingState ENTITY_TRACKING_STATE_PAUSED = 2

Tracking is currently paused.

EntityTrackingState ENTITY_TRACKING_STATE_TRACKING = 3

This anchor is currently being tracked.

#### ENTITY_TRACKING_STATE_STOPPED {#ENTITY_TRACKING_STATE_STOPPED}

```gdscript
ENTITY_TRACKING_STATE_STOPPED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENTITY_TRACKING_STATE_PAUSED {#ENTITY_TRACKING_STATE_PAUSED}

```gdscript
ENTITY_TRACKING_STATE_PAUSED = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENTITY_TRACKING_STATE_TRACKING {#ENTITY_TRACKING_STATE_TRACKING}

```gdscript
ENTITY_TRACKING_STATE_TRACKING = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
