# ExternalTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ExternalTexture**

> Displays the content of an external buffer provided by the platform.

<details><summary>官方英文描述（中文精译进行中）</summary>

Displays the content of an external buffer provided by the platform.

Requires the OES_EGL_image_external [https://registry.khronos.org/OpenGL/extensions/OES/OES_EGL_image_external.txt] extension (OpenGL) or VK_ANDROID_external_memory_android_hardware_buffer [https://registry.khronos.org/vulkan/specs/1.1-extensions/html/vkspec.html#VK_ANDROID_external_memory_android_hardware_buffer] extension (Vulkan).

Note: This is currently only supported in Android builds.

</details>

## 方法

### get_external_texture_id {#get_external_texture_id}

```gdscript
int get_external_texture_id() const
```

Returns the external texture ID.

Depending on your use case, you may need to pass this to platform APIs, for example, when creating an android.graphics.SurfaceTexture on Android.

### set_external_buffer_id {#set_external_buffer_id}

```gdscript
void set_external_buffer_id(external_buffer_id: int)
```

Sets the external buffer ID.

Depending on your use case, you may need to call this with data received from a platform API, for example, SurfaceTexture.getHardwareBuffer() on Android.

### set_size {#set_size}

```gdscript
void set_size(size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
