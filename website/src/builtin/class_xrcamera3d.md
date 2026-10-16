# XRCamera3D

**继承**：[Camera3D](class_camera3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **XRCamera3D**

> A camera node which automatically positions itself based on XR tracking data.

<details><summary>官方英文描述（中文精译进行中）</summary>

A camera node which automatically positions itself based on XR tracking data.

In contrast to XRController3D, the render thread has access to more up-to-date tracking data, and the location of the XRCamera3D node can lag a few milliseconds behind what is used for rendering.

Note: If Viewport.use_xr is true, most of the camera properties are overridden by the active XRInterface. The only properties that can be trusted are the near and far planes.

</details>
