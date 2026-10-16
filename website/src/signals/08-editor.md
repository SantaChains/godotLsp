<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 编辑器扩展

> 仅在编辑器内运行的 Editor* 面板、脚本编辑器、文件系统、调试会话信号，游戏运行期不触发。

## EditorDebuggerSession（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal breaked(can_debug: bool)` | 附加的远端实例进入断点状态时触发，携带是否可调试。 | 断点进入钩子。 [类页锚点](../builtin/class_editordebuggersession.md#breaked) |
| `signal continued()` | 远端实例退出断点、继续运行时触发。 | 恢复执行钩子。 [类页锚点](../builtin/class_editordebuggersession.md#continued) |
| `signal started()` | 远端实例附加到本调试会话时触发。 | 会话建立。 [类页锚点](../builtin/class_editordebuggersession.md#started) |
| `signal stopped()` | 远端实例从本会话断开时触发。 | 会话结束清理。 [类页锚点](../builtin/class_editordebuggersession.md#stopped) |

## EditorDock（继承 MarginContainer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal closed()` | 用上下文菜单关闭按钮关闭停靠面板时、移出父节点前触发。 | 关闭前收尾。 [类页锚点](../builtin/class_editordock.md#closed) |
| `signal opened()` | 经 Editor > Editor Docks 菜单打开面板时、显示前触发。 | 打开初始化。 [类页锚点](../builtin/class_editordock.md#opened) |

## EditorFileSystem（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal filesystem_changed()` | 文件系统发生变化时触发。 | 扫描后重建资源表。 [类页锚点](../builtin/class_editorfilesystem.md#filesystem_changed) |
| `signal resources_reimported(resources: PackedStringArray)` | 资源完成重新导入时触发，携带资源路径数组。 | 导入完成刷新。 [类页锚点](../builtin/class_editorfilesystem.md#resources_reimported) |
| `signal resources_reimporting(resources: PackedStringArray)` | 资源即将重新导入前触发，携带路径数组。 | 导入前清理。 [类页锚点](../builtin/class_editorfilesystem.md#resources_reimporting) |
| `signal resources_reload(resources: PackedStringArray)` | 扫描时至少一个资源被重载时触发，携带路径数组。 | 热重载刷新。 [类页锚点](../builtin/class_editorfilesystem.md#resources_reload) |
| `signal script_classes_updated()` | 全局脚本类列表更新时触发。 | class_name 表重建。 [类页锚点](../builtin/class_editorfilesystem.md#script_classes_updated) |
| `signal sources_changed(exist: bool)` | 任一导入文件的源改变时触发，携带是否存在。 | 源变更重导。 [类页锚点](../builtin/class_editorfilesystem.md#sources_changed) |

## EditorInspector（继承 ScrollContainer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal edited_object_changed()` | 检视器正在编辑的对象切换时触发。 | 换对象重置面板。 [类页锚点](../builtin/class_editorinspector.md#edited_object_changed) |
| `signal object_id_selected(id: int)` | 点对象的 Edit 按钮时触发，携带对象 id。 | 打开子检视。 [类页锚点](../builtin/class_editorinspector.md#object_id_selected) |
| `signal property_deleted(property: String)` | 属性从检视器移除时触发，携带属性名。 | 动态属性清理。 [类页锚点](../builtin/class_editorinspector.md#property_deleted) |
| `signal property_edited(property: String)` | 属性在检视器被编辑时触发，携带属性名。 | 值变更回调。 [类页锚点](../builtin/class_editorinspector.md#property_edited) |
| `signal property_keyed(property: String, value: Variant, advance: bool)` | 属性被打成动画关键帧时触发，携带属性/值/是否前进。 | 关键帧钩子。 [类页锚点](../builtin/class_editorinspector.md#property_keyed) |
| `signal property_selected(property: String)` | 选中某属性时触发，携带属性名。 | 属性聚焦。 [类页锚点](../builtin/class_editorinspector.md#property_selected) |
| `signal property_toggled(property: String, checked: bool)` | 布尔属性被切换时触发，携带属性名与状态。 | 开关同步。 [类页锚点](../builtin/class_editorinspector.md#property_toggled) |
| `signal resource_selected(resource: Resource, path: String)` | 检视器选中某资源时触发，携带资源与路径。 | 资源选择。 [类页锚点](../builtin/class_editorinspector.md#resource_selected) |
| `signal restart_requested()` | 编辑了需重启才生效的属性时触发。 | 提示重启。 [类页锚点](../builtin/class_editorinspector.md#restart_requested) |

## EditorPlugin（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal main_screen_changed(screen_name: String)` | 用户切换工作区（2D/3D/脚本/游戏/资产商店）时触发，携带屏幕名。 | 切换视图钩子。 [类页锚点](../builtin/class_editorplugin.md#main_screen_changed) |
| `signal project_settings_changed()` | 任一项目设置改变时触发。 | 设置生效刷新。 [类页锚点](../builtin/class_editorplugin.md#project_settings_changed) |
| `signal resource_saved(resource: Resource)` | 给定资源落盘保存时触发，携带资源。 | 保存后处理。 [类页锚点](../builtin/class_editorplugin.md#resource_saved) |
| `signal scene_changed(scene_root: Node)` | 编辑器切换当前场景时触发，携带场景根节点。 | 换场景初始化。 [类页锚点](../builtin/class_editorplugin.md#scene_changed) |
| `signal scene_closed(filepath: String)` | 用户关闭场景时触发，携带文件路径。 | 关闭清理。 [类页锚点](../builtin/class_editorplugin.md#scene_closed) |
| `signal scene_saved(filepath: String)` | 场景落盘保存时触发，携带文件路径。 | 保存后钩子。 [类页锚点](../builtin/class_editorplugin.md#scene_saved) |

## EditorProperty（继承 Container）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal multiple_properties_changed(properties: PackedStringArray, value: Array)` | 希望同时修改多个属性时自行发射，携带属性名数组与值数组。 | 批量改值。 [类页锚点](../builtin/class_editorproperty.md#multiple_properties_changed) |
| `signal object_id_selected(property: StringName, id: int)` | 子检视器使用，选中对象时触发。 | 子检视跳转。 [类页锚点](../builtin/class_editorproperty.md#object_id_selected) |
| `signal property_can_revert_changed(property: StringName, can_revert: bool)` | 属性是否可回退（有无非默认值、是否显示回退图标）改变时触发。 | 回退图标刷新。 [类页锚点](../builtin/class_editorproperty.md#property_can_revert_changed) |
| `signal property_changed(property: StringName, value: Variant, field: StringName, changing: bool)` | 属性变更信号；不要手动发射，用 emit_changed()。 | 值变更，走 emit_changed。 [类页锚点](../builtin/class_editorproperty.md#property_changed) |
| `signal property_checked(property: StringName, checked: bool)` | 属性被勾选时触发，携带属性名与状态。 | 勾选同步。 [类页锚点](../builtin/class_editorproperty.md#property_checked) |
| `signal property_deleted(property: StringName)` | 属性被删除时触发，携带属性名。 | 删除清理。 [类页锚点](../builtin/class_editorproperty.md#property_deleted) |
| `signal property_favorited(property: StringName, favorited: bool)` | 希望把属性标为收藏（置顶显示）时自行发射。 | 收藏置顶。 [类页锚点](../builtin/class_editorproperty.md#property_favorited) |
| `signal property_keyed(property: StringName)` | 希望把当前值打成关键帧时自行发射（先确认可关键帧）。 | 关键帧。 [类页锚点](../builtin/class_editorproperty.md#property_keyed) |
| `signal property_keyed_with_value(property: StringName, value: Variant)` | 以单值给属性打关键帧时自行发射，携带属性与值。 | 带值关键帧。 [类页锚点](../builtin/class_editorproperty.md#property_keyed_with_value) |
| `signal property_overridden()` | 请求对当前项目做设置覆盖时触发。 | 覆盖态同步。 [类页锚点](../builtin/class_editorproperty.md#property_overridden) |
| `signal property_pinned(property: StringName, pinned: bool)` | 标记/取消标记属性值强制保存（即使等于默认）时自行发射。 | 钉住保存。 [类页锚点](../builtin/class_editorproperty.md#property_pinned) |
| `signal resource_selected(path: String, resource: Resource)` | 要编辑子资源时携带资源发射。 | 打开子资源。 [类页锚点](../builtin/class_editorproperty.md#resource_selected) |
| `signal selected(path: String, focusable_idx: int)` | 属性项被选中时触发，携带路径与可聚焦索引。 | 选中聚焦。 [类页锚点](../builtin/class_editorproperty.md#selected) |

## EditorResourcePicker（继承 HBoxContainer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal resource_changed(resource: Resource)` | 被编辑资源的值改变时触发，携带资源。 | 资源换值。 [类页锚点](../builtin/class_editorresourcepicker.md#resource_changed) |
| `signal resource_selected(resource: Resource, inspect: bool)` | 资源值设定且用户点击编辑时触发，携带资源与是否检视。 | 编辑资源。 [类页锚点](../builtin/class_editorresourcepicker.md#resource_selected) |

## EditorResourcePreview（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal preview_invalidated(path: String)` | 预览失效（内容改变）时触发，携带路径。 | 重建缩略图。 [类页锚点](../builtin/class_editorresourcepreview.md#preview_invalidated) |

## EditorSelection（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal selection_changed()` | 当前选择集变化时触发。 | 选择同步。 [类页锚点](../builtin/class_editorselection.md#selection_changed) |

## EditorSettings（继承 Resource）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal settings_changed()` | 任一编辑器设置改变后触发。 | 编辑器偏好刷新。 [类页锚点](../builtin/class_editorsettings.md#settings_changed) |

## EditorSpinSlider（继承 Range）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal grabbed()` | 抓住数值旋钮/滑条时触发。 | 拖动开始。 [类页锚点](../builtin/class_editorspinslider.md#grabbed) |
| `signal ungrabbed()` | 松开旋钮/滑条时触发。 | 拖动结束提交。 [类页锚点](../builtin/class_editorspinslider.md#ungrabbed) |
| `signal updown_pressed()` | 点按上下步进按钮时触发。 | 步进回调。 [类页锚点](../builtin/class_editorspinslider.md#updown_pressed) |
| `signal value_focus_entered()` | 数值输入框获得焦点时触发。 | 聚焦。 [类页锚点](../builtin/class_editorspinslider.md#value_focus_entered) |
| `signal value_focus_exited()` | 数值输入框失去焦点时触发。 | 失焦提交。 [类页锚点](../builtin/class_editorspinslider.md#value_focus_exited) |

## EditorUndoRedoManager（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal history_changed()` | 任一历史列表变化（提交动作或清空）时触发。 | 撤销栈刷新。 [类页锚点](../builtin/class_editorundoredomanager.md#history_changed) |
| `signal version_changed()` | undo/redo 导致任一历史版本变化时触发。 | 版本同步。 [类页锚点](../builtin/class_editorundoredomanager.md#version_changed) |

## FileSystemDock（继承 EditorDock）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal display_mode_changed()` | 切换文件显示模式或分栏模式时触发。 | 视图切换。 [类页锚点](../builtin/class_filesystemdock.md#display_mode_changed) |
| `signal file_removed(file: String)` | 给定文件被移除时触发，携带路径。 | 文件删除同步。 [类页锚点](../builtin/class_filesystemdock.md#file_removed) |
| `signal files_moved(old_file: String, new_file: String)` | 文件从旧路径移到新路径时触发。 | 移动同步。 [类页锚点](../builtin/class_filesystemdock.md#files_moved) |
| `signal folder_color_changed()` | 文件夹颜色改变时触发。 | 颜色刷新。 [类页锚点](../builtin/class_filesystemdock.md#folder_color_changed) |
| `signal folder_moved(old_folder: String, new_folder: String)` | 文件夹从旧路径移到新路径时触发。 | 移动同步。 [类页锚点](../builtin/class_filesystemdock.md#folder_moved) |
| `signal folder_removed(folder: String)` | 给定文件夹被移除时触发，携带路径。 | 删除同步。 [类页锚点](../builtin/class_filesystemdock.md#folder_removed) |
| `signal inherit(file: String)` | 新建一个继承指定路径场景的场景时触发。 | 继承新建钩子。 [类页锚点](../builtin/class_filesystemdock.md#inherit) |
| `signal instantiate(files: PackedStringArray)` | 在编辑器内实例化给定场景时触发，携带路径数组。 | 实例化钩子。 [类页锚点](../builtin/class_filesystemdock.md#instantiate) |
| `signal resource_removed(resource: Resource)` | 外部资源文件被删除时触发，携带资源。 | 资源清理。 [类页锚点](../builtin/class_filesystemdock.md#resource_removed) |
| `signal selection_changed()` | 文件面板选择变化时触发。 | 选择同步。 [类页锚点](../builtin/class_filesystemdock.md#selection_changed) |

## OpenXRBindingModifierEditor（继承 PanelContainer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal binding_modifier_removed(binding_modifier_editor: Object)` | （描述待补，参见类页） | 连接：`obj.binding_modifier_removed.connect(_on_binding_modifier_removed)`；回调 `func _on_binding_modifier_removed(binding_modifier_editor: Object):` [类页锚点](../builtin/class_openxrbindingmodifiereditor.md#binding_modifier_removed) |

## ScriptCreateDialog（继承 ConfirmationDialog）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal script_created(script: Script)` | 用户点 OK 创建脚本时触发，携带脚本。 | 新脚本后处理。 [类页锚点](../builtin/class_scriptcreatedialog.md#script_created) |

## ScriptEditor（继承 PanelContainer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal editor_script_changed(script: Script)` | 用户切换活动脚本时触发，携带脚本。 | 切换脚本。 [类页锚点](../builtin/class_scripteditor.md#editor_script_changed) |
| `signal script_close(script: Script)` | 编辑器即将关闭活动脚本时触发，携带脚本。 | 关闭保存检查。 [类页锚点](../builtin/class_scripteditor.md#script_close) |

## ScriptEditorBase（继承 VBoxContainer）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal edited_script_changed()` | 脚本校验完成后触发。 | 校验后刷新。 [类页锚点](../builtin/class_scripteditorbase.md#edited_script_changed) |
| `signal go_to_help(what: String)` | 用户请求打开特定文档页时触发，携带主题。 | 跳帮助。 [类页锚点](../builtin/class_scripteditorbase.md#go_to_help) |
| `signal go_to_method(script: Object, method: String)` | 请求查看脚本某方法时触发，携带脚本与方法名。 | 定位方法。 [类页锚点](../builtin/class_scripteditorbase.md#go_to_method) |
| `signal name_changed()` | 校验后或被编辑资源改变时触发。 | 改名刷新。 [类页锚点](../builtin/class_scripteditorbase.md#name_changed) |
| `signal replace_in_files_requested(text: String)` | 用户请求在文件系统内查找替换时触发，携带文本。 | 全局替换。 [类页锚点](../builtin/class_scripteditorbase.md#replace_in_files_requested) |
| `signal request_help(topic: String)` | 用户请求上下文帮助时触发，携带主题。 | 上下文帮助。 [类页锚点](../builtin/class_scripteditorbase.md#request_help) |
| `signal request_open_script_at_line(script: Object, line: int)` | 请求查看脚本指定行时触发，携带脚本与行号。 | 打开定位。 [类页锚点](../builtin/class_scripteditorbase.md#request_open_script_at_line) |
| `signal request_save_history()` | 上下文跳转且条目在同一脚本时触发。 | 保存历史栈。 [类页锚点](../builtin/class_scripteditorbase.md#request_save_history) |
| `signal request_save_previous_state(state: Dictionary)` | 切换脚本或在同脚本内移动光标跨 10 列以上时触发，携带状态字典。 | 保存前态。 [类页锚点](../builtin/class_scripteditorbase.md#request_save_previous_state) |
| `signal search_in_files_requested(text: String)` | 用户请求在文件系统内搜索文本时触发，携带文本。 | 全局搜索。 [类页锚点](../builtin/class_scripteditorbase.md#search_in_files_requested) |

## VisualShaderNodeInput（继承 VisualShaderNode）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal input_type_changed()` | （描述待补，参见类页） | 连接：`obj.input_type_changed.connect(_on_input_type_changed)`；回调 `func _on_input_type_changed():` [类页锚点](../builtin/class_visualshadernodeinput.md#input_type_changed) |

<!-- SIGNALS:END -->
