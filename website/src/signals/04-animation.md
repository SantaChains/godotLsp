<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 动画与补间

> AnimationMixer/Player/Library、帧动画精灵、Tween/Tweener、骨骼相关。

## AnimatedSprite2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_changed()` | 切换到另一个动画时触发。 | 状态切换钩子。 [类页锚点](../builtin/class_animatedsprite2d.md#animation_changed) |
| `signal animation_finished()` | 当前帧动画播放完一轮时触发。 | 动作结束切待机。 [类页锚点](../builtin/class_animatedsprite2d.md#animation_finished) |
| `signal animation_looped()` | 循环动画播完一轮时触发。 | 循环计数。 [类页锚点](../builtin/class_animatedsprite2d.md#animation_looped) |
| `signal frame_changed()` | 当前帧号改变时触发。 | 逐帧触发事件（脚步声）。 [类页锚点](../builtin/class_animatedsprite2d.md#frame_changed) |
| `signal sprite_frames_changed()` | sprite_frames 资源替换时触发。 | 换图集后重置。 [类页锚点](../builtin/class_animatedsprite2d.md#sprite_frames_changed) |

## AnimatedSprite3D（继承 SpriteBase3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_changed()` | 3D 切换动画时触发。 | 状态钩子。 [类页锚点](../builtin/class_animatedsprite3d.md#animation_changed) |
| `signal animation_finished()` | 3D 精灵帧动画播完时触发。 | 同 2D。 [类页锚点](../builtin/class_animatedsprite3d.md#animation_finished) |
| `signal animation_looped()` | 3D 循环一轮时触发。 | 循环计数。 [类页锚点](../builtin/class_animatedsprite3d.md#animation_looped) |
| `signal frame_changed()` | 3D 帧号改变时触发。 | 逐帧事件。 [类页锚点](../builtin/class_animatedsprite3d.md#frame_changed) |
| `signal sprite_frames_changed()` | 3D 精灵图集替换时触发。 | 换图集重置。 [类页锚点](../builtin/class_animatedsprite3d.md#sprite_frames_changed) |

## AnimationLibrary（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_added(anim_name: StringName)` | 新增动画片段时触发，携带名。 | 动态建库刷新。 [类页锚点](../builtin/class_animationlibrary.md#animation_added) |
| `signal animation_changed(anim_name: StringName)` | 库内容整体改变时触发。 | 刷新。 [类页锚点](../builtin/class_animationlibrary.md#animation_changed) |
| `signal animation_removed(anim_name: StringName)` | 移除动画片段时触发。 | 配合上者。 [类页锚点](../builtin/class_animationlibrary.md#animation_removed) |
| `signal animation_renamed(old_name: StringName, new_name: StringName)` | 动画改名时触发。 | 更新引用。 [类页锚点](../builtin/class_animationlibrary.md#animation_renamed) |

## AnimationMixer（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_finished(anim_name: StringName)` | 当前播放动画播放完毕时触发。 | 播完切状态：`$Mixer.animation_finished.connect(_done)`。 [类页锚点](../builtin/class_animationmixer.md#animation_finished) |
| `signal animation_libraries_updated()` | 动画库发生变更时通知。 | 动态加库后刷新。 [类页锚点](../builtin/class_animationmixer.md#animation_libraries_updated) |
| `signal animation_list_changed()` | 动画列表增删时触发。 | 运行时动态加动画后刷新。 [类页锚点](../builtin/class_animationmixer.md#animation_list_changed) |
| `signal animation_started(anim_name: StringName)` | 开始播放某动画时触发，携带动画名。 | 动画切换钩子。 [类页锚点](../builtin/class_animationmixer.md#animation_started) |
| `signal caches_cleared()` | 缓存被自动或手动 clear_caches() 清空时通知。 | 清缓存后重建。 [类页锚点](../builtin/class_animationmixer.md#caches_cleared) |
| `signal mixer_applied()` | 混合结果已应用到目标对象时通知。 | 应用后钩子。 [类页锚点](../builtin/class_animationmixer.md#mixer_applied) |
| `signal mixer_updated()` | 属性处理流程更新时通知。 | 更新钩子。 [类页锚点](../builtin/class_animationmixer.md#mixer_updated) |

## AnimationNode（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_node_removed(object_id: int, node_name: String)` | 内部树中某动画节点被移除时触发，携带对象 id 与节点名。 | 节点树变更。 [类页锚点](../builtin/class_animationnode.md#animation_node_removed) |
| `signal animation_node_renamed(object_id: int, old_name: String, new_name: String)` | 内部树中节点改名时触发，携带 id/旧名/新名。 | 更新引用。 [类页锚点](../builtin/class_animationnode.md#animation_node_renamed) |
| `signal node_updated(object_id: int)` | 动画资源改变或连线变化时触发，携带对象 id。 | 刷新节点。 [类页锚点](../builtin/class_animationnode.md#node_updated) |
| `signal tree_changed()` | 内部树任一节点变化时触发。 | 树结构刷新。 [类页锚点](../builtin/class_animationnode.md#tree_changed) |

## AnimationNodeBlendSpace2D（继承 AnimationRootNode）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal triangles_updated()` | 混合空间三角形增删或顶点移动时触发。 | 重建三角插值。 [类页锚点](../builtin/class_animationnodeblendspace2d.md#triangles_updated) |

## AnimationNodeBlendTree（继承 AnimationRootNode）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal node_changed(node_name: StringName)` | 输入端口信息改变时触发，携带节点名。 | 端口重连。 [类页锚点](../builtin/class_animationnodeblendtree.md#node_changed) |

## AnimationNodeStateMachinePlayback（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal state_finished(state: StringName)` | 某状态播放结束时触发，携带状态名。 | 状态完成转移。 [类页锚点](../builtin/class_animationnodestatemachineplayback.md#state_finished) |
| `signal state_started(state: StringName)` | 某状态开始播放时触发，携带状态名。 | 进入状态钩子。 [类页锚点](../builtin/class_animationnodestatemachineplayback.md#state_started) |

## AnimationNodeStateMachineTransition（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal advance_condition_changed()` | advance_condition 改变时触发。 | 转移条件刷新。 [类页锚点](../builtin/class_animationnodestatemachinetransition.md#advance_condition_changed) |

## AnimationPlayer（继承 AnimationMixer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_changed(old_name: StringName, new_name: StringName)` | 动画库内容改变时触发。 | 编辑期/运行时库更新。 [类页锚点](../builtin/class_animationplayer.md#animation_changed) |
| `signal current_animation_changed(anim_name: StringName)` | 当前动画名改变时触发。 | 追踪 AnimationPlayer 当前片段。 [类页锚点](../builtin/class_animationplayer.md#current_animation_changed) |

## AnimationTree（继承 AnimationMixer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal animation_player_changed()` | 绑定的动画播放器改变时触发。 | 重绑后重置。 [类页锚点](../builtin/class_animationtree.md#animation_player_changed) |

## BoneMap（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal bone_map_updated()` | BoneMap 键值改变时触发。 | 骨骼映射刷新。 [类页锚点](../builtin/class_bonemap.md#bone_map_updated) |
| `signal profile_updated()` | 配置文件值或其引用改变时触发。 | 配置刷新。 [类页锚点](../builtin/class_bonemap.md#profile_updated) |

## Skeleton2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal bone_setup_changed()` | 挂接的 Bone2D 配置改变时触发。 | 2D 骨骼重建。 [类页锚点](../builtin/class_skeleton2d.md#bone_setup_changed) |

## Skeleton3D（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal bone_enabled_changed(bone_idx: int)` | 用 set_bone_enabled() 切换某骨骼启用时触发，携带骨号。 | 骨骼开关同步。 [类页锚点](../builtin/class_skeleton3d.md#bone_enabled_changed) |
| `signal bone_list_changed()` | 骨骼列表变化（增骨/改父/解绑/清空）时触发。 | 重建骨骼表。 [类页锚点](../builtin/class_skeleton3d.md#bone_list_changed) |
| `signal pose_updated()` | 姿态更新时触发。 | 姿态后处理。 [类页锚点](../builtin/class_skeleton3d.md#pose_updated) |
| `signal rest_updated()` | 静止姿势更新时触发。 | 重置姿态。 [类页锚点](../builtin/class_skeleton3d.md#rest_updated) |
| `signal show_rest_only_changed()` | show_rest_only 值改变时触发。 | 视图切换。 [类页锚点](../builtin/class_skeleton3d.md#show_rest_only_changed) |
| `signal skeleton_updated()` | 最终姿态计算完成并将应用到蒙皮前触发。 | 蒙皮前钩子。 [类页锚点](../builtin/class_skeleton3d.md#skeleton_updated) |

## SkeletonModifier3D（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal modification_processed()` | 修饰处理完成时通知。 | 修饰后钩子。 [类页锚点](../builtin/class_skeletonmodifier3d.md#modification_processed) |

## Tween（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | 补间全部完成（含循环结束）时触发。 | 序列播完执行下一步。 [类页锚点](../builtin/class_tween.md#finished) |
| `signal loop_finished(loop_count: int)` | 一轮循环结束时触发。 | 循环计数。 [类页锚点](../builtin/class_tween.md#loop_finished) |
| `signal step_finished(idx: int)` | 单个 step 补间完成时触发。 | 分步编排。 [类页锚点](../builtin/class_tween.md#step_finished) |

## Tweener（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal finished()` | 单个补间项完成时触发。 | 项级完成钩子。 [类页锚点](../builtin/class_tweener.md#finished) |

<!-- SIGNALS:END -->
