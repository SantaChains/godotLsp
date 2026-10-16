# 节点选型目录

**Godot 有数百个节点，但开局只需要按"我要做什么"选对基类，其余按职责挂子节点。** 本页是选型导航，不是全量类表；每个类的完整签名查[类参考](../builtin/index.md)。

## 按需求直接选

| 我要做什么 | 首选节点 | 关键子节点 |
|---|---|---|
| 玩家直接控制的角色 | [CharacterBody2D](../builtin/class_characterbody2d.md) / [CharacterBody3D](../builtin/class_characterbody3d.md) | CollisionShape、Sprite、Camera |
| 墙、地板、不动的障碍 | [StaticBody2D](../builtin/class_staticbody2d.md) / [StaticBody3D](../builtin/class_staticbody3d.md) | CollisionShape |
| 受重力碰撞驱动的物体（箱子、弹珠） | [RigidBody2D](../builtin/class_rigidbody2d.md) / [RigidBody3D](../builtin/class_rigidbody3d.md) | CollisionShape |
| 动画/代码驱动移动并推开别人的门/平台 | [AnimatableBody2D](../builtin/class_animatablebody2d.md) / [AnimatableBody3D](../builtin/class_animatablebody3d.md) | CollisionShape |
| 触发器、拾取物、视野区、局部重力/音频区 | [Area2D](../builtin/class_area2d.md) / [Area3D](../builtin/class_area3d.md) | CollisionShape |
| 角色帧动画 | [AnimatedSprite2D](../builtin/class_animatedsprite2d.md) | SpriteFrames 资源 |
| 单张纹理 | [Sprite2D](../builtin/class_sprite2d.md) | — |
| 跟随镜头 | [Camera2D](../builtin/class_camera2d.md) / [Camera3D](../builtin/class_camera3d.md) | — |
| 瓦片地图 | [TileMapLayer](../builtin/class_tilemaplayer.md)（4.x，多实例叠加） | TileSet 资源 |
| 路径移动 | [Path2D](../builtin/class_path2d.md) + [PathFollow2D](../builtin/class_pathfollow2d.md) | — |
| 玩家与 UI 之间的非空间音乐 | [AudioStreamPlayer](../builtin/class_audiostreamplayer.md) | — |
| 随距离衰减的环境音 | [AudioStreamPlayer2D](../builtin/class_audiostreamplayer2d.md) / [AudioStreamPlayer3D](../builtin/class_audiostreamplayer3d.md) | — |

## 2D 物理四体辨析（最易混）

四种物理体都继承 [PhysicsBody2D](../builtin/class_physicsbody2d.md)，区别在谁决定运动：

| 节点 | 谁决定位置 | 受重力 | 强行移动时对别人 |
|---|---|---|---|
| StaticBody2D | 不动 | 否 | 移动即**传送**，不推动别人 |
| AnimatableBody2D | 代码/动画 | 否 | 移动会**推开**挡路物体（移动平台、自动门） |
| RigidBody2D | 物理仿真（重力+碰撞） | 是 | 由质量/弹性/摩擦解算 |
| CharacterBody2D | 脚本 `move_and_slide()` | 否（自己管 velocity） | 精确运动控制，不被物理撞飞 |

> 选错的典型症状：用 RigidBody2D 做玩家导致抖动和滑墙；用 StaticBody2D 做移动平台发现载不动角色；用 CharacterBody2D 却期待它被撞击推开。

## Area2D 的双用途

[Area2D](../builtin/class_area2d.md) 本身不阻挡（不是墙），只感知，两类用途：

1. **进出检测**：`body_entered` / `body_exited`（物理体进出）、`area_entered` / `area_exited`（其他 Area 进出）。拾取物监听 `body_entered` 判断是不是玩家，是则 `queue_free()` 并加分；剧情触发点、检查点、敌人视野（进入切追击状态）同构。
2. **局部覆盖区**：覆盖重力/风/音频 bus，做水下区、零重力区、毒气沼泽（每秒扣血/改速度）。

碰撞靠子节点 [CollisionShape2D](../builtin/class_collisionshape2d.md)（矩形/圆/胶囊等几何）或 [CollisionPolygon2D](../builtin/class_collisionpolygon2d.md)（任意多边形轮廓）定义形状，缺它 Area 不工作。

**碰撞层与遮罩**（layer=我是谁，mask=我检测谁）：

- 玩家设在 layer 1；地图触发区 Area2D 的 layer 设为 2（我是谁），mask 设为 1（我检测玩家）。
- 想被谁检测到，就让对方的 mask 包含自己的 layer。物理碰撞同理，双方 layer/mask 对齐才会交互。

## 2D 视觉、光照与路径

| 类别 | 节点 | 职责 |
|---|---|---|
| 相机 | Camera2D | 观察参考点；拖动/平滑/缩放/限界 |
| 线条 | [Line2D](../builtin/class_line2d.md) | 顶点数组画线；宽度/渐变/圆角 |
| 填充形状 | [Polygon2D](../builtin/class_polygon2d.md) | 顶点填充；可带纹理与骨骼权重 |
| 点光 | [PointLight2D](../builtin/class_pointlight2d.md) | 纹理定光形，阴影自中心投射 |
| 平行光 | [DirectionalLight2D](../builtin/class_directionallight2d.md) | 整场模拟日/月，阴影不随距离扭曲 |
| 阴影遮挡 | [LightOccluder2D](../builtin/class_lightoccluder2d.md) | 配 OccluderPolygon2D 在光后方投阴影 |
| 打底压暗 | [CanvasModulate](../builtin/class_canvasmodulate.md) | 先把层压暗，2D 光源再重新照亮 |
| 射线检测 | [RayCast2D](../builtin/class_raycast2d.md) | 无限细直线；报命中点/对象 |
| 形状检测 | [ShapeCast2D](../builtin/class_shapecast2d.md) | 用矩形/圆形而非细线扫掠 |
| 远程变换 | [RemoteTransform2D](../builtin/class_remotetransform2d.md) | 把自身变换推给远端节点，非真实父子 |
| 寻路 | NavigationRegion2D / NavigationAgent2D / NavigationLink2D / NavigationObstacle2D | 烘焙区域、代理、链接、动态障碍 |
| 屏幕可见 | VisibleOnScreenNotifier2D / VisibleOnScreenEnabler2D | 进出屏幕发信号 / 屏内自动启用处理 |
| 合并绘制 | [CanvasGroup](../builtin/class_canvasgroup.md) | 子节点合并一次绘制；整体透明度不互相叠加 |
| 屏幕捕获 | [BackBufferCopy](../builtin/class_backbuffercopy.md) | 捕获区域供着色器 `hint_screen_texture` 采样 |

关节仅三种 2D：[DampedSpringJoint2D](../builtin/class_dampedspringjoint2d.md)（弹簧，长度/刚度/阻尼）、[GrooveJoint2D](../builtin/class_groovejoint2d.md)（活塞，沿一轴伸缩）、[PinJoint2D](../builtin/class_pinjoint2d.md)（钉于一点自由旋转）。**2D 无滑动关节**，滑动仅 3D。

## 3D 对应结构

3D 与 2D 同构：[Node3D](../builtin/class_node3d.md) 为基类，视觉走 [VisualInstance3D](../builtin/class_visualinstance3d.md) → [GeometryInstance3D](../builtin/class_geometryinstance3d.md) → [MeshInstance3D](../builtin/class_meshinstance3d.md)；四体为 StaticBody3D / AnimatableBody3D / RigidBody3D / CharacterBody3D；区域 Area3D；光源 [OmniLight3D](../builtin/class_omnilight3d.md)（点光）/ [SpotLight3D](../builtin/class_spotlight3d.md)（锥角聚光）/ [DirectionalLight3D](../builtin/class_directionallight3d.md)（日月）；环境 [WorldEnvironment](../builtin/class_worldenvironment.md)；海量重复网格用 [MultiMeshInstance3D](../builtin/class_multimeshinstance3d.md)。3D 关节六种：Pin/Hinge/Slider/ConeTwist/Generic6DOF，外加载具 VehicleBody3D/VehicleWheel3D（官方承认有已知问题）。

## UI（Control）选型

排布交给容器，不要手动算坐标：

| 需求 | 节点 |
|---|---|
| 横向/纵向排列 | HBoxContainer / VBoxContainer |
| 网格 | GridContainer |
| 自动换行 | FlowContainer（H/V 两种） |
| 统一内边距 / 居中 / 锁比例 | MarginContainer / CenterContainer / AspectRatioContainer |
| 超界滚动 | ScrollContainer |
| 可拖分割条 / 多页 | SplitContainer / TabContainer |
| 按钮 | Button / TextureButton / LinkButton |
| 勾选 | CheckBox（配 ButtonGroup 做单选）/ CheckButton |
| 文本 | Label / RichTextLabel / LineEdit / TextEdit |
| 取值 | HSlider/VSlider、ProgressBar、SpinBox、OptionButton |
| 底板/色块/图片 | Panel / ColorRect / TextureRect / NinePatchRect |
| 弹层 | PopupPanel / PopupMenu / AcceptDialog / ConfirmationDialog / FileDialog |

> 避坑：**没有独立 RadioButton**，单选是 CheckBox/CheckButton 配 [ButtonGroup](../builtin/class_buttongroup.md)（资源，非节点）；[Theme](../builtin/class_theme.md) 是资源不是节点，挂 Control 或项目设置。

## 视口与窗口

[Viewport](../builtin/class_viewport.md) 是渲染世界基类与输入/音频枢纽；[SubViewport](../builtin/class_subviewport.md) 渲染到纹理（小地图、镜面、画中画），由 [SubViewportContainer](../builtin/class_subviewportcontainer.md) 放进 UI；[Window](../builtin/class_window.md) 承载原生弹出子窗口。

## 4.x 版本基线（迁移避坑）

- TileMap 弃用，改 [TileMapLayer](../builtin/class_tilemaplayer.md)，多实例叠加代替单节点多图层。
- AnimationPlayer 与 AnimationTree 统一继承 AnimationMixer。
- VisibilityNotifier/Enabler 更名 VisibleOnScreenNotifier/Enabler。
- ParallaxBackground+ParallaxLayer 的旧方案在 4.3+ 由 [Parallax2D](../builtin/class_parallax2d.md) 取代（旧节点保留）。
- 4.4 起 3D 可选用内置 Jolt Physics，默认仍 Godot Physics。

## 延伸阅读

- 官方：[Node 类参考](../builtin/class_node.md)、[2D 节点清单](https://docs.godotengine.org/en/stable/classes/categories.html)
- 本站：[节点 Node](../paradigm/02-node.md)、[组合优于继承](09-composition.md)、[场景 Scene](03-scene.md)
