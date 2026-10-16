# EditorDock

**继承**：[MarginContainer](class_margincontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorDock**

> EditorDock is a Container node that can be docked in one of the editor's dock slots. Docks are added by plugins to provide space for controls related to an EditorPlugin. The editor comes with a few built-in docks, such as the Scene dock, FileSystem dock, etc.

<details><summary>官方英文描述（中文精译进行中）</summary>

EditorDock is a Container node that can be docked in one of the editor's dock slots. Docks are added by plugins to provide space for controls related to an EditorPlugin. The editor comes with a few built-in docks, such as the Scene dock, FileSystem dock, etc.

You can add a dock by using EditorPlugin.add_dock(). The dock can be customized by changing its properties.

```gdscript
@tool
extends EditorPlugin
# Dock reference.
var dock
# Plugin initialization.
func _enter_tree():
    dock = EditorDock.new()
    dock.title = "My Dock"
    dock.dock_icon = preload("./dock_icon.png")
    dock.default_slot = EditorDock.DOCK_SLOT_RIGHT_UL
    var dock_content = preload("./dock_content.tscn").instantiate()
    dock.add_child(dock_content)
    add_dock(dock)
# Plugin clean-up.
func _exit_tree():
    remove_dock(dock)
    dock.queue_free()
    dock = null
```

</details>

## 方法

### _load_layout_from_config {#_load_layout_from_config}

```gdscript
void _load_layout_from_config(config: ConfigFile, section: String) virtual
```

Implement this method to handle loading this dock's layout. It's equivalent to EditorPlugin._set_window_layout(). section is a unique section based on layout_key.

### _save_layout_to_config {#_save_layout_to_config}

```gdscript
void _save_layout_to_config(config: ConfigFile, section: String) virtual const
```

Implement this method to handle saving this dock's layout. It's equivalent to EditorPlugin._get_window_layout(). section is a unique section based on layout_key.

### _update_layout {#_update_layout}

```gdscript
void _update_layout(layout: int) virtual
```

Implement this method to handle the layout switching for this dock. layout is one of the DockLayout constants.

```gdscript
func _update_layout(layout):
    box_container.vertical = (layout == DOCK_LAYOUT_VERTICAL)
```

### close {#close}

```gdscript
void close()
```

Closes the dock, making its tab hidden.

### get_available_layouts {#get_available_layouts}

```gdscript
bitfield::EditorDock.DockLayout get_available_layouts() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_slot {#get_default_slot}

```gdscript
EditorDock.DockSlot get_default_slot() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dock_icon {#get_dock_icon}

```gdscript
Texture2D get_dock_icon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dock_shortcut {#get_dock_shortcut}

```gdscript
Shortcut get_dock_shortcut() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_force_show_icon {#get_force_show_icon}

```gdscript
bool get_force_show_icon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_icon_name {#get_icon_name}

```gdscript
StringName get_icon_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_layout_key {#get_layout_key}

```gdscript
String get_layout_key() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title {#get_title}

```gdscript
String get_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title_color {#get_title_color}

```gdscript
Color get_title_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_closable {#is_closable}

```gdscript
bool is_closable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_global {#is_global}

```gdscript
bool is_global() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_transient {#is_transient}

```gdscript
bool is_transient() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### make_visible {#make_visible}

```gdscript
void make_visible()
```

Focuses the dock's tab (or window if it's floating). If the dock was closed, it will be opened. If it's a bottom dock, makes the bottom panel visible.

### open {#open}

```gdscript
void open()
```

Opens the dock. It will appear in the last used dock slot. If the dock has no default slot, it will be opened floating.

Note: This does not focus the dock. If you want to open and focus the dock, use make_visible().

### set_available_layouts {#set_available_layouts}

```gdscript
void set_available_layouts(layouts: bitfield::EditorDock.DockLayout)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_closable {#set_closable}

```gdscript
void set_closable(closable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_slot {#set_default_slot}

```gdscript
void set_default_slot(slot: EditorDock.DockSlot)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dock_icon {#set_dock_icon}

```gdscript
void set_dock_icon(icon: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dock_shortcut {#set_dock_shortcut}

```gdscript
void set_dock_shortcut(shortcut: Shortcut)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_force_show_icon {#set_force_show_icon}

```gdscript
void set_force_show_icon(force: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global {#set_global}

```gdscript
void set_global(global: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_icon_name {#set_icon_name}

```gdscript
void set_icon_name(icon_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layout_key {#set_layout_key}

```gdscript
void set_layout_key(layout_key: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title {#set_title}

```gdscript
void set_title(title: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title_color {#set_title_color}

```gdscript
void set_title_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transient {#set_transient}

```gdscript
void set_transient(transient: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### title {#title}

```gdscript
title: String
```

- void set_title(value: String)
- String get_title()

The title of the dock's tab. If empty, the dock's Node.name will be used. If the name is auto-generated (contains @), the first child's name will be used instead.

### layout_key {#layout_key}

```gdscript
layout_key: String
```

- void set_layout_key(value: String)
- String get_layout_key()

The key representing this dock in the editor's layout file. If empty, the dock's displayed name will be used instead.

### global {#global}

```gdscript
global: bool
```

- void set_global(value: bool)
- bool is_global()

If true, the dock appears in the Editor > Editor Docks menu and can be closed. Non-global docks can still be closed using close() or when closable is true.

### transient {#transient}

```gdscript
transient: bool
```

- void set_transient(value: bool)
- bool is_transient()

If true, the dock is not automatically opened or closed when loading an editor layout, only moved. It also can't be opened using a shortcut. This is meant for docks that are opened and closed in specific cases, such as when selecting a TileMap or AnimationTree node.

### closable {#closable}

```gdscript
closable: bool
```

- void set_closable(value: bool)
- bool is_closable()

If true, the dock can be closed with the Close button in the context popup. Docks with global enabled are always closable.

### icon_name {#icon_name}

```gdscript
icon_name: StringName
```

- void set_icon_name(value: StringName)
- StringName get_icon_name()

The icon for the dock, as a name from the EditorIcons theme type in the editor theme. You can find the list of available icons here [https://godot-editor-icons.github.io/].

### dock_icon {#dock_icon}

```gdscript
dock_icon: Texture2D
```

- void set_dock_icon(value: Texture2D)
- Texture2D get_dock_icon()

The icon for the dock, as a texture. If specified, it will override icon_name.

### force_show_icon {#force_show_icon}

```gdscript
force_show_icon: bool
```

- void set_force_show_icon(value: bool)
- bool get_force_show_icon()

If true, the dock will always display an icon, regardless of EditorSettings.interface/editor/docks/dock_tab_style or EditorSettings.interface/editor/docks/bottom_dock_tab_style.

### title_color {#title_color}

```gdscript
title_color: Color
```

- void set_title_color(value: Color)
- Color get_title_color()

The color of the dock tab's title. If its alpha is 0.0, the default font color will be used.

### dock_shortcut {#dock_shortcut}

```gdscript
dock_shortcut: Shortcut
```

- void set_dock_shortcut(value: Shortcut)
- Shortcut get_dock_shortcut()

The shortcut used to open the dock.

### default_slot {#default_slot}

```gdscript
default_slot: int
```

- void set_default_slot(value: DockSlot)
- DockSlot get_default_slot()

The default dock slot used when adding the dock with EditorPlugin.add_dock().

After the dock is added, it can be moved to a different slot and the editor will automatically remember its position between sessions. If you remove and re-add the dock, it will be reset to default.

### available_layouts {#available_layouts}

```gdscript
available_layouts: int
```

- void set_available_layouts(value: BitField[DockLayout])
- BitField[DockLayout] get_available_layouts()

The available layouts for this dock, as a bitmask. By default, the dock allows vertical and floating layouts.

## 信号

### opened {#opened}

```gdscript
signal opened()
```

Emitted when the dock is opened via the Editor > Editor Docks menu, before it's made visible.

### closed {#closed}

```gdscript
signal closed()
```

Emitted when the dock is closed with the Close button in the context popup, before it's removed from its parent. See closable.

## 枚举

### DockLayout {#enum-DockLayout}

```gdscript
enum DockLayout（位标志） { DOCK_LAYOUT_VERTICAL = 1, DOCK_LAYOUT_HORIZONTAL = 2, DOCK_LAYOUT_FLOATING = 4, DOCK_LAYOUT_ALL = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOCK_LAYOUT_VERTICAL {#DOCK_LAYOUT_VERTICAL}

```gdscript
DOCK_LAYOUT_VERTICAL = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOCK_LAYOUT_HORIZONTAL {#DOCK_LAYOUT_HORIZONTAL}

```gdscript
DOCK_LAYOUT_HORIZONTAL = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOCK_LAYOUT_FLOATING {#DOCK_LAYOUT_FLOATING}

```gdscript
DOCK_LAYOUT_FLOATING = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOCK_LAYOUT_ALL {#DOCK_LAYOUT_ALL}

```gdscript
DOCK_LAYOUT_ALL = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DockSlot {#enum-DockSlot}

```gdscript
enum DockSlot { DOCK_SLOT_NONE = -1, DOCK_SLOT_LEFT_UL = 0, DOCK_SLOT_LEFT_BL = 1, DOCK_SLOT_LEFT_UR = 2, DOCK_SLOT_LEFT_BR = 3, DOCK_SLOT_RIGHT_UL = 4, DOCK_SLOT_RIGHT_BL = 5, DOCK_SLOT_RIGHT_UR = 6, DOCK_SLOT_RIGHT_BR = 7, DOCK_SLOT_BOTTOM = 8, DOCK_SLOT_BOTTOM_L = 9, DOCK_SLOT_BOTTOM_R = 10, DOCK_SLOT_MAX = 11 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOCK_SLOT_NONE {#DOCK_SLOT_NONE}

```gdscript
DOCK_SLOT_NONE = -1
```

The dock is closed.

#### DOCK_SLOT_LEFT_UL {#DOCK_SLOT_LEFT_UL}

```gdscript
DOCK_SLOT_LEFT_UL = 0
```

Dock slot, left side, upper-left (empty in default layout).

#### DOCK_SLOT_LEFT_BL {#DOCK_SLOT_LEFT_BL}

```gdscript
DOCK_SLOT_LEFT_BL = 1
```

Dock slot, left side, bottom-left (empty in default layout).

#### DOCK_SLOT_LEFT_UR {#DOCK_SLOT_LEFT_UR}

```gdscript
DOCK_SLOT_LEFT_UR = 2
```

Dock slot, left side, upper-right (in default layout includes Scene and Import docks).

#### DOCK_SLOT_LEFT_BR {#DOCK_SLOT_LEFT_BR}

```gdscript
DOCK_SLOT_LEFT_BR = 3
```

Dock slot, left side, bottom-right (in default layout includes FileSystem and History docks).

#### DOCK_SLOT_RIGHT_UL {#DOCK_SLOT_RIGHT_UL}

```gdscript
DOCK_SLOT_RIGHT_UL = 4
```

Dock slot, right side, upper-left (in default layout includes Inspector, Signal, and Group docks).

#### DOCK_SLOT_RIGHT_BL {#DOCK_SLOT_RIGHT_BL}

```gdscript
DOCK_SLOT_RIGHT_BL = 5
```

Dock slot, right side, bottom-left (empty in default layout).

#### DOCK_SLOT_RIGHT_UR {#DOCK_SLOT_RIGHT_UR}

```gdscript
DOCK_SLOT_RIGHT_UR = 6
```

Dock slot, right side, upper-right (empty in default layout).

#### DOCK_SLOT_RIGHT_BR {#DOCK_SLOT_RIGHT_BR}

```gdscript
DOCK_SLOT_RIGHT_BR = 7
```

Dock slot, right side, bottom-right (empty in default layout).

#### DOCK_SLOT_BOTTOM {#DOCK_SLOT_BOTTOM}

```gdscript
DOCK_SLOT_BOTTOM = 8
```

Bottom panel.

#### DOCK_SLOT_BOTTOM_L {#DOCK_SLOT_BOTTOM_L}

```gdscript
DOCK_SLOT_BOTTOM_L = 9
```

Dock slot at the bottom, below bottom panel, on the left side.

#### DOCK_SLOT_BOTTOM_R {#DOCK_SLOT_BOTTOM_R}

```gdscript
DOCK_SLOT_BOTTOM_R = 10
```

Dock slot at the bottom, below bottom panel, on the right side.

#### DOCK_SLOT_MAX {#DOCK_SLOT_MAX}

```gdscript
DOCK_SLOT_MAX = 11
```

Represents the size of the DockSlot enum.

---
