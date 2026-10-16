# 暴露类与方法

**_bind_methods() 把 C++ 函数注册到 Godot 类型系统。GDScript 即可调用。**

## 绑定方法

```cpp
void MyAlgorithm::_bind_methods() {
    ClassDB::bind_method(
        D_METHOD("fast_simulate", "count"),
        &MyAlgorithm::fast_simulate
    );
    ClassDB::bind_method(
        D_METHOD("set_speed", "value"),
        &MyAlgorithm::set_speed
    );
    ClassDB::bind_method(
        D_METHOD("get_speed"),
        &MyAlgorithm::get_speed
    );
    ADD_PROPERTY(PropertyInfo(Variant::FLOAT, "speed"),
        "set_speed", "get_speed");
}
```

## GDScript 侧

```gdscript
extends Node

var algo := MyAlgorithm.new()
var result: float = algo.fast_simulate(100000)
print(result)
algo.free()  # RefCounted 自动管理，Node 需要手动
```

## 内存管理

| 基类 | 管理方式 |
|---|---|
| `RefCounted` | 引用计数，自动释放 |
| `Node` | 手动 `free()`，或进场景树后 `queue_free()` |

> 值得注意的是：**纯计算类继承 RefCounted，不要继承 Node**。Node 有场景树开销，纯算法不需要。

## 信号绑定

```cpp
// C++
ADD_SIGNAL(MethodInfo("progress",
    PropertyInfo(Variant::INT, "percent")));

// emit
emit_signal("progress", 50);
```

```gdscript
# GDScript
algo.progress.connect(func(p): print("进度: ", p))
```

## 数组与字典

```cpp
// 接收 PackedFloat32Array（零拷贝，连续内存）
void process(const PackedFloat32Array& data);

// 返回 TypedArray
TypedArray<Vector3> get_positions();
```

## 延伸阅读

- 本站：[性能边界](04-boundaries.md)
- 官方：[ClassDB](https://docs.godotengine.org/en/stable/classes/class_classdb.html)
