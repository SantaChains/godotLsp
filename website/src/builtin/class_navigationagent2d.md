# NavigationAgent2D

**继承**：[Node](class_node.md) → [Object](class_object.md) → **NavigationAgent2D**

> A 2D agent used to pathfind to a position while avoiding static and dynamic obstacles. The calculation can be used by the parent node to dynamically move it along the path. Requires navigation data to work correctly.

<details><summary>官方英文描述</summary>

A 2D agent used to pathfind to a position while avoiding static and dynamic obstacles. The calculation can be used by the parent node to dynamically move it along the path. Requires navigation data to work correctly.

Dynamic obstacles are avoided using RVO collision avoidance. Avoidance is computed before physics, so the pathfinding information can be used safely in the physics step.

Note: After setting the target_position property, the get_next_path_position() method must be used once every physics frame to update the internal path logic of the navigation agent. The vector position it returns should be used as the next movement position for the agent's parent node.

Note: Several methods of this class, such as get_next_path_position(), can trigger a new path calculation. Calling these in your callback to an agent's signal, such as waypoint_reached, can cause infinite recursion. It is recommended to call these methods in the physics step or, alternatively, delay their call until the end of the frame (see Object.call_deferred() or Object.CONNECT_DEFERRED).

</details>

## 方法

### distance_to_target {#distance_to_target}

```gdscript
float distance_to_target() const
```

Returns the distance to the target position, using the agent's global position. The user must set target_position in order for this to be accurate.

### get_avoidance_enabled {#get_avoidance_enabled}

```gdscript
bool get_avoidance_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_avoidance_layer_value {#get_avoidance_layer_value}

```gdscript
bool get_avoidance_layer_value(layer_number: int) const
```

Returns whether or not the specified layer of the avoidance_layers bitmask is enabled, given a layer_number between 1 and 32.

### get_avoidance_layers {#get_avoidance_layers}

```gdscript
int get_avoidance_layers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_avoidance_mask {#get_avoidance_mask}

```gdscript
int get_avoidance_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_avoidance_mask_value {#get_avoidance_mask_value}

```gdscript
bool get_avoidance_mask_value(mask_number: int) const
```

Returns whether or not the specified mask of the avoidance_mask bitmask is enabled, given a mask_number between 1 and 32.

### get_avoidance_priority {#get_avoidance_priority}

```gdscript
float get_avoidance_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_navigation_path {#get_current_navigation_path}

```gdscript
PackedVector2Array get_current_navigation_path() const
```

Returns this agent's current path from start to finish in global coordinates. The path only updates when the target position is changed or the agent requires a repath. The path array is not intended to be used in direct path movement as the agent has its own internal path logic that would get corrupted by changing the path array manually. Use the intended get_next_path_position() once every physics frame to receive the next path point for the agents movement as this function also updates the internal path logic.

### get_current_navigation_path_index {#get_current_navigation_path_index}

```gdscript
int get_current_navigation_path_index() const
```

Returns which index the agent is currently on in the navigation path's PackedVector2Array.

### get_current_navigation_result {#get_current_navigation_result}

```gdscript
NavigationPathQueryResult2D get_current_navigation_result() const
```

Returns the path query result for the path the agent is currently following.

### get_debug_enabled {#get_debug_enabled}

```gdscript
bool get_debug_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_path_custom_color {#get_debug_path_custom_color}

```gdscript
Color get_debug_path_custom_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_path_custom_line_width {#get_debug_path_custom_line_width}

```gdscript
float get_debug_path_custom_line_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_path_custom_point_size {#get_debug_path_custom_point_size}

```gdscript
float get_debug_path_custom_point_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_use_custom {#get_debug_use_custom}

```gdscript
bool get_debug_use_custom() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_final_position {#get_final_position}

```gdscript
Vector2 get_final_position()
```

Returns the reachable final position of the current navigation path in global coordinates. This position can change if the agent needs to update the navigation path which makes the agent emit the path_changed signal.

### get_max_neighbors {#get_max_neighbors}

```gdscript
int get_max_neighbors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_speed {#get_max_speed}

```gdscript
float get_max_speed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_layer_value {#get_navigation_layer_value}

```gdscript
bool get_navigation_layer_value(layer_number: int) const
```

Returns whether or not the specified layer of the navigation_layers bitmask is enabled, given a layer_number between 1 and 32.

### get_navigation_layers {#get_navigation_layers}

```gdscript
int get_navigation_layers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

Returns the RID of the navigation map for this NavigationAgent node. This function returns always the map set on the NavigationAgent node and not the map of the abstract agent on the NavigationServer. If the agent map is changed directly with the NavigationServer API the NavigationAgent node will not be aware of the map change. Use set_navigation_map() to change the navigation map for the NavigationAgent and also update the agent on the NavigationServer.

### get_neighbor_distance {#get_neighbor_distance}

```gdscript
float get_neighbor_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_next_path_position {#get_next_path_position}

```gdscript
Vector2 get_next_path_position()
```

Returns the next position in global coordinates that can be moved to, making sure that there are no static objects in the way. If the agent does not have a navigation path, it will return the position of the agent's parent. The use of this function once every physics frame is required to update the internal path logic of the NavigationAgent.

### get_path_desired_distance {#get_path_desired_distance}

```gdscript
float get_path_desired_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_length {#get_path_length}

```gdscript
float get_path_length() const
```

Returns the length of the currently calculated path. The returned value is 0.0, if the path is still calculating or no calculation has been requested yet.

### get_path_max_distance {#get_path_max_distance}

```gdscript
float get_path_max_distance()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_metadata_flags {#get_path_metadata_flags}

```gdscript
bitfield::NavigationPathQueryParameters2D.PathMetadataFlags get_path_metadata_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_postprocessing {#get_path_postprocessing}

```gdscript
NavigationPathQueryParameters2D.PathPostProcessing get_path_postprocessing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_return_max_length {#get_path_return_max_length}

```gdscript
float get_path_return_max_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_return_max_radius {#get_path_return_max_radius}

```gdscript
float get_path_return_max_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_search_max_distance {#get_path_search_max_distance}

```gdscript
float get_path_search_max_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path_search_max_polygons {#get_path_search_max_polygons}

```gdscript
int get_path_search_max_polygons() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pathfinding_algorithm {#get_pathfinding_algorithm}

```gdscript
NavigationPathQueryParameters2D.PathfindingAlgorithm get_pathfinding_algorithm() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the RID of this agent on the NavigationServer2D.

### get_simplify_epsilon {#get_simplify_epsilon}

```gdscript
float get_simplify_epsilon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_simplify_path {#get_simplify_path}

```gdscript
bool get_simplify_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_desired_distance {#get_target_desired_distance}

```gdscript
float get_target_desired_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_position {#get_target_position}

```gdscript
Vector2 get_target_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_time_horizon_agents {#get_time_horizon_agents}

```gdscript
float get_time_horizon_agents() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_time_horizon_obstacles {#get_time_horizon_obstacles}

```gdscript
float get_time_horizon_obstacles() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_velocity {#get_velocity}

```gdscript
Vector2 get_velocity()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_navigation_finished {#is_navigation_finished}

```gdscript
bool is_navigation_finished()
```

Returns true if the agent's navigation has finished. If the target is reachable, navigation ends when the target is reached. If the target is unreachable, navigation ends when the last waypoint of the path is reached.

Note: While true prefer to stop calling update functions like get_next_path_position(). This avoids jittering the standing agent due to calling repeated path updates.

### is_target_reachable {#is_target_reachable}

```gdscript
bool is_target_reachable()
```

Returns true if get_final_position() is within target_desired_distance of the target_position.

### is_target_reached {#is_target_reached}

```gdscript
bool is_target_reached() const
```

Returns true if the agent reached the target, i.e. the agent moved within target_desired_distance of the target_position. It may not always be possible to reach the target but it should always be possible to reach the final position. See get_final_position().

### set_avoidance_enabled {#set_avoidance_enabled}

```gdscript
void set_avoidance_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_avoidance_layer_value {#set_avoidance_layer_value}

```gdscript
void set_avoidance_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the avoidance_layers bitmask, given a layer_number between 1 and 32.

### set_avoidance_layers {#set_avoidance_layers}

```gdscript
void set_avoidance_layers(layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_avoidance_mask {#set_avoidance_mask}

```gdscript
void set_avoidance_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_avoidance_mask_value {#set_avoidance_mask_value}

```gdscript
void set_avoidance_mask_value(mask_number: int, value: bool)
```

Based on value, enables or disables the specified mask in the avoidance_mask bitmask, given a mask_number between 1 and 32.

### set_avoidance_priority {#set_avoidance_priority}

```gdscript
void set_avoidance_priority(priority: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_enabled {#set_debug_enabled}

```gdscript
void set_debug_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_path_custom_color {#set_debug_path_custom_color}

```gdscript
void set_debug_path_custom_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_path_custom_line_width {#set_debug_path_custom_line_width}

```gdscript
void set_debug_path_custom_line_width(line_width: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_path_custom_point_size {#set_debug_path_custom_point_size}

```gdscript
void set_debug_path_custom_point_size(point_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_use_custom {#set_debug_use_custom}

```gdscript
void set_debug_use_custom(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_neighbors {#set_max_neighbors}

```gdscript
void set_max_neighbors(max_neighbors: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_speed {#set_max_speed}

```gdscript
void set_max_speed(max_speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_layer_value {#set_navigation_layer_value}

```gdscript
void set_navigation_layer_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the navigation_layers bitmask, given a layer_number between 1 and 32.

### set_navigation_layers {#set_navigation_layers}

```gdscript
void set_navigation_layers(navigation_layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_navigation_map {#set_navigation_map}

```gdscript
void set_navigation_map(navigation_map: RID)
```

Sets the RID of the navigation map this NavigationAgent node should use and also updates the agent on the NavigationServer.

### set_neighbor_distance {#set_neighbor_distance}

```gdscript
void set_neighbor_distance(neighbor_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_desired_distance {#set_path_desired_distance}

```gdscript
void set_path_desired_distance(desired_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_max_distance {#set_path_max_distance}

```gdscript
void set_path_max_distance(max_speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_metadata_flags {#set_path_metadata_flags}

```gdscript
void set_path_metadata_flags(flags: bitfield::NavigationPathQueryParameters2D.PathMetadataFlags)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_postprocessing {#set_path_postprocessing}

```gdscript
void set_path_postprocessing(path_postprocessing: NavigationPathQueryParameters2D.PathPostProcessing)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_return_max_length {#set_path_return_max_length}

```gdscript
void set_path_return_max_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_return_max_radius {#set_path_return_max_radius}

```gdscript
void set_path_return_max_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_search_max_distance {#set_path_search_max_distance}

```gdscript
void set_path_search_max_distance(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_search_max_polygons {#set_path_search_max_polygons}

```gdscript
void set_path_search_max_polygons(max_polygons: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pathfinding_algorithm {#set_pathfinding_algorithm}

```gdscript
void set_pathfinding_algorithm(pathfinding_algorithm: NavigationPathQueryParameters2D.PathfindingAlgorithm)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_simplify_epsilon {#set_simplify_epsilon}

```gdscript
void set_simplify_epsilon(epsilon: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_simplify_path {#set_simplify_path}

```gdscript
void set_simplify_path(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_desired_distance {#set_target_desired_distance}

```gdscript
void set_target_desired_distance(desired_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_position {#set_target_position}

```gdscript
void set_target_position(position: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_time_horizon_agents {#set_time_horizon_agents}

```gdscript
void set_time_horizon_agents(time_horizon: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_time_horizon_obstacles {#set_time_horizon_obstacles}

```gdscript
void set_time_horizon_obstacles(time_horizon: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity {#set_velocity}

```gdscript
void set_velocity(velocity: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity_forced {#set_velocity_forced}

```gdscript
void set_velocity_forced(velocity: Vector2)
```

Replaces the internal velocity in the collision avoidance simulation with velocity. When an agent is teleported to a new position this function should be used in the same frame. If called frequently this function can get agents stuck.

## 属性

### target_position {#target_position}

```gdscript
target_position: Vector2
```

- void set_target_position(value: Vector2)
- Vector2 get_target_position()

If set, a new navigation path from the current agent position to the target_position is requested from the NavigationServer.

### path_desired_distance {#path_desired_distance}

```gdscript
path_desired_distance: float
```

- void set_path_desired_distance(value: float)
- float get_path_desired_distance()

The distance threshold before a path point is considered to be reached. This allows agents to not have to hit a path point on the path exactly, but only to reach its general area. If this value is set too high, the NavigationAgent will skip points on the path, which can lead to it leaving the navigation mesh. If this value is set too low, the NavigationAgent will be stuck in a repath loop because it will constantly overshoot the distance to the next point on each physics frame update.

### target_desired_distance {#target_desired_distance}

```gdscript
target_desired_distance: float
```

- void set_target_desired_distance(value: float)
- float get_target_desired_distance()

The distance threshold before the target is considered to be reached. On reaching the target, target_reached is emitted and navigation ends (see is_navigation_finished() and navigation_finished).

You can make navigation end early by setting this property to a value greater than path_desired_distance (navigation will end before reaching the last waypoint).

You can also make navigation end closer to the target than each individual path position by setting this property to a value lower than path_desired_distance (navigation won't immediately end when reaching the last waypoint). However, if the value set is too low, the agent will be stuck in a repath loop because it will constantly overshoot the distance to the target on each physics frame update.

### path_max_distance {#path_max_distance}

```gdscript
path_max_distance: float
```

- void set_path_max_distance(value: float)
- float get_path_max_distance()

The maximum distance the agent is allowed away from the ideal path to the final position. This can happen due to trying to avoid collisions. When the maximum distance is exceeded, it recalculates the ideal path.

### navigation_layers {#navigation_layers}

```gdscript
navigation_layers: int
```

- void set_navigation_layers(value: int)
- int get_navigation_layers()

A bitfield determining which navigation layers of navigation regions this agent will use to calculate a path. Changing it during runtime will clear the current navigation path and generate a new one, according to the new navigation layers.

### pathfinding_algorithm {#pathfinding_algorithm}

```gdscript
pathfinding_algorithm: int
```

- void set_pathfinding_algorithm(value: PathfindingAlgorithm)
- PathfindingAlgorithm get_pathfinding_algorithm()

The pathfinding algorithm used in the path query.

### path_postprocessing {#path_postprocessing}

```gdscript
path_postprocessing: int
```

- void set_path_postprocessing(value: PathPostProcessing)
- PathPostProcessing get_path_postprocessing()

The path postprocessing applied to the raw path corridor found by the pathfinding_algorithm.

### path_metadata_flags {#path_metadata_flags}

```gdscript
path_metadata_flags: int
```

- void set_path_metadata_flags(value: BitField[PathMetadataFlags])
- BitField[PathMetadataFlags] get_path_metadata_flags()

Additional information to return with the navigation path.

### simplify_path {#simplify_path}

```gdscript
simplify_path: bool
```

- void set_simplify_path(value: bool)
- bool get_simplify_path()

If true a simplified version of the path will be returned with less critical path points removed. The simplification amount is controlled by simplify_epsilon. The simplification uses a variant of Ramer-Douglas-Peucker algorithm for curve point decimation.

Path simplification can be helpful to mitigate various path following issues that can arise with certain agent types and script behaviors. E.g. "steering" agents or avoidance in "open fields".

### simplify_epsilon {#simplify_epsilon}

```gdscript
simplify_epsilon: float
```

- void set_simplify_epsilon(value: float)
- float get_simplify_epsilon()

The path simplification amount in worlds units.

### path_return_max_length {#path_return_max_length}

```gdscript
path_return_max_length: float
```

- void set_path_return_max_length(value: float)
- float get_path_return_max_length()

The maximum allowed length of the returned path in world units. A path will be clipped when going over this length.

### path_return_max_radius {#path_return_max_radius}

```gdscript
path_return_max_radius: float
```

- void set_path_return_max_radius(value: float)
- float get_path_return_max_radius()

The maximum allowed radius in world units that the returned path can be from the path start. The path will be clipped when going over this radius. Compared to path_return_max_length, this allows the agent to go that much further, if they need to walk around a corner.

Note: This will perform a sphere clip considering only the actual navigation mesh path points with the first path position being the sphere's center.

### path_search_max_polygons {#path_search_max_polygons}

```gdscript
path_search_max_polygons: int
```

- void set_path_search_max_polygons(value: int)
- int get_path_search_max_polygons()

The maximum number of polygons that are searched before the pathfinding cancels the search for a path to the (possibly unreachable or very far away) target position polygon. In this case the pathfinding resets and builds a path from the start polygon to the polygon that was found closest to the target position so far. A value of 0 or below counts as unlimited. In case of unlimited the pathfinding will search all polygons connected with the start polygon until either the target position polygon is found or all available polygon search options are exhausted.

### path_search_max_distance {#path_search_max_distance}

```gdscript
path_search_max_distance: float
```

- void set_path_search_max_distance(value: float)
- float get_path_search_max_distance()

The maximum distance a searched polygon can be away from the start polygon before the pathfinding cancels the search for a path to the (possibly unreachable or very far away) target position polygon. In this case the pathfinding resets and builds a path from the start polygon to the polygon that was found closest to the target position so far. A value of 0 or below counts as unlimited. In case of unlimited the pathfinding will search all polygons connected with the start polygon until either the target position polygon is found or all available polygon search options are exhausted.

### avoidance_enabled {#avoidance_enabled}

```gdscript
avoidance_enabled: bool
```

- void set_avoidance_enabled(value: bool)
- bool get_avoidance_enabled()

If true the agent is registered for an RVO avoidance callback on the NavigationServer2D. When velocity is used and the processing is completed a safe_velocity Vector2 is received with a signal connection to velocity_computed. Avoidance processing with many registered agents has a significant performance cost and should only be enabled on agents that currently require it.

### velocity {#velocity}

```gdscript
velocity: Vector2
```

- void set_velocity(value: Vector2)
- Vector2 get_velocity()

Sets the new wanted velocity for the agent. The avoidance simulation will try to fulfill this velocity if possible but will modify it to avoid collision with other agents and obstacles. When an agent is teleported to a new position, use set_velocity_forced() as well to reset the internal simulation velocity.

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The radius of the avoidance agent. This is the "body" of the avoidance agent and not the avoidance maneuver starting radius (which is controlled by neighbor_distance).

Does not affect normal pathfinding. To change an actor's pathfinding radius bake NavigationPolygon resources with a different NavigationPolygon.agent_radius property and use different navigation maps for each actor size.

### neighbor_distance {#neighbor_distance}

```gdscript
neighbor_distance: float
```

- void set_neighbor_distance(value: float)
- float get_neighbor_distance()

The distance to search for other agents.

### max_neighbors {#max_neighbors}

```gdscript
max_neighbors: int
```

- void set_max_neighbors(value: int)
- int get_max_neighbors()

The maximum number of neighbors for the agent to consider.

### time_horizon_agents {#time_horizon_agents}

```gdscript
time_horizon_agents: float
```

- void set_time_horizon_agents(value: float)
- float get_time_horizon_agents()

The minimal amount of time for which this agent's velocities, that are computed with the collision avoidance algorithm, are safe with respect to other agents. The larger the number, the sooner the agent will respond to other agents, but less freedom in choosing its velocities. A too high value will slow down agents movement considerably. Must be positive.

### time_horizon_obstacles {#time_horizon_obstacles}

```gdscript
time_horizon_obstacles: float
```

- void set_time_horizon_obstacles(value: float)
- float get_time_horizon_obstacles()

The minimal amount of time for which this agent's velocities, that are computed with the collision avoidance algorithm, are safe with respect to static avoidance obstacles. The larger the number, the sooner the agent will respond to static avoidance obstacles, but less freedom in choosing its velocities. A too high value will slow down agents movement considerably. Must be positive.

### max_speed {#max_speed}

```gdscript
max_speed: float
```

- void set_max_speed(value: float)
- float get_max_speed()

The maximum speed that an agent can move.

### avoidance_layers {#avoidance_layers}

```gdscript
avoidance_layers: int
```

- void set_avoidance_layers(value: int)
- int get_avoidance_layers()

A bitfield determining the avoidance layers for this NavigationAgent. Other agents with a matching bit on the avoidance_mask will avoid this agent.

### avoidance_mask {#avoidance_mask}

```gdscript
avoidance_mask: int
```

- void set_avoidance_mask(value: int)
- int get_avoidance_mask()

A bitfield determining what other avoidance agents and obstacles this NavigationAgent will avoid when a bit matches at least one of their avoidance_layers.

### avoidance_priority {#avoidance_priority}

```gdscript
avoidance_priority: float
```

- void set_avoidance_priority(value: float)
- float get_avoidance_priority()

The agent does not adjust the velocity for other agents that would match the avoidance_mask but have a lower avoidance_priority. This in turn makes the other agents with lower priority adjust their velocities even more to avoid collision with this agent.

### debug_enabled {#debug_enabled}

```gdscript
debug_enabled: bool
```

- void set_debug_enabled(value: bool)
- bool get_debug_enabled()

If true shows debug visuals for this agent.

### debug_use_custom {#debug_use_custom}

```gdscript
debug_use_custom: bool
```

- void set_debug_use_custom(value: bool)
- bool get_debug_use_custom()

If true uses the defined debug_path_custom_color for this agent instead of global color.

### debug_path_custom_color {#debug_path_custom_color}

```gdscript
debug_path_custom_color: Color
```

- void set_debug_path_custom_color(value: Color)
- Color get_debug_path_custom_color()

If debug_use_custom is true uses this color for this agent instead of global color.

### debug_path_custom_point_size {#debug_path_custom_point_size}

```gdscript
debug_path_custom_point_size: float
```

- void set_debug_path_custom_point_size(value: float)
- float get_debug_path_custom_point_size()

If debug_use_custom is true uses this rasterized point size for rendering path points for this agent instead of global point size.

### debug_path_custom_line_width {#debug_path_custom_line_width}

```gdscript
debug_path_custom_line_width: float
```

- void set_debug_path_custom_line_width(value: float)
- float get_debug_path_custom_line_width()

If debug_use_custom is true uses this line width for rendering paths for this agent instead of global line width.

## 信号

### path_changed {#path_changed}

```gdscript
signal path_changed()
```

Emitted when the agent had to update the loaded path:

- because path was previously empty.
- because navigation map has changed.
- because agent pushed further away from the current path segment than the path_max_distance.

### target_reached {#target_reached}

```gdscript
signal target_reached()
```

Signals that the agent reached the target, i.e. the agent moved within target_desired_distance of the target_position. This signal is emitted only once per loaded path.

This signal will be emitted just before navigation_finished when the target is reachable.

It may not always be possible to reach the target but it should always be possible to reach the final position. See get_final_position().

### waypoint_reached {#waypoint_reached}

```gdscript
signal waypoint_reached(details: Dictionary)
```

Signals that the agent reached a waypoint. Emitted when the agent moves within path_desired_distance of the next position of the path.

The details dictionary may contain the following keys depending on the value of path_metadata_flags:

- position: The position of the waypoint that was reached.
- type: The type of navigation primitive (region or link) that contains this waypoint.
- rid: The RID of the containing navigation primitive (region or link).
- owner: The object which manages the containing navigation primitive (region or link).

### link_reached {#link_reached}

```gdscript
signal link_reached(details: Dictionary)
```

Signals that the agent reached a navigation link. Emitted when the agent moves within path_desired_distance of the next position of the path when that position is a navigation link.

The details dictionary may contain the following keys depending on the value of path_metadata_flags:

- position: The start position of the link that was reached.
- type: Always NavigationPathQueryResult2D.PATH_SEGMENT_TYPE_LINK.
- rid: The RID of the link.
- owner: The object which manages the link (usually NavigationLink2D).
- link_entry_position: If owner is available and the owner is a NavigationLink2D, it will contain the global position of the link's point the agent is entering.
- link_exit_position: If owner is available and the owner is a NavigationLink2D, it will contain the global position of the link's point which the agent is exiting.

### navigation_finished {#navigation_finished}

```gdscript
signal navigation_finished()
```

Signals that the agent's navigation has finished. If the target is reachable, navigation ends when the target is reached. If the target is unreachable, navigation ends when the last waypoint of the path is reached. This signal is emitted only once per loaded path.

This signal will be emitted just after target_reached when the target is reachable.

### velocity_computed {#velocity_computed}

```gdscript
signal velocity_computed(safe_velocity: Vector2)
```

Notifies when the collision avoidance velocity is calculated. Emitted every update as long as avoidance_enabled is true and the agent has a navigation map.
