# 脚本即组件

**GDScript 脚本附加在节点上，是节点的行为定义，不是独立对象。**

## 纠正直觉偏差

> 值得注意的是：GDScript 不是"纯 OOP 语言"。你写的 `extends CharacterBody2D` 脚本就是 CharacterBody2D 本身，不是"挂在 CharacterBody2D 上的组件"。

## 与 Unity 的区别

| | Unity | Godot |
|---|---|---|
| 对象 | GameObject（空容器） | Node（有类型） |
| 脚本 | Component（挂上去） | 就是节点本身 |
| 组合 | 给空物体挂多个组件 | 把子节点作为功能拆分 |

## 组合优于继承的 Godot 版

```
# 不要这样：深继承链
RigidBody2D → Enemy → FlyingEnemy → BossFlyingEnemy

# 要这样：节点组合
Enemy (CharacterBody2D)
├── Health (Resource)
├── Attack (脚本)
└── Movement (脚本)
```

继承只用于"类型 IS-A"，不用于"功能叠加"。功能叠加靠子节点。

## 延伸阅读

- 本站：[节点](02-node.md)、[组合优于继承](09-composition.md)
- 官方：[Scripting](https://docs.godotengine.org/en/stable/tutorials/scripting/index.html)
