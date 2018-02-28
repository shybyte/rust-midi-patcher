use effects::effect::{Effect};
use effects::effect::DeviceName;
use pm::MidiMessage;
use std::sync::{Arc, Mutex};
use utils::control_change;
use virtual_midi::VirtualMidiOutput;
use utils::play_note_on;
use utils::play_note_off;


pub struct PedalButton {
    input_device: String,
    output_device: String,
    note: u8,
    prev_control_value: u8,
}

impl PedalButton {
    pub fn new(input_device: &str, output_device: &str, note: u8) -> PedalButton {
        PedalButton {
            input_device: input_device.to_string(),
            output_device: output_device.to_string(),
            prev_control_value: 0,
            note,
        }
    }
}

impl Effect for PedalButton {
    fn on_midi_event(&mut self, device: &DeviceName,
                     midi_message: MidiMessage,
                     virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if device.contains(&self.input_device) {
//            eprintln!("midi_message = {:?}", midi_message);
            let control_value = midi_message.data2;
            if control_value > 120 && self.prev_control_value <= 120 {
                control_change(&self.input_device, &virtual_midi_out, 74, midi_message.data2);
                eprintln!("Play {} {}", self.note, self.output_device);
                play_note_on(&self.output_device, &virtual_midi_out, self.note, 127);
                play_note_off(&self.output_device, &virtual_midi_out, self.note);
            }
            self.prev_control_value = control_value;
        }
    }
}

