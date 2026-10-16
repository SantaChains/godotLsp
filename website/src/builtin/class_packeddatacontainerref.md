# PackedDataContainerRef

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **PackedDataContainerRef**

> When packing nested containers using PackedDataContainer, they are recursively packed into PackedDataContainerRef (only applies to Array and Dictionary). Their data can be retrieved the same way as from PackedDataContainer.

<details><summary>官方英文描述（中文精译进行中）</summary>

When packing nested containers using PackedDataContainer, they are recursively packed into PackedDataContainerRef (only applies to Array and Dictionary). Their data can be retrieved the same way as from PackedDataContainer.

```gdscript
var packed = PackedDataContainer.new()
packed.pack([1, 2, 3, ["nested1", "nested2"], 4, 5, 6])
for element in packed:
    if element is PackedDataContainerRef:
        for subelement in element:
            print("::", subelement)
    else:
        print(element)
```

Prints:

```
1
2
3
::nested1
::nested2
4
5
6
```

</details>

## 方法

### size {#size}

```gdscript
int size() const
```

Returns the size of the packed container (see Array.size() and Dictionary.size()).
