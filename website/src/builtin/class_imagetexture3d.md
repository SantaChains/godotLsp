# ImageTexture3D

**继承**：[Texture3D](class_texture3d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ImageTexture3D**

> ImageTexture3D is a 3-dimensional ImageTexture that has a width, height, and depth. See also ImageTextureLayered.

<details><summary>官方英文描述（中文精译进行中）</summary>

ImageTexture3D is a 3-dimensional ImageTexture that has a width, height, and depth. See also ImageTextureLayered.

3D textures are typically used to store density maps for FogMaterial, color correction LUTs for Environment, vector fields for GPUParticlesAttractorVectorField3D and collision maps for GPUParticlesCollisionSDF3D. 3D textures can also be used in custom shaders.

</details>

## 方法

### create {#create}

```gdscript
Error create(format: Image.Format, width: int, height: int, depth: int, use_mipmaps: bool, data: typedarray::Image)
```

Creates the ImageTexture3D with specified format, width, height, and depth. If use_mipmaps is true, generates mipmaps for the ImageTexture3D.

### update {#update}

```gdscript
void update(data: typedarray::Image)
```

Replaces the texture's existing data with the layers specified in data. The size of data must match the parameters that were used for create(). In other words, the texture cannot be resized or have its format changed by calling update().
