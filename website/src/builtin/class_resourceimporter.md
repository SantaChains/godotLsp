# ResourceImporter

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **ResourceImporter**

> This is the base class for Godot's resource importers. To implement your own resource importers using editor plugins, see EditorImportPlugin.

<details><summary>官方英文描述（中文精译进行中）</summary>

This is the base class for Godot's resource importers. To implement your own resource importers using editor plugins, see EditorImportPlugin.

</details>

## 方法

### _get_build_dependencies {#_get_build_dependencies}

```gdscript
PackedStringArray _get_build_dependencies(path: String) virtual const
```

Called when the engine compilation profile editor wants to check what build options an imported resource needs. For example, ResourceImporterDynamicFont has a property called ResourceImporterDynamicFont.multichannel_signed_distance_field, that depends on the engine to be build with the "msdfgen" module. If that resource happened to be a custom one, it would be handled like this:

```gdscript
func _get_build_dependencies(path):
    var resource = load(path)
    var dependencies = PackedStringArray()
    if resource.multichannel_signed_distance_field:
        dependencies.push_back("module_msdfgen_enabled")
    return dependencies
```

## 枚举

### ImportOrder {#enum-ImportOrder}

```gdscript
enum ImportOrder { IMPORT_ORDER_DEFAULT = 0, IMPORT_ORDER_SCENE = 100 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_ORDER_DEFAULT {#IMPORT_ORDER_DEFAULT}

```gdscript
IMPORT_ORDER_DEFAULT = 0
```

The default import order.

#### IMPORT_ORDER_SCENE {#IMPORT_ORDER_SCENE}

```gdscript
IMPORT_ORDER_SCENE = 100
```

The import order for scenes, which ensures scenes are imported after all other core resources such as textures. Custom importers should generally have an import order lower than 100 to avoid issues when importing scenes that rely on custom resources.

---
