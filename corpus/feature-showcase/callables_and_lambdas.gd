extends Node

func _ready() -> void:
	# 内联 lambda（GDScript 2.0 一等公民）
	var cb := func(x: int) -> int: return x * 2
	print(cb.call(21))

	# Callable 绑定参数
	var bound := _double.bind(3)
	bound.call()

	# 把 lambda 当作 Callable 参数传递
	apply_to_list([1, 2, 3], func(n: int) -> void: print(n * n))

func _double(a: int, b: int = 0) -> int:
	return a + b

func apply_to_list(data: Array, op: Callable) -> void:
	for d in data:
		op.call(d)
