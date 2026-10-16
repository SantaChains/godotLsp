# EditorResourceConversionPlugin

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorResourceConversionPlugin**

> EditorResourceConversionPlugin is invoked when the context menu is brought up for a resource in the editor inspector. Relevant conversion plugins will appear as menu options to convert the given resource to a target type.

<details><summary>官方英文描述（中文精译进行中）</summary>

EditorResourceConversionPlugin is invoked when the context menu is brought up for a resource in the editor inspector. Relevant conversion plugins will appear as menu options to convert the given resource to a target type.

Below shows an example of a basic plugin that will convert an ImageTexture to a PortableCompressedTexture2D.

```
extends EditorResourceConversionPlugin
func _handles(resource: Resource):
    return resource is ImageTexture
func _converts_to():
    return "PortableCompressedTexture2D"
func _convert(itex: Resource):
    var ptex = PortableCompressedTexture2D.new()
    ptex.create_from_image(itex.get_image(), PortableCompressedTexture2D.COMPRESSION_MODE_LOSSLESS)
    return ptex
```

To use an EditorResourceConversionPlugin, register it using the EditorPlugin.add_resource_conversion_plugin() method first.

</details>

## 方法

### _convert {#_convert}

```gdscript
Resource _convert(resource: Resource) virtual const
```

Takes an input Resource and converts it to the type given in _converts_to(). The returned Resource is the result of the conversion, and the input Resource remains unchanged.

### _converts_to {#_converts_to}

```gdscript
String _converts_to() virtual const
```

Returns the class name of the target type of Resource that this plugin converts source resources to.

### _handles {#_handles}

```gdscript
bool _handles(resource: Resource) virtual const
```

Called to determine whether a particular Resource can be converted to the target resource type by this plugin.
