# ResourcePreloader

**继承**：[Node](class_node.md) → [Object](class_object.md) → **ResourcePreloader**

> This node is used to preload sub-resources inside a scene, so when the scene is loaded, all the resources are ready to use and can be retrieved from the preloader. You can add the resources using the ResourcePreloader tab when the node is selected.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node is used to preload sub-resources inside a scene, so when the scene is loaded, all the resources are ready to use and can be retrieved from the preloader. You can add the resources using the ResourcePreloader tab when the node is selected.

GDScript has a simplified @GDScript.preload() built-in method which can be used in most situations, leaving the use of ResourcePreloader for more advanced scenarios.

</details>

## 方法

### add_resource {#add_resource}

```gdscript
void add_resource(name: StringName, resource: Resource)
```

Adds a resource to the preloader with the given name. If a resource with the given name already exists, the new resource will be renamed to "name N" where N is an incrementing number starting from 2.

### get_resource {#get_resource}

```gdscript
Resource get_resource(name: StringName) const
```

Returns the resource associated to name.

### get_resource_list {#get_resource_list}

```gdscript
PackedStringArray get_resource_list() const
```

Returns the list of resources inside the preloader.

### has_resource {#has_resource}

```gdscript
bool has_resource(name: StringName) const
```

Returns true if the preloader contains a resource associated to name.

### remove_resource {#remove_resource}

```gdscript
void remove_resource(name: StringName)
```

Removes the resource associated to name from the preloader.

### rename_resource {#rename_resource}

```gdscript
void rename_resource(name: StringName, newname: StringName)
```

Renames a resource inside the preloader from name to newname.

## 属性

### resources {#resources}

```gdscript
resources: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
