# FramebufferCacheRD

**继承**：[Object](class_object.md) → **FramebufferCacheRD**

> Framebuffer cache manager for RenderingDevice-based renderers. Provides a way to create a framebuffer and reuse it in subsequent calls for as long as the used textures exists. Framebuffers will automatically be cleaned up when dependent objects are freed.

<details><summary>官方英文描述（中文精译进行中）</summary>

Framebuffer cache manager for RenderingDevice-based renderers. Provides a way to create a framebuffer and reuse it in subsequent calls for as long as the used textures exists. Framebuffers will automatically be cleaned up when dependent objects are freed.

</details>

## 方法

### get_cache_multipass {#get_cache_multipass}

```gdscript
static RID get_cache_multipass(textures: typedarray::RID, passes: typedarray::RDFramebufferPass, views: int)
```

Creates, or obtains a cached, framebuffer. textures lists textures accessed. passes defines the subpasses and texture allocation, if left empty a single pass is created and textures are allocated depending on their usage flags. views defines the number of views used when rendering.
