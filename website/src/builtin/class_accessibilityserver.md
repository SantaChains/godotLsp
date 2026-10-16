# AccessibilityServer

**继承**：[Object](class_object.md) → **AccessibilityServer**

## 方法

### create_element {#create_element}

```gdscript
RID create_element(window_id: int, role: AccessibilityServer.AccessibilityRole)
```

Creates a new, empty accessibility element resource.

Note: An accessibility element is created and freed automatically for each Node. In general, this function should not be called manually.

### create_sub_element {#create_sub_element}

```gdscript
RID create_sub_element(parent_rid: RID, role: AccessibilityServer.AccessibilityRole, insert_pos: int = -1)
```

Creates a new, empty accessibility sub-element resource. Sub-elements can be used to provide accessibility information for objects which are not Nodes, such as list items, table cells, or menu items. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the free_element() method.

### create_sub_text_edit_elements {#create_sub_text_edit_elements}

```gdscript
RID create_sub_text_edit_elements(parent_rid: RID, shaped_text: RID, min_height: float, insert_pos: int = -1, is_last_line: bool = false)
```

Creates a new, empty accessibility sub-element from the shaped text buffer. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the free_element() method.

If is_last_line is true, no trailing newline is appended to the text content. Set to true for the last line in multi-line text fields and for single-line text fields.

### element_get_meta {#element_get_meta}

```gdscript
Variant element_get_meta(id: RID) const
```

Returns the metadata of the accessibility element id.

### element_set_meta {#element_set_meta}

```gdscript
void element_set_meta(id: RID, meta: Variant)
```

Sets the metadata of the accessibility element id to meta.

### free_element {#free_element}

```gdscript
void free_element(id: RID)
```

Frees the accessibility element id created by create_element(), create_sub_element(), or create_sub_text_edit_elements().

### get_window_root {#get_window_root}

```gdscript
RID get_window_root(window_id: int) const
```

Returns the main accessibility element of the OS native window.

### has_element {#has_element}

```gdscript
bool has_element(id: RID) const
```

Returns true if id is a valid accessibility element.

### is_supported {#is_supported}

```gdscript
bool is_supported() const
```

Returns true if screen reader is support by this implementation.

### set_window_focused {#set_window_focused}

```gdscript
void set_window_focused(window_id: int, focused: bool)
```

Sets the window focused state for assistive apps.

Note: This method is implemented on Linux, macOS, and Windows.

Note: Advanced users only! Window objects call this method automatically.

### set_window_rect {#set_window_rect}

```gdscript
void set_window_rect(window_id: int, rect_out: Rect2, rect_in: Rect2)
```

Sets window outer (with decorations) and inner (without decorations) bounds for assistive apps.

Note: This method is implemented on Linux, macOS, and Windows.

Note: Advanced users only! Window objects call this method automatically.

### update_add_action {#update_add_action}

```gdscript
void update_add_action(id: RID, action: AccessibilityServer.AccessibilityAction, callable: Callable)
```

Adds a callback for the accessibility action (action which can be performed by using a special screen reader command or buttons on the Braille display), and marks this action as supported. The action callback receives one Variant argument, which value depends on action type.

### update_add_child {#update_add_child}

```gdscript
void update_add_child(id: RID, child_id: RID)
```

Adds a child accessibility element.

Note: Node children and sub-elements are added to the child list automatically.

### update_add_custom_action {#update_add_custom_action}

```gdscript
void update_add_custom_action(id: RID, action_id: int, action_description: String)
```

Adds support for a custom accessibility action. action_id is passed as an argument to the callback of ACTION_CUSTOM action.

### update_add_related_controls {#update_add_related_controls}

```gdscript
void update_add_related_controls(id: RID, related_id: RID)
```

Adds an element that is controlled by this element.

### update_add_related_described_by {#update_add_related_described_by}

```gdscript
void update_add_related_described_by(id: RID, related_id: RID)
```

Adds an element that describes this element.

### update_add_related_details {#update_add_related_details}

```gdscript
void update_add_related_details(id: RID, related_id: RID)
```

Adds an element that details this element.

### update_add_related_flow_to {#update_add_related_flow_to}

```gdscript
void update_add_related_flow_to(id: RID, related_id: RID)
```

Adds an element that this element flow into.

### update_add_related_labeled_by {#update_add_related_labeled_by}

```gdscript
void update_add_related_labeled_by(id: RID, related_id: RID)
```

Adds an element that labels this element.

### update_add_related_radio_group {#update_add_related_radio_group}

```gdscript
void update_add_related_radio_group(id: RID, related_id: RID)
```

Adds an element that is part of the same radio group.

Note: This method should be called on each element of the group, using all other elements as related_id.

### update_set_active_descendant {#update_set_active_descendant}

```gdscript
void update_set_active_descendant(id: RID, other_id: RID)
```

Adds an element that is an active descendant of this element.

### update_set_background_color {#update_set_background_color}

```gdscript
void update_set_background_color(id: RID, color: Color)
```

Sets element background color.

### update_set_bounds {#update_set_bounds}

```gdscript
void update_set_bounds(id: RID, rect: Rect2)
```

Sets element bounding box, relative to the node position.

### update_set_braille_label {#update_set_braille_label}

```gdscript
void update_set_braille_label(id: RID, name: String)
```

Sets element accessibility label for Braille display.

### update_set_braille_role_description {#update_set_braille_role_description}

```gdscript
void update_set_braille_role_description(id: RID, description: String)
```

Sets element accessibility role description for Braille display.

### update_set_checked {#update_set_checked}

```gdscript
void update_set_checked(id: RID, checekd: bool)
```

Sets element checked state.

### update_set_classname {#update_set_classname}

```gdscript
void update_set_classname(id: RID, classname: String)
```

Sets element class name.

### update_set_color_value {#update_set_color_value}

```gdscript
void update_set_color_value(id: RID, color: Color)
```

Sets element color value.

### update_set_description {#update_set_description}

```gdscript
void update_set_description(id: RID, description: String)
```

Sets element accessibility description.

### update_set_error_message {#update_set_error_message}

```gdscript
void update_set_error_message(id: RID, other_id: RID)
```

Sets an element which contains an error message for this element.

### update_set_extra_info {#update_set_extra_info}

```gdscript
void update_set_extra_info(id: RID, name: String)
```

Sets element accessibility extra information added to the element name.

### update_set_flag {#update_set_flag}

```gdscript
void update_set_flag(id: RID, flag: AccessibilityServer.AccessibilityFlags, value: bool)
```

Sets element flag.

### update_set_focus {#update_set_focus}

```gdscript
void update_set_focus(id: RID)
```

Sets currently focused element.

### update_set_foreground_color {#update_set_foreground_color}

```gdscript
void update_set_foreground_color(id: RID, color: Color)
```

Sets element foreground color.

### update_set_in_page_link_target {#update_set_in_page_link_target}

```gdscript
void update_set_in_page_link_target(id: RID, other_id: RID)
```

Sets target element for the link.

### update_set_language {#update_set_language}

```gdscript
void update_set_language(id: RID, language: String)
```

Sets element text language.

### update_set_list_item_count {#update_set_list_item_count}

```gdscript
void update_set_list_item_count(id: RID, size: int)
```

Sets number of items in the list.

### update_set_list_item_expanded {#update_set_list_item_expanded}

```gdscript
void update_set_list_item_expanded(id: RID, expanded: bool)
```

Sets list/tree item expanded status.

### update_set_list_item_index {#update_set_list_item_index}

```gdscript
void update_set_list_item_index(id: RID, index: int)
```

Sets the position of the element in the list.

### update_set_list_item_level {#update_set_list_item_level}

```gdscript
void update_set_list_item_level(id: RID, level: int)
```

Sets the hierarchical level of the element in the list.

### update_set_list_item_selected {#update_set_list_item_selected}

```gdscript
void update_set_list_item_selected(id: RID, selected: bool)
```

Sets list/tree item selected status.

### update_set_list_orientation {#update_set_list_orientation}

```gdscript
void update_set_list_orientation(id: RID, vertical: bool)
```

Sets the orientation of the list elements.

### update_set_live {#update_set_live}

```gdscript
void update_set_live(id: RID, live: AccessibilityServer.AccessibilityLiveMode)
```

Sets the priority of the live region updates.

### update_set_member_of {#update_set_member_of}

```gdscript
void update_set_member_of(id: RID, group_id: RID)
```

Sets the element to be a member of the group.

### update_set_name {#update_set_name}

```gdscript
void update_set_name(id: RID, name: String)
```

Sets element accessibility name.

### update_set_next_on_line {#update_set_next_on_line}

```gdscript
void update_set_next_on_line(id: RID, other_id: RID)
```

Sets next element on the line.

### update_set_num_jump {#update_set_num_jump}

```gdscript
void update_set_num_jump(id: RID, jump: float)
```

Sets numeric value jump.

### update_set_num_range {#update_set_num_range}

```gdscript
void update_set_num_range(id: RID, min: float, max: float)
```

Sets numeric value range.

### update_set_num_step {#update_set_num_step}

```gdscript
void update_set_num_step(id: RID, step: float)
```

Sets numeric value step.

### update_set_num_value {#update_set_num_value}

```gdscript
void update_set_num_value(id: RID, position: float)
```

Sets numeric value.

### update_set_placeholder {#update_set_placeholder}

```gdscript
void update_set_placeholder(id: RID, placeholder: String)
```

Sets placeholder text.

### update_set_popup_type {#update_set_popup_type}

```gdscript
void update_set_popup_type(id: RID, popup: AccessibilityServer.AccessibilityPopupType)
```

Sets popup type for popup buttons.

### update_set_previous_on_line {#update_set_previous_on_line}

```gdscript
void update_set_previous_on_line(id: RID, other_id: RID)
```

Sets previous element on the line.

### update_set_role {#update_set_role}

```gdscript
void update_set_role(id: RID, role: AccessibilityServer.AccessibilityRole)
```

Sets element accessibility role.

### update_set_role_description {#update_set_role_description}

```gdscript
void update_set_role_description(id: RID, description: String)
```

Sets element accessibility role description text.

### update_set_scroll_x {#update_set_scroll_x}

```gdscript
void update_set_scroll_x(id: RID, position: float)
```

Sets scroll bar x position.

### update_set_scroll_x_range {#update_set_scroll_x_range}

```gdscript
void update_set_scroll_x_range(id: RID, min: float, max: float)
```

Sets scroll bar x range.

### update_set_scroll_y {#update_set_scroll_y}

```gdscript
void update_set_scroll_y(id: RID, position: float)
```

Sets scroll bar y position.

### update_set_scroll_y_range {#update_set_scroll_y_range}

```gdscript
void update_set_scroll_y_range(id: RID, min: float, max: float)
```

Sets scroll bar y range.

### update_set_shortcut {#update_set_shortcut}

```gdscript
void update_set_shortcut(id: RID, shortcut: String)
```

Sets the list of keyboard shortcuts used by element.

### update_set_state_description {#update_set_state_description}

```gdscript
void update_set_state_description(id: RID, description: String)
```

Sets human-readable description of the current checked state.

### update_set_table_cell_position {#update_set_table_cell_position}

```gdscript
void update_set_table_cell_position(id: RID, row_index: int, column_index: int)
```

Sets cell position in the table.

### update_set_table_cell_span {#update_set_table_cell_span}

```gdscript
void update_set_table_cell_span(id: RID, row_span: int, column_span: int)
```

Sets cell row/column span.

### update_set_table_column_count {#update_set_table_column_count}

```gdscript
void update_set_table_column_count(id: RID, count: int)
```

Sets number of columns in the table.

### update_set_table_column_index {#update_set_table_column_index}

```gdscript
void update_set_table_column_index(id: RID, index: int)
```

Sets position of the column.

### update_set_table_row_count {#update_set_table_row_count}

```gdscript
void update_set_table_row_count(id: RID, count: int)
```

Sets number of rows in the table.

### update_set_table_row_index {#update_set_table_row_index}

```gdscript
void update_set_table_row_index(id: RID, index: int)
```

Sets position of the row in the table.

### update_set_text_align {#update_set_text_align}

```gdscript
void update_set_text_align(id: RID, align: HorizontalAlignment)
```

Sets element text alignment.

### update_set_text_decorations {#update_set_text_decorations}

```gdscript
void update_set_text_decorations(id: RID, underline: bool, strikethrough: bool, overline: bool, color: Color = Color(0, 0, 0, 1))
```

Sets text underline/overline/strikethrough.

### update_set_text_orientation {#update_set_text_orientation}

```gdscript
void update_set_text_orientation(id: RID, vertical: bool)
```

Sets text orientation.

### update_set_text_selection {#update_set_text_selection}

```gdscript
void update_set_text_selection(id: RID, text_start_id: RID, start_char: int, text_end_id: RID, end_char: int)
```

Sets text selection to the text field. text_start_id and text_end_id should be elements created by create_sub_text_edit_elements(). Character offsets are relative to the corresponding element.

### update_set_tooltip {#update_set_tooltip}

```gdscript
void update_set_tooltip(id: RID, tooltip: String)
```

Sets tooltip text.

### update_set_transform {#update_set_transform}

```gdscript
void update_set_transform(id: RID, transform: Transform2D)
```

Sets element 2D transform.

### update_set_url {#update_set_url}

```gdscript
void update_set_url(id: RID, url: String)
```

Sets link URL.

### update_set_value {#update_set_value}

```gdscript
void update_set_value(id: RID, value: String)
```

Sets element text value.

## 枚举

### AccessibilityRole {#enum-AccessibilityRole}

```gdscript
enum AccessibilityRole { ROLE_UNKNOWN = 0, ROLE_DEFAULT_BUTTON = 1, ROLE_AUDIO = 2, ROLE_VIDEO = 3, ROLE_STATIC_TEXT = 4, ROLE_CONTAINER = 5, ROLE_PANEL = 6, ROLE_BUTTON = 7, ROLE_LINK = 8, ROLE_CHECK_BOX = 9, ROLE_RADIO_BUTTON = 10, ROLE_CHECK_BUTTON = 11, ROLE_SCROLL_BAR = 12, ROLE_SCROLL_VIEW = 13, ROLE_SPLITTER = 14, ROLE_SLIDER = 15, ROLE_SPIN_BUTTON = 16, ROLE_PROGRESS_INDICATOR = 17, ROLE_TEXT_FIELD = 18, ROLE_MULTILINE_TEXT_FIELD = 19, ROLE_COLOR_PICKER = 20, ROLE_TABLE = 21, ROLE_CELL = 22, ROLE_ROW = 23, ROLE_ROW_GROUP = 24, ROLE_ROW_HEADER = 25, ROLE_COLUMN_HEADER = 26, ROLE_TREE = 27, ROLE_TREE_ITEM = 28, ROLE_LIST = 29, ROLE_LIST_ITEM = 30, ROLE_LIST_BOX = 31, ROLE_LIST_BOX_OPTION = 32, ROLE_TAB_BAR = 33, ROLE_TAB = 34, ROLE_TAB_PANEL = 35, ROLE_MENU_BAR = 36, ROLE_MENU = 37, ROLE_MENU_ITEM = 38, ROLE_MENU_ITEM_CHECK_BOX = 39, ROLE_MENU_ITEM_RADIO = 40, ROLE_IMAGE = 41, ROLE_WINDOW = 42, ROLE_TITLE_BAR = 43, ROLE_DIALOG = 44, ROLE_TOOLTIP = 45, ROLE_REGION = 46, ROLE_TEXT_RUN = 47 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROLE_UNKNOWN {#ROLE_UNKNOWN}

```gdscript
ROLE_UNKNOWN = 0
```

Unknown or custom role.

#### ROLE_DEFAULT_BUTTON {#ROLE_DEFAULT_BUTTON}

```gdscript
ROLE_DEFAULT_BUTTON = 1
```

Default dialog button element.

#### ROLE_AUDIO {#ROLE_AUDIO}

```gdscript
ROLE_AUDIO = 2
```

Audio player element.

#### ROLE_VIDEO {#ROLE_VIDEO}

```gdscript
ROLE_VIDEO = 3
```

Video player element.

#### ROLE_STATIC_TEXT {#ROLE_STATIC_TEXT}

```gdscript
ROLE_STATIC_TEXT = 4
```

Non-editable text label.

#### ROLE_CONTAINER {#ROLE_CONTAINER}

```gdscript
ROLE_CONTAINER = 5
```

Container element. Elements with this role are used for internal structure and ignored by screen readers.

#### ROLE_PANEL {#ROLE_PANEL}

```gdscript
ROLE_PANEL = 6
```

Panel container element.

#### ROLE_BUTTON {#ROLE_BUTTON}

```gdscript
ROLE_BUTTON = 7
```

Button element.

#### ROLE_LINK {#ROLE_LINK}

```gdscript
ROLE_LINK = 8
```

Link element.

#### ROLE_CHECK_BOX {#ROLE_CHECK_BOX}

```gdscript
ROLE_CHECK_BOX = 9
```

Check box element.

#### ROLE_RADIO_BUTTON {#ROLE_RADIO_BUTTON}

```gdscript
ROLE_RADIO_BUTTON = 10
```

Radio button element.

#### ROLE_CHECK_BUTTON {#ROLE_CHECK_BUTTON}

```gdscript
ROLE_CHECK_BUTTON = 11
```

Check button element.

#### ROLE_SCROLL_BAR {#ROLE_SCROLL_BAR}

```gdscript
ROLE_SCROLL_BAR = 12
```

Scroll bar element.

#### ROLE_SCROLL_VIEW {#ROLE_SCROLL_VIEW}

```gdscript
ROLE_SCROLL_VIEW = 13
```

Scroll container element.

#### ROLE_SPLITTER {#ROLE_SPLITTER}

```gdscript
ROLE_SPLITTER = 14
```

Container splitter handle element.

#### ROLE_SLIDER {#ROLE_SLIDER}

```gdscript
ROLE_SLIDER = 15
```

Slider element.

#### ROLE_SPIN_BUTTON {#ROLE_SPIN_BUTTON}

```gdscript
ROLE_SPIN_BUTTON = 16
```

Spin box element.

#### ROLE_PROGRESS_INDICATOR {#ROLE_PROGRESS_INDICATOR}

```gdscript
ROLE_PROGRESS_INDICATOR = 17
```

Progress indicator element.

#### ROLE_TEXT_FIELD {#ROLE_TEXT_FIELD}

```gdscript
ROLE_TEXT_FIELD = 18
```

Editable text field element.

#### ROLE_MULTILINE_TEXT_FIELD {#ROLE_MULTILINE_TEXT_FIELD}

```gdscript
ROLE_MULTILINE_TEXT_FIELD = 19
```

Multiline editable text field element.

#### ROLE_COLOR_PICKER {#ROLE_COLOR_PICKER}

```gdscript
ROLE_COLOR_PICKER = 20
```

Color picker element.

#### ROLE_TABLE {#ROLE_TABLE}

```gdscript
ROLE_TABLE = 21
```

Table element.

#### ROLE_CELL {#ROLE_CELL}

```gdscript
ROLE_CELL = 22
```

Table/tree cell element.

#### ROLE_ROW {#ROLE_ROW}

```gdscript
ROLE_ROW = 23
```

Table/tree row element.

#### ROLE_ROW_GROUP {#ROLE_ROW_GROUP}

```gdscript
ROLE_ROW_GROUP = 24
```

Table/tree row group element.

#### ROLE_ROW_HEADER {#ROLE_ROW_HEADER}

```gdscript
ROLE_ROW_HEADER = 25
```

Table/tree row header element.

#### ROLE_COLUMN_HEADER {#ROLE_COLUMN_HEADER}

```gdscript
ROLE_COLUMN_HEADER = 26
```

Table/tree column header element.

#### ROLE_TREE {#ROLE_TREE}

```gdscript
ROLE_TREE = 27
```

Tree view element.

#### ROLE_TREE_ITEM {#ROLE_TREE_ITEM}

```gdscript
ROLE_TREE_ITEM = 28
```

Tree view item element.

#### ROLE_LIST {#ROLE_LIST}

```gdscript
ROLE_LIST = 29
```

List element.

#### ROLE_LIST_ITEM {#ROLE_LIST_ITEM}

```gdscript
ROLE_LIST_ITEM = 30
```

List item element.

#### ROLE_LIST_BOX {#ROLE_LIST_BOX}

```gdscript
ROLE_LIST_BOX = 31
```

List view element.

#### ROLE_LIST_BOX_OPTION {#ROLE_LIST_BOX_OPTION}

```gdscript
ROLE_LIST_BOX_OPTION = 32
```

List view item element.

#### ROLE_TAB_BAR {#ROLE_TAB_BAR}

```gdscript
ROLE_TAB_BAR = 33
```

Tab bar element.

#### ROLE_TAB {#ROLE_TAB}

```gdscript
ROLE_TAB = 34
```

Tab bar item element.

#### ROLE_TAB_PANEL {#ROLE_TAB_PANEL}

```gdscript
ROLE_TAB_PANEL = 35
```

Tab panel element.

#### ROLE_MENU_BAR {#ROLE_MENU_BAR}

```gdscript
ROLE_MENU_BAR = 36
```

Menu bar element.

#### ROLE_MENU {#ROLE_MENU}

```gdscript
ROLE_MENU = 37
```

Popup menu element.

#### ROLE_MENU_ITEM {#ROLE_MENU_ITEM}

```gdscript
ROLE_MENU_ITEM = 38
```

Popup menu item element.

#### ROLE_MENU_ITEM_CHECK_BOX {#ROLE_MENU_ITEM_CHECK_BOX}

```gdscript
ROLE_MENU_ITEM_CHECK_BOX = 39
```

Popup menu check button item element.

#### ROLE_MENU_ITEM_RADIO {#ROLE_MENU_ITEM_RADIO}

```gdscript
ROLE_MENU_ITEM_RADIO = 40
```

Popup menu radio button item element.

#### ROLE_IMAGE {#ROLE_IMAGE}

```gdscript
ROLE_IMAGE = 41
```

Image element.

#### ROLE_WINDOW {#ROLE_WINDOW}

```gdscript
ROLE_WINDOW = 42
```

Window element.

#### ROLE_TITLE_BAR {#ROLE_TITLE_BAR}

```gdscript
ROLE_TITLE_BAR = 43
```

Embedded window title bar element.

#### ROLE_DIALOG {#ROLE_DIALOG}

```gdscript
ROLE_DIALOG = 44
```

Dialog window element.

#### ROLE_TOOLTIP {#ROLE_TOOLTIP}

```gdscript
ROLE_TOOLTIP = 45
```

Tooltip element.

#### ROLE_REGION {#ROLE_REGION}

```gdscript
ROLE_REGION = 46
```

Region/landmark element. Screen readers can navigate between regions using landmark navigation.

#### ROLE_TEXT_RUN {#ROLE_TEXT_RUN}

```gdscript
ROLE_TEXT_RUN = 47
```

Unifor text run.

Note: This role is used for internal text elements, and should not be assigned to nodes.

---

### AccessibilityPopupType {#enum-AccessibilityPopupType}

```gdscript
enum AccessibilityPopupType { POPUP_MENU = 0, POPUP_LIST = 1, POPUP_TREE = 2, POPUP_DIALOG = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### POPUP_MENU {#POPUP_MENU}

```gdscript
POPUP_MENU = 0
```

Popup menu.

#### POPUP_LIST {#POPUP_LIST}

```gdscript
POPUP_LIST = 1
```

Popup list.

#### POPUP_TREE {#POPUP_TREE}

```gdscript
POPUP_TREE = 2
```

Popup tree view.

#### POPUP_DIALOG {#POPUP_DIALOG}

```gdscript
POPUP_DIALOG = 3
```

Popup dialog.

---

### AccessibilityFlags {#enum-AccessibilityFlags}

```gdscript
enum AccessibilityFlags { FLAG_HIDDEN = 0, FLAG_MULTISELECTABLE = 1, FLAG_REQUIRED = 2, FLAG_VISITED = 3, FLAG_BUSY = 4, FLAG_MODAL = 5, FLAG_TOUCH_PASSTHROUGH = 6, FLAG_READONLY = 7, FLAG_DISABLED = 8, FLAG_CLIPS_CHILDREN = 9 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_HIDDEN {#FLAG_HIDDEN}

```gdscript
FLAG_HIDDEN = 0
```

Element is hidden for accessibility tools.

#### FLAG_MULTISELECTABLE {#FLAG_MULTISELECTABLE}

```gdscript
FLAG_MULTISELECTABLE = 1
```

Element supports multiple item selection.

#### FLAG_REQUIRED {#FLAG_REQUIRED}

```gdscript
FLAG_REQUIRED = 2
```

Element require user input.

#### FLAG_VISITED {#FLAG_VISITED}

```gdscript
FLAG_VISITED = 3
```

Element is a visited link.

#### FLAG_BUSY {#FLAG_BUSY}

```gdscript
FLAG_BUSY = 4
```

Element content is not ready (e.g. loading).

#### FLAG_MODAL {#FLAG_MODAL}

```gdscript
FLAG_MODAL = 5
```

Element is modal window.

#### FLAG_TOUCH_PASSTHROUGH {#FLAG_TOUCH_PASSTHROUGH}

```gdscript
FLAG_TOUCH_PASSTHROUGH = 6
```

Element allows touches to be passed through when a screen reader is in touch exploration mode.

#### FLAG_READONLY {#FLAG_READONLY}

```gdscript
FLAG_READONLY = 7
```

Element is text field with selectable but read-only text.

#### FLAG_DISABLED {#FLAG_DISABLED}

```gdscript
FLAG_DISABLED = 8
```

Element is disabled.

#### FLAG_CLIPS_CHILDREN {#FLAG_CLIPS_CHILDREN}

```gdscript
FLAG_CLIPS_CHILDREN = 9
```

Element clips children.

---

### AccessibilityAction {#enum-AccessibilityAction}

```gdscript
enum AccessibilityAction { ACTION_CLICK = 0, ACTION_FOCUS = 1, ACTION_BLUR = 2, ACTION_COLLAPSE = 3, ACTION_EXPAND = 4, ACTION_DECREMENT = 5, ACTION_INCREMENT = 6, ACTION_HIDE_TOOLTIP = 7, ACTION_SHOW_TOOLTIP = 8, ACTION_SET_TEXT_SELECTION = 9, ACTION_REPLACE_SELECTED_TEXT = 10, ACTION_SCROLL_BACKWARD = 11, ACTION_SCROLL_DOWN = 12, ACTION_SCROLL_FORWARD = 13, ACTION_SCROLL_LEFT = 14, ACTION_SCROLL_RIGHT = 15, ACTION_SCROLL_UP = 16, ACTION_SCROLL_INTO_VIEW = 17, ACTION_SCROLL_TO_POINT = 18, ACTION_SET_SCROLL_OFFSET = 19, ACTION_SET_VALUE = 20, ACTION_SHOW_CONTEXT_MENU = 21, ACTION_CUSTOM = 22 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACTION_CLICK {#ACTION_CLICK}

```gdscript
ACTION_CLICK = 0
```

Single click action, callback argument is not set.

#### ACTION_FOCUS {#ACTION_FOCUS}

```gdscript
ACTION_FOCUS = 1
```

Focus action, callback argument is not set.

#### ACTION_BLUR {#ACTION_BLUR}

```gdscript
ACTION_BLUR = 2
```

Blur action, callback argument is not set.

#### ACTION_COLLAPSE {#ACTION_COLLAPSE}

```gdscript
ACTION_COLLAPSE = 3
```

Collapse action, callback argument is not set.

#### ACTION_EXPAND {#ACTION_EXPAND}

```gdscript
ACTION_EXPAND = 4
```

Expand action, callback argument is not set.

#### ACTION_DECREMENT {#ACTION_DECREMENT}

```gdscript
ACTION_DECREMENT = 5
```

Decrement action, callback argument is not set.

#### ACTION_INCREMENT {#ACTION_INCREMENT}

```gdscript
ACTION_INCREMENT = 6
```

Increment action, callback argument is not set.

#### ACTION_HIDE_TOOLTIP {#ACTION_HIDE_TOOLTIP}

```gdscript
ACTION_HIDE_TOOLTIP = 7
```

Hide tooltip action, callback argument is not set.

#### ACTION_SHOW_TOOLTIP {#ACTION_SHOW_TOOLTIP}

```gdscript
ACTION_SHOW_TOOLTIP = 8
```

Show tooltip action, callback argument is not set.

#### ACTION_SET_TEXT_SELECTION {#ACTION_SET_TEXT_SELECTION}

```gdscript
ACTION_SET_TEXT_SELECTION = 9
```

Set text selection action, callback argument is set to Dictionary with the following keys:

- "start_element" accessibility element of the selection start.
- "start_char" character offset relative to the accessibility element of the selection start.
- "end_element" accessibility element of the selection end.
- "end_char" character offset relative to the accessibility element of the selection end.

#### ACTION_REPLACE_SELECTED_TEXT {#ACTION_REPLACE_SELECTED_TEXT}

```gdscript
ACTION_REPLACE_SELECTED_TEXT = 10
```

Replace text action, callback argument is set to String with the replacement text.

#### ACTION_SCROLL_BACKWARD {#ACTION_SCROLL_BACKWARD}

```gdscript
ACTION_SCROLL_BACKWARD = 11
```

Scroll backward action, callback argument is not set.

#### ACTION_SCROLL_DOWN {#ACTION_SCROLL_DOWN}

```gdscript
ACTION_SCROLL_DOWN = 12
```

Scroll down action, callback argument is set to AccessibilityScrollUnit.

#### ACTION_SCROLL_FORWARD {#ACTION_SCROLL_FORWARD}

```gdscript
ACTION_SCROLL_FORWARD = 13
```

Scroll forward action, callback argument is not set.

#### ACTION_SCROLL_LEFT {#ACTION_SCROLL_LEFT}

```gdscript
ACTION_SCROLL_LEFT = 14
```

Scroll left action, callback argument is set to AccessibilityScrollUnit.

#### ACTION_SCROLL_RIGHT {#ACTION_SCROLL_RIGHT}

```gdscript
ACTION_SCROLL_RIGHT = 15
```

Scroll right action, callback argument is set to AccessibilityScrollUnit.

#### ACTION_SCROLL_UP {#ACTION_SCROLL_UP}

```gdscript
ACTION_SCROLL_UP = 16
```

Scroll up action, callback argument is set to AccessibilityScrollUnit.

#### ACTION_SCROLL_INTO_VIEW {#ACTION_SCROLL_INTO_VIEW}

```gdscript
ACTION_SCROLL_INTO_VIEW = 17
```

Scroll into view action, callback argument is set to AccessibilityScrollHint.

#### ACTION_SCROLL_TO_POINT {#ACTION_SCROLL_TO_POINT}

```gdscript
ACTION_SCROLL_TO_POINT = 18
```

Scroll to point action, callback argument is set to Vector2 with the relative point coordinates.

#### ACTION_SET_SCROLL_OFFSET {#ACTION_SET_SCROLL_OFFSET}

```gdscript
ACTION_SET_SCROLL_OFFSET = 19
```

Set scroll offset action, callback argument is set to Vector2 with the scroll offset.

#### ACTION_SET_VALUE {#ACTION_SET_VALUE}

```gdscript
ACTION_SET_VALUE = 20
```

Set value action, callback argument is set to String or number with the new value.

#### ACTION_SHOW_CONTEXT_MENU {#ACTION_SHOW_CONTEXT_MENU}

```gdscript
ACTION_SHOW_CONTEXT_MENU = 21
```

Show context menu action, callback argument is not set.

#### ACTION_CUSTOM {#ACTION_CUSTOM}

```gdscript
ACTION_CUSTOM = 22
```

Custom action, callback argument is set to the integer action ID.

---

### AccessibilityLiveMode {#enum-AccessibilityLiveMode}

```gdscript
enum AccessibilityLiveMode { LIVE_OFF = 0, LIVE_POLITE = 1, LIVE_ASSERTIVE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIVE_OFF {#LIVE_OFF}

```gdscript
LIVE_OFF = 0
```

Indicates that updates to the live region should not be presented.

#### LIVE_POLITE {#LIVE_POLITE}

```gdscript
LIVE_POLITE = 1
```

Indicates that updates to the live region should be presented at the next opportunity (for example at the end of speaking the current sentence).

#### LIVE_ASSERTIVE {#LIVE_ASSERTIVE}

```gdscript
LIVE_ASSERTIVE = 2
```

Indicates that updates to the live region have the highest priority and should be presented immediately.

---

### AccessibilityScrollUnit {#enum-AccessibilityScrollUnit}

```gdscript
enum AccessibilityScrollUnit { SCROLL_UNIT_ITEM = 0, SCROLL_UNIT_PAGE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCROLL_UNIT_ITEM {#SCROLL_UNIT_ITEM}

```gdscript
SCROLL_UNIT_ITEM = 0
```

The amount by which to scroll. A single item of a list, line of text.

#### SCROLL_UNIT_PAGE {#SCROLL_UNIT_PAGE}

```gdscript
SCROLL_UNIT_PAGE = 1
```

The amount by which to scroll. A single page.

---

### AccessibilityScrollHint {#enum-AccessibilityScrollHint}

```gdscript
enum AccessibilityScrollHint { SCROLL_HINT_TOP_LEFT = 0, SCROLL_HINT_BOTTOM_RIGHT = 1, SCROLL_HINT_TOP_EDGE = 2, SCROLL_HINT_BOTTOM_EDGE = 3, SCROLL_HINT_LEFT_EDGE = 4, SCROLL_HINT_RIGHT_EDGE = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCROLL_HINT_TOP_LEFT {#SCROLL_HINT_TOP_LEFT}

```gdscript
SCROLL_HINT_TOP_LEFT = 0
```

A preferred position for the node scrolled into view. Top-left edge of the scroll container.

#### SCROLL_HINT_BOTTOM_RIGHT {#SCROLL_HINT_BOTTOM_RIGHT}

```gdscript
SCROLL_HINT_BOTTOM_RIGHT = 1
```

A preferred position for the node scrolled into view. Bottom-right edge of the scroll container.

#### SCROLL_HINT_TOP_EDGE {#SCROLL_HINT_TOP_EDGE}

```gdscript
SCROLL_HINT_TOP_EDGE = 2
```

A preferred position for the node scrolled into view. Top edge of the scroll container.

#### SCROLL_HINT_BOTTOM_EDGE {#SCROLL_HINT_BOTTOM_EDGE}

```gdscript
SCROLL_HINT_BOTTOM_EDGE = 3
```

A preferred position for the node scrolled into view. Bottom edge of the scroll container.

#### SCROLL_HINT_LEFT_EDGE {#SCROLL_HINT_LEFT_EDGE}

```gdscript
SCROLL_HINT_LEFT_EDGE = 4
```

A preferred position for the node scrolled into view. Left edge of the scroll container.

#### SCROLL_HINT_RIGHT_EDGE {#SCROLL_HINT_RIGHT_EDGE}

```gdscript
SCROLL_HINT_RIGHT_EDGE = 5
```

A preferred position for the node scrolled into view. Right edge of the scroll container.

---
