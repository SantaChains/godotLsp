# PathFollow3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **PathFollow3D**

> This node takes its parent Path3D, and returns the coordinates of a point within it, given a distance from the first vertex.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node takes its parent Path3D, and returns the coordinates of a point within it, given a distance from the first vertex.

It is useful for making other nodes follow a path, without coding the movement pattern. For that, the nodes must be children of this node. The descendant nodes will then move accordingly when setting the progress in this node.

</details>

## 方法

### correct_posture {#correct_posture}

```gdscript
static Transform3D correct_posture(transform: Transform3D, rotation_mode: PathFollow3D.RotationMode)
```

Correct the transform. rotation_mode implicitly specifies how posture (forward, up and sideway direction) is calculated.

### get_cubic_interpolation {#get_cubic_interpolation}

```gdscript
bool get_cubic_interpolation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_offset {#get_h_offset}

```gdscript
float get_h_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_progress {#get_progress}

```gdscript
float get_progress() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_progress_ratio {#get_progress_ratio}

```gdscript
float get_progress_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_mode {#get_rotation_mode}

```gdscript
PathFollow3D.RotationMode get_rotation_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_offset {#get_v_offset}

```gdscript
float get_v_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_loop {#has_loop}

```gdscript
bool has_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_tilt_enabled {#is_tilt_enabled}

```gdscript
bool is_tilt_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_model_front {#is_using_model_front}

```gdscript
bool is_using_model_front() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cubic_interpolation {#set_cubic_interpolation}

```gdscript
void set_cubic_interpolation(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_h_offset {#set_h_offset}

```gdscript
void set_h_offset(h_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop {#set_loop}

```gdscript
void set_loop(loop: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_progress {#set_progress}

```gdscript
void set_progress(progress: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_progress_ratio {#set_progress_ratio}

```gdscript
void set_progress_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_mode {#set_rotation_mode}

```gdscript
void set_rotation_mode(rotation_mode: PathFollow3D.RotationMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tilt_enabled {#set_tilt_enabled}

```gdscript
void set_tilt_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_model_front {#set_use_model_front}

```gdscript
void set_use_model_front(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_offset {#set_v_offset}

```gdscript
void set_v_offset(v_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### progress {#progress}

```gdscript
progress: float
```

- void set_progress(value: float)
- float get_progress()

The distance from the first vertex, measured in 3D units along the path. Changing this value sets this node's position to a point within the path.

### progress_ratio {#progress_ratio}

```gdscript
progress_ratio: float
```

- void set_progress_ratio(value: float)
- float get_progress_ratio()

The distance from the first vertex, considering 0.0 as the first vertex and 1.0 as the last. This is just another way of expressing the progress within the path, as the progress supplied is multiplied internally by the path's length.

It can be set or get only if the PathFollow3D is the child of a Path3D which is part of the scene tree, and that this Path3D has a Curve3D with a non-zero length. Otherwise, trying to set this field will print an error, and getting this field will return 0.0.

### h_offset {#h_offset}

```gdscript
h_offset: float
```

- void set_h_offset(value: float)
- float get_h_offset()

The node's offset along the curve.

### v_offset {#v_offset}

```gdscript
v_offset: float
```

- void set_v_offset(value: float)
- float get_v_offset()

The node's offset perpendicular to the curve.

### rotation_mode {#rotation_mode}

```gdscript
rotation_mode: int
```

- void set_rotation_mode(value: RotationMode)
- RotationMode get_rotation_mode()

Allows or forbids rotation on one or more axes, depending on the RotationMode constants being used.

### use_model_front {#use_model_front}

```gdscript
use_model_front: bool
```

- void set_use_model_front(value: bool)
- bool is_using_model_front()

If true, the node moves on the travel path with orienting the +Z axis as forward. See also Vector3.FORWARD and Vector3.MODEL_FRONT.

### cubic_interp {#cubic_interp}

```gdscript
cubic_interp: bool
```

- void set_cubic_interpolation(value: bool)
- bool get_cubic_interpolation()

If true, the position between two cached points is interpolated cubically, and linearly otherwise.

The points along the Curve3D of the Path3D are precomputed before use, for faster calculations. The point at the requested offset is then calculated interpolating between two adjacent cached points. This may present a problem if the curve makes sharp turns, as the cached points may not follow the curve closely enough.

There are two answers to this problem: either increase the number of cached points and increase memory consumption, or make a cubic interpolation between two points at the cost of (slightly) slower calculations.

### loop {#loop}

```gdscript
loop: bool
```

- void set_loop(value: bool)
- bool has_loop()

If true, any offset outside the path's length will wrap around, instead of stopping at the ends. Use it for cyclic paths.

### tilt_enabled {#tilt_enabled}

```gdscript
tilt_enabled: bool
```

- void set_tilt_enabled(value: bool)
- bool is_tilt_enabled()

If true, the tilt property of Curve3D takes effect.

## 枚举

### RotationMode {#enum-RotationMode}

```gdscript
enum RotationMode { ROTATION_NONE = 0, ROTATION_Y = 1, ROTATION_XY = 2, ROTATION_XYZ = 3, ROTATION_ORIENTED = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_NONE {#ROTATION_NONE}

```gdscript
ROTATION_NONE = 0
```

Forbids the PathFollow3D to rotate.

#### ROTATION_Y {#ROTATION_Y}

```gdscript
ROTATION_Y = 1
```

Allows the PathFollow3D to rotate in the Y axis only.

#### ROTATION_XY {#ROTATION_XY}

```gdscript
ROTATION_XY = 2
```

Allows the PathFollow3D to rotate in both the X, and Y axes.

#### ROTATION_XYZ {#ROTATION_XYZ}

```gdscript
ROTATION_XYZ = 3
```

Allows the PathFollow3D to rotate in any axis.

#### ROTATION_ORIENTED {#ROTATION_ORIENTED}

```gdscript
ROTATION_ORIENTED = 4
```

Uses the up vector information in a Curve3D to enforce orientation. This rotation mode requires the Path3D's Curve3D.up_vector_enabled property to be set to true.

---
