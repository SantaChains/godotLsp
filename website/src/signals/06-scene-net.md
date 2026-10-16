<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 场景树 / 导航 / 多人 / 网络

> SceneTree 帧与节点变动、寻路代理与区域、多人同步与连接、HTTP/WebRTC、系统与设置。

## DisplayServer（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal orientation_changed(orientation: int)` | 设备屏幕方向改变时触发，携带方向值。 | 横竖屏重排。 [类页锚点](../builtin/class_displayserver.md#orientation_changed) |

## GDExtensionManager（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal extension_loaded(extension: GDExtension)` | 编辑器加载完新扩展后触发，携带扩展。 | 加载后初始化。 [类页锚点](../builtin/class_gdextensionmanager.md#extension_loaded) |
| `signal extension_unloading(extension: GDExtension)` | 编辑器开始卸载扩展前触发，携带扩展。 | 卸载前清理。 [类页锚点](../builtin/class_gdextensionmanager.md#extension_unloading) |
| `signal extensions_reloaded()` | 重载一个或多个扩展完成后触发。 | 重载后刷新。 [类页锚点](../builtin/class_gdextensionmanager.md#extensions_reloaded) |

## HTTPRequest（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal request_completed(result: int, response_code: int, headers: PackedStringArray, body: PackedByteArray)` | HTTP 请求完成（成功或失败）时触发，携带结果码与数据。 | 发请求后在此收响应。 [类页锚点](../builtin/class_httprequest.md#request_completed) |

## JavaScriptBridge（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal pwa_update_available()` | 检测到 PWA 新版本待激活时触发。 | 提示刷新页面。 [类页锚点](../builtin/class_javascriptbridge.md#pwa_update_available) |

## MultiplayerAPI（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal connected_to_server()` | 作为客户端连上服务器时触发。 | 连接成功进游戏。 [类页锚点](../builtin/class_multiplayerapi.md#connected_to_server) |
| `signal connection_failed()` | 连接失败时触发。 | 失败提示重试。 [类页锚点](../builtin/class_multiplayerapi.md#connection_failed) |
| `signal peer_connected(id: int)` | 有新对端连接时触发，携带远端 id。 | 加入广播。 [类页锚点](../builtin/class_multiplayerapi.md#peer_connected) |
| `signal peer_disconnected(id: int)` | 对端断开时触发，携带远端 id。 | 离场清理。 [类页锚点](../builtin/class_multiplayerapi.md#peer_disconnected) |
| `signal server_disconnected()` | 与服务器断开时触发。 | 断线回主菜单。 [类页锚点](../builtin/class_multiplayerapi.md#server_disconnected) |

## MultiplayerPeer（继承 PacketPeer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal peer_connected(id: int)` | 远端对端连接时触发，携带 id。 | 加入广播。 [类页锚点](../builtin/class_multiplayerpeer.md#peer_connected) |
| `signal peer_disconnected(id: int)` | 远端对端断开时触发，携带 id。 | 离场清理。 [类页锚点](../builtin/class_multiplayerpeer.md#peer_disconnected) |

## MultiplayerSpawner（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal despawned(node: Node)` | 自动回收网络实体时触发。 | 离场清理。 [类页锚点](../builtin/class_multiplayerspawner.md#despawned) |
| `signal spawned(node: Node)` | 自动生成一个网络实体时触发，携带节点。 | 生成后初始化。 [类页锚点](../builtin/class_multiplayerspawner.md#spawned) |

## MultiplayerSynchronizer（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal delta_synchronized()` | 本同步器收到新的增量（delta）同步状态、属性已更新后触发。 | 增量同步落地钩子。 [类页锚点](../builtin/class_multiplayersynchronizer.md#delta_synchronized) |
| `signal synchronized()` | 一次状态同步完成时触发。 | 同步后刷新本地表现。 [类页锚点](../builtin/class_multiplayersynchronizer.md#synchronized) |
| `signal visibility_changed(for_peer: int)` | 某对端可见性经 update_visibility() 更新时触发，携带 for_peer。 | 可见性广播同步。 [类页锚点](../builtin/class_multiplayersynchronizer.md#visibility_changed) |

## NavigationAgent2D（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal link_reached(details: Dictionary)` | 代理到达导航链接时触发，携带详情字典。 | 过链钩子。 [类页锚点](../builtin/class_navigationagent2d.md#link_reached) |
| `signal navigation_finished()` | 2D 导航结束时触发。 | 导航生命周期收尾。 [类页锚点](../builtin/class_navigationagent2d.md#navigation_finished) |
| `signal path_changed()` | 2D 路径重算完成时触发。 | 拿到新路径。 [类页锚点](../builtin/class_navigationagent2d.md#path_changed) |
| `signal target_reached()` | 2D 代理到达目标点时触发。 | 到达后停止/换目标。 [类页锚点](../builtin/class_navigationagent2d.md#target_reached) |
| `signal velocity_computed(safe_velocity: Vector2)` | 避障速度计算完成时触发，携带安全速度。 | 取避障速度。 [类页锚点](../builtin/class_navigationagent2d.md#velocity_computed) |
| `signal waypoint_reached(details: Dictionary)` | 到达路径途经点时触发，携带详情。 | 逐点动作。 [类页锚点](../builtin/class_navigationagent2d.md#waypoint_reached) |

## NavigationAgent3D（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal link_reached(details: Dictionary)` | 3D 代理到达导航链接时触发。 | 过链钩子。 [类页锚点](../builtin/class_navigationagent3d.md#link_reached) |
| `signal navigation_finished()` | 3D 导航结束时触发。 | 收尾。 [类页锚点](../builtin/class_navigationagent3d.md#navigation_finished) |
| `signal path_changed()` | 3D 路径重算完成时触发。 | 新路径。 [类页锚点](../builtin/class_navigationagent3d.md#path_changed) |
| `signal target_reached()` | 3D 代理到达目标时触发。 | 到达停止。 [类页锚点](../builtin/class_navigationagent3d.md#target_reached) |
| `signal velocity_computed(safe_velocity: Vector3)` | 3D 避障速度计算完成时触发，携带安全速度。 | 取速度。 [类页锚点](../builtin/class_navigationagent3d.md#velocity_computed) |
| `signal waypoint_reached(details: Dictionary)` | 3D 到达途经点时触发。 | 逐点动作。 [类页锚点](../builtin/class_navigationagent3d.md#waypoint_reached) |

## NavigationRegion2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal bake_finished()` | 2D 导航多边形烘焙完成时触发。 | 烘焙后启用寻路。 [类页锚点](../builtin/class_navigationregion2d.md#bake_finished) |
| `signal navigation_polygon_changed()` | 导航多边形被替换或内部提交变更时触发。 | 重建区域。 [类页锚点](../builtin/class_navigationregion2d.md#navigation_polygon_changed) |

## NavigationRegion3D（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal bake_finished()` | 3D 导航网格烘焙完成时触发。 | 烘焙后启用寻路。 [类页锚点](../builtin/class_navigationregion3d.md#bake_finished) |
| `signal navigation_mesh_changed()` | 导航网格变更时通知。 | 重建区域。 [类页锚点](../builtin/class_navigationregion3d.md#navigation_mesh_changed) |

## NavigationServer2D（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal avoidance_debug_changed()` | 避障调试设置改变时触发。 | 调试刷新。 [类页锚点](../builtin/class_navigationserver2d.md#avoidance_debug_changed) |
| `signal map_changed(map: RID)` | 导航图更新（区域移动/修改）时触发，携带 RID。 | 图刷新。 [类页锚点](../builtin/class_navigationserver2d.md#map_changed) |
| `signal navigation_debug_changed()` | 导航调试设置改变时触发。 | 调试刷新。 [类页锚点](../builtin/class_navigationserver2d.md#navigation_debug_changed) |

## NavigationServer3D（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal avoidance_debug_changed()` | 3D 避障调试设置改变时触发。 | 调试刷新。 [类页锚点](../builtin/class_navigationserver3d.md#avoidance_debug_changed) |
| `signal map_changed(map: RID)` | 3D 导航图更新时触发，携带 RID。 | 图刷新。 [类页锚点](../builtin/class_navigationserver3d.md#map_changed) |
| `signal navigation_debug_changed()` | 3D 导航调试设置改变时触发。 | 调试刷新。 [类页锚点](../builtin/class_navigationserver3d.md#navigation_debug_changed) |

## ProjectSettings（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal settings_changed()` | 任一设置改变时触发（每帧至多一次）。 | 设置生效刷新。 [类页锚点](../builtin/class_projectsettings.md#settings_changed) |

## SceneMultiplayer（继承 MultiplayerAPI）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal peer_authenticating(id: int)` | 新对端连入且设置了有效 auth_callback 时触发；完成 complete_auth(id) 前不发 peer_connected，该对端只在 get_authenticating_peers() 中，仅收发认证数据。 | 在此发认证数据并判定后 complete_auth(id)。 [类页锚点](../builtin/class_scenemultiplayer.md#peer_authenticating) |
| `signal peer_authentication_failed(id: int)` | 对端在认证完成前断开（auth_timeout 到期或手动 disconnect_peer）时触发，替代 peer_disconnected，携带 id。 | 认证失败清理该对端。 [类页锚点](../builtin/class_scenemultiplayer.md#peer_authentication_failed) |
| `signal peer_packet(id: int, packet: PackedByteArray)` | 收到 send_bytes 发来的自定义数据包时触发，携带发送者 id 与字节。 | 低等级自定义报文收发。 [类页锚点](../builtin/class_scenemultiplayer.md#peer_packet) |

## SceneTree（继承 MainLoop）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal node_added(node: Node)` | 节点加入场景树时触发。 | 全局登记新节点。 [类页锚点](../builtin/class_scenetree.md#node_added) |
| `signal node_configuration_warning_changed(node: Node)` | 调用 update_configuration_warnings() 时触发，携带节点。 | 警告刷新。 [类页锚点](../builtin/class_scenetree.md#node_configuration_warning_changed) |
| `signal node_removed(node: Node)` | 节点离开场景树时触发。 | 全局清理。 [类页锚点](../builtin/class_scenetree.md#node_removed) |
| `signal node_renamed(node: Node)` | 节点改名时触发，携带节点。 | 全局名映射更新。 [类页锚点](../builtin/class_scenetree.md#node_renamed) |
| `signal physics_frame()` | 每物理步处理前触发。 | 全局物理帧钩子。 [类页锚点](../builtin/class_scenetree.md#physics_frame) |
| `signal process_frame()` | 每渲染帧处理前触发。 | 需要每帧钩子但不想写 _process 时用。 [类页锚点](../builtin/class_scenetree.md#process_frame) |
| `signal scene_changed()` | 当前主场景切换时触发。 | 切场景后初始化。 [类页锚点](../builtin/class_scenetree.md#scene_changed) |
| `signal tree_changed()` | 树结构变化（移动/改名等）时触发。 | 结构变更刷新。 [类页锚点](../builtin/class_scenetree.md#tree_changed) |
| `signal tree_process_mode_changed()` | 任一节点 process_mode 改变时触发。 | 处理模式同步。 [类页锚点](../builtin/class_scenetree.md#tree_process_mode_changed) |

## UndoRedo（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal version_changed()` | 调用 undo() 或 redo() 后触发。 | 撤销栈版本同步。 [类页锚点](../builtin/class_undoredo.md#version_changed) |

## WebRTCPeerConnection（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal data_channel_received(channel: WebRTCDataChannel)` | 收到带内（negotiated:false，默认）新建的数据通道时触发，携带通道对象；必须持有引用，否则自动关闭。 | 保存引用后收发数据。 [类页锚点](../builtin/class_webrtcpeerconnection.md#data_channel_received) |
| `signal ice_candidate_created(media: String, index: int, name: String)` | 生成新 ICE 候选时触发，三参数经信令服务器转发给对端。 | 把候选转发给远端。 [类页锚点](../builtin/class_webrtcpeerconnection.md#ice_candidate_created) |
| `signal session_description_created(type: String, sdp: String)` | create_offer/answer 后会话描述（type+sdp）就绪时触发，经信令转发对端。 | 发送 SDP 给远端。 [类页锚点](../builtin/class_webrtcpeerconnection.md#session_description_created) |

<!-- SIGNALS:END -->
