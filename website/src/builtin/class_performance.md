# Performance

**继承**：[Object](class_object.md) → **Performance**

## 方法

### add_custom_monitor {#add_custom_monitor}

```gdscript
void add_custom_monitor(id: StringName, callable: Callable, arguments: Array = [], type: Performance.MonitorType = 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_monitor {#get_custom_monitor}

```gdscript
Variant get_custom_monitor(id: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_monitor_names {#get_custom_monitor_names}

```gdscript
typedarray::StringName get_custom_monitor_names()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_monitor_types {#get_custom_monitor_types}

```gdscript
PackedInt32Array get_custom_monitor_types()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_monitor {#get_monitor}

```gdscript
float get_monitor(monitor: Performance.Monitor) const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_monitor_modification_time {#get_monitor_modification_time}

```gdscript
int get_monitor_modification_time()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_custom_monitor {#has_custom_monitor}

```gdscript
bool has_custom_monitor(id: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_custom_monitor {#remove_custom_monitor}

```gdscript
void remove_custom_monitor(id: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### Monitor {#enum-Monitor}

```gdscript
enum Monitor { TIME_FPS = 0, TIME_PROCESS = 1, TIME_PHYSICS_PROCESS = 2, TIME_NAVIGATION_PROCESS = 3, MEMORY_STATIC = 4, MEMORY_STATIC_MAX = 5, MEMORY_MESSAGE_BUFFER_MAX = 6, OBJECT_COUNT = 7, OBJECT_RESOURCE_COUNT = 8, OBJECT_NODE_COUNT = 9, OBJECT_ORPHAN_NODE_COUNT = 10, RENDER_TOTAL_OBJECTS_IN_FRAME = 11, RENDER_TOTAL_PRIMITIVES_IN_FRAME = 12, RENDER_TOTAL_DRAW_CALLS_IN_FRAME = 13, RENDER_VIDEO_MEM_USED = 14, RENDER_TEXTURE_MEM_USED = 15, RENDER_BUFFER_MEM_USED = 16, PHYSICS_2D_ACTIVE_OBJECTS = 17, PHYSICS_2D_COLLISION_PAIRS = 18, PHYSICS_2D_ISLAND_COUNT = 19, PHYSICS_3D_ACTIVE_OBJECTS = 20, PHYSICS_3D_COLLISION_PAIRS = 21, PHYSICS_3D_ISLAND_COUNT = 22, AUDIO_OUTPUT_LATENCY = 23, NAVIGATION_ACTIVE_MAPS = 24, NAVIGATION_REGION_COUNT = 25, NAVIGATION_AGENT_COUNT = 26, NAVIGATION_LINK_COUNT = 27, NAVIGATION_POLYGON_COUNT = 28, NAVIGATION_EDGE_COUNT = 29, NAVIGATION_EDGE_MERGE_COUNT = 30, NAVIGATION_EDGE_CONNECTION_COUNT = 31, NAVIGATION_EDGE_FREE_COUNT = 32, NAVIGATION_OBSTACLE_COUNT = 33, PIPELINE_COMPILATIONS_CANVAS = 34, PIPELINE_COMPILATIONS_MESH = 35, PIPELINE_COMPILATIONS_SURFACE = 36, PIPELINE_COMPILATIONS_DRAW = 37, PIPELINE_COMPILATIONS_SPECIALIZATION = 38, NAVIGATION_2D_ACTIVE_MAPS = 39, NAVIGATION_2D_REGION_COUNT = 40, NAVIGATION_2D_AGENT_COUNT = 41, NAVIGATION_2D_LINK_COUNT = 42, NAVIGATION_2D_POLYGON_COUNT = 43, NAVIGATION_2D_EDGE_COUNT = 44, NAVIGATION_2D_EDGE_MERGE_COUNT = 45, NAVIGATION_2D_EDGE_CONNECTION_COUNT = 46, NAVIGATION_2D_EDGE_FREE_COUNT = 47, NAVIGATION_2D_OBSTACLE_COUNT = 48, NAVIGATION_3D_ACTIVE_MAPS = 49, NAVIGATION_3D_REGION_COUNT = 50, NAVIGATION_3D_AGENT_COUNT = 51, NAVIGATION_3D_LINK_COUNT = 52, NAVIGATION_3D_POLYGON_COUNT = 53, NAVIGATION_3D_EDGE_COUNT = 54, NAVIGATION_3D_EDGE_MERGE_COUNT = 55, NAVIGATION_3D_EDGE_CONNECTION_COUNT = 56, NAVIGATION_3D_EDGE_FREE_COUNT = 57, NAVIGATION_3D_OBSTACLE_COUNT = 58, MONITOR_MAX = 59 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TIME_FPS {#TIME_FPS}

```gdscript
TIME_FPS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TIME_PROCESS {#TIME_PROCESS}

```gdscript
TIME_PROCESS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TIME_PHYSICS_PROCESS {#TIME_PHYSICS_PROCESS}

```gdscript
TIME_PHYSICS_PROCESS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TIME_NAVIGATION_PROCESS {#TIME_NAVIGATION_PROCESS}

```gdscript
TIME_NAVIGATION_PROCESS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MEMORY_STATIC {#MEMORY_STATIC}

```gdscript
MEMORY_STATIC = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MEMORY_STATIC_MAX {#MEMORY_STATIC_MAX}

```gdscript
MEMORY_STATIC_MAX = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MEMORY_MESSAGE_BUFFER_MAX {#MEMORY_MESSAGE_BUFFER_MAX}

```gdscript
MEMORY_MESSAGE_BUFFER_MAX = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OBJECT_COUNT {#OBJECT_COUNT}

```gdscript
OBJECT_COUNT = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OBJECT_RESOURCE_COUNT {#OBJECT_RESOURCE_COUNT}

```gdscript
OBJECT_RESOURCE_COUNT = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OBJECT_NODE_COUNT {#OBJECT_NODE_COUNT}

```gdscript
OBJECT_NODE_COUNT = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OBJECT_ORPHAN_NODE_COUNT {#OBJECT_ORPHAN_NODE_COUNT}

```gdscript
OBJECT_ORPHAN_NODE_COUNT = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_TOTAL_OBJECTS_IN_FRAME {#RENDER_TOTAL_OBJECTS_IN_FRAME}

```gdscript
RENDER_TOTAL_OBJECTS_IN_FRAME = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_TOTAL_PRIMITIVES_IN_FRAME {#RENDER_TOTAL_PRIMITIVES_IN_FRAME}

```gdscript
RENDER_TOTAL_PRIMITIVES_IN_FRAME = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_TOTAL_DRAW_CALLS_IN_FRAME {#RENDER_TOTAL_DRAW_CALLS_IN_FRAME}

```gdscript
RENDER_TOTAL_DRAW_CALLS_IN_FRAME = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_VIDEO_MEM_USED {#RENDER_VIDEO_MEM_USED}

```gdscript
RENDER_VIDEO_MEM_USED = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_TEXTURE_MEM_USED {#RENDER_TEXTURE_MEM_USED}

```gdscript
RENDER_TEXTURE_MEM_USED = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_BUFFER_MEM_USED {#RENDER_BUFFER_MEM_USED}

```gdscript
RENDER_BUFFER_MEM_USED = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_2D_ACTIVE_OBJECTS {#PHYSICS_2D_ACTIVE_OBJECTS}

```gdscript
PHYSICS_2D_ACTIVE_OBJECTS = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_2D_COLLISION_PAIRS {#PHYSICS_2D_COLLISION_PAIRS}

```gdscript
PHYSICS_2D_COLLISION_PAIRS = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_2D_ISLAND_COUNT {#PHYSICS_2D_ISLAND_COUNT}

```gdscript
PHYSICS_2D_ISLAND_COUNT = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_3D_ACTIVE_OBJECTS {#PHYSICS_3D_ACTIVE_OBJECTS}

```gdscript
PHYSICS_3D_ACTIVE_OBJECTS = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_3D_COLLISION_PAIRS {#PHYSICS_3D_COLLISION_PAIRS}

```gdscript
PHYSICS_3D_COLLISION_PAIRS = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_3D_ISLAND_COUNT {#PHYSICS_3D_ISLAND_COUNT}

```gdscript
PHYSICS_3D_ISLAND_COUNT = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AUDIO_OUTPUT_LATENCY {#AUDIO_OUTPUT_LATENCY}

```gdscript
AUDIO_OUTPUT_LATENCY = 23
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_ACTIVE_MAPS {#NAVIGATION_ACTIVE_MAPS}

```gdscript
NAVIGATION_ACTIVE_MAPS = 24
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_REGION_COUNT {#NAVIGATION_REGION_COUNT}

```gdscript
NAVIGATION_REGION_COUNT = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_AGENT_COUNT {#NAVIGATION_AGENT_COUNT}

```gdscript
NAVIGATION_AGENT_COUNT = 26
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_LINK_COUNT {#NAVIGATION_LINK_COUNT}

```gdscript
NAVIGATION_LINK_COUNT = 27
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_POLYGON_COUNT {#NAVIGATION_POLYGON_COUNT}

```gdscript
NAVIGATION_POLYGON_COUNT = 28
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_EDGE_COUNT {#NAVIGATION_EDGE_COUNT}

```gdscript
NAVIGATION_EDGE_COUNT = 29
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_EDGE_MERGE_COUNT {#NAVIGATION_EDGE_MERGE_COUNT}

```gdscript
NAVIGATION_EDGE_MERGE_COUNT = 30
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_EDGE_CONNECTION_COUNT {#NAVIGATION_EDGE_CONNECTION_COUNT}

```gdscript
NAVIGATION_EDGE_CONNECTION_COUNT = 31
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_EDGE_FREE_COUNT {#NAVIGATION_EDGE_FREE_COUNT}

```gdscript
NAVIGATION_EDGE_FREE_COUNT = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_OBSTACLE_COUNT {#NAVIGATION_OBSTACLE_COUNT}

```gdscript
NAVIGATION_OBSTACLE_COUNT = 33
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PIPELINE_COMPILATIONS_CANVAS {#PIPELINE_COMPILATIONS_CANVAS}

```gdscript
PIPELINE_COMPILATIONS_CANVAS = 34
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PIPELINE_COMPILATIONS_MESH {#PIPELINE_COMPILATIONS_MESH}

```gdscript
PIPELINE_COMPILATIONS_MESH = 35
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PIPELINE_COMPILATIONS_SURFACE {#PIPELINE_COMPILATIONS_SURFACE}

```gdscript
PIPELINE_COMPILATIONS_SURFACE = 36
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PIPELINE_COMPILATIONS_DRAW {#PIPELINE_COMPILATIONS_DRAW}

```gdscript
PIPELINE_COMPILATIONS_DRAW = 37
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PIPELINE_COMPILATIONS_SPECIALIZATION {#PIPELINE_COMPILATIONS_SPECIALIZATION}

```gdscript
PIPELINE_COMPILATIONS_SPECIALIZATION = 38
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_ACTIVE_MAPS {#NAVIGATION_2D_ACTIVE_MAPS}

```gdscript
NAVIGATION_2D_ACTIVE_MAPS = 39
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_REGION_COUNT {#NAVIGATION_2D_REGION_COUNT}

```gdscript
NAVIGATION_2D_REGION_COUNT = 40
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_AGENT_COUNT {#NAVIGATION_2D_AGENT_COUNT}

```gdscript
NAVIGATION_2D_AGENT_COUNT = 41
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_LINK_COUNT {#NAVIGATION_2D_LINK_COUNT}

```gdscript
NAVIGATION_2D_LINK_COUNT = 42
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_POLYGON_COUNT {#NAVIGATION_2D_POLYGON_COUNT}

```gdscript
NAVIGATION_2D_POLYGON_COUNT = 43
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_EDGE_COUNT {#NAVIGATION_2D_EDGE_COUNT}

```gdscript
NAVIGATION_2D_EDGE_COUNT = 44
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_EDGE_MERGE_COUNT {#NAVIGATION_2D_EDGE_MERGE_COUNT}

```gdscript
NAVIGATION_2D_EDGE_MERGE_COUNT = 45
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_EDGE_CONNECTION_COUNT {#NAVIGATION_2D_EDGE_CONNECTION_COUNT}

```gdscript
NAVIGATION_2D_EDGE_CONNECTION_COUNT = 46
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_EDGE_FREE_COUNT {#NAVIGATION_2D_EDGE_FREE_COUNT}

```gdscript
NAVIGATION_2D_EDGE_FREE_COUNT = 47
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_2D_OBSTACLE_COUNT {#NAVIGATION_2D_OBSTACLE_COUNT}

```gdscript
NAVIGATION_2D_OBSTACLE_COUNT = 48
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_ACTIVE_MAPS {#NAVIGATION_3D_ACTIVE_MAPS}

```gdscript
NAVIGATION_3D_ACTIVE_MAPS = 49
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_REGION_COUNT {#NAVIGATION_3D_REGION_COUNT}

```gdscript
NAVIGATION_3D_REGION_COUNT = 50
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_AGENT_COUNT {#NAVIGATION_3D_AGENT_COUNT}

```gdscript
NAVIGATION_3D_AGENT_COUNT = 51
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_LINK_COUNT {#NAVIGATION_3D_LINK_COUNT}

```gdscript
NAVIGATION_3D_LINK_COUNT = 52
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_POLYGON_COUNT {#NAVIGATION_3D_POLYGON_COUNT}

```gdscript
NAVIGATION_3D_POLYGON_COUNT = 53
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_EDGE_COUNT {#NAVIGATION_3D_EDGE_COUNT}

```gdscript
NAVIGATION_3D_EDGE_COUNT = 54
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_EDGE_MERGE_COUNT {#NAVIGATION_3D_EDGE_MERGE_COUNT}

```gdscript
NAVIGATION_3D_EDGE_MERGE_COUNT = 55
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_EDGE_CONNECTION_COUNT {#NAVIGATION_3D_EDGE_CONNECTION_COUNT}

```gdscript
NAVIGATION_3D_EDGE_CONNECTION_COUNT = 56
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_EDGE_FREE_COUNT {#NAVIGATION_3D_EDGE_FREE_COUNT}

```gdscript
NAVIGATION_3D_EDGE_FREE_COUNT = 57
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NAVIGATION_3D_OBSTACLE_COUNT {#NAVIGATION_3D_OBSTACLE_COUNT}

```gdscript
NAVIGATION_3D_OBSTACLE_COUNT = 58
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MONITOR_MAX {#MONITOR_MAX}

```gdscript
MONITOR_MAX = 59
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MonitorType {#enum-MonitorType}

```gdscript
enum MonitorType { MONITOR_TYPE_QUANTITY = 0, MONITOR_TYPE_MEMORY = 1, MONITOR_TYPE_TIME = 2, MONITOR_TYPE_PERCENTAGE = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MONITOR_TYPE_QUANTITY {#MONITOR_TYPE_QUANTITY}

```gdscript
MONITOR_TYPE_QUANTITY = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MONITOR_TYPE_MEMORY {#MONITOR_TYPE_MEMORY}

```gdscript
MONITOR_TYPE_MEMORY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MONITOR_TYPE_TIME {#MONITOR_TYPE_TIME}

```gdscript
MONITOR_TYPE_TIME = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MONITOR_TYPE_PERCENTAGE {#MONITOR_TYPE_PERCENTAGE}

```gdscript
MONITOR_TYPE_PERCENTAGE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
