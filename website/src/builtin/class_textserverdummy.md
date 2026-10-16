# TextServerDummy

**继承**：[TextServerExtension](class_textserverextension.md) → [TextServer](class_textserver.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TextServerDummy**

> A dummy TextServer interface that doesn't do anything. Useful for freeing up memory when rendering text is not needed, as text servers are resource-intensive. It can also be used for performance comparisons in complex GUIs to check the impact of text rendering.

<details><summary>官方英文描述（中文精译进行中）</summary>

A dummy TextServer interface that doesn't do anything. Useful for freeing up memory when rendering text is not needed, as text servers are resource-intensive. It can also be used for performance comparisons in complex GUIs to check the impact of text rendering.

A dummy text server is always available at the start of a project. Here's how to access it:

```gdscript
var dummy_text_server = TextServerManager.find_interface("Dummy")
if dummy_text_server != null:
    TextServerManager.set_primary_interface(dummy_text_server)
    # If the other text servers are unneeded, they can be removed:
    for i in TextServerManager.get_interface_count():
        var text_server = TextServerManager.get_interface(i)
        if text_server != dummy_text_server:
            TextServerManager.remove_interface(text_server)
```

The command line argument --text-driver Dummy (case-sensitive) can be used to force the "Dummy" TextServer on any project.

</details>
