# OpenXRVisibilityMask

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRVisibilityMask**

> The visibility mask allows us to black out the part of the render result that is invisible due to lens distortion.

<details><summary>官方英文描述（中文精译进行中）</summary>

The visibility mask allows us to black out the part of the render result that is invisible due to lens distortion.

As this is rendered first, it prevents fragments with expensive lighting calculations to be processed as they are discarded through z-checking.

</details>
