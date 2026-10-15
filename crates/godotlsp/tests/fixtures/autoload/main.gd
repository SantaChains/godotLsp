extends Node

func _ready() -> void:
	GameConfig.high_score = 100
	GameConfig.save_score()
