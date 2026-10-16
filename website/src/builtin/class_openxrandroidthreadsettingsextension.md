# OpenXRAndroidThreadSettingsExtension

**继承**：[OpenXRExtensionWrapper](class_openxrextensionwrapper.md) → [Object](class_object.md) → **OpenXRAndroidThreadSettingsExtension**

> For XR to be comfortable, it is important for applications to deliver frames quickly and consistently. In order to make sure the important application threads get their full share of time, these threads must be identified to the system, which will adjust their scheduling priority accordingly.

<details><summary>官方英文描述</summary>

For XR to be comfortable, it is important for applications to deliver frames quickly and consistently. In order to make sure the important application threads get their full share of time, these threads must be identified to the system, which will adjust their scheduling priority accordingly.

</details>

## 方法

### set_application_thread_type {#set_application_thread_type}

```gdscript
bool set_application_thread_type(thread_type: OpenXRAndroidThreadSettingsExtension.ThreadType, thread_id: int = 0)
```

Sets the thread type of the given thread, so that the XR runtime can adjust its scheduling priority accordingly.

thread_id refers to the OS thread id (ie from gettid()). When thread_id is 0, it will set the thread type of the current thread.

NOTE: The id returned by Thread.get_id() is incompatible with thread_id.

## 枚举

### ThreadType {#enum-ThreadType}

```gdscript
enum ThreadType { THREAD_TYPE_APPLICATION_MAIN = 0, THREAD_TYPE_APPLICATION_WORKER = 1, THREAD_TYPE_RENDERER_MAIN = 2, THREAD_TYPE_RENDERER_WORKER = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### THREAD_TYPE_APPLICATION_MAIN {#THREAD_TYPE_APPLICATION_MAIN}

```gdscript
THREAD_TYPE_APPLICATION_MAIN = 0
```

Hints to the XR runtime that the thread is doing time critical CPU tasks.

#### THREAD_TYPE_APPLICATION_WORKER {#THREAD_TYPE_APPLICATION_WORKER}

```gdscript
THREAD_TYPE_APPLICATION_WORKER = 1
```

Hints to the XR runtime that the thread is doing background CPU tasks.

#### THREAD_TYPE_RENDERER_MAIN {#THREAD_TYPE_RENDERER_MAIN}

```gdscript
THREAD_TYPE_RENDERER_MAIN = 2
```

Hints to the XR runtime that the thread is doing time critical graphics device tasks.

#### THREAD_TYPE_RENDERER_WORKER {#THREAD_TYPE_RENDERER_WORKER}

```gdscript
THREAD_TYPE_RENDERER_WORKER = 3
```

Hints to the XR runtime that the thread is doing background graphics device tasks.

---
