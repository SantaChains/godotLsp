# 组合优于继承

**深继承的真正死因不是"代码丑"，而是组合爆炸和脆弱基类。Godot 的节点树就是组合的实践。**

## 深继承为什么死

```
RigidBody2D → Enemy → FlyingEnemy → BossFlyingEnemy
                    → GroundEnemy → BossGroundEnemy
```

问题：
- Boss 既要飞又要近战？需要新类 `BossFlyingMeleeEnemy`。
- 加一个"会隐身"的特性？每个子类都要加。
- 基类改一个字段，所有子类崩。

这就是 **fragile base class**。

## Godot 的解法：节点组合

```
Enemy (CharacterBody2D)
├── Health.gd       # 血量逻辑
├── Attack.gd       # 攻击逻辑
├── Movement.gd     # 移动逻辑
└── Stealth.gd      # 隐身（可选）
```

会隐身就加 Stealth 子节点，不会就不加。不需要新建类。

## 信号解耦 vs 直接调用

```gdscript
# 直接调用：耦合
player.health.take_damage(10)

# 信号解耦：松耦合
player.died.emit("shot")
```

直接调用要求知道对方接口。信号只要求知道事件名。

## 与 ECS 的关系

Godot 节点树是**场景图 + 组件混合体**，不是纯 ECS。但组合优于继承的原则和 ECS 一致：**数据和行为拆开，靠组合拼装**。

## 延伸阅读

- 本站：[节点](02-node.md)、[脚本即组件](08-script-as-component.md)
- 思想篇：[DOD](../thought/01-dod.md)、[ECS](../thought/02-ecs.md)
- Robert Nystrom《Game Programming Patterns》—— Component 模式
