<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 物理与区域检测

> Area 触发器、刚体进出、碰撞对象输入、屏幕可见通知。

## Area2D（继承 CollisionObject2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal area_entered(area: Area2D)` | 另一 Area 进入本区域时触发。 | 区域重叠检测。 [类页锚点](../builtin/class_area2d.md#area_entered) |
| `signal area_exited(area: Area2D)` | 另一 Area 离开时触发。 | 重叠解除。 [类页锚点](../builtin/class_area2d.md#area_exited) |
| `signal area_shape_entered(area_rid: RID, area: Area2D, area_shape_index: int, local_shape_index: int)` | 对方 Area 的形状进入时触发。 | 形状级区域检测。 [类页锚点](../builtin/class_area2d.md#area_shape_entered) |
| `signal area_shape_exited(area_rid: RID, area: Area2D, area_shape_index: int, local_shape_index: int)` | 对方 Area 形状离开时触发。 | 配合上者。 [类页锚点](../builtin/class_area2d.md#area_shape_exited) |
| `signal body_entered(body: Node2D)` | 物理体进入本区域时触发，携带对方节点。 | 触发器：`$Zone.body_entered.connect(_on_enter)`，需 monitoring=true。 [类页锚点](../builtin/class_area2d.md#body_entered) |
| `signal body_exited(body: Node2D)` | 物理体离开本区域时触发。 | 离开结算。 [类页锚点](../builtin/class_area2d.md#body_exited) |
| `signal body_shape_entered(body_rid: RID, body: Node2D, body_shape_index: int, local_shape_index: int)` | 对方某个形状进入本区域某形状时触发，带 RID 与形状索引。 | 需要定位具体碰撞形状时用。 [类页锚点](../builtin/class_area2d.md#body_shape_entered) |
| `signal body_shape_exited(body_rid: RID, body: Node2D, body_shape_index: int, local_shape_index: int)` | 对方形状离开时触发。 | 配合上者。 [类页锚点](../builtin/class_area2d.md#body_shape_exited) |

## Area3D（继承 CollisionObject3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal area_entered(area: Area3D)` | 另一 3D Area 进入时触发。 | 3D 区域重叠。 [类页锚点](../builtin/class_area3d.md#area_entered) |
| `signal area_exited(area: Area3D)` | 另一 3D Area 离开时触发。 | 重叠解除。 [类页锚点](../builtin/class_area3d.md#area_exited) |
| `signal area_shape_entered(area_rid: RID, area: Area3D, area_shape_index: int, local_shape_index: int)` | 对方 3D Area 形状进入时触发。 | 形状级区域检测。 [类页锚点](../builtin/class_area3d.md#area_shape_entered) |
| `signal area_shape_exited(area_rid: RID, area: Area3D, area_shape_index: int, local_shape_index: int)` | 对方 3D Area 形状离开时触发。 | 配合上者。 [类页锚点](../builtin/class_area3d.md#area_shape_exited) |
| `signal body_entered(body: Node3D)` | 3D 物理体进入本区域时触发，携带 Node3D。 | 3D 触发器，需 monitoring=true。 [类页锚点](../builtin/class_area3d.md#body_entered) |
| `signal body_exited(body: Node3D)` | 3D 物理体离开时触发。 | 离开结算。 [类页锚点](../builtin/class_area3d.md#body_exited) |
| `signal body_shape_entered(body_rid: RID, body: Node3D, body_shape_index: int, local_shape_index: int)` | 3D 对方形状进入时触发，带 RID 与形状索引。 | 形状级 3D 检测。 [类页锚点](../builtin/class_area3d.md#body_shape_entered) |
| `signal body_shape_exited(body_rid: RID, body: Node3D, body_shape_index: int, local_shape_index: int)` | 3D 对方形状离开时触发。 | 配合上者。 [类页锚点](../builtin/class_area3d.md#body_shape_exited) |

## CollisionObject2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal input_event(viewport: Node, event: InputEvent, shape_idx: int)` | 指针/输入在本碰撞体上按下时触发，携带事件与形状索引。 | 2D 可点物体拾取。 [类页锚点](../builtin/class_collisionobject2d.md#input_event) |
| `signal mouse_entered()` | 鼠标悬停到碰撞体时触发。 | 2D hover。 [类页锚点](../builtin/class_collisionobject2d.md#mouse_entered) |
| `signal mouse_exited()` | 鼠标离开碰撞体时触发。 | 配合上者。 [类页锚点](../builtin/class_collisionobject2d.md#mouse_exited) |
| `signal mouse_shape_entered(shape_idx: int)` | 鼠标进入某形状时触发，带形状索引。 | 分形状 hover。 [类页锚点](../builtin/class_collisionobject2d.md#mouse_shape_entered) |
| `signal mouse_shape_exited(shape_idx: int)` | 鼠标离开某形状时触发。 | 配合上者。 [类页锚点](../builtin/class_collisionobject2d.md#mouse_shape_exited) |

## CollisionObject3D（继承 Node3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal input_event(camera: Node, event: InputEvent, event_position: Vector3, normal: Vector3, shape_idx: int)` | 指针在 3D 碰撞体上按下时触发。 | 3D 点击拾取。 [类页锚点](../builtin/class_collisionobject3d.md#input_event) |
| `signal mouse_entered()` | 鼠标悬停 3D 碰撞体时触发。 | 3D hover。 [类页锚点](../builtin/class_collisionobject3d.md#mouse_entered) |
| `signal mouse_exited()` | 鼠标离开 3D 碰撞体时触发。 | 配合上者。 [类页锚点](../builtin/class_collisionobject3d.md#mouse_exited) |

## RigidBody2D（继承 PhysicsBody2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal body_entered(body: Node)` | 另一个刚体进入本刚体的区域通知范围时触发。 | 刚体间碰撞通知（非 Area）。 [类页锚点](../builtin/class_rigidbody2d.md#body_entered) |
| `signal body_exited(body: Node)` | 另一刚体离开时触发。 | 配合上者。 [类页锚点](../builtin/class_rigidbody2d.md#body_exited) |
| `signal body_shape_entered(body_rid: RID, body: Node, body_shape_index: int, local_shape_index: int)` | 本刚体某形状与另一物理体/TileMap 形状碰撞开始时触发，带 RID 与形状索引。 | 形状级碰撞开始。 [类页锚点](../builtin/class_rigidbody2d.md#body_shape_entered) |
| `signal body_shape_exited(body_rid: RID, body: Node, body_shape_index: int, local_shape_index: int)` | 上述形状碰撞结束时触发。 | 形状级碰撞结束。 [类页锚点](../builtin/class_rigidbody2d.md#body_shape_exited) |
| `signal sleeping_state_changed()` | 刚体进入/退出休眠时触发。 | 休眠唤醒钩子。 [类页锚点](../builtin/class_rigidbody2d.md#sleeping_state_changed) |

## RigidBody3D（继承 PhysicsBody3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal body_entered(body: Node)` | 另一 3D 刚体进入通知范围时触发。 | 3D 刚体通知。 [类页锚点](../builtin/class_rigidbody3d.md#body_entered) |
| `signal body_exited(body: Node)` | 另一 3D 刚体离开时触发。 | 配合上者。 [类页锚点](../builtin/class_rigidbody3d.md#body_exited) |
| `signal body_shape_entered(body_rid: RID, body: Node, body_shape_index: int, local_shape_index: int)` | 本 3D 刚体某形状与另一物理体/GridMap 形状碰撞开始时触发。 | 3D 形状级碰撞开始。 [类页锚点](../builtin/class_rigidbody3d.md#body_shape_entered) |
| `signal body_shape_exited(body_rid: RID, body: Node, body_shape_index: int, local_shape_index: int)` | 3D 形状碰撞结束时触发。 | 配合上者。 [类页锚点](../builtin/class_rigidbody3d.md#body_shape_exited) |
| `signal sleeping_state_changed()` | 3D 刚体休眠状态改变时触发。 | 休眠唤醒钩子。 [类页锚点](../builtin/class_rigidbody3d.md#sleeping_state_changed) |

## VisibleOnScreenNotifier2D（继承 Node2D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal screen_entered()` | 节点进入可视屏幕时触发。 | 离屏裁剪、只渲染可见物。 [类页锚点](../builtin/class_visibleonscreennotifier2d.md#screen_entered) |
| `signal screen_exited()` | 节点离开可视屏幕时触发。 | 回收离屏对象。 [类页锚点](../builtin/class_visibleonscreennotifier2d.md#screen_exited) |

## VisibleOnScreenNotifier3D（继承 VisualInstance3D）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal screen_entered()` | 3D 节点进入相机视锥时触发。 | 3D 可见剔除。 [类页锚点](../builtin/class_visibleonscreennotifier3d.md#screen_entered) |
| `signal screen_exited()` | 3D 节点离开视锥时触发。 | 配合上者。 [类页锚点](../builtin/class_visibleonscreennotifier3d.md#screen_exited) |

<!-- SIGNALS:END -->
