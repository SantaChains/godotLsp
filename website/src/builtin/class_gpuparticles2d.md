# GPUParticles2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GPUParticles2D**

> 2D particle node used to create a variety of particle systems and effects. GPUParticles2D features an emitter that generates some number of particles at a given rate.

<details><summary>官方英文描述（中文精译进行中）</summary>

2D particle node used to create a variety of particle systems and effects. GPUParticles2D features an emitter that generates some number of particles at a given rate.

Use the process_material property to add a ParticleProcessMaterial to configure particle appearance and behavior. Alternatively, you can add a ShaderMaterial which will be applied to all particles.

2D particles can optionally collide with LightOccluder2D, but they don't collide with PhysicsBody2D nodes.

</details>

## 方法

### capture_rect {#capture_rect}

```gdscript
Rect2 capture_rect() const
```

Returns a rectangle containing the positions of all existing particles.

Note: When using threaded rendering this method synchronizes the rendering thread. Calling it often may have a negative impact on performance.

### convert_from_particles {#convert_from_particles}

```gdscript
void convert_from_particles(particles: Node)
```

Sets this node's properties to match a given CPUParticles2D node.

### emit_particle {#emit_particle}

```gdscript
void emit_particle(xform: Transform2D, velocity: Vector2, color: Color, custom: Color, flags: int)
```

Emits a single particle. Whether xform, velocity, color and custom are applied depends on the value of flags. See EmitFlags.

The default ParticleProcessMaterial will overwrite color and use the contents of custom as (rotation, age, animation, lifetime).

Note: emit_particle() is only supported on the Forward+ and Mobile rendering methods, not Compatibility.

### get_amount {#get_amount}

```gdscript
int get_amount() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_amount_ratio {#get_amount_ratio}

```gdscript
float get_amount_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_base_size {#get_collision_base_size}

```gdscript
float get_collision_base_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_order {#get_draw_order}

```gdscript
GPUParticles2D.DrawOrder get_draw_order() const
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

### get_fractional_delta {#get_fractional_delta}

```gdscript
bool get_fractional_delta() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interp_to_end {#get_interp_to_end}

```gdscript
float get_interp_to_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interpolate {#get_interpolate}

```gdscript
bool get_interpolate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_lifetime {#get_lifetime}

```gdscript
float get_lifetime() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_one_shot {#get_one_shot}

```gdscript
bool get_one_shot() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pre_process_time {#get_pre_process_time}

```gdscript
float get_pre_process_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_material {#get_process_material}

```gdscript
Material get_process_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_randomness_ratio {#get_randomness_ratio}

```gdscript
float get_randomness_ratio() const
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

### get_sub_emitter {#get_sub_emitter}

```gdscript
NodePath get_sub_emitter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_trail_lifetime {#get_trail_lifetime}

```gdscript
float get_trail_lifetime() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_trail_section_subdivisions {#get_trail_section_subdivisions}

```gdscript
int get_trail_section_subdivisions() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_trail_sections {#get_trail_sections}

```gdscript
int get_trail_sections() const
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

### get_visibility_rect {#get_visibility_rect}

```gdscript
Rect2 get_visibility_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_emitting {#is_emitting}

```gdscript
bool is_emitting() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_trail_enabled {#is_trail_enabled}

```gdscript
bool is_trail_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### request_particles_process {#request_particles_process}

```gdscript
void request_particles_process(process_time: float, process_time_residual: float = 0)
```

Requests the particles to process for extra process time during a single frame.

process_time defines the time that the particles will process while emitting is on. process_time_residual defines the time that particles will process with emitting turned off for the simulation. When combined with speed_scale set to 0.0, this is useful to be able to seek a particle system timeline.

### restart {#restart}

```gdscript
void restart(keep_seed: bool = false)
```

Restarts the particle emission cycle, clearing existing particles. To avoid particles vanishing from the viewport, wait for the finished signal before calling.

Note: The finished signal is only emitted by one_shot emitters.

If keep_seed is true, the current random seed will be preserved. Useful for seeking and playback.

### set_amount {#set_amount}

```gdscript
void set_amount(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_amount_ratio {#set_amount_ratio}

```gdscript
void set_amount_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_base_size {#set_collision_base_size}

```gdscript
void set_collision_base_size(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_order {#set_draw_order}

```gdscript
void set_draw_order(order: GPUParticles2D.DrawOrder)
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

### set_fractional_delta {#set_fractional_delta}

```gdscript
void set_fractional_delta(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interp_to_end {#set_interp_to_end}

```gdscript
void set_interp_to_end(interp: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interpolate {#set_interpolate}

```gdscript
void set_interpolate(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lifetime {#set_lifetime}

```gdscript
void set_lifetime(secs: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_shot {#set_one_shot}

```gdscript
void set_one_shot(secs: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pre_process_time {#set_pre_process_time}

```gdscript
void set_pre_process_time(secs: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_material {#set_process_material}

```gdscript
void set_process_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_randomness_ratio {#set_randomness_ratio}

```gdscript
void set_randomness_ratio(ratio: float)
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

### set_sub_emitter {#set_sub_emitter}

```gdscript
void set_sub_emitter(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_trail_enabled {#set_trail_enabled}

```gdscript
void set_trail_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_trail_lifetime {#set_trail_lifetime}

```gdscript
void set_trail_lifetime(secs: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_trail_section_subdivisions {#set_trail_section_subdivisions}

```gdscript
void set_trail_section_subdivisions(subdivisions: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_trail_sections {#set_trail_sections}

```gdscript
void set_trail_sections(sections: int)
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

### set_visibility_rect {#set_visibility_rect}

```gdscript
void set_visibility_rect(visibility_rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### emitting {#emitting}

```gdscript
emitting: bool
```

- void set_emitting(value: bool)
- bool is_emitting()

If true, particles are being emitted. emitting can be used to start and stop particles from emitting. However, if one_shot is true setting emitting to true will not restart the emission cycle unless all active particles have finished processing. Use the finished signal to be notified once all active particles finish processing.

Note: For one_shot emitters, due to the particles being computed on the GPU, there may be a short period after receiving the finished signal during which setting this to true will not restart the emission cycle.

Tip: If your one_shot emitter needs to immediately restart emitting particles once finished signal is received, consider calling restart() instead of setting emitting.

### amount {#amount}

```gdscript
amount: int
```

- void set_amount(value: int)
- int get_amount()

The number of particles to emit in one emission cycle. The effective emission rate is (amount * amount_ratio) / lifetime particles per second. Higher values will increase GPU requirements, even if not all particles are visible at a given time or if amount_ratio is decreased.

Note: Changing this value will cause the particle system to restart. To avoid this, change amount_ratio instead.

### amount_ratio {#amount_ratio}

```gdscript
amount_ratio: float
```

- void set_amount_ratio(value: float)
- float get_amount_ratio()

The ratio of particles that should actually be emitted. If set to a value lower than 1.0, this will set the amount of emitted particles throughout the lifetime to amount * amount_ratio. Unlike changing amount, changing amount_ratio while emitting does not affect already-emitted particles and doesn't cause the particle system to restart. amount_ratio can be used to create effects that make the number of emitted particles vary over time.

Note: Reducing the amount_ratio has no performance benefit, since resources need to be allocated and processed for the total amount of particles regardless of the amount_ratio. If you don't intend to change the number of particles emitted while the particles are emitting, make sure amount_ratio is set to 1 and change amount to your liking instead.

### sub_emitter {#sub_emitter}

```gdscript
sub_emitter: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

Particle texture. If null, particles will be squares with a size of 1×1 pixels.

Note: To use a flipbook texture, assign a new CanvasItemMaterial to the GPUParticles2D's CanvasItem.material property, then enable CanvasItemMaterial.particles_animation and set CanvasItemMaterial.particles_anim_h_frames, CanvasItemMaterial.particles_anim_v_frames, and CanvasItemMaterial.particles_anim_loop to match the flipbook texture.

### lifetime {#lifetime}

```gdscript
lifetime: float
```

- void set_lifetime(value: float)
- float get_lifetime()

The amount of time each particle will exist (in seconds). The effective emission rate is (amount * amount_ratio) / lifetime particles per second.

### interp_to_end {#interp_to_end}

```gdscript
interp_to_end: float
```

- void set_interp_to_end(value: float)
- float get_interp_to_end()

Causes all the particles in this node to interpolate towards the end of their lifetime.

Note: This only works when used with a ParticleProcessMaterial. It needs to be manually implemented for custom process shaders.

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

Note: This can be very expensive if set to a high number as it requires running the particle shader a number of times equal to the fixed_fps (or 30, if fixed_fps is 0) for every second. In extreme cases it can even lead to a GPU crash due to the volume of work done in a single frame.

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

### fixed_fps {#fixed_fps}

```gdscript
fixed_fps: int
```

- void set_fixed_fps(value: int)
- int get_fixed_fps()

The particle system's frame rate is fixed to a value. For example, changing the value to 2 will make the particles render at 2 frames per second. Note this does not slow down the simulation of the particle system itself.

### interpolate {#interpolate}

```gdscript
interpolate: bool
```

- void set_interpolate(value: bool)
- bool get_interpolate()

Enables particle interpolation, which makes the particle movement smoother when their fixed_fps is lower than the screen refresh rate.

### fract_delta {#fract_delta}

```gdscript
fract_delta: bool
```

- void set_fractional_delta(value: bool)
- bool get_fractional_delta()

If true, results in fractional delta calculation which has a smoother particles display effect.

### collision_base_size {#collision_base_size}

```gdscript
collision_base_size: float
```

- void set_collision_base_size(value: float)
- float get_collision_base_size()

Multiplier for particle's collision radius. 1.0 corresponds to the size of the sprite. If particles appear to sink into the ground when colliding, increase this value. If particles appear to float when colliding, decrease this value. Only effective if ParticleProcessMaterial.collision_mode is ParticleProcessMaterial.COLLISION_RIGID or ParticleProcessMaterial.COLLISION_HIDE_ON_CONTACT.

Note: Particles always have a spherical collision shape.

### visibility_rect {#visibility_rect}

```gdscript
visibility_rect: Rect2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### local_coords {#local_coords}

```gdscript
local_coords: bool
```

- void set_use_local_coordinates(value: bool)
- bool get_use_local_coordinates()

If true, particles use the parent node's coordinate space (known as local coordinates). This will cause particles to move and rotate along the GPUParticles2D node (and its parents) when it is moved or rotated. If false, particles use global coordinates; they will not move or rotate along the GPUParticles2D node (and its parents) when it is moved or rotated.

### draw_order {#draw_order}

```gdscript
draw_order: int
```

- void set_draw_order(value: DrawOrder)
- DrawOrder get_draw_order()

Particle draw order.

### trail_enabled {#trail_enabled}

```gdscript
trail_enabled: bool
```

- void set_trail_enabled(value: bool)
- bool is_trail_enabled()

If true, enables particle trails using a mesh skinning system.

Note: Unlike GPUParticles3D, the number of trail sections and subdivisions is set with the trail_sections and trail_section_subdivisions properties.

### trail_lifetime {#trail_lifetime}

```gdscript
trail_lifetime: float
```

- void set_trail_lifetime(value: float)
- float get_trail_lifetime()

The amount of time the particle's trail should represent (in seconds). Only effective if trail_enabled is true.

### trail_sections {#trail_sections}

```gdscript
trail_sections: int
```

- void set_trail_sections(value: int)
- int get_trail_sections()

The number of sections to use for the particle trail rendering. Higher values can result in smoother trail curves, at the cost of performance due to increased mesh complexity. See also trail_section_subdivisions. Only effective if trail_enabled is true.

### trail_section_subdivisions {#trail_section_subdivisions}

```gdscript
trail_section_subdivisions: int
```

- void set_trail_section_subdivisions(value: int)
- int get_trail_section_subdivisions()

The number of subdivisions to use for the particle trail rendering. Higher values can result in smoother trail curves, at the cost of performance due to increased mesh complexity. See also trail_sections. Only effective if trail_enabled is true.

### process_material {#process_material}

```gdscript
process_material: ParticleProcessMaterial,ShaderMaterial
```

- void set_process_material(value: Material)
- Material get_process_material()

Material for processing particles. Can be a ParticleProcessMaterial or a ShaderMaterial.

## 信号

### finished {#finished}

```gdscript
signal finished()
```

Emitted when all active particles have finished processing. To immediately restart the emission cycle, call restart().

This signal is never emitted when one_shot is disabled, as particles will be emitted and processed continuously.

Note: For one_shot emitters, due to the particles being computed on the GPU, there may be a short period after receiving the signal during which setting emitting to true will not restart the emission cycle. This delay is avoided by instead calling restart().

## 枚举

### DrawOrder {#enum-DrawOrder}

```gdscript
enum DrawOrder { DRAW_ORDER_INDEX = 0, DRAW_ORDER_LIFETIME = 1, DRAW_ORDER_REVERSE_LIFETIME = 2 }
```

DrawOrder DRAW_ORDER_INDEX = 0

Particles are drawn in the order emitted.

DrawOrder DRAW_ORDER_LIFETIME = 1

Particles are drawn in order of remaining lifetime. In other words, the particle with the highest lifetime is drawn at the front.

DrawOrder DRAW_ORDER_REVERSE_LIFETIME = 2

Particles are drawn in reverse order of remaining lifetime. In other words, the particle with the lowest lifetime is drawn at the front.

#### DRAW_ORDER_INDEX {#DRAW_ORDER_INDEX}

```gdscript
DRAW_ORDER_INDEX = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DRAW_ORDER_LIFETIME {#DRAW_ORDER_LIFETIME}

```gdscript
DRAW_ORDER_LIFETIME = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DRAW_ORDER_REVERSE_LIFETIME {#DRAW_ORDER_REVERSE_LIFETIME}

```gdscript
DRAW_ORDER_REVERSE_LIFETIME = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### EmitFlags {#enum-EmitFlags}

```gdscript
enum EmitFlags { EMIT_FLAG_POSITION = 1, EMIT_FLAG_ROTATION_SCALE = 2, EMIT_FLAG_VELOCITY = 4, EMIT_FLAG_COLOR = 8, EMIT_FLAG_CUSTOM = 16 }
```

EmitFlags EMIT_FLAG_POSITION = 1

Particle starts at the specified position.

EmitFlags EMIT_FLAG_ROTATION_SCALE = 2

Particle starts with specified rotation and scale.

EmitFlags EMIT_FLAG_VELOCITY = 4

Particle starts with the specified velocity vector, which defines the emission direction and speed.

EmitFlags EMIT_FLAG_COLOR = 8

Particle starts with specified color.

EmitFlags EMIT_FLAG_CUSTOM = 16

Particle starts with specified CUSTOM data.

#### EMIT_FLAG_POSITION {#EMIT_FLAG_POSITION}

```gdscript
EMIT_FLAG_POSITION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EMIT_FLAG_ROTATION_SCALE {#EMIT_FLAG_ROTATION_SCALE}

```gdscript
EMIT_FLAG_ROTATION_SCALE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EMIT_FLAG_VELOCITY {#EMIT_FLAG_VELOCITY}

```gdscript
EMIT_FLAG_VELOCITY = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EMIT_FLAG_COLOR {#EMIT_FLAG_COLOR}

```gdscript
EMIT_FLAG_COLOR = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EMIT_FLAG_CUSTOM {#EMIT_FLAG_CUSTOM}

```gdscript
EMIT_FLAG_CUSTOM = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
