# CameraTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CameraTexture**

> This texture gives access to the camera texture provided by a CameraFeed.

<details><summary>官方英文描述（中文精译进行中）</summary>

This texture gives access to the camera texture provided by a CameraFeed.

Note: Many cameras supply YCbCr images which need to be converted in a shader.

</details>

## 方法

### get_camera_active {#get_camera_active}

```gdscript
bool get_camera_active() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_camera_feed_id {#get_camera_feed_id}

```gdscript
int get_camera_feed_id() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_which_feed {#get_which_feed}

```gdscript
CameraServer.FeedImage get_which_feed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_camera_active {#set_camera_active}

```gdscript
void set_camera_active(active: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_camera_feed_id {#set_camera_feed_id}

```gdscript
void set_camera_feed_id(feed_id: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_which_feed {#set_which_feed}

```gdscript
void set_which_feed(which_feed: CameraServer.FeedImage)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### camera_feed_id {#camera_feed_id}

```gdscript
camera_feed_id: int
```

- void set_camera_feed_id(value: int)
- int get_camera_feed_id()

The ID of the CameraFeed for which we want to display the image.

### which_feed {#which_feed}

```gdscript
which_feed: int
```

- void set_which_feed(value: FeedImage)
- FeedImage get_which_feed()

Which image within the CameraFeed we want access to, important if the camera image is split in a Y and CbCr component.

### camera_is_active {#camera_is_active}

```gdscript
camera_is_active: bool
```

- void set_camera_active(value: bool)
- bool get_camera_active()

Convenience property that gives access to the active property of the CameraFeed.
