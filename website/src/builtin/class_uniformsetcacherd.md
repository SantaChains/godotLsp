# UniformSetCacheRD

**继承**：[Object](class_object.md) → **UniformSetCacheRD**

> Uniform set cache manager for RenderingDevice-based renderers. Provides a way to create a uniform set and reuse it in subsequent calls for as long as the uniform set exists. Uniform set will automatically be cleaned up when dependent objects are freed.

<details><summary>官方英文描述</summary>

Uniform set cache manager for RenderingDevice-based renderers. Provides a way to create a uniform set and reuse it in subsequent calls for as long as the uniform set exists. Uniform set will automatically be cleaned up when dependent objects are freed.

</details>

## 方法

### get_cache {#get_cache}

```gdscript
static RID get_cache(shader: RID, set: int, uniforms: typedarray::RDUniform)
```

Creates/returns a cached uniform set based on the provided uniforms for a given shader.
