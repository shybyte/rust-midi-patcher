use effects::effect::Effect;
use effects::effect::DeviceName;
use pm::MidiMessage;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use utils::is_note_on;
use virtual_midi::VirtualMidiOutput;
use utils::is_note_off;
use utils::play_note_on;
use utils::play_note_off;
use utils::midi_filter::MidiFilter;

pub struct ButtonMelody {
    button_device: String,
    button_note: u8,
    notes: Vec<i8>,
    base_note: i8,
    notes_index: usize,
    output_device: String,
    current_note: u8,
    reset_duration: Duration,
    last_timestamp: Instant,
}

impl ButtonMelody {
    pub fn new(button_device: &str,
               button_note: u8,
               output_device: &str,
               notes: Vec<i8>,
               base_note: i8,
               reset_duration: Duration,
    ) -> ButtonMelody {
        ButtonMelody {
            button_device: button_device.to_string(),
            button_note,
            output_device: output_device.to_string(),
            notes,
            base_note,
            reset_duration,
            notes_index: 0,
            current_note: 0,
            last_timestamp: Instant::now(),
        }
    }
}

impl Effect for ButtonMelody {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if !(device.contains(&self.button_device) && (is_note_on(midi_message)
            || is_note_off(midi_message)) && midi_message.data1 == self.button_note) {
            return;
        }

        let timestamp = Instant::now();
        let duration_since_last_event = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;

        if { duration_since_last_event > self.reset_duration } {
            self.notes_index = 0;
        }

        if is_note_on(midi_message) {
            let played_note = self.notes[self.notes_index];
            self.notes_index = (self.notes_index + 1) % self.notes.len();
            let final_played_note = (played_note + self.base_note) as u8;
            play_note_on(&self.output_device, virtual_midi_out, final_played_note, midi_message.data2);
            self.current_note = final_played_note;
        } else if is_note_off(midi_message) {
            play_note_off(&self.output_device, virtual_midi_out, self.current_note);
        }
    }
}


pub struct HarmonyButtonMelody {
    pub harmony_input_filter: MidiFilter,
    pub button_melody: ButtonMelody
}

impl Effect for HarmonyButtonMelody {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if self.harmony_input_filter.matches(device, midi_message) {
            self.button_melody.base_note = midi_message.data1 as i8;
            return;
        }

        self.button_melody.on_midi_event(device, midi_message, virtual_midi_out);
    }
}


