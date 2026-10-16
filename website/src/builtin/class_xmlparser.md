# XMLParser

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **XMLParser**

> Provides a low-level interface for creating parsers for XML [https://en.wikipedia.org/wiki/XML] files. This class can serve as base to make custom XML parsers.

<details><summary>官方英文描述（中文精译进行中）</summary>

Provides a low-level interface for creating parsers for XML [https://en.wikipedia.org/wiki/XML] files. This class can serve as base to make custom XML parsers.

To parse XML, you must open a file with the open() method or a buffer with the open_buffer() method. Then, the read() method must be called to parse the next nodes. Most of the methods take into consideration the currently parsed node.

Here is an example of using XMLParser to parse an SVG file (which is based on XML), printing each element and its attributes as a dictionary:

```
var parser = XMLParser.new()
parser.open("path/to/file.svg")
while parser.read() != ERR_FILE_EOF:
    if parser.get_node_type() == XMLParser.NODE_ELEMENT:
        var node_name = parser.get_node_name()
        var attributes_dict = {}
        for idx in range(parser.get_attribute_count()):
            attributes_dict[parser.get_attribute_name(idx)] = parser.get_attribute_value(idx)
        print("The ", node_name, " element has the following attributes: ", attributes_dict)
```

```
var parser = new XmlParser();
parser.Open("path/to/file.svg");
while (parser.Read() != Error.FileEof)
{
    if (parser.GetNodeType() == XmlParser.NodeType.Element)
    {
        var nodeName = parser.GetNodeName();
        var attributesDict = new Godot.Collections.Dictionary();
        for (int idx = 0; idx < parser.GetAttributeCount(); idx++)
        {
            attributesDict[parser.GetAttributeName(idx)] = parser.GetAttributeValue(idx);
        }
        GD.Print($"The {nodeName} element has the following attributes: {attributesDict}");
    }
}
```

</details>

## 方法

### get_attribute_count {#get_attribute_count}

```gdscript
int get_attribute_count() const
```

Returns the number of attributes in the currently parsed element.

Note: If this method is used while the currently parsed node is not NODE_ELEMENT or NODE_ELEMENT_END, this count will not be updated and will still reflect the last element.

### get_attribute_name {#get_attribute_name}

```gdscript
String get_attribute_name(idx: int) const
```

Returns the name of an attribute of the currently parsed element, specified by the idx index.

### get_attribute_value {#get_attribute_value}

```gdscript
String get_attribute_value(idx: int) const
```

Returns the value of an attribute of the currently parsed element, specified by the idx index.

### get_current_line {#get_current_line}

```gdscript
int get_current_line() const
```

Returns the current line in the parsed file, counting from 0.

### get_named_attribute_value {#get_named_attribute_value}

```gdscript
String get_named_attribute_value(name: String) const
```

Returns the value of an attribute of the currently parsed element, specified by its name. This method will raise an error if the element has no such attribute.

### get_named_attribute_value_safe {#get_named_attribute_value_safe}

```gdscript
String get_named_attribute_value_safe(name: String) const
```

Returns the value of an attribute of the currently parsed element, specified by its name. This method will return an empty string if the element has no such attribute.

### get_node_data {#get_node_data}

```gdscript
String get_node_data() const
```

Returns the contents of a text node. This method will raise an error if the current parsed node is of any other type.

### get_node_name {#get_node_name}

```gdscript
String get_node_name() const
```

Returns the name of a node. This method will raise an error if the currently parsed node is a text node.

Note: The content of a NODE_CDATA node and the comment string of a NODE_COMMENT node are also considered names.

### get_node_offset {#get_node_offset}

```gdscript
int get_node_offset() const
```

Returns the byte offset of the currently parsed node since the beginning of the file or buffer. This is usually equivalent to the number of characters before the read position.

### get_node_type {#get_node_type}

```gdscript
XMLParser.NodeType get_node_type()
```

Returns the type of the current node. Compare with NodeType constants.

### has_attribute {#has_attribute}

```gdscript
bool has_attribute(name: String) const
```

Returns true if the currently parsed element has an attribute with the name.

### is_empty {#is_empty}

```gdscript
bool is_empty() const
```

Returns true if the currently parsed element is empty, e.g. .

### open {#open}

```gdscript
Error open(file: String)
```

Opens an XML file for parsing. This method returns an error code.

### open_buffer {#open_buffer}

```gdscript
Error open_buffer(buffer: PackedByteArray)
```

Opens an XML raw buffer for parsing. This method returns an error code.

### read {#read}

```gdscript
Error read()
```

Parses the next node in the file. This method returns an error code.

### seek {#seek}

```gdscript
Error seek(position: int)
```

Moves the buffer cursor to a certain offset (since the beginning) and reads the next node there. This method returns an error code.

### skip_section {#skip_section}

```gdscript
void skip_section()
```

Skips the current section. If the currently parsed node contains more inner nodes, they will be ignored and the cursor will go to the closing of the current element.

## 枚举

### NodeType {#enum-NodeType}

```gdscript
enum NodeType { NODE_NONE = 0, NODE_ELEMENT = 1, NODE_ELEMENT_END = 2, NODE_TEXT = 3, NODE_COMMENT = 4, NODE_CDATA = 5, NODE_UNKNOWN = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NODE_NONE {#NODE_NONE}

```gdscript
NODE_NONE = 0
```

There's no node (no file or buffer opened).

#### NODE_ELEMENT {#NODE_ELEMENT}

```gdscript
NODE_ELEMENT = 1
```

An element node type, also known as a tag, e.g. .

#### NODE_ELEMENT_END {#NODE_ELEMENT_END}

```gdscript
NODE_ELEMENT_END = 2
```

An end of element node type, e.g. .

#### NODE_TEXT {#NODE_TEXT}

```gdscript
NODE_TEXT = 3
```

A text node type, i.e. text that is not inside an element. This includes whitespace.

#### NODE_COMMENT {#NODE_COMMENT}

```gdscript
NODE_COMMENT = 4
```

A comment node type, e.g. <!--A comment-->.

#### NODE_CDATA {#NODE_CDATA}

```gdscript
NODE_CDATA = 5
```

A node type for CDATA (Character Data) sections, e.g. <![CDATA[CDATA section]]>.

#### NODE_UNKNOWN {#NODE_UNKNOWN}

```gdscript
NODE_UNKNOWN = 6
```

An unknown node type.

---
