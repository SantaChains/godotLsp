# PolygonPathFinder

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PolygonPathFinder**

## 方法

### find_path {#find_path}

```gdscript
PackedVector2Array find_path(from: Vector2, to: Vector2)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_bounds {#get_bounds}

```gdscript
Rect2 get_bounds() const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_closest_point {#get_closest_point}

```gdscript
Vector2 get_closest_point(point: Vector2) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_intersections {#get_intersections}

```gdscript
PackedVector2Array get_intersections(from: Vector2, to: Vector2) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_point_penalty {#get_point_penalty}

```gdscript
float get_point_penalty(idx: int) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### is_point_inside {#is_point_inside}

```gdscript
bool is_point_inside(point: Vector2) const
```

Returns true if point falls inside the polygon area.

```
var polygon_path_finder = PolygonPathFinder.new()
var points = [Vector2(0.0, 0.0), Vector2(1.0, 0.0), Vector2(0.0, 1.0)]
var connections = [0, 1, 1, 2, 2, 0]
polygon_path_finder.setup(points, connections)
print(polygon_path_finder.is_point_inside(Vector2(0.2, 0.2))) # Prints true
print(polygon_path_finder.is_point_inside(Vector2(1.0, 1.0))) # Prints false
```

```
var polygonPathFinder = new PolygonPathFinder();
Vector2[] points =
[
    new Vector2(0.0f, 0.0f),
    new Vector2(1.0f, 0.0f),
    new Vector2(0.0f, 1.0f)
];
int[] connections = [0, 1, 1, 2, 2, 0];
polygonPathFinder.Setup(points, connections);
GD.Print(polygonPathFinder.IsPointInside(new Vector2(0.2f, 0.2f))); // Prints True
GD.Print(polygonPathFinder.IsPointInside(new Vector2(1.0f, 1.0f))); // Prints False
```

### set_point_penalty {#set_point_penalty}

```gdscript
void set_point_penalty(idx: int, penalty: float)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### setup {#setup}

```gdscript
void setup(points: PackedVector2Array, connections: PackedInt32Array)
```

Sets up PolygonPathFinder with an array of points that define the vertices of the polygon, and an array of indices that determine the edges of the polygon.

The length of connections must be even, returns an error if odd.

```
var polygon_path_finder = PolygonPathFinder.new()
var points = [Vector2(0.0, 0.0), Vector2(1.0, 0.0), Vector2(0.0, 1.0)]
var connections = [0, 1, 1, 2, 2, 0]
polygon_path_finder.setup(points, connections)
```

```
var polygonPathFinder = new PolygonPathFinder();
Vector2[] points =
[
    new Vector2(0.0f, 0.0f),
    new Vector2(1.0f, 0.0f),
    new Vector2(0.0f, 1.0f)
];
int[] connections = [0, 1, 1, 2, 2, 0];
polygonPathFinder.Setup(points, connections);
```

## 属性

### data {#data}

```gdscript
data: Dictionary
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
