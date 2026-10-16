# GridContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GridContainer**

> GridContainer arranges its child controls in a grid layout. The number of columns is specified by the columns property, whereas the number of rows depends on how many are needed for the child controls. The number of rows and columns is preserved for every size of the container.

<details><summary>官方英文描述（中文精译进行中）</summary>

GridContainer arranges its child controls in a grid layout. The number of columns is specified by the columns property, whereas the number of rows depends on how many are needed for the child controls. The number of rows and columns is preserved for every size of the container.

Note: GridContainer only works with child nodes inheriting from Control. It won't rearrange child nodes inheriting from Node2D.

</details>

## 方法

### get_columns {#get_columns}

```gdscript
int get_columns() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_columns {#set_columns}

```gdscript
void set_columns(columns: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### columns {#columns}

```gdscript
columns: int
```

- void set_columns(value: int)
- int get_columns()

The number of columns in the GridContainer. If modified, GridContainer reorders its Control-derived children to accommodate the new layout.
