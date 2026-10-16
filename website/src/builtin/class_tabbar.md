# TabBar

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TabBar**

> A control that provides a horizontal bar with tabs. Similar to TabContainer but is only in charge of drawing tabs, not interacting with children.

<details><summary>官方英文描述（中文精译进行中）</summary>

A control that provides a horizontal bar with tabs. Similar to TabContainer but is only in charge of drawing tabs, not interacting with children.

</details>

## 方法

### add_tab {#add_tab}

```gdscript
void add_tab(title: String = "", icon: Texture2D = null)
```

Adds a new tab.

### clear_tabs {#clear_tabs}

```gdscript
void clear_tabs()
```

Clears all tabs.

### ensure_tab_visible {#ensure_tab_visible}

```gdscript
void ensure_tab_visible(idx: int)
```

Moves the scroll view to make the tab visible.

### get_clip_tabs {#get_clip_tabs}

```gdscript
bool get_clip_tabs() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_close_with_middle_mouse {#get_close_with_middle_mouse}

```gdscript
bool get_close_with_middle_mouse() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_tab {#get_current_tab}

```gdscript
int get_current_tab() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### get_max_tab_width {#get_max_tab_width}

```gdscript
int get_max_tab_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset_buttons_visible {#get_offset_buttons_visible}

```gdscript
bool get_offset_buttons_visible() const
```

Returns true if the offset buttons (the ones that appear when there's not enough space for all tabs) are visible.

### get_previous_tab {#get_previous_tab}

```gdscript
int get_previous_tab() const
```

Returns the previously active tab index.

### get_scroll_to_selected {#get_scroll_to_selected}

```gdscript
bool get_scroll_to_selected() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scrolling_enabled {#get_scrolling_enabled}

```gdscript
bool get_scrolling_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_select_with_rmb {#get_select_with_rmb}

```gdscript
bool get_select_with_rmb() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### get_tab_button_icon {#get_tab_button_icon}

```gdscript
Texture2D get_tab_button_icon(tab_idx: int) const
```

Returns the icon for the right button of the tab at index tab_idx or null if the right button has no icon.

### get_tab_close_display_policy {#get_tab_close_display_policy}

```gdscript
TabBar.CloseButtonDisplayPolicy get_tab_close_display_policy() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_count {#get_tab_count}

```gdscript
int get_tab_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_icon {#get_tab_icon}

```gdscript
Texture2D get_tab_icon(tab_idx: int) const
```

Returns the icon for the tab at index tab_idx or null if the tab has no icon.

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

### get_tab_language {#get_tab_language}

```gdscript
String get_tab_language(tab_idx: int) const
```

Returns tab title language code.

### get_tab_metadata {#get_tab_metadata}

```gdscript
Variant get_tab_metadata(tab_idx: int) const
```

Returns the metadata value set to the tab at index tab_idx using set_tab_metadata(). If no metadata was previously set, returns null by default.

### get_tab_offset {#get_tab_offset}

```gdscript
int get_tab_offset() const
```

Returns the number of hidden tabs offsetted to the left.

### get_tab_rect {#get_tab_rect}

```gdscript
Rect2 get_tab_rect(tab_idx: int) const
```

Returns tab Rect2 with local position and size.

### get_tab_text_direction {#get_tab_text_direction}

```gdscript
Control.TextDirection get_tab_text_direction(tab_idx: int) const
```

Returns tab title text base writing direction.

### get_tab_title {#get_tab_title}

```gdscript
String get_tab_title(tab_idx: int) const
```

Returns the title of the tab at index tab_idx.

### get_tab_tooltip {#get_tab_tooltip}

```gdscript
String get_tab_tooltip(tab_idx: int) const
```

Returns the tooltip text of the tab at index tab_idx.

### get_tabs_rearrange_group {#get_tabs_rearrange_group}

```gdscript
int get_tabs_rearrange_group() const
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

### move_tab {#move_tab}

```gdscript
void move_tab(from: int, to: int)
```

Moves a tab from from to to.

### remove_tab {#remove_tab}

```gdscript
void remove_tab(tab_idx: int)
```

Removes the tab at index tab_idx.

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

### set_clip_tabs {#set_clip_tabs}

```gdscript
void set_clip_tabs(clip_tabs: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_close_with_middle_mouse {#set_close_with_middle_mouse}

```gdscript
void set_close_with_middle_mouse(enabled: bool)
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

### set_max_tab_width {#set_max_tab_width}

```gdscript
void set_max_tab_width(width: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_to_selected {#set_scroll_to_selected}

```gdscript
void set_scroll_to_selected(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scrolling_enabled {#set_scrolling_enabled}

```gdscript
void set_scrolling_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_select_with_rmb {#set_select_with_rmb}

```gdscript
void set_select_with_rmb(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

Sets an icon for the button of the tab at index tab_idx (located to the right, before the close button), making it visible and clickable (See tab_button_pressed). Giving it a null value will hide the button.

### set_tab_close_display_policy {#set_tab_close_display_policy}

```gdscript
void set_tab_close_display_policy(policy: TabBar.CloseButtonDisplayPolicy)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_count {#set_tab_count}

```gdscript
void set_tab_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_disabled {#set_tab_disabled}

```gdscript
void set_tab_disabled(tab_idx: int, disabled: bool)
```

If disabled is true, disables the tab at index tab_idx, making it non-interactable.

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

### set_tab_language {#set_tab_language}

```gdscript
void set_tab_language(tab_idx: int, language: String)
```

Sets the language code of the title for the tab at index tab_idx to language. This is used for line-breaking and text shaping algorithms. If language is empty, the current locale is used.

### set_tab_metadata {#set_tab_metadata}

```gdscript
void set_tab_metadata(tab_idx: int, metadata: Variant)
```

Sets the metadata value for the tab at index tab_idx, which can be retrieved later using get_tab_metadata().

### set_tab_text_direction {#set_tab_text_direction}

```gdscript
void set_tab_text_direction(tab_idx: int, direction: Control.TextDirection)
```

Sets tab title base writing direction.

### set_tab_title {#set_tab_title}

```gdscript
void set_tab_title(tab_idx: int, title: String)
```

Sets a title for the tab at index tab_idx.

### set_tab_tooltip {#set_tab_tooltip}

```gdscript
void set_tab_tooltip(tab_idx: int, tooltip: String)
```

Sets a tooltip for tab at index tab_idx.

Note: By default, if the tooltip is empty and the tab text is truncated (not all characters fit into the tab), the title will be displayed as a tooltip. To hide the tooltip, assign " " as the tooltip text.

### set_tabs_rearrange_group {#set_tabs_rearrange_group}

```gdscript
void set_tabs_rearrange_group(group_id: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### current_tab {#current_tab}

```gdscript
current_tab: int
```

- void set_current_tab(value: int)
- int get_current_tab()

The index of the current selected tab. A value of -1 means that no tab is selected and can only be set when deselect_enabled is true or if all tabs are hidden or disabled.

### tab_alignment {#tab_alignment}

```gdscript
tab_alignment: int
```

- void set_tab_alignment(value: AlignmentMode)
- AlignmentMode get_tab_alignment()

The horizontal alignment of the tabs.

### clip_tabs {#clip_tabs}

```gdscript
clip_tabs: bool
```

- void set_clip_tabs(value: bool)
- bool get_clip_tabs()

If true, tabs overflowing this node's width will be hidden, displaying two navigation buttons instead. Otherwise, this node's minimum size is updated so that all tabs are visible.

### close_with_middle_mouse {#close_with_middle_mouse}

```gdscript
close_with_middle_mouse: bool
```

- void set_close_with_middle_mouse(value: bool)
- bool get_close_with_middle_mouse()

If true, middle-clicking on a tab will emit the tab_close_pressed signal.

### tab_close_display_policy {#tab_close_display_policy}

```gdscript
tab_close_display_policy: int
```

- void set_tab_close_display_policy(value: CloseButtonDisplayPolicy)
- CloseButtonDisplayPolicy get_tab_close_display_policy()

When the close button will appear on the tabs.

### max_tab_width {#max_tab_width}

```gdscript
max_tab_width: int
```

- void set_max_tab_width(value: int)
- int get_max_tab_width()

Sets the maximum width which all tabs should be limited to. Unlimited if set to 0.

### scrolling_enabled {#scrolling_enabled}

```gdscript
scrolling_enabled: bool
```

- void set_scrolling_enabled(value: bool)
- bool get_scrolling_enabled()

if true, the mouse's scroll wheel can be used to navigate the scroll view.

### drag_to_rearrange_enabled {#drag_to_rearrange_enabled}

```gdscript
drag_to_rearrange_enabled: bool
```

- void set_drag_to_rearrange_enabled(value: bool)
- bool get_drag_to_rearrange_enabled()

If true, tabs can be rearranged with mouse drag.

### switch_on_drag_hover {#switch_on_drag_hover}

```gdscript
switch_on_drag_hover: bool
```

- void set_switch_on_drag_hover(value: bool)
- bool get_switch_on_drag_hover()

If true, hovering over a tab while dragging something will switch to that tab. Does not have effect when hovering another tab to rearrange. The delay for when this happens is dictated by hover_switch_wait_msec.

### tabs_rearrange_group {#tabs_rearrange_group}

```gdscript
tabs_rearrange_group: int
```

- void set_tabs_rearrange_group(value: int)
- int get_tabs_rearrange_group()

TabBars with the same rearrange group ID will allow dragging the tabs between them. Enable drag with drag_to_rearrange_enabled.

Setting this to -1 will disable rearranging between TabBars.

### scroll_to_selected {#scroll_to_selected}

```gdscript
scroll_to_selected: bool
```

- void set_scroll_to_selected(value: bool)
- bool get_scroll_to_selected()

If true, the tab offset will be changed to keep the currently selected tab visible.

### select_with_rmb {#select_with_rmb}

```gdscript
select_with_rmb: bool
```

- void set_select_with_rmb(value: bool)
- bool get_select_with_rmb()

If true, enables selecting a tab with the right mouse button.

### deselect_enabled {#deselect_enabled}

```gdscript
deselect_enabled: bool
```

- void set_deselect_enabled(value: bool)
- bool get_deselect_enabled()

If true, all tabs can be deselected so that no tab is selected. Click on the current tab to deselect it.

### tab_count {#tab_count}

```gdscript
tab_count: int
```

- void set_tab_count(value: int)
- int get_tab_count()

The number of tabs currently in the bar.

## 信号

### tab_selected {#tab_selected}

```gdscript
signal tab_selected(tab: int)
```

Emitted when a tab is selected via click, directional input, or script, even if it is the current tab.

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

### tab_rmb_clicked {#tab_rmb_clicked}

```gdscript
signal tab_rmb_clicked(tab: int)
```

Emitted when a tab is right-clicked.

### tab_close_pressed {#tab_close_pressed}

```gdscript
signal tab_close_pressed(tab: int)
```

Emitted when a tab's close button is pressed or, if close_with_middle_mouse is true, when middle-clicking on a tab.

Note: Tabs are not removed automatically; this behavior needs to be coded manually. For example:

```
$TabBar.tab_close_pressed.connect($TabBar.remove_tab)
```

```
GetNode("TabBar").TabClosePressed += GetNode("TabBar").RemoveTab;
```

### tab_button_pressed {#tab_button_pressed}

```gdscript
signal tab_button_pressed(tab: int)
```

Emitted when a tab's right button is pressed. See set_tab_button_icon().

### tab_hovered {#tab_hovered}

```gdscript
signal tab_hovered(tab: int)
```

Emitted when a tab is hovered by the mouse.

### active_tab_rearranged {#active_tab_rearranged}

```gdscript
signal active_tab_rearranged(idx_to: int)
```

Emitted when the active tab is rearranged via mouse drag. See drag_to_rearrange_enabled.

## 枚举

### AlignmentMode {#enum-AlignmentMode}

```gdscript
enum AlignmentMode { ALIGNMENT_LEFT = 0, ALIGNMENT_CENTER = 1, ALIGNMENT_RIGHT = 2, ALIGNMENT_MAX = 3 }
```

AlignmentMode ALIGNMENT_LEFT = 0

Aligns tabs to the left.

AlignmentMode ALIGNMENT_CENTER = 1

Aligns tabs in the middle.

AlignmentMode ALIGNMENT_RIGHT = 2

Aligns tabs to the right.

AlignmentMode ALIGNMENT_MAX = 3

Represents the size of the AlignmentMode enum.

#### ALIGNMENT_LEFT {#ALIGNMENT_LEFT}

```gdscript
ALIGNMENT_LEFT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ALIGNMENT_CENTER {#ALIGNMENT_CENTER}

```gdscript
ALIGNMENT_CENTER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ALIGNMENT_RIGHT {#ALIGNMENT_RIGHT}

```gdscript
ALIGNMENT_RIGHT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ALIGNMENT_MAX {#ALIGNMENT_MAX}

```gdscript
ALIGNMENT_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CloseButtonDisplayPolicy {#enum-CloseButtonDisplayPolicy}

```gdscript
enum CloseButtonDisplayPolicy { CLOSE_BUTTON_SHOW_NEVER = 0, CLOSE_BUTTON_SHOW_ACTIVE_ONLY = 1, CLOSE_BUTTON_SHOW_ALWAYS = 2, CLOSE_BUTTON_MAX = 3 }
```

CloseButtonDisplayPolicy CLOSE_BUTTON_SHOW_NEVER = 0

Never show the close buttons.

CloseButtonDisplayPolicy CLOSE_BUTTON_SHOW_ACTIVE_ONLY = 1

Only show the close button on the currently active tab.

CloseButtonDisplayPolicy CLOSE_BUTTON_SHOW_ALWAYS = 2

Show the close button on all tabs.

CloseButtonDisplayPolicy CLOSE_BUTTON_MAX = 3

Represents the size of the CloseButtonDisplayPolicy enum.

#### CLOSE_BUTTON_SHOW_NEVER {#CLOSE_BUTTON_SHOW_NEVER}

```gdscript
CLOSE_BUTTON_SHOW_NEVER = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CLOSE_BUTTON_SHOW_ACTIVE_ONLY {#CLOSE_BUTTON_SHOW_ACTIVE_ONLY}

```gdscript
CLOSE_BUTTON_SHOW_ACTIVE_ONLY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CLOSE_BUTTON_SHOW_ALWAYS {#CLOSE_BUTTON_SHOW_ALWAYS}

```gdscript
CLOSE_BUTTON_SHOW_ALWAYS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CLOSE_BUTTON_MAX {#CLOSE_BUTTON_MAX}

```gdscript
CLOSE_BUTTON_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
