extends Node2D

class_name Player

var health := 100

func take_damage(amount: int) -> void:
	health -= amount
