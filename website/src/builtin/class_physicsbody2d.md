# PhysicsBody2D

**继承**：[CollisionObject2D](class_collisionobject2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **PhysicsBody2D**

> PhysicsBody2D is an abstract base class for 2D game objects affected by physics. All 2D physics bodies inherit from it.

<details><summary>官方英文描述（中文精译进行中）</summary>

PhysicsBody2D is an abstract base class for 2D game objects affected by physics. All 2D physics bodies inherit from it.

</details>

## 方法

### add_collision_exception_with {#add_collision_exception_with}

```gdscript
void add_collision_exception_with(body: Node)
```

Adds a body to the list of bodies that this body can't collide with.

### get_collision_exceptions {#get_collision_exceptions}

```gdscript
typedarray::PhysicsBody2D get_collision_exceptions()
```

Returns an array of nodes that were added as collision exceptions for this body.

### get_gravity {#get_gravity}

```gdscript
Vector2 get_gravity() const
```

Returns the gravity vector computed from all sources that can affect the body, including all gravity overrides from Area2D nodes and the global world gravity.

### move_and_collide {#move_and_collide}

```gdscript
KinematicCollision2D move_and_collide(motion: Vector2, test_only: bool = false, safe_margin: float = 0.08, recovery_as_collision: bool = false)
```

Moves the body along the vector motion. In order to be frame rate independent in Node._physics_process() or Node._process(), motion should be computed using delta.

Returns a KinematicCollision2D, which contains information about the collision when stopped, or when touching another body along the motion.

If test_only is true, the body does not move but the would-be collision information is given.

safe_margin is the extra margin used for collision recovery (see CharacterBody2D.safe_margin for more details).

If recovery_as_collision is true, any depenetration from the recovery phase is also reported as a collision; this is used e.g. by CharacterBody2D for improving floor detection during floor snapping.

### remove_collision_exception_with {#remove_collision_exception_with}

```gdscript
void remove_collision_exception_with(body: Node)
```

Removes a body from the list of bodies that this body can't collide with.

### test_move {#test_move}

```gdscript
bool test_move(from: Transform2D, motion: Vector2, collision: KinematicCollision2D = null, safe_margin: float = 0.08, recovery_as_collision: bool = false)
```

Checks for collisions without moving the body. In order to be frame rate independent in Node._physics_process() or Node._process(), motion should be computed using delta.

Virtually sets the node's position, scale and rotation to that of the given Transform2D, then tries to move the body along the vector motion. Returns true if a collision would stop the body from moving along the whole path.

collision is an optional object of type KinematicCollision2D, which contains additional information about the collision when stopped, or when touching another body along the motion.

safe_margin is the extra margin used for collision recovery (see CharacterBody2D.safe_margin for more details).

If recovery_as_collision is true, any depenetration from the recovery phase is also reported as a collision; this is useful for checking whether the body would touch any other bodies.
