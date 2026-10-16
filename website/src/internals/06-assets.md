# 资产管线

`res://` 下的一切——贴图、音频、模型、场景——并不是运行时直接读原始文件。编辑器在导入阶段把源文件翻译成引擎高效读取的产物，`.import` 侧写文件记录这次翻译的全部信息。本章拆解 res:// 虚拟文件系统、导入产物的落盘结构、`preload` 与 `load` 的时机差异，以及资产之间如何互相引用。

## res:// 虚拟文件系统

`res://` 映射到工程根目录，但它的语义随运行环境不同：

- **编辑器/开发态**：`res://` = 工程目录本身，源文件与 `.godot/` 缓存都在。
- **导出态**：全部资产打包进单个 `.pck` 包，`res://` 变成包内虚拟路径——这也是导出后**不能用 `FileAccess` 直接按磁盘路径读工程文件**的原因；需按 res:// 路径读已导入产物。

## .import 文件与导入产物

对每个可导入资产，编辑器扫描时生成同名 `.import` 侧写（源码锚点 `editor/file_system/editor_file_system.cpp` 写盘段）。文件分三节：

```ini
[remap]

importer="texture"
type="CompressedTexture2D"
uid="uid://c8b3x2yq5n4v1"
path=".godot/imported/icon.svg-9db5e6a2f3.png"
valid=true

[deps]

source_file="res://icon.svg"
dest_files=[".godot/imported/icon.svg-9db5e6a2f3.png"]

[params]

compress/mode=0
...
```

| 节 | 内容 | 作用 |
|---|---|---|
| `[remap]` | importer 名、目标类型、uid、**产物路径**、valid 标志 | 运行时 `load("res://icon.svg")` 命中后**重定向**到产物路径加载 |
| `[deps]` | 源文件 + 产物清单 | 增量重导入判据 |
| `[params]` | 导入参数 | Inspector「Import」面板里调的就是它们 |

产物统一落在 `.godot/imported/`，文件名规则为 **`源文件名 + "-" + md5`**（源码 `get_import_base_path`：路径 md5 派生），扩展名由 importer 决定。整条流程：

```text
编辑器扫描 res:// → 按扩展名选 importer（texture/audio/scene/…）
 → 读 .import 的 [params] → 产出翻译后的文件进 .godot/imported/
 → 重写 .import 的 [remap]/[deps]
```

> 需要注意的是：源文件永远不被修改，`[remap]` 里的 `path` 才是运行时真正读的文件。git 里应提交源文件 + `.import`，**忽略 `.godot/`**——它随时可由 `godot --headless --import` 重建（同时生成 [uid 缓存](03-uid.md)）。

另一个扫描器规则值得知道：**`.` 开头的目录被完全跳过**（如 `.git/`、`.venv/`、自建的 `.tmp/`），里面的任何文件不参与导入、不出现在资源系统里。想要「引擎看不见的存放区」，点前缀目录就是官方途径。

## preload 与 load 的时机差异

```gdscript
const TEX := preload("res://icon.svg")    # 编译期：脚本编译时立即加载
var tex := load("res://icon.svg")         # 运行时：执行到这行才加载
```

| | `preload` | `load` |
|---|---|---|
| 参数 | 必须是**常量字符串** | 任意表达式（可拼接变量） |
| 加载时机 | 脚本编译/分析期 | 运行到该行 |
| 路径不存在 | **parse error**（分析期即红，源码 `gdscript_analyzer.cpp::reduce_preload`：常量路径归约后 `ResourceLoader::exists` 校验，无加载器或文件不存在直接报错） | 返回 null（🟡 静默失败） |
| 返回类型 | 可推断（编辑器知道目标类型） | `Resource` 基类，需自行 `as` 收窄 |

`preload` 的编译期校验意味着：**写 preload 时路径打错，编辑器立刻报错**；`load` 的错误则推迟到运行时。前者是确定性引用的首选，后者服务于「运行时才知道路径」的场景（配置驱动、动态关卡）。资源加载后由缓存统一持有——同一路径重复 `load` 返回同一份实例。

## 资产引用图

工程是**全联通**的引用网络：

```text
.tscn ──ext_resource──> .gd / .tscn / .tres / 贴图音频……
.gd ──preload/load──> 任意 res:// 资产
资源 ──uid──> 侧写文件 .uid 固定身份
```

每个引用点都是「路径 + uid」双通道（见 [UID 系统](03-uid.md)）：path 是最终事实，uid 保证文件移动后引用仍存活。删除或移动一个文件之前，先知道谁在引用它——这正是 LSP 做 `res://` 闭环校验与跨文件引用查找的原因（见[脚本致命面](07-fatal.md)）。

## 延伸阅读

- 官方英文文档：[Importing assets](https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_assets.html)、[Resource path](https://docs.godotengine.org/en/stable/tutorials/io/data_paths.html)
- 本站：[UID 系统](03-uid.md)、[Variant 序列化](05-variant.md)（`.import` 参数值的写入格式）、[脚本致命面与静默失败](07-fatal.md)
- 类参考：[ResourceLoader](../builtin/class_resourceloader.md)、[Resource](../builtin/class_resource.md)
