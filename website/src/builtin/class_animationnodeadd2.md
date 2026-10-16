# AnimationNodeAdd2

**继承**：[AnimationNodeSync](class_animationnodesync.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeAdd2**

> A resource to add to an AnimationNodeBlendTree. Blends two animations additively based on the amount value.

<details><summary>官方英文描述</summary>

A resource to add to an AnimationNodeBlendTree. Blends two animations additively based on the amount value.

If the amount is greater than 1.0, the animation connected to "in" port is blended with the amplified animation connected to "add" port.

If the amount is less than 0.0, the animation connected to "in" port is blended with the inverted animation connected to "add" port.

</details>
