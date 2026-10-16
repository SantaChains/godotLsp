# JacobianIK3D

**继承**：[IterateIK3D](class_iterateik3d.md) → [ChainIK3D](class_chainik3d.md) → [IKModifier3D](class_ikmodifier3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **JacobianIK3D**

> JacobianIK3D calculates rotations for all joints simultaneously, producing natural and smooth movement. It is particularly suited for biological animations.

<details><summary>官方英文描述（中文精译进行中）</summary>

JacobianIK3D calculates rotations for all joints simultaneously, producing natural and smooth movement. It is particularly suited for biological animations.

The resulting twist around the forward vector will always be kept from the previous pose.

Note: It converges more slowly than other IK solvers, leading to gentler and less immediate tracking of targets.

</details>
