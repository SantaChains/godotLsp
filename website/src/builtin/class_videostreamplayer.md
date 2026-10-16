# VideoStreamPlayer

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **VideoStreamPlayer**

> A control used for playback of VideoStream resources.

<details><summary>官方英文描述</summary>

A control used for playback of VideoStream resources.

Supported video formats are Ogg Theora [https://www.theora.org/] (.ogv, VideoStreamTheora) and any format exposed via a GDExtension plugin.

Warning: On Web, video playback will perform poorly due to missing architecture-specific assembly optimizations.

</details>

## 方法

### get_audio_track {#get_audio_track}

```gdscript
int get_audio_track() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_buffering_msec {#get_buffering_msec}

```gdscript
int get_buffering_msec() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bus {#get_bus}

```gdscript
StringName get_bus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_speed_scale {#get_speed_scale}

```gdscript
float get_speed_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream {#get_stream}

```gdscript
VideoStream get_stream() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream_length {#get_stream_length}

```gdscript
float get_stream_length() const
```

The length of the current stream, in seconds.

### get_stream_name {#get_stream_name}

```gdscript
String get_stream_name() const
```

Returns the video stream's name, or "" if no video stream is assigned.

### get_stream_position {#get_stream_position}

```gdscript
float get_stream_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_video_texture {#get_video_texture}

```gdscript
Texture2D get_video_texture() const
```

Returns the current frame as a Texture2D.

### get_volume {#get_volume}

```gdscript
float get_volume() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_volume_db {#get_volume_db}

```gdscript
float get_volume_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_autoplay {#has_autoplay}

```gdscript
bool has_autoplay() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_expand {#has_expand}

```gdscript
bool has_expand() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_loop {#has_loop}

```gdscript
bool has_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_paused {#is_paused}

```gdscript
bool is_paused() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_playing {#is_playing}

```gdscript
bool is_playing() const
```

Returns true if the video is playing.

Note: The video is still considered playing if paused during playback.

### play {#play}

```gdscript
void play()
```

Starts the video playback from the beginning. If the video is paused, this will not unpause the video.

### set_audio_track {#set_audio_track}

```gdscript
void set_audio_track(track: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autoplay {#set_autoplay}

```gdscript
void set_autoplay(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_buffering_msec {#set_buffering_msec}

```gdscript
void set_buffering_msec(msec: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bus {#set_bus}

```gdscript
void set_bus(bus: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_expand {#set_expand}

```gdscript
void set_expand(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop {#set_loop}

```gdscript
void set_loop(loop: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_paused {#set_paused}

```gdscript
void set_paused(paused: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_speed_scale {#set_speed_scale}

```gdscript
void set_speed_scale(speed_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream {#set_stream}

```gdscript
void set_stream(stream: VideoStream)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream_position {#set_stream_position}

```gdscript
void set_stream_position(position: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_volume {#set_volume}

```gdscript
void set_volume(volume: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_volume_db {#set_volume_db}

```gdscript
void set_volume_db(db: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stop {#stop}

```gdscript
void stop()
```

Stops the video playback and sets the stream position to 0.

Note: Although the stream position will be set to 0, the first frame of the video stream won't become the current frame.

## 属性

### audio_track {#audio_track}

```gdscript
audio_track: int
```

- void set_audio_track(value: int)
- int get_audio_track()

The embedded audio track to play.

### stream {#stream}

```gdscript
stream: VideoStream
```

- void set_stream(value: VideoStream)
- VideoStream get_stream()

The assigned video stream. See description for supported formats.

### volume_db {#volume_db}

```gdscript
volume_db: float
```

- void set_volume_db(value: float)
- float get_volume_db()

Audio volume in dB.

### volume {#volume}

```gdscript
volume: float
```

- void set_volume(value: float)
- float get_volume()

Audio volume as a linear value.

### speed_scale {#speed_scale}

```gdscript
speed_scale: float
```

- void set_speed_scale(value: float)
- float get_speed_scale()

The stream's current speed scale. 1.0 is the normal speed, while 2.0 is double speed and 0.5 is half speed. A speed scale of 0.0 pauses the video, similar to setting paused to true.

### autoplay {#autoplay}

```gdscript
autoplay: bool
```

- void set_autoplay(value: bool)
- bool has_autoplay()

If true, playback starts when the scene loads.

### paused {#paused}

```gdscript
paused: bool
```

- void set_paused(value: bool)
- bool is_paused()

If true, the video is paused.

### expand {#expand}

```gdscript
expand: bool
```

- void set_expand(value: bool)
- bool has_expand()

If true, the video scales to the control size. Otherwise, the control minimum size will be automatically adjusted to match the video stream's dimensions.

### loop {#loop}

```gdscript
loop: bool
```

- void set_loop(value: bool)
- bool has_loop()

If true, the video restarts when it reaches its end.

### buffering_msec {#buffering_msec}

```gdscript
buffering_msec: int
```

- void set_buffering_msec(value: int)
- int get_buffering_msec()

Amount of time in milliseconds to store in buffer while playing.

### stream_position {#stream_position}

```gdscript
stream_position: float
```

- void set_stream_position(value: float)
- float get_stream_position()

The current position of the stream, in seconds.

### bus {#bus}

```gdscript
bus: StringName
```

- void set_bus(value: StringName)
- StringName get_bus()

Audio bus to use for sound playback.

## 信号

### finished {#finished}

```gdscript
signal finished()
```

Emitted when playback is finished.
