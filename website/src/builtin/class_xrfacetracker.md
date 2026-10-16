# XRFaceTracker

**继承**：[XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRFaceTracker**

> An instance of this object represents a tracked face and its corresponding blend shapes. The blend shapes come from the Unified Expressions [https://docs.vrcft.io/docs/tutorial-avatars/tutorial-avatars-extras/unified-blendshapes] standard, and contain extended details and visuals for each blend shape. Additionally the Tracking Standard Comparison [https://docs.vrcft.io/docs/tutorial-avatars/tutorial-avatars-extras/compatibility/overview] page documents the relationship between Unified Expressions and other standards.

<details><summary>官方英文描述（中文精译进行中）</summary>

An instance of this object represents a tracked face and its corresponding blend shapes. The blend shapes come from the Unified Expressions [https://docs.vrcft.io/docs/tutorial-avatars/tutorial-avatars-extras/unified-blendshapes] standard, and contain extended details and visuals for each blend shape. Additionally the Tracking Standard Comparison [https://docs.vrcft.io/docs/tutorial-avatars/tutorial-avatars-extras/compatibility/overview] page documents the relationship between Unified Expressions and other standards.

As face trackers are turned on they are registered with the XRServer.

</details>

## 方法

### get_blend_shape {#get_blend_shape}

```gdscript
float get_blend_shape(blend_shape: XRFaceTracker.BlendShapeEntry) const
```

Returns the requested face blend shape weight.

### get_blend_shapes {#get_blend_shapes}

```gdscript
PackedFloat32Array get_blend_shapes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_shape {#set_blend_shape}

```gdscript
void set_blend_shape(blend_shape: XRFaceTracker.BlendShapeEntry, weight: float)
```

Sets a face blend shape weight.

### set_blend_shapes {#set_blend_shapes}

```gdscript
void set_blend_shapes(weights: PackedFloat32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### blend_shapes {#blend_shapes}

```gdscript
blend_shapes: PackedFloat32Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### BlendShapeEntry {#enum-BlendShapeEntry}

```gdscript
enum BlendShapeEntry { FT_EYE_LOOK_OUT_RIGHT = 0, FT_EYE_LOOK_IN_RIGHT = 1, FT_EYE_LOOK_UP_RIGHT = 2, FT_EYE_LOOK_DOWN_RIGHT = 3, FT_EYE_LOOK_OUT_LEFT = 4, FT_EYE_LOOK_IN_LEFT = 5, FT_EYE_LOOK_UP_LEFT = 6, FT_EYE_LOOK_DOWN_LEFT = 7, FT_EYE_CLOSED_RIGHT = 8, FT_EYE_CLOSED_LEFT = 9, FT_EYE_SQUINT_RIGHT = 10, FT_EYE_SQUINT_LEFT = 11, FT_EYE_WIDE_RIGHT = 12, FT_EYE_WIDE_LEFT = 13, FT_EYE_DILATION_RIGHT = 14, FT_EYE_DILATION_LEFT = 15, FT_EYE_CONSTRICT_RIGHT = 16, FT_EYE_CONSTRICT_LEFT = 17, FT_BROW_PINCH_RIGHT = 18, FT_BROW_PINCH_LEFT = 19, FT_BROW_LOWERER_RIGHT = 20, FT_BROW_LOWERER_LEFT = 21, FT_BROW_INNER_UP_RIGHT = 22, FT_BROW_INNER_UP_LEFT = 23, FT_BROW_OUTER_UP_RIGHT = 24, FT_BROW_OUTER_UP_LEFT = 25, FT_NOSE_SNEER_RIGHT = 26, FT_NOSE_SNEER_LEFT = 27, FT_NASAL_DILATION_RIGHT = 28, FT_NASAL_DILATION_LEFT = 29, FT_NASAL_CONSTRICT_RIGHT = 30, FT_NASAL_CONSTRICT_LEFT = 31, FT_CHEEK_SQUINT_RIGHT = 32, FT_CHEEK_SQUINT_LEFT = 33, FT_CHEEK_PUFF_RIGHT = 34, FT_CHEEK_PUFF_LEFT = 35, FT_CHEEK_SUCK_RIGHT = 36, FT_CHEEK_SUCK_LEFT = 37, FT_JAW_OPEN = 38, FT_MOUTH_CLOSED = 39, FT_JAW_RIGHT = 40, FT_JAW_LEFT = 41, FT_JAW_FORWARD = 42, FT_JAW_BACKWARD = 43, FT_JAW_CLENCH = 44, FT_JAW_MANDIBLE_RAISE = 45, FT_LIP_SUCK_UPPER_RIGHT = 46, FT_LIP_SUCK_UPPER_LEFT = 47, FT_LIP_SUCK_LOWER_RIGHT = 48, FT_LIP_SUCK_LOWER_LEFT = 49, FT_LIP_SUCK_CORNER_RIGHT = 50, FT_LIP_SUCK_CORNER_LEFT = 51, FT_LIP_FUNNEL_UPPER_RIGHT = 52, FT_LIP_FUNNEL_UPPER_LEFT = 53, FT_LIP_FUNNEL_LOWER_RIGHT = 54, FT_LIP_FUNNEL_LOWER_LEFT = 55, FT_LIP_PUCKER_UPPER_RIGHT = 56, FT_LIP_PUCKER_UPPER_LEFT = 57, FT_LIP_PUCKER_LOWER_RIGHT = 58, FT_LIP_PUCKER_LOWER_LEFT = 59, FT_MOUTH_UPPER_UP_RIGHT = 60, FT_MOUTH_UPPER_UP_LEFT = 61, FT_MOUTH_LOWER_DOWN_RIGHT = 62, FT_MOUTH_LOWER_DOWN_LEFT = 63, FT_MOUTH_UPPER_DEEPEN_RIGHT = 64, FT_MOUTH_UPPER_DEEPEN_LEFT = 65, FT_MOUTH_UPPER_RIGHT = 66, FT_MOUTH_UPPER_LEFT = 67, FT_MOUTH_LOWER_RIGHT = 68, FT_MOUTH_LOWER_LEFT = 69, FT_MOUTH_CORNER_PULL_RIGHT = 70, FT_MOUTH_CORNER_PULL_LEFT = 71, FT_MOUTH_CORNER_SLANT_RIGHT = 72, FT_MOUTH_CORNER_SLANT_LEFT = 73, FT_MOUTH_FROWN_RIGHT = 74, FT_MOUTH_FROWN_LEFT = 75, FT_MOUTH_STRETCH_RIGHT = 76, FT_MOUTH_STRETCH_LEFT = 77, FT_MOUTH_DIMPLE_RIGHT = 78, FT_MOUTH_DIMPLE_LEFT = 79, FT_MOUTH_RAISER_UPPER = 80, FT_MOUTH_RAISER_LOWER = 81, FT_MOUTH_PRESS_RIGHT = 82, FT_MOUTH_PRESS_LEFT = 83, FT_MOUTH_TIGHTENER_RIGHT = 84, FT_MOUTH_TIGHTENER_LEFT = 85, FT_TONGUE_OUT = 86, FT_TONGUE_UP = 87, FT_TONGUE_DOWN = 88, FT_TONGUE_RIGHT = 89, FT_TONGUE_LEFT = 90, FT_TONGUE_ROLL = 91, FT_TONGUE_BLEND_DOWN = 92, FT_TONGUE_CURL_UP = 93, FT_TONGUE_SQUISH = 94, FT_TONGUE_FLAT = 95, FT_TONGUE_TWIST_RIGHT = 96, FT_TONGUE_TWIST_LEFT = 97, FT_SOFT_PALATE_CLOSE = 98, FT_THROAT_SWALLOW = 99, FT_NECK_FLEX_RIGHT = 100, FT_NECK_FLEX_LEFT = 101, FT_EYE_CLOSED = 102, FT_EYE_WIDE = 103, FT_EYE_SQUINT = 104, FT_EYE_DILATION = 105, FT_EYE_CONSTRICT = 106, FT_BROW_DOWN_RIGHT = 107, FT_BROW_DOWN_LEFT = 108, FT_BROW_DOWN = 109, FT_BROW_UP_RIGHT = 110, FT_BROW_UP_LEFT = 111, FT_BROW_UP = 112, FT_NOSE_SNEER = 113, FT_NASAL_DILATION = 114, FT_NASAL_CONSTRICT = 115, FT_CHEEK_PUFF = 116, FT_CHEEK_SUCK = 117, FT_CHEEK_SQUINT = 118, FT_LIP_SUCK_UPPER = 119, FT_LIP_SUCK_LOWER = 120, FT_LIP_SUCK = 121, FT_LIP_FUNNEL_UPPER = 122, FT_LIP_FUNNEL_LOWER = 123, FT_LIP_FUNNEL = 124, FT_LIP_PUCKER_UPPER = 125, FT_LIP_PUCKER_LOWER = 126, FT_LIP_PUCKER = 127, FT_MOUTH_UPPER_UP = 128, FT_MOUTH_LOWER_DOWN = 129, FT_MOUTH_OPEN = 130, FT_MOUTH_RIGHT = 131, FT_MOUTH_LEFT = 132, FT_MOUTH_SMILE_RIGHT = 133, FT_MOUTH_SMILE_LEFT = 134, FT_MOUTH_SMILE = 135, FT_MOUTH_SAD_RIGHT = 136, FT_MOUTH_SAD_LEFT = 137, FT_MOUTH_SAD = 138, FT_MOUTH_STRETCH = 139, FT_MOUTH_DIMPLE = 140, FT_MOUTH_TIGHTENER = 141, FT_MOUTH_PRESS = 142, FT_MAX = 143 }
```

BlendShapeEntry FT_EYE_LOOK_OUT_RIGHT = 0

Right eye looks outwards.

BlendShapeEntry FT_EYE_LOOK_IN_RIGHT = 1

Right eye looks inwards.

BlendShapeEntry FT_EYE_LOOK_UP_RIGHT = 2

Right eye looks upwards.

BlendShapeEntry FT_EYE_LOOK_DOWN_RIGHT = 3

Right eye looks downwards.

BlendShapeEntry FT_EYE_LOOK_OUT_LEFT = 4

Left eye looks outwards.

BlendShapeEntry FT_EYE_LOOK_IN_LEFT = 5

Left eye looks inwards.

BlendShapeEntry FT_EYE_LOOK_UP_LEFT = 6

Left eye looks upwards.

BlendShapeEntry FT_EYE_LOOK_DOWN_LEFT = 7

Left eye looks downwards.

BlendShapeEntry FT_EYE_CLOSED_RIGHT = 8

Closes the right eyelid.

BlendShapeEntry FT_EYE_CLOSED_LEFT = 9

Closes the left eyelid.

BlendShapeEntry FT_EYE_SQUINT_RIGHT = 10

Squeezes the right eye socket muscles.

BlendShapeEntry FT_EYE_SQUINT_LEFT = 11

Squeezes the left eye socket muscles.

BlendShapeEntry FT_EYE_WIDE_RIGHT = 12

Right eyelid widens beyond relaxed.

BlendShapeEntry FT_EYE_WIDE_LEFT = 13

Left eyelid widens beyond relaxed.

BlendShapeEntry FT_EYE_DILATION_RIGHT = 14

Dilates the right eye pupil.

BlendShapeEntry FT_EYE_DILATION_LEFT = 15

Dilates the left eye pupil.

BlendShapeEntry FT_EYE_CONSTRICT_RIGHT = 16

Constricts the right eye pupil.

BlendShapeEntry FT_EYE_CONSTRICT_LEFT = 17

Constricts the left eye pupil.

BlendShapeEntry FT_BROW_PINCH_RIGHT = 18

Right eyebrow pinches in.

BlendShapeEntry FT_BROW_PINCH_LEFT = 19

Left eyebrow pinches in.

BlendShapeEntry FT_BROW_LOWERER_RIGHT = 20

Outer right eyebrow pulls down.

BlendShapeEntry FT_BROW_LOWERER_LEFT = 21

Outer left eyebrow pulls down.

BlendShapeEntry FT_BROW_INNER_UP_RIGHT = 22

Inner right eyebrow pulls up.

BlendShapeEntry FT_BROW_INNER_UP_LEFT = 23

Inner left eyebrow pulls up.

BlendShapeEntry FT_BROW_OUTER_UP_RIGHT = 24

Outer right eyebrow pulls up.

BlendShapeEntry FT_BROW_OUTER_UP_LEFT = 25

Outer left eyebrow pulls up.

BlendShapeEntry FT_NOSE_SNEER_RIGHT = 26

Right side face sneers.

BlendShapeEntry FT_NOSE_SNEER_LEFT = 27

Left side face sneers.

BlendShapeEntry FT_NASAL_DILATION_RIGHT = 28

Right side nose canal dilates.

BlendShapeEntry FT_NASAL_DILATION_LEFT = 29

Left side nose canal dilates.

BlendShapeEntry FT_NASAL_CONSTRICT_RIGHT = 30

Right side nose canal constricts.

BlendShapeEntry FT_NASAL_CONSTRICT_LEFT = 31

Left side nose canal constricts.

BlendShapeEntry FT_CHEEK_SQUINT_RIGHT = 32

Raises the right side cheek.

BlendShapeEntry FT_CHEEK_SQUINT_LEFT = 33

Raises the left side cheek.

BlendShapeEntry FT_CHEEK_PUFF_RIGHT = 34

Puffs the right side cheek.

BlendShapeEntry FT_CHEEK_PUFF_LEFT = 35

Puffs the left side cheek.

BlendShapeEntry FT_CHEEK_SUCK_RIGHT = 36

Sucks in the right side cheek.

BlendShapeEntry FT_CHEEK_SUCK_LEFT = 37

Sucks in the left side cheek.

BlendShapeEntry FT_JAW_OPEN = 38

Opens jawbone.

BlendShapeEntry FT_MOUTH_CLOSED = 39

Closes the mouth.

BlendShapeEntry FT_JAW_RIGHT = 40

Pushes jawbone right.

BlendShapeEntry FT_JAW_LEFT = 41

Pushes jawbone left.

BlendShapeEntry FT_JAW_FORWARD = 42

Pushes jawbone forward.

BlendShapeEntry FT_JAW_BACKWARD = 43

Pushes jawbone backward.

BlendShapeEntry FT_JAW_CLENCH = 44

Flexes jaw muscles.

BlendShapeEntry FT_JAW_MANDIBLE_RAISE = 45

Raises the jawbone.

BlendShapeEntry FT_LIP_SUCK_UPPER_RIGHT = 46

Upper right lip part tucks in the mouth.

BlendShapeEntry FT_LIP_SUCK_UPPER_LEFT = 47

Upper left lip part tucks in the mouth.

BlendShapeEntry FT_LIP_SUCK_LOWER_RIGHT = 48

Lower right lip part tucks in the mouth.

BlendShapeEntry FT_LIP_SUCK_LOWER_LEFT = 49

Lower left lip part tucks in the mouth.

BlendShapeEntry FT_LIP_SUCK_CORNER_RIGHT = 50

Right lip corner folds into the mouth.

BlendShapeEntry FT_LIP_SUCK_CORNER_LEFT = 51

Left lip corner folds into the mouth.

BlendShapeEntry FT_LIP_FUNNEL_UPPER_RIGHT = 52

Upper right lip part pushes into a funnel.

BlendShapeEntry FT_LIP_FUNNEL_UPPER_LEFT = 53

Upper left lip part pushes into a funnel.

BlendShapeEntry FT_LIP_FUNNEL_LOWER_RIGHT = 54

Lower right lip part pushes into a funnel.

BlendShapeEntry FT_LIP_FUNNEL_LOWER_LEFT = 55

Lower left lip part pushes into a funnel.

BlendShapeEntry FT_LIP_PUCKER_UPPER_RIGHT = 56

Upper right lip part pushes outwards.

BlendShapeEntry FT_LIP_PUCKER_UPPER_LEFT = 57

Upper left lip part pushes outwards.

BlendShapeEntry FT_LIP_PUCKER_LOWER_RIGHT = 58

Lower right lip part pushes outwards.

BlendShapeEntry FT_LIP_PUCKER_LOWER_LEFT = 59

Lower left lip part pushes outwards.

BlendShapeEntry FT_MOUTH_UPPER_UP_RIGHT = 60

Upper right part of the lip pulls up.

BlendShapeEntry FT_MOUTH_UPPER_UP_LEFT = 61

Upper left part of the lip pulls up.

BlendShapeEntry FT_MOUTH_LOWER_DOWN_RIGHT = 62

Lower right part of the lip pulls up.

BlendShapeEntry FT_MOUTH_LOWER_DOWN_LEFT = 63

Lower left part of the lip pulls up.

BlendShapeEntry FT_MOUTH_UPPER_DEEPEN_RIGHT = 64

Upper right lip part pushes in the cheek.

BlendShapeEntry FT_MOUTH_UPPER_DEEPEN_LEFT = 65

Upper left lip part pushes in the cheek.

BlendShapeEntry FT_MOUTH_UPPER_RIGHT = 66

Moves upper lip right.

BlendShapeEntry FT_MOUTH_UPPER_LEFT = 67

Moves upper lip left.

BlendShapeEntry FT_MOUTH_LOWER_RIGHT = 68

Moves lower lip right.

BlendShapeEntry FT_MOUTH_LOWER_LEFT = 69

Moves lower lip left.

BlendShapeEntry FT_MOUTH_CORNER_PULL_RIGHT = 70

Right lip corner pulls diagonally up and out.

BlendShapeEntry FT_MOUTH_CORNER_PULL_LEFT = 71

Left lip corner pulls diagonally up and out.

BlendShapeEntry FT_MOUTH_CORNER_SLANT_RIGHT = 72

Right corner lip slants up.

BlendShapeEntry FT_MOUTH_CORNER_SLANT_LEFT = 73

Left corner lip slants up.

BlendShapeEntry FT_MOUTH_FROWN_RIGHT = 74

Right corner lip pulls down.

BlendShapeEntry FT_MOUTH_FROWN_LEFT = 75

Left corner lip pulls down.

BlendShapeEntry FT_MOUTH_STRETCH_RIGHT = 76

Mouth corner lip pulls out and down.

BlendShapeEntry FT_MOUTH_STRETCH_LEFT = 77

Mouth corner lip pulls out and down.

BlendShapeEntry FT_MOUTH_DIMPLE_RIGHT = 78

Right lip corner is pushed backwards.

BlendShapeEntry FT_MOUTH_DIMPLE_LEFT = 79

Left lip corner is pushed backwards.

BlendShapeEntry FT_MOUTH_RAISER_UPPER = 80

Raises and slightly pushes out the upper mouth.

BlendShapeEntry FT_MOUTH_RAISER_LOWER = 81

Raises and slightly pushes out the lower mouth.

BlendShapeEntry FT_MOUTH_PRESS_RIGHT = 82

Right side lips press and flatten together vertically.

BlendShapeEntry FT_MOUTH_PRESS_LEFT = 83

Left side lips press and flatten together vertically.

BlendShapeEntry FT_MOUTH_TIGHTENER_RIGHT = 84

Right side lips squeeze together horizontally.

BlendShapeEntry FT_MOUTH_TIGHTENER_LEFT = 85

Left side lips squeeze together horizontally.

BlendShapeEntry FT_TONGUE_OUT = 86

Tongue visibly sticks out of the mouth.

BlendShapeEntry FT_TONGUE_UP = 87

Tongue points upwards.

BlendShapeEntry FT_TONGUE_DOWN = 88

Tongue points downwards.

BlendShapeEntry FT_TONGUE_RIGHT = 89

Tongue points right.

BlendShapeEntry FT_TONGUE_LEFT = 90

Tongue points left.

BlendShapeEntry FT_TONGUE_ROLL = 91

Sides of the tongue funnel, creating a roll.

BlendShapeEntry FT_TONGUE_BLEND_DOWN = 92

Tongue arches up then down inside the mouth.

BlendShapeEntry FT_TONGUE_CURL_UP = 93

Tongue arches down then up inside the mouth.

BlendShapeEntry FT_TONGUE_SQUISH = 94

Tongue squishes together and thickens.

BlendShapeEntry FT_TONGUE_FLAT = 95

Tongue flattens and thins out.

BlendShapeEntry FT_TONGUE_TWIST_RIGHT = 96

Tongue tip rotates clockwise, with the rest following gradually.

BlendShapeEntry FT_TONGUE_TWIST_LEFT = 97

Tongue tip rotates counter-clockwise, with the rest following gradually.

BlendShapeEntry FT_SOFT_PALATE_CLOSE = 98

Inner mouth throat closes.

BlendShapeEntry FT_THROAT_SWALLOW = 99

The Adam's apple visibly swallows.

BlendShapeEntry FT_NECK_FLEX_RIGHT = 100

Right side neck visibly flexes.

BlendShapeEntry FT_NECK_FLEX_LEFT = 101

Left side neck visibly flexes.

BlendShapeEntry FT_EYE_CLOSED = 102

Closes both eye lids.

BlendShapeEntry FT_EYE_WIDE = 103

Widens both eye lids.

BlendShapeEntry FT_EYE_SQUINT = 104

Squints both eye lids.

BlendShapeEntry FT_EYE_DILATION = 105

Dilates both pupils.

BlendShapeEntry FT_EYE_CONSTRICT = 106

Constricts both pupils.

BlendShapeEntry FT_BROW_DOWN_RIGHT = 107

Pulls the right eyebrow down and in.

BlendShapeEntry FT_BROW_DOWN_LEFT = 108

Pulls the left eyebrow down and in.

BlendShapeEntry FT_BROW_DOWN = 109

Pulls both eyebrows down and in.

BlendShapeEntry FT_BROW_UP_RIGHT = 110

Right brow appears worried.

BlendShapeEntry FT_BROW_UP_LEFT = 111

Left brow appears worried.

BlendShapeEntry FT_BROW_UP = 112

Both brows appear worried.

BlendShapeEntry FT_NOSE_SNEER = 113

Entire face sneers.

BlendShapeEntry FT_NASAL_DILATION = 114

Both nose canals dilate.

BlendShapeEntry FT_NASAL_CONSTRICT = 115

Both nose canals constrict.

BlendShapeEntry FT_CHEEK_PUFF = 116

Puffs both cheeks.

BlendShapeEntry FT_CHEEK_SUCK = 117

Sucks in both cheeks.

BlendShapeEntry FT_CHEEK_SQUINT = 118

Raises both cheeks.

BlendShapeEntry FT_LIP_SUCK_UPPER = 119

Tucks in the upper lips.

BlendShapeEntry FT_LIP_SUCK_LOWER = 120

Tucks in the lower lips.

BlendShapeEntry FT_LIP_SUCK = 121

Tucks in both lips.

BlendShapeEntry FT_LIP_FUNNEL_UPPER = 122

Funnels in the upper lips.

BlendShapeEntry FT_LIP_FUNNEL_LOWER = 123

Funnels in the lower lips.

BlendShapeEntry FT_LIP_FUNNEL = 124

Funnels in both lips.

BlendShapeEntry FT_LIP_PUCKER_UPPER = 125

Upper lip part pushes outwards.

BlendShapeEntry FT_LIP_PUCKER_LOWER = 126

Lower lip part pushes outwards.

BlendShapeEntry FT_LIP_PUCKER = 127

Lips push outwards.

BlendShapeEntry FT_MOUTH_UPPER_UP = 128

Raises the upper lips.

BlendShapeEntry FT_MOUTH_LOWER_DOWN = 129

Lowers the lower lips.

BlendShapeEntry FT_MOUTH_OPEN = 130

Mouth opens, revealing teeth.

BlendShapeEntry FT_MOUTH_RIGHT = 131

Moves mouth right.

BlendShapeEntry FT_MOUTH_LEFT = 132

Moves mouth left.

BlendShapeEntry FT_MOUTH_SMILE_RIGHT = 133

Right side of the mouth smiles.

BlendShapeEntry FT_MOUTH_SMILE_LEFT = 134

Left side of the mouth smiles.

BlendShapeEntry FT_MOUTH_SMILE = 135

Mouth expresses a smile.

BlendShapeEntry FT_MOUTH_SAD_RIGHT = 136

Right side of the mouth expresses sadness.

BlendShapeEntry FT_MOUTH_SAD_LEFT = 137

Left side of the mouth expresses sadness.

BlendShapeEntry FT_MOUTH_SAD = 138

Mouth expresses sadness.

BlendShapeEntry FT_MOUTH_STRETCH = 139

Mouth stretches.

BlendShapeEntry FT_MOUTH_DIMPLE = 140

Lip corners dimple.

BlendShapeEntry FT_MOUTH_TIGHTENER = 141

Mouth tightens.

BlendShapeEntry FT_MOUTH_PRESS = 142

Mouth presses together.

BlendShapeEntry FT_MAX = 143

Represents the size of the BlendShapeEntry enum.

#### FT_EYE_LOOK_OUT_RIGHT {#FT_EYE_LOOK_OUT_RIGHT}

```gdscript
FT_EYE_LOOK_OUT_RIGHT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_IN_RIGHT {#FT_EYE_LOOK_IN_RIGHT}

```gdscript
FT_EYE_LOOK_IN_RIGHT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_UP_RIGHT {#FT_EYE_LOOK_UP_RIGHT}

```gdscript
FT_EYE_LOOK_UP_RIGHT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_DOWN_RIGHT {#FT_EYE_LOOK_DOWN_RIGHT}

```gdscript
FT_EYE_LOOK_DOWN_RIGHT = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_OUT_LEFT {#FT_EYE_LOOK_OUT_LEFT}

```gdscript
FT_EYE_LOOK_OUT_LEFT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_IN_LEFT {#FT_EYE_LOOK_IN_LEFT}

```gdscript
FT_EYE_LOOK_IN_LEFT = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_UP_LEFT {#FT_EYE_LOOK_UP_LEFT}

```gdscript
FT_EYE_LOOK_UP_LEFT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_LOOK_DOWN_LEFT {#FT_EYE_LOOK_DOWN_LEFT}

```gdscript
FT_EYE_LOOK_DOWN_LEFT = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_CLOSED_RIGHT {#FT_EYE_CLOSED_RIGHT}

```gdscript
FT_EYE_CLOSED_RIGHT = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_CLOSED_LEFT {#FT_EYE_CLOSED_LEFT}

```gdscript
FT_EYE_CLOSED_LEFT = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_SQUINT_RIGHT {#FT_EYE_SQUINT_RIGHT}

```gdscript
FT_EYE_SQUINT_RIGHT = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_SQUINT_LEFT {#FT_EYE_SQUINT_LEFT}

```gdscript
FT_EYE_SQUINT_LEFT = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_WIDE_RIGHT {#FT_EYE_WIDE_RIGHT}

```gdscript
FT_EYE_WIDE_RIGHT = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_WIDE_LEFT {#FT_EYE_WIDE_LEFT}

```gdscript
FT_EYE_WIDE_LEFT = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_DILATION_RIGHT {#FT_EYE_DILATION_RIGHT}

```gdscript
FT_EYE_DILATION_RIGHT = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_DILATION_LEFT {#FT_EYE_DILATION_LEFT}

```gdscript
FT_EYE_DILATION_LEFT = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_CONSTRICT_RIGHT {#FT_EYE_CONSTRICT_RIGHT}

```gdscript
FT_EYE_CONSTRICT_RIGHT = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_CONSTRICT_LEFT {#FT_EYE_CONSTRICT_LEFT}

```gdscript
FT_EYE_CONSTRICT_LEFT = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_PINCH_RIGHT {#FT_BROW_PINCH_RIGHT}

```gdscript
FT_BROW_PINCH_RIGHT = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_PINCH_LEFT {#FT_BROW_PINCH_LEFT}

```gdscript
FT_BROW_PINCH_LEFT = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_LOWERER_RIGHT {#FT_BROW_LOWERER_RIGHT}

```gdscript
FT_BROW_LOWERER_RIGHT = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_LOWERER_LEFT {#FT_BROW_LOWERER_LEFT}

```gdscript
FT_BROW_LOWERER_LEFT = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_INNER_UP_RIGHT {#FT_BROW_INNER_UP_RIGHT}

```gdscript
FT_BROW_INNER_UP_RIGHT = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_INNER_UP_LEFT {#FT_BROW_INNER_UP_LEFT}

```gdscript
FT_BROW_INNER_UP_LEFT = 23
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_OUTER_UP_RIGHT {#FT_BROW_OUTER_UP_RIGHT}

```gdscript
FT_BROW_OUTER_UP_RIGHT = 24
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_OUTER_UP_LEFT {#FT_BROW_OUTER_UP_LEFT}

```gdscript
FT_BROW_OUTER_UP_LEFT = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NOSE_SNEER_RIGHT {#FT_NOSE_SNEER_RIGHT}

```gdscript
FT_NOSE_SNEER_RIGHT = 26
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NOSE_SNEER_LEFT {#FT_NOSE_SNEER_LEFT}

```gdscript
FT_NOSE_SNEER_LEFT = 27
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NASAL_DILATION_RIGHT {#FT_NASAL_DILATION_RIGHT}

```gdscript
FT_NASAL_DILATION_RIGHT = 28
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NASAL_DILATION_LEFT {#FT_NASAL_DILATION_LEFT}

```gdscript
FT_NASAL_DILATION_LEFT = 29
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NASAL_CONSTRICT_RIGHT {#FT_NASAL_CONSTRICT_RIGHT}

```gdscript
FT_NASAL_CONSTRICT_RIGHT = 30
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NASAL_CONSTRICT_LEFT {#FT_NASAL_CONSTRICT_LEFT}

```gdscript
FT_NASAL_CONSTRICT_LEFT = 31
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_SQUINT_RIGHT {#FT_CHEEK_SQUINT_RIGHT}

```gdscript
FT_CHEEK_SQUINT_RIGHT = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_SQUINT_LEFT {#FT_CHEEK_SQUINT_LEFT}

```gdscript
FT_CHEEK_SQUINT_LEFT = 33
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_PUFF_RIGHT {#FT_CHEEK_PUFF_RIGHT}

```gdscript
FT_CHEEK_PUFF_RIGHT = 34
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_PUFF_LEFT {#FT_CHEEK_PUFF_LEFT}

```gdscript
FT_CHEEK_PUFF_LEFT = 35
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_SUCK_RIGHT {#FT_CHEEK_SUCK_RIGHT}

```gdscript
FT_CHEEK_SUCK_RIGHT = 36
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_SUCK_LEFT {#FT_CHEEK_SUCK_LEFT}

```gdscript
FT_CHEEK_SUCK_LEFT = 37
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_OPEN {#FT_JAW_OPEN}

```gdscript
FT_JAW_OPEN = 38
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_CLOSED {#FT_MOUTH_CLOSED}

```gdscript
FT_MOUTH_CLOSED = 39
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_RIGHT {#FT_JAW_RIGHT}

```gdscript
FT_JAW_RIGHT = 40
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_LEFT {#FT_JAW_LEFT}

```gdscript
FT_JAW_LEFT = 41
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_FORWARD {#FT_JAW_FORWARD}

```gdscript
FT_JAW_FORWARD = 42
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_BACKWARD {#FT_JAW_BACKWARD}

```gdscript
FT_JAW_BACKWARD = 43
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_CLENCH {#FT_JAW_CLENCH}

```gdscript
FT_JAW_CLENCH = 44
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_JAW_MANDIBLE_RAISE {#FT_JAW_MANDIBLE_RAISE}

```gdscript
FT_JAW_MANDIBLE_RAISE = 45
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_UPPER_RIGHT {#FT_LIP_SUCK_UPPER_RIGHT}

```gdscript
FT_LIP_SUCK_UPPER_RIGHT = 46
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_UPPER_LEFT {#FT_LIP_SUCK_UPPER_LEFT}

```gdscript
FT_LIP_SUCK_UPPER_LEFT = 47
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_LOWER_RIGHT {#FT_LIP_SUCK_LOWER_RIGHT}

```gdscript
FT_LIP_SUCK_LOWER_RIGHT = 48
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_LOWER_LEFT {#FT_LIP_SUCK_LOWER_LEFT}

```gdscript
FT_LIP_SUCK_LOWER_LEFT = 49
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_CORNER_RIGHT {#FT_LIP_SUCK_CORNER_RIGHT}

```gdscript
FT_LIP_SUCK_CORNER_RIGHT = 50
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_CORNER_LEFT {#FT_LIP_SUCK_CORNER_LEFT}

```gdscript
FT_LIP_SUCK_CORNER_LEFT = 51
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL_UPPER_RIGHT {#FT_LIP_FUNNEL_UPPER_RIGHT}

```gdscript
FT_LIP_FUNNEL_UPPER_RIGHT = 52
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL_UPPER_LEFT {#FT_LIP_FUNNEL_UPPER_LEFT}

```gdscript
FT_LIP_FUNNEL_UPPER_LEFT = 53
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL_LOWER_RIGHT {#FT_LIP_FUNNEL_LOWER_RIGHT}

```gdscript
FT_LIP_FUNNEL_LOWER_RIGHT = 54
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL_LOWER_LEFT {#FT_LIP_FUNNEL_LOWER_LEFT}

```gdscript
FT_LIP_FUNNEL_LOWER_LEFT = 55
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER_UPPER_RIGHT {#FT_LIP_PUCKER_UPPER_RIGHT}

```gdscript
FT_LIP_PUCKER_UPPER_RIGHT = 56
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER_UPPER_LEFT {#FT_LIP_PUCKER_UPPER_LEFT}

```gdscript
FT_LIP_PUCKER_UPPER_LEFT = 57
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER_LOWER_RIGHT {#FT_LIP_PUCKER_LOWER_RIGHT}

```gdscript
FT_LIP_PUCKER_LOWER_RIGHT = 58
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER_LOWER_LEFT {#FT_LIP_PUCKER_LOWER_LEFT}

```gdscript
FT_LIP_PUCKER_LOWER_LEFT = 59
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_UP_RIGHT {#FT_MOUTH_UPPER_UP_RIGHT}

```gdscript
FT_MOUTH_UPPER_UP_RIGHT = 60
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_UP_LEFT {#FT_MOUTH_UPPER_UP_LEFT}

```gdscript
FT_MOUTH_UPPER_UP_LEFT = 61
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_LOWER_DOWN_RIGHT {#FT_MOUTH_LOWER_DOWN_RIGHT}

```gdscript
FT_MOUTH_LOWER_DOWN_RIGHT = 62
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_LOWER_DOWN_LEFT {#FT_MOUTH_LOWER_DOWN_LEFT}

```gdscript
FT_MOUTH_LOWER_DOWN_LEFT = 63
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_DEEPEN_RIGHT {#FT_MOUTH_UPPER_DEEPEN_RIGHT}

```gdscript
FT_MOUTH_UPPER_DEEPEN_RIGHT = 64
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_DEEPEN_LEFT {#FT_MOUTH_UPPER_DEEPEN_LEFT}

```gdscript
FT_MOUTH_UPPER_DEEPEN_LEFT = 65
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_RIGHT {#FT_MOUTH_UPPER_RIGHT}

```gdscript
FT_MOUTH_UPPER_RIGHT = 66
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_LEFT {#FT_MOUTH_UPPER_LEFT}

```gdscript
FT_MOUTH_UPPER_LEFT = 67
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_LOWER_RIGHT {#FT_MOUTH_LOWER_RIGHT}

```gdscript
FT_MOUTH_LOWER_RIGHT = 68
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_LOWER_LEFT {#FT_MOUTH_LOWER_LEFT}

```gdscript
FT_MOUTH_LOWER_LEFT = 69
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_CORNER_PULL_RIGHT {#FT_MOUTH_CORNER_PULL_RIGHT}

```gdscript
FT_MOUTH_CORNER_PULL_RIGHT = 70
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_CORNER_PULL_LEFT {#FT_MOUTH_CORNER_PULL_LEFT}

```gdscript
FT_MOUTH_CORNER_PULL_LEFT = 71
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_CORNER_SLANT_RIGHT {#FT_MOUTH_CORNER_SLANT_RIGHT}

```gdscript
FT_MOUTH_CORNER_SLANT_RIGHT = 72
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_CORNER_SLANT_LEFT {#FT_MOUTH_CORNER_SLANT_LEFT}

```gdscript
FT_MOUTH_CORNER_SLANT_LEFT = 73
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_FROWN_RIGHT {#FT_MOUTH_FROWN_RIGHT}

```gdscript
FT_MOUTH_FROWN_RIGHT = 74
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_FROWN_LEFT {#FT_MOUTH_FROWN_LEFT}

```gdscript
FT_MOUTH_FROWN_LEFT = 75
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_STRETCH_RIGHT {#FT_MOUTH_STRETCH_RIGHT}

```gdscript
FT_MOUTH_STRETCH_RIGHT = 76
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_STRETCH_LEFT {#FT_MOUTH_STRETCH_LEFT}

```gdscript
FT_MOUTH_STRETCH_LEFT = 77
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_DIMPLE_RIGHT {#FT_MOUTH_DIMPLE_RIGHT}

```gdscript
FT_MOUTH_DIMPLE_RIGHT = 78
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_DIMPLE_LEFT {#FT_MOUTH_DIMPLE_LEFT}

```gdscript
FT_MOUTH_DIMPLE_LEFT = 79
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_RAISER_UPPER {#FT_MOUTH_RAISER_UPPER}

```gdscript
FT_MOUTH_RAISER_UPPER = 80
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_RAISER_LOWER {#FT_MOUTH_RAISER_LOWER}

```gdscript
FT_MOUTH_RAISER_LOWER = 81
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_PRESS_RIGHT {#FT_MOUTH_PRESS_RIGHT}

```gdscript
FT_MOUTH_PRESS_RIGHT = 82
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_PRESS_LEFT {#FT_MOUTH_PRESS_LEFT}

```gdscript
FT_MOUTH_PRESS_LEFT = 83
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_TIGHTENER_RIGHT {#FT_MOUTH_TIGHTENER_RIGHT}

```gdscript
FT_MOUTH_TIGHTENER_RIGHT = 84
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_TIGHTENER_LEFT {#FT_MOUTH_TIGHTENER_LEFT}

```gdscript
FT_MOUTH_TIGHTENER_LEFT = 85
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_OUT {#FT_TONGUE_OUT}

```gdscript
FT_TONGUE_OUT = 86
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_UP {#FT_TONGUE_UP}

```gdscript
FT_TONGUE_UP = 87
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_DOWN {#FT_TONGUE_DOWN}

```gdscript
FT_TONGUE_DOWN = 88
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_RIGHT {#FT_TONGUE_RIGHT}

```gdscript
FT_TONGUE_RIGHT = 89
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_LEFT {#FT_TONGUE_LEFT}

```gdscript
FT_TONGUE_LEFT = 90
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_ROLL {#FT_TONGUE_ROLL}

```gdscript
FT_TONGUE_ROLL = 91
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_BLEND_DOWN {#FT_TONGUE_BLEND_DOWN}

```gdscript
FT_TONGUE_BLEND_DOWN = 92
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_CURL_UP {#FT_TONGUE_CURL_UP}

```gdscript
FT_TONGUE_CURL_UP = 93
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_SQUISH {#FT_TONGUE_SQUISH}

```gdscript
FT_TONGUE_SQUISH = 94
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_FLAT {#FT_TONGUE_FLAT}

```gdscript
FT_TONGUE_FLAT = 95
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_TWIST_RIGHT {#FT_TONGUE_TWIST_RIGHT}

```gdscript
FT_TONGUE_TWIST_RIGHT = 96
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_TONGUE_TWIST_LEFT {#FT_TONGUE_TWIST_LEFT}

```gdscript
FT_TONGUE_TWIST_LEFT = 97
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_SOFT_PALATE_CLOSE {#FT_SOFT_PALATE_CLOSE}

```gdscript
FT_SOFT_PALATE_CLOSE = 98
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_THROAT_SWALLOW {#FT_THROAT_SWALLOW}

```gdscript
FT_THROAT_SWALLOW = 99
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NECK_FLEX_RIGHT {#FT_NECK_FLEX_RIGHT}

```gdscript
FT_NECK_FLEX_RIGHT = 100
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NECK_FLEX_LEFT {#FT_NECK_FLEX_LEFT}

```gdscript
FT_NECK_FLEX_LEFT = 101
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_CLOSED {#FT_EYE_CLOSED}

```gdscript
FT_EYE_CLOSED = 102
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_WIDE {#FT_EYE_WIDE}

```gdscript
FT_EYE_WIDE = 103
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_SQUINT {#FT_EYE_SQUINT}

```gdscript
FT_EYE_SQUINT = 104
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_DILATION {#FT_EYE_DILATION}

```gdscript
FT_EYE_DILATION = 105
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_EYE_CONSTRICT {#FT_EYE_CONSTRICT}

```gdscript
FT_EYE_CONSTRICT = 106
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_DOWN_RIGHT {#FT_BROW_DOWN_RIGHT}

```gdscript
FT_BROW_DOWN_RIGHT = 107
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_DOWN_LEFT {#FT_BROW_DOWN_LEFT}

```gdscript
FT_BROW_DOWN_LEFT = 108
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_DOWN {#FT_BROW_DOWN}

```gdscript
FT_BROW_DOWN = 109
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_UP_RIGHT {#FT_BROW_UP_RIGHT}

```gdscript
FT_BROW_UP_RIGHT = 110
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_UP_LEFT {#FT_BROW_UP_LEFT}

```gdscript
FT_BROW_UP_LEFT = 111
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_BROW_UP {#FT_BROW_UP}

```gdscript
FT_BROW_UP = 112
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NOSE_SNEER {#FT_NOSE_SNEER}

```gdscript
FT_NOSE_SNEER = 113
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NASAL_DILATION {#FT_NASAL_DILATION}

```gdscript
FT_NASAL_DILATION = 114
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_NASAL_CONSTRICT {#FT_NASAL_CONSTRICT}

```gdscript
FT_NASAL_CONSTRICT = 115
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_PUFF {#FT_CHEEK_PUFF}

```gdscript
FT_CHEEK_PUFF = 116
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_SUCK {#FT_CHEEK_SUCK}

```gdscript
FT_CHEEK_SUCK = 117
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_CHEEK_SQUINT {#FT_CHEEK_SQUINT}

```gdscript
FT_CHEEK_SQUINT = 118
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_UPPER {#FT_LIP_SUCK_UPPER}

```gdscript
FT_LIP_SUCK_UPPER = 119
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK_LOWER {#FT_LIP_SUCK_LOWER}

```gdscript
FT_LIP_SUCK_LOWER = 120
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_SUCK {#FT_LIP_SUCK}

```gdscript
FT_LIP_SUCK = 121
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL_UPPER {#FT_LIP_FUNNEL_UPPER}

```gdscript
FT_LIP_FUNNEL_UPPER = 122
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL_LOWER {#FT_LIP_FUNNEL_LOWER}

```gdscript
FT_LIP_FUNNEL_LOWER = 123
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_FUNNEL {#FT_LIP_FUNNEL}

```gdscript
FT_LIP_FUNNEL = 124
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER_UPPER {#FT_LIP_PUCKER_UPPER}

```gdscript
FT_LIP_PUCKER_UPPER = 125
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER_LOWER {#FT_LIP_PUCKER_LOWER}

```gdscript
FT_LIP_PUCKER_LOWER = 126
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_LIP_PUCKER {#FT_LIP_PUCKER}

```gdscript
FT_LIP_PUCKER = 127
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_UPPER_UP {#FT_MOUTH_UPPER_UP}

```gdscript
FT_MOUTH_UPPER_UP = 128
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_LOWER_DOWN {#FT_MOUTH_LOWER_DOWN}

```gdscript
FT_MOUTH_LOWER_DOWN = 129
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_OPEN {#FT_MOUTH_OPEN}

```gdscript
FT_MOUTH_OPEN = 130
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_RIGHT {#FT_MOUTH_RIGHT}

```gdscript
FT_MOUTH_RIGHT = 131
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_LEFT {#FT_MOUTH_LEFT}

```gdscript
FT_MOUTH_LEFT = 132
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_SMILE_RIGHT {#FT_MOUTH_SMILE_RIGHT}

```gdscript
FT_MOUTH_SMILE_RIGHT = 133
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_SMILE_LEFT {#FT_MOUTH_SMILE_LEFT}

```gdscript
FT_MOUTH_SMILE_LEFT = 134
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_SMILE {#FT_MOUTH_SMILE}

```gdscript
FT_MOUTH_SMILE = 135
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_SAD_RIGHT {#FT_MOUTH_SAD_RIGHT}

```gdscript
FT_MOUTH_SAD_RIGHT = 136
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_SAD_LEFT {#FT_MOUTH_SAD_LEFT}

```gdscript
FT_MOUTH_SAD_LEFT = 137
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_SAD {#FT_MOUTH_SAD}

```gdscript
FT_MOUTH_SAD = 138
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_STRETCH {#FT_MOUTH_STRETCH}

```gdscript
FT_MOUTH_STRETCH = 139
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_DIMPLE {#FT_MOUTH_DIMPLE}

```gdscript
FT_MOUTH_DIMPLE = 140
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_TIGHTENER {#FT_MOUTH_TIGHTENER}

```gdscript
FT_MOUTH_TIGHTENER = 141
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MOUTH_PRESS {#FT_MOUTH_PRESS}

```gdscript
FT_MOUTH_PRESS = 142
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FT_MAX {#FT_MAX}

```gdscript
FT_MAX = 143
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
