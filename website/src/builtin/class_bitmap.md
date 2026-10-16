# BitMap

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **BitMap**

> A two-dimensional array of boolean values, can be used to efficiently store a binary matrix (every matrix element takes only one bit) and query the values using natural cartesian coordinates.

<details><summary>官方英文描述</summary>

A two-dimensional array of boolean values, can be used to efficiently store a binary matrix (every matrix element takes only one bit) and query the values using natural cartesian coordinates.

</details>

## 方法

### convert_to_image {#convert_to_image}

```gdscript
Image convert_to_image() const
```

Returns an image of the same size as the bitmap and with an Format of type Image.FORMAT_L8. true bits of the bitmap are being converted into white pixels, and false bits into black.

### create {#create}

```gdscript
void create(size: Vector2i)
```

Creates a bitmap with the specified size, filled with false.

### create_from_image_alpha {#create_from_image_alpha}

```gdscript
void create_from_image_alpha(image: Image, threshold: float = 0.1)
```

Creates a bitmap that matches the given image dimensions, every element of the bitmap is set to false if the alpha value of the image at that position is equal to threshold or less, and true in other case.

### get_bit {#get_bit}

```gdscript
bool get_bit(x: int, y: int) const
```

Returns bitmap's value at the specified position.

### get_bitv {#get_bitv}

```gdscript
bool get_bitv(position: Vector2i) const
```

Returns bitmap's value at the specified position.

### get_size {#get_size}

```gdscript
Vector2i get_size() const
```

Returns bitmap's dimensions.

### get_true_bit_count {#get_true_bit_count}

```gdscript
int get_true_bit_count() const
```

Returns the number of bitmap elements that are set to true.

### grow_mask {#grow_mask}

```gdscript
void grow_mask(pixels: int, rect: Rect2i)
```

Applies morphological dilation or erosion to the bitmap. If pixels is positive, dilation is applied to the bitmap. If pixels is negative, erosion is applied to the bitmap. rect defines the area where the morphological operation is applied. Pixels located outside the rect are unaffected by grow_mask().

### opaque_to_polygons {#opaque_to_polygons}

```gdscript
typedarray::PackedVector2Array opaque_to_polygons(rect: Rect2i, epsilon: float = 2.0) const
```

Creates an Array of polygons covering a rectangular portion of the bitmap. It uses a marching squares algorithm, followed by Ramer-Douglas-Peucker (RDP) reduction of the number of vertices. Each polygon is described as a PackedVector2Array of its vertices.

To get polygons covering the whole bitmap, pass:

```gdscript
Rect2(Vector2(), get_size())
```

epsilon is passed to RDP to control how accurately the polygons cover the bitmap: a lower epsilon corresponds to more points in the polygons.

### resize {#resize}

```gdscript
void resize(new_size: Vector2i)
```

Resizes the image to new_size.

### set_bit {#set_bit}

```gdscript
void set_bit(x: int, y: int, bit: bool)
```

Sets the bitmap's element at the specified position, to the specified value.

### set_bit_rect {#set_bit_rect}

```gdscript
void set_bit_rect(rect: Rect2i, bit: bool)
```

Sets a rectangular portion of the bitmap to the specified value.

### set_bitv {#set_bitv}

```gdscript
void set_bitv(position: Vector2i, bit: bool)
```

Sets the bitmap's element at the specified position, to the specified value.

## 属性

### data {#data}

```gdscript
data: Dictionary
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
