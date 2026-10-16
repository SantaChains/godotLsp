# Godot Server 架构即 DOD

**场景系统是可选层。底层 Server 是 DOD 风格。万级同屏绕过节点树。**

## 双层架构

```
场景层（你写的代码）
  Node / Sprite2D / CharacterBody2D
      ↓ 调用
Server 层（底层引擎）
  RenderingServer / PhysicsServer2D / AudioServer
      ↓
  RID 句柄 → 连续内存数组（SoA）
```

## RID 句柄

RID 是 opaque handle，类似文件描述符。Server 用 RID 索引内部连续数组。你不直接操作内存，只传 RID。

> 值得注意的是：RID 不是指针。它内部是一个索引+版本号。引用计数管理生命周期，悬空 RID 会报错而非崩。

## 何时绕过节点树

| 场景 | 方案 |
|---|---|
| 几十个敌人 | 节点树 + _process |
| 几百个子弹 | 对象池 + 管理器遍历 |
| 上万个同屏实例 | MultiMesh + Server API |

## MultiMesh

MultiMesh 一次 draw call 画上千个实例。GPU 批量处理，CPU 只传一次变换数组。这是 Godot 官方推荐的万级实例方案。

```gdscript
# MultiMeshInstance2D 节点
var mm = MultiMesh.new()
mm.transform_format = MultiMesh.TRANSFORM_2D
mm.instance_count = 1000
for i in 1000:
    mm.set_instance_transform_2d(i, Transform2D(0, Vector2(x, y)))
```

## 延伸阅读

- 本站：[DOD](01-dod.md)、[批量与结构优化](../perf/03-bulk.md)
- 官方：[Using servers](https://docs.godotengine.org/en/stable/tutorials/performance/using_servers.html)
