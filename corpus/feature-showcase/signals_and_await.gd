extends Node

signal health_changed(old: int, new: int)
signal died

@export var max_hp: int = 100

func _ready() -> void:
	health_changed.connect(_on_health_changed)
	await died          # 协程挂起，等待信号
	print("cleanup")

func _on_health_changed(old: int, new: int) -> void:
	if new <= 0:
		died.emit()

func heal(amount: int) -> void:
	var before := _hp
	_hp = min(_hp + amount, max_hp)
	health_changed.emit(before, _hp)

var _hp: int = max_hp
