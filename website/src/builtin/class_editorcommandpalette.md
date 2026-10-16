# EditorCommandPalette

**继承**：[ConfirmationDialog](class_confirmationdialog.md) → [AcceptDialog](class_acceptdialog.md) → [Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorCommandPalette**

> Object that holds all the available Commands and their shortcuts text. These Commands can be accessed through Editor > Command Palette menu.

<details><summary>官方英文描述（中文精译进行中）</summary>

Object that holds all the available Commands and their shortcuts text. These Commands can be accessed through Editor > Command Palette menu.

Command key names use slash delimiters to distinguish sections, for example: "example/command1" then example will be the section name.

```
var command_palette = EditorInterface.get_command_palette()
# external_command is a function that will be called with the command is executed.
var command_callable = Callable(self, "external_command").bind(arguments)
command_palette.add_command("command", "test/command",command_callable)
```

```
EditorCommandPalette commandPalette = EditorInterface.Singleton.GetCommandPalette();
// ExternalCommand is a function that will be called with the command is executed.
Callable commandCallable = new Callable(this, MethodName.ExternalCommand);
commandPalette.AddCommand("command", "test/command", commandCallable)
```

Note: This class shouldn't be instantiated directly. Instead, access the singleton using EditorInterface.get_command_palette().

</details>

## 方法

### add_command {#add_command}

```gdscript
void add_command(command_name: String, key_name: String, binded_callable: Callable, shortcut_text: String = "None")
```

Adds a custom command to EditorCommandPalette.

- command_name: String (Name of the Command. This is displayed to the user.)
- key_name: String (Name of the key for a particular Command. This is used to uniquely identify the Command.)
- binded_callable: Callable (Callable of the Command. This will be executed when the Command is selected.)
- shortcut_text: String (Shortcut text of the Command if available.)

### remove_command {#remove_command}

```gdscript
void remove_command(key_name: String)
```

Removes the custom command from EditorCommandPalette.

- key_name: String (Name of the key for a particular Command.)
