# 文件持久化

**运行期写盘只用 `user://`；`res://` 在导出后只读。** 存档、设置、配置三类需求分别对应三套 API：键值设置用 `ConfigFile`，二进制对象用 `FileAccess.store_var`，自由文本/互通用 `FileAccess` 或 JSON。

## 路径协议

| 协议 | 指向 | 运行期可写 |
|---|---|---|
| `res://` | 项目根（导出后打进 PCK/可执行文件） | 否（编辑器内可写，导出后只读） |
| `user://` | 平台专属可写目录（存档/设置唯一落点） | 是 |
| `C://`、绝对路径 | 系统真实路径 | 受平台权限限制，不推荐 |

> 把存档写进 `res://` 在编辑器里能成功，导出后静默失败——这是新手最常见的"存档不生效"。

## ConfigFile：设置项（INI 风格，首选）

音量、画质、键位这类分节键值，用 `ConfigFile`，零手写解析，自动保留类型：

```gdscript
const SETTINGS := "user://settings.cfg"

func save_settings() -> void:
    var cfg := ConfigFile.new()
    cfg.set_value("audio", "master_volume", 0.8)   # 节, 键, 值
    cfg.set_value("video", "fullscreen", false)
    cfg.set_value("keybind", "interact", "E")
    var err := cfg.save(SETTINGS)                  # 返回 Error，OK 为 0
    if err != OK:
        push_error("保存设置失败: %d" % err)

func load_settings() -> void:
    var cfg := ConfigFile.new()
    if cfg.load(SETTINGS) != OK:
        return                                     # 首次运行无文件，用默认值
    var vol := cfg.get_value("audio", "master_volume", 1.0)  # 带默认值
    var full := cfg.get_value("video", "fullscreen", false)
```

常用 API：`has_section(name)`、`get_sections()`、`get_keys(section)`、`erase_section(section)`、`erase_key(section,key)`。`save`/`load` 均返回 `Error`，必须检查，不能假设成功。

## FileAccess：自由文本与二进制

### 文本读写

```gdscript
const SAVE := "user://save.txt"

func write_text() -> void:
    var f := FileAccess.open(SAVE, FileAccess.WRITE)   # 无则新建，有则覆盖
    if f == null:
        push_error("无法打开写文件")
        return
    f.store_string("我的分数是：" + str(100))
    f.close()                               # 关闭才落盘

func read_text() -> void:
    if not FileAccess.file_exists(SAVE):
        return                              # 先检查存在，防报错
    var f := FileAccess.open(SAVE, FileAccess.READ)
    if f == null:
        return
    var content: String = f.get_as_text()   # 全文读为 String
    f.close()
    print(content)
```

### 二进制 Variant（store_var / get_var）

引擎原生类型（int/float/String/Vector2/Color/嵌套 Array/Dictionary）可直接二进制序列化，紧凑且无需格式协商：

```gdscript
func save_data(data: Dictionary) -> void:
    var f := FileAccess.open("user://save.bin", FileAccess.WRITE)
    if f:
        f.store_var(data)                   # 写整个 Variant
        f.close()

func load_data() -> Variant:
    if not FileAccess.file_exists("user://save.bin"):
        return null
    var f := FileAccess.open("user://save.bin", FileAccess.READ)
    if f == null:
        return null
    var data := f.get_var()
    f.close()
    return data
```

## 删除文件：DirAccess

`FileAccess` 不负责删除，删文件用 `DirAccess`：

```gdscript
var dir := DirAccess.open("user://")
if dir:
    var err := dir.remove("save.bin")        # 相对 user:// 的文件名
    if err != OK:
        push_error("删除失败")
```

`DirAccess` 还提供 `file_exists`、`dir_exists`、`list_dir_begin/next`、`make_dir_recursive`。

## JSON：跨格式互操作

与外部程序/网络交换文本时用 `JSON`：

```gdscript
var payload := {"score": 100, "name": "hero"}
var text := JSON.stringify(payload, "  ")    # 缩进美化
var parsed: Variant = JSON.parse_string(text)
if typeof(parsed) != TYPE_DICTIONARY:
    push_error("JSON 非法")
```

自定义 Resource 的存盘走 `ResourceSaver.save` / `ResourceLoader.load`，见[资源 Resource](../paradigm/07-resource.md)。

## 选型对照

| 需求 | 方案 | 理由 |
|---|---|---|
| 音量/键位/画质等分节设置 | ConfigFile | INI 分节、自动类型、零解析 |
| 游戏存档（原生类型嵌套结构） | FileAccess.store_var/get_var | 紧凑二进制，无需格式协商 |
| 可读文本日志/导出 | FileAccess.store_string | 人可读 |
| 与外部程序/网络交换 | JSON | 通用文本格式 |
| 自定义游戏对象 | ResourceSaver/.tres | 类型安全，编辑器可预览 |

## 陷阱

- 写 `res://`：编辑器成功、导出失败，永远写 `user://`。
- 忘记 `close()`：缓冲未落盘。
- 不检查 `open` 返回 null：路径无权/被占用时静默跳过。
- 直接读不存在的文件：先 `FileAccess.file_exists`。
- ConfigFile 的 `save`/`load` 返回 Error 被忽略：磁盘满、权限不足时无任何提示。

## 延伸阅读

- 官方：[FileAccess](../builtin/class_fileaccess.md)、[ConfigFile](../builtin/class_configfile.md)、[DirAccess](../builtin/class_diraccess.md)、[JSON](../builtin/class_json.md)
- 本站：[资源 Resource](../paradigm/07-resource.md)、[错误处理模型](14-errors.md)
