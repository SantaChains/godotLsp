# RID

> The RID Variant type is used to access a low-level resource by its unique ID. RIDs are opaque, which means they do not grant access to the resource by themselves. They are used by the low-level server classes, such as DisplayServer, RenderingServer, TextServer, etc.

<details><summary>官方英文描述（中文精译进行中）</summary>

The RID Variant type is used to access a low-level resource by its unique ID. RIDs are opaque, which means they do not grant access to the resource by themselves. They are used by the low-level server classes, such as DisplayServer, RenderingServer, TextServer, etc.

A low-level resource may correspond to a high-level Resource, such as Texture or Mesh.

Note: RIDs are only useful during the current session. It won't correspond to a similar resource if sent over a network, or loaded from a file at a later time.

Note: In a boolean context, an RID will evaluate to false if it has the invalid ID 0. Otherwise, an RID will always evaluate to true. This is equivalent to calling is_valid().

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 方法

### get_id {#get_id}

```gdscript
int get_id() const
```

Returns the ID of the referenced low-level resource.

### is_valid {#is_valid}

```gdscript
bool is_valid() const
```

Returns true if the RID is not 0.
