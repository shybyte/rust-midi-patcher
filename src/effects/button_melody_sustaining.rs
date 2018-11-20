use crate::effects::effect::Effect;
use crate::effects::effect::DeviceName;
use crate::pm::MidiMessage;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::utils::is_note_on;
use crate::virtual_midi::VirtualMidiOutput;
use crate::utils::play_note_on;
use crate::utils::play_note_off;

pub struct ButtonMelodySustaining {
    button_device: String,
    button_note_start: u8,
    button_note_stop: u8,
    notes: Vec<u8>,
    notes_index: usize,
    output_device: String,
    current_note: Option<u8>,
    debounce_duration: Duration,
    reset_duration: Duration,
    last_timestamp: Instant,
}

impl ButtonMelodySustaining {
    pub fn new(button_device: &str,
               button_note_start: u8,
               button_note_stop: u8,
               output_device: &str,
               notes: Vec<u8>,
               reset_duration: Duration,
               debounce_duration: Duration) -> ButtonMelodySustaining {
        ButtonMelodySustaining {
            button_device: button_device.to_string(),
            button_note_start: button_note_start,
            button_note_stop: button_note_stop,
            output_device: output_device.to_string(),
            notes,
            reset_duration,
            debounce_duration,
            notes_index: 0,
            current_note: None,
            last_timestamp: Instant::now(),
        }
    }
}

impl ButtonMelodySustaining {
    fn stop_current_note(&mut self, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>)  {
        if let Some(current_note) = self.current_note {
            play_note_off(&self.output_device, virtual_midi_out, current_note);
            self.current_note = None;
        }
    }
}

impl Effect for ButtonMelodySustaining {
    fn on_midi_event(&mut self, device: &DeviceName, midi_message: MidiMessage, virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if !(device.contains(&self.button_device) && (is_note_on(midi_message)) &&
            (midi_message.data1 == self.button_note_start || midi_message.data1 == self.button_note_stop)) {
            return;
        }

        let timestamp = Instant::now();
        let duration_since_last_event = timestamp - self.last_timestamp;

        if { duration_since_last_event < self.debounce_duration} {
            return;
        } else if { duration_since_last_event > self.reset_duration} {
            self.notes_index = 0;
        }

        self.last_timestamp = timestamp;

        if midi_message.data1 == self.button_note_start {
            let played_note = self.notes[self.notes_index];
            self.notes_index = (self.notes_index + 1) % self.notes.len();
            play_note_on(&self.output_device, virtual_midi_out, played_note, midi_message.data2);
            self.stop_current_note(virtual_midi_out);
            self.current_note = Some(played_note);
        } else if midi_message.data1 == self.button_note_stop {
            self.stop_current_note(virtual_midi_out);
        }
    }
}



