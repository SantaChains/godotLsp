# CameraFeed

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **CameraFeed**

> A camera feed gives you access to a single physical camera attached to your device. When enabled, Godot will start capturing frames from the camera which can then be used. See also CameraServer.

<details><summary>官方英文描述</summary>

A camera feed gives you access to a single physical camera attached to your device. When enabled, Godot will start capturing frames from the camera which can then be used. See also CameraServer.

Note: Many cameras will return YCbCr images which are split into two textures and need to be combined in a shader. Godot does this automatically for you if you set the environment to show the camera image in the background.

Note: This class is currently only implemented on Linux, Android, macOS, and iOS. On other platforms no CameraFeeds will be available. To get a CameraFeed on iOS, enable EditorExportPlatformIOS.modules/camera.

</details>

## 方法

### _activate_feed {#_activate_feed}

```gdscript
bool _activate_feed() virtual
```

Called when the camera feed is activated.

### _deactivate_feed {#_deactivate_feed}

```gdscript
void _deactivate_feed() virtual
```

Called when the camera feed is deactivated.

### _get_formats {#_get_formats}

```gdscript
Array _get_formats() virtual const
```

Override this method to define supported formats of the camera feed.

### _set_format {#_set_format}

```gdscript
bool _set_format(index: int, parameters: Dictionary) virtual
```

Override this method to set the format of the camera feed.

### get_datatype {#get_datatype}

```gdscript
CameraFeed.FeedDataType get_datatype() const
```

Returns feed image data type.

### get_formats {#get_formats}

```gdscript
Array get_formats() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_id {#get_id}

```gdscript
int get_id() const
```

Returns the unique ID for this feed.

### get_name {#get_name}

```gdscript
String get_name() const
```

Returns the camera's name.

### get_position {#get_position}

```gdscript
CameraFeed.FeedPosition get_position() const
```

Returns the position of camera on the device.

### get_texture_tex_id {#get_texture_tex_id}

```gdscript
int get_texture_tex_id(feed_image_type: CameraServer.FeedImage)
```

Returns the texture backend ID (usable by some external libraries that need a handle to a texture to write data).

### get_transform {#get_transform}

```gdscript
Transform2D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_active {#is_active}

```gdscript
bool is_active() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_active {#set_active}

```gdscript
void set_active(active: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_external {#set_external}

```gdscript
void set_external(width: int, height: int)
```

Sets the feed as external feed provided by another library.

### set_format {#set_format}

```gdscript
bool set_format(index: int, parameters: Dictionary)
```

Sets the feed format parameters for the given index in the formats array. Returns true on success. By default, the YUYV encoded stream is transformed to FEED_RGB. The YUYV encoded stream output format can be changed by setting parameters's output entry to one of the following:

- "separate" will result in FEED_YCBCR_SEP;
- "grayscale" will result in desaturated FEED_RGB;
- "copy" will result in FEED_YCBCR.

### set_name {#set_name}

```gdscript
void set_name(name: String)
```

Sets the camera's name.

### set_position {#set_position}

```gdscript
void set_position(position: CameraFeed.FeedPosition)
```

Sets the position of this camera.

### set_rgb_image {#set_rgb_image}

```gdscript
void set_rgb_image(rgb_image: Image)
```

Sets RGB image for this feed.

### set_transform {#set_transform}

```gdscript
void set_transform(transform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ycbcr_image {#set_ycbcr_image}

```gdscript
void set_ycbcr_image(ycbcr_image: Image)
```

Sets YCbCr image for this feed.

### set_ycbcr_images {#set_ycbcr_images}

```gdscript
void set_ycbcr_images(y_image: Image, cbcr_image: Image)
```

Sets Y and CbCr images for this feed.

## 属性

### feed_is_active {#feed_is_active}

```gdscript
feed_is_active: bool
```

- void set_active(value: bool)
- bool is_active()

If true, the feed is active.

### feed_transform {#feed_transform}

```gdscript
feed_transform: Transform2D
```

- void set_transform(value: Transform2D)
- Transform2D get_transform()

The transform applied to the camera's image.

### formats {#formats}

```gdscript
formats: Array
```

- Array get_formats()

Formats supported by the feed. Each entry is a Dictionary describing format parameters.

## 信号

### frame_changed {#frame_changed}

```gdscript
signal frame_changed()
```

Emitted when a new frame is available.

### format_changed {#format_changed}

```gdscript
signal format_changed()
```

Emitted when the format has changed.

## 枚举

### FeedDataType {#enum-FeedDataType}

```gdscript
enum FeedDataType { FEED_NOIMAGE = 0, FEED_RGB = 1, FEED_YCBCR = 2, FEED_YCBCR_SEP = 3, FEED_EXTERNAL = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEED_NOIMAGE {#FEED_NOIMAGE}

```gdscript
FEED_NOIMAGE = 0
```

No image set for the feed.

#### FEED_RGB {#FEED_RGB}

```gdscript
FEED_RGB = 1
```

Feed supplies RGB images.

#### FEED_YCBCR {#FEED_YCBCR}

```gdscript
FEED_YCBCR = 2
```

Feed supplies YCbCr images that need to be converted to RGB.

#### FEED_YCBCR_SEP {#FEED_YCBCR_SEP}

```gdscript
FEED_YCBCR_SEP = 3
```

Feed supplies separate Y and CbCr images that need to be combined and converted to RGB.

#### FEED_EXTERNAL {#FEED_EXTERNAL}

```gdscript
FEED_EXTERNAL = 4
```

Feed supplies external image.

---

### FeedPosition {#enum-FeedPosition}

```gdscript
enum FeedPosition { FEED_UNSPECIFIED = 0, FEED_FRONT = 1, FEED_BACK = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEED_UNSPECIFIED {#FEED_UNSPECIFIED}

```gdscript
FEED_UNSPECIFIED = 0
```

Unspecified position.

#### FEED_FRONT {#FEED_FRONT}

```gdscript
FEED_FRONT = 1
```

Camera is mounted at the front of the device.

#### FEED_BACK {#FEED_BACK}

```gdscript
FEED_BACK = 2
```

Camera is mounted at the back of the device.

---
