# C++ 环境搭建

**godot-cpp 绑定仓库 + SCons 构建 + .gdextension 配置文件。**

## 项目结构

```
my_extension/
├── godot-cpp/          # 官方 C++ 绑定（git submodule）
├── src/
│   ├── register_types.cpp
│   └── my_algorithm.h
├── demo/               # Godot 项目
│   └── bin/
│       ├── libmy_extension.windows.editor.x86_64.dll
│       └── my_extension.gdextension
└── SConstruct
```

## .gdextension 文件

```ini
[configuration]
entry_symbol = "my_extension_init"
compatibility_minimum = "4.2"

[libraries]
windows.editor.x86_64 = "res://bin/libmy_extension.windows.editor.x86_64.dll"
linux.debug.x86_64 = "res://bin/libmy_extension.linux.template_debug.x86_64.so"
macos.debug = "res://bin/libmy_extension.macos.template_debug.framework"
```

| 字段 | 含义 |
|---|---|
| `entry_symbol` | C 入口函数名，在 register_types.cpp 定义 |
| `compatibility_minimum` | 最低兼容 Godot 版本 |
| `libraries` | 各平台二进制路径 |

## 构建

```bash
# 初始化 godot-cpp submodule
git submodule add https://github.com/godotengine/godot-cpp

# 构建
scons platform=windows target=editor
```

## 最小 C++ 类

```cpp
// my_algorithm.h
#include <godot_cpp/classes/node.hpp>
using namespace godot;

class MyAlgorithm : public Node {
    GDCLASS(MyAlgorithm, Node)
protected:
    static void _bind_methods();
public:
    double fast_simulate(int count);  // 你暴露给 GDScript 的方法
};
```

```cpp
// register_types.cpp
#include <godot_cpp/core/class_db.hpp>
#include "my_algorithm.h"

void register_my_types() {
    ClassDB::register_class<MyAlgorithm>();
}

void unregister_my_types() {}

extern "C" void my_extension_init() {
    initialize_module(register_my_types, unregister_my_types, true);
}
```

## 延伸阅读

- 本站：[暴露类与方法](03-binding.md)
- 官方：[GDExtension C++ example](https://docs.godotengine.org/en/stable/tutorials/scripting/cpp/gdextension_cpp_example.html)
