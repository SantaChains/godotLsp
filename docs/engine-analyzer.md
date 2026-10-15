# godot engine source code analyzer

> **定位**:从 Godot 源码(`.refs/godot`,4.7-stable)提取「AI 程序化生成合法工程」所需的引擎内部逻辑,
> 每条规格**仅凭本文即可脱离 Godot 复刻**——但保持极简,不复刻不值得复刻的部分。
> 编辑侧安全规则见 `godot-editing-rules.md`(R4「uid 不要手写」针对人工改文件;本文是其**程序化生成例外**的规格)。
> 源码锚点:`core/io/resource_uid.cpp`(444 行,全文核验 2026-10-16);
> `scene/resources/resource_format_text.cpp`、`scene/resources/packed_scene.cpp`、
> `scene/main/node.cpp`、`core/variant/variant_parser.cpp`、`core/config/project_settings.cpp`、
> `main/main.cpp`(2026-10-16 关键路径核验)。
>
> **致命度标记**(全文通用):🔴 = 拒载/拒开(引擎返回 ERR,场景或工程加载失败);🟡 = 降级(WARN + 占位替换或回退,功能受损但能跑);⚪ = 静默(无任何报错,结果悄悄不对)。

## UID(程序化生成合法资源标识)

### U1 编码算法(可精确复刻)

Godot UID = **63 位非负整数**的 base-34 文本编码,前缀 `uid://`。

- **字母表 34 字符,有历史缺陷(GH-83843,不可修)**:`a..y` 共 25 个字母(**无 `z`**)+ `0..8` 共 9 个数字(**无 `9`**)。
  - 字符值:字母 `c-'a'` → 0..24;数字 `c-'0'+25` → 25..33。
- **编码**(`id_to_text`,resource_uid.cpp:55):对 id 循环取模 34 得低位字符,**先得低位后得高位**,输出时反转。最大 id `0x7FFFFFFFFFFFFFFF` 编码为 13 字符。
- **解码**(`text_to_id`,:92):`uid = uid*34 + char_value`,从 `uid://` 之后逐字符;遇非法字符或 `z`/`9` → INVALID_ID(-1);最终 `& 0x7FFFFFFFFFFFFFFF`。
- **极简复刻(任何语言 ≤20 行)**:

```
ALPHABET = "abcdefghijklmnopqrstuvwxy012345678"   # 34 字符,无 z 无 9
fn encode(id: u64) -> String:            # id 必须 < 2^63
    s = ""
    loop { s = ALPHABET[id % 34] + s; id /= 34; if id == 0 { break } }
    return "uid://" + s
fn decode(text: String) -> u64:          # 无 "uid://" 前缀或含非法字符 → 无效
    v = 0
    for c in text[6..]: v = v*34 + char_value(c)   # char_value 见上
    return v & 0x7FFFFFFFFFFFFFFF
```

### U2 生成(极简方案 = CSPRNG 随机)

- 引擎原生 `create_id`(:113)就是**加密随机 8 字节 → `& 0x7FFFFFFFFFFFFFFF` → 查重重试**。没有结构、没有时间戳、没有路径派生。
- **程序化生成等价实现**:`id = CSPRNG_u64() & 0x7FFFFFFFFFFFFFFF`,再 encode。63 位空间 + 项目资源量(典型 10^3~10^5)使碰撞概率可忽略;工程内**生成器自己维护已发 id 集合**保证本地唯一即可(引擎对重复 id 的注册是 `ERR_FAIL_COND` 拒绝,resource_uid.cpp:164)。
- `create_id_for_path`(:134)是**路径确定性派生**(seed = `project_name.hash64() * path.lower().hash64() * md5(path).hash64()` + Godot 私有 `RandomPCG`),复刻需移植 Godot 的 `String::hash64` 与 PCG 变体——**不值得,标记为不复刻**(远期若需「同路径同 uid 的确定性管线」再立项,见路线图 X2)。

### U3 绑定:引擎如何接受外部生成的 uid

三份存储,职责不同,**外部工具只允许碰前两份**:

| 存储 | 内容 | 谁写 | 外部工具 |
|---|---|---|---|
| `<资源名>.uid` 侧写文件(4.4+) | 单行 `uid://xxxxxx` | 编辑器扫描时为缺侧写的资源生成;**已有侧写则原样信任** | ✅ 可写(这是官方外部注册通道) |
| `.tscn`/`.tres` 的 `ext_resource uid="uid://…"` | 引用声明 | 编辑器保存 | ✅ 可写(格式合法即可,见 U1) |
| `.godot/uid_cache.bin` | id→path 全量映射 | **引擎扫描生成**(`--import`),格式:u32 条数 + 每条(u64 id + u32 len + UTF-8 path) | ❌ 禁止手写(godot-editing-rules R3′) |

**绑定流程(AI 管线,零引擎参与生成、一次导入注册)**:
1. 生成 id → encode(U1/U2),维护工程内已发集合;
2. 为资源写 `<file>.uid` 侧写(单行,无 BOM 无多余空白);
3. 引用处写 `uid="uid://…"`(或按 editing-rules R4 干脆省略 uid 只写 path——引擎照样加载);
4. 跑一次 `godot --headless --path <工程> --import`:扫描器读侧写 → `add_id` 注册 → 写 `uid_cache.bin`;
5. 验证门:`godot_verify.py check`(editing-rules §0),无 `Unrecognized UID` / `invalid UID` 告警即合法。

**加载语义**(外部生成的兜底保障):`ext_resource` 的 uid 缺失或无效**只 WARN 不致命**,回退按 `path` 加载(resource_format_text.cpp:501/504)——即 AI 管线哪怕 uid 全错,只要 path 正确,工程依然可加载。这是整个管线的安全网。

### U4 不变量(复刻实现的验收清单)

- 任意合法 id 经 encode→decode 恒等;decode 拒绝 `z`/`9`/大写/负数前缀格式;
- encode 输出长度 1..13,首字符可为字母或数字;
- 同一工程生成器发出的 id 集合无重复;
- `.uid` 侧写内容 = `"uid://" + encode(id)` 单行;
- `--import` 后 `uid_cache.bin` 可被 U1 的格式定义解析(条数 + id/len/path 三元组)。

## 节点(场景节点图的完整规则)

### N1 `[node]` 标签:字段表与致命面

解析锚点 `resource_format_text.cpp::_parse_node_tag`(:184);实例化锚点 `packed_scene.cpp::SceneState::instantiate`(:200 起)。字段表:

| 字段 | 必填 | 语义 | 缺失/非法后果 |
|---|---|---|---|
| `name` | 是 | 节点名 | 缺失时后续以空名注册(用同名兄弟会 🟡 编辑器改名) |
| `parent` | 首节点禁用,其余必填 | NodePath,**相对场景根** | 首节点带 parent → 🔴 `Invalid scene: root node X cannot specify a parent node`(packed_scene.cpp:221,即 editing-rules R6-1);其余节点缺失 → 🔴 `Invalid scene: node X does not specify its parent node`(:209) |
| `type` | 与 `instance` 二选一 | C++ 类名(经 ClassDB) | 缺失 → 该节点按「实例化占位」处理;类型不存在 → 🟡 WARN `cannot be created. A placeholder will be created instead`,按父节点类型降级 Control/Node2D/Node3D,兜底 Node(:335-350) |
| `instance` | 与 `type` 二选一 | ExtResource id(指向 PackedScene) | 场景加载失败 → 🔴 `Scene instance is missing.`(:278,运行时;编辑器有 MissingNode 兜底) |
| `owner` | 否 | NodePath(相对根) | 缺省:非根节点 owner 自动 = 根(`owner = 0`,resource_format_text.cpp:265-267) |
| `index` | 否 | 兄弟排序下标 | 超界忽略 |
| `groups` | 否 | 字符串数组 | 无致命面 |
| `instance_placeholder` | 否 | ExtResource id(编辑器占位实例) | 用于继承根 → 🔴 `Instance Placeholder can't be used for inheritance`(:250) |

**结构硬约束**(程序化生成器必须保证,违一即拒载):
1. 第一个 `[node]` 必须是无 `parent` 的根节点;
2. 根节点之后每个 `[node]` 必须有 `parent`,且 parent 路径指向**已在它之前声明的节点**(实例化按声明序线性进行,前向引用 → parent 解析失败);
3. `type` 与 `instance` 互斥(编辑器保存的文件两者都不出现 = 实例化场景的内部节点,程序化生成时**必须写 type 或 instance 之一**)。

**其他标签的致命面**(同函数核验):
- `[ext_resource]` 缺 `id` → 🔴 ERR_FILE_CORRUPT(:480-485);缺 `type`/`path` 同理;引用资源不存在:abort_on_missing 开(编辑器默认)→ 🔴 拒载,关(运行时)→ 🟡 依赖错误通知继续;
- `[sub_resource]` 缺 `type` 或 `id` → 🔴 ERR_FILE_CORRUPT(:552-563);
- `[editable]` 缺 `path` → 🔴 ERR_FILE_CORRUPT(:391-397);文件头:首个 tag 非 `gd_scene`/`gd_resource` → 🔴 `Unrecognized file type`(:1180-1184);`gd_resource` 缺 `type` → 🔴(:1167-1171);`format` > 引擎 FORMAT_VERSION → 🔴 `Saved with newer format version`(:1153-1156,缺 format 字段则视为最新版,兼容)。

### N2 parent 路径解析(可精确复刻)

- 解析时引擎对 parent 值调 `np.prepend_period()`(resource_format_text.cpp:208)——即 tscn 里写的相对路径被**前置 `.`** 变成「相对根」;因此 tscn 中 `parent="."` = 根的直接子节点,`parent="Player/Hand"` = 根下 Player 的子节点。**根节点名不出现在任何 parent 路径中**。
- 路径段匹配按节点**名字**(非类型、非 uid);`..` 上溯、`%Name` 唯一名在场景文件 parent 中**无效**(SceneState 只存整数索引化路径)。
- 实例化时 parent 路径失效(引用的中间节点被删):DEBUG 构建 WARN + 挂根;非 DEBUG 构建 → 该节点进 `stray_instances` 被 **memdelete 静默删除**(packed_scene.cpp:548-551)→ ⚪ 节点消失。生成器按「parent 必须是前文已声明的直系/祖先链」生成即天然规避。

### N3 owner 与 `%Name` 唯一名(get_node 逻辑,可精确复刻)

`Node::get_node_or_null`(node.cpp:1904)是 `$`/`get_node`/信号 `from`/`to` 的共同底座:

- 相对路径从**当前节点**开始;绝对路径(以 `/` 开头)从场景树根开始,且**不在树内的节点用绝对路径直接返回 null**(node.cpp:1910)。
- `.` = 自身;`..` = 父(根的 `..` → null);每段按名字在子节点哈希表中查找,失配即整条路径 null。
- **`%Name` 唯一名**:名字带 `%` 前缀(`UNIQUE_NODE_PREFIX`,string_name.h:36)时,先查当前节点的 `owned_unique_nodes`,查不到再查**当前节点 owner** 的表(node.cpp:1942-1950)。注册逻辑:
  - `unique_name_in_owner = true` 的节点在**设置 owner 时**注册进 owner 的表(node.cpp:2234-2246);
  - **owner 为空(孤儿节点)的 `%Name` 永远查不到**(注册前 `ERR_FAIL_NULL(data.owner)`)→ ⚠ 生成器必须保证每个节点 owner 指向场景根(或依赖引擎缺省 owner=根,见 N1);
  - 同一场景内两个节点同 `%` 名:先注册者赢,后者被**取消 unique 标记**并 WARN(node.cpp:2238-2243)→ 🟡 `%Name` 解析到旧节点,静默错绑。

**GDScript `$`/`%` 语法层**(锚点 `gdscript_parser.cpp::parse_get_node`,:3582-3689;tokenizer :1448 `$`=DOLLAR):

- `$Path` = `get_node("Path")` 语法糖;`%Name` = `get_node("%Name")`;混合合法:`$Path/%Name`、`$"字面 路径"`、`$%Name`;
- **`%` 只能出现在路径段开头**(紧跟 `$` 或 `/` 之后,:3620-3624)——`A%B` / 中缀 `%` 是取模运算(tokenizer PERCENT 分支按 PREC_FACTOR 归约);
- **`/` 只能出现在开头或节点名后**(:3629-3633);段名须是合法节点名字符或字符串字面量;
- 语法错误(`$` 后无路径、字面量非字符串)→ 脚本 parse error,`$` 路径在**脚本编译期不做存在性检查**——指向不存在节点仍是运行时错误(见 P2)。

### N4 不变量(节点生成验收清单)

- 第一个 `[node]` 无 parent;其余节点 parent 均为前文已声明的祖先链路径;
- 每个节点 `type` 是引擎内置类名或已注册的 `class_name`(否则占位降级,editing-rules R5);
- 同一父下无重名子节点(编辑器会改名,运行时 `_add_child_nocheck` 不查重 → ⚪ 同名并存但按名查找永远命中第一个);
- 需 `%` 引用的节点必须有 `unique_name_in_owner = true` 且整个场景该名字唯一;
- owner 缺省即可(引擎自动 = 根),显式 owner 只在挂到实例化场景内部时才需要——程序化生成**不写 owner**。

## 信号(连接的完整规则)

### S1 `[connection]` 标签:四字段缺一即拒载

解析锚点 `resource_format_text.cpp::_parse_node_tag`(:311-390)。

| 字段 | 必填 | 语义 |
|---|---|---|
| `signal` | **是,缺失 → 🔴 ERR_FILE_CORRUPT** `missing 'signal' field from connection tag` | 发射的信号名 |
| `method` | **是,同上** | 目标方法名(接收者脚本内) |
| `from` | **是,同上** | 发射者 NodePath(相对场景根,存时 `.simplified()`) |
| `to` | **是,同上** | 接收者 NodePath(同上) |
| `flags` | 否 | 默认 `CONNECT_PERSIST`(=3 即 persist);程序化生成**写默认不写字段** |
| `binds`/`unbinds` | 否 | 绑定参数/解绑个数;生成器不用 |

> 对比:节点级字段缺失走 WARN/占位,而 connection 四字段缺失是**硬拒绝**——这是 AI 生成最容易踩的 🔴 之一。

### S2 连接语义(实例化期行为)

锚点 `packed_scene.cpp`(:649-683):

1. `from`/`to` 各自 `simplified()`(去双斜杠与空段)后按 N2 规则解析为节点;
2. **任一端节点找不到 → `continue`,连接静默丢弃**(:662-664)→ ⚪ 信号永不触发且无报错;
3. 端点存在但 `signal` 名在发射者类型上不存在、或 `method` 不存在于接收者 → `connect()` 运行时返回错误(ERR 打印),连接失败 → 🟡(崩溃日志里有 `Method expected`/`Signal ... not found`);
4. 连接在**所有节点实例化完成之后**才执行(节点先全部建好,再统一连线),所以 tscn 中 `[connection]` 引用文件中后声明的节点是合法的(与 parent 不同,无前向引用限制);
5. `to` 节点脚本还没 attach 好不会发生——脚本作为节点属性在连接前已设置完毕。

**生成器规则**:from/to 用相对根的最短路径(或 `%Name`);signal/method 必须逐字符匹配源码里 `signal X` 声明与 `func X` 定义(GDScript 大小写敏感);生成后用 godotlsp 自身查 `signal` 存在性 + 目标脚本 `method` 存在性做静态验证门。

### S3 不变量

- 四字段齐备;from/to 是场景内已声明节点的合法相对路径;
- signal 是 from 节点类型的真实信号(内置或脚本声明);method 是 to 节点脚本的真实方法名(参数签名不匹配仅 WARN 不拒连);
- 同一 (from, signal, to, method) 组合在文件中唯一(重复连接 = 信号触发两次回调,⚪ 不报错)。

## 字符串与 Variant 序列化(tscn 值的写入格式)

### V1 字符串转义(双向精确复刻)

解析端锚点 `variant_parser.cpp::get_token`(:277-414);写出端锚点 `String::c_escape_multiline`(ustring.cpp:4490,由 `VariantWriter::write` STRING 分支调用,variant_parser.cpp:2034)。

**解析端接受的转义集**(`\` 后跟):
| 序列 | 结果 | 序列 | 结果 |
|---|---|---|---|
| `\b` | 退格(8) | `\n` | 换行(10) |
| `\t` | 制表(9) | `\f` | 换页(12) |
| `\r` | 回车(13) | `\uXXXX` | 4 位十六进制码点 |
| `\"` `\\` | 字面 `"` `\`(经 default 分支) | `\UXXXXXX` | 6 位十六进制码点(含代理外区) |

- **未知转义 `\x` → 字面 `x`**(default 分支 :351-353)——宽容处理,不报错;
- `\u` 高位代理 + `\u` 低位代理自动合并成完整码点(:356-375);不配对 → 🔴 `ERR_PARSE_ERROR`;
- **未闭合字符串(EOF 前 `"` 未归)→ 🔴 ERR_PARSE_ERROR,整个文件拒载**;
- 字符串内**原始换行合法**(解析逐字符进行,`"` 才终止)——引擎自己保存时也这么写。

**写出端极简规则**(生成器照此转义,两条足矣):
```
s.replace("\\", "\\\\").replace("\"", "\\\"")
```
换行原样保留(不转义),其余字符原样。解析端宽容 + 写出端保守,双向闭环成立。

### V2 常用 Variant 构造串(值语法)

`tscn` 属性值 = `Type(field, field, ...)` 构造串,生成器高频子集:

| 类型 | 写法示例 |
|---|---|
| int/float | `5` / `1.5`(float 可带 `e` 指数;`inf`/`nan` 特殊记号) |
| bool | `true` / `false` |
| String | `"..."`(按 V1) |
| StringName | `&"..."` |
| NodePath | `NodePath("Player/Hand")` |
| Vector2/3 | `Vector2(1, 2)`(逗号空格分隔,浮点成员用 `rtos_fix` 规则) |
| Color | `Color(r, g, b, a)` |
| Rect2 | `Rect2(x, y, w, h)` |
| Transform2D | `Transform2D(6 个数,列主序)` |
| Array | `[v1, v2, ...]` |
| Dictionary | `{"k": v, ...}` |
| ExtResource 引用 | `ExtResource("1_abcde")`(id 字符串) |
| SubResource 引用 | `SubResource("Rectangle2")` |
| Packed*Array | `PackedInt32Array(1, 2)` / `PackedFloat32Array(...)` / `PackedStringArray(...)` 等 |

- 节点属性中的 NodePath 属性(`node_paths` 字段声明过的)在实例化期**延迟解析**:值先存字符串,节点全部建好后 `get_node_or_null` 转成真节点(packed_scene.cpp:639)——节点不存在则置 null → ⚪。
- float 输出建议保留一位以上小数(`1.0` 非 `1`),整数型构造(int 字段)写裸整数——`rtos_fix` 的兼容规则写法,避免 i/f 后缀问题。

## 工程生成

### P1 project.godot:致命面与最小模板

锚点 `project_settings.cpp::_load_settings_text`(:955-995)。

- **`config_version`**:必须 `<=` 引擎 CONFIG_VERSION(4.x = 5);更大 → 🔴 `ERR_FILE_CANT_OPEN` **整个项目打不开**(:983);缺失视为 0,兼容。生成器写 `config_version=5`。
- **任何 parse 错误**(坏引号、坏枚举值)→ 🔴 `Error parsing 'project.godot' at line N ... File might be corrupted`(:978)——editing-rules R1/R2 的根源。
- **`config/features`**:含不受支持 feature → 项目管理器弹 unsupported 警告,但**不拒开**(:119-133)→ 🟡。生成器写 `PackedStringArray("4.7", "Forward Plus")`。
- **主场景**:`application/run/main_scene` 为空且启动参数无场景 → 运行报错退出 `Can't run project: no main scene defined`(main.cpp:2337-2344)。生成器必写。
- **autoload**(锚点 project_settings.cpp:347-370 + main.cpp:4499-4545):
  - 键 `autoload/名字`,值 `*res://path.gd` → `*` 前缀 = **单例**(注册全局常量,脚本里直接用名字);无 `*` = 普通 autoload 节点;
  - 脚本 autoload **必须继承 Node**(基类经 `get_instance_base_type` 校验),否则该 autoload 被 `ERR_CONTINUE` **跳过**→ 🟡 全局常量缺失、下游 `Identifier not found` 假错误;
  - autoload 路径无效/文件缺失 → 同样 ERR_CONTINUE 跳过(运行时),编辑器则显式红;
  - 顺序 = 文件内出现顺序,autoload 之间按序加载(先到者全局可用)。

最小合法模板(4.7,已按上游源码核验):

```ini
; Engine configuration file.
config_version=5

[application]
config/name="GeneratedProject"
run/main_scene="res://scenes/main.tscn"
config/features=PackedStringArray("4.7", "Forward Plus")

[autoload]
GameState="*res://scripts/game_state.gd"

[rendering]
renderer/rendering_method="forward_plus"
```

### P2 脚本致命面(运行期)

- `class_name X` 全局类注册依赖 `--import` 缓存(editing-rules R5)——程序化生成后**必须先跑一次 import**再验证;
- `@onready` 在 `_ready` 前求值,`$` 路径此时才解析——tscn 层级错误(`$Path` 指向不存在节点)在**运行时**才炸(`Node not found`),静态验证必须由 godotlsp 的节点树解析兜住(本文 N 节的规则即验证器规格);
- `extends` 指向不存在脚本/不继承预期基类 → 脚本 parse 失败,节点脚本空挂 → 🟡。

### P3 不变量

- `config_version=5` + features 写实际目标引擎版本;
- 主场景文件存在且可被本规格 N/S 节验证通过;
- 所有 autoload 脚本 `extends Node`(或子类),路径真实存在,`*` 前缀只在需要全局常量时加;
- 所有 `res://` 引用(preload/ext_resource/autoload)目标文件存在于生成文件清单(可静态闭环校验,无需引擎)。

## 管线(生成 → 验证闭环)

1. **生成**:按本规格产出 project.godot + .tscn/.tres(N/S 规则)+ .gd + `<file>.uid` 侧写(U3);字符串一律按 V1 转义;
2. **静态验证(零引擎)**:res:// 闭环引用检查 + tscn 结构四查(N1 硬约束/S1 四字段/节点名唯一/`%` 名唯一)+ gdscript-engine parse(extends/signal/method 存在性);
3. **注册**:一次 `godot --headless --path <工程> --import`(生成 .godot 缓存:uid_cache.bin + class_name 全局类缓存);
4. **引擎门**:`godot_verify.py check`(editing-rules §0)抓 import/运行 stderr 的 ERR/WARN;exit code 不可信(exit=0 也可能有错);
5. **迭代**:4 报错 → 回 1 修复,同文件幂等重写(生成器必须支持整目录重建,禁手改产物)。
