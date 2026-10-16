# UID 系统

Godot 里每个资源有两重身份：**路径**和 **UID**。路径会随文件移动、重命名而失效，UID 则跟随文件本身不变。`uid://` 就是这套稳定标识的文本形式。本章拆解它的编码算法、生成方式、三份存储，以及最关键的一条兜底语义——uid 错了为什么工程照样能跑。

## 编码算法：base-34

一个 UID 本质是 **63 位非负整数**，`uid://` 前缀后跟它的 base-34 文本编码。字母表有 34 个字符，但**没有 `z` 也没有 `9`**——这是历史缺陷（GH-83843），因为兼容已发布工程而永久保留：

```text
字母表: a b c ... x y   0 1 2 ... 7 8
值:     0 1 2 ... 23 24 25 26 ... 32 33
```

- **编码**（源码 `id_to_text`，`core/io/resource_uid.cpp`）：对 id 循环取模 34 取低位字符，先得低位后得高位，输出时反转；最大合法 id 编码为 13 字符。
- **解码**（`text_to_id`）：逐字符 `uid = uid * 34 + char_value`，遇到 `z`、`9`、大写或非法字符 → 判定无效（INVALID_ID）。
- 任何语言复刻 ≤ 20 行：

```text
ALPHABET = "abcdefghijklmnopqrstuvwxy012345678"   # 34 字符，无 z 无 9
encode(id): 循环 id % 34 取字符、id /= 34，逆序拼接，前缀 "uid://"
decode(s):  从前缀之后逐字符 v = v*34 + char_value(c)，最终 & 0x7FFFFFFFFFFFFFFF
```

## 生成：加密随机 + 查重

引擎的 `create_id` 没有任何结构：**加密安全随机 8 字节 → 屏蔽符号位得 63 位 → 查重重试**。没有时间戳、没有路径派生。63 位空间配合工程典型的资源量（10³~10⁵），碰撞概率可忽略；引擎侧对重复 id 的注册是硬拒绝，所以生成工具自己维护「已发 id 集合」保证本地唯一即可。

## 三份存储

| 存储 | 内容 | 谁写 | 外部工具能否写 |
|---|---|---|---|
| `<资源名>.uid` 侧写文件（4.4+） | 单行 `uid://xxxxxx` | 编辑器扫描时为缺侧写的资源生成；**已有侧写原样信任** | 可以——这是官方外部注册通道 |
| `.tscn`/`.tres` 里的 `ext_resource uid="uid://…"` | 引用声明 | 编辑器保存 | 可以，格式合法即可 |
| `.godot/uid_cache.bin` | id→path 全量映射缓存 | **引擎扫描生成**（`--import`） | 禁止手写 |

`.uid` 侧写的工作方式：编辑器发现某脚本/场景/资源没有配套 `.uid` 文件时，生成一个随机 uid 写进去；**文件已存在则直接采用**。4.4 引入这套侧写，正是为了让原先不支持内嵌 uid 的类型（脚本、shader）也纳入 uid 体系——官方定位是「随主文件被追踪的元数据文件」，保存脚本时自动生成。这就是为什么把 `.uid` 提交进 git 能保持 uid 稳定——不提交的话，同事 clone 或 CI 检出后重新扫描会生成**不同的** uid，所有引用随之漂移。

uid 在脚本里也是一等公民：`load("uid://c8b3x2yq5n4v1")` 是合法加载方式——文件在工程内随便移动，脚本不用改一行。代价是可读性归零，实践上 uid 引用适合机器维护（编辑器、LSP），人类书写仍以 path 为宜。

## 兜底语义：uid 失效只 WARN 回退 path

加载 `ext_resource` 时，uid 缺失或无效**只打印 WARN，不拒载**——引擎回退按 `path` 字段加载（源码锚点 `resource_format_text.cpp`）。即使 uid 全错，只要 path 正确，工程依然可加载。这是整条资源链路的安全网：uid 是加速与稳定引用通道，path 才是最终事实。

> 值得注意的是：反方向不成立——**只有 uid 没有 path 的引用无法回退**。写引用时保证 path 正确永远是第一位，uid 只是让引用在文件移动后依然存活。

```ini
[ext_resource type="Script" uid="uid://c8b3x2yq5n4v1" path="res://player.gd" id="1_abcd"]
```

## 延伸阅读

- 官方英文文档：[ResourceUID](https://docs.godotengine.org/en/stable/classes/class_resourceuid.html)、[UID changes coming to Godot 4.4](https://godotengine.org/article/uid-changes-coming-to-godot-4-4/)（侧写文件的官方公告）、[Importing assets](https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_assets.html)
- 本站：[资产管线](06-assets.md)（`.import` 与导入产物）、[场景文件 .tscn 解析](02-tscn.md)（`ext_resource` 在场景中的角色）
- 类参考：[Resource](../builtin/class_resource.md)、[ResourceLoader](../builtin/class_resourceloader.md)
