# Object

> An advanced Variant type. All classes in the engine inherit from Object. Each class may define new properties, methods or signals, which are available to all inheriting classes. For example, a Sprite2D instance is able to call Node.add_child() because it inherits from Node.

<details><summary>官方英文描述（中文精译进行中）</summary>

An advanced Variant type. All classes in the engine inherit from Object. Each class may define new properties, methods or signals, which are available to all inheriting classes. For example, a Sprite2D instance is able to call Node.add_child() because it inherits from Node.

You can create new instances, using Object.new() in GDScript, or new GodotObject in C#.

To delete an Object instance, call free(). This is necessary for most classes inheriting Object, because they do not manage memory on their own, and will otherwise cause memory leaks when no longer in use. There are a few classes that perform memory management. For example, RefCounted (and by extension Resource) deletes itself when no longer referenced, and Node deletes its children when freed.

Objects can have a Script attached to them. Once the Script is instantiated, it effectively acts as an extension to the base class, allowing it to define and inherit new properties, methods and signals.

Inside a Script, _get_property_list() may be overridden to customize properties in several ways. This allows them to be available to the editor, display as lists of options, sub-divide into groups, save on disk, etc. Scripting languages offer easier ways to customize properties, such as with the @GDScript.@export annotation.

Godot is very dynamic. An object's script, and therefore its properties, methods and signals, can be changed at run-time. Because of this, there can be occasions where, for example, a property required by a method may not exist. To prevent run-time errors, see methods such as set(), get(), call(), has_method(), has_signal(), etc. Note that these methods are much slower than direct references.

In GDScript, you can also check if a given property, method, or signal name exists in an object with the in operator:

```gdscript
var node = Node.new()
print("name" in node)         # Prints true
print("get_parent" in node)   # Prints true
print("tree_entered" in node) # Prints true
print("unknown" in node)      # Prints false
```

Notifications are int constants commonly sent and received by objects. For example, on every rendered frame, the SceneTree notifies nodes inside the tree with a Node.NOTIFICATION_PROCESS. The nodes receive it and may call Node._process() to update. To make use of notifications, see notification() and _notification().

Lastly, every object can also contain metadata (data about data). set_meta() can be useful to store information that the object itself does not depend on. To keep your code clean, making excessive use of metadata is discouraged.

Note: Unlike references to a RefCounted, references to an object stored in a variable can become invalid without being set to null. To check if an object has been deleted, do not compare it against null. Instead, use @GlobalScope.is_instance_valid(). It's also recommended to inherit from RefCounted for classes storing data instead of Object.

Note: The script is not exposed like most properties. To set or get an object's Script in code, use set_script() and get_script(), respectively.

Note: In a boolean context, an Object will evaluate to false if it is equal to null or it has been freed. Otherwise, an Object will always evaluate to true. See also @GlobalScope.is_instance_valid().

</details>

## 方法

### add_user_signal {#add_user_signal}

```gdscript
void add_user_signal(signal: String, arguments: Array = [])
```

Adds a user-defined signal named signal. Optional arguments for the signal can be added as an Array of dictionaries, each defining a name String and a type int (see Variant.Type). See also has_user_signal() and remove_user_signal().

```
add_user_signal("hurt", [
    { "name": "damage", "type": TYPE_INT },
    { "name": "source", "type": TYPE_OBJECT }
])
```

```
AddUserSignal("Hurt",
[
    new Godot.Collections.Dictionary()
    {
        { "name", "damage" },
        { "type", (int)Variant.Type.Int },
    },
    new Godot.Collections.Dictionary()
    {
        { "name", "source" },
        { "type", (int)Variant.Type.Object },
    },
]);
```

### call {#call}

```gdscript
Variant call(method: StringName, ...)
```

Calls the method on the object and returns the result. This method supports a variable number of arguments, so parameters can be passed as a comma separated list.

```
var node = Node3D.new()
node.call("rotate", Vector3(1.0, 0.0, 0.0), 1.571)
```

```
var node = new Node3D();
node.Call(Node3D.MethodName.Rotate, new Vector3(1f, 0f, 0f), 1.571f);
```

Note: In C#, method must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the MethodName class to avoid allocating a new StringName on each call.

### call_deferred {#call_deferred}

```gdscript
Variant call_deferred(method: StringName, ...)
```

Calls the method on the object during idle time. Always returns null, not the method's result.

Idle time happens mainly at the end of process and physics frames. In it, deferred calls will be run until there are none left, which means you can defer calls from other deferred calls and they'll still be run in the current idle time cycle. This means you should not call a method deferred from itself (or from a method called by it), as this causes infinite recursion the same way as if you had called the method directly.

This method supports a variable number of arguments, so parameters can be passed as a comma separated list.

```
var node = Node3D.new()
node.call_deferred("rotate", Vector3(1.0, 0.0, 0.0), 1.571)
```

```
var node = new Node3D();
node.CallDeferred(Node3D.MethodName.Rotate, new Vector3(1f, 0f, 0f), 1.571f);
```

For methods that are deferred from the same thread, the order of execution at idle time is identical to the order in which call_deferred was called.

See also Callable.call_deferred().

Note: In C#, method must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the MethodName class to avoid allocating a new StringName on each call.

Note: If you're looking to delay the function call by a frame, refer to the SceneTree.process_frame and SceneTree.physics_frame signals.

```gdscript
var node = Node3D.new()
# Make a Callable and bind the arguments to the node's rotate() call.
var callable = node.rotate.bind(Vector3(1.0, 0.0, 0.0), 1.571)
# Connect the callable to the process_frame signal, so it gets called in the next process frame.
# CONNECT_ONE_SHOT makes sure it only gets called once instead of every frame.
get_tree().process_frame.connect(callable, CONNECT_ONE_SHOT)
```

### callv {#callv}

```gdscript
Variant callv(method: StringName, arg_array: Array)
```

Calls the method on the object and returns the result. Unlike call(), this method expects all parameters to be contained inside arg_array.

```
var node = Node3D.new()
node.callv("rotate", [Vector3(1.0, 0.0, 0.0), 1.571])
```

```
var node = new Node3D();
node.Callv(Node3D.MethodName.Rotate, [new Vector3(1f, 0f, 0f), 1.571f]);
```

Note: In C#, method must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the MethodName class to avoid allocating a new StringName on each call.

### can_translate_messages {#can_translate_messages}

```gdscript
bool can_translate_messages() const
```

Returns true if the object is allowed to translate messages with tr() and tr_n(). See also set_message_translation().

### cancel_free {#cancel_free}

```gdscript
void cancel_free()
```

If this method is called during NOTIFICATION_PREDELETE, this object will reject being freed and will remain allocated. This is mostly an internal function used for error handling to avoid the user from freeing objects when they are not intended to.

### connect {#connect}

```gdscript
Error connect(signal: StringName, callable: Callable, flags: int = 0)
```

Connects a signal by name to a callable. Optional flags can be also added to configure the connection's behavior (see ConnectFlags constants).

A signal can only be connected once to the same Callable. If the signal is already connected, this method returns @GlobalScope.ERR_INVALID_PARAMETER and generates an error, unless the signal is connected with CONNECT_REFERENCE_COUNTED. To prevent this, use is_connected() first to check for existing connections.

Note: If the callable's object is freed, the connection will be lost.

Note: In GDScript, it is generally recommended to connect signals with Signal.connect() instead.

Note: This method, and all other signal-related methods, are thread-safe.

### disconnect {#disconnect}

```gdscript
void disconnect(signal: StringName, callable: Callable)
```

Disconnects a signal by name from a given callable. If the connection does not exist, generates an error. Use is_connected() to make sure that the connection exists.

### emit_signal {#emit_signal}

```gdscript
Error emit_signal(signal: StringName, ...)
```

Emits the given signal by name. The signal must exist, so it should be a built-in signal of this class or one of its inherited classes, or a user-defined signal (see add_user_signal()). This method supports a variable number of arguments, so parameters can be passed as a comma separated list.

Returns @GlobalScope.ERR_UNAVAILABLE if signal does not exist or the parameters are invalid.

```
emit_signal("hit", "sword", 100)
emit_signal("game_over")
```

```
EmitSignal(SignalName.Hit, "sword", 100);
EmitSignal(SignalName.GameOver);
```

Note: In C#, signal must be in snake_case when referring to built-in Godot signals. Prefer using the names exposed in the SignalName class to avoid allocating a new StringName on each call.

### get {#get}

```gdscript
Variant get(property: StringName) const
```

Returns the Variant value of the given property. If the property does not exist, this method returns null.

```
var node = Node2D.new()
node.rotation = 1.5
var a = node.get("rotation") # a is 1.5
```

```
var node = new Node2D();
node.Rotation = 1.5f;
var a = node.Get(Node2D.PropertyName.Rotation); // a is 1.5
```

Note: In C#, property must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the PropertyName class to avoid allocating a new StringName on each call.

### get_class {#get_class}

```gdscript
String get_class() const
```

Returns the object's built-in class name, as a String. See also is_class().

Note: This method ignores class_name declarations. If this object's script has defined a class_name, the base, built-in class name is returned instead.

### get_incoming_connections {#get_incoming_connections}

```gdscript
typedarray::Dictionary get_incoming_connections() const
```

Returns an Array of signal connections received by this object. Each connection is represented as a Dictionary that contains three entries:

- signal is a reference to the Signal;
- callable is a reference to the Callable;
- flags is a combination of ConnectFlags.

### get_indexed {#get_indexed}

```gdscript
Variant get_indexed(property_path: NodePath) const
```

Gets the object's property indexed by the given property_path. The path should be a NodePath relative to the current object and can use the colon character (:) to access nested properties.

Examples: "position:x" or "material:next_pass:blend_mode".

```
var node = Node2D.new()
node.position = Vector2(5, -10)
var a = node.get_indexed("position")   # a is Vector2(5, -10)
var b = node.get_indexed("position:y") # b is -10
```

```
var node = new Node2D();
node.Position = new Vector2(5, -10);
var a = node.GetIndexed("position");   // a is Vector2(5, -10)
var b = node.GetIndexed("position:y"); // b is -10
```

Note: In C#, property_path must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the PropertyName class to avoid allocating a new StringName on each call.

Note: This method does not support actual paths to nodes in the SceneTree, only sub-property paths. In the context of nodes, use Node.get_node_and_resource() instead.

### get_instance_id {#get_instance_id}

```gdscript
int get_instance_id() const
```

Returns the object's unique instance ID. This ID can be saved in EncodedObjectAsID, and can be used to retrieve this object instance with @GlobalScope.instance_from_id().

Note: This ID is only useful during the current session. It won't correspond to a similar object if the ID is sent over a network, or loaded from a file at a later time.

### get_meta {#get_meta}

```gdscript
Variant get_meta(name: StringName, default: Variant = null) const
```

Returns the object's metadata value for the given entry name. If the entry does not exist, returns default. If default is null, an error is also generated.

Note: A metadata's name must be a valid identifier as per StringName.is_valid_identifier() method.

Note: Metadata that has a name starting with an underscore (_) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method.

### get_meta_list {#get_meta_list}

```gdscript
typedarray::StringName get_meta_list() const
```

Returns the object's metadata entry names as an Array of StringNames.

### get_method_argument_count {#get_method_argument_count}

```gdscript
int get_method_argument_count(method: StringName) const
```

Returns the number of arguments of the given method by name.

Note: In C#, method must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the MethodName class to avoid allocating a new StringName on each call.

### get_method_list {#get_method_list}

```gdscript
typedarray::Dictionary get_method_list() const
```

Returns this object's methods and their signatures as an Array of dictionaries. Each Dictionary contains the following entries:

- name is the name of the method, as a String;
- args is an Array of dictionaries representing the arguments;
- default_args is the default arguments as an Array of variants;
- flags is a combination of MethodFlags;
- id is the method's internal identifier int;
- return is the returned value, as a Dictionary;

Note: The dictionaries of args and return are formatted identically to the results of get_property_list(), although not all entries are used.

### get_property_list {#get_property_list}

```gdscript
typedarray::Dictionary get_property_list() const
```

Returns the object's property list as an Array of dictionaries. Each Dictionary contains the following entries:

- name is the property's name, as a String;
- class_name is an empty StringName, unless the property is @GlobalScope.TYPE_OBJECT and it inherits from a class;
- type is the property's type, as an int (see Variant.Type);
- hint is how the property is meant to be edited (see PropertyHint);
- hint_string depends on the hint (see PropertyHint);
- usage is a combination of PropertyUsageFlags.

Note: In GDScript, all class members are treated as properties. In C# and GDExtension, it may be necessary to explicitly mark class members as Godot properties using decorators or attributes.

### get_script {#get_script}

```gdscript
Variant get_script() const
```

Returns the object's Script instance, or null if no script is attached.

### get_signal_connection_list {#get_signal_connection_list}

```gdscript
typedarray::Dictionary get_signal_connection_list(signal: StringName) const
```

Returns an Array of connections for the given signal name. Each connection is represented as a Dictionary that contains three entries:

- signal is a reference to the Signal;
- callable is a reference to the connected Callable;
- flags is a combination of ConnectFlags.

### get_signal_list {#get_signal_list}

```gdscript
typedarray::Dictionary get_signal_list() const
```

Returns the list of existing signals as an Array of dictionaries.

Note: Due to the implementation, each Dictionary is formatted very similarly to the returned values of get_method_list().

### get_translation_domain {#get_translation_domain}

```gdscript
StringName get_translation_domain() const
```

Returns the name of the translation domain used by tr() and tr_n(). See also TranslationServer.

### has_connections {#has_connections}

```gdscript
bool has_connections(signal: StringName) const
```

Returns true if any connection exists on the given signal name.

Note: In C#, signal must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the SignalName class to avoid allocating a new StringName on each call.

### has_meta {#has_meta}

```gdscript
bool has_meta(name: StringName) const
```

Returns true if a metadata entry is found with the given name. See also get_meta(), set_meta() and remove_meta().

Note: A metadata's name must be a valid identifier as per StringName.is_valid_identifier() method.

Note: Metadata that has a name starting with an underscore (_) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method.

### has_method {#has_method}

```gdscript
bool has_method(method: StringName) const
```

Returns true if the given method name exists in the object.

Note: In C#, method must be in snake_case when referring to built-in Godot methods. Prefer using the names exposed in the MethodName class to avoid allocating a new StringName on each call.

### has_signal {#has_signal}

```gdscript
bool has_signal(signal: StringName) const
```

Returns true if the given signal name exists in the object.

Note: In C#, signal must be in snake_case when referring to built-in Godot signals. Prefer using the names exposed in the SignalName class to avoid allocating a new StringName on each call.

### has_user_signal {#has_user_signal}

```gdscript
bool has_user_signal(signal: StringName) const
```

Returns true if the given user-defined signal name exists. Only signals added with add_user_signal() are included. See also remove_user_signal().

### is_blocking_signals {#is_blocking_signals}

```gdscript
bool is_blocking_signals() const
```

Returns true if the object is blocking its signals from being emitted. See set_block_signals().

### is_class {#is_class}

```gdscript
bool is_class(class: StringName) const
```

Returns true if the object inherits from the given class. See also get_class().

```
var sprite2d = Sprite2D.new()
sprite2d.is_class("Sprite2D") # Returns true
sprite2d.is_class("Node")     # Returns true
sprite2d.is_class("Node3D")   # Returns false
```

```
var sprite2D = new Sprite2D();
sprite2D.IsClass("Sprite2D"); // Returns true
sprite2D.IsClass("Node");     // Returns true
sprite2D.IsClass("Node3D");   // Returns false
```

Note: This method ignores class_name declarations in the object's script.

### is_connected {#is_connected}

```gdscript
bool is_connected(signal: StringName, callable: Callable) const
```

Returns true if a connection exists between the given signal name and callable.

Note: In C#, signal must be in snake_case when referring to built-in Godot signals. Prefer using the names exposed in the SignalName class to avoid allocating a new StringName on each call.

### is_queued_for_deletion {#is_queued_for_deletion}

```gdscript
bool is_queued_for_deletion() const
```

Returns true if the methods Node.queue_free() or SceneTree.queue_delete() was called for the object.

Note: This method does not return true on children of the node that Node.queue_free() has been called on, even though they will be freed together with the parent.

### notification {#notification}

```gdscript
void notification(what: int, reversed: bool = false)
```

Sends the given what notification to all classes inherited by the object, triggering calls to _notification(), starting from the highest ancestor (the Object class) and going down to the object's script.

If reversed is true, the call order is reversed.

```
var player = Node2D.new()
player.set_script(load("res://player.gd"))
player.notification(NOTIFICATION_ENTER_TREE)
# The call order is Object -> Node -> Node2D -> player.gd.
player.notification(NOTIFICATION_ENTER_TREE, true)
# The call order is player.gd -> Node2D -> Node -> Object.
```

```
var player = new Node2D();
player.SetScript(GD.Load("res://player.gd"));
player.Notification(NotificationEnterTree);
// The call order is GodotObject -> Node -> Node2D -> player.gd.
player.Notification(NotificationEnterTree, true);
// The call order is player.gd -> Node2D -> Node -> GodotObject.
```

### notify_property_list_changed {#notify_property_list_changed}

```gdscript
void notify_property_list_changed()
```

Emits the property_list_changed signal. This is mainly used to refresh the editor, so that the Inspector and editor plugins are properly updated.

### property_can_revert {#property_can_revert}

```gdscript
bool property_can_revert(property: StringName) const
```

Returns true if the given property has a custom default value. Use property_get_revert() to get the property's default value.

Note: This method is used by the Inspector dock to display a revert icon. The object must implement _property_can_revert() to customize the default value. If _property_can_revert() is not implemented, this method returns false.

### property_get_revert {#property_get_revert}

```gdscript
Variant property_get_revert(property: StringName) const
```

Returns the custom default value of the given property. Use property_can_revert() to check if the property has a custom default value.

Note: This method is used by the Inspector dock to display a revert icon. The object must implement _property_get_revert() to customize the default value. If _property_get_revert() is not implemented, this method returns null.

### remove_meta {#remove_meta}

```gdscript
void remove_meta(name: StringName)
```

Removes the given entry name from the object's metadata. See also has_meta(), get_meta() and set_meta().

Note: A metadata's name must be a valid identifier as per StringName.is_valid_identifier() method.

Note: Metadata that has a name starting with an underscore (_) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method.

### remove_user_signal {#remove_user_signal}

```gdscript
void remove_user_signal(signal: StringName)
```

Removes the given user signal signal from the object. See also add_user_signal() and has_user_signal().

### set {#set}

```gdscript
void set(property: StringName, value: Variant)
```

Assigns value to the given property. If the property does not exist or the given value's type doesn't match, nothing happens.

```
var node = Node2D.new()
node.set("global_scale", Vector2(8, 2.5))
print(node.global_scale) # Prints (8.0, 2.5)
```

```
var node = new Node2D();
node.Set(Node2D.PropertyName.GlobalScale, new Vector2(8, 2.5f));
GD.Print(node.GlobalScale); // Prints (8, 2.5)
```

Note: In C#, property must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the PropertyName class to avoid allocating a new StringName on each call.

### set_block_signals {#set_block_signals}

```gdscript
void set_block_signals(enable: bool)
```

If set to true, the object becomes unable to emit signals. As such, emit_signal() and signal connections will not work, until it is set to false.

### set_deferred {#set_deferred}

```gdscript
void set_deferred(property: StringName, value: Variant)
```

Assigns value to the given property, at the end of the current frame. This is equivalent to calling set() through call_deferred().

```
var node = Node2D.new()
add_child(node)
node.rotation = 1.5
node.set_deferred("rotation", 3.0)
print(node.rotation) # Prints 1.5
await get_tree().process_frame
print(node.rotation) # Prints 3.0
```

```
var node = new Node2D();
node.Rotation = 1.5f;
node.SetDeferred(Node2D.PropertyName.Rotation, 3f);
GD.Print(node.Rotation); // Prints 1.5
await ToSignal(GetTree(), SceneTree.SignalName.ProcessFrame);
GD.Print(node.Rotation); // Prints 3.0
```

Note: In C#, property must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the PropertyName class to avoid allocating a new StringName on each call.

### set_indexed {#set_indexed}

```gdscript
void set_indexed(property_path: NodePath, value: Variant)
```

Assigns a new value to the property identified by the property_path. The path should be a NodePath relative to this object, and can use the colon character (:) to access nested properties.

```
var node = Node2D.new()
node.set_indexed("position", Vector2(42, 0))
node.set_indexed("position:y", -10)
print(node.position) # Prints (42.0, -10.0)
```

```
var node = new Node2D();
node.SetIndexed("position", new Vector2(42, 0));
node.SetIndexed("position:y", -10);
GD.Print(node.Position); // Prints (42, -10)
```

Note: In C#, property_path must be in snake_case when referring to built-in Godot properties. Prefer using the names exposed in the PropertyName class to avoid allocating a new StringName on each call.

### set_message_translation {#set_message_translation}

```gdscript
void set_message_translation(enable: bool)
```

If set to true, allows the object to translate messages with tr() and tr_n(). Enabled by default. See also can_translate_messages().

### set_meta {#set_meta}

```gdscript
void set_meta(name: StringName, value: Variant)
```

Adds or changes the entry name inside the object's metadata. The metadata value can be any Variant, although some types cannot be serialized correctly.

If value is null, the entry is removed. This is the equivalent of using remove_meta(). See also has_meta() and get_meta().

Note: A metadata's name must be a valid identifier as per StringName.is_valid_identifier() method.

Note: Metadata that has a name starting with an underscore (_) is considered editor-only. Editor-only metadata is not displayed in the Inspector and should not be edited, although it can still be found by this method.

### set_script {#set_script}

```gdscript
void set_script(script: Variant)
```

Attaches script to the object, and instantiates it. As a result, the script's _init() is called. A Script is used to extend the object's functionality.

If a script already exists, its instance is detached, and its property values and state are lost. Built-in property values are still kept.

### set_translation_domain {#set_translation_domain}

```gdscript
void set_translation_domain(domain: StringName)
```

Sets the name of the translation domain used by tr() and tr_n(). See also TranslationServer.

### to_string {#to_string}

```gdscript
String to_string()
```

Returns a String representing the object. Defaults to "". Override _to_string() to customize the string representation of the object.

### tr {#tr}

```gdscript
String tr(message: StringName, context: StringName = &"") const
```

Translates a message, using the translation catalogs configured in the Project Settings. Further context can be specified to help with the translation. Note that most Control nodes automatically translate their strings, so this method is mostly useful for formatted strings or custom drawn text.

If can_translate_messages() is false, or no translation is available, this method returns the message without changes. See set_message_translation().

For detailed examples, see Internationalizing games.

Note: This method can't be used without an Object instance, as it requires the can_translate_messages() method. To translate strings in a static context, use TranslationServer.translate().

### tr_n {#tr_n}

```gdscript
String tr_n(message: StringName, plural_message: StringName, n: int, context: StringName = &"") const
```

Translates a message or plural_message, using the translation catalogs configured in the Project Settings. Further context can be specified to help with the translation.

If can_translate_messages() is false, or no translation is available, this method returns message or plural_message, without changes. See set_message_translation().

The n is the number, or amount, of the message's subject. It is used by the translation system to fetch the correct plural form for the current language.

For detailed examples, see Localization using gettext.

Note: Negative and float numbers may not properly apply to some countable subjects. It's recommended to handle these cases with tr().

Note: This method can't be used without an Object instance, as it requires the can_translate_messages() method. To translate strings in a static context, use TranslationServer.translate_plural().

## 信号

### script_changed {#script_changed}

```gdscript
signal script_changed()
```

Emitted when the object's script is changed.

Note: When this signal is emitted, the new script is not initialized yet. If you need to access the new script, defer connections to this signal with CONNECT_DEFERRED.

### property_list_changed {#property_list_changed}

```gdscript
signal property_list_changed()
```

Emitted when notify_property_list_changed() is called.

## 枚举

### ConnectFlags {#enum-ConnectFlags}

```gdscript
enum ConnectFlags（位标志） { CONNECT_DEFERRED = 1, CONNECT_PERSIST = 2, CONNECT_ONE_SHOT = 4, CONNECT_REFERENCE_COUNTED = 8, CONNECT_APPEND_SOURCE_OBJECT = 16 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONNECT_DEFERRED {#CONNECT_DEFERRED}

```gdscript
CONNECT_DEFERRED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONNECT_PERSIST {#CONNECT_PERSIST}

```gdscript
CONNECT_PERSIST = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONNECT_ONE_SHOT {#CONNECT_ONE_SHOT}

```gdscript
CONNECT_ONE_SHOT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONNECT_REFERENCE_COUNTED {#CONNECT_REFERENCE_COUNTED}

```gdscript
CONNECT_REFERENCE_COUNTED = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONNECT_APPEND_SOURCE_OBJECT {#CONNECT_APPEND_SOURCE_OBJECT}

```gdscript
CONNECT_APPEND_SOURCE_OBJECT = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 常量

### NOTIFICATION_POSTINITIALIZE {#NOTIFICATION_POSTINITIALIZE}

```gdscript
const NOTIFICATION_POSTINITIALIZE = 0
```

Notification received when the object is initialized, before its script is attached. Used internally.

NOTIFICATION_PREDELETE = 1 

Notification received when the object is about to be deleted. Can be used like destructors in object-oriented programming languages.

This notification is sent in reversed order.

NOTIFICATION_EXTENSION_RELOADED = 2 

Notification received when the object finishes hot reloading. This notification is only sent for extensions classes and derived.

### NOTIFICATION_PREDELETE {#NOTIFICATION_PREDELETE}

```gdscript
const NOTIFICATION_PREDELETE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### NOTIFICATION_EXTENSION_RELOADED {#NOTIFICATION_EXTENSION_RELOADED}

```gdscript
const NOTIFICATION_EXTENSION_RELOADED = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
