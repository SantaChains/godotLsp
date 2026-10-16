extends Node2D

const SCENE := preload("uid://c9fixtmain")

func _ready() -> void:
	var s := preload("res://main.tscn")
	print(s)
