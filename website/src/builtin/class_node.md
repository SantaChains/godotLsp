# Node

**继承**：[Object](class_object.md) → **Node**

> Nodes are Godot's building blocks. They can be assigned as the child of another node, resulting in a tree arrangement. A given node can contain any number of nodes as children with the requirement that all siblings (direct children of a node) should have unique names.

<details><summary>官方英文描述（中文精译进行中）</summary>

Nodes are Godot's building blocks. They can be assigned as the child of another node, resulting in a tree arrangement. A given node can contain any number of nodes as children with the requirement that all siblings (direct children of a node) should have unique names.

A tree of nodes is called a scene. Scenes can be saved to the disk and then instantiated into other scenes. This allows for very high flexibility in the architecture and data model of Godot projects.

Scene tree: The SceneTree contains the active tree of nodes. When a node is added to the scene tree, it receives the NOTIFICATION_ENTER_TREE notification and its _enter_tree() callback is triggered. Child nodes are always added after their parent node, i.e. the _enter_tree() callback of a parent node will be triggered before its child's.

Once all nodes have been added in the scene tree, they receive the NOTIFICATION_READY notification and their respective _ready() callbacks are triggered. For groups of nodes, the _ready() callback is called in reverse order, starting with the children and moving up to the parent nodes.

This means that when adding a node to the scene tree, the following order will be used for the callbacks: _enter_tree() of the parent, _enter_tree() of the children, _ready() of the children and finally _ready() of the parent (recursively for the entire scene tree).

Processing: Nodes can override the "process" state, so that they receive a callback on each frame requesting them to process (do something). Normal processing (callback _process(), toggled with set_process()) happens as fast as possible and is dependent on the frame rate, so the processing time delta (in seconds) is passed as an argument. Physics processing (callback _physics_process(), toggled with set_physics_process()) happens a fixed number of times per second (60 by default) and is useful for code related to the physics engine.

Nodes can also process input events. When present, the _input() function will be called for each input that the program receives. In many cases, this can be overkill (unless used for simple projects), and the _unhandled_input() function might be preferred; it is called when the input event was not handled by anyone else (typically, GUI Control nodes), ensuring that the node only receives the events that were meant for it.

To keep track of the scene hierarchy (especially when instantiating scenes into other scenes), an "owner" can be set for the node with the owner property. This keeps track of who instantiated what. This is mostly useful when writing editors and tools, though.

Finally, when a node is freed with Object.free() or queue_free(), it will also free all its children.

Groups: Nodes can be added to as many groups as you want to be easy to manage, you could create groups like "enemies" or "collectables" for example, depending on your game. See add_to_group(), is_in_group() and remove_from_group(). You can then retrieve all nodes in these groups, iterate them and even call methods on groups via the methods on SceneTree.

Networking with nodes: After connecting to a server (or making one, see ENetMultiplayerPeer), it is possible to use the built-in RPC (remote procedure call) system to communicate over the network. By calling rpc() with a method name, it will be called locally and in all connected peers (peers = clients and the server that accepts connections). To identify which node receives the RPC call, Godot will use its NodePath (make sure node names are the same on all peers). Also, take a look at the high-level networking tutorial and corresponding demos.

Note: The script property is part of the Object class, not Node. It isn't exposed like most properties but does have a setter and getter (see Object.set_script() and Object.get_script()).

</details>

## 方法

### _enter_tree {#_enter_tree}

```gdscript
void _enter_tree() virtual
```

Called when the node enters the SceneTree (e.g. upon instantiating, scene changing, or after calling add_child() in a script). If the node has children, its _enter_tree() callback will be called first, and then that of the children.

Corresponds to the NOTIFICATION_ENTER_TREE notification in Object._notification().

### _exit_tree {#_exit_tree}

```gdscript
void _exit_tree() virtual
```

Called when the node is about to leave the SceneTree (e.g. upon freeing, scene changing, or after calling remove_child() in a script). If the node has children, its _exit_tree() callback will be called last, after all its children have left the tree.

Corresponds to the NOTIFICATION_EXIT_TREE notification in Object._notification() and signal tree_exiting. To get notified when the node has already left the active tree, connect to the tree_exited.

### _get_accessibility_configuration_warnings {#_get_accessibility_configuration_warnings}

```gdscript
PackedStringArray _get_accessibility_configuration_warnings() virtual const
```

The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a tool script, and accessibility warnings are enabled in the editor settings.

Returning an empty array produces no warnings.

### _get_configuration_warnings {#_get_configuration_warnings}

```gdscript
PackedStringArray _get_configuration_warnings() virtual const
```

The elements in the array returned from this method are displayed as warnings in the Scene dock if the script that overrides it is a tool script.

Returning an empty array produces no warnings.

Call update_configuration_warnings() when the warnings need to be updated for this node.

```gdscript
@export var energy = 0:
    set(value):
        energy = value
        update_configuration_warnings()
func _get_configuration_warnings():
    if energy < 0:
        return ["Energy must be 0 or greater."]
    else:
        return []
```

### _get_focused_accessibility_element {#_get_focused_accessibility_element}

```gdscript
RID _get_focused_accessibility_element() virtual const
```

Called during accessibility information updates to determine the currently focused sub-element, should return a sub-element RID or the value returned by get_accessibility_element().

### _input {#_input}

```gdscript
void _input(event: InputEvent) virtual
```

Called when there is an input event. The input event propagates up through the node tree until a node consumes it.

It is only called if input processing is enabled, which is done automatically if this method is overridden, and can be toggled with set_process_input().

To consume the input event and stop it propagating further to other nodes, Viewport.set_input_as_handled() can be called.

For gameplay input, _unhandled_input() and _unhandled_key_input() are usually a better fit as they allow the GUI to intercept the events first.

Note: This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).

### _physics_process {#_physics_process}

```gdscript
void _physics_process(delta: float) virtual
```

Called once on each physics tick, and allows Nodes to synchronize their logic with physics ticks. delta is the logical time between physics ticks in seconds and is equal to Engine.time_scale / Engine.physics_ticks_per_second.

It is only called if physics processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with set_physics_process().

Processing happens in order of process_physics_priority, lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).

Corresponds to the NOTIFICATION_PHYSICS_PROCESS notification in Object._notification().

Note: This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).

Note: Accumulated delta may diverge from real world seconds.

### _process {#_process}

```gdscript
void _process(delta: float) virtual
```

Called on each idle frame, prior to rendering, and after physics ticks have been processed. delta is the time between frames in seconds.

It is only called if processing is enabled for this Node, which is done automatically if this method is overridden, and can be toggled with set_process().

Processing happens in order of process_priority, lower priority values are called first. Nodes with the same priority are processed in tree order, or top to bottom as seen in the editor (also known as pre-order traversal).

Corresponds to the NOTIFICATION_PROCESS notification in Object._notification().

Note: This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).

Note: When the engine is struggling and the frame rate is lowered, delta will increase. When delta is increased, it's capped at a maximum of Engine.time_scale * Engine.max_physics_steps_per_frame / Engine.physics_ticks_per_second. As a result, accumulated delta may not represent real world time.

Note: When --fixed-fps is enabled or the engine is running in Movie Maker mode (see MovieWriter), process delta will always be the same for every frame, regardless of how much time the frame took to render.

Note: Frame delta may be post-processed by OS.delta_smoothing if this is enabled for the project.

### _ready {#_ready}

```gdscript
void _ready() virtual
```

Called when the node is "ready", i.e. when both the node and its children have entered the scene tree. If the node has children, their _ready() callbacks get triggered first, and the parent node will receive the ready notification afterwards.

Corresponds to the NOTIFICATION_READY notification in Object._notification(). See also the @onready annotation for variables.

Usually used for initialization. For even earlier initialization, Object._init() may be used. See also _enter_tree().

Note: This method may be called only once for each node. After removing a node from the scene tree and adding it again, _ready() will not be called a second time. This can be bypassed by requesting another call with request_ready(), which may be called anywhere before adding the node again.

### _shortcut_input {#_shortcut_input}

```gdscript
void _shortcut_input(event: InputEvent) virtual
```

Called when an InputEventKey, InputEventShortcut, or InputEventJoypadButton hasn't been consumed by _input() or any GUI Control item. It is called before _unhandled_key_input() and _unhandled_input(). The input event propagates up through the node tree until a node consumes it.

It is only called if shortcut processing is enabled, which is done automatically if this method is overridden, and can be toggled with set_process_shortcut_input().

To consume the input event and stop it propagating further to other nodes, Viewport.set_input_as_handled() can be called.

This method can be used to handle shortcuts. For generic GUI events, use _input() instead. Gameplay events should usually be handled with either _unhandled_input() or _unhandled_key_input().

Note: This method is only called if the node is present in the scene tree (i.e. if it's not orphan).

### _unhandled_input {#_unhandled_input}

```gdscript
void _unhandled_input(event: InputEvent) virtual
```

Called when an InputEvent hasn't been consumed by _input() or any GUI Control item. It is called after _shortcut_input() and after _unhandled_key_input(). The input event propagates up through the node tree until a node consumes it.

It is only called if unhandled input processing is enabled, which is done automatically if this method is overridden, and can be toggled with set_process_unhandled_input().

To consume the input event and stop it propagating further to other nodes, Viewport.set_input_as_handled() can be called.

For gameplay input, this method is usually a better fit than _input(), as GUI events need a higher priority. For keyboard shortcuts, consider using _shortcut_input() instead, as it is called before this method. Finally, to handle keyboard events, consider using _unhandled_key_input() for performance reasons.

Note: This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).

### _unhandled_key_input {#_unhandled_key_input}

```gdscript
void _unhandled_key_input(event: InputEvent) virtual
```

Called when an InputEventKey hasn't been consumed by _input() or any GUI Control item. It is called after _shortcut_input() but before _unhandled_input(). The input event propagates up through the node tree until a node consumes it.

It is only called if unhandled key input processing is enabled, which is done automatically if this method is overridden, and can be toggled with set_process_unhandled_key_input().

To consume the input event and stop it propagating further to other nodes, Viewport.set_input_as_handled() can be called.

This method can be used to handle Unicode character input with Alt, Alt + Ctrl, and Alt + Shift modifiers, after shortcuts were handled.

For gameplay input, this and _unhandled_input() are usually a better fit than _input(), as GUI events should be handled first. This method also performs better than _unhandled_input(), since unrelated events such as InputEventMouseMotion are automatically filtered. For shortcuts, consider using _shortcut_input() instead.

Note: This method is only called if the node is present in the scene tree (i.e. if it's not an orphan).

### add_child {#add_child}

```gdscript
void add_child(node: Node, force_readable_name: bool = false, internal: Node.InternalMode = 0)
```

Adds a child node. Nodes can have any number of children, but every child must have a unique name. Child nodes are automatically deleted when the parent node is deleted, so an entire scene can be removed by deleting its topmost node.

If force_readable_name is true, improves the readability of the added node. If not named, the node is renamed to its type, and if it shares name with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to false, which assigns a dummy name featuring @ in both situations.

If internal is different than INTERNAL_MODE_DISABLED, the child will be added as internal node. These nodes are ignored by methods like get_children(), unless their parameter include_internal is true. It also prevents these nodes being duplicated with their parent. The intended usage is to hide the internal nodes from the user, so the user won't accidentally delete or modify them. Used by some GUI nodes, e.g. ColorPicker.

Note: If node already has a parent, this method will fail. Use remove_child() first to remove node from its current parent. For example:

```
var child_node = get_child(0)
if child_node.get_parent():
    child_node.get_parent().remove_child(child_node)
add_child(child_node)
```

```
Node childNode = GetChild(0);
if (childNode.GetParent() != null)
{
    childNode.GetParent().RemoveChild(childNode);
}
AddChild(childNode);
```

If you need the child node to be added below a specific node in the list of children, use add_sibling() instead of this method.

Note: If you want a child to be persisted to a PackedScene, you must set owner in addition to calling add_child(). This is typically relevant for tool scripts and editor plugins. If add_child() is called without setting owner, the newly added Node will not be visible in the scene tree, though it will be visible in the 2D/3D view.

### add_sibling {#add_sibling}

```gdscript
void add_sibling(sibling: Node, force_readable_name: bool = false)
```

Adds a sibling node to this node's parent, and moves the added sibling right below this node.

If force_readable_name is true, improves the readability of the added sibling. If not named, the sibling is renamed to its type, and if it shares name with a sibling, a number is suffixed more appropriately. This operation is very slow. As such, it is recommended leaving this to false, which assigns a dummy name featuring @ in both situations.

Use add_child() instead of this method if you don't need the child node to be added below a specific node in the list of children.

Note: If this node is internal, the added sibling will be internal too (see add_child()'s internal parameter).

### add_to_group {#add_to_group}

```gdscript
void add_to_group(group: StringName, persistent: bool = false)
```

Adds the node to the group. Groups can be helpful to organize a subset of nodes, for example "enemies" or "collectables". See notes in the description, and the group methods in SceneTree.

If persistent is true, the group will be stored when saved inside a PackedScene. All groups created and displayed in the Groups dock are persistent.

Note: To improve performance, the order of group names is not guaranteed and may vary between project runs. Therefore, do not rely on the group order.

Note: SceneTree's group methods will not work on this node if not inside the tree (see is_inside_tree()).

### atr {#atr}

```gdscript
String atr(message: String, context: StringName = "") const
```

Translates a message, using the translation catalogs configured in the Project Settings. Further context can be specified to help with the translation. Note that most Control nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.

This method works the same as Object.tr(), with the addition of respecting the auto_translate_mode state.

If Object.can_translate_messages() is false, or no translation is available, this method returns the message without changes. See Object.set_message_translation().

For detailed examples, see Internationalizing games.

### atr_n {#atr_n}

```gdscript
String atr_n(message: String, plural_message: StringName, n: int, context: StringName = "") const
```

Translates a message or plural_message, using the translation catalogs configured in the Project Settings. Further context can be specified to help with the translation.

This method works the same as Object.tr_n(), with the addition of respecting the auto_translate_mode state.

If Object.can_translate_messages() is false, or no translation is available, this method returns message or plural_message, without changes. See Object.set_message_translation().

The n is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.

For detailed examples, see Localization using gettext.

Note: Negative and float numbers may not properly apply to some countable subjects. It's recommended to handle these cases with atr().

### call_deferred_thread_group {#call_deferred_thread_group}

```gdscript
Variant call_deferred_thread_group(method: StringName, ...)
```

This function is similar to Object.call_deferred() except that the call will take place when the node thread group is processed. If the node thread group processes in sub-threads, then the call will be done on that thread, right before NOTIFICATION_PROCESS or NOTIFICATION_PHYSICS_PROCESS, the _process() or _physics_process() or their internal versions are called.

### call_thread_safe {#call_thread_safe}

```gdscript
Variant call_thread_safe(method: StringName, ...)
```

This function ensures that the calling of this function will succeed, no matter whether it's being done from a thread or not. If called from a thread that is not allowed to call the function, the call will become deferred. Otherwise, the call will go through directly.

### can_auto_translate {#can_auto_translate}

```gdscript
bool can_auto_translate() const
```

Returns true if this node can automatically translate messages depending on the current locale. See auto_translate_mode, atr(), and atr_n().

### can_process {#can_process}

```gdscript
bool can_process() const
```

Returns true if the node can receive processing notifications and input callbacks (NOTIFICATION_PROCESS, _input(), etc.) from the SceneTree and Viewport. The returned value depends on process_mode:

- If set to PROCESS_MODE_PAUSABLE, returns true when the game is processing, i.e. SceneTree.paused is false;
- If set to PROCESS_MODE_WHEN_PAUSED, returns true when the game is paused, i.e. SceneTree.paused is true;
- If set to PROCESS_MODE_ALWAYS, always returns true;
- If set to PROCESS_MODE_DISABLED, always returns false;
- If set to PROCESS_MODE_INHERIT, use the parent node's process_mode to determine the result.

If the node is not inside the tree, returns false no matter the value of process_mode.

### create_tween {#create_tween}

```gdscript
Tween create_tween()
```

Creates a new Tween and binds it to this node.

This is the equivalent of doing:

```
get_tree().create_tween().bind_node(self)
```

```
GetTree().CreateTween().BindNode(this);
```

The Tween will start automatically on the next process frame or physics frame (depending on TweenProcessMode). See Tween.bind_node() for more info on Tweens bound to nodes.

Note: The method can still be used when the node is not inside SceneTree. It can fail in an unlikely case of using a custom MainLoop.

### duplicate {#duplicate}

```gdscript
Node duplicate(flags: int = 15) const
```

Duplicates the node, returning a new node with all of its properties, signals, groups, and children copied from the original, recursively. The behavior can be tweaked through the flags (see DuplicateFlags). Internal nodes are not duplicated.

Note: For nodes with a Script attached, if Object._init() has been defined with required parameters, the duplicated node will not have a Script.

Note: By default, this method will duplicate only properties marked for serialization (i.e. using @GlobalScope.PROPERTY_USAGE_STORAGE, or in GDScript, @GDScript.@export). If you want to duplicate all properties, use DUPLICATE_INTERNAL_STATE.

### find_child {#find_child}

```gdscript
Node find_child(pattern: String, recursive: bool = true, owned: bool = true) const
```

Finds the first descendant of this node whose name matches pattern, returning null if no match is found. The matching is done against node names, not their paths, through String.match(). As such, it is case-sensitive, "*" matches zero or more characters, and "?" matches any single character.

If recursive is false, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see internal parameter in add_child()).

If owned is true, only descendants with a valid owner node are checked.

Note: This method can be very slow. Consider storing a reference to the found node in a variable. Alternatively, use get_node() with unique names (see unique_name_in_owner).

Note: To find all descendant nodes matching a pattern or a class type, see find_children().

### find_children {#find_children}

```gdscript
typedarray::Node find_children(pattern: String, type: String = "", recursive: bool = true, owned: bool = true) const
```

Finds all descendants of this node whose names match pattern, returning an empty Array if no match is found. The matching is done against node names, not their paths, through String.match(). As such, it is case-sensitive, "*" matches zero or more characters, and "?" matches any single character.

If type is not empty, only descendants inheriting from type are included (see Object.is_class()).

If recursive is false, only this node's direct children are checked. Nodes are checked in tree order, so this node's first direct child is checked first, then its own direct children, etc., before moving to the second direct child, and so on. Internal children are also included in the search (see internal parameter in add_child()).

If owned is true, only descendants with a valid owner node are checked.

Note: This method can be very slow. Consider storing references to the found nodes in a variable.

Note: To find a single descendant node matching a pattern, see find_child().

### find_parent {#find_parent}

```gdscript
Node find_parent(pattern: String) const
```

Finds the first ancestor of this node whose name matches pattern, returning null if no match is found. The matching is done through String.match(). As such, it is case-sensitive, "*" matches zero or more characters, and "?" matches any single character. See also find_child() and find_children().

Note: As this method walks upwards in the scene tree, it can be slow in large, deeply nested nodes. Consider storing a reference to the found node in a variable. Alternatively, use get_node() with unique names (see unique_name_in_owner).

### get_accessibility_element {#get_accessibility_element}

```gdscript
RID get_accessibility_element() const
```

Returns main accessibility element RID.

Note: This method should be called only during accessibility information updates (NOTIFICATION_ACCESSIBILITY_UPDATE).

### get_auto_translate_mode {#get_auto_translate_mode}

```gdscript
Node.AutoTranslateMode get_auto_translate_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_child {#get_child}

```gdscript
Node get_child(idx: int, include_internal: bool = false) const
```

Fetches a child node by its index. Each child node has an index relative to its siblings (see get_index()). The first child is at index 0. Negative values can also be used to start from the end of the list. This method can be used in combination with get_child_count() to iterate over this node's children. If no child exists at the given index, this method returns null and an error is generated.

If include_internal is false, internal children are ignored (see add_child()'s internal parameter).

```gdscript
# Assuming the following are children of this node, in order:
# First, Middle, Last.
var a = get_child(0).name  # a is "First"
var b = get_child(1).name  # b is "Middle"
var b = get_child(2).name  # b is "Last"
var c = get_child(-1).name # c is "Last"
```

Note: To fetch a node by NodePath, use get_node().

### get_child_count {#get_child_count}

```gdscript
int get_child_count(include_internal: bool = false) const
```

Returns the number of children of this node.

If include_internal is false, internal children are not counted (see add_child()'s internal parameter).

### get_children {#get_children}

```gdscript
typedarray::Node get_children(include_internal: bool = false) const
```

Returns all children of this node inside an Array.

If include_internal is false, excludes internal children from the returned array (see add_child()'s internal parameter).

### get_editor_description {#get_editor_description}

```gdscript
String get_editor_description() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_groups {#get_groups}

```gdscript
typedarray::StringName get_groups() const
```

Returns an Array of group names that the node has been added to.

Note: To improve performance, the order of group names is not guaranteed and may vary between project runs. Therefore, do not rely on the group order.

Note: This method may also return some group names starting with an underscore (_). These are internally used by the engine. To avoid conflicts, do not use custom groups starting with underscores. To exclude internal groups, see the following code snippet:

```
# Stores the node's non-internal groups only (as an array of StringNames).
var non_internal_groups = []
for group in get_groups():
    if not str(group).begins_with("_"):
        non_internal_groups.push_back(group)
```

```
// Stores the node's non-internal groups only (as a List of StringNames).
List nonInternalGroups = new List();
foreach (string group in GetGroups())
{
    if (!group.BeginsWith("_"))
        nonInternalGroups.Add(group);
}
```

### get_index {#get_index}

```gdscript
int get_index(include_internal: bool = false) const
```

Returns this node's order among its siblings. The first node's index is 0. See also get_child().

If include_internal is false, returns the index ignoring internal children. The first, non-internal child will have an index of 0 (see add_child()'s internal parameter).

### get_last_exclusive_window {#get_last_exclusive_window}

```gdscript
Window get_last_exclusive_window() const
```

Returns the Window that contains this node, or the last exclusive child in a chain of windows starting with the one that contains this node.

### get_multiplayer {#get_multiplayer}

```gdscript
MultiplayerAPI get_multiplayer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_multiplayer_authority {#get_multiplayer_authority}

```gdscript
int get_multiplayer_authority() const
```

Returns the peer ID of the multiplayer authority for this node. See set_multiplayer_authority().

### get_name {#get_name}

```gdscript
StringName get_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node {#get_node}

```gdscript
Node get_node(path: NodePath) const
```

Fetches a node. The NodePath can either be a relative path (from this node), or an absolute path (from the SceneTree.root) to a node. If path does not point to a valid node, generates an error and returns null. Attempts to access methods on the return value will result in an "Attempt to call  on a null instance." error.

Note: Fetching by absolute path only works when the node is inside the scene tree (see is_inside_tree()).

Example: Assume this method is called from the Character node, inside the following tree:

```
┖╴root
   ┠╴Character (you are here!)
   ┃  ┠╴Sword
   ┃  ┖╴Backpack
   ┃     ┖╴Dagger
   ┠╴MyGame
   ┖╴Swamp
      ┠╴Alligator
      ┠╴Mosquito
      ┖╴Goblin
```

The following calls will return a valid node:

```
get_node("Sword")
get_node("Backpack/Dagger")
get_node("../Swamp/Alligator")
get_node("/root/MyGame")
```

```
GetNode("Sword");
GetNode("Backpack/Dagger");
GetNode("../Swamp/Alligator");
GetNode("/root/MyGame");
```

### get_node_and_resource {#get_node_and_resource}

```gdscript
Array get_node_and_resource(path: NodePath)
```

Fetches a node and its most nested resource as specified by the NodePath's subname. Returns an Array of size 3 where:

- Element 0 is the Node, or null if not found;
- Element 1 is the subname's last nested Resource, or null if not found;
- Element 2 is the remaining NodePath, referring to an existing, non-Resource property (see Object.get_indexed()).

Example: Assume that the child's Sprite2D.texture has been assigned an AtlasTexture:

```
var a = get_node_and_resource("Area2D/Sprite2D")
print(a[0].name) # Prints Sprite2D
print(a[1])      # Prints 
print(a[2])      # Prints ^""
var b = get_node_and_resource("Area2D/Sprite2D:texture:atlas")
print(b[0].name)        # Prints Sprite2D
print(b[1].get_class()) # Prints AtlasTexture
print(b[2])             # Prints ^""
var c = get_node_and_resource("Area2D/Sprite2D:texture:atlas:region")
print(c[0].name)        # Prints Sprite2D
print(c[1].get_class()) # Prints AtlasTexture
print(c[2])             # Prints ^":region"
```

```
var a = GetNodeAndResource(NodePath("Area2D/Sprite2D"));
GD.Print(a[0].Name); // Prints Sprite2D
GD.Print(a[1]);      // Prints 
GD.Print(a[2]);      // Prints ^"
var b = GetNodeAndResource(NodePath("Area2D/Sprite2D:texture:atlas"));
GD.Print(b[0].name);        // Prints Sprite2D
GD.Print(b[1].get_class()); // Prints AtlasTexture
GD.Print(b[2]);             // Prints ^""
var c = GetNodeAndResource(NodePath("Area2D/Sprite2D:texture:atlas:region"));
GD.Print(c[0].name);        // Prints Sprite2D
GD.Print(c[1].get_class()); // Prints AtlasTexture
GD.Print(c[2]);             // Prints ^":region"
```

### get_node_or_null {#get_node_or_null}

```gdscript
Node get_node_or_null(path: NodePath) const
```

Fetches a node by NodePath. Similar to get_node(), but does not generate an error if path does not point to a valid node.

### get_node_rpc_config {#get_node_rpc_config}

```gdscript
Variant get_node_rpc_config() const
```

Returns a Dictionary mapping method names to their RPC configuration defined for this node using rpc_config().

Note: This method only returns the RPC configuration assigned via rpc_config(). See Script.get_rpc_config() to retrieve the RPCs defined by the Script.

### get_orphan_node_ids {#get_orphan_node_ids}

```gdscript
static typedarray::int get_orphan_node_ids()
```

Returns object IDs of all orphan nodes (nodes outside the SceneTree). Used for debugging.

Note: get_orphan_node_ids() only works in debug builds. When called in a project exported in release mode, get_orphan_node_ids() will return an empty array.

### get_owner {#get_owner}

```gdscript
Node get_owner() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_parent {#get_parent}

```gdscript
Node get_parent() const
```

Returns this node's parent node, or null if the node doesn't have a parent.

### get_path {#get_path}

```gdscript
NodePath get_path() const
```

Returns the node's absolute path, relative to the SceneTree.root. If the node is not inside the scene tree, this method fails and returns an empty NodePath.

### get_path_to {#get_path_to}

```gdscript
NodePath get_path_to(node: Node, use_unique_path: bool = false) const
```

Returns the relative NodePath from this node to the specified node. Both nodes must be in the same SceneTree or scene hierarchy, otherwise this method fails and returns an empty NodePath.

If use_unique_path is true, returns the shortest path accounting for this node's unique name (see unique_name_in_owner).

Note: If you get a relative path which starts from a unique node, the path may be longer than a normal relative path, due to the addition of the unique node's name.

### get_physics_interpolation_mode {#get_physics_interpolation_mode}

```gdscript
Node.PhysicsInterpolationMode get_physics_interpolation_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_process_delta_time {#get_physics_process_delta_time}

```gdscript
float get_physics_process_delta_time() const
```

Returns the time elapsed (in seconds) since the last physics callback. This value is identical to _physics_process()'s delta parameter, and is often consistent at run-time, unless Engine.physics_ticks_per_second is changed. See also NOTIFICATION_PHYSICS_PROCESS.

Note: The returned value will be larger than expected if running at a framerate lower than Engine.physics_ticks_per_second / Engine.max_physics_steps_per_frame FPS. This is done to avoid "spiral of death" scenarios where performance would plummet due to an ever-increasing number of physics steps per frame. This behavior affects both _process() and _physics_process(). As a result, avoid using delta for time measurements in real-world seconds. Use the Time singleton's methods for this purpose instead, such as Time.get_ticks_usec().

### get_physics_process_priority {#get_physics_process_priority}

```gdscript
int get_physics_process_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_delta_time {#get_process_delta_time}

```gdscript
float get_process_delta_time() const
```

Returns the time elapsed (in seconds) since the last process callback. This value is identical to _process()'s delta parameter, and may vary from frame to frame. See also NOTIFICATION_PROCESS.

Note: The returned value will be larger than expected if running at a framerate lower than Engine.physics_ticks_per_second / Engine.max_physics_steps_per_frame FPS. This is done to avoid "spiral of death" scenarios where performance would plummet due to an ever-increasing number of physics steps per frame. This behavior affects both _process() and _physics_process(). As a result, avoid using delta for time measurements in real-world seconds. Use the Time singleton's methods for this purpose instead, such as Time.get_ticks_usec().

### get_process_mode {#get_process_mode}

```gdscript
Node.ProcessMode get_process_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_priority {#get_process_priority}

```gdscript
int get_process_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_thread_group {#get_process_thread_group}

```gdscript
Node.ProcessThreadGroup get_process_thread_group() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_thread_group_order {#get_process_thread_group_order}

```gdscript
int get_process_thread_group_order() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_thread_messages {#get_process_thread_messages}

```gdscript
bitfield::Node.ProcessThreadMessages get_process_thread_messages() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scene_file_path {#get_scene_file_path}

```gdscript
String get_scene_file_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scene_instance_load_placeholder {#get_scene_instance_load_placeholder}

```gdscript
bool get_scene_instance_load_placeholder() const
```

Returns true if this node is an instance load placeholder. See InstancePlaceholder and set_scene_instance_load_placeholder().

### get_tree {#get_tree}

```gdscript
SceneTree get_tree() const
```

Returns the SceneTree that contains this node. If this node is not inside the tree, generates an error and returns null. See also is_inside_tree().

### get_tree_string {#get_tree_string}

```gdscript
String get_tree_string()
```

Returns the tree as a String. Used mainly for debugging purposes. This version displays the path relative to the current node, and is good for copy/pasting into the get_node() function. It also can be used in game UI/UX.

May print, for example:

```
TheGame
TheGame/Menu
TheGame/Menu/Label
TheGame/Menu/Camera2D
TheGame/SplashScreen
TheGame/SplashScreen/Camera2D
```

### get_tree_string_pretty {#get_tree_string_pretty}

```gdscript
String get_tree_string_pretty()
```

Similar to get_tree_string(), this returns the tree as a String. This version displays a more graphical representation similar to what is displayed in the Scene Dock. It is useful for inspecting larger trees.

May print, for example:

```
┖╴TheGame
   ┠╴Menu
   ┃  ┠╴Label
   ┃  ┖╴Camera2D
   ┖╴SplashScreen
      ┖╴Camera2D
```

### get_viewport {#get_viewport}

```gdscript
Viewport get_viewport() const
```

Returns the node's closest Viewport ancestor, if the node is inside the tree. Otherwise, returns null.

### get_window {#get_window}

```gdscript
Window get_window() const
```

Returns the Window that contains this node. If the node is in the main window, this is equivalent to getting the root node (get_tree().get_root()).

### has_node {#has_node}

```gdscript
bool has_node(path: NodePath) const
```

Returns true if the path points to a valid node. See also get_node().

### has_node_and_resource {#has_node_and_resource}

```gdscript
bool has_node_and_resource(path: NodePath) const
```

Returns true if path points to a valid node and its subnames point to a valid Resource, e.g. Area2D/CollisionShape2D:shape. Properties that are not Resource types (such as nodes or other Variant types) are not considered. See also get_node_and_resource().

### is_ancestor_of {#is_ancestor_of}

```gdscript
bool is_ancestor_of(node: Node) const
```

Returns true if the given node is a direct or indirect child of this node.

### is_displayed_folded {#is_displayed_folded}

```gdscript
bool is_displayed_folded() const
```

Returns true if the node is folded (collapsed) in the Scene dock. This method is intended to be used in editor plugins and tools. See also set_display_folded().

### is_editable_instance {#is_editable_instance}

```gdscript
bool is_editable_instance(node: Node) const
```

Returns true if node has editable children enabled relative to this node. This method is intended to be used in editor plugins and tools. See also set_editable_instance().

### is_greater_than {#is_greater_than}

```gdscript
bool is_greater_than(node: Node) const
```

Returns true if the given node occurs later in the scene hierarchy than this node. A node occurring later is usually processed last.

### is_in_group {#is_in_group}

```gdscript
bool is_in_group(group: StringName) const
```

Returns true if this node has been added to the given group. See add_to_group() and remove_from_group(). See also notes in the description, and the SceneTree's group methods.

### is_inside_tree {#is_inside_tree}

```gdscript
bool is_inside_tree() const
```

Returns true if this node is currently inside a SceneTree. See also get_tree().

### is_multiplayer_authority {#is_multiplayer_authority}

```gdscript
bool is_multiplayer_authority() const
```

Returns true if the local system is the multiplayer authority of this node.

### is_node_ready {#is_node_ready}

```gdscript
bool is_node_ready() const
```

Returns true if the node is ready, i.e. it's inside scene tree and all its children are initialized.

request_ready() resets it back to false.

### is_part_of_edited_scene {#is_part_of_edited_scene}

```gdscript
bool is_part_of_edited_scene() const
```

Returns true if the node is part of the scene currently opened in the editor.

### is_physics_interpolated {#is_physics_interpolated}

```gdscript
bool is_physics_interpolated() const
```

Returns true if physics interpolation is enabled for this node (see physics_interpolation_mode).

Note: Interpolation will only be active if both the flag is set and physics interpolation is enabled within the SceneTree. This can be tested using is_physics_interpolated_and_enabled().

### is_physics_interpolated_and_enabled {#is_physics_interpolated_and_enabled}

```gdscript
bool is_physics_interpolated_and_enabled() const
```

Returns true if physics interpolation is enabled (see physics_interpolation_mode) and enabled in the SceneTree.

This is a convenience version of is_physics_interpolated() that also checks whether physics interpolation is enabled globally.

See SceneTree.physics_interpolation and ProjectSettings.physics/common/physics_interpolation.

### is_physics_processing {#is_physics_processing}

```gdscript
bool is_physics_processing() const
```

Returns true if physics processing is enabled (see set_physics_process()).

### is_physics_processing_internal {#is_physics_processing_internal}

```gdscript
bool is_physics_processing_internal() const
```

Returns true if internal physics processing is enabled (see set_physics_process_internal()).

### is_processing {#is_processing}

```gdscript
bool is_processing() const
```

Returns true if processing is enabled (see set_process()).

### is_processing_input {#is_processing_input}

```gdscript
bool is_processing_input() const
```

Returns true if the node is processing input (see set_process_input()).

### is_processing_internal {#is_processing_internal}

```gdscript
bool is_processing_internal() const
```

Returns true if internal processing is enabled (see set_process_internal()).

### is_processing_shortcut_input {#is_processing_shortcut_input}

```gdscript
bool is_processing_shortcut_input() const
```

Returns true if the node is processing shortcuts (see set_process_shortcut_input()).

### is_processing_unhandled_input {#is_processing_unhandled_input}

```gdscript
bool is_processing_unhandled_input() const
```

Returns true if the node is processing unhandled input (see set_process_unhandled_input()).

### is_processing_unhandled_key_input {#is_processing_unhandled_key_input}

```gdscript
bool is_processing_unhandled_key_input() const
```

Returns true if the node is processing unhandled key input (see set_process_unhandled_key_input()).

### is_unique_name_in_owner {#is_unique_name_in_owner}

```gdscript
bool is_unique_name_in_owner() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### move_child {#move_child}

```gdscript
void move_child(child_node: Node, to_index: int)
```

Moves child_node to the given index. A node's index is the order among its siblings. If to_index is negative, the index is counted from the end of the list. See also get_child() and get_index().

Note: The processing order of several engine callbacks (_ready(), _process(), etc.) and notifications sent through propagate_notification() is affected by tree order. CanvasItem nodes are also rendered in tree order. See also process_priority.

### notify_deferred_thread_group {#notify_deferred_thread_group}

```gdscript
void notify_deferred_thread_group(what: int)
```

Similar to call_deferred_thread_group(), but for notifications.

### notify_thread_safe {#notify_thread_safe}

```gdscript
void notify_thread_safe(what: int)
```

Similar to call_thread_safe(), but for notifications.

### print_orphan_nodes {#print_orphan_nodes}

```gdscript
static void print_orphan_nodes()
```

Prints all orphan nodes (nodes outside the SceneTree). Useful for debugging.

Note: This method only works in debug builds. It does nothing in a project exported in release mode.

### print_tree {#print_tree}

```gdscript
void print_tree()
```

Prints the node and its children to the console, recursively. The node does not have to be inside the tree. This method outputs NodePaths relative to this node, and is good for copy/pasting into get_node(). See also print_tree_pretty().

May print, for example:

```
.
Menu
Menu/Label
Menu/Camera2D
SplashScreen
SplashScreen/Camera2D
```

### print_tree_pretty {#print_tree_pretty}

```gdscript
void print_tree_pretty()
```

Prints the node and its children to the console, recursively. The node does not have to be inside the tree. Similar to print_tree(), but the graphical representation looks like what is displayed in the editor's Scene dock. It is useful for inspecting larger trees.

May print, for example:

```
┖╴TheGame
   ┠╴Menu
   ┃  ┠╴Label
   ┃  ┖╴Camera2D
   ┖╴SplashScreen
      ┖╴Camera2D
```

### propagate_call {#propagate_call}

```gdscript
void propagate_call(method: StringName, args: Array = [], parent_first: bool = false)
```

Calls the given method name, passing args as arguments, on this node and all of its children, recursively.

If parent_first is true, the method is called on this node first, then on all of its children. If false, the children's methods are called first.

### propagate_notification {#propagate_notification}

```gdscript
void propagate_notification(what: int)
```

Calls Object.notification() with what on this node and all of its children, recursively.

### queue_accessibility_update {#queue_accessibility_update}

```gdscript
void queue_accessibility_update()
```

Queues an accessibility information update for this node.

### queue_free {#queue_free}

```gdscript
void queue_free()
```

Queues this node to be deleted at the end of the current frame. When deleted, all of its children are deleted as well, and all references to the node and its children become invalid.

Unlike with Object.free(), the node is not deleted instantly, and it can still be accessed before deletion. It is also safe to call queue_free() multiple times. Use Object.is_queued_for_deletion() to check if the node will be deleted at the end of the frame.

Note: The node will only be freed after all other deferred calls are finished. Using this method is not always the same as calling Object.free() through Object.call_deferred().

### remove_child {#remove_child}

```gdscript
void remove_child(node: Node)
```

Removes a child node. The node, along with its children, are not deleted. To delete a node, see queue_free().

Note: When this node is inside the tree, this method sets the owner of the removed node (or its descendants) to null, if their owner is no longer an ancestor (see is_ancestor_of()).

### remove_from_group {#remove_from_group}

```gdscript
void remove_from_group(group: StringName)
```

Removes the node from the given group. Does nothing if the node is not in the group. See also notes in the description, and the SceneTree's group methods.

### reparent {#reparent}

```gdscript
void reparent(new_parent: Node, keep_global_transform: bool = true)
```

Changes the parent of this Node to the new_parent. The node needs to already have a parent. The node's owner is preserved if its owner is still reachable from the new location (i.e., the node is still a descendant of the new parent after the operation).

If keep_global_transform is true, the node's global transform will be preserved if supported. Node2D, Node3D and Control support this argument (but Control keeps only position).

Warning: If ProjectSettings.physics/common/physics_interpolation is enabled and reparenting causes a large change in global transform, the object may appear to move from its old position to its new one over the next physics tick. To avoid this, call reset_physics_interpolation() after reparenting.

### replace_by {#replace_by}

```gdscript
void replace_by(node: Node, keep_groups: bool = false)
```

Replaces this node by the given node. All children of this node are moved to node.

If keep_groups is true, the node is added to the same groups that the replaced node is in (see add_to_group()).

Warning: The replaced node is removed from the tree, but it is not deleted. To prevent memory leaks, store a reference to the node in a variable, or use Object.free().

### request_ready {#request_ready}

```gdscript
void request_ready()
```

Requests _ready() to be called again the next time the node enters the tree. Does not immediately call _ready().

Note: This method only affects the current node. If the node's children also need to request ready, this method needs to be called for each one of them. When the node and its children enter the tree again, the order of _ready() callbacks will be the same as normal.

### reset_physics_interpolation {#reset_physics_interpolation}

```gdscript
void reset_physics_interpolation()
```

When physics interpolation is active, moving a node to a radically different transform (such as placement within a level) can result in a visible glitch as the object is rendered moving from the old to new position over the physics tick.

That glitch can be prevented by calling this method, which temporarily disables interpolation until the physics tick is complete.

The notification NOTIFICATION_RESET_PHYSICS_INTERPOLATION will be received by the node and all children recursively.

Note: This function should be called after moving the node, rather than before.

### rpc {#rpc}

```gdscript
Error rpc(method: StringName, ...)
```

Sends a remote procedure call request for the given method to peers on the network (and locally), sending additional arguments to the method called by the RPC. The call request will only be received by nodes with the same NodePath, including the exact same name. Behavior depends on the RPC configuration for the given method (see rpc_config() and @GDScript.@rpc). By default, methods are not exposed to RPCs.

May return @GlobalScope.OK if the call is successful, @GlobalScope.ERR_INVALID_PARAMETER if the arguments passed in the method do not match, @GlobalScope.ERR_UNCONFIGURED if the node's multiplayer cannot be fetched (such as when the node is not inside the tree), @GlobalScope.ERR_CONNECTION_ERROR if multiplayer's connection is not available.

Note: You can only safely use RPCs on clients after you received the MultiplayerAPI.connected_to_server signal from the MultiplayerAPI. You also need to keep track of the connection state, either by the MultiplayerAPI signals like MultiplayerAPI.server_disconnected or by checking (get_multiplayer().peer.get_connection_status() == CONNECTION_CONNECTED).

### rpc_config {#rpc_config}

```gdscript
void rpc_config(method: StringName, config: Variant)
```

Changes the RPC configuration for the given method. config should either be null to disable the feature (as by default), or a Dictionary containing the following entries:

- rpc_mode: see RPCMode;
- transfer_mode: see TransferMode;
- call_local: if true, the method will also be called locally;
- channel: an int representing the channel to send the RPC on.

Note: In GDScript, this method corresponds to the @GDScript.@rpc annotation, with various parameters passed (@rpc(any), @rpc(authority)...). See also the high-level multiplayer tutorial.

### rpc_id {#rpc_id}

```gdscript
Error rpc_id(peer_id: int, method: StringName, ...)
```

Sends a rpc() to a specific peer identified by peer_id (see MultiplayerPeer.set_target_peer()).

May return @GlobalScope.OK if the call is successful, @GlobalScope.ERR_INVALID_PARAMETER if the arguments passed in the method do not match, @GlobalScope.ERR_UNCONFIGURED if the node's multiplayer cannot be fetched (such as when the node is not inside the tree), @GlobalScope.ERR_CONNECTION_ERROR if multiplayer's connection is not available.

### set_auto_translate_mode {#set_auto_translate_mode}

```gdscript
void set_auto_translate_mode(mode: Node.AutoTranslateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deferred_thread_group {#set_deferred_thread_group}

```gdscript
void set_deferred_thread_group(property: StringName, value: Variant)
```

Similar to call_deferred_thread_group(), but for setting properties.

### set_display_folded {#set_display_folded}

```gdscript
void set_display_folded(fold: bool)
```

If set to true, the node appears folded in the Scene dock. As a result, all of its children are hidden. This method is intended to be used in editor plugins and tools, but it also works in release builds. See also is_displayed_folded().

### set_editable_instance {#set_editable_instance}

```gdscript
void set_editable_instance(node: Node, is_editable: bool)
```

Set to true to allow all nodes owned by node to be available, and editable, in the Scene dock, even if their owner is not the scene root. This method is intended to be used in editor plugins and tools, but it also works in release builds. See also is_editable_instance().

### set_editor_description {#set_editor_description}

```gdscript
void set_editor_description(editor_description: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_multiplayer_authority {#set_multiplayer_authority}

```gdscript
void set_multiplayer_authority(id: int, recursive: bool = true)
```

Sets the node's multiplayer authority to the peer with the given peer id. The multiplayer authority is the peer that has authority over the node on the network. Defaults to peer ID 1 (the server). Useful in conjunction with rpc_config() and the MultiplayerAPI.

If recursive is true, the given peer is recursively set as the authority for all children of this node.

Warning: This does not automatically replicate the new authority to other peers. It is the developer's responsibility to do so. You may replicate the new authority's information using MultiplayerSpawner.spawn_function, an RPC, or a MultiplayerSynchronizer. Furthermore, the parent's authority does not propagate to newly added children.

### set_name {#set_name}

```gdscript
void set_name(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_owner {#set_owner}

```gdscript
void set_owner(owner: Node)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_interpolation_mode {#set_physics_interpolation_mode}

```gdscript
void set_physics_interpolation_mode(mode: Node.PhysicsInterpolationMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_process {#set_physics_process}

```gdscript
void set_physics_process(enable: bool)
```

If set to true, enables physics (fixed framerate) processing. When a node is being processed, it will receive a NOTIFICATION_PHYSICS_PROCESS at a fixed (usually 60 FPS, see Engine.physics_ticks_per_second to change) interval (and the _physics_process() callback will be called if it exists).

Note: If _physics_process() is overridden, this will be automatically enabled before _ready() is called.

### set_physics_process_internal {#set_physics_process_internal}

```gdscript
void set_physics_process_internal(enable: bool)
```

If set to true, enables internal physics for this node. Internal physics processing happens in isolation from the normal _physics_process() calls and is used by some nodes internally to guarantee proper functioning even if the node is paused or physics processing is disabled for scripting (set_physics_process()).

Warning: Built-in nodes rely on internal processing for their internal logic. Disabling it is unsafe and may lead to unexpected behavior. Use this method if you know what you are doing.

### set_physics_process_priority {#set_physics_process_priority}

```gdscript
void set_physics_process_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process {#set_process}

```gdscript
void set_process(enable: bool)
```

If set to true, enables processing. When a node is being processed, it will receive a NOTIFICATION_PROCESS on every drawn frame (and the _process() callback will be called if it exists).

Note: If _process() is overridden, this will be automatically enabled before _ready() is called.

Note: This method only affects the _process() callback, i.e. it has no effect on other callbacks like _physics_process(). If you want to disable all processing for the node, set process_mode to PROCESS_MODE_DISABLED.

### set_process_input {#set_process_input}

```gdscript
void set_process_input(enable: bool)
```

If set to true, enables input processing.

Note: If _input() is overridden, this will be automatically enabled before _ready() is called. Input processing is also already enabled for GUI controls, such as Button and TextEdit.

### set_process_internal {#set_process_internal}

```gdscript
void set_process_internal(enable: bool)
```

If set to true, enables internal processing for this node. Internal processing happens in isolation from the normal _process() calls and is used by some nodes internally to guarantee proper functioning even if the node is paused or processing is disabled for scripting (set_process()).

Warning: Built-in nodes rely on internal processing for their internal logic. Disabling it is unsafe and may lead to unexpected behavior. Use this method if you know what you are doing.

### set_process_mode {#set_process_mode}

```gdscript
void set_process_mode(mode: Node.ProcessMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_priority {#set_process_priority}

```gdscript
void set_process_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_shortcut_input {#set_process_shortcut_input}

```gdscript
void set_process_shortcut_input(enable: bool)
```

If set to true, enables shortcut processing for this node.

Note: If _shortcut_input() is overridden, this will be automatically enabled before _ready() is called.

### set_process_thread_group {#set_process_thread_group}

```gdscript
void set_process_thread_group(mode: Node.ProcessThreadGroup)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_thread_group_order {#set_process_thread_group_order}

```gdscript
void set_process_thread_group_order(order: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_thread_messages {#set_process_thread_messages}

```gdscript
void set_process_thread_messages(flags: bitfield::Node.ProcessThreadMessages)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_unhandled_input {#set_process_unhandled_input}

```gdscript
void set_process_unhandled_input(enable: bool)
```

If set to true, enables unhandled input processing. It enables the node to receive all input that was not previously handled (usually by a Control).

Note: If _unhandled_input() is overridden, this will be automatically enabled before _ready() is called. Unhandled input processing is also already enabled for GUI controls, such as Button and TextEdit.

### set_process_unhandled_key_input {#set_process_unhandled_key_input}

```gdscript
void set_process_unhandled_key_input(enable: bool)
```

If set to true, enables unhandled key input processing.

Note: If _unhandled_key_input() is overridden, this will be automatically enabled before _ready() is called.

### set_scene_file_path {#set_scene_file_path}

```gdscript
void set_scene_file_path(scene_file_path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scene_instance_load_placeholder {#set_scene_instance_load_placeholder}

```gdscript
void set_scene_instance_load_placeholder(load_placeholder: bool)
```

If set to true, the node becomes an InstancePlaceholder when packed and instantiated from a PackedScene. See also get_scene_instance_load_placeholder().

### set_thread_safe {#set_thread_safe}

```gdscript
void set_thread_safe(property: StringName, value: Variant)
```

Similar to call_thread_safe(), but for setting properties.

### set_translation_domain_inherited {#set_translation_domain_inherited}

```gdscript
void set_translation_domain_inherited()
```

Makes this node inherit the translation domain from its parent node. If this node has no parent, the main translation domain will be used.

This is the default behavior for all nodes. Calling Object.set_translation_domain() disables this behavior.

### set_unique_name_in_owner {#set_unique_name_in_owner}

```gdscript
void set_unique_name_in_owner(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update_configuration_warnings {#update_configuration_warnings}

```gdscript
void update_configuration_warnings()
```

Refreshes the warnings displayed for this node in the Scene dock. Use _get_configuration_warnings() to customize the warning messages to display.

## 属性

### name {#name}

```gdscript
name: StringName
```

- void set_name(value: StringName)
- StringName get_name()

The name of the node. This name must be unique among the siblings (other child nodes from the same parent). When set to an existing sibling's name, the node is automatically renamed.

Note: When changing the name, the following characters will be replaced with an underscore: (. : @ / " %). In particular, the @ character is reserved for auto-generated names. See also String.validate_node_name().

### unique_name_in_owner {#unique_name_in_owner}

```gdscript
unique_name_in_owner: bool
```

- void set_unique_name_in_owner(value: bool)
- bool is_unique_name_in_owner()

If true, the node can be accessed from any node sharing the same owner or from the owner itself, with special %Name syntax in get_node().

Note: If another node with the same owner shares the same name as this node, the other node will no longer be accessible as unique.

### scene_file_path {#scene_file_path}

```gdscript
scene_file_path: String
```

- void set_scene_file_path(value: String)
- String get_scene_file_path()

The original scene's file path, if the node has been instantiated from a PackedScene file. Only scene root nodes contains this.

### owner {#owner}

```gdscript
owner: Node
```

- void set_owner(value: Node)
- Node get_owner()

The owner of this node. The owner must be an ancestor of this node. When packing the owner node in a PackedScene, all the nodes it owns are also saved with it. See also unique_name_in_owner.

Note: In the editor, nodes not owned by the scene root are usually not displayed in the Scene dock, and will not be saved. To prevent this, remember to set the owner after calling add_child().

Note: The owner needs to be the current scene root. See Instancing scenes in the documentation for more information.

### multiplayer {#multiplayer}

```gdscript
multiplayer: MultiplayerAPI
```

- MultiplayerAPI get_multiplayer()

The MultiplayerAPI instance associated with this node. See SceneTree.get_multiplayer().

Note: Renaming the node, or moving it in the tree, will not move the MultiplayerAPI to the new path, you will have to update this manually.

### process_mode {#process_mode}

```gdscript
process_mode: int
```

- void set_process_mode(value: ProcessMode)
- ProcessMode get_process_mode()

The node's processing behavior. To check if the node can process in its current mode, use can_process().

### process_priority {#process_priority}

```gdscript
process_priority: int
```

- void set_process_priority(value: int)
- int get_process_priority()

The node's execution order of the process callbacks (_process(), NOTIFICATION_PROCESS, and NOTIFICATION_INTERNAL_PROCESS). Nodes whose priority value is lower call their process callbacks first, regardless of tree order.

### process_physics_priority {#process_physics_priority}

```gdscript
process_physics_priority: int
```

- void set_physics_process_priority(value: int)
- int get_physics_process_priority()

Similar to process_priority but for NOTIFICATION_PHYSICS_PROCESS, _physics_process(), or NOTIFICATION_INTERNAL_PHYSICS_PROCESS.

### process_thread_group {#process_thread_group}

```gdscript
process_thread_group: int
```

- void set_process_thread_group(value: ProcessThreadGroup)
- ProcessThreadGroup get_process_thread_group()

Set the process thread group for this node (basically, whether it receives NOTIFICATION_PROCESS, NOTIFICATION_PHYSICS_PROCESS, _process() or _physics_process() (and the internal versions) on the main thread or in a sub-thread.

By default, the thread group is PROCESS_THREAD_GROUP_INHERIT, which means that this node belongs to the same thread group as the parent node. The thread groups means that nodes in a specific thread group will process together, separate to other thread groups (depending on process_thread_group_order). If the value is set is PROCESS_THREAD_GROUP_SUB_THREAD, this thread group will occur on a sub thread (not the main thread), otherwise if set to PROCESS_THREAD_GROUP_MAIN_THREAD it will process on the main thread. If there is not a parent or grandparent node set to something other than inherit, the node will belong to the default thread group. This default group will process on the main thread and its group order is 0.

During processing in a sub-thread, accessing most functions in nodes outside the thread group is forbidden (and it will result in an error in debug mode). Use Object.call_deferred(), call_thread_safe(), call_deferred_thread_group() and the likes in order to communicate from the thread groups to the main thread (or to other thread groups).

To better understand process thread groups, the idea is that any node set to any other value than PROCESS_THREAD_GROUP_INHERIT will include any child (and grandchild) nodes set to inherit into its process thread group. This means that the processing of all the nodes in the group will happen together, at the same time as the node including them.

### process_thread_group_order {#process_thread_group_order}

```gdscript
process_thread_group_order: int
```

- void set_process_thread_group_order(value: int)
- int get_process_thread_group_order()

Change the process thread group order. Groups with a lesser order will process before groups with a greater order. This is useful when a large amount of nodes process in sub thread and, afterwards, another group wants to collect their result in the main thread, as an example.

### process_thread_messages {#process_thread_messages}

```gdscript
process_thread_messages: int
```

- void set_process_thread_messages(value: BitField[ProcessThreadMessages])
- BitField[ProcessThreadMessages] get_process_thread_messages()

Set whether the current thread group will process messages (calls to call_deferred_thread_group() on threads), and whether it wants to receive them during regular process or physics process callbacks.

### physics_interpolation_mode {#physics_interpolation_mode}

```gdscript
physics_interpolation_mode: int
```

- void set_physics_interpolation_mode(value: PhysicsInterpolationMode)
- PhysicsInterpolationMode get_physics_interpolation_mode()

The physics interpolation mode to use for this node. Only effective if ProjectSettings.physics/common/physics_interpolation or SceneTree.physics_interpolation is true.

By default, nodes inherit the physics interpolation mode from their parent. This property can enable or disable physics interpolation individually for each node, regardless of their parents' physics interpolation mode.

Note: Some node types like VehicleWheel3D have physics interpolation disabled by default, as they rely on their own custom solution.

Note: When teleporting a node to a distant position, it's recommended to temporarily disable interpolation with reset_physics_interpolation() after moving the node. This avoids creating a visual streak between the old and new positions.

### auto_translate_mode {#auto_translate_mode}

```gdscript
auto_translate_mode: int
```

- void set_auto_translate_mode(value: AutoTranslateMode)
- AutoTranslateMode get_auto_translate_mode()

Defines if any text should automatically change to its translated version depending on the current locale (for nodes such as Label, RichTextLabel, Window, etc.). Also decides if the node's strings should be parsed for translation template generation.

Note: For the root node, auto translate mode can also be set via ProjectSettings.internationalization/rendering/root_node_auto_translate.

### editor_description {#editor_description}

```gdscript
editor_description: String
```

- void set_editor_description(value: String)
- String get_editor_description()

An optional description to the node. It will be displayed as a tooltip when hovering over the node in the editor's Scene dock.

## 信号

### ready {#ready}

```gdscript
signal ready()
```

Emitted when the node is considered ready, after _ready() is called.

### renamed {#renamed}

```gdscript
signal renamed()
```

Emitted when the node's name is changed, if the node is inside the tree.

### tree_entered {#tree_entered}

```gdscript
signal tree_entered()
```

Emitted when the node enters the tree.

This signal is emitted after the related NOTIFICATION_ENTER_TREE notification.

### tree_exiting {#tree_exiting}

```gdscript
signal tree_exiting()
```

Emitted when the node is just about to exit the tree. The node is still valid. As such, this is the right place for de-initialization (or a "destructor", if you will).

This signal is emitted after the node's _exit_tree(), and before the related NOTIFICATION_EXIT_TREE.

### tree_exited {#tree_exited}

```gdscript
signal tree_exited()
```

Emitted after the node exits the tree and is no longer active.

This signal is emitted after the related NOTIFICATION_EXIT_TREE notification.

### child_entered_tree {#child_entered_tree}

```gdscript
signal child_entered_tree(node: Node)
```

Emitted when the child node enters the SceneTree, usually because this node entered the tree (see tree_entered), or add_child() has been called.

This signal is emitted after the child node's own NOTIFICATION_ENTER_TREE and tree_entered.

### child_exiting_tree {#child_exiting_tree}

```gdscript
signal child_exiting_tree(node: Node)
```

Emitted when the child node is about to exit the SceneTree, usually because this node is exiting the tree (see tree_exiting), or because the child node is being removed or freed.

When this signal is received, the child node is still accessible inside the tree. This signal is emitted after the child node's own tree_exiting and NOTIFICATION_EXIT_TREE.

### child_order_changed {#child_order_changed}

```gdscript
signal child_order_changed()
```

Emitted when the list of children is changed. This happens when child nodes are added, moved or removed.

### replacing_by {#replacing_by}

```gdscript
signal replacing_by(node: Node)
```

Emitted when this node is being replaced by the node, see replace_by().

This signal is emitted after node has been added as a child of the original parent node, but before all original child nodes have been reparented to node.

### editor_description_changed {#editor_description_changed}

```gdscript
signal editor_description_changed(node: Node)
```

Emitted when the node's editor description field changed.

### editor_state_changed {#editor_state_changed}

```gdscript
signal editor_state_changed()
```

Emitted when an attribute of the node that is relevant to the editor is changed. Only emitted in the editor.

## 枚举

### ProcessMode {#enum-ProcessMode}

```gdscript
enum ProcessMode { PROCESS_MODE_INHERIT = 0, PROCESS_MODE_PAUSABLE = 1, PROCESS_MODE_WHEN_PAUSED = 2, PROCESS_MODE_ALWAYS = 3, PROCESS_MODE_DISABLED = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROCESS_MODE_INHERIT {#PROCESS_MODE_INHERIT}

```gdscript
PROCESS_MODE_INHERIT = 0
```

Inherits process_mode from the node's parent. This is the default for any newly created node.

#### PROCESS_MODE_PAUSABLE {#PROCESS_MODE_PAUSABLE}

```gdscript
PROCESS_MODE_PAUSABLE = 1
```

Processes when SceneTree.paused is false. This is the inverse of PROCESS_MODE_WHEN_PAUSED, and the default for the root node.

#### PROCESS_MODE_WHEN_PAUSED {#PROCESS_MODE_WHEN_PAUSED}

```gdscript
PROCESS_MODE_WHEN_PAUSED = 2
```

Processes only when SceneTree.paused is true. This is the inverse of PROCESS_MODE_PAUSABLE.

#### PROCESS_MODE_ALWAYS {#PROCESS_MODE_ALWAYS}

```gdscript
PROCESS_MODE_ALWAYS = 3
```

Always processes. Keeps processing, ignoring SceneTree.paused. This is the inverse of PROCESS_MODE_DISABLED.

#### PROCESS_MODE_DISABLED {#PROCESS_MODE_DISABLED}

```gdscript
PROCESS_MODE_DISABLED = 4
```

Never processes. Completely disables processing, ignoring SceneTree.paused. This is the inverse of PROCESS_MODE_ALWAYS.

---

### ProcessThreadGroup {#enum-ProcessThreadGroup}

```gdscript
enum ProcessThreadGroup { PROCESS_THREAD_GROUP_INHERIT = 0, PROCESS_THREAD_GROUP_MAIN_THREAD = 1, PROCESS_THREAD_GROUP_SUB_THREAD = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROCESS_THREAD_GROUP_INHERIT {#PROCESS_THREAD_GROUP_INHERIT}

```gdscript
PROCESS_THREAD_GROUP_INHERIT = 0
```

Process this node based on the thread group mode of the first parent (or grandparent) node that has a thread group mode that is not inherit. See process_thread_group for more information.

#### PROCESS_THREAD_GROUP_MAIN_THREAD {#PROCESS_THREAD_GROUP_MAIN_THREAD}

```gdscript
PROCESS_THREAD_GROUP_MAIN_THREAD = 1
```

Process this node (and child nodes set to inherit) on the main thread. See process_thread_group for more information.

#### PROCESS_THREAD_GROUP_SUB_THREAD {#PROCESS_THREAD_GROUP_SUB_THREAD}

```gdscript
PROCESS_THREAD_GROUP_SUB_THREAD = 2
```

Process this node (and child nodes set to inherit) on a sub-thread. See process_thread_group for more information.

---

flags ProcessThreadMessages:

### ProcessThreadMessages {#enum-ProcessThreadMessages}

```gdscript
enum ProcessThreadMessages（位标志） { FLAG_PROCESS_THREAD_MESSAGES = 1, FLAG_PROCESS_THREAD_MESSAGES_PHYSICS = 2, FLAG_PROCESS_THREAD_MESSAGES_ALL = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_PROCESS_THREAD_MESSAGES {#FLAG_PROCESS_THREAD_MESSAGES}

```gdscript
FLAG_PROCESS_THREAD_MESSAGES = 1
```

Allows this node to process threaded messages created with call_deferred_thread_group() right before _process() is called.

#### FLAG_PROCESS_THREAD_MESSAGES_PHYSICS {#FLAG_PROCESS_THREAD_MESSAGES_PHYSICS}

```gdscript
FLAG_PROCESS_THREAD_MESSAGES_PHYSICS = 2
```

Allows this node to process threaded messages created with call_deferred_thread_group() right before _physics_process() is called.

#### FLAG_PROCESS_THREAD_MESSAGES_ALL {#FLAG_PROCESS_THREAD_MESSAGES_ALL}

```gdscript
FLAG_PROCESS_THREAD_MESSAGES_ALL = 3
```

Allows this node to process threaded messages created with call_deferred_thread_group() right before either _process() or _physics_process() are called.

---

### PhysicsInterpolationMode {#enum-PhysicsInterpolationMode}

```gdscript
enum PhysicsInterpolationMode { PHYSICS_INTERPOLATION_MODE_INHERIT = 0, PHYSICS_INTERPOLATION_MODE_ON = 1, PHYSICS_INTERPOLATION_MODE_OFF = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PHYSICS_INTERPOLATION_MODE_INHERIT {#PHYSICS_INTERPOLATION_MODE_INHERIT}

```gdscript
PHYSICS_INTERPOLATION_MODE_INHERIT = 0
```

Inherits physics_interpolation_mode from the node's parent. This is the default for any newly created node.

#### PHYSICS_INTERPOLATION_MODE_ON {#PHYSICS_INTERPOLATION_MODE_ON}

```gdscript
PHYSICS_INTERPOLATION_MODE_ON = 1
```

Enables physics interpolation for this node and for children set to PHYSICS_INTERPOLATION_MODE_INHERIT. This is the default for the root node.

#### PHYSICS_INTERPOLATION_MODE_OFF {#PHYSICS_INTERPOLATION_MODE_OFF}

```gdscript
PHYSICS_INTERPOLATION_MODE_OFF = 2
```

Disables physics interpolation for this node and for children set to PHYSICS_INTERPOLATION_MODE_INHERIT.

---

### DuplicateFlags {#enum-DuplicateFlags}

```gdscript
enum DuplicateFlags { DUPLICATE_SIGNALS = 1, DUPLICATE_GROUPS = 2, DUPLICATE_SCRIPTS = 4, DUPLICATE_USE_INSTANTIATION = 8, DUPLICATE_INTERNAL_STATE = 16, DUPLICATE_DEFAULT = 15 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DUPLICATE_SIGNALS {#DUPLICATE_SIGNALS}

```gdscript
DUPLICATE_SIGNALS = 1
```

Duplicate the node's signal connections that are connected with the Object.CONNECT_PERSIST flag.

#### DUPLICATE_GROUPS {#DUPLICATE_GROUPS}

```gdscript
DUPLICATE_GROUPS = 2
```

Duplicate the node's groups.

#### DUPLICATE_SCRIPTS {#DUPLICATE_SCRIPTS}

```gdscript
DUPLICATE_SCRIPTS = 4
```

Duplicate the node's script (also overriding the duplicated children's scripts, if combined with DUPLICATE_USE_INSTANTIATION).

#### DUPLICATE_USE_INSTANTIATION {#DUPLICATE_USE_INSTANTIATION}

```gdscript
DUPLICATE_USE_INSTANTIATION = 8
```

Duplicate using PackedScene.instantiate(). If the node comes from a scene saved on disk, reuses PackedScene.instantiate() as the base for the duplicated node and its children.

#### DUPLICATE_INTERNAL_STATE {#DUPLICATE_INTERNAL_STATE}

```gdscript
DUPLICATE_INTERNAL_STATE = 16
```

Duplicate also non-serializable variables (i.e. without @GlobalScope.PROPERTY_USAGE_STORAGE).

#### DUPLICATE_DEFAULT {#DUPLICATE_DEFAULT}

```gdscript
DUPLICATE_DEFAULT = 15
```

Duplicate using default flags. This constant is useful to add or remove a single flag.

```gdscript
# Duplicate non-exported variables.
var dupe = duplicate(DUPLICATE_DEFAULT | DUPLICATE_INTERNAL_STATE)
```

---

### InternalMode {#enum-InternalMode}

```gdscript
enum InternalMode { INTERNAL_MODE_DISABLED = 0, INTERNAL_MODE_FRONT = 1, INTERNAL_MODE_BACK = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERNAL_MODE_DISABLED {#INTERNAL_MODE_DISABLED}

```gdscript
INTERNAL_MODE_DISABLED = 0
```

The node will not be internal.

#### INTERNAL_MODE_FRONT {#INTERNAL_MODE_FRONT}

```gdscript
INTERNAL_MODE_FRONT = 1
```

The node will be placed at the beginning of the parent's children, before any non-internal sibling.

#### INTERNAL_MODE_BACK {#INTERNAL_MODE_BACK}

```gdscript
INTERNAL_MODE_BACK = 2
```

The node will be placed at the end of the parent's children, after any non-internal sibling.

---

### AutoTranslateMode {#enum-AutoTranslateMode}

```gdscript
enum AutoTranslateMode { AUTO_TRANSLATE_MODE_INHERIT = 0, AUTO_TRANSLATE_MODE_ALWAYS = 1, AUTO_TRANSLATE_MODE_DISABLED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AUTO_TRANSLATE_MODE_INHERIT {#AUTO_TRANSLATE_MODE_INHERIT}

```gdscript
AUTO_TRANSLATE_MODE_INHERIT = 0
```

Inherits auto_translate_mode from the node's parent. This is the default for any newly created node.

#### AUTO_TRANSLATE_MODE_ALWAYS {#AUTO_TRANSLATE_MODE_ALWAYS}

```gdscript
AUTO_TRANSLATE_MODE_ALWAYS = 1
```

Always automatically translate. This is the inverse of AUTO_TRANSLATE_MODE_DISABLED, and the default for the root node.

#### AUTO_TRANSLATE_MODE_DISABLED {#AUTO_TRANSLATE_MODE_DISABLED}

```gdscript
AUTO_TRANSLATE_MODE_DISABLED = 2
```

Never automatically translate. This is the inverse of AUTO_TRANSLATE_MODE_ALWAYS.

String parsing for translation template generation will be skipped for this node and children that are set to AUTO_TRANSLATE_MODE_INHERIT.

---

## 常量

### NOTIFICATION_ENTER_TREE {#NOTIFICATION_ENTER_TREE}

```gdscript
const NOTIFICATION_ENTER_TREE = 10
```

Notification received when the node enters a SceneTree. See _enter_tree().

This notification is received before the related tree_entered signal.

NOTIFICATION_EXIT_TREE = 11 

Notification received when the node is about to exit a SceneTree. See _exit_tree().

This notification is received after the related tree_exiting signal.

This notification is sent in reversed order.

NOTIFICATION_MOVED_IN_PARENT = 12 

Deprecated: This notification is no longer sent by the engine. Use NOTIFICATION_CHILD_ORDER_CHANGED instead.

NOTIFICATION_READY = 13 

Notification received when the node is ready. See _ready().

NOTIFICATION_PAUSED = 14 

Notification received when the node is paused. See process_mode.

NOTIFICATION_UNPAUSED = 15 

Notification received when the node is unpaused. See process_mode.

NOTIFICATION_PHYSICS_PROCESS = 16 

Notification received from the tree every physics frame when is_physics_processing() returns true. See _physics_process().

NOTIFICATION_PROCESS = 17 

Notification received from the tree every rendered frame when is_processing() returns true. See _process().

NOTIFICATION_PARENTED = 18 

Notification received when the node is set as a child of another node (see add_child() and add_sibling()).

Note: This does not mean that the node entered the SceneTree.

NOTIFICATION_UNPARENTED = 19 

Notification received when the parent node calls remove_child() on this node.

Note: This does not mean that the node exited the SceneTree.

NOTIFICATION_SCENE_INSTANTIATED = 20 

Notification received only by the newly instantiated scene root node, when PackedScene.instantiate() is completed.

NOTIFICATION_DRAG_BEGIN = 21 

Notification received when a drag operation begins. All nodes receive this notification, not only the dragged one.

Can be triggered either by dragging a Control that provides drag data (see Control._get_drag_data()) or using Control.force_drag().

Use Viewport.gui_get_drag_data() to get the dragged data.

NOTIFICATION_DRAG_END = 22 

Notification received when a drag operation ends.

Use Viewport.gui_is_drag_successful() to check if the drag succeeded.

NOTIFICATION_PATH_RENAMED = 23 

Notification received when the node's name or one of its ancestors' name is changed. This notification is not received when the node is removed from the SceneTree.

NOTIFICATION_CHILD_ORDER_CHANGED = 24 

Notification received when the list of children is changed. This happens when child nodes are added, moved or removed.

NOTIFICATION_INTERNAL_PROCESS = 25 

Notification received from the tree every rendered frame when is_processing_internal() returns true.

NOTIFICATION_INTERNAL_PHYSICS_PROCESS = 26 

Notification received from the tree every physics frame when is_physics_processing_internal() returns true.

NOTIFICATION_POST_ENTER_TREE = 27 

Notification received when the node enters the tree, just before NOTIFICATION_READY may be received. Unlike the latter, it is sent every time the node enters tree, not just once.

NOTIFICATION_DISABLED = 28 

Notification received when the node is disabled. See PROCESS_MODE_DISABLED.

NOTIFICATION_ENABLED = 29 

Notification received when the node is enabled again after being disabled. See PROCESS_MODE_DISABLED.

NOTIFICATION_RESET_PHYSICS_INTERPOLATION = 2001 

Notification received when reset_physics_interpolation() is called on the node or its ancestors.

NOTIFICATION_EDITOR_PRE_SAVE = 9001 

Notification received right before the scene with the node is saved in the editor. This notification is only sent in the Godot editor and will not occur in exported projects.

NOTIFICATION_EDITOR_POST_SAVE = 9002 

Notification received right after the scene with the node is saved in the editor. This notification is only sent in the Godot editor and will not occur in exported projects.

NOTIFICATION_WM_MOUSE_ENTER = 1002 

Notification received when the mouse enters the window.

Implemented for embedded windows and on desktop and web platforms.

NOTIFICATION_WM_MOUSE_EXIT = 1003 

Notification received when the mouse leaves the window.

Implemented for embedded windows and on desktop and web platforms.

NOTIFICATION_WM_WINDOW_FOCUS_IN = 1004 

Notification received from the OS when the node's Window ancestor is focused. This may be a change of focus between two windows of the same engine instance, or from the OS desktop or a third-party application to a window of the game (in which case NOTIFICATION_APPLICATION_FOCUS_IN is also received).

A Window node receives this notification when it is focused.

NOTIFICATION_WM_WINDOW_FOCUS_OUT = 1005 

Notification received from the OS when the node's Window ancestor is defocused. This may be a change of focus between two windows of the same engine instance, or from a window of the game to the OS desktop or a third-party application (in which case NOTIFICATION_APPLICATION_FOCUS_OUT is also received).

A Window node receives this notification when it is defocused.

NOTIFICATION_WM_CLOSE_REQUEST = 1006 

Notification received from the OS when a close request is sent (e.g. closing the window with a "Close" button or Alt + F4).

Implemented on desktop platforms.

NOTIFICATION_WM_GO_BACK_REQUEST = 1007 

Notification received from the OS when a go back request is sent (e.g. pressing the "Back" button on Android).

Implemented only on Android.

NOTIFICATION_WM_SIZE_CHANGED = 1008 

Notification received when the window is resized.

Note: Only the resized Window node receives this notification, and it's not propagated to the child nodes.

NOTIFICATION_WM_DPI_CHANGE = 1009 

Notification received from the OS when the screen's dots per inch (DPI) scale is changed. Only implemented on macOS.

NOTIFICATION_VP_MOUSE_ENTER = 1010 

Notification received when the mouse cursor enters the Viewport's visible area, that is not occluded behind other Controls or Windows, provided its Viewport.gui_disable_input is false and regardless if it's currently focused or not.

NOTIFICATION_VP_MOUSE_EXIT = 1011 

Notification received when the mouse cursor leaves the Viewport's visible area, that is not occluded behind other Controls or Windows, provided its Viewport.gui_disable_input is false and regardless if it's currently focused or not.

NOTIFICATION_WM_POSITION_CHANGED = 1012 

Notification received when the window is moved.

NOTIFICATION_WM_OUTPUT_MAX_LINEAR_VALUE_CHANGED = 1013 

Notification received when the output max linear value returned by Window.get_output_max_linear_value() has changed.

This occurs when HDR output is enabled or disabled and when any HDR output luminance values of the window have changed, such as when the player adjusts their screen brightness setting or moves the window to a different screen.

NOTIFICATION_OS_MEMORY_WARNING = 2009 

Notification received from the OS when the application is exceeding its allocated memory.

Implemented only on iOS.

NOTIFICATION_TRANSLATION_CHANGED = 2010 

Notification received when translations may have changed. Can be triggered by the user changing the locale, changing auto_translate_mode or when the node enters the scene tree. Can be used to respond to language changes, for example to change the UI strings on the fly. Useful when working with the built-in translation support, like Object.tr().

Note: This notification is received alongside NOTIFICATION_ENTER_TREE, so if you are instantiating a scene, the child nodes will not be initialized yet. You can use it to setup translations for this node, child nodes created from script, or if you want to access child nodes added in the editor, make sure the node is ready using is_node_ready().

```gdscript
func _notification(what):
    if what == NOTIFICATION_TRANSLATION_CHANGED:
        if not is_node_ready():
            await ready # Wait until ready signal.
        $Label.text = atr("%d Bananas") % banana_counter
```

NOTIFICATION_WM_ABOUT = 2011 

Notification received from the OS when a request for "About" information is sent.

Implemented only on macOS.

NOTIFICATION_CRASH = 2012 

Notification received from Godot's crash handler when the engine is about to crash.

Implemented on desktop platforms, if the crash handler is enabled.

NOTIFICATION_OS_IME_UPDATE = 2013 

Notification received from the OS when an update of the Input Method Engine occurs (e.g. change of IME cursor position or composition string).

Implemented on desktop and web platforms.

NOTIFICATION_APPLICATION_RESUMED = 2014 

Notification received from the OS when the application is resumed.

Specific to the Android and iOS platforms.

NOTIFICATION_APPLICATION_PAUSED = 2015 

Notification received from the OS when the application is paused.

Specific to the Android and iOS platforms.

Note: On iOS, you only have approximately 5 seconds to finish a task started by this signal. If you go over this allotment, iOS will kill the app instead of pausing it.

NOTIFICATION_APPLICATION_FOCUS_IN = 2016 

Notification received from the OS when the application is focused, i.e. when changing the focus from the OS desktop or a thirdparty application to any open window of the Godot instance.

Implemented on desktop and mobile platforms.

NOTIFICATION_APPLICATION_FOCUS_OUT = 2017 

Notification received from the OS when the application is defocused, i.e. when changing the focus from any open window of the Godot instance to the OS desktop or a thirdparty application.

Implemented on desktop and mobile platforms.

NOTIFICATION_TEXT_SERVER_CHANGED = 2018 

Notification received when the TextServer is changed.

NOTIFICATION_APPLICATION_PIP_MODE_ENTERED = 2019 

Notification received when the application enters picture-in-picture mode.

NOTIFICATION_APPLICATION_PIP_MODE_EXITED = 2020 

Notification received when the application exits picture-in-picture mode.

NOTIFICATION_ACCESSIBILITY_UPDATE = 3000 

Notification received when an accessibility information update is required.

NOTIFICATION_ACCESSIBILITY_INVALIDATE = 3001 

Notification received when accessibility elements are invalidated. All node accessibility elements are automatically deleted after receiving this message, therefore all existing references to such elements should be discarded.

### NOTIFICATION_EXIT_TREE {#NOTIFICATION_EXIT_TREE}

```gdscript
const NOTIFICATION_EXIT_TREE = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_MOVED_IN_PARENT {#NOTIFICATION_MOVED_IN_PARENT}

```gdscript
const NOTIFICATION_MOVED_IN_PARENT = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_READY {#NOTIFICATION_READY}

```gdscript
const NOTIFICATION_READY = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_PAUSED {#NOTIFICATION_PAUSED}

```gdscript
const NOTIFICATION_PAUSED = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_UNPAUSED {#NOTIFICATION_UNPAUSED}

```gdscript
const NOTIFICATION_UNPAUSED = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_PHYSICS_PROCESS {#NOTIFICATION_PHYSICS_PROCESS}

```gdscript
const NOTIFICATION_PHYSICS_PROCESS = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_PROCESS {#NOTIFICATION_PROCESS}

```gdscript
const NOTIFICATION_PROCESS = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_PARENTED {#NOTIFICATION_PARENTED}

```gdscript
const NOTIFICATION_PARENTED = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_UNPARENTED {#NOTIFICATION_UNPARENTED}

```gdscript
const NOTIFICATION_UNPARENTED = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_SCENE_INSTANTIATED {#NOTIFICATION_SCENE_INSTANTIATED}

```gdscript
const NOTIFICATION_SCENE_INSTANTIATED = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_DRAG_BEGIN {#NOTIFICATION_DRAG_BEGIN}

```gdscript
const NOTIFICATION_DRAG_BEGIN = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_DRAG_END {#NOTIFICATION_DRAG_END}

```gdscript
const NOTIFICATION_DRAG_END = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_PATH_RENAMED {#NOTIFICATION_PATH_RENAMED}

```gdscript
const NOTIFICATION_PATH_RENAMED = 23
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_CHILD_ORDER_CHANGED {#NOTIFICATION_CHILD_ORDER_CHANGED}

```gdscript
const NOTIFICATION_CHILD_ORDER_CHANGED = 24
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_INTERNAL_PROCESS {#NOTIFICATION_INTERNAL_PROCESS}

```gdscript
const NOTIFICATION_INTERNAL_PROCESS = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_INTERNAL_PHYSICS_PROCESS {#NOTIFICATION_INTERNAL_PHYSICS_PROCESS}

```gdscript
const NOTIFICATION_INTERNAL_PHYSICS_PROCESS = 26
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_POST_ENTER_TREE {#NOTIFICATION_POST_ENTER_TREE}

```gdscript
const NOTIFICATION_POST_ENTER_TREE = 27
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_DISABLED {#NOTIFICATION_DISABLED}

```gdscript
const NOTIFICATION_DISABLED = 28
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_ENABLED {#NOTIFICATION_ENABLED}

```gdscript
const NOTIFICATION_ENABLED = 29
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_RESET_PHYSICS_INTERPOLATION {#NOTIFICATION_RESET_PHYSICS_INTERPOLATION}

```gdscript
const NOTIFICATION_RESET_PHYSICS_INTERPOLATION = 2001
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_EDITOR_PRE_SAVE {#NOTIFICATION_EDITOR_PRE_SAVE}

```gdscript
const NOTIFICATION_EDITOR_PRE_SAVE = 9001
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_EDITOR_POST_SAVE {#NOTIFICATION_EDITOR_POST_SAVE}

```gdscript
const NOTIFICATION_EDITOR_POST_SAVE = 9002
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_MOUSE_ENTER {#NOTIFICATION_WM_MOUSE_ENTER}

```gdscript
const NOTIFICATION_WM_MOUSE_ENTER = 1002
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_MOUSE_EXIT {#NOTIFICATION_WM_MOUSE_EXIT}

```gdscript
const NOTIFICATION_WM_MOUSE_EXIT = 1003
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_WINDOW_FOCUS_IN {#NOTIFICATION_WM_WINDOW_FOCUS_IN}

```gdscript
const NOTIFICATION_WM_WINDOW_FOCUS_IN = 1004
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_WINDOW_FOCUS_OUT {#NOTIFICATION_WM_WINDOW_FOCUS_OUT}

```gdscript
const NOTIFICATION_WM_WINDOW_FOCUS_OUT = 1005
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_CLOSE_REQUEST {#NOTIFICATION_WM_CLOSE_REQUEST}

```gdscript
const NOTIFICATION_WM_CLOSE_REQUEST = 1006
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_GO_BACK_REQUEST {#NOTIFICATION_WM_GO_BACK_REQUEST}

```gdscript
const NOTIFICATION_WM_GO_BACK_REQUEST = 1007
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_SIZE_CHANGED {#NOTIFICATION_WM_SIZE_CHANGED}

```gdscript
const NOTIFICATION_WM_SIZE_CHANGED = 1008
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_DPI_CHANGE {#NOTIFICATION_WM_DPI_CHANGE}

```gdscript
const NOTIFICATION_WM_DPI_CHANGE = 1009
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_VP_MOUSE_ENTER {#NOTIFICATION_VP_MOUSE_ENTER}

```gdscript
const NOTIFICATION_VP_MOUSE_ENTER = 1010
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_VP_MOUSE_EXIT {#NOTIFICATION_VP_MOUSE_EXIT}

```gdscript
const NOTIFICATION_VP_MOUSE_EXIT = 1011
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_POSITION_CHANGED {#NOTIFICATION_WM_POSITION_CHANGED}

```gdscript
const NOTIFICATION_WM_POSITION_CHANGED = 1012
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_OUTPUT_MAX_LINEAR_VALUE_CHANGED {#NOTIFICATION_WM_OUTPUT_MAX_LINEAR_VALUE_CHANGED}

```gdscript
const NOTIFICATION_WM_OUTPUT_MAX_LINEAR_VALUE_CHANGED = 1013
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_OS_MEMORY_WARNING {#NOTIFICATION_OS_MEMORY_WARNING}

```gdscript
const NOTIFICATION_OS_MEMORY_WARNING = 2009
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_TRANSLATION_CHANGED {#NOTIFICATION_TRANSLATION_CHANGED}

```gdscript
const NOTIFICATION_TRANSLATION_CHANGED = 2010
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_WM_ABOUT {#NOTIFICATION_WM_ABOUT}

```gdscript
const NOTIFICATION_WM_ABOUT = 2011
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_CRASH {#NOTIFICATION_CRASH}

```gdscript
const NOTIFICATION_CRASH = 2012
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_OS_IME_UPDATE {#NOTIFICATION_OS_IME_UPDATE}

```gdscript
const NOTIFICATION_OS_IME_UPDATE = 2013
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_APPLICATION_RESUMED {#NOTIFICATION_APPLICATION_RESUMED}

```gdscript
const NOTIFICATION_APPLICATION_RESUMED = 2014
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_APPLICATION_PAUSED {#NOTIFICATION_APPLICATION_PAUSED}

```gdscript
const NOTIFICATION_APPLICATION_PAUSED = 2015
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_APPLICATION_FOCUS_IN {#NOTIFICATION_APPLICATION_FOCUS_IN}

```gdscript
const NOTIFICATION_APPLICATION_FOCUS_IN = 2016
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_APPLICATION_FOCUS_OUT {#NOTIFICATION_APPLICATION_FOCUS_OUT}

```gdscript
const NOTIFICATION_APPLICATION_FOCUS_OUT = 2017
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_TEXT_SERVER_CHANGED {#NOTIFICATION_TEXT_SERVER_CHANGED}

```gdscript
const NOTIFICATION_TEXT_SERVER_CHANGED = 2018
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_APPLICATION_PIP_MODE_ENTERED {#NOTIFICATION_APPLICATION_PIP_MODE_ENTERED}

```gdscript
const NOTIFICATION_APPLICATION_PIP_MODE_ENTERED = 2019
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_APPLICATION_PIP_MODE_EXITED {#NOTIFICATION_APPLICATION_PIP_MODE_EXITED}

```gdscript
const NOTIFICATION_APPLICATION_PIP_MODE_EXITED = 2020
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_ACCESSIBILITY_UPDATE {#NOTIFICATION_ACCESSIBILITY_UPDATE}

```gdscript
const NOTIFICATION_ACCESSIBILITY_UPDATE = 3000
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_ACCESSIBILITY_INVALIDATE {#NOTIFICATION_ACCESSIBILITY_INVALIDATE}

```gdscript
const NOTIFICATION_ACCESSIBILITY_INVALIDATE = 3001
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
