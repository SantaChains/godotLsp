# CharacterBody3D

**继承**：[PhysicsBody3D](class_physicsbody3d.md) → [CollisionObject3D](class_collisionobject3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CharacterBody3D**

> CharacterBody3D is a specialized class for physics bodies that are meant to be user-controlled. They are not affected by physics at all, but they affect other physics bodies in their path. They are mainly used to provide high-level API to move objects with wall and slope detection (move_and_slide() method) in addition to the general collision detection provided by PhysicsBody3D.move_and_collide(). This makes it useful for highly configurable physics bodies that must move in specific ways and collide with the world, as is often the case with user-controlled characters.

<details><summary>官方英文描述</summary>

CharacterBody3D is a specialized class for physics bodies that are meant to be user-controlled. They are not affected by physics at all, but they affect other physics bodies in their path. They are mainly used to provide high-level API to move objects with wall and slope detection (move_and_slide() method) in addition to the general collision detection provided by PhysicsBody3D.move_and_collide(). This makes it useful for highly configurable physics bodies that must move in specific ways and collide with the world, as is often the case with user-controlled characters.

For game objects that don't require complex movement or collision detection, such as moving platforms, AnimatableBody3D is simpler to configure.

</details>

## 方法

### apply_floor_snap {#apply_floor_snap}

```gdscript
void apply_floor_snap()
```

Allows to manually apply a snap to the floor regardless of the body's velocity. This function does nothing when is_on_floor() returns true.

### get_floor_angle {#get_floor_angle}

```gdscript
float get_floor_angle(up_direction: Vector3 = Vector3(0, 1, 0)) const
```

Returns the floor's collision angle at the last collision point according to up_direction, which is Vector3.UP by default. This value is always positive and only valid after calling move_and_slide() and when is_on_floor() returns true.

### get_floor_max_angle {#get_floor_max_angle}

```gdscript
float get_floor_max_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_floor_normal {#get_floor_normal}

```gdscript
Vector3 get_floor_normal() const
```

Returns the collision normal of the floor at the last collision point. Only valid after calling move_and_slide() and when is_on_floor() returns true.

Warning: The collision normal is not always the same as the surface normal.

### get_floor_snap_length {#get_floor_snap_length}

```gdscript
float get_floor_snap_length()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_last_motion {#get_last_motion}

```gdscript
Vector3 get_last_motion() const
```

Returns the last motion applied to the CharacterBody3D during the last call to move_and_slide(). The movement can be split into multiple motions when sliding occurs, and this method return the last one, which is useful to retrieve the current direction of the movement.

### get_last_slide_collision {#get_last_slide_collision}

```gdscript
KinematicCollision3D get_last_slide_collision()
```

Returns a KinematicCollision3D if a collision occurred. The returned value contains information about the latest collision that occurred during the last call to move_and_slide(). Returns null if no collision occurred. See also get_slide_collision().

### get_max_slides {#get_max_slides}

```gdscript
int get_max_slides() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_motion_mode {#get_motion_mode}

```gdscript
CharacterBody3D.MotionMode get_motion_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_platform_angular_velocity {#get_platform_angular_velocity}

```gdscript
Vector3 get_platform_angular_velocity() const
```

Returns the angular velocity of the platform at the last collision point. Only valid after calling move_and_slide().

### get_platform_floor_layers {#get_platform_floor_layers}

```gdscript
int get_platform_floor_layers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_platform_on_leave {#get_platform_on_leave}

```gdscript
CharacterBody3D.PlatformOnLeave get_platform_on_leave() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_platform_velocity {#get_platform_velocity}

```gdscript
Vector3 get_platform_velocity() const
```

Returns the linear velocity of the platform at the last collision point. Only valid after calling move_and_slide().

### get_platform_wall_layers {#get_platform_wall_layers}

```gdscript
int get_platform_wall_layers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position_delta {#get_position_delta}

```gdscript
Vector3 get_position_delta() const
```

Returns the travel (position delta) that occurred during the last call to move_and_slide().

### get_real_velocity {#get_real_velocity}

```gdscript
Vector3 get_real_velocity() const
```

Returns the current real velocity since the last call to move_and_slide(). For example, when you climb a slope, you will move diagonally even though the velocity is horizontal. This method returns the diagonal movement, as opposed to velocity which returns the requested velocity.

### get_safe_margin {#get_safe_margin}

```gdscript
float get_safe_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_slide_collision {#get_slide_collision}

```gdscript
KinematicCollision3D get_slide_collision(slide_idx: int)
```

Returns a KinematicCollision3D, which contains information about a collision that occurred during the last call to move_and_slide(). Since the body can collide several times in a single call to move_and_slide(), you must specify the index of the collision in the range 0 to (get_slide_collision_count() - 1). See also get_last_slide_collision().

### get_slide_collision_count {#get_slide_collision_count}

```gdscript
int get_slide_collision_count() const
```

Returns the number of times the body collided and changed direction during the last call to move_and_slide().

### get_up_direction {#get_up_direction}

```gdscript
Vector3 get_up_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_velocity {#get_velocity}

```gdscript
Vector3 get_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wall_min_slide_angle {#get_wall_min_slide_angle}

```gdscript
float get_wall_min_slide_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wall_normal {#get_wall_normal}

```gdscript
Vector3 get_wall_normal() const
```

Returns the collision normal of the wall at the last collision point. Only valid after calling move_and_slide() and when is_on_wall() returns true.

Warning: The collision normal is not always the same as the surface normal.

### is_floor_block_on_wall_enabled {#is_floor_block_on_wall_enabled}

```gdscript
bool is_floor_block_on_wall_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_floor_constant_speed_enabled {#is_floor_constant_speed_enabled}

```gdscript
bool is_floor_constant_speed_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_floor_stop_on_slope_enabled {#is_floor_stop_on_slope_enabled}

```gdscript
bool is_floor_stop_on_slope_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_on_ceiling {#is_on_ceiling}

```gdscript
bool is_on_ceiling() const
```

Returns true if the body collided with the ceiling on the last call of move_and_slide(). Otherwise, returns false. The up_direction and floor_max_angle are used to determine whether a surface is "ceiling" or not.

### is_on_ceiling_only {#is_on_ceiling_only}

```gdscript
bool is_on_ceiling_only() const
```

Returns true if the body collided only with the ceiling on the last call of move_and_slide(). Otherwise, returns false. The up_direction and floor_max_angle are used to determine whether a surface is "ceiling" or not.

### is_on_floor {#is_on_floor}

```gdscript
bool is_on_floor() const
```

Returns true if the body collided with the floor on the last call of move_and_slide(). Otherwise, returns false. The up_direction and floor_max_angle are used to determine whether a surface is "floor" or not.

### is_on_floor_only {#is_on_floor_only}

```gdscript
bool is_on_floor_only() const
```

Returns true if the body collided only with the floor on the last call of move_and_slide(). Otherwise, returns false. The up_direction and floor_max_angle are used to determine whether a surface is "floor" or not.

### is_on_wall {#is_on_wall}

```gdscript
bool is_on_wall() const
```

Returns true if the body collided with a wall on the last call of move_and_slide(). Otherwise, returns false. The up_direction and floor_max_angle are used to determine whether a surface is "wall" or not.

### is_on_wall_only {#is_on_wall_only}

```gdscript
bool is_on_wall_only() const
```

Returns true if the body collided only with a wall on the last call of move_and_slide(). Otherwise, returns false. The up_direction and floor_max_angle are used to determine whether a surface is "wall" or not.

### is_slide_on_ceiling_enabled {#is_slide_on_ceiling_enabled}

```gdscript
bool is_slide_on_ceiling_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### move_and_slide {#move_and_slide}

```gdscript
bool move_and_slide()
```

Moves the body based on velocity. If the body collides with another, it will slide along the other body rather than stop immediately. If the other body is a CharacterBody3D or RigidBody3D, it will also be affected by the motion of the other body. You can use this to make moving and rotating platforms, or to make nodes push other nodes.

This method should be used in Node._physics_process() (or in a method called by Node._physics_process()), as it uses the physics step's delta value automatically in calculations. Otherwise, the simulation will run at an incorrect speed.

Modifies velocity if a slide collision occurred. To get the latest collision call get_last_slide_collision(), for more detailed information about collisions that occurred, use get_slide_collision().

When the body touches a moving platform, the platform's velocity is automatically added to the body motion. If a collision occurs due to the platform's motion, it will always be first in the slide collisions.

Returns true if the body collided, otherwise, returns false.

### set_floor_block_on_wall_enabled {#set_floor_block_on_wall_enabled}

```gdscript
void set_floor_block_on_wall_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_floor_constant_speed_enabled {#set_floor_constant_speed_enabled}

```gdscript
void set_floor_constant_speed_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_floor_max_angle {#set_floor_max_angle}

```gdscript
void set_floor_max_angle(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_floor_snap_length {#set_floor_snap_length}

```gdscript
void set_floor_snap_length(floor_snap_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_floor_stop_on_slope_enabled {#set_floor_stop_on_slope_enabled}

```gdscript
void set_floor_stop_on_slope_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_slides {#set_max_slides}

```gdscript
void set_max_slides(max_slides: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motion_mode {#set_motion_mode}

```gdscript
void set_motion_mode(mode: CharacterBody3D.MotionMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_platform_floor_layers {#set_platform_floor_layers}

```gdscript
void set_platform_floor_layers(exclude_layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_platform_on_leave {#set_platform_on_leave}

```gdscript
void set_platform_on_leave(on_leave_apply_velocity: CharacterBody3D.PlatformOnLeave)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_platform_wall_layers {#set_platform_wall_layers}

```gdscript
void set_platform_wall_layers(exclude_layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_safe_margin {#set_safe_margin}

```gdscript
void set_safe_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_slide_on_ceiling_enabled {#set_slide_on_ceiling_enabled}

```gdscript
void set_slide_on_ceiling_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_up_direction {#set_up_direction}

```gdscript
void set_up_direction(up_direction: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity {#set_velocity}

```gdscript
void set_velocity(velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wall_min_slide_angle {#set_wall_min_slide_angle}

```gdscript
void set_wall_min_slide_angle(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### motion_mode {#motion_mode}

```gdscript
motion_mode: int
```

- void set_motion_mode(value: MotionMode)
- MotionMode get_motion_mode()

Sets the motion mode which defines the behavior of move_and_slide().

### up_direction {#up_direction}

```gdscript
up_direction: Vector3
```

- void set_up_direction(value: Vector3)
- Vector3 get_up_direction()

Vector pointing upwards, used to determine what is a wall and what is a floor (or a ceiling) when calling move_and_slide(). Defaults to Vector3.UP. As the vector will be normalized it can't be equal to Vector3.ZERO, if you want all collisions to be reported as walls, consider using MOTION_MODE_FLOATING as motion_mode.

### slide_on_ceiling {#slide_on_ceiling}

```gdscript
slide_on_ceiling: bool
```

- void set_slide_on_ceiling_enabled(value: bool)
- bool is_slide_on_ceiling_enabled()

If true, during a jump against the ceiling, the body will slide, if false it will be stopped and will fall vertically.

### velocity {#velocity}

```gdscript
velocity: Vector3
```

- void set_velocity(value: Vector3)
- Vector3 get_velocity()

Current velocity vector (typically meters per second), used and modified during calls to move_and_slide().

Note: A common mistake is setting this property to the desired velocity multiplied by delta, which produces a motion vector (typically in meters).

### max_slides {#max_slides}

```gdscript
max_slides: int
```

- void set_max_slides(value: int)
- int get_max_slides()

Maximum number of times the body can change direction before it stops when calling move_and_slide(). Must be greater than zero.

### wall_min_slide_angle {#wall_min_slide_angle}

```gdscript
wall_min_slide_angle: float
```

- void set_wall_min_slide_angle(value: float)
- float get_wall_min_slide_angle()

Minimum angle (in radians) where the body is allowed to slide when it encounters a wall. The default value equals 15 degrees. When motion_mode is MOTION_MODE_GROUNDED, it only affects movement if floor_block_on_wall is true.

### floor_stop_on_slope {#floor_stop_on_slope}

```gdscript
floor_stop_on_slope: bool
```

- void set_floor_stop_on_slope_enabled(value: bool)
- bool is_floor_stop_on_slope_enabled()

If true, the body will not slide on slopes when calling move_and_slide() when the body is standing still.

If false, the body will slide on floor's slopes when velocity applies a downward force.

### floor_constant_speed {#floor_constant_speed}

```gdscript
floor_constant_speed: bool
```

- void set_floor_constant_speed_enabled(value: bool)
- bool is_floor_constant_speed_enabled()

If false (by default), the body will move faster on downward slopes and slower on upward slopes.

If true, the body will always move at the same speed on the ground no matter the slope. Note that you need to use floor_snap_length to stick along a downward slope at constant speed.

### floor_block_on_wall {#floor_block_on_wall}

```gdscript
floor_block_on_wall: bool
```

- void set_floor_block_on_wall_enabled(value: bool)
- bool is_floor_block_on_wall_enabled()

If true, the body will be able to move on the floor only. This option avoids to be able to walk on walls, it will however allow to slide down along them.

### floor_max_angle {#floor_max_angle}

```gdscript
floor_max_angle: float
```

- void set_floor_max_angle(value: float)
- float get_floor_max_angle()

Maximum angle (in radians) where a slope is still considered a floor (or a ceiling), rather than a wall, when calling move_and_slide(). The default value equals 45 degrees.

### floor_snap_length {#floor_snap_length}

```gdscript
floor_snap_length: float
```

- void set_floor_snap_length(value: float)
- float get_floor_snap_length()

Sets a snapping distance. When set to a value different from 0.0, the body is kept attached to slopes when calling move_and_slide(). The snapping vector is determined by the given distance along the opposite direction of the up_direction.

As long as the snapping vector is in contact with the ground and the body moves against up_direction, the body will remain attached to the surface. Snapping is not applied if the body moves along up_direction, meaning it contains vertical rising velocity, so it will be able to detach from the ground when jumping or when the body is pushed up by something. If you want to apply a snap without taking into account the velocity, use apply_floor_snap().

### platform_on_leave {#platform_on_leave}

```gdscript
platform_on_leave: int
```

- void set_platform_on_leave(value: PlatformOnLeave)
- PlatformOnLeave get_platform_on_leave()

Sets the behavior to apply when you leave a moving platform. By default, to be physically accurate, when you leave the last platform velocity is applied.

### platform_floor_layers {#platform_floor_layers}

```gdscript
platform_floor_layers: int
```

- void set_platform_floor_layers(value: int)
- int get_platform_floor_layers()

Collision layers that will be included for detecting floor bodies that will act as moving platforms to be followed by the CharacterBody3D. By default, all floor bodies are detected and propagate their velocity.

### platform_wall_layers {#platform_wall_layers}

```gdscript
platform_wall_layers: int
```

- void set_platform_wall_layers(value: int)
- int get_platform_wall_layers()

Collision layers that will be included for detecting wall bodies that will act as moving platforms to be followed by the CharacterBody3D. By default, all wall bodies are ignored.

### safe_margin {#safe_margin}

```gdscript
safe_margin: float
```

- void set_safe_margin(value: float)
- float get_safe_margin()

Extra margin used for collision recovery when calling move_and_slide().

If the body is at least this close to another body, it will consider them to be colliding and will be pushed away before performing the actual motion.

A higher value means it's more flexible for detecting collision, which helps with consistently detecting walls and floors.

A lower value forces the collision algorithm to use more exact detection, so it can be used in cases that specifically require precision, e.g at very low scale to avoid visible jittering, or for stability with a stack of character bodies.

## 枚举

### MotionMode {#enum-MotionMode}

```gdscript
enum MotionMode { MOTION_MODE_GROUNDED = 0, MOTION_MODE_FLOATING = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MOTION_MODE_GROUNDED {#MOTION_MODE_GROUNDED}

```gdscript
MOTION_MODE_GROUNDED = 0
```

Apply when notions of walls, ceiling and floor are relevant. In this mode the body motion will react to slopes (acceleration/slowdown). This mode is suitable for grounded games like platformers.

#### MOTION_MODE_FLOATING {#MOTION_MODE_FLOATING}

```gdscript
MOTION_MODE_FLOATING = 1
```

Apply when there is no notion of floor or ceiling. All collisions will be reported as on_wall. In this mode, when you slide, the speed will always be constant. This mode is suitable for games without ground like space games.

---

### PlatformOnLeave {#enum-PlatformOnLeave}

```gdscript
enum PlatformOnLeave { PLATFORM_ON_LEAVE_ADD_VELOCITY = 0, PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY = 1, PLATFORM_ON_LEAVE_DO_NOTHING = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLATFORM_ON_LEAVE_ADD_VELOCITY {#PLATFORM_ON_LEAVE_ADD_VELOCITY}

```gdscript
PLATFORM_ON_LEAVE_ADD_VELOCITY = 0
```

Add the last platform velocity to the velocity when you leave a moving platform.

#### PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY {#PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY}

```gdscript
PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY = 1
```

Add the last platform velocity to the velocity when you leave a moving platform, but any downward motion is ignored. It's useful to keep full jump height even when the platform is moving down.

#### PLATFORM_ON_LEAVE_DO_NOTHING {#PLATFORM_ON_LEAVE_DO_NOTHING}

```gdscript
PLATFORM_ON_LEAVE_DO_NOTHING = 2
```

Do nothing when leaving a platform.

---
