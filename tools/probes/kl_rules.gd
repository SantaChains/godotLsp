extends SceneTree
##
## knowledge-lock 规则行为回归(25 条断言)。
##
## 覆盖:SLOW / FREEZE_OBJ / ONE_WAY_UP / ZOOM_OUT / COLOR_LOCK / GHOST / REWIND / TIMED /
##       以及 restore_all_effects(防跨关卡残留)。
##
## 用法(由 tools/godot_probe.py 注入工程副本后运行):
##   uv run tools/godot_probe.py <knowledge-lock 工程> tools/probes/kl_rules.gd
##
## 依赖:scripts/rules/rule_manager.gd、scripts/objects/{moving_platform,color_gate}.gd、
##       scripts/player/player_physics.gd、scenes/levels/level_03.tscn。
## 测试对象(移动平台/单向平台/彩门)在运行时构造,因此不依赖关卡内容。

var _rules: Node
var _player: CharacterBody2D
var _cam: Camera2D
var _mover: AnimatableBody2D
var _oneway: CollisionShape2D
var _gate_same: StaticBody2D
var _gate_other: StaticBody2D
var _gate_plain: StaticBody2D
var _fails := 0
var _phase := 0
var _wait := 0
var _timed_fired := 0
var _drag := false
var _x_before := 0.0
var _frozen_pos := Vector2.ZERO


func _check(label: String, cond: bool, extra: String = "") -> void:
	print(("PASS | " if cond else "FAIL | ") + label + ("  " + extra if extra else ""))
	if not cond:
		_fails += 1


func _gate(color: int) -> StaticBody2D:
	var g = StaticBody2D.new()
	g.set_script(load("res://scripts/objects/color_gate.gd"))
	g.color_id = color
	g.collision_layer = 1
	g.position = Vector2(-150 + color * 40, -40)
	return g


func _process(_delta: float) -> bool:
	if _wait > 0:
		_wait -= 1
		if _drag:
			_player.global_position.x += 5.0
		return false
	match _phase:
		0:
			var lvl = (load("res://scenes/levels/level_03.tscn") as PackedScene).instantiate()
			root.add_child(lvl)
			_rules = root.get_node_or_null("/root/RuleManager")
			_player = lvl.get_node("Player")
			_cam = lvl.get_node_or_null("Camera2D")
			_mover = AnimatableBody2D.new()
			_mover.set_script(load("res://scripts/objects/moving_platform.gd"))
			_mover.position = Vector2(0, -20)
			lvl.add_child(_mover)
			var oneway_body := StaticBody2D.new()
			oneway_body.add_to_group("one_way_platform")
			var oneway_shape := CollisionShape2D.new()
			oneway_shape.shape = RectangleShape2D.new()
			oneway_body.add_child(oneway_shape)
			oneway_body.position = Vector2(-150, -20)
			lvl.add_child(oneway_body)
			_oneway = oneway_shape
			_gate_same = _gate(0)
			_gate_other = _gate(1)
			_gate_plain = StaticBody2D.new()  # 没有 color_id 的门:应被跳过
			_gate_plain.add_to_group("color_gate")
			_gate_plain.collision_layer = 1
			lvl.add_child(_gate_same)
			lvl.add_child(_gate_other)
			lvl.add_child(_gate_plain)
			_phase = 1
		1:
			_check("color_gate.gd 自动加入 color_gate 组", _gate_same.is_in_group("color_gate"))
			_rules.apply_rule("time", _rules.TimeState.SLOW)
			_check("SLOW: time_scale=0.5", is_equal_approx(Engine.time_scale, 0.5), str(Engine.time_scale))
			_rules.apply_rule("time", _rules.TimeState.FREEZE_OBJ)
			_frozen_pos = _mover.position
			_phase = 2
			_wait = 30
		2:
			_check("FREEZE_OBJ: process_mode=DISABLED", _mover.process_mode == Node.PROCESS_MODE_DISABLED)
			_check("FREEZE_OBJ: 位置真停", _mover.position.is_equal_approx(_frozen_pos), str(_mover.position))
			_rules.apply_rule("time", _rules.TimeState.NORMAL)
			_frozen_pos = _mover.position
			_phase = 3
			_wait = 30
		3:
			_check("回 NORMAL: time_scale=1.0", is_equal_approx(Engine.time_scale, 1.0), str(Engine.time_scale))
			_check("回 NORMAL: 平台重新移动", not _mover.position.is_equal_approx(_frozen_pos))
			_rules.apply_rule("collision", _rules.CollisionState.ONE_WAY_UP)
			_check("ONE_WAY_UP: 形状切单向", _oneway.one_way_collision == true)
			_rules.apply_rule("collision", _rules.CollisionState.SOLID)
			_check("回 SOLID: 单向关闭", _oneway.one_way_collision == false)
			_rules.apply_rule("space", _rules.SpaceState.ZOOM_OUT)
			_phase = 4
			_wait = 45
		4:
			_check("ZOOM_OUT: 镜头 zoom<1", _cam != null and _cam.zoom.x < 0.9, str(_cam.zoom if _cam else Vector2.ZERO))
			_rules.apply_rule("space", _rules.SpaceState.NORMAL)
			_phase = 5
			_wait = 45
		5:
			_check("回 NORMAL: 镜头回 1.0", _cam != null and _cam.zoom.is_equal_approx(Vector2.ONE))
			_rules.apply_rule("collision", _rules.CollisionState.COLOR_LOCK)
			_phase = 6
			_wait = 3
		6:
			_check("COLOR_LOCK: 同色门(钥匙)穿透", _gate_same.get_collision_layer_value(1) == false)
			_check("COLOR_LOCK: 异色门保持阻挡", _gate_other.get_collision_layer_value(1) == true)
			_check("COLOR_LOCK: 无 color_id 的门被跳过", _gate_plain.get_collision_layer_value(1) == true)
			_rules.apply_rule("collision", _rules.CollisionState.SOLID)
			_phase = 7
			_wait = 3
		7:
			_check("回 SOLID: 彩门全部恢复",
				_gate_other.get_collision_layer_value(1) == true and _gate_same.get_collision_layer_value(1) == true)
			_rules.apply_rule("life_death", _rules.LifeDeathState.GHOST)
			_phase = 8
			_wait = 3
		8:
			_check("GHOST: 玩家碰撞关闭(真穿透)", _player.get_collision_mask_value(1) == false)
			_rules.apply_rule("life_death", _rules.LifeDeathState.NORMAL)
			_phase = 9
			_wait = 3
		9:
			_check("回 NORMAL: 玩家碰撞恢复", _player.get_collision_mask_value(1) == true)
			_player.global_position = Vector2(-200, 100)
			_player.velocity = Vector2.ZERO
			_drag = true
			_phase = 10
			_wait = 40
		10:
			_drag = false
			_x_before = _player.global_position.x
			_rules.apply_rule("time", _rules.TimeState.REWIND)
			_phase = 11
			_wait = 30
		11:
			_check("REWIND: 玩家被倒放回更早位置", _player.global_position.x < _x_before - 20.0, str(_player.global_position.x))
			_check("REWIND: 回放期间物理暂停", _player.process_mode == Node.PROCESS_MODE_DISABLED)
			_rules.apply_rule("time", _rules.TimeState.NORMAL)
			_phase = 12
			_wait = 3
		12:
			_check("REWIND 结束: 物理恢复", _player.process_mode != Node.PROCESS_MODE_DISABLED)
			_rules.timed_duration = 0.2
			_rules.time_expired.connect(func() -> void: _timed_fired += 1)
			_rules.apply_rule("life_death", _rules.LifeDeathState.TIMED)
			_phase = 13
			_wait = 40
		13:
			_check("TIMED: 归零触发 time_expired", _timed_fired >= 1, str(_timed_fired))
			_check("TIMED: timed_remaining 归零", _rules.timed_remaining == 0.0, str(_rules.timed_remaining))
			_rules.restore_all_effects()
			_phase = 14
			_wait = 40
		14:
			_check("restore_all_effects: time_scale 回 1.0", is_equal_approx(Engine.time_scale, 1.0), str(Engine.time_scale))
			_check("restore_all_effects: 冻结解除", _mover.process_mode != Node.PROCESS_MODE_DISABLED)
			_check("restore_all_effects: 单向关闭", _oneway.one_way_collision == false)
			_check("restore_all_effects: 镜头回 1.0", _cam == null or _cam.zoom.is_equal_approx(Vector2.ONE))
			print("RESULT failures=", _fails)
			_phase = 15
		15:
			quit(1 if _fails > 0 else 0)
	return false
