# Animation

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Animation**

## 方法

### add_marker {#add_marker}

```gdscript
void add_marker(name: StringName, time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### add_track {#add_track}

```gdscript
int add_track(type: Animation.TrackType, at_position: int = -1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### animation_track_get_key_animation {#animation_track_get_key_animation}

```gdscript
StringName animation_track_get_key_animation(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### animation_track_insert_key {#animation_track_insert_key}

```gdscript
int animation_track_insert_key(track_idx: int, time: float, animation: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### animation_track_set_key_animation {#animation_track_set_key_animation}

```gdscript
void animation_track_set_key_animation(track_idx: int, key_idx: int, animation: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_get_key_end_offset {#audio_track_get_key_end_offset}

```gdscript
float audio_track_get_key_end_offset(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_get_key_start_offset {#audio_track_get_key_start_offset}

```gdscript
float audio_track_get_key_start_offset(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_get_key_stream {#audio_track_get_key_stream}

```gdscript
Resource audio_track_get_key_stream(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_insert_key {#audio_track_insert_key}

```gdscript
int audio_track_insert_key(track_idx: int, time: float, stream: Resource, start_offset: float = 0, end_offset: float = 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_is_use_blend {#audio_track_is_use_blend}

```gdscript
bool audio_track_is_use_blend(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_set_key_end_offset {#audio_track_set_key_end_offset}

```gdscript
void audio_track_set_key_end_offset(track_idx: int, key_idx: int, offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_set_key_start_offset {#audio_track_set_key_start_offset}

```gdscript
void audio_track_set_key_start_offset(track_idx: int, key_idx: int, offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_set_key_stream {#audio_track_set_key_stream}

```gdscript
void audio_track_set_key_stream(track_idx: int, key_idx: int, stream: Resource)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### audio_track_set_use_blend {#audio_track_set_use_blend}

```gdscript
void audio_track_set_use_blend(track_idx: int, enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_get_key_in_handle {#bezier_track_get_key_in_handle}

```gdscript
Vector2 bezier_track_get_key_in_handle(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_get_key_out_handle {#bezier_track_get_key_out_handle}

```gdscript
Vector2 bezier_track_get_key_out_handle(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_get_key_value {#bezier_track_get_key_value}

```gdscript
float bezier_track_get_key_value(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_insert_key {#bezier_track_insert_key}

```gdscript
int bezier_track_insert_key(track_idx: int, time: float, value: float, in_handle: Vector2 = Vector2(0, 0), out_handle: Vector2 = Vector2(0, 0))
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_interpolate {#bezier_track_interpolate}

```gdscript
float bezier_track_interpolate(track_idx: int, time: float) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_set_key_in_handle {#bezier_track_set_key_in_handle}

```gdscript
void bezier_track_set_key_in_handle(track_idx: int, key_idx: int, in_handle: Vector2, balanced_value_time_ratio: float = 1.0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_set_key_out_handle {#bezier_track_set_key_out_handle}

```gdscript
void bezier_track_set_key_out_handle(track_idx: int, key_idx: int, out_handle: Vector2, balanced_value_time_ratio: float = 1.0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bezier_track_set_key_value {#bezier_track_set_key_value}

```gdscript
void bezier_track_set_key_value(track_idx: int, key_idx: int, value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### blend_shape_track_insert_key {#blend_shape_track_insert_key}

```gdscript
int blend_shape_track_insert_key(track_idx: int, time: float, amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### blend_shape_track_interpolate {#blend_shape_track_interpolate}

```gdscript
float blend_shape_track_interpolate(track_idx: int, time_sec: float, backward: bool = false) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### clear {#clear}

```gdscript
void clear()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### compress {#compress}

```gdscript
void compress(page_size: int = 8192, fps: int = 120, split_tolerance: float = 4.0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### copy_track {#copy_track}

```gdscript
void copy_track(track_idx: int, to_animation: Animation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### find_track {#find_track}

```gdscript
int find_track(path: NodePath, type: Animation.TrackType) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_length {#get_length}

```gdscript
float get_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_mode {#get_loop_mode}

```gdscript
Animation.LoopMode get_loop_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_marker_at_time {#get_marker_at_time}

```gdscript
StringName get_marker_at_time(time: float) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_marker_color {#get_marker_color}

```gdscript
Color get_marker_color(name: StringName) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_marker_names {#get_marker_names}

```gdscript
PackedStringArray get_marker_names() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_marker_time {#get_marker_time}

```gdscript
float get_marker_time(name: StringName) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_next_marker {#get_next_marker}

```gdscript
StringName get_next_marker(time: float) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_prev_marker {#get_prev_marker}

```gdscript
StringName get_prev_marker(time: float) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_step {#get_step}

```gdscript
float get_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_track_count {#get_track_count}

```gdscript
int get_track_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_marker {#has_marker}

```gdscript
bool has_marker(name: StringName) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_capture_included {#is_capture_included}

```gdscript
bool is_capture_included() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### method_track_get_name {#method_track_get_name}

```gdscript
StringName method_track_get_name(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### method_track_get_params {#method_track_get_params}

```gdscript
Array method_track_get_params(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### optimize {#optimize}

```gdscript
void optimize(allowed_velocity_err: float = 0.01, allowed_angular_err: float = 0.01, precision: int = 3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### position_track_insert_key {#position_track_insert_key}

```gdscript
int position_track_insert_key(track_idx: int, time: float, position: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### position_track_interpolate {#position_track_interpolate}

```gdscript
Vector3 position_track_interpolate(track_idx: int, time_sec: float, backward: bool = false) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_marker {#remove_marker}

```gdscript
void remove_marker(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_track {#remove_track}

```gdscript
void remove_track(track_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### rotation_track_insert_key {#rotation_track_insert_key}

```gdscript
int rotation_track_insert_key(track_idx: int, time: float, rotation: Quaternion)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### rotation_track_interpolate {#rotation_track_interpolate}

```gdscript
Quaternion rotation_track_interpolate(track_idx: int, time_sec: float, backward: bool = false) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scale_track_insert_key {#scale_track_insert_key}

```gdscript
int scale_track_insert_key(track_idx: int, time: float, scale: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scale_track_interpolate {#scale_track_interpolate}

```gdscript
Vector3 scale_track_interpolate(track_idx: int, time_sec: float, backward: bool = false) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_length {#set_length}

```gdscript
void set_length(time_sec: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop_mode {#set_loop_mode}

```gdscript
void set_loop_mode(loop_mode: Animation.LoopMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_marker_color {#set_marker_color}

```gdscript
void set_marker_color(name: StringName, color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_step {#set_step}

```gdscript
void set_step(size_sec: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_find_key {#track_find_key}

```gdscript
int track_find_key(track_idx: int, time: float, find_mode: Animation.FindMode = 0, limit: bool = false, backward: bool = false) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_interpolation_loop_wrap {#track_get_interpolation_loop_wrap}

```gdscript
bool track_get_interpolation_loop_wrap(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_interpolation_type {#track_get_interpolation_type}

```gdscript
Animation.InterpolationType track_get_interpolation_type(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_key_count {#track_get_key_count}

```gdscript
int track_get_key_count(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_key_time {#track_get_key_time}

```gdscript
float track_get_key_time(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_key_transition {#track_get_key_transition}

```gdscript
float track_get_key_transition(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_key_value {#track_get_key_value}

```gdscript
Variant track_get_key_value(track_idx: int, key_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_path {#track_get_path}

```gdscript
NodePath track_get_path(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_get_type {#track_get_type}

```gdscript
Animation.TrackType track_get_type(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_insert_key {#track_insert_key}

```gdscript
int track_insert_key(track_idx: int, time: float, key: Variant, transition: float = 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_is_compressed {#track_is_compressed}

```gdscript
bool track_is_compressed(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_is_enabled {#track_is_enabled}

```gdscript
bool track_is_enabled(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_is_imported {#track_is_imported}

```gdscript
bool track_is_imported(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_move_down {#track_move_down}

```gdscript
void track_move_down(track_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_move_to {#track_move_to}

```gdscript
void track_move_to(track_idx: int, to_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_move_up {#track_move_up}

```gdscript
void track_move_up(track_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_remove_key {#track_remove_key}

```gdscript
void track_remove_key(track_idx: int, key_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_remove_key_at_time {#track_remove_key_at_time}

```gdscript
void track_remove_key_at_time(track_idx: int, time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_enabled {#track_set_enabled}

```gdscript
void track_set_enabled(track_idx: int, enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_imported {#track_set_imported}

```gdscript
void track_set_imported(track_idx: int, imported: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_interpolation_loop_wrap {#track_set_interpolation_loop_wrap}

```gdscript
void track_set_interpolation_loop_wrap(track_idx: int, interpolation: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_interpolation_type {#track_set_interpolation_type}

```gdscript
void track_set_interpolation_type(track_idx: int, interpolation: Animation.InterpolationType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_key_time {#track_set_key_time}

```gdscript
void track_set_key_time(track_idx: int, key_idx: int, time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_key_transition {#track_set_key_transition}

```gdscript
void track_set_key_transition(track_idx: int, key_idx: int, transition: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_key_value {#track_set_key_value}

```gdscript
void track_set_key_value(track_idx: int, key: int, value: Variant)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_set_path {#track_set_path}

```gdscript
void track_set_path(track_idx: int, path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### track_swap {#track_swap}

```gdscript
void track_swap(track_idx: int, with_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### value_track_get_update_mode {#value_track_get_update_mode}

```gdscript
Animation.UpdateMode value_track_get_update_mode(track_idx: int) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### value_track_interpolate {#value_track_interpolate}

```gdscript
Variant value_track_interpolate(track_idx: int, time_sec: float, backward: bool = false) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### value_track_set_update_mode {#value_track_set_update_mode}

```gdscript
void value_track_set_update_mode(track_idx: int, mode: Animation.UpdateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### length {#length}

```gdscript
length: float
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### loop_mode {#loop_mode}

```gdscript
loop_mode: int
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### step {#step}

```gdscript
step: float
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### capture_included {#capture_included}

```gdscript
capture_included: bool
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### TrackType {#enum-TrackType}

```gdscript
enum TrackType { TYPE_VALUE = 0, TYPE_POSITION_3D = 1, TYPE_ROTATION_3D = 2, TYPE_SCALE_3D = 3, TYPE_BLEND_SHAPE = 4, TYPE_METHOD = 5, TYPE_BEZIER = 6, TYPE_AUDIO = 7, TYPE_ANIMATION = 8 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_VALUE {#TYPE_VALUE}

```gdscript
TYPE_VALUE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_POSITION_3D {#TYPE_POSITION_3D}

```gdscript
TYPE_POSITION_3D = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_ROTATION_3D {#TYPE_ROTATION_3D}

```gdscript
TYPE_ROTATION_3D = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_SCALE_3D {#TYPE_SCALE_3D}

```gdscript
TYPE_SCALE_3D = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_BLEND_SHAPE {#TYPE_BLEND_SHAPE}

```gdscript
TYPE_BLEND_SHAPE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_METHOD {#TYPE_METHOD}

```gdscript
TYPE_METHOD = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_BEZIER {#TYPE_BEZIER}

```gdscript
TYPE_BEZIER = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_AUDIO {#TYPE_AUDIO}

```gdscript
TYPE_AUDIO = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_ANIMATION {#TYPE_ANIMATION}

```gdscript
TYPE_ANIMATION = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### InterpolationType {#enum-InterpolationType}

```gdscript
enum InterpolationType { INTERPOLATION_NEAREST = 0, INTERPOLATION_LINEAR = 1, INTERPOLATION_CUBIC = 2, INTERPOLATION_LINEAR_ANGLE = 3, INTERPOLATION_CUBIC_ANGLE = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERPOLATION_NEAREST {#INTERPOLATION_NEAREST}

```gdscript
INTERPOLATION_NEAREST = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERPOLATION_LINEAR {#INTERPOLATION_LINEAR}

```gdscript
INTERPOLATION_LINEAR = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERPOLATION_CUBIC {#INTERPOLATION_CUBIC}

```gdscript
INTERPOLATION_CUBIC = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERPOLATION_LINEAR_ANGLE {#INTERPOLATION_LINEAR_ANGLE}

```gdscript
INTERPOLATION_LINEAR_ANGLE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERPOLATION_CUBIC_ANGLE {#INTERPOLATION_CUBIC_ANGLE}

```gdscript
INTERPOLATION_CUBIC_ANGLE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### UpdateMode {#enum-UpdateMode}

```gdscript
enum UpdateMode { UPDATE_CONTINUOUS = 0, UPDATE_DISCRETE = 1, UPDATE_CAPTURE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_CONTINUOUS {#UPDATE_CONTINUOUS}

```gdscript
UPDATE_CONTINUOUS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_DISCRETE {#UPDATE_DISCRETE}

```gdscript
UPDATE_DISCRETE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_CAPTURE {#UPDATE_CAPTURE}

```gdscript
UPDATE_CAPTURE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### LoopMode {#enum-LoopMode}

```gdscript
enum LoopMode { LOOP_NONE = 0, LOOP_LINEAR = 1, LOOP_PINGPONG = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOP_NONE {#LOOP_NONE}

```gdscript
LOOP_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOP_LINEAR {#LOOP_LINEAR}

```gdscript
LOOP_LINEAR = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOP_PINGPONG {#LOOP_PINGPONG}

```gdscript
LOOP_PINGPONG = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### LoopedFlag {#enum-LoopedFlag}

```gdscript
enum LoopedFlag { LOOPED_FLAG_NONE = 0, LOOPED_FLAG_END = 1, LOOPED_FLAG_START = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOPED_FLAG_NONE {#LOOPED_FLAG_NONE}

```gdscript
LOOPED_FLAG_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOPED_FLAG_END {#LOOPED_FLAG_END}

```gdscript
LOOPED_FLAG_END = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOPED_FLAG_START {#LOOPED_FLAG_START}

```gdscript
LOOPED_FLAG_START = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### FindMode {#enum-FindMode}

```gdscript
enum FindMode { FIND_MODE_NEAREST = 0, FIND_MODE_APPROX = 1, FIND_MODE_EXACT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FIND_MODE_NEAREST {#FIND_MODE_NEAREST}

```gdscript
FIND_MODE_NEAREST = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FIND_MODE_APPROX {#FIND_MODE_APPROX}

```gdscript
FIND_MODE_APPROX = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FIND_MODE_EXACT {#FIND_MODE_EXACT}

```gdscript
FIND_MODE_EXACT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
