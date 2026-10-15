extends Node
class_name Enemy

# 显式常量与类型注解
const MAX_HP: int = 100

var hp: int = MAX_HP
var _speed: float = 1.5

func _ready() -> void:
	print("enemy spawned with hp ", hp)

func take_damage(amount: int) -> void:
	hp = max(hp - amount, 0)
	if hp == 0:
		_die()

func _die() -> void:
	queue_free()

# 内部类：命名空间 + 自身作用域
class Armor:
	var defense: int = 10
	func reduce(dmg: int) -> int:
		return max(dmg - defense, 0)
