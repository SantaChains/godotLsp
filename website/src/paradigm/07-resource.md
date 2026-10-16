# 资源 Resource

**资源是与节点分离的数据容器。可被多节点共享，保存为 .tres/.res 文件。**

## 节点 vs 资源

| | Node | Resource |
|---|---|---|
| 用途 | 逻辑、渲染、物理 | 数据、配置、资产 |
| 树结构 | 是 | 否 |
| 生命周期 | 进入/离开场景树 | 引用计数，无引用自动释放 |
| 文件 | .tscn | .tres/.res |

## 共享资源

```gdscript
# 三个敌人共享同一个 AnimationLibrary
# 改一处，三处生效
var anim_lib = preload("res://enemy_anim.tres")
```

## 自定义资源

```gdscript
# 定义自定义资源类
class_name ItemData extends Resource
@export var item_name: String
@export var damage: int
@export var icon: Texture2D
```

在编辑器里可以创建 `.tres` 文件，配置 ItemData 的字段。

## 延伸阅读

- 本站：[场景](03-scene.md)
- 官方：[Resources](https://docs.godotengine.org/en/stable/getting_started/step_by_step/resources.html)
