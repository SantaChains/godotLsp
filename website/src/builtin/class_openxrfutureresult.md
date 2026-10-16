# OpenXRFutureResult

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRFutureResult**

> Result object tracking the asynchronous result of an OpenXR Future object, you can use this object to track the result status.

<details><summary>官方英文描述</summary>

Result object tracking the asynchronous result of an OpenXR Future object, you can use this object to track the result status.

</details>

## 方法

### cancel_future {#cancel_future}

```gdscript
void cancel_future()
```

Cancel this future, this will interrupt and stop the asynchronous function.

### get_future {#get_future}

```gdscript
int get_future() const
```

Return the XrFutureEXT value this result relates to.

### get_result_value {#get_result_value}

```gdscript
Variant get_result_value() const
```

Returns the result value of our asynchronous function (if set by the extension). The type of this result value depends on the function being called. Consult the documentation of the relevant function.

### get_status {#get_status}

```gdscript
OpenXRFutureResult.ResultStatus get_status() const
```

Returns the status of this result.

### set_result_value {#set_result_value}

```gdscript
void set_result_value(result_value: Variant)
```

Stores the result value we expose to the user.

Note: This method should only be called by an OpenXR extension that implements an asynchronous function.

## 信号

### completed {#completed}

```gdscript
signal completed(result: OpenXRFutureResult)
```

Emitted when the asynchronous function is finished or has been cancelled.

## 枚举

### ResultStatus {#enum-ResultStatus}

```gdscript
enum ResultStatus { RESULT_RUNNING = 0, RESULT_FINISHED = 1, RESULT_CANCELLED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESULT_RUNNING {#RESULT_RUNNING}

```gdscript
RESULT_RUNNING = 0
```

The asynchronous function is running.

#### RESULT_FINISHED {#RESULT_FINISHED}

```gdscript
RESULT_FINISHED = 1
```

The asynchronous function has finished.

#### RESULT_CANCELLED {#RESULT_CANCELLED}

```gdscript
RESULT_CANCELLED = 2
```

The asynchronous function has been cancelled.

---
