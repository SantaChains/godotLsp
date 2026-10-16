# AnimationNodeAdd3

**继承**：[AnimationNodeSync](class_animationnodesync.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeAdd3**

> A resource to add to an AnimationNodeBlendTree. Blends two animations out of three additively out of three based on the amount value.

<details><summary>官方英文描述（中文精译进行中）</summary>

A resource to add to an AnimationNodeBlendTree. Blends two animations out of three additively out of three based on the amount value.

This animation node has three inputs:

- The base animation to add to
- A "-add" animation to blend with when the blend amount is negative
- A "+add" animation to blend with when the blend amount is positive

If the absolute value of the amount is greater than 1.0, the animation connected to "in" port is blended with the amplified animation connected to "-add"/"+add" port.

Note: The signs are only used to distinguish ports, and additive blending occurs based on absolute values always, meaning the animation of a "-add" port does not subtract from the animation of an "in" port.

</details>
