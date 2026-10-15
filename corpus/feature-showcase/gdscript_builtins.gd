extends Node

var vec: Vector2 = Vector2(1, 2)
var dict: Dictionary = {"a": 1, "b": 2}
var arr: Array[int] = [1, 2, 3]
var names: PackedStringArray = ["x", "y"]
var flags: PackedInt32Array = [1, 2, 4]

func demo() -> void:
	vec = vec.rotated(PI / 2)
	dict.merge({"c": 3})
	arr.append(4)
	var found := dict.get("a", -1)
	var length := vec.length()
	for key in dict:
		print(key, dict[key])
