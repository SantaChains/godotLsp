# Godot Server 架构即 DOD

**Godot 是双层结构：你写的场景/节点是方便人的上层，RenderingServer/PhysicsServer/AudioServer 这些 Server 才是持有数据的底层。Server 内部按 DOD 方式组织，节点只是它的一句句调用。万级同屏的本质就是绕开节点树，直接在这一层批量提交。**

## 双层架构

```
场景层（你的代码）
  Node / Sprite2D / CharacterBody2D
      │ 每个节点只是一个薄封装，调用时把 RID 传给 Server
      ▼
Server 层（引擎持有真实数据）
  RenderingServer / PhysicsServer2D / AudioServer / TextServer
      │
      ▼
  RID 句柄 → Server 内部连续存储（偏向 SoA 批量处理）
```

节点负责组织生命周期、编辑器可见性、信号连接这些"关系"；Server 负责真正的渲染提交、物理求解、音频混音这些"批量数字工作"。绝大多数时候你不需要碰 Server，节点够用；瓶颈出在"数量"上时，瓶颈也正好在节点这层的调度成本，解法是下沉。

## RID 句柄

RID（Resource ID）是不透明句柄，类似文件描述符：它本身不含资源，只是交给 Server 查表的钥匙。

- **不是指针**：你不能通过 RID 直接读写对象内存；4.x 的 RID 内部缓存了不透明指针用于加速，但对外仍是句柄。
- **不是 RefCounted**：生命周期由 Server 分配（allocate）与释放（free）管理，不走对象引用计数；节点释放时对应 Server 资源由引擎回收，手动直调 Server 时必须自己配对 free。
- 无效/空 RID 内部为 0，传错给 Server 会在报错路径上失败，而不是直接解引用崩溃。

## 何时下沉，何时留在节点树

| 规模 | 推荐做法 |
|---|---|
| 几十个实体 | 节点树 + 各自 `_process`，直观优先 |
| 几百个同类动态对象 | 对象池 + 单个管理器遍历数组，减少调度分叉 |
| 上千上万静态/重复实例 | MultiMesh 一次提交；纯渲染需求可直调 RenderingServer |

判断标准不是数量一个数字，而是"每帧重复提交的 draw call / 物理体 / 节点回调是否已成瓶颈"，先看 Debugger 监视器。

## MultiMesh：一次 draw call 画上千实例

MultiMesh 把上千个相同网格/纹理的实例变换打包，CPU 只传一次变换数组，GPU 批量绘制，是官方推荐的万级静态重复方案（落叶、人群、石子、副本）。挂在 `MultiMeshInstance2D`（2D）或 `MultiMeshInstance3D`（3D）上。

```gdscript
# 挂在 MultiMeshInstance2D 节点上；texture 在此节点或 Inspector 指定
@onready var mmi: MultiMeshInstance2D = $MultiMeshInstance2D

func _ready() -> void:
    var mm := MultiMesh.new()
    mm.transform_format = MultiMesh.TRANSFORM_2D
    mm.instance_count = 1000
    # positions 是预算好的 PackedVector2Array，与 instance_count 等长
    for i in mm.instance_count:
        var xform := Transform2D(0.0, positions[i])  # 旋转0，平移到点位
        mm.set_instance_transform_2d(i, xform)
    mmi.multimesh = mm
```

> MultiMesh 适合**外观相同、只变变换**的实例；每个实例行为、动画、材质都不同时，它帮不上忙，那是管理器/ECS 的问题（见[ECS](02-ecs.md)）。需要绕过场景树做纯代码动画提交时，见[直调 RenderingServer](../perf/07-renderingserver-direct.md)。

## 核心句

**节点是给人组织关系的，Server 是给机器批量算数的。数量小时用人的层，数量大时下沉到机器的层。**

## 延伸阅读

- 本站：[DOD](01-dod.md)、[批量与结构优化](../perf/03-bulk.md)、[绕过场景树直调 RenderingServer](../perf/07-renderingserver-direct.md)
- 官方：[Why does Godot use Servers and RIDs?](https://godotengine.org/article/why-does-godot-use-servers-and-rids/)、[Using servers](https://docs.godotengine.org/en/stable/tutorials/performance/using_servers.html)
