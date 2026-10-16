# VisualShaderNodeFaceForward

**继承**：[VisualShaderNodeVectorBase](class_visualshadernodevectorbase.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeFaceForward**

> Translates to faceforward(N, I, Nref) in the shader language. The function has three vector parameters: N, the vector to orient, I, the incident vector, and Nref, the reference vector. If the dot product of I and Nref is smaller than zero the return value is N. Otherwise, -N is returned.

<details><summary>官方英文描述（中文精译进行中）</summary>

Translates to faceforward(N, I, Nref) in the shader language. The function has three vector parameters: N, the vector to orient, I, the incident vector, and Nref, the reference vector. If the dot product of I and Nref is smaller than zero the return value is N. Otherwise, -N is returned.

</details>
