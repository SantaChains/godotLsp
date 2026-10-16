# PlaceholderCubemapArray

**继承**：[PlaceholderTextureLayered](class_placeholdertexturelayered.md) → [TextureLayered](class_texturelayered.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PlaceholderCubemapArray**

> This class replaces a CubemapArray or a CubemapArray-derived class in 2 conditions:

<details><summary>官方英文描述</summary>

This class replaces a CubemapArray or a CubemapArray-derived class in 2 conditions:

- In dedicated server mode, where the image data shouldn't affect game logic. This allows reducing the exported PCK's size significantly.
- When the CubemapArray-derived class is missing, for example when using a different engine version.

Note: This class is not intended for rendering or for use in shaders. Operations like calculating UV are not guaranteed to work.

</details>
