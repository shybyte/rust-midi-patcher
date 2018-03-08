use effects::effect::{Effect};
use effects::effect::DeviceName;
use pm::MidiMessage;
use std::sync::{Arc, Mutex};
use virtual_midi::VirtualMidiOutput;
use utils::pitch_wheel;


pub struct ControlToPitchForwarder {
    input_device: String,
    output_device: String,
}

impl ControlToPitchForwarder {
    pub fn new(input_device: &str, output_device: &str) -> ControlToPitchForwarder {
        ControlToPitchForwarder {
            input_device: input_device.to_string(),
            output_device: output_device.to_string(),
        }
    }
}

impl Effect for ControlToPitchForwarder {
    fn on_midi_event(&mut self, device: &DeviceName,
                     midi_message: MidiMessage,
                     virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if device.contains(&self.input_device) {
            // eprintln!("==========>  ControlToPitchForwarder {:?}", midi_message);
            pitch_wheel(&self.output_device, &virtual_midi_out, 0, midi_message.data2);
        }
    }
}

