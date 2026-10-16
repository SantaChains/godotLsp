# CPUParticles3D

**继承**：[GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CPUParticles3D**

> CPU-based 3D particle node used to create a variety of particle systems and effects.

<details><summary>官方英文描述</summary>

CPU-based 3D particle node used to create a variety of particle systems and effects.

See also GPUParticles3D, which provides the same functionality with hardware acceleration, but may not run on older devices.

</details>

## 方法

### capture_aabb {#capture_aabb}

```gdscript
AABB capture_aabb() const
```

Returns the axis-aligned bounding box that contains all the particles that are active in the current frame.

### convert_from_particles {#convert_from_particles}

```gdscript
void convert_from_particles(particles: Node)
```

Sets this node's properties to match a given GPUParticles3D node with an assigned ParticleProcessMaterial.

### get_amount {#get_amount}

```gdscript
int get_amount() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color {#get_color}

```gdscript
Color get_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color_initial_ramp {#get_color_initial_ramp}

```gdscript
Gradient get_color_initial_ramp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color_ramp {#get_color_ramp}

```gdscript
Gradient get_color_ramp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_direction {#get_direction}

```gdscript
Vector3 get_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_order {#get_draw_order}

```gdscript
CPUParticles3D.DrawOrder get_draw_order() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_box_extents {#get_emission_box_extents}

```gdscript
Vector3 get_emission_box_extents() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_colors {#get_emission_colors}

```gdscript
PackedColorArray get_emission_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_normals {#get_emission_normals}

```gdscript
PackedVector3Array get_emission_normals() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_points {#get_emission_points}

```gdscript
PackedVector3Array get_emission_points() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_ring_axis {#get_emission_ring_axis}

```gdscript
Vector3 get_emission_ring_axis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_ring_cone_angle {#get_emission_ring_cone_angle}

```gdscript
float get_emission_ring_cone_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_ring_height {#get_emission_ring_height}

```gdscript
float get_emission_ring_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_ring_inner_radius {#get_emission_ring_inner_radius}

```gdscript
float get_emission_ring_inner_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_ring_radius {#get_emission_ring_radius}

```gdscript
float get_emission_ring_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_shape {#get_emission_shape}

```gdscript
CPUParticles3D.EmissionShape get_emission_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_sphere_radius {#get_emission_sphere_radius}

```gdscript
float get_emission_sphere_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_explosiveness_ratio {#get_explosiveness_ratio}

```gdscript
float get_explosiveness_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fixed_fps {#get_fixed_fps}

```gdscript
int get_fixed_fps() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_flatness {#get_flatness}

```gdscript
float get_flatness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fractional_delta {#get_fractional_delta}

```gdscript
bool get_fractional_delta() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity {#get_gravity}

```gdscript
Vector3 get_gravity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_lifetime {#get_lifetime}

```gdscript
float get_lifetime() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_lifetime_randomness {#get_lifetime_randomness}

```gdscript
float get_lifetime_randomness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_one_shot {#get_one_shot}

```gdscript
bool get_one_shot() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_param_curve {#get_param_curve}

```gdscript
Curve get_param_curve(param: CPUParticles3D.Parameter) const
```

Returns the Curve of the parameter specified by Parameter.

### get_param_max {#get_param_max}

```gdscript
float get_param_max(param: CPUParticles3D.Parameter) const
```

Returns the maximum value range for the given parameter.

### get_param_min {#get_param_min}

```gdscript
float get_param_min(param: CPUParticles3D.Parameter) const
```

Returns the minimum value range for the given parameter.

### get_particle_flag {#get_particle_flag}

```gdscript
bool get_particle_flag(particle_flag: CPUParticles3D.ParticleFlags) const
```

Returns the enabled state of the given particle flag.

### get_pre_process_time {#get_pre_process_time}

```gdscript
float get_pre_process_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_randomness_ratio {#get_randomness_ratio}

```gdscript
float get_randomness_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale_curve_x {#get_scale_curve_x}

```gdscript
Curve get_scale_curve_x() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale_curve_y {#get_scale_curve_y}

```gdscript
Curve get_scale_curve_y() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale_curve_z {#get_scale_curve_z}

```gdscript
Curve get_scale_curve_z() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_seed {#get_seed}

```gdscript
int get_seed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_speed_scale {#get_speed_scale}

```gdscript
float get_speed_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_split_scale {#get_split_scale}

```gdscript
bool get_split_scale()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spread {#get_spread}

```gdscript
float get_spread() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_fixed_seed {#get_use_fixed_seed}

```gdscript
bool get_use_fixed_seed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_local_coordinates {#get_use_local_coordinates}

```gdscript
bool get_use_local_coordinates() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visibility_aabb {#get_visibility_aabb}

```gdscript
AABB get_visibility_aabb() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_emitting {#is_emitting}

```gdscript
bool is_emitting() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### request_particles_process {#request_particles_process}

```gdscript
void request_particles_process(process_time: float, process_time_residual: float = 0.0)
```

Requests the particles to process for extra process time during a single frame.

process_time defines the time that the particles will process while emitting is on. process_time_residual defines the time that particles will process with emitting turned off for the simulation. When combined with speed_scale set to 0.0, this is useful to be able to seek a particle system timeline.

### restart {#restart}

```gdscript
void restart(keep_seed: bool = false)
```

Restarts the particle emitter.

If keep_seed is true, the current random seed will be preserved. Useful for seeking and playback.

### set_amount {#set_amount}

```gdscript
void set_amount(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color {#set_color}

```gdscript
void set_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_initial_ramp {#set_color_initial_ramp}

```gdscript
void set_color_initial_ramp(ramp: Gradient)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_ramp {#set_color_ramp}

```gdscript
void set_color_ramp(ramp: Gradient)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_direction {#set_direction}

```gdscript
void set_direction(direction: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_order {#set_draw_order}

```gdscript
void set_draw_order(order: CPUParticles3D.DrawOrder)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_box_extents {#set_emission_box_extents}

```gdscript
void set_emission_box_extents(extents: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_colors {#set_emission_colors}

```gdscript
void set_emission_colors(array: PackedColorArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_normals {#set_emission_normals}

```gdscript
void set_emission_normals(array: PackedVector3Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_points {#set_emission_points}

```gdscript
void set_emission_points(array: PackedVector3Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_ring_axis {#set_emission_ring_axis}

```gdscript
void set_emission_ring_axis(axis: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_ring_cone_angle {#set_emission_ring_cone_angle}

```gdscript
void set_emission_ring_cone_angle(cone_angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_ring_height {#set_emission_ring_height}

```gdscript
void set_emission_ring_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_ring_inner_radius {#set_emission_ring_inner_radius}

```gdscript
void set_emission_ring_inner_radius(inner_radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_ring_radius {#set_emission_ring_radius}

```gdscript
void set_emission_ring_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_shape {#set_emission_shape}

```gdscript
void set_emission_shape(shape: CPUParticles3D.EmissionShape)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_sphere_radius {#set_emission_sphere_radius}

```gdscript
void set_emission_sphere_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emitting {#set_emitting}

```gdscript
void set_emitting(emitting: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_explosiveness_ratio {#set_explosiveness_ratio}

```gdscript
void set_explosiveness_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fixed_fps {#set_fixed_fps}

```gdscript
void set_fixed_fps(fps: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flatness {#set_flatness}

```gdscript
void set_flatness(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fractional_delta {#set_fractional_delta}

```gdscript
void set_fractional_delta(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity {#set_gravity}

```gdscript
void set_gravity(accel_vec: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lifetime {#set_lifetime}

```gdscript
void set_lifetime(secs: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lifetime_randomness {#set_lifetime_randomness}

```gdscript
void set_lifetime_randomness(random: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: Mesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_shot {#set_one_shot}

```gdscript
void set_one_shot(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_param_curve {#set_param_curve}

```gdscript
void set_param_curve(param: CPUParticles3D.Parameter, curve: Curve)
```

Sets the Curve of the parameter specified by Parameter. Should be a unit Curve.

### set_param_max {#set_param_max}

```gdscript
void set_param_max(param: CPUParticles3D.Parameter, value: float)
```

Sets the maximum value for the given parameter.

### set_param_min {#set_param_min}

```gdscript
void set_param_min(param: CPUParticles3D.Parameter, value: float)
```

Sets the minimum value for the given parameter.

### set_particle_flag {#set_particle_flag}

```gdscript
void set_particle_flag(particle_flag: CPUParticles3D.ParticleFlags, enable: bool)
```

Enables or disables the given particle flag.

### set_pre_process_time {#set_pre_process_time}

```gdscript
void set_pre_process_time(secs: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_randomness_ratio {#set_randomness_ratio}

```gdscript
void set_randomness_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale_curve_x {#set_scale_curve_x}

```gdscript
void set_scale_curve_x(scale_curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale_curve_y {#set_scale_curve_y}

```gdscript
void set_scale_curve_y(scale_curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale_curve_z {#set_scale_curve_z}

```gdscript
void set_scale_curve_z(scale_curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_seed {#set_seed}

```gdscript
void set_seed(seed: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_speed_scale {#set_speed_scale}

```gdscript
void set_speed_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_split_scale {#set_split_scale}

```gdscript
void set_split_scale(split_scale: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spread {#set_spread}

```gdscript
void set_spread(degrees: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_fixed_seed {#set_use_fixed_seed}

```gdscript
void set_use_fixed_seed(use_fixed_seed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_local_coordinates {#set_use_local_coordinates}

```gdscript
void set_use_local_coordinates(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visibility_aabb {#set_visibility_aabb}

```gdscript
void set_visibility_aabb(aabb: AABB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### emitting {#emitting}

```gdscript
emitting: bool
```

- void set_emitting(value: bool)
- bool is_emitting()

If true, particles are being emitted. emitting can be used to start and stop particles from emitting. However, if one_shot is true setting emitting to true will not restart the emission cycle until after all active particles finish processing. You can use the finished signal to be notified once all active particles finish processing.

### amount {#amount}

```gdscript
amount: int
```

- void set_amount(value: int)
- int get_amount()

Number of particles emitted in one emission cycle.

### lifetime {#lifetime}

```gdscript
lifetime: float
```

- void set_lifetime(value: float)
- float get_lifetime()

Amount of time each particle will exist.

### one_shot {#one_shot}

```gdscript
one_shot: bool
```

- void set_one_shot(value: bool)
- bool get_one_shot()

If true, only one emission cycle occurs. If set true during a cycle, emission will stop at the cycle's end.

### preprocess {#preprocess}

```gdscript
preprocess: float
```

- void set_pre_process_time(value: float)
- float get_pre_process_time()

Particle system starts as if it had already run for this many seconds.

### speed_scale {#speed_scale}

```gdscript
speed_scale: float
```

- void set_speed_scale(value: float)
- float get_speed_scale()

Particle system's running speed scaling ratio. A value of 0 can be used to pause the particles.

### explosiveness {#explosiveness}

```gdscript
explosiveness: float
```

- void set_explosiveness_ratio(value: float)
- float get_explosiveness_ratio()

How rapidly particles in an emission cycle are emitted. If greater than 0, there will be a gap in emissions before the next cycle begins.

### randomness {#randomness}

```gdscript
randomness: float
```

- void set_randomness_ratio(value: float)
- float get_randomness_ratio()

Emission lifetime randomness ratio.

### use_fixed_seed {#use_fixed_seed}

```gdscript
use_fixed_seed: bool
```

- void set_use_fixed_seed(value: bool)
- bool get_use_fixed_seed()

If true, particles will use the same seed for every simulation using the seed defined in seed. This is useful for situations where the visual outcome should be consistent across replays, for example when using Movie Maker mode.

### seed {#seed}

```gdscript
seed: int
```

- void set_seed(value: int)
- int get_seed()

Sets the random seed used by the particle system. Only effective if use_fixed_seed is true.

### lifetime_randomness {#lifetime_randomness}

```gdscript
lifetime_randomness: float
```

- void set_lifetime_randomness(value: float)
- float get_lifetime_randomness()

Particle lifetime randomness ratio.

### fixed_fps {#fixed_fps}

```gdscript
fixed_fps: int
```

- void set_fixed_fps(value: int)
- int get_fixed_fps()

The particle system's frame rate is fixed to a value. For example, changing the value to 2 will make the particles render at 2 frames per second. Note this does not slow down the particle system itself.

### fract_delta {#fract_delta}

```gdscript
fract_delta: bool
```

- void set_fractional_delta(value: bool)
- bool get_fractional_delta()

If true, results in fractional delta calculation which has a smoother particles display effect.

### visibility_aabb {#visibility_aabb}

```gdscript
visibility_aabb: AABB
```

- void set_visibility_aabb(value: AABB)
- AABB get_visibility_aabb()

The AABB that determines the node's region which needs to be visible on screen for the particle system to be active.

Grow the box if particles suddenly appear/disappear when the node enters/exits the screen. The AABB can be grown via code or with the Particles → Generate AABB editor tool.

### local_coords {#local_coords}

```gdscript
local_coords: bool
```

- void set_use_local_coordinates(value: bool)
- bool get_use_local_coordinates()

If true, particles use the parent node's coordinate space (known as local coordinates). This will cause particles to move and rotate along the CPUParticles3D node (and its parents) when it is moved or rotated. If false, particles use global coordinates; they will not move or rotate along the CPUParticles3D node (and its parents) when it is moved or rotated.

### draw_order {#draw_order}

```gdscript
draw_order: int
```

- void set_draw_order(value: DrawOrder)
- DrawOrder get_draw_order()

Particle draw order.

### mesh {#mesh}

```gdscript
mesh: Mesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

The Mesh used for each particle. If null, particles will be spheres.

### emission_shape {#emission_shape}

```gdscript
emission_shape: int
```

- void set_emission_shape(value: EmissionShape)
- EmissionShape get_emission_shape()

Particles will be emitted inside this region.

### emission_sphere_radius {#emission_sphere_radius}

```gdscript
emission_sphere_radius: float
```

- void set_emission_sphere_radius(value: float)
- float get_emission_sphere_radius()

The sphere's radius if EmissionShape is set to EMISSION_SHAPE_SPHERE.

### emission_box_extents {#emission_box_extents}

```gdscript
emission_box_extents: Vector3
```

- void set_emission_box_extents(value: Vector3)
- Vector3 get_emission_box_extents()

The rectangle's extents if emission_shape is set to EMISSION_SHAPE_BOX.

### emission_points {#emission_points}

```gdscript
emission_points: PackedVector3Array
```

- void set_emission_points(value: PackedVector3Array)
- PackedVector3Array get_emission_points()

Sets the initial positions to spawn particles when using EMISSION_SHAPE_POINTS or EMISSION_SHAPE_DIRECTED_POINTS.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.

### emission_normals {#emission_normals}

```gdscript
emission_normals: PackedVector3Array
```

- void set_emission_normals(value: PackedVector3Array)
- PackedVector3Array get_emission_normals()

Sets the direction the particles will be emitted in when using EMISSION_SHAPE_DIRECTED_POINTS.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.

### emission_colors {#emission_colors}

```gdscript
emission_colors: PackedColorArray
```

- void set_emission_colors(value: PackedColorArray)
- PackedColorArray get_emission_colors()

Sets the Colors to modulate particles by when using EMISSION_SHAPE_POINTS or EMISSION_SHAPE_DIRECTED_POINTS.

Note: emission_colors multiplies the particle mesh's vertex colors. To have a visible effect on a BaseMaterial3D, BaseMaterial3D.vertex_color_use_as_albedo must be true. For a ShaderMaterial, ALBEDO *= COLOR.rgb; must be inserted in the shader's fragment() function. Otherwise, emission_colors will have no visible effect.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedColorArray for more details.

### emission_ring_axis {#emission_ring_axis}

```gdscript
emission_ring_axis: Vector3
```

- void set_emission_ring_axis(value: Vector3)
- Vector3 get_emission_ring_axis()

The axis of the ring when using the emitter EMISSION_SHAPE_RING.

### emission_ring_height {#emission_ring_height}

```gdscript
emission_ring_height: float
```

- void set_emission_ring_height(value: float)
- float get_emission_ring_height()

The height of the ring when using the emitter EMISSION_SHAPE_RING.

### emission_ring_radius {#emission_ring_radius}

```gdscript
emission_ring_radius: float
```

- void set_emission_ring_radius(value: float)
- float get_emission_ring_radius()

The radius of the ring when using the emitter EMISSION_SHAPE_RING.

### emission_ring_inner_radius {#emission_ring_inner_radius}

```gdscript
emission_ring_inner_radius: float
```

- void set_emission_ring_inner_radius(value: float)
- float get_emission_ring_inner_radius()

The inner radius of the ring when using the emitter EMISSION_SHAPE_RING.

### emission_ring_cone_angle {#emission_ring_cone_angle}

```gdscript
emission_ring_cone_angle: float
```

- void set_emission_ring_cone_angle(value: float)
- float get_emission_ring_cone_angle()

The angle of the cone when using the emitter EMISSION_SHAPE_RING. The default angle of 90 degrees results in a ring, while an angle of 0 degrees results in a cone. Intermediate values will result in a ring where one end is larger than the other.

Note: Depending on emission_ring_height, the angle may be clamped if the ring's end is reached to form a perfect cone.

### particle_flag_align_y {#particle_flag_align_y}

```gdscript
particle_flag_align_y: bool
```

- void set_particle_flag(particle_flag: ParticleFlags, enable: bool)
- bool get_particle_flag(particle_flag: ParticleFlags) const

Align Y axis of particle with the direction of its velocity.

### particle_flag_rotate_y {#particle_flag_rotate_y}

```gdscript
particle_flag_rotate_y: bool
```

- void set_particle_flag(particle_flag: ParticleFlags, enable: bool)
- bool get_particle_flag(particle_flag: ParticleFlags) const

If true, particles rotate around Y axis by angle_min.

### particle_flag_disable_z {#particle_flag_disable_z}

```gdscript
particle_flag_disable_z: bool
```

- void set_particle_flag(particle_flag: ParticleFlags, enable: bool)
- bool get_particle_flag(particle_flag: ParticleFlags) const

If true, particles will not move on the Z axis.

### direction {#direction}

```gdscript
direction: Vector3
```

- void set_direction(value: Vector3)
- Vector3 get_direction()

Unit vector specifying the particles' emission direction.

### spread {#spread}

```gdscript
spread: float
```

- void set_spread(value: float)
- float get_spread()

Each particle's initial direction range from +spread to -spread degrees. Applied to X/Z plane and Y/Z planes.

### flatness {#flatness}

```gdscript
flatness: float
```

- void set_flatness(value: float)
- float get_flatness()

Amount of spread in Y/Z plane. A value of 1 restricts particles to X/Z plane.

### gravity {#gravity}

```gdscript
gravity: Vector3
```

- void set_gravity(value: Vector3)
- Vector3 get_gravity()

Gravity applied to every particle.

### initial_velocity_min {#initial_velocity_min}

```gdscript
initial_velocity_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum value of the initial velocity.

### initial_velocity_max {#initial_velocity_max}

```gdscript
initial_velocity_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum value of the initial velocity.

### angular_velocity_min {#angular_velocity_min}

```gdscript
angular_velocity_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum initial angular velocity (rotation speed) applied to each particle in degrees per second.

### angular_velocity_max {#angular_velocity_max}

```gdscript
angular_velocity_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum initial angular velocity (rotation speed) applied to each particle in degrees per second.

### angular_velocity_curve {#angular_velocity_curve}

```gdscript
angular_velocity_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's angular velocity (rotation speed) will vary along this Curve over its lifetime. Should be a unit Curve.

### orbit_velocity_min {#orbit_velocity_min}

```gdscript
orbit_velocity_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum orbit velocity.

### orbit_velocity_max {#orbit_velocity_max}

```gdscript
orbit_velocity_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum orbit velocity.

### orbit_velocity_curve {#orbit_velocity_curve}

```gdscript
orbit_velocity_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's orbital velocity will vary along this Curve. Should be a unit Curve.

### linear_accel_min {#linear_accel_min}

```gdscript
linear_accel_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum linear acceleration.

### linear_accel_max {#linear_accel_max}

```gdscript
linear_accel_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum linear acceleration.

### linear_accel_curve {#linear_accel_curve}

```gdscript
linear_accel_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's linear acceleration will vary along this Curve. Should be a unit Curve.

### radial_accel_min {#radial_accel_min}

```gdscript
radial_accel_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum radial acceleration.

### radial_accel_max {#radial_accel_max}

```gdscript
radial_accel_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum radial acceleration.

### radial_accel_curve {#radial_accel_curve}

```gdscript
radial_accel_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's radial acceleration will vary along this Curve. Should be a unit Curve.

### tangential_accel_min {#tangential_accel_min}

```gdscript
tangential_accel_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum tangent acceleration.

### tangential_accel_max {#tangential_accel_max}

```gdscript
tangential_accel_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum tangent acceleration.

### tangential_accel_curve {#tangential_accel_curve}

```gdscript
tangential_accel_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's tangential acceleration will vary along this Curve. Should be a unit Curve.

### damping_min {#damping_min}

```gdscript
damping_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum damping.

### damping_max {#damping_max}

```gdscript
damping_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum damping.

### damping_curve {#damping_curve}

```gdscript
damping_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Damping will vary along this Curve. Should be a unit Curve.

### angle_min {#angle_min}

```gdscript
angle_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum angle.

### angle_max {#angle_max}

```gdscript
angle_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum angle.

### angle_curve {#angle_curve}

```gdscript
angle_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's rotation will be animated along this Curve. Should be a unit Curve.

### scale_amount_min {#scale_amount_min}

```gdscript
scale_amount_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum scale.

### scale_amount_max {#scale_amount_max}

```gdscript
scale_amount_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum scale.

### scale_amount_curve {#scale_amount_curve}

```gdscript
scale_amount_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's scale will vary along this Curve. Should be a unit Curve.

### split_scale {#split_scale}

```gdscript
split_scale: bool
```

- void set_split_scale(value: bool)
- bool get_split_scale()

If set to true, three different scale curves can be specified, one per scale axis.

### scale_curve_x {#scale_curve_x}

```gdscript
scale_curve_x: Curve
```

- void set_scale_curve_x(value: Curve)
- Curve get_scale_curve_x()

Curve for the scale over life, along the x axis.

### scale_curve_y {#scale_curve_y}

```gdscript
scale_curve_y: Curve
```

- void set_scale_curve_y(value: Curve)
- Curve get_scale_curve_y()

Curve for the scale over life, along the y axis.

### scale_curve_z {#scale_curve_z}

```gdscript
scale_curve_z: Curve
```

- void set_scale_curve_z(value: Curve)
- Curve get_scale_curve_z()

Curve for the scale over life, along the z axis.

### color {#color}

```gdscript
color: Color
```

- void set_color(value: Color)
- Color get_color()

Each particle's initial color.

Note: color multiplies the particle mesh's vertex colors. To have a visible effect on a BaseMaterial3D, BaseMaterial3D.vertex_color_use_as_albedo must be true. For a ShaderMaterial, ALBEDO *= COLOR.rgb; must be inserted in the shader's fragment() function. Otherwise, color will have no visible effect.

### color_ramp {#color_ramp}

```gdscript
color_ramp: Gradient
```

- void set_color_ramp(value: Gradient)
- Gradient get_color_ramp()

Each particle's color will vary along this Gradient over its lifetime (multiplied with color).

Note: color_ramp multiplies the particle mesh's vertex colors. To have a visible effect on a BaseMaterial3D, BaseMaterial3D.vertex_color_use_as_albedo must be true. For a ShaderMaterial, ALBEDO *= COLOR.rgb; must be inserted in the shader's fragment() function. Otherwise, color_ramp will have no visible effect.

### color_initial_ramp {#color_initial_ramp}

```gdscript
color_initial_ramp: Gradient
```

- void set_color_initial_ramp(value: Gradient)
- Gradient get_color_initial_ramp()

Each particle's initial color will vary along this Gradient (multiplied with color).

Note: color_initial_ramp multiplies the particle mesh's vertex colors. To have a visible effect on a BaseMaterial3D, BaseMaterial3D.vertex_color_use_as_albedo must be true. For a ShaderMaterial, ALBEDO *= COLOR.rgb; must be inserted in the shader's fragment() function. Otherwise, color_initial_ramp will have no visible effect.

### hue_variation_min {#hue_variation_min}

```gdscript
hue_variation_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum hue variation.

### hue_variation_max {#hue_variation_max}

```gdscript
hue_variation_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum hue variation.

### hue_variation_curve {#hue_variation_curve}

```gdscript
hue_variation_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's hue will vary along this Curve. Should be a unit Curve.

### anim_speed_min {#anim_speed_min}

```gdscript
anim_speed_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum particle animation speed.

### anim_speed_max {#anim_speed_max}

```gdscript
anim_speed_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum particle animation speed.

### anim_speed_curve {#anim_speed_curve}

```gdscript
anim_speed_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's animation speed will vary along this Curve. Should be a unit Curve.

### anim_offset_min {#anim_offset_min}

```gdscript
anim_offset_min: float
```

- void set_param_min(param: Parameter, value: float)
- float get_param_min(param: Parameter) const

Minimum animation offset.

### anim_offset_max {#anim_offset_max}

```gdscript
anim_offset_max: float
```

- void set_param_max(param: Parameter, value: float)
- float get_param_max(param: Parameter) const

Maximum animation offset.

### anim_offset_curve {#anim_offset_curve}

```gdscript
anim_offset_curve: Curve
```

- void set_param_curve(param: Parameter, curve: Curve)
- Curve get_param_curve(param: Parameter) const

Each particle's animation offset will vary along this Curve. Should be a unit Curve.

## 信号

### finished {#finished}

```gdscript
signal finished()
```

Emitted when all active particles have finished processing. When one_shot is disabled, particles will process continuously, so this is never emitted.

## 枚举

### DrawOrder {#enum-DrawOrder}

```gdscript
enum DrawOrder { DRAW_ORDER_INDEX = 0, DRAW_ORDER_LIFETIME = 1, DRAW_ORDER_VIEW_DEPTH = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DRAW_ORDER_INDEX {#DRAW_ORDER_INDEX}

```gdscript
DRAW_ORDER_INDEX = 0
```

Particles are drawn in the order emitted.

#### DRAW_ORDER_LIFETIME {#DRAW_ORDER_LIFETIME}

```gdscript
DRAW_ORDER_LIFETIME = 1
```

Particles are drawn in order of remaining lifetime. In other words, the particle with the highest lifetime is drawn at the front.

#### DRAW_ORDER_VIEW_DEPTH {#DRAW_ORDER_VIEW_DEPTH}

```gdscript
DRAW_ORDER_VIEW_DEPTH = 2
```

Particles are drawn in order of depth.

---

### Parameter {#enum-Parameter}

```gdscript
enum Parameter { PARAM_INITIAL_LINEAR_VELOCITY = 0, PARAM_ANGULAR_VELOCITY = 1, PARAM_ORBIT_VELOCITY = 2, PARAM_LINEAR_ACCEL = 3, PARAM_RADIAL_ACCEL = 4, PARAM_TANGENTIAL_ACCEL = 5, PARAM_DAMPING = 6, PARAM_ANGLE = 7, PARAM_SCALE = 8, PARAM_HUE_VARIATION = 9, PARAM_ANIM_SPEED = 10, PARAM_ANIM_OFFSET = 11, PARAM_MAX = 12 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_INITIAL_LINEAR_VELOCITY {#PARAM_INITIAL_LINEAR_VELOCITY}

```gdscript
PARAM_INITIAL_LINEAR_VELOCITY = 0
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set initial velocity properties.

#### PARAM_ANGULAR_VELOCITY {#PARAM_ANGULAR_VELOCITY}

```gdscript
PARAM_ANGULAR_VELOCITY = 1
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set angular velocity properties.

#### PARAM_ORBIT_VELOCITY {#PARAM_ORBIT_VELOCITY}

```gdscript
PARAM_ORBIT_VELOCITY = 2
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set orbital velocity properties.

#### PARAM_LINEAR_ACCEL {#PARAM_LINEAR_ACCEL}

```gdscript
PARAM_LINEAR_ACCEL = 3
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set linear acceleration properties.

#### PARAM_RADIAL_ACCEL {#PARAM_RADIAL_ACCEL}

```gdscript
PARAM_RADIAL_ACCEL = 4
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set radial acceleration properties.

#### PARAM_TANGENTIAL_ACCEL {#PARAM_TANGENTIAL_ACCEL}

```gdscript
PARAM_TANGENTIAL_ACCEL = 5
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set tangential acceleration properties.

#### PARAM_DAMPING {#PARAM_DAMPING}

```gdscript
PARAM_DAMPING = 6
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set damping properties.

#### PARAM_ANGLE {#PARAM_ANGLE}

```gdscript
PARAM_ANGLE = 7
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set angle properties.

#### PARAM_SCALE {#PARAM_SCALE}

```gdscript
PARAM_SCALE = 8
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set scale properties.

#### PARAM_HUE_VARIATION {#PARAM_HUE_VARIATION}

```gdscript
PARAM_HUE_VARIATION = 9
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set hue variation properties.

#### PARAM_ANIM_SPEED {#PARAM_ANIM_SPEED}

```gdscript
PARAM_ANIM_SPEED = 10
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set animation speed properties.

#### PARAM_ANIM_OFFSET {#PARAM_ANIM_OFFSET}

```gdscript
PARAM_ANIM_OFFSET = 11
```

Use with set_param_min(), set_param_max(), and set_param_curve() to set animation offset properties.

#### PARAM_MAX {#PARAM_MAX}

```gdscript
PARAM_MAX = 12
```

Represents the size of the Parameter enum.

---

### ParticleFlags {#enum-ParticleFlags}

```gdscript
enum ParticleFlags { PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY = 0, PARTICLE_FLAG_ROTATE_Y = 1, PARTICLE_FLAG_DISABLE_Z = 2, PARTICLE_FLAG_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY {#PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY}

```gdscript
PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY = 0
```

Use with set_particle_flag() to set particle_flag_align_y.

#### PARTICLE_FLAG_ROTATE_Y {#PARTICLE_FLAG_ROTATE_Y}

```gdscript
PARTICLE_FLAG_ROTATE_Y = 1
```

Use with set_particle_flag() to set particle_flag_rotate_y.

#### PARTICLE_FLAG_DISABLE_Z {#PARTICLE_FLAG_DISABLE_Z}

```gdscript
PARTICLE_FLAG_DISABLE_Z = 2
```

Use with set_particle_flag() to set particle_flag_disable_z.

#### PARTICLE_FLAG_MAX {#PARTICLE_FLAG_MAX}

```gdscript
PARTICLE_FLAG_MAX = 3
```

Represents the size of the ParticleFlags enum.

---

### EmissionShape {#enum-EmissionShape}

```gdscript
enum EmissionShape { EMISSION_SHAPE_POINT = 0, EMISSION_SHAPE_SPHERE = 1, EMISSION_SHAPE_SPHERE_SURFACE = 2, EMISSION_SHAPE_BOX = 3, EMISSION_SHAPE_POINTS = 4, EMISSION_SHAPE_DIRECTED_POINTS = 5, EMISSION_SHAPE_RING = 6, EMISSION_SHAPE_MAX = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EMISSION_SHAPE_POINT {#EMISSION_SHAPE_POINT}

```gdscript
EMISSION_SHAPE_POINT = 0
```

All particles will be emitted from a single point.

#### EMISSION_SHAPE_SPHERE {#EMISSION_SHAPE_SPHERE}

```gdscript
EMISSION_SHAPE_SPHERE = 1
```

Particles will be emitted in the volume of a sphere.

#### EMISSION_SHAPE_SPHERE_SURFACE {#EMISSION_SHAPE_SPHERE_SURFACE}

```gdscript
EMISSION_SHAPE_SPHERE_SURFACE = 2
```

Particles will be emitted on the surface of a sphere.

#### EMISSION_SHAPE_BOX {#EMISSION_SHAPE_BOX}

```gdscript
EMISSION_SHAPE_BOX = 3
```

Particles will be emitted in the volume of a box.

#### EMISSION_SHAPE_POINTS {#EMISSION_SHAPE_POINTS}

```gdscript
EMISSION_SHAPE_POINTS = 4
```

Particles will be emitted at a position chosen randomly among emission_points. Particle color will be modulated by emission_colors.

#### EMISSION_SHAPE_DIRECTED_POINTS {#EMISSION_SHAPE_DIRECTED_POINTS}

```gdscript
EMISSION_SHAPE_DIRECTED_POINTS = 5
```

Particles will be emitted at a position chosen randomly among emission_points. Particle velocity and rotation will be set based on emission_normals. Particle color will be modulated by emission_colors.

#### EMISSION_SHAPE_RING {#EMISSION_SHAPE_RING}

```gdscript
EMISSION_SHAPE_RING = 6
```

Particles will be emitted in a ring or cylinder.

#### EMISSION_SHAPE_MAX {#EMISSION_SHAPE_MAX}

```gdscript
EMISSION_SHAPE_MAX = 7
```

Represents the size of the EmissionShape enum.

---
