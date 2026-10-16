# 三大支柱总览

**节点（Node）组织对象，场景（Scene）打包复用，信号（Signal）解耦通信**——这是 Godot 的全部。

## 一句话模型

> 场景打包复用对象；节点构成对象树承载逻辑；信号让节点互通信而不互持有。

这三句话拆开：

- **节点**：一切都是节点。`Node` 是基类，`Node2D`/`Node3D`/`Control`/`Area2D` 都是它的子类。节点有父子关系，构成树。
- **场景**：保存为 `.tscn` 文件的一棵节点树 = 预制体。可以实例化、嵌套、复用。
- **信号**：节点声明"我发生了什么"（如 `died`），其他节点连接后响应，不需要知道对方是谁。

## 与 Unity / UE 对照

| 概念 | Godot | Unity | Unreal |
|---|---|---|---|
| 对象容器 | Node（继承树） | GameObject | AActor |
| 组件 | 脚本直接挂节点 | Component（挂 GameObject） | UActorComponent |
| 预制体 | Scene（.tscn） | Prefab | Blueprint |
| 事件 | Signal（声明式） | UnityEvent / C# event | Delegates |
| 父子 | Node 树（坐标继承） | Transform 层级 | Attachment |

差异最大处：

1. **Godot 没有空 GameObject**——节点本身就是有类型的（Sprite2D、CollisionShape2D），不需要"空物体"容器。
2. **Godot 脚本是节点的一部分**，不是独立组件。`extends Node2D` 的脚本就是 Node2D 本身。
3. **信号是声明式数据**，不是函数指针。在 `.tscn` 里 `[connection]` 段声明，编辑器里可视化连接。

## 反直觉点

> 值得注意的是：Godot 的"组合"不是 Unity 那种"给空物体挂组件"，而是**把子节点作为功能拆分**。一个玩家角色不是一个巨脚本，而是 `Player`（根）+ `Sprite2D` + `CollisionShape2D` + `Camera2D` + `AnimationPlayer`，每个子节点管自己的事。

## 延伸阅读

- 本站：[节点](02-node.md)、[场景](03-scene.md)、[信号](04-signals.md)
- 官方：[Node and Scene](https://docs.godotengine.org/en/stable/getting_started/step_by_step/nodes_and_scenes.html)
