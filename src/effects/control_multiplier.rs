#![allow(clippy::useless_let_if_seq)]


use crate::effects::effect::Effect;
use crate::effects::effect::DeviceName;
use std::sync::{Arc, Mutex};
use crate::utils::control_change;
use crate::virtual_midi::VirtualMidiOutput;
use portmidi::MidiMessage;
use crate::utils::midi_filter::MidiFilter;


pub struct ControlMultiplier {
    input_device1: MidiFilter,
    input_device2: MidiFilter,
    output_device: String,
    control_index: u8,
    value1: u8,
    value2: u8,
}

impl ControlMultiplier {
    pub fn new(input_device1: MidiFilter, input_device2: MidiFilter, output_device: &str, control_index: u8) -> ControlMultiplier {
        ControlMultiplier {
            input_device1: input_device1,
            input_device2: input_device2,
            output_device: output_device.to_string(),
            control_index,
            value1: 0,
            value2: 0,
        }
    }
}

impl Effect for ControlMultiplier {
    fn on_midi_event(&mut self, device: &DeviceName,
                     midi_message: MidiMessage,
                     virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        let mut value_set = false;

        if self.input_device1.matches(device, midi_message) {
            self.value1 = midi_message.data2;
            value_set = true;
        }

        if self.input_device2.matches(device, midi_message) {
            self.value2 = midi_message.data2;
            value_set = true;
        }

        if value_set {
            let value = u16::from(self.value1) * u16::from(self.value2) / 128;
            eprintln!("value = {:?} {:?} => {:?}", self.value1, self.value2, value);
            control_change(&self.output_device, &virtual_midi_out, self.control_index, value as u8);
        }
    }
}
