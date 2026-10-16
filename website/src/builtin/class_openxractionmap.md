# OpenXRActionMap

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRActionMap**

> OpenXR uses an action system similar to Godots Input map system to bind inputs and outputs on various types of XR controllers to named actions. OpenXR specifies more detail on these inputs and outputs than Godot supports.

<details><summary>官方英文描述</summary>

OpenXR uses an action system similar to Godots Input map system to bind inputs and outputs on various types of XR controllers to named actions. OpenXR specifies more detail on these inputs and outputs than Godot supports.

Another important distinction is that OpenXR offers no control over these bindings. The bindings we register are suggestions, it is up to the XR runtime to offer users the ability to change these bindings. This allows the XR runtime to fill in the gaps if new hardware becomes available.

The action map therefore needs to be loaded at startup and can't be changed afterwards. This resource is a container for the entire action map.

</details>

## 方法

### add_action_set {#add_action_set}

```gdscript
void add_action_set(action_set: OpenXRActionSet)
```

Add an action set.

### add_interaction_profile {#add_interaction_profile}

```gdscript
void add_interaction_profile(interaction_profile: OpenXRInteractionProfile)
```

Add an interaction profile.

### create_default_action_sets {#create_default_action_sets}

```gdscript
void create_default_action_sets()
```

Setup this action set with our default actions.

### find_action_set {#find_action_set}

```gdscript
OpenXRActionSet find_action_set(name: String) const
```

Retrieve an action set by name.

### find_interaction_profile {#find_interaction_profile}

```gdscript
OpenXRInteractionProfile find_interaction_profile(name: String) const
```

Find an interaction profile by its name (path).

### get_action_set {#get_action_set}

```gdscript
OpenXRActionSet get_action_set(idx: int) const
```

Retrieve the action set at this index.

### get_action_set_count {#get_action_set_count}

```gdscript
int get_action_set_count() const
```

Retrieve the number of actions sets in our action map.

### get_action_sets {#get_action_sets}

```gdscript
Array get_action_sets() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interaction_profile {#get_interaction_profile}

```gdscript
OpenXRInteractionProfile get_interaction_profile(idx: int) const
```

Get the interaction profile at this index.

### get_interaction_profile_count {#get_interaction_profile_count}

```gdscript
int get_interaction_profile_count() const
```

Retrieve the number of interaction profiles in our action map.

### get_interaction_profiles {#get_interaction_profiles}

```gdscript
Array get_interaction_profiles() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_action_set {#remove_action_set}

```gdscript
void remove_action_set(action_set: OpenXRActionSet)
```

Remove an action set.

### remove_interaction_profile {#remove_interaction_profile}

```gdscript
void remove_interaction_profile(interaction_profile: OpenXRInteractionProfile)
```

Remove an interaction profile.

### set_action_sets {#set_action_sets}

```gdscript
void set_action_sets(action_sets: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interaction_profiles {#set_interaction_profiles}

```gdscript
void set_interaction_profiles(interaction_profiles: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### action_sets {#action_sets}

```gdscript
action_sets: OpenXRActionSet
```

- void set_action_sets(value: Array)
- Array get_action_sets()

Collection of OpenXRActionSets that are part of this action map.

### interaction_profiles {#interaction_profiles}

```gdscript
interaction_profiles: OpenXRInteractionProfile
```

- void set_interaction_profiles(value: Array)
- Array get_interaction_profiles()

Collection of OpenXRInteractionProfiles that are part of this action map.
