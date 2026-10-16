https://www.youtube.com/watch?v=22VYNOtrcgM
Node2D: 0:30
Camera2D: 0:40
Animated/Sprite2D: 0:53
CollisionObject2D: 1:08
PhysicsBody2D: 1:18
Collision Polygon/Shape 2D: 1:26
StaticBody2D: 1:38
AnimatableBody2D: 1:54
RigidBody2D: 2:09
CharacterBody2D: 2:20
Joint2D: 2:32
DampedSpringJoint2D: 2:42
GrooveJoint2D: 2:52
PinJoint2D: 3:03
Area2D: 3:11
AudioListener2D: 3:30
AudioStreamPlayer2D: 3:40
CPU/GPU Particles 2D: 3:50
TileMap: 4:09
CanvasModulate: 4:30
Light2D: 4:40
PointLight2D: 4:47
DirectionalLight2D: 5:11
LightOccluder2D: 5:24
Line2D: 5:34
Marker2D: 5:48
MeshInstance2D: 5:57
MultiMeshInstance2D: 6:06
Navigation Region/Link/Obstacle 2D: 6:16
Parallax Background/Layer: 6:36
Follow/Path2D: 6:48
Polygon2D: 6:59
Raycast2D: 7:08
ShapeCast2D: 7:17
RemoteTransform2D: 7:29
Skeleton2D Physical/Bone2D: 7:43
VisibleOnScreen Enabler/Notifier 2D: 8:02
CanvasGroup: 8:18
BackBufferCopy: 8:28

各位观众，我们回来了。
Godot 节点详解系列第三集。
我还没疯，今天要讲完所有 2D 节点。
一集搞定。
拿点零食，跟我一起探索所有 2D 节点。
今天用这个 tileset 和角色做演示。
下载链接在简介里。
如果要用，请支持原作者。
我们开始吧。
Node2D。
Node2D 是所有 2D 节点的基类。
它管理旋转、位置和缩放。
今天要讲的节点都继承自 Node2D。
Camera2D。它定义了场景的观察视角。
很简单。包含旋转和位置参数，
还能通过拖拽和平滑让镜头更舒服。
AnimatedSprite2D 和 Sprite2D。它们
用于在屏幕上渲染 2D 纹理。可以显示
大图集的一部分。AnimatedSprite2D 内置了
精灵表动画工具。
CollisionObject2D。
它不能单独使用，
但作为所有碰撞物体的基类。它有碰撞层
和启用开关等属性。
PhysicsBody2D。
它同样不能
单独使用，是所有受物理影响的 2D 物体的基类。
CollisionShape2D 和 CollisionPolygon2D。下面要讲的节点
都需要碰撞形状。它必须是物理体的子节点，
你可以在检查器里设置形状。
StaticBody2D。
StaticBody2D
是不可移动的物理体。
但其他物理体可以受它影响。
它可以
用作墙壁、地板或天花板。如果移动它，它不会
推动其他物体，而是直接瞬移。AnimatableBody2D。
AnimatableBody2D。
它和 StaticBody2D 几乎一样，只是它可以
移动。它仍不受重力影响，但如果你通过代码
或动画移动它，
它会推动挡路的其他物理体。
RigidBody2D。
它受重力和其它物理体影响。
用于所有可移动、非玩家直接控制的
物理物品。
CharacterBody2D。
它是专为玩家控制
设计的物理体。内置了通过代码
在物理模拟中精确控制移动的功能。
Join2D 不能单独使用，它是所有 2D 物理关节的基类。
这些节点用于连接两个物理体，Godot 包含以下 2D
关节。
Dampened Spring Joint2D。
它像弹簧一样连接两个物理体，
包含弹簧长度、刚度
和阻尼参数。
Groove Joint2D。
名字有点让人困惑，但它
像活塞一样连接两个物理体，只能沿一个
运动轴伸缩。
最后是 Pin Joint2D。
它用于将两个物理体
钉在一起。它们在一个点连接，可以自由旋转。
Area2D。
Area2D 有两种用途。
一，检测碰撞对象
是否进入或离开。
二，拥有独立物理和音频设置的空间区域。
你可以改变重力、风、音频通道，从而创建
水下区域或零重力区域。
AudioListener2D。
默认相机是 2D 音频的监听点，但如果你想从其他
位置收听，可以用这个节点设置监听点。
AudioStreamPlayer2D。
这是 Godot 的 2D 音频源。
它可以从空间某点播放音频，包含所有常用音频设置。
音量、音调、范围、衰减。
CPUParticles2D 和 GPUParticles2D。
这是 Godot 的 2D 粒子系统。
可以构建很多有趣酷炫的粒子系统。
参数非常多，这里无法详述，但我觉得粒子
系统很直观。
学习的好方法就是添加到场景中
调整参数。
TileMap。
TileMap 节点很庞大，这里无法详述。
它用于构建 2D 瓦片地图。
是绘制瓦片地图和设置瓦片集的内置工具。
Godot 还能为瓦片地图分配物理层和导航层。
功能非常强大。
我会在文档链接中提供更多教程。
现在聊聊 Godot 的 2D 光照。
CanvasModulate 节点。
设置 2D 阴影需要它。
它先让场景变暗，以便 2D 光源重新照亮。
Light2D 节点。
不能单独使用，是所有 2D 光源的基类。
包含光色、能量和混合模式设置。
PointLight2D。
点光源用于从一个点照亮场景。
但你需要给它一个纹理。
这决定了光的外观。
它并非中心一个点向四周发光，
而是用这个纹理来照亮场景。
那为何叫点光源，不叫纹理光源？
因为它从中心投射阴影。
光照用纹理，阴影投射则从点出发。
DirectionalLight2D。
DirectionalLight2D 用于一次性照亮整个场景。
它模拟太阳或月亮，从一个角度照射，
阴影不会像点光源那样随距离扭曲。
LightOccluder2D。
LightOccluder2D 是用于在场景中投射阴影的形状。
你可以设置物体的形状，它就会据此
在场景中投射阴影。
Line2D。
Line2D 节点顾名思义。
通过向数组添加点来绘制 2D 线条。
它有很多样式属性，比如宽度、渐变，
以及线条拐角的圆角和渲染方式。
Marker2D。Marker2D 是一个调试节点。它就是个 Node2D，只是在编辑器中显示位置和旋转。
可用于标记不可见节点或空间中的点。
空间中的点。
MeshInstance2D。
MeshInstance2D 节点用于在 2D 中渲染网格，也就是 3D 模型。
它有渲染网格和材质的属性。
MultiMeshInstance2D。
MultiMeshInstance2D 可高效渲染大量网格。
建议在大量网格密集时使用。
比如一片草地。
NavigationRegion2D、NavigationLink2D 和 NavigationObstacle2D。
这些节点用于设置 2D 寻路。
比 3D 寻路节点复杂些，因为需要手动设置导航区域。
手动设置。
而 3D 中有自动生成导航网格的工具。
文档中有关于 2D 寻路设置的详细说明，链接在简介里。
ParallaxLayer 和 ParallaxBackground 节点。
ParallaxLayer 和 ParallaxBackground 用于在 2D 场景中设置视差滚动背景。
设置视差滚动背景。
任何 2D 图形都可以，只要它们是相应视差层的子节点。
相应视差层的子节点。
Path2D 和 PathFollow2D。
Path2D 和 PathFollow2D 节点用于设置 2D 路径或曲线，
然后用 PathFollow2D 节点将另一个节点绑定到该路径。
Polygon2D。
Polygon2D 节点允许你在 2D 中设置形状。
你可以在此数组中添加多边形点，或者直接在编辑器中拖放。
或者直接在编辑器中拖放。
Raycast2D。
Raycast2D 节点用于检测直线碰撞。
它能报告是否命中、碰撞点和碰撞对象。
Shapecast2D。
Shapecast2D 与 Raycast2D 类似，但用形状
检测碰撞。
Raycast 发射无限细的线，而 Shapecast 发射形状，比如矩形
或圆形。
RemoteTransform2D。
RemoteTransform2D 节点能将自身变换同步给另一节点。
它相当于该节点的虚拟父级，但非实际层级父级。
可设置是否影响位置、旋转、缩放或全部。
Skeleton2D、Bone2D 和 PhysicalBone2D。
这些节点用于搭建 2D 骨骼和骨骼动画。
有第三方软件可制作骨骼，
但 Godot 也支持在引擎内自建骨骼。
构建和管理 2D 骨骼涉及很多内容。
文档有详细说明。
链接在简介。
VisibleOnScreenNotifier2D 和 VisibleOnScreenEnabler2D。
这些节点用于检测视觉元素是否在屏幕上。
Notifier 在可见时发送信号，
Enabler 在可见时自动启用自身。
CanvasGroup。
CanvasGroup 节点可将所有子节点合并为一次绘制调用。
适合合并纹理，比如
半透明瓶子及其内容。
最后是 BackBufferCopy。
BackBufferCopy 节点用于捕获屏幕区域，供
着色器使用。
可定义捕获区域，在着色器中通过
`hint_screen_texture` 变量访问。
好了，以上就是所有 2D 节点。
有人可能注意到我漏了 TouchScreenButton。
它会归入 UI 节点。
下期视频讲所有 UI 和窗口节点。
应该就全覆盖了。
然后我会把它们整合成一个长视频。
一个视频涵盖所有节点。
感谢大家对本系列的支持。
做起来很开心，听到你们的反馈也很开心。
订阅以观看下期。
下次见，拜拜。

https://www.youtube.com/watch?v=_mAfdeops_E

1:26 Node3D
1:42 Marke3D
1:52 VisualInstance3D
2:09 GeometryInstance3D
2:28 MeshInstance3D
2:41 MultiMeshInstance3D
2:59 Label3D
3:13 Decal
3:23 Sprite3D or AnimatedSprite3D
3:44 WorldEnviroment
4:02 FogVolume
4:09 GPU Particles3D
4:33 GPU ParticlesAttractor3D GPU ParticlesCollision
4:52 OmniLight3D
5:25 SpotLight3D
5:37 DirectionalLight3D
5:57 CSGShade3D
6:21 SoftBody3D
6:37 CollisionObject3D
7:10 PhysicsBody3D
7:21 CollisionPolygon3D CollisionShape3D
7:35 StaticBody3D
7:59 AnimatableBody3D
8:11 RigidBody3D
8:27 CharacterBody3D
8:41 Area3D
