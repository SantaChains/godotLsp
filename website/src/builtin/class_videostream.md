# VideoStream

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VideoStream**

> Base resource type for all video streams. Classes that derive from VideoStream can all be used as resource types to play back videos in VideoStreamPlayer.

<details><summary>官方英文描述</summary>

Base resource type for all video streams. Classes that derive from VideoStream can all be used as resource types to play back videos in VideoStreamPlayer.

</details>

## 方法

### _instantiate_playback {#_instantiate_playback}

```gdscript
VideoStreamPlayback _instantiate_playback() virtual
```

Called when the video starts playing, to initialize and return a subclass of VideoStreamPlayback.

### get_file {#get_file}

```gdscript
String get_file()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_file {#set_file}

```gdscript
void set_file(file: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### file {#file}

```gdscript
file: String
```

- void set_file(value: String)
- String get_file()

The video file path or URI that this VideoStream resource handles.

For VideoStreamTheora, this filename should be an Ogg Theora video file with the .ogv extension.
