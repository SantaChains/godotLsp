# PlaceholderMesh

**继承**：[Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PlaceholderMesh**

> This class is used when loading a project that uses a Mesh subclass in 2 conditions:

<details><summary>官方英文描述（中文精译进行中）</summary>

This class is used when loading a project that uses a Mesh subclass in 2 conditions:

- When running the project exported in dedicated server mode, only the texture's dimensions are kept (as they may be relied upon for gameplay purposes or positioning of other elements). This allows reducing the exported PCK's size significantly.
- When this subclass is missing due to using a different engine version or build (e.g. modules disabled).

</details>

## 方法

### set_aabb {#set_aabb}

```gdscript
void set_aabb(aabb: AABB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### aabb {#aabb}

```gdscript
aabb: AABB
```

- void set_aabb(value: AABB)
- AABB get_aabb()

The smallest AABB enclosing this mesh in local space.
