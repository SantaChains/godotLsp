extends Node

enum State { IDLE, RUN, JUMP, FALL }
enum Direction { LEFT = -1, RIGHT = 1 }

const GRAVITY: float = 9.8
const TAU_HALF: float = TAU / 2.0

func move(dir: Direction) -> void:
	position.x += dir * speed

var position := Vector2.ZERO
var speed := 200.0

func current() -> State:
	return State.IDLE
