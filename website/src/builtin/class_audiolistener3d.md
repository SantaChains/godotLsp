# AudioListener3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **AudioListener3D**

> Once added to the scene tree and enabled using make_current(), this node will override the location sounds are heard from. This can be used to listen from a location different from the Camera3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Once added to the scene tree and enabled using make_current(), this node will override the location sounds are heard from. This can be used to listen from a location different from the Camera3D.

</details>

## 方法

### clear_current {#clear_current}

```gdscript
void clear_current()
```

Disables the listener to use the current camera's listener instead.

### get_doppler_tracking {#get_doppler_tracking}

```gdscript
AudioListener3D.DopplerTracking get_doppler_tracking() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_listener_transform {#get_listener_transform}

```gdscript
Transform3D get_listener_transform() const
```

Returns the listener's global orthonormalized Transform3D.

### is_current {#is_current}

```gdscript
bool is_current() const
```

Returns true if the listener was made current using make_current(), false otherwise.

Note: There may be more than one AudioListener3D marked as "current" in the scene tree, but only the one that was made current last will be used.

### make_current {#make_current}

```gdscript
void make_current()
```

Enables the listener. This will override the current camera's listener.

### set_doppler_tracking {#set_doppler_tracking}

```gdscript
void set_doppler_tracking(mode: AudioListener3D.DopplerTracking)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### doppler_tracking {#doppler_tracking}

```gdscript
doppler_tracking: int
```

- void set_doppler_tracking(value: DopplerTracking)
- DopplerTracking get_doppler_tracking()

If not DOPPLER_TRACKING_DISABLED, this listener will simulate the Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] for objects changed in particular _process methods.

Note: The Doppler effect will only be heard on AudioStreamPlayer3Ds if AudioStreamPlayer3D.doppler_tracking is not set to AudioStreamPlayer3D.DOPPLER_TRACKING_DISABLED.

## 枚举

### DopplerTracking {#enum-DopplerTracking}

```gdscript
enum DopplerTracking { DOPPLER_TRACKING_DISABLED = 0, DOPPLER_TRACKING_IDLE_STEP = 1, DOPPLER_TRACKING_PHYSICS_STEP = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOPPLER_TRACKING_DISABLED {#DOPPLER_TRACKING_DISABLED}

```gdscript
DOPPLER_TRACKING_DISABLED = 0
```

Disables Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] simulation (default).

#### DOPPLER_TRACKING_IDLE_STEP {#DOPPLER_TRACKING_IDLE_STEP}

```gdscript
DOPPLER_TRACKING_IDLE_STEP = 1
```

Simulate Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] by tracking positions of objects that are changed in _process. Changes in the relative velocity of this listener compared to those objects affect how audio is perceived (changing the audio's AudioStreamPlayer3D.pitch_scale).

#### DOPPLER_TRACKING_PHYSICS_STEP {#DOPPLER_TRACKING_PHYSICS_STEP}

```gdscript
DOPPLER_TRACKING_PHYSICS_STEP = 2
```

Simulate Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] by tracking positions of objects that are changed in _physics_process. Changes in the relative velocity of this listener compared to those objects affect how audio is perceived (changing the audio's AudioStreamPlayer3D.pitch_scale).

---
