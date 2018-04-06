use effects::effect::Effect;
use effects::effect::DeviceName;
use pm::MidiMessage;
use std::sync::{Arc, Mutex};
use utils::control_change;
use virtual_midi::VirtualMidiOutput;
use utils::range_mapper::RangeToRangeMapper;


pub struct ControlForwarder {
    input_device: String,
    output_device: String,
    control_index: u8,
    value_mapper: Option<RangeToRangeMapper>,
}

impl ControlForwarder {
    pub fn new(input_device: &str, output_device: &str, control_index: u8) -> ControlForwarder {
        ControlForwarder {
            input_device: input_device.to_string(),
            output_device: output_device.to_string(),
            control_index,
            value_mapper: None,
        }
    }

    pub fn new_with_value_mapper(input_device: &str, output_device: &str, control_index: u8,
               value_mapper: RangeToRangeMapper) -> ControlForwarder {
        ControlForwarder {
            input_device: input_device.to_string(),
            output_device: output_device.to_string(),
            control_index,
            value_mapper: Some(value_mapper),
        }
    }
}

impl Effect for ControlForwarder {
    fn on_midi_event(&mut self, device: &DeviceName,
                     midi_message: MidiMessage,
                     virtual_midi_out: &Arc<Mutex<VirtualMidiOutput>>) {
        if device.contains(&self.input_device) {
            eprintln!("midi_message = {:?}", midi_message);
            let mut value = midi_message.data2;

            if let Some(ref value_mapper) = self.value_mapper {
                value = value_mapper.map(value)
            }

            control_change(&self.output_device, &virtual_midi_out, self.control_index, value);
        }
    }
}
