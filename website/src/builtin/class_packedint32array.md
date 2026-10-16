# PackedInt32Array

> 紧凑 32 位整数数组。

<details><summary>官方英文描述</summary>

An array specifically designed to hold 32-bit integer values. Packs data tightly, so it saves memory for large array sizes.

Note: This type stores signed 32-bit integers, which means it can take values in the interval [-2^31, 2^31 - 1], i.e. [-2147483648, 2147483647]. Exceeding those bounds will wrap around. In comparison, int uses signed 64-bit integers which can hold much larger values. If you need to pack 64-bit integers tightly, see PackedInt64Array.

Note: Packed arrays are always passed by reference. To get a copy of an array that can be modified independently of the original array, use duplicate(). This is not the case for built-in properties and methods. In these cases the returned packed array is a copy, and changing it will not affect the original value. To update a built-in property of this type, modify the returned array and then assign it to the property again.

Note: In a boolean context, a packed array will evaluate to false if it's empty. Otherwise, a packed array will always evaluate to true.

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 方法

### append {#append}

```gdscript
bool append(value: int)
```

Appends an element at the end of the array (alias of push_back()).

### append_array {#append_array}

```gdscript
void append_array(array: PackedInt32Array)
```

Appends a PackedInt32Array at the end of this array.

### bsearch {#bsearch}

```gdscript
int bsearch(value: int, before: bool = true) const
```

Finds the index of an existing value (or the insertion index that maintains sorting order, if the value is not yet present in the array) using binary search. Optionally, a before specifier can be passed. If false, the returned index comes after all existing entries of the value in the array.

Note: Calling bsearch() on an unsorted array results in unexpected behavior.

### clear {#clear}

```gdscript
void clear()
```

Clears the array. This is equivalent to using resize() with a size of 0.

### count {#count}

```gdscript
int count(value: int) const
```

Returns the number of times an element is in the array.

### duplicate {#duplicate}

```gdscript
PackedInt32Array duplicate() const
```

Creates a copy of the array, and returns it.

### erase {#erase}

```gdscript
bool erase(value: int)
```

Removes the first occurrence of a value from the array and returns true. If the value does not exist in the array, nothing happens and false is returned. To remove an element by index, use remove_at() instead.

### fill {#fill}

```gdscript
void fill(value: int)
```

Assigns the given value to all elements in the array. This can typically be used together with resize() to create an array with a given size and initialized elements.

### find {#find}

```gdscript
int find(value: int, from: int = 0) const
```

Searches the array for a value and returns its index or -1 if not found. Optionally, the initial search index can be passed.

### get {#get}

```gdscript
int get(index: int) const
```

Returns the 32-bit integer at the given index in the array. If index is out-of-bounds or negative, this method fails and returns 0.

This method is similar (but not identical) to the [] operator. Most notably, when this method fails, it doesn't pause project execution if run from the editor.

### has {#has}

```gdscript
bool has(value: int) const
```

Returns true if the array contains value.

### insert {#insert}

```gdscript
int insert(at_index: int, value: int)
```

Inserts a new integer at a given position in the array. The position must be valid, or at the end of the array (idx == size()).

### is_empty {#is_empty}

```gdscript
bool is_empty() const
```

Returns true if the array is empty.

### push_back {#push_back}

```gdscript
bool push_back(value: int)
```

Appends a value to the array.

### remove_at {#remove_at}

```gdscript
void remove_at(index: int)
```

Removes an element from the array by index.

### resize {#resize}

```gdscript
int resize(new_size: int)
```

Sets the size of the array. If the array is grown, reserves elements at the end of the array. If the array is shrunk, truncates the array to the new size. Calling resize() once and assigning the new values is faster than adding new elements one by one.

Returns @GlobalScope.OK on success, or one of the following Error constants if this method fails: @GlobalScope.ERR_INVALID_PARAMETER if the size is negative, or @GlobalScope.ERR_OUT_OF_MEMORY if allocations fail. Use size() to find the actual size of the array after resize.

### reverse {#reverse}

```gdscript
void reverse()
```

Reverses the order of the elements in the array.

### rfind {#rfind}

```gdscript
int rfind(value: int, from: int = -1) const
```

Searches the array in reverse order. Optionally, a start search index can be passed. If negative, the start index is considered relative to the end of the array.

### set {#set}

```gdscript
void set(index: int, value: int)
```

Changes the integer at the given index.

### size {#size}

```gdscript
int size() const
```

Returns the number of elements in the array.

### slice {#slice}

```gdscript
PackedInt32Array slice(begin: int, end: int = 2147483647) const
```

Returns the slice of the PackedInt32Array, from begin (inclusive) to end (exclusive), as a new PackedInt32Array.

The absolute value of begin and end will be clamped to the array size, so the default value for end makes it slice to the size of the array by default (i.e. arr.slice(1) is a shorthand for arr.slice(1, arr.size())).

If either begin or end are negative, they will be relative to the end of the array (i.e. arr.slice(0, -2) is a shorthand for arr.slice(0, arr.size() - 2)).

### sort {#sort}

```gdscript
void sort()
```

Sorts the elements of the array in ascending order.

### to_byte_array {#to_byte_array}

```gdscript
PackedByteArray to_byte_array() const
```

Returns a copy of the data converted to a PackedByteArray, where each element has been encoded as 4 bytes.

The size of the new array will be int32_array.size() * 4.
