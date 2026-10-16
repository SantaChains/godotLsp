# int

> Signed 64-bit integer type. This means that it can take values from -2^63 to 2^63 - 1, i.e. from -9223372036854775808 to 9223372036854775807. When it exceeds these bounds, it will wrap around.

<details><summary>官方英文描述</summary>

Signed 64-bit integer type. This means that it can take values from -2^63 to 2^63 - 1, i.e. from -9223372036854775808 to 9223372036854775807. When it exceeds these bounds, it will wrap around.

ints can be automatically converted to floats when necessary, for example when passing them as arguments in functions. The float will be as close to the original integer as possible.

Likewise, floats can be automatically converted into ints. This will truncate the float, discarding anything after the floating-point.

Note: In a boolean context, an int will evaluate to false if it equals 0, and to true otherwise.

```
var x: int = 1 # x is 1
x = 4.2 # x is 4, because 4.2 gets truncated
var max_int = 9223372036854775807 # Biggest value an int can store
max_int += 1 # max_int is -9223372036854775808, because it wrapped around
```

```
int x = 1; // x is 1
x = (int)4.2; // x is 4, because 4.2 gets truncated
// We use long below, because GDScript's int is 64-bit while C#'s int is 32-bit.
long maxLong = 9223372036854775807; // Biggest value a long can store
maxLong++; // maxLong is now -9223372036854775808, because it wrapped around.
// Alternatively with C#'s 32-bit int type, which has a smaller maximum value.
int maxInt = 2147483647; // Biggest value an int can store
maxInt++; // maxInt is now -2147483648, because it wrapped around
```

You can use the 0b literal for binary representation, the 0x literal for hexadecimal representation, and the _ symbol to separate long numbers and improve readability.

```
var x = 0b1001 # x is 9
var y = 0xF5 # y is 245
var z = 10_000_000 # z is 10000000
```

```
int x = 0b1001; // x is 9
int y = 0xF5; // y is 245
int z = 10_000_000; // z is 10000000
```

</details>
