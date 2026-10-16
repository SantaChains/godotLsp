# AudioStreamInteractive

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamInteractive**

> This is an audio stream that can playback music interactively, combining clips and a transition table. Clips must be added first, and then the transition rules via the add_transition(). Additionally, this stream exports a property parameter to control the playback via AudioStreamPlayer, AudioStreamPlayer2D, or AudioStreamPlayer3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

This is an audio stream that can playback music interactively, combining clips and a transition table. Clips must be added first, and then the transition rules via the add_transition(). Additionally, this stream exports a property parameter to control the playback via AudioStreamPlayer, AudioStreamPlayer2D, or AudioStreamPlayer3D.

The way this is used is by filling a number of clips, then configuring the transition table. From there, clips are selected for playback and the music will smoothly go from the current to the new one while using the corresponding transition rule defined in the transition table.

</details>

## 方法

### add_transition {#add_transition}

```gdscript
void add_transition(from_clip: int, to_clip: int, from_time: AudioStreamInteractive.TransitionFromTime, to_time: AudioStreamInteractive.TransitionToTime, fade_mode: AudioStreamInteractive.FadeMode, fade_beats: float, use_filler_clip: bool = false, filler_clip: int = -1, hold_previous: bool = false)
```

Add a transition between two clips. Provide the indices of the source and destination clips, or use the CLIP_ANY constant to indicate that transition happens to/from any clip to this one.

* from_time indicates the moment in the current clip the transition will begin after triggered.

* to_time indicates the time in the next clip that the playback will start from.

* fade_mode indicates how the fade will happen between clips. If unsure, just use FADE_AUTOMATIC which uses the most common type of fade for each situation.

* fade_beats indicates how many beats the fade will take. Using decimals is allowed.

* use_filler_clip indicates that there will be a filler clip used between the source and destination clips.

* filler_clip the index of the filler clip.

* If hold_previous is used, then this clip will be remembered. This can be used together with AUTO_ADVANCE_RETURN_TO_HOLD to return to this clip after another is done playing.

### erase_transition {#erase_transition}

```gdscript
void erase_transition(from_clip: int, to_clip: int)
```

Erase a transition by providing from_clip and to_clip clip indices. CLIP_ANY can be used for either argument or both.

### get_clip_auto_advance {#get_clip_auto_advance}

```gdscript
AudioStreamInteractive.AutoAdvanceMode get_clip_auto_advance(clip_index: int) const
```

Return whether a clip has auto-advance enabled. See set_clip_auto_advance().

### get_clip_auto_advance_next_clip {#get_clip_auto_advance_next_clip}

```gdscript
int get_clip_auto_advance_next_clip(clip_index: int) const
```

Return the clip towards which the clip referenced by clip_index will auto-advance to.

### get_clip_count {#get_clip_count}

```gdscript
int get_clip_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_clip_name {#get_clip_name}

```gdscript
StringName get_clip_name(clip_index: int) const
```

Return the name of a clip.

### get_clip_stream {#get_clip_stream}

```gdscript
AudioStream get_clip_stream(clip_index: int) const
```

Return the AudioStream associated with a clip.

### get_initial_clip {#get_initial_clip}

```gdscript
int get_initial_clip() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transition_fade_beats {#get_transition_fade_beats}

```gdscript
float get_transition_fade_beats(from_clip: int, to_clip: int) const
```

Return the time (in beats) for a transition (see add_transition()).

### get_transition_fade_mode {#get_transition_fade_mode}

```gdscript
AudioStreamInteractive.FadeMode get_transition_fade_mode(from_clip: int, to_clip: int) const
```

Return the mode for a transition (see add_transition()).

### get_transition_filler_clip {#get_transition_filler_clip}

```gdscript
int get_transition_filler_clip(from_clip: int, to_clip: int) const
```

Return the filler clip for a transition (see add_transition()).

### get_transition_from_time {#get_transition_from_time}

```gdscript
AudioStreamInteractive.TransitionFromTime get_transition_from_time(from_clip: int, to_clip: int) const
```

Return the source time position for a transition (see add_transition()).

### get_transition_list {#get_transition_list}

```gdscript
PackedInt32Array get_transition_list() const
```

Return the list of transitions (from, to interleaved).

### get_transition_to_time {#get_transition_to_time}

```gdscript
AudioStreamInteractive.TransitionToTime get_transition_to_time(from_clip: int, to_clip: int) const
```

Return the destination time position for a transition (see add_transition()).

### has_transition {#has_transition}

```gdscript
bool has_transition(from_clip: int, to_clip: int) const
```

Returns true if a given transition exists (was added via add_transition()).

### is_transition_holding_previous {#is_transition_holding_previous}

```gdscript
bool is_transition_holding_previous(from_clip: int, to_clip: int) const
```

Return whether a transition uses the hold previous functionality (see add_transition()).

### is_transition_using_filler_clip {#is_transition_using_filler_clip}

```gdscript
bool is_transition_using_filler_clip(from_clip: int, to_clip: int) const
```

Return whether a transition uses the filler clip functionality (see add_transition()).

### set_clip_auto_advance {#set_clip_auto_advance}

```gdscript
void set_clip_auto_advance(clip_index: int, mode: AudioStreamInteractive.AutoAdvanceMode)
```

Set whether a clip will auto-advance by changing the auto-advance mode.

### set_clip_auto_advance_next_clip {#set_clip_auto_advance_next_clip}

```gdscript
void set_clip_auto_advance_next_clip(clip_index: int, auto_advance_next_clip: int)
```

Set the index of the next clip towards which this clip will auto advance to when finished. If the clip being played loops, then auto-advance will be ignored.

### set_clip_count {#set_clip_count}

```gdscript
void set_clip_count(clip_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_clip_name {#set_clip_name}

```gdscript
void set_clip_name(clip_index: int, name: StringName)
```

Set the name of the current clip (for easier identification).

### set_clip_stream {#set_clip_stream}

```gdscript
void set_clip_stream(clip_index: int, stream: AudioStream)
```

Set the AudioStream associated with the current clip.

### set_initial_clip {#set_initial_clip}

```gdscript
void set_initial_clip(clip_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### clip_count {#clip_count}

```gdscript
clip_count: int
```

- void set_clip_count(value: int)
- int get_clip_count()

Amount of clips contained in this interactive player.

### initial_clip {#initial_clip}

```gdscript
initial_clip: int
```

- void set_initial_clip(value: int)
- int get_initial_clip()

Index of the initial clip, which will be played first when this stream is played.

## 枚举

### TransitionFromTime {#enum-TransitionFromTime}

```gdscript
enum TransitionFromTime { TRANSITION_FROM_TIME_IMMEDIATE = 0, TRANSITION_FROM_TIME_NEXT_BEAT = 1, TRANSITION_FROM_TIME_NEXT_BAR = 2, TRANSITION_FROM_TIME_END = 3 }
```

TransitionFromTime TRANSITION_FROM_TIME_IMMEDIATE = 0

Start transition as soon as possible, don't wait for any specific time position.

TransitionFromTime TRANSITION_FROM_TIME_NEXT_BEAT = 1

Transition when the clip playback position reaches the next beat.

TransitionFromTime TRANSITION_FROM_TIME_NEXT_BAR = 2

Transition when the clip playback position reaches the next bar.

TransitionFromTime TRANSITION_FROM_TIME_END = 3

Transition when the current clip finished playing.

#### TRANSITION_FROM_TIME_IMMEDIATE {#TRANSITION_FROM_TIME_IMMEDIATE}

```gdscript
TRANSITION_FROM_TIME_IMMEDIATE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSITION_FROM_TIME_NEXT_BEAT {#TRANSITION_FROM_TIME_NEXT_BEAT}

```gdscript
TRANSITION_FROM_TIME_NEXT_BEAT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSITION_FROM_TIME_NEXT_BAR {#TRANSITION_FROM_TIME_NEXT_BAR}

```gdscript
TRANSITION_FROM_TIME_NEXT_BAR = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSITION_FROM_TIME_END {#TRANSITION_FROM_TIME_END}

```gdscript
TRANSITION_FROM_TIME_END = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### TransitionToTime {#enum-TransitionToTime}

```gdscript
enum TransitionToTime { TRANSITION_TO_TIME_SAME_POSITION = 0, TRANSITION_TO_TIME_START = 1, TRANSITION_TO_TIME_PREVIOUS_POSITION = 2 }
```

TransitionToTime TRANSITION_TO_TIME_SAME_POSITION = 0

Transition to the same position in the destination clip. This is useful when both clips have exactly the same length and the music should fade between them.

TransitionToTime TRANSITION_TO_TIME_START = 1

Transition to the start of the destination clip.

TransitionToTime TRANSITION_TO_TIME_PREVIOUS_POSITION = 2

Transition to the last played position in the destination clip, if there was a previous transition from that clip. Otherwise, plays from the start of the destination clip.

#### TRANSITION_TO_TIME_SAME_POSITION {#TRANSITION_TO_TIME_SAME_POSITION}

```gdscript
TRANSITION_TO_TIME_SAME_POSITION = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSITION_TO_TIME_START {#TRANSITION_TO_TIME_START}

```gdscript
TRANSITION_TO_TIME_START = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSITION_TO_TIME_PREVIOUS_POSITION {#TRANSITION_TO_TIME_PREVIOUS_POSITION}

```gdscript
TRANSITION_TO_TIME_PREVIOUS_POSITION = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### FadeMode {#enum-FadeMode}

```gdscript
enum FadeMode { FADE_DISABLED = 0, FADE_IN = 1, FADE_OUT = 2, FADE_CROSS = 3, FADE_AUTOMATIC = 4 }
```

FadeMode FADE_DISABLED = 0

Do not use fade for the transition. This is useful when transitioning from a clip-end to clip-beginning, and each clip has their begin/end.

FadeMode FADE_IN = 1

Use a fade-in in the next clip, let the current clip finish.

FadeMode FADE_OUT = 2

Use a fade-out in the current clip, the next clip will start by itself.

FadeMode FADE_CROSS = 3

Use a cross-fade between clips.

FadeMode FADE_AUTOMATIC = 4

Use automatic fade logic depending on the transition from/to. It is recommended to use this by default.

#### FADE_DISABLED {#FADE_DISABLED}

```gdscript
FADE_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FADE_IN {#FADE_IN}

```gdscript
FADE_IN = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FADE_OUT {#FADE_OUT}

```gdscript
FADE_OUT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FADE_CROSS {#FADE_CROSS}

```gdscript
FADE_CROSS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FADE_AUTOMATIC {#FADE_AUTOMATIC}

```gdscript
FADE_AUTOMATIC = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### AutoAdvanceMode {#enum-AutoAdvanceMode}

```gdscript
enum AutoAdvanceMode { AUTO_ADVANCE_DISABLED = 0, AUTO_ADVANCE_ENABLED = 1, AUTO_ADVANCE_RETURN_TO_HOLD = 2 }
```

AutoAdvanceMode AUTO_ADVANCE_DISABLED = 0

Disable auto-advance (default).

AutoAdvanceMode AUTO_ADVANCE_ENABLED = 1

Enable auto-advance, a clip must be specified.

AutoAdvanceMode AUTO_ADVANCE_RETURN_TO_HOLD = 2

Enable auto-advance, but instead of specifying a clip, the playback will return to hold (see add_transition()).

#### AUTO_ADVANCE_DISABLED {#AUTO_ADVANCE_DISABLED}

```gdscript
AUTO_ADVANCE_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AUTO_ADVANCE_ENABLED {#AUTO_ADVANCE_ENABLED}

```gdscript
AUTO_ADVANCE_ENABLED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AUTO_ADVANCE_RETURN_TO_HOLD {#AUTO_ADVANCE_RETURN_TO_HOLD}

```gdscript
AUTO_ADVANCE_RETURN_TO_HOLD = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 常量

### CLIP_ANY {#CLIP_ANY}

```gdscript
const CLIP_ANY = -1
```

This constant describes that any clip is valid for a specific transition as either source or destination.
