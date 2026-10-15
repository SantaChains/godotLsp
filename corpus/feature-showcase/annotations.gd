@tool
extends Node

@export var label: String = "hello"
@export_range(0, 100) var amount: int = 10
@export_group("Combat")
@export var attack: float = 5.0
@onready var timer: Timer = $Timer
@onready var sprite: Sprite2D = $Sprite2D

@warning_ignore("unused_variable")
var _debug := 0

@rpc("authority", "call_remote", "reliable")
func sync_state(data: Dictionary) -> void:
	pass

func _ready() -> void:
	if Engine.is_editor_hint():
		print("editor")
