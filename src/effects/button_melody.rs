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
use std::collections::HashMap;

pub trait HasBaseNote {
    fn set_base_note(&mut self, base_note: i8);
}

pub struct ButtonMelody {
    button_device: String,
    button_notes: Vec<u8>,
    notes: Vec<i8>,
    base_note: i8,
    notes_index: usize,
    output_device: String,
    current_note: Option<u8>,
    reset_duration: Duration,
    last_timestamp: Instant,
    reset_filter: Option<MidiFilter>,
}

impl ButtonMelody {
    pub fn new(button_device: &str,
               button_notes: Vec<u8>,
               output_device: &str,
               notes: Vec<i8>,
               base_note: i8,
               reset_duration: Duration,
    ) -> ButtonMelody {
        ButtonMelody {
            button_device: button_device.to_string(),
            button_notes,
            output_device: output_device.to_string(),
            notes,
            base_note,
            reset_duration,
            notes_index: 0,
            current_note: None,
            last_timestamp: Instant::now(),
            reset_filter: None,
        }
    }

    pub fn with_reset_filter(mut self, filter: MidiFilter) -> Self {
        self.reset_filter = Some(filter);
        self
    }
}

impl Effect for ButtonMelody {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if let Some(ref reset_signal_filter) = self.reset_filter {
            eprintln!(" =================> Test {} {} {:?}", device, midi_message, reset_signal_filter);
            if reset_signal_filter.matches(device, midi_message) {
                eprintln!(" ====================> Reset");
                self.notes_index = 0;
            }
        }

        if !(device.contains(&self.button_device) && (is_note_on(midi_message)
            || is_note_off(midi_message)) && self.button_notes.contains(&midi_message.data1)) {
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
            self.current_note = Some(final_played_note);
        } else if is_note_off(midi_message) {
            self.stop(virtual_midi_out);
        }
    }

    fn stop(&mut self, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if let Some(current_note) = self.current_note {
            play_note_off(&self.output_device, virtual_midi_out, current_note);
            self.current_note = None;
        }
    }
}

impl HasBaseNote for ButtonMelody {
    fn set_base_note(&mut self, base_note: i8) {
        self.base_note = base_note;
    }
}


pub struct ButtonMultiMelody {
    button_device: String,
    button_notes: Vec<u8>,
    notes_by_base_note: HashMap<i8, Vec<i8>>,
    // sequence by base note
    base_note: i8,
    notes_index: usize,
    output_device: String,
    current_note: Option<u8>,
    reset_duration: Duration,
    last_timestamp: Instant,
}

impl ButtonMultiMelody {
    pub fn new(button_device: &str,
               button_notes: Vec<u8>,
               output_device: &str,
               notes_by_base_note: HashMap<i8, Vec<i8>>,    // sequence by base note
               base_note: i8,
               reset_duration: Duration,
    ) -> Self {
        ButtonMultiMelody {
            button_device: button_device.to_string(),
            button_notes,
            output_device: output_device.to_string(),
            notes_by_base_note,
            base_note,
            reset_duration,
            notes_index: 0,
            current_note: None,
            last_timestamp: Instant::now(),
        }
    }
}


impl Effect for ButtonMultiMelody {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if !(device.contains(&self.button_device) && (is_note_on(midi_message)
            || is_note_off(midi_message)) && self.button_notes.contains(&midi_message.data1)) {
            return;
        }

        let timestamp = Instant::now();
        let duration_since_last_event = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;

        if { duration_since_last_event > self.reset_duration } {
            self.notes_index = 0;
        }


        if is_note_on(midi_message) {
            if let Some(notes) = self.notes_by_base_note.get(&self.base_note) {
                let played_note = notes[self.notes_index % notes.len()];
                self.notes_index = (self.notes_index + 1) % notes.len();
                let final_played_note = (played_note + self.base_note) as u8;
                play_note_on(&self.output_device, virtual_midi_out, final_played_note, midi_message.data2);
                self.current_note = Some(final_played_note);
            }
        } else if is_note_off(midi_message) {
            self.stop(virtual_midi_out);
        }
    }

    fn stop(&mut self, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if let Some(current_note) = self.current_note {
            play_note_off(&self.output_device, virtual_midi_out, current_note);
            self.current_note = None;
        }
    }
}


impl HasBaseNote for ButtonMultiMelody {
    fn set_base_note(&mut self, base_note: i8) {
        self.base_note = base_note;
    }
}

pub struct HarmonyButtonMelody<T: HasBaseNote + Effect> {
    pub harmony_input_filter: MidiFilter,
    pub stop_signal_filter: Option<MidiFilter>,
    pub button_melodies: Vec<T>,
    pub active: bool,
}


impl<T> Effect for HarmonyButtonMelody<T>
    where T: HasBaseNote + Effect {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if let Some(ref stop_signal_filter) = self.stop_signal_filter {
            if stop_signal_filter.matches(device, midi_message) {
                eprintln!(" ========> Stop");
                for button_melody in &mut self.button_melodies {
                    button_melody.stop(virtual_midi_out);
                }
                self.active = false;
            }
        }

        if self.harmony_input_filter.matches(device, midi_message) {
            eprintln!(" ========> New Base");
            for button_melody in &mut self.button_melodies {
                button_melody.set_base_note(midi_message.data1 as i8);
            }
            self.active = true;
        }

        if self.active {
            for button_melody in &mut self.button_melodies {
                button_melody.on_midi_event(device, midi_message, virtual_midi_out);
            }
        }
    }
}




