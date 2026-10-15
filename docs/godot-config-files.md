# Godot 非 GDScript 文件类型:高亮 / 补全 / Hover 设计与源码核验

> 范围:`editors/vscode` 对 GDScript 之外文件类型的语言支持。所有语法与键位结论
> 均核验自 `.refs/godot`(Godot 4.7.0 源码),文本语法经 `vscode-textmate` 真实
> 令牌化验证,键值数据集由 `tools/extract_project_settings.py` 从源码提取。

## 1. 文件类型总览

| 扩展名 | 语言 ID | 语法/格式权威来源(源码位置) | 高亮 | 补全 | Hover |
|---|---|---|---|---|---|
| `.gd` | gdscript | GDScript 语言服务器(主链路,另见 `docs/lsp-design.md`) | ✅ | ✅ | ✅ |
| `.tscn` `.tres` | godot-resource | `scene/resources/resource_format_text.cpp` | ✅ | 节名(白名单) | ❌ |
| `project.godot` `.godot/*.cfg` `override.cfg` `.cfg` | godot-project | `core/config/config_file.cpp` → `VariantParser::parse_tag_assign_eof` | ✅ | ✅ | ✅ |
| `.gdextension` | godot-project | 同上 + `core/extension/gdextension_library_loader.cpp` 键位图 | ✅ | ✅ | ✅ |
| `.import` | godot-project | 同上 + `editor/file_system/editor_file_system.cpp` `_reimport_file` 写入格式 | ✅ | ✅ | ✅ |
| `.uid` | godot-uid | `core/io/resource_uid.cpp`(字符集 `a-y0-8`,≤13 位) | ✅ | — | — |

**为什么 `.tscn/.tres` 与 ConfigFile 家族共用同一套词法内核**:两者最终都走
`VariantParser::parse_tag_assign_eof`(ConfigFile 是全权委托;资源文本 loader 同函数),
差别只在节头白名单与属性形态(资源格式的属性带 `identifier=Variant`,如
`groups=["a"]]` 双括号、`node_paths=PackedInt32Array(...)`)。

## 2. ConfigFile 语法要点(核验自 `core/io/variant_parser.cpp`)

| 结构 | 规则 | 高亮处置 |
|---|---|---|
| 节头 | `[` 后到第一个未转义 `]` 全为节名,支持 `\]` 转义;资源格式另有白名单 | `entity.name.tag.godot-project` |
| 键 | 任意非空白字符序列,止于第一个 `=`;引号键 `"k"=v` 合法 | `variable.other.property` |
| 注释 | `;`(行首与值后均认可) | `comment.line.semicolon.godot-project` |
| 值·字符串 | `"..."`;`&"..."`/`@"..."` 为 StringName 前缀;换行在引号内原样保留(跨物理行) | `string.quoted.double` + StringName 前缀作用域 |
| 值·数字 | 仅 `-` 号;无 `0x/0b/0o`;容许 `1.` 尾随点;指数形式 | `constant.numeric` |
| 值·颜色 | `#rgb/#rgba/#rrggbb/#rrggbbaa`(Color::html 只认 3/4/6/8 位) | `constant.other.color` |
| 值·常量 | `true/false/null/nil/inf/-inf/inf_neg/nan` | `constant.language` |
| 值·构造器 | `Vector2(..)`/`Color(..)`/`Array[Type](...)`/`Packed*`(含 3.x 别名 `Pool*`/`ByteArray` 等) | 构造器名 + 内部标识符着色 |
| 跨行 | 括号未闭合或字符串未闭合时,换行是值的一部分 | 值规则跨行延续 |

**设计取舍**:键统一着 `variable`(而非 `entity.name.tag`)——ConfigFile 的键是
「设置项标识符」,与 ini 的 section 头语义不同;节名才配 tag 作用域。颜色与
StringName 前缀单独着色,因为它们是 Godot 工程文件里最高频的两种字面量。

## 3. 补全:键 / 值两级,全部数据驱动

### 3.1 project.godot(及 override.cfg、.godot/*.cfg)

- **节名**:由数据集键的第一段派生(application/display/rendering/…),另手工补
  `[autoload]`——它不是 GLOBAL_DEF 注册的设置,而是 `project_settings.cpp:301`
  `begins_with("autoload/")` 特殊分支。
- **节内键**:给出 597 个设置键去掉节前缀的文档内形式(如 `[application]` 下补
  `config/name`)。数据集来源:`GLOBAL_DEF` 宏家族 8 个变体
  (plain/PropertyInfo 两种形态,596 个注册点),经 `tools/extract_project_settings.py`
  提取;hintString 只收严格字符串字面量(拒绝 `"512," + itos(N) + ",1"` 这类 C++
  拼接);类型按 PropertyInfo 声明或默认值字面量推断(591/597 有类型,剩余 6 个默认值
  本身是变量/运行期调用,宁缺毋滥)。
- **键的排序**:`basic`(编辑器项目设置对话框的基础分组)优先。
- **值**:`hint=enum/enum_suggestion` → 按 hintString 逗号表补候选值(91 个键);
  `type=bool` → `true/false`。已在引号内插裸值,空值插带引号值。
- **特性标签覆写键**(如 `rendering/renderer/rendering_method.mobile`)是真实注册,
  照常补全。

### 3.2 .gdextension

键位图核验自 `core/extension/gdextension_library_loader.cpp`:

| 节 | 键 | 说明 |
|---|---|---|
| `[configuration]` | `entry_symbol`(必需) | 库入口函数名 |
| | `compatibility_minimum`(必需,≥4.1.0) | 最低兼容版本 |
| | `compatibility_maximum` / `reloadable` / `autodetect_library_prefix` | 可选 |
| `[libraries]` | 特性标签(点分) | 键按 `OS::has_feature` 开放匹配,全部命中且标签最多者胜出 |
| `[dependencies]` | 特性标签 | 值为 `{源路径: 目标路径}` 字典 |
| `[icons]` | 类名 | 值为图标路径 |

`[libraries]`/`[dependencies]` 的键是开放词表(引擎不枚举),补全提供 12 个常用
约定标签(windows.x86_64 / linux.x86_64 / macos / android.arm64 / web.wasm32 /
debug / release …),hover 中如实标注「常用约定」而非引擎枚举。

### 3.3 .import

写入格式核验自 `editor_file_system.cpp` `_reimport_file`(源码注释:"Write manually,
as order matters ([remap] has to go first for performance)"):

```
[remap]
importer="texture"            importer_version=N
type="CompressedTexture2D"    uid="uid://..."
group_file="..."              path="res://.godot/imported/..."   (变体: path.<feature>=)
valid=false                    (仅导入失败时)   metadata={...}   generator_parameters={...}
[deps]
files=[...]  source_file="..."  dest_files=[...]
[params]
<importer 的 get_import_options 声明>
```

- `[remap]`/`[deps]` 键按上表补全;
- `[params]` 键**由本文件 `[remap] importer=` 决定**——读文档自身声明再查表,
  18 个 importer / 200+ 参数(核验自各 importer 的 `get_import_options()` 中
  `ImportOption(PropertyInfo(...), default)` 声明;gltf/fbx/blend 后端选项归入
  `scene` importer;单类多 mode 的 `ResourceImporterLayeredTexture` 按 switch 序
  位配对 importer 名与资源类型);
- `importer=` 值补全 18 个 importer 名并标注产物类型;`type=` 值按 importer 映射
  给出(如 texture → CompressedTexture2D)。

## 4. Hover:哪些属性需要 hover

| 位置 | 内容 |
|---|---|
| project.godot 键 | 全路径键 + 类型 + 标记(基础/需重启/无注册默认值/内部/新建工程默认)+ 范围(数值上下限与步进)或枚举候选或文件过滤器 + 默认值与新建工程默认值(两者不同时并列)+ 注册点源码文件 |
| project.godot 节名 | 节内设置项计数 |
| .gdextension 键 | 必需/可选 + 语义说明 + 来源标注 |
| .import 键 | 同上(remap 10 键 / deps 3 键 / params 按 importer) |
| 值位置 | 委托所属键的 hover(改值时也能看到枚举/范围) |

**为什么 hover 要带「注册点」**:设置键的语义散落在引擎各处,标注
`core/config/project_settings.cpp` 这类来源让用户可溯源;`restart` 标记直接来自
`GLOBAL_DEF_RST` 宏变体,是编辑器里「修改后需重启」提示的同一事实。

## 5. 已知缺口(诚实清单)

1. **`.tscn/.tres` 的节内属性键**(`name=`/`type=`/`parent=`/`script = ExtResource(...)`)
   不在本插件——属性键全集依赖引擎类数据库(节点类成员),属于另一条数据链路,
   不做半成品。当前仅节名白名单高亮。
2. **`.import` 的 `[params]` 仅覆盖内置 importer**;`EditorImportPlugin` 自定义
   插件的参数运行时才可知,静态不可穷举。
3. **`.gdshader`/`.gdshaderinc` 无语法映射**(等待专门的 shader 语法工作)。
4. **编辑器设置**(`.godot/editor/*.cfg`)不是项目设置,不在 597 键数据集内,
   不做补全(键名以 `editors/` 等前缀区分,源码核验见 `editor/editor_settings.cpp`)。
5. **`.uid` 文件**为单行 opaque 值,只做高亮校验,无补全/hover。
6. **`rendering/renderer/rendering_method` 等少数键的 enum hintString 为动态注册**,
   hover 只显示类型不列候选(宁缺毋滥)。

## 6. 验证基线

- 文本语法:`vscode-textmate` + `vscode-oniguruma` 真实令牌化真实工程样本与合成
  边界样本(转义节名、`groups=["a"]]` 双括号、`&"..."` StringName、多行
  `metadata={...}`、`.import` 三节结构)。
- 数据集:`python tools/extract_project_settings.py` 单命令重生成,产出入库
  (`editors/vscode/data/project-settings.json`,185K,Godot 4.7.0,597 设置键 +
  18 importer × params + importer→类型映射);clone 后无需 Godot 源码。
- 纯逻辑层:`node --test` 27 项(解析器语义/上下文判定/数据集查询/hover 组装/
  importer 映射回归),`tsc --noEmit` 干净,`esbuild` 打包通过。
- 冒烟:真实格式的三类样本(project.godot 含多行 `[input]` 字典、.gdextension、.import
  含跨行 metadata)走完整 Provider 调用链。
