# CCDIK3D

**继承**：[IterateIK3D](class_iterateik3d.md) → [ChainIK3D](class_chainik3d.md) → [IKModifier3D](class_ikmodifier3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CCDIK3D**

> CCDIK3D is rotation based IK, enabling fast and effective tracking even with large joint rotations. It's especially suitable for chains with limitations, providing smoother and more stable target tracking compared to FABRIK3D.

<details><summary>官方英文描述</summary>

CCDIK3D is rotation based IK, enabling fast and effective tracking even with large joint rotations. It's especially suitable for chains with limitations, providing smoother and more stable target tracking compared to FABRIK3D.

The resulting twist around the forward vector will always be kept from the previous pose.

Note: When the target is close to the root, it can cause unnatural movement, including joint flips and oscillations.

</details>
