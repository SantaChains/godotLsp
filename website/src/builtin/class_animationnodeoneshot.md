# AnimationNodeOneShot

**继承**：[AnimationNodeSync](class_animationnodesync.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeOneShot**

> A resource to add to an AnimationNodeBlendTree. This animation node will execute a sub-animation and return once it finishes. Blend times for fading in and out can be customized, as well as filters.

<details><summary>官方英文描述（中文精译进行中）</summary>

A resource to add to an AnimationNodeBlendTree. This animation node will execute a sub-animation and return once it finishes. Blend times for fading in and out can be customized, as well as filters.

After setting the request and changing the animation playback, the one-shot node automatically clears the request on the next process frame by setting its request value to ONE_SHOT_REQUEST_NONE.

```
# Play child animation connected to "shot" port.
animation_tree.set("parameters/OneShot/request", AnimationNodeOneShot.ONE_SHOT_REQUEST_FIRE)
# Alternative syntax (same result as above).
animation_tree["parameters/OneShot/request"] = AnimationNodeOneShot.ONE_SHOT_REQUEST_FIRE
# Abort child animation connected to "shot" port.
animation_tree.set("parameters/OneShot/request", AnimationNodeOneShot.ONE_SHOT_REQUEST_ABORT)
# Alternative syntax (same result as above).
animation_tree["parameters/OneShot/request"] = AnimationNodeOneShot.ONE_SHOT_REQUEST_ABORT
# Abort child animation with fading out connected to "shot" port.
animation_tree.set("parameters/OneShot/request", AnimationNodeOneShot.ONE_SHOT_REQUEST_FADE_OUT)
# Alternative syntax (same result as above).
animation_tree["parameters/OneShot/request"] = AnimationNodeOneShot.ONE_SHOT_REQUEST_FADE_OUT
# Get current state (read-only).
animation_tree.get("parameters/OneShot/active")
# Alternative syntax (same result as above).
animation_tree["parameters/OneShot/active"]
# Get current internal state (read-only).
animation_tree.get("parameters/OneShot/internal_active")
# Alternative syntax (same result as above).
animation_tree["parameters/OneShot/internal_active"]
```

```
// Play child animation connected to "shot" port.
animationTree.Set("parameters/OneShot/request", (int)AnimationNodeOneShot.OneShotRequest.Fire);
// Abort child animation connected to "shot" port.
animationTree.Set("parameters/OneShot/request", (int)AnimationNodeOneShot.OneShotRequest.Abort);
// Abort child animation with fading out connected to "shot" port.
animationTree.Set("parameters/OneShot/request", (int)AnimationNodeOneShot.OneShotRequest.FadeOut);
// Get current state (read-only).
animationTree.Get("parameters/OneShot/active");
// Get current internal state (read-only).
animationTree.Get("parameters/OneShot/internal_active");
```

</details>

## 方法

### get_autorestart_delay {#get_autorestart_delay}

```gdscript
float get_autorestart_delay() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_autorestart_random_delay {#get_autorestart_random_delay}

```gdscript
float get_autorestart_random_delay() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fadein_curve {#get_fadein_curve}

```gdscript
Curve get_fadein_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fadein_time {#get_fadein_time}

```gdscript
float get_fadein_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fadeout_curve {#get_fadeout_curve}

```gdscript
Curve get_fadeout_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fadeout_time {#get_fadeout_time}

```gdscript
float get_fadeout_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mix_mode {#get_mix_mode}

```gdscript
AnimationNodeOneShot.MixMode get_mix_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_autorestart {#has_autorestart}

```gdscript
bool has_autorestart() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_aborted_on_reset {#is_aborted_on_reset}

```gdscript
bool is_aborted_on_reset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_loop_broken_at_end {#is_loop_broken_at_end}

```gdscript
bool is_loop_broken_at_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_abort_on_reset {#set_abort_on_reset}

```gdscript
void set_abort_on_reset(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autorestart {#set_autorestart}

```gdscript
void set_autorestart(active: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autorestart_delay {#set_autorestart_delay}

```gdscript
void set_autorestart_delay(time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autorestart_random_delay {#set_autorestart_random_delay}

```gdscript
void set_autorestart_random_delay(time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_break_loop_at_end {#set_break_loop_at_end}

```gdscript
void set_break_loop_at_end(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fadein_curve {#set_fadein_curve}

```gdscript
void set_fadein_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fadein_time {#set_fadein_time}

```gdscript
void set_fadein_time(time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fadeout_curve {#set_fadeout_curve}

```gdscript
void set_fadeout_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fadeout_time {#set_fadeout_time}

```gdscript
void set_fadeout_time(time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mix_mode {#set_mix_mode}

```gdscript
void set_mix_mode(mode: AnimationNodeOneShot.MixMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mix_mode {#mix_mode}

```gdscript
mix_mode: int
```

- void set_mix_mode(value: MixMode)
- MixMode get_mix_mode()

The blend type.

### fadein_time {#fadein_time}

```gdscript
fadein_time: float
```

- void set_fadein_time(value: float)
- float get_fadein_time()

The fade-in duration. For example, setting this to 1.0 for a 5 second length animation will produce a cross-fade that starts at 0 second and ends at 1 second during the animation.

Note: AnimationNodeOneShot transitions the current state after the fading has finished.

### fadein_curve {#fadein_curve}

```gdscript
fadein_curve: Curve
```

- void set_fadein_curve(value: Curve)
- Curve get_fadein_curve()

Determines how cross-fading between animations is eased. If empty, the transition will be linear. Should be a unit Curve.

### fadeout_time {#fadeout_time}

```gdscript
fadeout_time: float
```

- void set_fadeout_time(value: float)
- float get_fadeout_time()

The fade-out duration. For example, setting this to 1.0 for a 5 second length animation will produce a cross-fade that starts at 4 second and ends at 5 second during the animation.

Note: AnimationNodeOneShot transitions the current state after the fading has finished.

### fadeout_curve {#fadeout_curve}

```gdscript
fadeout_curve: Curve
```

- void set_fadeout_curve(value: Curve)
- Curve get_fadeout_curve()

Determines how cross-fading between animations is eased. If empty, the transition will be linear. Should be a unit Curve.

### break_loop_at_end {#break_loop_at_end}

```gdscript
break_loop_at_end: bool
```

- void set_break_loop_at_end(value: bool)
- bool is_loop_broken_at_end()

If true, breaks the loop at the end of the loop cycle for transition, even if the animation is looping.

### abort_on_reset {#abort_on_reset}

```gdscript
abort_on_reset: bool
```

- void set_abort_on_reset(value: bool)
- bool is_aborted_on_reset()

If true, the sub-animation will abort if resumed with a reset after a prior interruption.

### autorestart {#autorestart}

```gdscript
autorestart: bool
```

- void set_autorestart(value: bool)
- bool has_autorestart()

If true, the sub-animation will restart automatically after finishing.

In other words, to start auto restarting, the animation must be played once with the ONE_SHOT_REQUEST_FIRE request. The ONE_SHOT_REQUEST_ABORT request stops the auto restarting, but it does not disable the autorestart itself. So, the ONE_SHOT_REQUEST_FIRE request will start auto restarting again.

### autorestart_delay {#autorestart_delay}

```gdscript
autorestart_delay: float
```

- void set_autorestart_delay(value: float)
- float get_autorestart_delay()

The delay after which the automatic restart is triggered, in seconds.

### autorestart_random_delay {#autorestart_random_delay}

```gdscript
autorestart_random_delay: float
```

- void set_autorestart_random_delay(value: float)
- float get_autorestart_random_delay()

If autorestart is true, a random additional delay (in seconds) between 0 and this value will be added to autorestart_delay.

## 枚举

### OneShotRequest {#enum-OneShotRequest}

```gdscript
enum OneShotRequest { ONE_SHOT_REQUEST_NONE = 0, ONE_SHOT_REQUEST_FIRE = 1, ONE_SHOT_REQUEST_ABORT = 2, ONE_SHOT_REQUEST_FADE_OUT = 3 }
```

OneShotRequest ONE_SHOT_REQUEST_NONE = 0

The default state of the request. Nothing is done.

OneShotRequest ONE_SHOT_REQUEST_FIRE = 1

The request to play the animation connected to "shot" port.

OneShotRequest ONE_SHOT_REQUEST_ABORT = 2

The request to stop the animation connected to "shot" port.

OneShotRequest ONE_SHOT_REQUEST_FADE_OUT = 3

The request to fade out the animation connected to "shot" port.

#### ONE_SHOT_REQUEST_NONE {#ONE_SHOT_REQUEST_NONE}

```gdscript
ONE_SHOT_REQUEST_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ONE_SHOT_REQUEST_FIRE {#ONE_SHOT_REQUEST_FIRE}

```gdscript
ONE_SHOT_REQUEST_FIRE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ONE_SHOT_REQUEST_ABORT {#ONE_SHOT_REQUEST_ABORT}

```gdscript
ONE_SHOT_REQUEST_ABORT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ONE_SHOT_REQUEST_FADE_OUT {#ONE_SHOT_REQUEST_FADE_OUT}

```gdscript
ONE_SHOT_REQUEST_FADE_OUT = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MixMode {#enum-MixMode}

```gdscript
enum MixMode { MIX_MODE_BLEND = 0, MIX_MODE_ADD = 1 }
```

MixMode MIX_MODE_BLEND = 0

Blends two animations. See also AnimationNodeBlend2.

MixMode MIX_MODE_ADD = 1

Blends two animations additively. See also AnimationNodeAdd2.

#### MIX_MODE_BLEND {#MIX_MODE_BLEND}

```gdscript
MIX_MODE_BLEND = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MIX_MODE_ADD {#MIX_MODE_ADD}

```gdscript
MIX_MODE_ADD = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
