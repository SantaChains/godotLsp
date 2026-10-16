# AnimationNodeBlend3

**继承**：[AnimationNodeSync](class_animationnodesync.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeBlend3**

> A resource to add to an AnimationNodeBlendTree. Blends two animations out of three linearly out of three based on the amount value.

<details><summary>官方英文描述（中文精译进行中）</summary>

A resource to add to an AnimationNodeBlendTree. Blends two animations out of three linearly out of three based on the amount value.

This animation node has three inputs:

- The base animation to blend with
- A "-blend" animation to blend with when the blend amount is negative value
- A "+blend" animation to blend with when the blend amount is positive value

In general, the blend value should be in the [-1.0, 1.0] range. Values outside of this range can blend amplified animations, however, AnimationNodeAdd3 works better for this purpose.

</details>
