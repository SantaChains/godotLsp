# SplitContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **SplitContainer**

> A container that arranges child controls horizontally or vertically and creates grabbers between them. The grabbers can be dragged around to change the size relations between the child controls.

<details><summary>官方英文描述</summary>

A container that arranges child controls horizontally or vertically and creates grabbers between them. The grabbers can be dragged around to change the size relations between the child controls.

</details>

## 方法

### clamp_split_offset {#clamp_split_offset}

```gdscript
void clamp_split_offset(priority_index: int = 0)
```

Clamps the split_offsets values to ensure they are within valid ranges and do not overlap with each other. When overlaps occur, this method prioritizes one split offset (at index priority_index) by clamping any overlapping split offsets to it.

### get_drag_area_control {#get_drag_area_control}

```gdscript
Control get_drag_area_control()
```

Deprecated: Use the first element of get_drag_area_controls() instead.

Returns the drag area Control. For example, you can move a pre-configured button into the drag area Control so that it rides along with the split bar. Try setting the Button anchors to center prior to the reparent() call.

```gdscript
$BarnacleButton.reparent($SplitContainer.get_drag_area_control())
```

Note: The drag area Control is drawn over the SplitContainer's children, so CanvasItem draw objects called from the Control and children added to the Control will also appear over the SplitContainer's children. Try setting Control.mouse_filter of custom children to Control.MOUSE_FILTER_IGNORE to prevent blocking the mouse from dragging if desired.

Warning: This is a required internal node, removing and freeing it may cause a crash.

### get_drag_area_controls {#get_drag_area_controls}

```gdscript
typedarray::Control get_drag_area_controls()
```

Returns an Array of the drag area Controls. These are the interactable Control nodes between each child. For example, this can be used to add a pre-configured button to a drag area Control so that it rides along with the split bar. Try setting the Button anchors to center prior to the Node.reparent() call.

```gdscript
$BarnacleButton.reparent($SplitContainer.get_drag_area_controls()[0])
```

Note: The drag area Controls are drawn over the SplitContainer's children, so CanvasItem draw objects called from a drag area and children added to it will also appear over the SplitContainer's children. Try setting Control.mouse_filter of custom children to Control.MOUSE_FILTER_IGNORE to prevent blocking the mouse from dragging if desired.

Warning: These are required internal nodes, removing or freeing them may cause a crash.

### get_drag_area_margin_begin {#get_drag_area_margin_begin}

```gdscript
int get_drag_area_margin_begin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_area_margin_end {#get_drag_area_margin_end}

```gdscript
int get_drag_area_margin_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_area_offset {#get_drag_area_offset}

```gdscript
int get_drag_area_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dragger_visibility {#get_dragger_visibility}

```gdscript
SplitContainer.DraggerVisibility get_dragger_visibility() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_split_offset {#get_split_offset}

```gdscript
int get_split_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_split_offsets {#get_split_offsets}

```gdscript
PackedInt32Array get_split_offsets() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collapsed {#is_collapsed}

```gdscript
bool is_collapsed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drag_area_highlight_in_editor_enabled {#is_drag_area_highlight_in_editor_enabled}

```gdscript
bool is_drag_area_highlight_in_editor_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_dragging_enabled {#is_dragging_enabled}

```gdscript
bool is_dragging_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_dragging_nested_intersections {#is_dragging_nested_intersections}

```gdscript
bool is_dragging_nested_intersections() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_touch_dragger_enabled {#is_touch_dragger_enabled}

```gdscript
bool is_touch_dragger_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_vertical {#is_vertical}

```gdscript
bool is_vertical() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collapsed {#set_collapsed}

```gdscript
void set_collapsed(collapsed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_area_highlight_in_editor {#set_drag_area_highlight_in_editor}

```gdscript
void set_drag_area_highlight_in_editor(drag_area_highlight_in_editor: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_area_margin_begin {#set_drag_area_margin_begin}

```gdscript
void set_drag_area_margin_begin(margin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_area_margin_end {#set_drag_area_margin_end}

```gdscript
void set_drag_area_margin_end(margin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_area_offset {#set_drag_area_offset}

```gdscript
void set_drag_area_offset(offset: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_nested_intersections {#set_drag_nested_intersections}

```gdscript
void set_drag_nested_intersections(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dragger_visibility {#set_dragger_visibility}

```gdscript
void set_dragger_visibility(mode: SplitContainer.DraggerVisibility)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dragging_enabled {#set_dragging_enabled}

```gdscript
void set_dragging_enabled(dragging_enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_split_offset {#set_split_offset}

```gdscript
void set_split_offset(offset: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_split_offsets {#set_split_offsets}

```gdscript
void set_split_offsets(offsets: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_touch_dragger_enabled {#set_touch_dragger_enabled}

```gdscript
void set_touch_dragger_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical {#set_vertical}

```gdscript
void set_vertical(vertical: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### split_offsets {#split_offsets}

```gdscript
split_offsets: PackedInt32Array
```

- void set_split_offsets(value: PackedInt32Array)
- PackedInt32Array get_split_offsets()

Offsets for each dragger in pixels. Each one is the offset of the split between the Control nodes before and after the dragger, with 0 being the default position. The default position is based on the Control nodes expand flags and minimum sizes. See Control.size_flags_horizontal, Control.size_flags_vertical, and Control.size_flags_stretch_ratio.

If none of the Control nodes before the dragger are expanded, the default position will be at the start of the SplitContainer. If none of the Control nodes after the dragger are expanded, the default position will be at the end of the SplitContainer. If the dragger is in between expanded Control nodes, the default position will be in the middle, based on the Control.size_flags_stretch_ratios and minimum sizes.

Note: If the split offsets cause Control nodes to overlap, the first split will take priority when resolving the positions.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### collapsed {#collapsed}

```gdscript
collapsed: bool
```

- void set_collapsed(value: bool)
- bool is_collapsed()

If true, the draggers will be disabled and the children will be sized as if all split_offsets were 0.

### dragging_enabled {#dragging_enabled}

```gdscript
dragging_enabled: bool
```

- void set_dragging_enabled(value: bool)
- bool is_dragging_enabled()

Enables or disables split dragging.

### dragger_visibility {#dragger_visibility}

```gdscript
dragger_visibility: int
```

- void set_dragger_visibility(value: DraggerVisibility)
- DraggerVisibility get_dragger_visibility()

Determines the dragger's visibility. This property does not determine whether dragging is enabled or not. Use dragging_enabled for that.

### vertical {#vertical}

```gdscript
vertical: bool
```

- void set_vertical(value: bool)
- bool is_vertical()

If true, the SplitContainer will arrange its children vertically, rather than horizontally.

Can't be changed when using HSplitContainer and VSplitContainer.

### touch_dragger_enabled {#touch_dragger_enabled}

```gdscript
touch_dragger_enabled: bool
```

- void set_touch_dragger_enabled(value: bool)
- bool is_touch_dragger_enabled()

If true, a touch-friendly drag handle will be enabled for better usability on smaller screens. Unlike the standard grabber, this drag handle overlaps the SplitContainer's children and does not affect their minimum separation. The standard grabber will no longer be drawn when this option is enabled.

### drag_nested_intersections {#drag_nested_intersections}

```gdscript
drag_nested_intersections: bool
```

- void set_drag_nested_intersections(value: bool)
- bool is_dragging_nested_intersections()

Adds extra draggers at the intersection of the draggers of two SplitContainers to allow dragging both at once. This must be set to true for both SplitContainers, and one needs to be a descendant of the other. They also must be orthogonal (their vertical are different) and the descendant must be next to at least one of the ancestor's draggers (within minimum_grab_thickness).

### drag_area_margin_begin {#drag_area_margin_begin}

```gdscript
drag_area_margin_begin: int
```

- void set_drag_area_margin_begin(value: int)
- int get_drag_area_margin_begin()

Reduces the size of the drag area and split bar split_bar_background at the beginning of the container.

### drag_area_margin_end {#drag_area_margin_end}

```gdscript
drag_area_margin_end: int
```

- void set_drag_area_margin_end(value: int)
- int get_drag_area_margin_end()

Reduces the size of the drag area and split bar split_bar_background at the end of the container.

### drag_area_offset {#drag_area_offset}

```gdscript
drag_area_offset: int
```

- void set_drag_area_offset(value: int)
- int get_drag_area_offset()

Shifts the drag area in the axis of the container to prevent the drag area from overlapping the ScrollBar or other selectable Control of a child node.

### drag_area_highlight_in_editor {#drag_area_highlight_in_editor}

```gdscript
drag_area_highlight_in_editor: bool
```

- void set_drag_area_highlight_in_editor(value: bool)
- bool is_drag_area_highlight_in_editor_enabled()

Highlights the drag area Rect2 so you can see where it is during development. The drag area is gold if dragging_enabled is true, and red if false.

### split_offset {#split_offset}

```gdscript
split_offset: int
```

- void set_split_offset(value: int)
- int get_split_offset()

Deprecated: Use split_offsets instead. The first element of the array is the split offset between the first two children.

The first element of split_offsets.

## 信号

### dragged {#dragged}

```gdscript
signal dragged(offset: int)
```

Emitted when any dragger is dragged by user.

### drag_started {#drag_started}

```gdscript
signal drag_started()
```

Emitted when the user starts dragging.

### drag_ended {#drag_ended}

```gdscript
signal drag_ended()
```

Emitted when the user ends dragging.

## 枚举

### DraggerVisibility {#enum-DraggerVisibility}

```gdscript
enum DraggerVisibility { DRAGGER_VISIBLE = 0, DRAGGER_HIDDEN = 1, DRAGGER_HIDDEN_COLLAPSED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DRAGGER_VISIBLE {#DRAGGER_VISIBLE}

```gdscript
DRAGGER_VISIBLE = 0
```

The split dragger icon is always visible when autohide is false, otherwise visible only when the cursor hovers it.

The size of the grabber icon determines the minimum separation.

The dragger icon is automatically hidden if the length of the grabber icon is longer than the split bar.

#### DRAGGER_HIDDEN {#DRAGGER_HIDDEN}

```gdscript
DRAGGER_HIDDEN = 1
```

The split dragger icon is never visible regardless of the value of autohide.

The size of the grabber icon determines the minimum separation.

#### DRAGGER_HIDDEN_COLLAPSED {#DRAGGER_HIDDEN_COLLAPSED}

```gdscript
DRAGGER_HIDDEN_COLLAPSED = 2
```

The split dragger icon is not visible, and the split bar is collapsed to zero thickness.

---
