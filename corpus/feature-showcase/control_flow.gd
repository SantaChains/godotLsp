extends Node

func classify(n: int) -> String:
	if n < 0:
		return "negative"
	elif n == 0:
		return "zero"
	else:
		return "positive"

func sum_to(n: int) -> int:
	var total := 0
	for i in range(1, n + 1):
		total += i
	return total

func countdown() -> void:
	var i := 3
	while i > 0:
		print(i)
		i -= 1

# match 模式：字面量 / 多候选 / 守卫 / 通配
func describe(v: Variant) -> String:
	match v:
		null:
			return "nothing"
		0:
			return "zero"
		"a", "b":
			return "letter"
		var x where x > 10:
			return "big"
		_:
			return "other"
