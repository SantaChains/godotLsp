# 性能边界与模式

**GDExtension 快，但跨语言调用有成本。热路径上不要频繁跨边界。**

## 跨语言调用成本

每次 GDScript → 原生调用都经过 C ABI：参数打包成 Variant → C 函数 → 返回值解包，涉及类型转换与边界检查。单次开销不大，但被放进每帧、每实体的循环里，累计成本会吃掉原生收益；原则是**按批调用，不要逐实体跨边界**。

## 正确模式：批量进，批量出

```cpp
// 错：GDScript 循环调 C++ 单步
// for i in 100000: result += algo.step(i)

// 对：一次调用传数组，C++ 内部批量算
PackedFloat32Array MyAlgorithm::batch_simulate(
    const PackedFloat32Array& inputs) {
    // C++ 内部 SIMD/多线程
    PackedFloat32Array output;
    output.resize(inputs.size());
    for (int i = 0; i < inputs.size(); i++) {
        output[i] = expensive_compute(inputs[i]);
    }
    return output;
}
```

## 不要做的事

| 反模式 | 原因 |
|---|---|
| 每帧调一次 C++ 单步 | ABI 开销抵消收益 |
| 在 C++ 里频繁调 Godot API | 反过来也是跨边界 |
| 用 C++ 做 UI | GDScript 更合适，C++ 没优势 |
| 修改引擎内部状态 | GDExtension 不支持，用 Custom Module |

## 该做的事

| 模式 | 说明 |
|---|---|
| 批量数组处理 | PackedFloat32Array 零拷贝 |
| C++ 内部多线程 | WorkerThreadPool 或 std::thread |
| SIMD | C++ 编译器自动向量化 |
| 预分配内存 | 避免每帧分配 |
| 信号回调进度 | 长时间操作不阻塞主线程 |

## 与 Server API 配合

万级实例的视觉部分用 MultiMesh（GPU 批量），逻辑部分用 GDExtension 批量计算变换数组，传回 GDScript 设置 MultiMesh。这是 Godot 高性能方案的完整链路。

## 延伸阅读

- 本站：[DOD](../thought/01-dod.md)、[批量与结构优化](../perf/03-bulk.md)
- 官方：[GDExtension performance](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html)
