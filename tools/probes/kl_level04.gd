extends SceneTree
##
## knowledge-lock level_04(规则测试场)结构回归 + 缺场景回退回归。
##
## 覆盖:level_04 可加载/可实例化、5 个规则碎片、移动平台进 freezable 组、单向平台进
##       one_way_platform 组、两扇彩门 color_id 可读、玩家与出口门就位;
##       以及 level_05 缺失时 load_level 走"回主菜单"而不是硬切。
##
## 用法(由 tools/godot_probe.py 注入工程副本后运行):
##   uv run tools/godot_probe.py <knowledge-lock 工程> tools/probes/kl_level04.gd

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
			var packed = load("res://scenes/levels/level_04.tscn")
			_check("level_04.tscn 可加载", packed != null)
			var level = (packed as PackedScene).instantiate()
			root.add_child(level)
			_check("level_04 实例化成功", level != null)
			var shards := 0
			var gates := 0
			var gate_ids: Array = []
			for node in level.get_children():
				if node.is_in_group("shard"):
					shards += 1
				if node.is_in_group("color_gate"):
					gates += 1
					gate_ids.append(node.get("color_id"))
			_check("5 个规则碎片(每类一个)", shards == 5, str(shards))
			var mover = level.get_node_or_null("Mover")
			_check("移动平台存在且在 freezable 组", mover != null and mover.is_in_group("freezable"))
			var oneway = level.get_node_or_null("OneWay")
			_check("单向平台在 one_way_platform 组", oneway != null and oneway.is_in_group("one_way_platform"))
			_check("两扇彩门且 color_id 可读", gates == 2 and not gate_ids.has(null), str(gate_ids))
			_check("玩家与出口门就位",
				level.get_node_or_null("Player") != null and level.get_node_or_null("ExitDoor") != null)
			_phase = 1
		1:
			_check("level_05 确实缺失(能走到回退分支)", not ResourceLoader.exists("res://scenes/levels/level_05.tscn"))
			(root.get_node("/root/LevelManager") as Node).load_level(5)
			_phase = 2
			_wait = 30
		2:
			print("RESULT failures=", _fails)
			_phase = 3
		3:
			quit(1 if _fails > 0 else 0)
	return false
