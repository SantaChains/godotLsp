extends Node

# 显式类型注解
var name: String = "player"
var position: Vector2 = Vector2.ZERO

# 类型推断（:= 触发）
var level := 1
var ratio := 0.75
var items := ["sword", "shield"]

func add(a: int, b: int) -> int:
	return a + b

func first() -> String:
	return items[0]

# Variant 与空安全
var maybe: Variant = null
func unwrap() -> int:
	if maybe != null:
		return int(maybe)
	return 0
