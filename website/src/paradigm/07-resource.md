# 资源 Resource

**资源是与节点分离的数据容器。可被多节点共享，保存为 .tres 文件。**

## 节点 vs 资源

| | Node | Resource |
|---|---|---|
| 用途 | 逻辑、渲染、物理 | 数据、配置、资产 |
| 树结构 | 有父子关系 | 扁平 |
| 生命周期 | 进入/离开场景树 | 引用计数，无引用自动释放 |
| 文件 | .tscn | .tres/.res |
| 编辑器可视 | 场景树里看到 | Inspector 里编辑数据 |

## 共享资源

```gdscript
# 三个敌人共享同一个 AnimationLibrary
# 改一处，三处生效
var anim_lib = preload("res://enemy_anim.tres")
```

如果敌人各自复制一份，改一个敌人的动画不影响其他。共享资源改一处全变。

> 反向注意：共享是同一对象，运行时改一个实例的字段会波及所有共享者。每个敌人需要独立血量/外观时，用 `duplicate()`（浅拷贝）复制一份再改；纯只读配置（武器数值表）才直接共享。

## 自定义资源

```gdscript
# item_data.gd
class_name ItemData extends Resource
@export var item_name: String
@export var damage: int
@export_range(1, 99) var price: int
@export var icon: Texture2D
@export_multiline var description: String
```

在编辑器里：
1. 右键文件系统 → New Resource → 选 ItemData
2. 保存为 `res://items/sword.tres`
3. 在 Inspector 里填字段

运行时加载：

```gdscript
var sword: ItemData = preload("res://items/sword.tres")
print(sword.item_name)  # "铁剑"
print(sword.damage)     # 15
```

## 运行时创建与保存

```gdscript
# 运行时创建
var data := ItemData.new()
data.item_name = "临时药水"
data.damage = 5

# 保存到磁盘
ResourceSaver.save(data, "res://items/potion.tres")

# 加载
var loaded = load("res://items/potion.tres") as ItemData
```

## 内置资源类型

| 资源 | 用途 |
|---|---|
| Texture2D | 贴图（.png/.svg 导入产物） |
| AudioStream | 音频（.wav/.ogg） |
| AnimationLibrary | 动画库 |
| TileSet | 瓦片集 |
| Curve | 数值曲线 |
| Gradient | 颜色渐变 |
| FontFile | 字体 |

## 延伸阅读

- 本站：[场景](03-scene.md)、[资产管线](../internals/06-assets.md)
- 官方：[Resources](https://docs.godotengine.org/en/stable/getting_started/step_by_step/resources.html)
