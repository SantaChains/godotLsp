# Container

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Container**

> Base class for all GUI containers. A Container automatically arranges its child controls in a certain way. This class can be inherited to make custom container types.

<details><summary>官方英文描述</summary>

Base class for all GUI containers. A Container automatically arranges its child controls in a certain way. This class can be inherited to make custom container types.

</details>

## 方法

### _get_allowed_size_flags_horizontal {#_get_allowed_size_flags_horizontal}

```gdscript
PackedInt32Array _get_allowed_size_flags_horizontal() virtual const
```

Implement to return a list of allowed horizontal SizeFlags for child nodes. This doesn't technically prevent the usages of any other size flags, if your implementation requires that. This only limits the options available to the user in the Inspector dock.

Note: Having no size flags is equal to having Control.SIZE_SHRINK_BEGIN. As such, this value is always implicitly allowed.

### _get_allowed_size_flags_vertical {#_get_allowed_size_flags_vertical}

```gdscript
PackedInt32Array _get_allowed_size_flags_vertical() virtual const
```

Implement to return a list of allowed vertical SizeFlags for child nodes. This doesn't technically prevent the usages of any other size flags, if your implementation requires that. This only limits the options available to the user in the Inspector dock.

Note: Having no size flags is equal to having Control.SIZE_SHRINK_BEGIN. As such, this value is always implicitly allowed.

### fit_child_in_rect {#fit_child_in_rect}

```gdscript
void fit_child_in_rect(child: Control, rect: Rect2)
```

Fit a child control in a given rect. This is mainly a helper for creating custom container classes.

### is_accessibility_region {#is_accessibility_region}

```gdscript
bool is_accessibility_region() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### queue_sort {#queue_sort}

```gdscript
void queue_sort()
```

Queue resort of the contained children. This is called automatically anyway, but can be called upon request.

### set_accessibility_region {#set_accessibility_region}

```gdscript
void set_accessibility_region(region: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### accessibility_region {#accessibility_region}

```gdscript
accessibility_region: bool
```

- void set_accessibility_region(value: bool)
- bool is_accessibility_region()

If true, this container is marked as a region for accessibility. Use Control.accessibility_name to give the region a descriptive name. Screen readers can navigate between regions using landmark navigation.

## 信号

### pre_sort_children {#pre_sort_children}

```gdscript
signal pre_sort_children()
```

Emitted when children are going to be sorted.

### sort_children {#sort_children}

```gdscript
signal sort_children()
```

Emitted when sorting the children is needed.

## 常量

### NOTIFICATION_PRE_SORT_CHILDREN {#NOTIFICATION_PRE_SORT_CHILDREN}

```gdscript
const NOTIFICATION_PRE_SORT_CHILDREN = 50
```

Notification just before children are going to be sorted, in case there's something to process beforehand.

NOTIFICATION_SORT_CHILDREN = 51 

Notification for when sorting the children, it must be obeyed immediately.

### NOTIFICATION_SORT_CHILDREN {#NOTIFICATION_SORT_CHILDREN}

```gdscript
const NOTIFICATION_SORT_CHILDREN = 51
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
