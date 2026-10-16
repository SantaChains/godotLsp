# CameraServer

**继承**：[Object](class_object.md) → **CameraServer**

> The CameraServer keeps track of different cameras accessible in Godot. These are external cameras such as webcams or the cameras on your phone.

<details><summary>官方英文描述（中文精译进行中）</summary>

The CameraServer keeps track of different cameras accessible in Godot. These are external cameras such as webcams or the cameras on your phone.

It is notably used to provide AR modules with a video feed from the camera.

Note: This class is currently only implemented on Linux, Android, macOS, and iOS. On other platforms no CameraFeeds will be available. To get a CameraFeed on iOS, enable EditorExportPlatformIOS.modules/camera.

</details>

## 方法

### add_feed {#add_feed}

```gdscript
void add_feed(feed: CameraFeed)
```

Adds the camera feed to the camera server.

### feeds {#feeds}

```gdscript
typedarray::CameraFeed feeds()
```

Returns an array of CameraFeeds.

### get_feed {#get_feed}

```gdscript
CameraFeed get_feed(index: int)
```

Returns the CameraFeed corresponding to the camera with the given index.

### get_feed_count {#get_feed_count}

```gdscript
int get_feed_count()
```

Returns the number of CameraFeeds registered.

### is_monitoring_feeds {#is_monitoring_feeds}

```gdscript
bool is_monitoring_feeds() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_feed {#remove_feed}

```gdscript
void remove_feed(feed: CameraFeed)
```

Removes the specified camera feed.

### set_monitoring_feeds {#set_monitoring_feeds}

```gdscript
void set_monitoring_feeds(is_monitoring_feeds: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### monitoring_feeds {#monitoring_feeds}

```gdscript
monitoring_feeds: bool
```

- void set_monitoring_feeds(value: bool)
- bool is_monitoring_feeds()

If true, the server is actively monitoring available camera feeds.

This has a performance cost, so only set it to true when you're actively accessing the camera.

Note: After setting it to true, you can receive updated camera feeds through the camera_feeds_updated signal.

```
func _ready():
    CameraServer.camera_feeds_updated.connect(_on_camera_feeds_updated)
    CameraServer.monitoring_feeds = true
func _on_camera_feeds_updated():
    var feeds = CameraServer.feeds()
```

```
public override void _Ready()
{
    CameraServer.CameraFeedsUpdated += OnCameraFeedsUpdated;
    CameraServer.MonitoringFeeds = true;
}
void OnCameraFeedsUpdated()
{
    var feeds = CameraServer.Feeds();
}
```

## 信号

### camera_feed_added {#camera_feed_added}

```gdscript
signal camera_feed_added(id: int)
```

Emitted when a CameraFeed is added (e.g. a webcam is plugged in).

### camera_feed_removed {#camera_feed_removed}

```gdscript
signal camera_feed_removed(id: int)
```

Emitted when a CameraFeed is removed (e.g. a webcam is unplugged).

### camera_feeds_updated {#camera_feeds_updated}

```gdscript
signal camera_feeds_updated()
```

Emitted when camera feeds are updated.

## 枚举

### FeedImage {#enum-FeedImage}

```gdscript
enum FeedImage { FEED_RGBA_IMAGE = 0, FEED_YCBCR_IMAGE = 0, FEED_Y_IMAGE = 0, FEED_CBCR_IMAGE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEED_RGBA_IMAGE {#FEED_RGBA_IMAGE}

```gdscript
FEED_RGBA_IMAGE = 0
```

The RGBA camera image.

#### FEED_YCBCR_IMAGE {#FEED_YCBCR_IMAGE}

```gdscript
FEED_YCBCR_IMAGE = 0
```

The YCbCr [https://en.wikipedia.org/wiki/YCbCr] camera image.

#### FEED_Y_IMAGE {#FEED_Y_IMAGE}

```gdscript
FEED_Y_IMAGE = 0
```

The Y component camera image.

#### FEED_CBCR_IMAGE {#FEED_CBCR_IMAGE}

```gdscript
FEED_CBCR_IMAGE = 1
```

The CbCr component camera image.

---
