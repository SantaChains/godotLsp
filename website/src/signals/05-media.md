<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 音频 / 视频 / 粒子 / 计时

> 播放器完成、音频总线、视频结束、粒子发射完毕、定时触发、喂给相机帧。

## AudioServer（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal bus_layout_changed()` | 音频总线新增/删除/移动时触发。 | 总线布局重建。 [类页锚点](../builtin/class_audioserver.md#bus_layout_changed) |
| `signal bus_renamed(bus_index: int, old_name: StringName, new_name: StringName)` | 指定索引总线改名时触发，携带索引/旧名/新名。 | 总线名同步。 [类页锚点](../builtin/class_audioserver.md#bus_renamed) |

## AudioStream（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal parameter_list_changed()` | 参数列表改变时通知。 | 动态参数刷新。 [类页锚点](../builtin/class_audiostream.md#parameter_list_changed) |

## AudioStreamPlayer（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | 音频完整播放完毕（未被打断）时触发。 | 播完切下一首。 [类页锚点](../builtin/class_audiostreamplayer.md#finished) |

## AudioStreamPlayer2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | 2D 位置音频播完时触发。 | 音效结束回收。 [类页锚点](../builtin/class_audiostreamplayer2d.md#finished) |

## AudioStreamPlayer3D（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | 3D 位置音频播完时触发。 | 音效结束回收。 [类页锚点](../builtin/class_audiostreamplayer3d.md#finished) |

## CPUParticles2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | CPU 2D 粒子发射周期结束时触发。 | 一次性特效完成销毁。 [类页锚点](../builtin/class_cpuparticles2d.md#finished) |

## CPUParticles3D（继承 GeometryInstance3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | CPU 3D 粒子发射结束时触发。 | 一次性特效完成。 [类页锚点](../builtin/class_cpuparticles3d.md#finished) |

## CameraFeed（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal format_changed()` | 采集格式改变时触发。 | 重设帧尺寸。 [类页锚点](../builtin/class_camerafeed.md#format_changed) |
| `signal frame_changed()` | 新一帧可用时触发。 | 取帧回调。 [类页锚点](../builtin/class_camerafeed.md#frame_changed) |

## GPUParticles2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | GPU 2D 粒子发射结束时触发。 | 一次性特效完成。 [类页锚点](../builtin/class_gpuparticles2d.md#finished) |

## GPUParticles3D（继承 GeometryInstance3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | GPU 3D 粒子发射结束时触发。 | 一次性特效完成。 [类页锚点](../builtin/class_gpuparticles3d.md#finished) |

## ParticleProcessMaterial（继承 Material）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal emission_shape_changed()` | 发射形状被改动时触发。 | 重建发射范围。 [类页锚点](../builtin/class_particleprocessmaterial.md#emission_shape_changed) |

## SceneTreeTimer（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal timeout()` | SceneTree.create_timer 返回的一次性计时器到时触发。 | await 之外的一次性延时回调。 [类页锚点](../builtin/class_scenetreetimer.md#timeout) |

## Timer（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal timeout()` | 计时到达设定时间时触发。 | 延时/循环：`$Timer.timeout.connect(_alarm)`。 [类页锚点](../builtin/class_timer.md#timeout) |

## VideoStreamPlayer（继承 Control）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | 视频播放结束时触发。 | 过场视频完毕进游戏。 [类页锚点](../builtin/class_videostreamplayer.md#finished) |

<!-- SIGNALS:END -->
