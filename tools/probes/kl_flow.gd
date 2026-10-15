extends SceneTree
##
## knowledge-lock 关卡流程回归:走真实路径 LevelManager.load_level(4),确认当前场景切到 Level04。
## 这条覆盖"level_04 真的能被关卡管理器加载"(而不是只有 load() 能读)。
##
## 用法(由 tools/godot_probe.py 注入工程副本后运行):
##   uv run tools/godot_probe.py <knowledge-lock 工程> tools/probes/kl_flow.gd

var _fails := 0
var _phase := 0
var _wait := 0


func _check(label: String, cond: bool, extra: String = "") -> void:
	print(("PASS | " if cond else "FAIL | ") + label + ("  " + extra if extra else ""))
	if not cond:
		_fails += 1


func _process(_delta: float) -> bool:
	if _wait > 0:
		_wait -= 1
		return false
	match _phase:
		0:
			(root.get_node("/root/LevelManager") as Node).load_level(4)
			_phase = 1
			_wait = 60
		1:
			var scene := current_scene
			_check("load_level(4) 后当前场景是 Level04",
				scene != null and scene.name == "Level04", str(scene.name if scene else "null"))
			print("RESULT failures=", _fails)
			_phase = 2
		2:
			quit(1 if _fails > 0 else 0)
	return false
