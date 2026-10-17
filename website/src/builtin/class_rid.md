# RID

> The RID Variant type is used to access a low-level resource by its unique ID. RIDs are opaque, which means they do not grant access to the resource by themselves. They are used by the low-level server classes, such as DisplayServer, RenderingServer, TextServer, etc.

<details><summary>官方英文描述</summary>

The RID Variant type is used to access a low-level resource by its unique ID. RIDs are opaque, which means they do not grant access to the resource by themselves. They are used by the low-level server classes, such as DisplayServer, RenderingServer, TextServer, etc.

A low-level resource may correspond to a high-level Resource, such as Texture or Mesh.

Note: RIDs are only useful during the current session. It won't correspond to a similar resource if sent over a network, or loaded from a file at a later time.

Note: In a boolean context, an RID will evaluate to false if it has the invalid ID 0. Otherwise, an RID will always evaluate to true. This is equivalent to calling is_valid().

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 构造函数

```gdscript
RID()
```

Constructs an empty RID with the invalid ID 0.

```gdscript
RID(from: RID)
```

Constructs an RID as a copy of the given RID.

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

## 运算符

```gdscript
operator !=(right: RID)
```

Returns true if the RIDs are not equal.

```gdscript
operator <(right: RID)
```

Returns true if the RID's ID is less than right's ID.

```gdscript
operator <=(right: RID)
```

Returns true if the RID's ID is less than or equal to right's ID.

```gdscript
operator ==(right: RID)
```

Returns true if both RIDs are equal, which means they both refer to the same low-level resource.

```gdscript
operator >(right: RID)
```

Returns true if the RID's ID is greater than right's ID.

```gdscript
operator >=(right: RID)
```

Returns true if the RID's ID is greater than or equal to right's ID.
