# PlaceholderCubemap

**继承**：[PlaceholderTextureLayered](class_placeholdertexturelayered.md) → [TextureLayered](class_texturelayered.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PlaceholderCubemap**

> This class replaces a Cubemap or a Cubemap-derived class in 2 conditions:

<details><summary>官方英文描述（中文精译进行中）</summary>

This class replaces a Cubemap or a Cubemap-derived class in 2 conditions:

- In dedicated server mode, where the image data shouldn't affect game logic. This allows reducing the exported PCK's size significantly.
- When the Cubemap-derived class is missing, for example when using a different engine version.

Note: This class is not intended for rendering or for use in shaders. Operations like calculating UV are not guaranteed to work.

</details>
