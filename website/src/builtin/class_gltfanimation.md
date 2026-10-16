# GLTFAnimation

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFAnimation**

## 方法

### get_additional_data {#get_additional_data}

```gdscript
Variant get_additional_data(extension_name: StringName)
```

Gets additional arbitrary data in this GLTFAnimation instance. This can be used to keep per-node state data in GLTFDocumentExtension classes, which is important because they are stateless.

The argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the return value can be anything you set. If nothing was set, the return value is null.

### get_loop {#get_loop}

```gdscript
bool get_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_original_name {#get_original_name}

```gdscript
String get_original_name()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_additional_data {#set_additional_data}

```gdscript
void set_additional_data(extension_name: StringName, additional_data: Variant)
```

Sets additional arbitrary data in this GLTFAnimation instance. This can be used to keep per-node state data in GLTFDocumentExtension classes, which is important because they are stateless.

The first argument should be the GLTFDocumentExtension name (does not have to match the extension name in the glTF file), and the second argument can be anything you want.

### set_loop {#set_loop}

```gdscript
void set_loop(loop: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_original_name {#set_original_name}

```gdscript
void set_original_name(original_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### original_name {#original_name}

```gdscript
original_name: String
```

- void set_original_name(value: String)
- String get_original_name()

The original name of the animation.

### loop {#loop}

```gdscript
loop: bool
```

- void set_loop(value: bool)
- bool get_loop()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!
