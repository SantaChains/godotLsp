<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# Server 低层

> 渲染/文本/相机服务器的低层回调，普通游戏逻辑一般不直接连接。

## CameraServer（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal camera_feed_added(id: int)` | 新增相机采集源时触发，携带 id。 | 采集源注册。 [类页锚点](../builtin/class_cameraserver.md#camera_feed_added) |
| `signal camera_feed_removed(id: int)` | 移除相机采集源时触发，携带 id。 | 注销。 [类页锚点](../builtin/class_cameraserver.md#camera_feed_removed) |
| `signal camera_feeds_updated()` | 相机采集源列表更新时触发。 | 刷新列表。 [类页锚点](../builtin/class_cameraserver.md#camera_feeds_updated) |

## RenderingServer（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal frame_post_draw()` | 帧末、RenderingServer 更新完所有视口后触发。 | 帧末钩子（勿重绘）。 [类页锚点](../builtin/class_renderingserver.md#frame_post_draw) |
| `signal frame_pre_draw()` | 帧初、RenderingServer 更新视口前触发。 | 帧初钩子。 [类页锚点](../builtin/class_renderingserver.md#frame_pre_draw) |

## TextServerManager（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal interface_added(interface_name: StringName)` | 新增文字排版接口时触发，携带接口名。 | 接口注册。 [类页锚点](../builtin/class_textservermanager.md#interface_added) |
| `signal interface_removed(interface_name: StringName)` | 移除文字排版接口时触发，携带接口名。 | 接口注销。 [类页锚点](../builtin/class_textservermanager.md#interface_removed) |

<!-- SIGNALS:END -->
