# FoldableGroup

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **FoldableGroup**

> A group of FoldableContainer-derived nodes. Only one container can be expanded at a time.

<details><summary>官方英文描述</summary>

A group of FoldableContainer-derived nodes. Only one container can be expanded at a time.

</details>

## 方法

### get_containers {#get_containers}

```gdscript
typedarray::FoldableContainer get_containers() const
```

Returns an Array of FoldableContainers that have this as their FoldableGroup (see FoldableContainer.foldable_group). This is equivalent to ButtonGroup but for FoldableContainers.

### get_expanded_container {#get_expanded_container}

```gdscript
FoldableContainer get_expanded_container() const
```

Returns the current expanded container.

### is_allow_folding_all {#is_allow_folding_all}

```gdscript
bool is_allow_folding_all() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_folding_all {#set_allow_folding_all}

```gdscript
void set_allow_folding_all(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### allow_folding_all {#allow_folding_all}

```gdscript
allow_folding_all: bool
```

- void set_allow_folding_all(value: bool)
- bool is_allow_folding_all()

If true, it is possible to fold all containers in this FoldableGroup.

## 信号

### expanded {#expanded}

```gdscript
signal expanded(container: FoldableContainer)
```

Emitted when one of the containers of the group is expanded.
