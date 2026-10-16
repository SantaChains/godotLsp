extends Node2D

const SCENE := preload("uid://c9fixtmain")
const COL := Color(0.5, 0.25, 1.0, 0.8)
const RED := Color.RED

func _ready() -> void:
	var s := preload("res://main.tscn")
	print(s)
