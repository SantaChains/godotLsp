# TabContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TabContainer**

> Arranges child controls into a tabbed view, creating a tab for each one. The active tab's corresponding control is made visible, while all other child controls are hidden. Ignores non-control children.

<details><summary>官方英文描述</summary>

Arranges child controls into a tabbed view, creating a tab for each one. The active tab's corresponding control is made visible, while all other child controls are hidden. Ignores non-control children.

Note: The drawing of the clickable tabs is handled by this node; TabBar is not needed.

</details>

## 方法

### are_tabs_visible {#are_tabs_visible}

```gdscript
bool are_tabs_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_clip_tabs {#get_clip_tabs}

```gdscript
bool get_clip_tabs() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_tab {#get_current_tab}

```gdscript
int get_current_tab() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_tab_control {#get_current_tab_control}

```gdscript
Control get_current_tab_control() const
```

Returns the child Control node located at the active tab index.

### get_deselect_enabled {#get_deselect_enabled}

```gdscript
bool get_deselect_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_to_rearrange_enabled {#get_drag_to_rearrange_enabled}

```gdscript
bool get_drag_to_rearrange_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_popup {#get_popup}

```gdscript
Popup get_popup() const
```

Returns the Popup node instance if one has been set already with set_popup().

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

### get_previous_tab {#get_previous_tab}

```gdscript
int get_previous_tab() const
```

Returns the previously active tab index.

### get_switch_on_drag_hover {#get_switch_on_drag_hover}

```gdscript
bool get_switch_on_drag_hover() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_alignment {#get_tab_alignment}

```gdscript
TabBar.AlignmentMode get_tab_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_bar {#get_tab_bar}

```gdscript
TabBar get_tab_bar() const
```

Returns the TabBar contained in this container.

Warning: This is a required internal node, removing and freeing it or editing its tabs may cause a crash. If you wish to edit the tabs, use the methods provided in TabContainer.

### get_tab_button_icon {#get_tab_button_icon}

```gdscript
Texture2D get_tab_button_icon(tab_idx: int) const
```

Returns the button icon from the tab at index tab_idx.

### get_tab_control {#get_tab_control}

```gdscript
Control get_tab_control(tab_idx: int) const
```

Returns the Control node from the tab at index tab_idx.

### get_tab_count {#get_tab_count}

```gdscript
int get_tab_count() const
```

Returns the number of tabs.

### get_tab_focus_mode {#get_tab_focus_mode}

```gdscript
Control.FocusMode get_tab_focus_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_icon {#get_tab_icon}

```gdscript
Texture2D get_tab_icon(tab_idx: int) const
```

Returns the Texture2D for the tab at index tab_idx or null if the tab has no Texture2D.

### get_tab_icon_max_width {#get_tab_icon_max_width}

```gdscript
int get_tab_icon_max_width(tab_idx: int) const
```

Returns the maximum allowed width of the icon for the tab at index tab_idx.

### get_tab_idx_at_point {#get_tab_idx_at_point}

```gdscript
int get_tab_idx_at_point(point: Vector2) const
```

Returns the index of the tab at local coordinates point. Returns -1 if the point is outside the control boundaries or if there's no tab at the queried position.

### get_tab_idx_from_control {#get_tab_idx_from_control}

```gdscript
int get_tab_idx_from_control(control: Control) const
```

Returns the index of the tab tied to the given control. The control must be a child of the TabContainer.

### get_tab_metadata {#get_tab_metadata}

```gdscript
Variant get_tab_metadata(tab_idx: int) const
```

Returns the metadata value set to the tab at index tab_idx using set_tab_metadata(). If no metadata was previously set, returns null by default.

### get_tab_title {#get_tab_title}

```gdscript
String get_tab_title(tab_idx: int) const
```

Returns the title of the tab at index tab_idx. Tab titles default to the name of the indexed child node, but this can be overridden with set_tab_title().

### get_tab_tooltip {#get_tab_tooltip}

```gdscript
String get_tab_tooltip(tab_idx: int) const
```

Returns the tooltip text of the tab at index tab_idx.

### get_tabs_position {#get_tabs_position}

```gdscript
TabContainer.TabPosition get_tabs_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tabs_rearrange_group {#get_tabs_rearrange_group}

```gdscript
int get_tabs_rearrange_group() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_hidden_tabs_for_min_size {#get_use_hidden_tabs_for_min_size}

```gdscript
bool get_use_hidden_tabs_for_min_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_all_tabs_in_front {#is_all_tabs_in_front}

```gdscript
bool is_all_tabs_in_front() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_tab_disabled {#is_tab_disabled}

```gdscript
bool is_tab_disabled(tab_idx: int) const
```

Returns true if the tab at index tab_idx is disabled.

### is_tab_hidden {#is_tab_hidden}

```gdscript
bool is_tab_hidden(tab_idx: int) const
```

Returns true if the tab at index tab_idx is hidden.

### select_next_available {#select_next_available}

```gdscript
bool select_next_available()
```

Selects the first available tab with greater index than the currently selected. Returns true if tab selection changed.

### select_previous_available {#select_previous_available}

```gdscript
bool select_previous_available()
```

Selects the first available tab with lower index than the currently selected. Returns true if tab selection changed.

### set_all_tabs_in_front {#set_all_tabs_in_front}

```gdscript
void set_all_tabs_in_front(is_front: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_clip_tabs {#set_clip_tabs}

```gdscript
void set_clip_tabs(clip_tabs: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current_tab {#set_current_tab}

```gdscript
void set_current_tab(tab_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deselect_enabled {#set_deselect_enabled}

```gdscript
void set_deselect_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_to_rearrange_enabled {#set_drag_to_rearrange_enabled}

```gdscript
void set_drag_to_rearrange_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_popup {#set_popup}

```gdscript
void set_popup(popup: Node)
```

If set on a Popup node instance, a popup menu icon appears in the top-right corner of the TabContainer (setting it to null will make it go away). Clicking it will expand the Popup node.

### set_switch_on_drag_hover {#set_switch_on_drag_hover}

```gdscript
void set_switch_on_drag_hover(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_alignment {#set_tab_alignment}

```gdscript
void set_tab_alignment(alignment: TabBar.AlignmentMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_button_icon {#set_tab_button_icon}

```gdscript
void set_tab_button_icon(tab_idx: int, icon: Texture2D)
```

Sets the button icon from the tab at index tab_idx.

### set_tab_disabled {#set_tab_disabled}

```gdscript
void set_tab_disabled(tab_idx: int, disabled: bool)
```

If disabled is true, disables the tab at index tab_idx, making it non-interactable.

### set_tab_focus_mode {#set_tab_focus_mode}

```gdscript
void set_tab_focus_mode(focus_mode: Control.FocusMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_hidden {#set_tab_hidden}

```gdscript
void set_tab_hidden(tab_idx: int, hidden: bool)
```

If hidden is true, hides the tab at index tab_idx, making it disappear from the tab area.

### set_tab_icon {#set_tab_icon}

```gdscript
void set_tab_icon(tab_idx: int, icon: Texture2D)
```

Sets an icon for the tab at index tab_idx.

### set_tab_icon_max_width {#set_tab_icon_max_width}

```gdscript
void set_tab_icon_max_width(tab_idx: int, width: int)
```

Sets the maximum allowed width of the icon for the tab at index tab_idx. This limit is applied on top of the default size of the icon and on top of icon_max_width. The height is adjusted according to the icon's ratio.

### set_tab_metadata {#set_tab_metadata}

```gdscript
void set_tab_metadata(tab_idx: int, metadata: Variant)
```

Sets the metadata value for the tab at index tab_idx, which can be retrieved later using get_tab_metadata().

### set_tab_title {#set_tab_title}

```gdscript
void set_tab_title(tab_idx: int, title: String)
```

Sets a custom title for the tab at index tab_idx (tab titles default to the name of the indexed child node). Set it back to the child's name to make the tab default to it again.

### set_tab_tooltip {#set_tab_tooltip}

```gdscript
void set_tab_tooltip(tab_idx: int, tooltip: String)
```

Sets a custom tooltip text for tab at index tab_idx.

Note: By default, if the tooltip is empty and the tab text is truncated (not all characters fit into the tab), the title will be displayed as a tooltip. To hide the tooltip, assign " " as the tooltip text.

### set_tabs_position {#set_tabs_position}

```gdscript
void set_tabs_position(tabs_position: TabContainer.TabPosition)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tabs_rearrange_group {#set_tabs_rearrange_group}

```gdscript
void set_tabs_rearrange_group(group_id: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tabs_visible {#set_tabs_visible}

```gdscript
void set_tabs_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_hidden_tabs_for_min_size {#set_use_hidden_tabs_for_min_size}

```gdscript
void set_use_hidden_tabs_for_min_size(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### tab_alignment {#tab_alignment}

```gdscript
tab_alignment: int
```

- void set_tab_alignment(value: AlignmentMode)
- AlignmentMode get_tab_alignment()

The position at which tabs will be placed.

### current_tab {#current_tab}

```gdscript
current_tab: int
```

- void set_current_tab(value: int)
- int get_current_tab()

The current tab index. When set, this index's Control node's visible property is set to true and all others are set to false.

A value of -1 means that no tab is selected.

### tabs_position {#tabs_position}

```gdscript
tabs_position: int
```

- void set_tabs_position(value: TabPosition)
- TabPosition get_tabs_position()

The horizontal alignment of the tabs.

### clip_tabs {#clip_tabs}

```gdscript
clip_tabs: bool
```

- void set_clip_tabs(value: bool)
- bool get_clip_tabs()

If true, tabs overflowing this node's width will be hidden, displaying two navigation buttons instead. Otherwise, this node's minimum size is updated so that all tabs are visible.

### tabs_visible {#tabs_visible}

```gdscript
tabs_visible: bool
```

- void set_tabs_visible(value: bool)
- bool are_tabs_visible()

If true, tabs are visible. If false, tabs' content and titles are hidden.

### all_tabs_in_front {#all_tabs_in_front}

```gdscript
all_tabs_in_front: bool
```

- void set_all_tabs_in_front(value: bool)
- bool is_all_tabs_in_front()

Deprecated: Due to internal changes this doesn't do anything anymore, as they're always in front.

This doesn't do anything.

### switch_on_drag_hover {#switch_on_drag_hover}

```gdscript
switch_on_drag_hover: bool
```

- void set_switch_on_drag_hover(value: bool)
- bool get_switch_on_drag_hover()

If true, hovering over a tab while dragging something will switch to that tab. Does not have effect when hovering another tab to rearrange.

### drag_to_rearrange_enabled {#drag_to_rearrange_enabled}

```gdscript
drag_to_rearrange_enabled: bool
```

- void set_drag_to_rearrange_enabled(value: bool)
- bool get_drag_to_rearrange_enabled()

If true, tabs can be rearranged with mouse drag.

### tabs_rearrange_group {#tabs_rearrange_group}

```gdscript
tabs_rearrange_group: int
```

- void set_tabs_rearrange_group(value: int)
- int get_tabs_rearrange_group()

TabContainers with the same rearrange group ID will allow dragging the tabs between them. Enable drag with drag_to_rearrange_enabled.

Setting this to -1 will disable rearranging between TabContainers.

### use_hidden_tabs_for_min_size {#use_hidden_tabs_for_min_size}

```gdscript
use_hidden_tabs_for_min_size: bool
```

- void set_use_hidden_tabs_for_min_size(value: bool)
- bool get_use_hidden_tabs_for_min_size()

If true, child Control nodes that are hidden have their minimum size take into account in the total, instead of only the currently visible one.

### tab_focus_mode {#tab_focus_mode}

```gdscript
tab_focus_mode: int
```

- void set_tab_focus_mode(value: FocusMode)
- FocusMode get_tab_focus_mode()

The focus access mode for the internal TabBar node.

### deselect_enabled {#deselect_enabled}

```gdscript
deselect_enabled: bool
```

- void set_deselect_enabled(value: bool)
- bool get_deselect_enabled()

If true, all tabs can be deselected so that no tab is selected. Click on the current_tab to deselect it.

Only the tab header will be shown if no tabs are selected.

## 信号

### active_tab_rearranged {#active_tab_rearranged}

```gdscript
signal active_tab_rearranged(idx_to: int)
```

Emitted when the active tab is rearranged via mouse drag. See drag_to_rearrange_enabled.

### tab_changed {#tab_changed}

```gdscript
signal tab_changed(tab: int)
```

Emitted when switching to another tab.

### tab_clicked {#tab_clicked}

```gdscript
signal tab_clicked(tab: int)
```

Emitted when a tab is clicked, even if it is the current tab.

### tab_hovered {#tab_hovered}

```gdscript
signal tab_hovered(tab: int)
```

Emitted when a tab is hovered by the mouse.

### tab_selected {#tab_selected}

```gdscript
signal tab_selected(tab: int)
```

Emitted when a tab is selected via click, directional input, or script, even if it is the current tab.

### tab_button_pressed {#tab_button_pressed}

```gdscript
signal tab_button_pressed(tab: int)
```

Emitted when the user clicks on the button icon on this tab.

### pre_popup_pressed {#pre_popup_pressed}

```gdscript
signal pre_popup_pressed()
```

Emitted when the TabContainer's Popup button is clicked. See set_popup() for details.

## 枚举

### TabPosition {#enum-TabPosition}

```gdscript
enum TabPosition { POSITION_TOP = 0, POSITION_BOTTOM = 1, POSITION_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### POSITION_TOP {#POSITION_TOP}

```gdscript
POSITION_TOP = 0
```

Places the tab bar at the top.

#### POSITION_BOTTOM {#POSITION_BOTTOM}

```gdscript
POSITION_BOTTOM = 1
```

Places the tab bar at the bottom. The tab bar's StyleBox will be flipped vertically.

#### POSITION_MAX {#POSITION_MAX}

```gdscript
POSITION_MAX = 2
```

Represents the size of the TabPosition enum.

---
