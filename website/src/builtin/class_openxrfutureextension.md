# OpenXRFutureExtension

**继承**：[OpenXRExtensionWrapper](class_openxrextensionwrapper.md) → [Object](class_object.md) → **OpenXRFutureExtension**

> This is a support extension in OpenXR that allows other OpenXR extensions to start asynchronous functions and get a callback after this function finishes. It is not intended for consumption within GDScript but can be accessed from GDExtension.

<details><summary>官方英文描述（中文精译进行中）</summary>

This is a support extension in OpenXR that allows other OpenXR extensions to start asynchronous functions and get a callback after this function finishes. It is not intended for consumption within GDScript but can be accessed from GDExtension.

</details>

## 方法

### cancel_future {#cancel_future}

```gdscript
void cancel_future(future: int)
```

Cancels an in-progress future. future must be an XrFutureEXT value previously returned by an API that started an asynchronous function.

### is_active {#is_active}

```gdscript
bool is_active() const
```

Returns true if futures are available in the OpenXR runtime used. This function will only return a usable result after OpenXR has been initialized.

### register_future {#register_future}

```gdscript
OpenXRFutureResult register_future(future: int, on_success: Callable = Callable())
```

Register an OpenXR Future object so we monitor for completion. future must be an XrFutureEXT value previously returned by an API that started an asynchronous function.

You can optionally specify on_success, it will be invoked on successful completion of the future.

Or you can use the returned OpenXRFutureResult object to await its OpenXRFutureResult.completed signal.

```gdscript
var future_result = OpenXRFutureExtension.register_future(future)
await future_result.completed
if future_result.get_status() == OpenXRFutureResult.RESULT_FINISHED:
    # Handle your success
    pass
```
