# InputEventMIDI

**继承**：[InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventMIDI**

> InputEventMIDI stores information about messages from MIDI [https://en.wikipedia.org/wiki/MIDI] (Musical Instrument Digital Interface) devices. These may include musical keyboards, synthesizers, and drum machines.

<details><summary>官方英文描述</summary>

InputEventMIDI stores information about messages from MIDI [https://en.wikipedia.org/wiki/MIDI] (Musical Instrument Digital Interface) devices. These may include musical keyboards, synthesizers, and drum machines.

MIDI messages can be received over a 5-pin MIDI connector or over USB. If your device supports both be sure to check the settings in the device to see which output it is using.

By default, Godot does not detect MIDI devices. You need to call OS.open_midi_inputs(), first. You can check which devices are detected with OS.get_connected_midi_inputs(), and close the connection with OS.close_midi_inputs().

```
func _ready():
    OS.open_midi_inputs()
    print(OS.get_connected_midi_inputs())
func _input(input_event):
    if input_event is InputEventMIDI:
        _print_midi_info(input_event)
func _print_midi_info(midi_event):
    print(midi_event)
    print("Channel ", midi_event.channel)
    print("Message ", midi_event.message)
    print("Pitch ", midi_event.pitch)
    print("Velocity ", midi_event.velocity)
    print("Instrument ", midi_event.instrument)
    print("Pressure ", midi_event.pressure)
    print("Controller number: ", midi_event.controller_number)
    print("Controller value: ", midi_event.controller_value)
```

```
public override void _Ready()
{
    OS.OpenMidiInputs();
    GD.Print(OS.GetConnectedMidiInputs());
}
public override void _Input(InputEvent inputEvent)
{
    if (inputEvent is InputEventMidi midiEvent)
    {
        PrintMIDIInfo(midiEvent);
    }
}
private void PrintMIDIInfo(InputEventMidi midiEvent)
{
    GD.Print(midiEvent);
    GD.Print($"Channel {midiEvent.Channel}");
    GD.Print($"Message {midiEvent.Message}");
    GD.Print($"Pitch {midiEvent.Pitch}");
    GD.Print($"Velocity {midiEvent.Velocity}");
    GD.Print($"Instrument {midiEvent.Instrument}");
    GD.Print($"Pressure {midiEvent.Pressure}");
    GD.Print($"Controller number: {midiEvent.ControllerNumber}");
    GD.Print($"Controller value: {midiEvent.ControllerValue}");
}
```

Note: Godot does not support MIDI output, so there is no way to emit MIDI messages from Godot. Only MIDI input is supported.

Note: On the Web platform, using MIDI input requires a browser permission to be granted first. This permission request is performed when calling OS.open_midi_inputs(). MIDI input will not work until the user accepts the permission request.

</details>

## 方法

### get_channel {#get_channel}

```gdscript
int get_channel() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_controller_number {#get_controller_number}

```gdscript
int get_controller_number() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_controller_value {#get_controller_value}

```gdscript
int get_controller_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_instrument {#get_instrument}

```gdscript
int get_instrument() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_message {#get_message}

```gdscript
MIDIMessage get_message() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pitch {#get_pitch}

```gdscript
int get_pitch() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pressure {#get_pressure}

```gdscript
int get_pressure() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_velocity {#get_velocity}

```gdscript
int get_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_channel {#set_channel}

```gdscript
void set_channel(channel: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_controller_number {#set_controller_number}

```gdscript
void set_controller_number(controller_number: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_controller_value {#set_controller_value}

```gdscript
void set_controller_value(controller_value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_instrument {#set_instrument}

```gdscript
void set_instrument(instrument: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_message {#set_message}

```gdscript
void set_message(message: MIDIMessage)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pitch {#set_pitch}

```gdscript
void set_pitch(pitch: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressure {#set_pressure}

```gdscript
void set_pressure(pressure: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_velocity {#set_velocity}

```gdscript
void set_velocity(velocity: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### channel {#channel}

```gdscript
channel: int
```

- void set_channel(value: int)
- int get_channel()

The MIDI channel of this message, ranging from 0 to 15. MIDI channel 9 is reserved for percussion instruments.

### message {#message}

```gdscript
message: int
```

- void set_message(value: MIDIMessage)
- MIDIMessage get_message()

Represents the type of MIDI message (see the MIDIMessage enum).

For more information, see the MIDI message status byte list chart [https://www.midi.org/specifications-old/item/table-2-expanded-messages-list-status-bytes].

### pitch {#pitch}

```gdscript
pitch: int
```

- void set_pitch(value: int)
- int get_pitch()

The pitch index number of this MIDI message. This value ranges from 0 to 127.

On a piano, the middle C is 60, followed by a C-sharp (61), then a D (62), and so on. Each octave is split in offsets of 12. See the "MIDI note number" column of the piano key frequency chart [https://en.wikipedia.org/wiki/Piano_key_frequencies] a full list.

### velocity {#velocity}

```gdscript
velocity: int
```

- void set_velocity(value: int)
- int get_velocity()

The velocity of the MIDI message. This value ranges from 0 to 127. For a musical keyboard, this corresponds to how quickly the key was pressed, and is rarely above 110 in practice.

Note: Some MIDI devices may send a @GlobalScope.MIDI_MESSAGE_NOTE_ON message with 0 velocity and expect it to be treated the same as a @GlobalScope.MIDI_MESSAGE_NOTE_OFF message. If necessary, this can be handled with a few lines of code:

```gdscript
func _input(event):
    if event is InputEventMIDI:
        if event.message == MIDI_MESSAGE_NOTE_ON and event.velocity > 0:
            print("Note pressed!")
```

### instrument {#instrument}

```gdscript
instrument: int
```

- void set_instrument(value: int)
- int get_instrument()

The instrument (also called program or preset) used on this MIDI message. This value ranges from 0 to 127.

To see what each value means, refer to the General MIDI's instrument list [https://en.wikipedia.org/wiki/General_MIDI#Program_change_events]. Keep in mind that the list is off by 1 because it does not begin from 0. A value of 0 corresponds to the acoustic grand piano.

### pressure {#pressure}

```gdscript
pressure: int
```

- void set_pressure(value: int)
- int get_pressure()

The strength of the key being pressed. This value ranges from 0 to 127.

Note: For many devices, this value is always 0. Other devices such as musical keyboards may simulate pressure by changing the velocity, instead.

### controller_number {#controller_number}

```gdscript
controller_number: int
```

- void set_controller_number(value: int)
- int get_controller_number()

The unique number of the controller, if message is @GlobalScope.MIDI_MESSAGE_CONTROL_CHANGE, otherwise this is 0. This value can be used to identify sliders for volume, balance, and panning, as well as switches and pedals on the MIDI device. See the General MIDI specification [https://en.wikipedia.org/wiki/General_MIDI#Controller_events] for a small list.

### controller_value {#controller_value}

```gdscript
controller_value: int
```

- void set_controller_value(value: int)
- int get_controller_value()

The value applied to the controller. If message is @GlobalScope.MIDI_MESSAGE_CONTROL_CHANGE, this value ranges from 0 to 127, otherwise it is 0. See also controller_value.
