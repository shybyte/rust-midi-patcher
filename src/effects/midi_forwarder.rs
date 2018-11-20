use crate::effects::effect::DeviceName;
use crate::effects::effect::Effect;
use crate::pm::MidiMessage;
use std::sync::{Arc, Mutex};
use crate::utils::midi_filter::MidiFilter;
use crate::utils::send_midi;
use crate::virtual_midi::VirtualMidiOutput;


pub struct MidiForwarder {
    input_filter: MidiFilter,
    output_device: String,
}

impl MidiForwarder {
    pub fn new(input_filter: MidiFilter, output_device: &str) -> MidiForwarder {
        MidiForwarder {
            input_filter,
            output_device: output_device.to_string(),
        }
    }
}

impl Effect for MidiForwarder {
    fn on_midi_event(&mut self, device: &DeviceName,
                     midi_message: MidiMessage,
                     virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if self.input_filter.matches(device, midi_message) {
            send_midi(&self.output_device, virtual_midi_out, midi_message);
        }
    }
}
