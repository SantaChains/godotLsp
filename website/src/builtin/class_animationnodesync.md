# AnimationNodeSync

**继承**：[AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeSync**

> An animation node used to combine, mix, or blend two or more animations together while keeping them synchronized within an AnimationTree.

<details><summary>官方英文描述</summary>

An animation node used to combine, mix, or blend two or more animations together while keeping them synchronized within an AnimationTree.

</details>

## 方法

### is_using_sync {#is_using_sync}

```gdscript
bool is_using_sync() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_sync {#set_use_sync}

```gdscript
void set_use_sync(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### sync {#sync}

```gdscript
sync: bool
```

- void set_use_sync(value: bool)
- bool is_using_sync()

If false, the blended animations' frame are stopped when the blend value is 0.

If true, forcing the blended animations to advance frame.
