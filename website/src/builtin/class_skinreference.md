# SkinReference

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkinReference**

> An internal object containing a mapping from a Skin used within the context of a particular MeshInstance3D to refer to the skeleton's RID in the RenderingServer.

<details><summary>官方英文描述（中文精译进行中）</summary>

An internal object containing a mapping from a Skin used within the context of a particular MeshInstance3D to refer to the skeleton's RID in the RenderingServer.

See also MeshInstance3D.get_skin_reference() and RenderingServer.instance_attach_skeleton().

Note that despite the similar naming, the skeleton RID used in the RenderingServer does not have a direct one-to-one correspondence to a Skeleton3D node.

In particular, a Skeleton3D node with no MeshInstance3D children may be unknown to the RenderingServer.

On the other hand, a Skeleton3D with multiple MeshInstance3D nodes which each have different MeshInstance3D.skin objects may have multiple SkinReference instances (and hence, multiple skeleton RIDs).

</details>

## 方法

### get_skeleton {#get_skeleton}

```gdscript
RID get_skeleton() const
```

Returns the RID owned by this SkinReference, as returned by RenderingServer.skeleton_create().

### get_skin {#get_skin}

```gdscript
Skin get_skin() const
```

Returns the Skin connected to this SkinReference. In the case of MeshInstance3D with no MeshInstance3D.skin assigned, this will reference an internal default Skin owned by that MeshInstance3D.

Note that a single Skin may have more than one SkinReference in the case that it is shared by meshes across multiple Skeleton3D nodes.
