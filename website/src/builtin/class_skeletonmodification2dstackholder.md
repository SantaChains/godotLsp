# SkeletonModification2DStackHolder

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DStackHolder**

> This SkeletonModification2D holds a reference to a SkeletonModificationStack2D, allowing you to use multiple modification stacks on a single Skeleton2D.

<details><summary>官方英文描述</summary>

This SkeletonModification2D holds a reference to a SkeletonModificationStack2D, allowing you to use multiple modification stacks on a single Skeleton2D.

Note: The modifications in the held SkeletonModificationStack2D will only be executed if their execution mode matches the execution mode of the SkeletonModification2DStackHolder.

</details>

## 方法

### get_held_modification_stack {#get_held_modification_stack}

```gdscript
SkeletonModificationStack2D get_held_modification_stack() const
```

Returns the SkeletonModificationStack2D that this modification is holding.

### set_held_modification_stack {#set_held_modification_stack}

```gdscript
void set_held_modification_stack(held_modification_stack: SkeletonModificationStack2D)
```

Sets the SkeletonModificationStack2D that this modification is holding. This modification stack will then be executed when this modification is executed.
